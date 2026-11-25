//! Core execution engine — applies tweaks atomically with ledger tracking,
//! handles missing pre-mutation sentinel rollback (`__ABSENT__`), and supports `SystemBackend`.

use crate::backup::state_ledger::{LedgerEntry, StateLedger};
use crate::core::backend::{is_unsupported_power_guid_error, RealBackend, SystemBackend};
use crate::core::error::{Result, WinBlastError};
use crate::core::reg_helpers::{
    decode_hex, escape_pipe, format_pipe_reg_target, merge_composite_pair, parse_pipe_reg_target,
    unescape_pipe, RegRawSnapshot, RegTreeSnapshot,
};
use crate::core::risk::{OptionalFeatureKind, PowerTargetScope, TweakAction, TweakDefinition};

/// Sentinel persisted in `StateLedger` when a registry value, absent service, or absent task
/// did not exist before mutation so rollback cleans up accurately instead of inventing keys/crashing.
pub const ABSENT_SENTINEL: &str = "__ABSENT__";
const VAL_ABSENT_SENTINEL: &str = "__VAL_ABSENT__";
const SUBKEY_ABSENT_SENTINEL: &str = "__KEY_ABSENT__";


/// Serialises every system mutation — Instant-mode toggles, batch apply/undo, Auto-Tune and
/// ledger rollback — so two workers never interleave on the same value (F-14). Re-entrant per
/// thread, because batch functions call the single-tweak functions.
static MUTATION_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

thread_local! {
    static HOLDS_MUTATION_LOCK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Held for the duration of one mutation entry point; see [`mutation_guard`].
pub struct MutationGuard {
    guard: Option<std::sync::MutexGuard<'static, ()>>,
}

impl Drop for MutationGuard {
    fn drop(&mut self) {
        if self.guard.take().is_some() {
            HOLDS_MUTATION_LOCK.with(|h| h.set(false));
        }
    }
}

/// Take the global mutation lock (a no-op if this thread already holds it).
pub fn mutation_guard() -> MutationGuard {
    if HOLDS_MUTATION_LOCK.with(|h| h.get()) {
        return MutationGuard { guard: None };
    }
    let guard = MUTATION_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    HOLDS_MUTATION_LOCK.with(|h| h.set(true));
    MutationGuard { guard: Some(guard) }
}

/// Write-ahead wrapper for changes made outside the tweak engine (A1, A6): takes the mutation
/// lock, records the change **before** making it, confirms it afterwards and drops the row if the
/// change failed. Without a ledger the change is refused, because it could not be undone (A7).
pub fn record_then_change<T, E: std::fmt::Display>(
    ledger: Option<&StateLedger>,
    tweak_id: &str,
    action_type: &str,
    original: &str,
    new_value: &str,
    change: impl FnOnce() -> std::result::Result<T, E>,
) -> std::result::Result<T, String> {
    let _lock = mutation_guard();
    let Some(l) = ledger else {
        return Err(
            "Undo history is unavailable (the change ledger could not be opened), so this change was not made."
                .to_string(),
        );
    };
    let id = l
        .begin_change(tweak_id, action_type, original, new_value)
        .map_err(|e| format!("Could not record the change before making it: {e}"))?;
    match change() {
        Ok(v) => {
            if let Err(e) = l.commit_change(id) {
                tracing::warn!("ledger commit of entry {id} failed: {e}");
            }
            Ok(v)
        }
        Err(e) => {
            if let Err(de) = l.discard_change(id) {
                tracing::warn!("ledger discard of entry {id} failed: {de}");
            }
            Err(e.to_string())
        }
    }
}

/// Undo a tweak by rolling back every reversible change WinBlast recorded for it, newest first,
/// so the user's own prior values come back (A4) — not catalog defaults. Returns `None` when
/// nothing is recorded for the tweak; the caller then decides whether to reset to defaults.
pub fn undo_tweak_with(
    tweak_id: &str,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<Option<usize>> {
    let _lock = mutation_guard();
    let mut entries: Vec<LedgerEntry> = Vec::new();
    ledger.for_each_history_by_tweak(tweak_id, |view| {
        if !view.rolled_back && view.is_reversible() {
            entries.push(view.to_owned());
        }
        true
    })?;
    if entries.is_empty() {
        return Ok(None);
    }
    for entry in &entries {
        rollback_entry_with(entry, ledger, backend)?;
    }
    Ok(Some(entries.len()))
}

/// Turn tweaks off: undo what WinBlast recorded, or — when nothing is recorded (the setting was
/// already optimized before WinBlast) — reset it to the Windows default.
pub fn undo_or_reset_tweaks_by_id_with(
    ids: &[String],
    tweaks: &[TweakDefinition],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let _lock = mutation_guard();
    let mut count = 0usize;
    for id in ids {
        match undo_tweak_with(id, ledger, backend)? {
            Some(n) => count += n,
            None => {
                if let Some(t) = tweaks.iter().find(|t| &t.id == id) {
                    count += revert_tweak_with(t, ledger, backend)?;
                }
            }
        }
    }
    Ok(count)
}

/// Ledger prefix of a [`RegValuePrior`] JSON payload (action kind `RegistryValue`).
pub const PRIOR_PREFIX: &str = "prior:";

/// State of one concrete registry value before WinBlast changed it. Rollback restores exactly
/// this type and these bytes (F-3) and deletes keys WinBlast created once they are empty (F-13).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegValuePrior {
    pub raw: Option<RegRawSnapshot>,
    /// Topmost key that did not exist before the change.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_key: Option<String>,
}

/// Target state of a registry value change.
#[derive(Debug, Clone, Copy)]
enum RegDesired<'a> {
    Dword(u32),
    Sz { value: &'a str, expand: bool },
    Delete,
}

/// Where a registry value change goes and who asked for it.
#[derive(Debug, Clone, Copy)]
struct RegChange<'a> {
    tweak_id: &'a str,
    /// "" for apply, "Revert:" for a reset to defaults.
    prefix: &'a str,
    hive: &'a str,
    path: &'a str,
    name: &'a str,
    vendor: Option<&'a str>,
}

/// Text of a `REG_SZ` / `REG_EXPAND_SZ` snapshot (UTF-16LE, trailing NULs removed).
fn snapshot_text(raw: &RegRawSnapshot) -> Option<String> {
    let bytes = decode_hex(&raw.hex_bytes).ok()?;
    let units: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
    let s = String::from_utf16(&units).ok()?;
    Some(s.trim_end_matches('\0').to_string())
}

/// True when the live value already equals the target, so nothing is written or recorded (F-15).
fn already_at_target(raw: &Option<RegRawSnapshot>, desired: RegDesired<'_>) -> bool {
    match (raw, desired) {
        (None, RegDesired::Delete) => true,
        (Some(r), RegDesired::Dword(v)) => match r.reg_type {
            4 => decode_hex(&r.hex_bytes)
                .ok()
                .and_then(|b| <[u8; 4]>::try_from(b.as_slice()).ok())
                .is_some_and(|b| u32::from_le_bytes(b) == v),
            1 | 2 => snapshot_text(r).is_some_and(|s| {
                let t = s.trim();
                t.parse::<u32>().ok() == Some(v) || t.parse::<i32>().ok().map(|i| i as u32) == Some(v)
            }),
            _ => false,
        },
        (Some(r), RegDesired::Sz { value, expand }) => {
            r.reg_type == if expand { 2 } else { 1 } && snapshot_text(r).as_deref() == Some(value)
        }
        _ => false,
    }
}

/// The topmost missing key on the way down to `path`, or `None` when `path` exists.
fn first_missing_key(backend: &dyn SystemBackend, hive: &str, path: &str) -> Option<String> {
    if backend.reg_key_exists(hive, path) {
        return None;
    }
    let mut topmost = path.to_string();
    let mut cur = path;
    while let Some((parent, _)) = cur.rsplit_once('\\') {
        if parent.is_empty() || backend.reg_key_exists(hive, parent) {
            break;
        }
        topmost = parent.to_string();
        cur = parent;
    }
    Some(topmost)
}

/// Delete empty keys from `path` upward, stopping after `topmost` (keys WinBlast created).
fn delete_created_keys(backend: &dyn SystemBackend, hive: &str, path: &str, topmost: &str) -> Result<()> {
    let mut cur = path.to_string();
    loop {
        backend.reg_delete_key_if_empty(hive, &cur)?;
        if cur.eq_ignore_ascii_case(topmost) || cur.len() <= topmost.len() {
            return Ok(());
        }
        match cur.rsplit_once('\\') {
            Some((parent, _)) => cur = parent.to_string(),
            None => return Ok(()),
        }
    }
}

/// Change one registry value on every concrete key it resolves to (network adapters, GPU class
/// subkeys), one ledger row per key (F-16). Each row is written **before** the change (F-4); a
/// value already at the target is skipped (F-15). With `absent_is_target`, a missing value
/// counts as already being at the target (Windows reads it as the default). Returns how many
/// values were changed.
fn apply_registry_value(
    change: RegChange<'_>,
    desired: RegDesired<'_>,
    absent_is_target: bool,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
    recorded_ids: &mut Vec<i64>,
) -> Result<usize> {
    let RegChange { tweak_id, prefix, hive, path, name, vendor } = change;
    let delete_mode = matches!(desired, RegDesired::Delete);
    let paths = backend.reg_resolve_paths(hive, path, vendor, delete_mode)?;
    if paths.is_empty() {
        if delete_mode {
            return Ok(0);
        }
        return Err(WinBlastError::Hardware(format!(
            "No target key found for {hive}\\{path} (no matching active adapter)"
        )));
    }
    let new_str = match desired {
        RegDesired::Dword(v) => v.to_string(),
        RegDesired::Sz { value, expand: true } => format!("expand_sz:{value}"),
        RegDesired::Sz { value, expand: false } => format!("sz:{value}"),
        RegDesired::Delete => ABSENT_SENTINEL.to_string(),
    };
    let mut changed = 0usize;
    for p in &paths {
        let raw = backend.reg_get_raw(hive, p, name);
        if already_at_target(&raw, desired) || (absent_is_target && raw.is_none()) {
            continue;
        }
        let created_key = if raw.is_none() && !delete_mode {
            first_missing_key(backend, hive, p)
        } else {
            None
        };
        let prior = RegValuePrior { raw, created_key };
        let action_key = format!("{prefix}RegistryValue:{}", format_pipe_reg_target(hive, p, name));
        let orig = format!("{PRIOR_PREFIX}{}", serde_json::to_string(&prior)?);
        let id = ledger.begin_change(tweak_id, &action_key, &orig, &new_str)?;
        recorded_ids.push(id);
        match desired {
            RegDesired::Dword(v) => backend.reg_set_dword_literal(hive, p, name, v)?,
            RegDesired::Sz { value, expand } => backend.reg_set_string_literal(hive, p, name, value, expand)?,
            RegDesired::Delete => backend.reg_delete_value_literal(hive, p, name)?,
        }
        ledger.commit_change(id)?;
        changed += 1;
    }
    Ok(changed)
}

fn parse_feature_kind(s: &str) -> OptionalFeatureKind {
    if s.eq_ignore_ascii_case("Capability") {
        OptionalFeatureKind::Capability
    } else {
        OptionalFeatureKind::OptionalFeature
    }
}

fn format_power_indices(scope: PowerTargetScope, ac: u32, dc: u32) -> String {
    match scope {
        PowerTargetScope::AcOnly => format!("ac={}", ac),
        PowerTargetScope::DcOnly => format!("dc={}", dc),
        PowerTargetScope::AcAndDc => format!("ac={},dc={}", ac, dc),
    }
}

fn parse_power_indices(s: &str) -> (Option<u32>, Option<u32>) {
    let mut ac = None;
    let mut dc = None;
    for part in s.split(',') {
        if let Some(v) = part.trim().strip_prefix("ac=") {
            ac = v.parse().ok();
        } else if let Some(v) = part.trim().strip_prefix("dc=") {
            dc = v.parse().ok();
        }
    }
    (ac, dc)
}

fn ensure_tweak_compatible(tweak: &TweakDefinition, backend: &dyn SystemBackend) -> Result<()> {
    let os = backend.detect_os()?;
    if !tweak.is_compatible(os.build_number, &os.edition) {
        return Err(WinBlastError::OsIncompatible(format!(
            "Tweak '{}' is not compatible with Windows build {} ({})",
            tweak.id, os.build_number, os.edition
        )));
    }
    if tweak.gpu_vendor.is_some() {
        let vendors: Vec<_> = backend
            .detect_gpus()
            .into_iter()
            .map(|g| g.vendor)
            .collect();
        if !tweak.is_gpu_vendor_compatible(&vendors) {
            return Err(WinBlastError::Hardware(format!(
                "Tweak '{}' requires a {} GPU",
                tweak.id,
                tweak.gpu_vendor.as_deref().unwrap_or("compatible")
            )));
        }
    }
    Ok(())
}

fn rollback_recorded_ids(
    recorded_ids: Vec<i64>,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) {
    if recorded_ids.is_empty() {
        return;
    }
    if let Ok(history) = ledger.get_history() {
        for id in recorded_ids.into_iter().rev() {
            if let Some(entry) = history.iter().find(|e| e.id == id && !e.rolled_back) {
                if let Err(e) = rollback_entry_with(entry, ledger, backend) {
                    tracing::warn!("compensating rollback of ledger entry {id} failed: {e}");
                }
            }
        }
    }
}

struct CompositeKeyMeta<'a> {
    hive: &'a str,
    path: &'a str,
    name: &'a str,
    entry_key: &'a str,
    pair_separator: &'a str,
    kv_separator: &'a str,
    trailing_separator: bool,
    vendor_ref: Option<&'a str>,
}

fn format_composite_action_key(prefix: &str, meta_in: CompositeKeyMeta<'_>) -> String {
    let mut meta = format!(
        "{}#{}|sep={}|kv={}|trail={}",
        format_pipe_reg_target(meta_in.hive, meta_in.path, meta_in.name),
        escape_pipe(meta_in.entry_key),
        escape_pipe(meta_in.pair_separator),
        escape_pipe(meta_in.kv_separator),
        meta_in.trailing_separator as u8
    );
    if let Some(v) = meta_in.vendor_ref.filter(|v| !v.trim().is_empty()) {
        meta.push_str(&format!("|vendor={}", escape_pipe(v)));
    }
    format!("{}CompositeStringKey:{}", prefix, meta)
}

/// Internal helper to execute a single list of actions atomically for a tweak.
/// If any mutation fails, already-committed actions in this slice are rolled back immediately.
fn apply_action_slice_with(
    tweak: &TweakDefinition,
    actions: &[TweakAction],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let mut recorded_ids: Vec<i64> = Vec::with_capacity(actions.len());
    let vendor_ref = tweak.gpu_vendor.as_deref();

    let res: Result<usize> = (|| {
        let mut applied = 0usize;
        let mut skipped_power = 0usize;
        let mut last_power_err: Option<WinBlastError> = None;
        for action in actions {
            match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    optimized_value,
                    ..
                } => {
                    applied += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "", hive, path, name, vendor: vendor_ref },
                        RegDesired::Dword(*optimized_value),
                        false,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    optimized_value,
                    expand_sz,
                    ..
                } => {
                    let desired = if optimized_value == ABSENT_SENTINEL {
                        RegDesired::Delete
                    } else {
                        RegDesired::Sz { value: optimized_value, expand: *expand_sz }
                    };
                    applied += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "", hive, path, name, vendor: vendor_ref },
                        desired,
                        false,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::RegistryDelete { hive, path, name } => {
                    let Some(snap) = backend.reg_get_raw(hive, path, name) else {
                        continue; // already absent (F-15)
                    };
                    let orig_str = serde_json::to_string(&snap)?;
                    let action_key = format!(
                        "RegistryDelete:{}",
                        format_pipe_reg_target(hive, path, name)
                    );
                    let id =
                        ledger.begin_change(&tweak.id, &action_key, &orig_str, ABSENT_SENTINEL)?;
                    recorded_ids.push(id);
                    backend.reg_delete_value(hive, path, name)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::RegistryDeleteTree { hive, path } => {
                    let Some(tree) = backend.reg_snapshot_tree(hive, path)? else {
                        continue; // already absent (F-15)
                    };
                    let orig_str = serde_json::to_string(&tree)?;
                    let action_key =
                        format!("RegistryDeleteTree:{}\\{}", hive.trim(), escape_pipe(path));
                    let id =
                        ledger.begin_change(&tweak.id, &action_key, &orig_str, ABSENT_SENTINEL)?;
                    recorded_ids.push(id);
                    backend.reg_delete_tree(hive, path)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::RegistryBinaryBit {
                    hive,
                    path,
                    name,
                    byte_index,
                    min_length,
                    bit_mask,
                    optimized_bits,
                    create_if_missing,
                    default_full_hex,
                    ..
                } => {
                    let req = (*min_length).max(*byte_index + 1);
                    let existing = backend.reg_get_binary(hive, path, name);
                    let (orig_masked, mut bytes) = match existing {
                        Some(b) => {
                            if b.len() < req {
                                return Err(WinBlastError::TweakFailed(format!(
                                    "Binary value {}\\{}\\{} shorter than {} bytes (len={})",
                                    hive,
                                    path,
                                    name,
                                    req,
                                    b.len()
                                )));
                            }
                            let orig = format!("{}|{}", bit_mask, b[*byte_index] & *bit_mask);
                            (orig, b)
                        }
                        None => {
                            if *create_if_missing {
                                if let Some(hex_tpl) = default_full_hex {
                                    let decoded = decode_hex(hex_tpl)?;
                                    if decoded.len() < req {
                                        return Err(WinBlastError::invalid_input(
                                            "default_full_hex shorter than min_length",
                                        ));
                                    }
                                    (ABSENT_SENTINEL.to_string(), decoded)
                                } else {
                                    return Err(WinBlastError::invalid_input(
                                        "Missing default_full_hex template for absent binary value",
                                    ));
                                }
                            } else {
                                return Err(WinBlastError::TweakFailed(format!(
                                    "Binary value {}\\{}\\{} absent; length guard prevented corruption",
                                    hive, path, name
                                )));
                            }
                        }
                    };
                    bytes[*byte_index] =
                        (bytes[*byte_index] & !bit_mask) | (optimized_bits & bit_mask);
                    let new_masked = format!("{}|{}", bit_mask, bytes[*byte_index] & *bit_mask);
                    if orig_masked == new_masked {
                        continue; // already at target (F-15)
                    }
                    let action_key = format!(
                        "RegistryBinaryBit:{}|{}",
                        format_pipe_reg_target(hive, path, name),
                        byte_index
                    );
                    let id =
                        ledger.begin_change(&tweak.id, &action_key, &orig_masked, &new_masked)?;
                    recorded_ids.push(id);
                    backend.reg_set_binary(hive, path, name, &bytes)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::CompositeStringKey {
                    hive,
                    path,
                    name,
                    entry_key,
                    optimized_entry_value,
                    pair_separator,
                    kv_separator,
                    trailing_separator,
                    ..
                } => {
                    let existing = backend.reg_get_string(hive, path, name, vendor_ref);
                    let (merged, prev_entry) = merge_composite_pair(
                        existing.as_ref().map(|s| s.value.as_str()),
                        entry_key,
                        optimized_entry_value,
                        pair_separator,
                        kv_separator,
                        *trailing_separator,
                    );
                    let orig_str = match (&existing, &prev_entry) {
                        (None, _) => VAL_ABSENT_SENTINEL.to_string(),
                        (Some(_), None) => SUBKEY_ABSENT_SENTINEL.to_string(),
                        (Some(_), Some(v)) => format!("entry:{}", v),
                    };
                    let new_str = if optimized_entry_value == ABSENT_SENTINEL {
                        SUBKEY_ABSENT_SENTINEL.to_string()
                    } else {
                        format!("entry:{}", optimized_entry_value)
                    };
                    if existing.as_ref().map(|s| s.value.as_str()) == Some(merged.as_str())
                        || (existing.is_none() && optimized_entry_value == ABSENT_SENTINEL)
                    {
                        continue; // already at target (F-15)
                    }
                    let action_key = format_composite_action_key(
                        "",
                        CompositeKeyMeta {
                            hive,
                            path,
                            name,
                            entry_key,
                            pair_separator,
                            kv_separator,
                            trailing_separator: *trailing_separator,
                            vendor_ref,
                        },
                    );
                    let id = ledger.begin_change(&tweak.id, &action_key, &orig_str, &new_str)?;
                    recorded_ids.push(id);
                    if merged.is_empty() && optimized_entry_value == ABSENT_SENTINEL {
                        backend.reg_delete_value_for_vendor(hive, path, name, vendor_ref)?;
                    } else {
                        backend.reg_set_string(hive, path, name, &merged, false, vendor_ref)?;
                    }
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::RegistryDynamicDword {
                    hive,
                    path,
                    name,
                    formula,
                    ..
                } => {
                    if !crate::scanner::dynamic_dword_input_available(backend, *formula) {
                        return Err(WinBlastError::TweakFailed(format!(
                            "{name}: the hardware value this setting is computed from ({formula:?}) could not be read; refusing to write a guessed value"
                        )));
                    }
                    let target = crate::scanner::resolve_dynamic_dword(backend, *formula);
                    applied += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "", hive, path, name, vendor: None },
                        RegDesired::Dword(target),
                        false,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::PowerCfgSetting {
                    subgroup_guid,
                    setting_guid,
                    scheme_guid,
                    optimized_ac,
                    optimized_dc,
                    scope,
                    unhide_attribute,
                    ..
                } => {
                    let resolved_scheme = match scheme_guid {
                        Some(s) if !s.trim().is_empty() => {
                            crate::hardware::power::normalize_guid_str(s)
                        }
                        _ => backend.get_active_power_scheme()?,
                    };
                    let (prev_ac, prev_dc) = match backend.read_power_setting_index(
                        Some(&resolved_scheme),
                        subgroup_guid,
                        setting_guid,
                    ) {
                        Ok(pair) => pair,
                        Err(e) if is_unsupported_power_guid_error(&e) => {
                            tracing::warn!(
                                "Skipping unsupported power setting {}/{}: {}",
                                subgroup_guid,
                                setting_guid,
                                e
                            );
                            skipped_power += 1;
                            last_power_err = Some(e);
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    let ac_opt =
                        if matches!(scope, PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc) {
                            Some(*optimized_ac)
                        } else {
                            None
                        };
                    let dc_opt =
                        if matches!(scope, PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc) {
                            Some(*optimized_dc)
                        } else {
                            None
                        };
                    if ac_opt.is_none_or(|v| v == prev_ac) && dc_opt.is_none_or(|v| v == prev_dc) {
                        continue; // already at target (F-15)
                    }
                    let action_key = format!(
                        "PowerCfgSetting:{}|{}|{}",
                        resolved_scheme,
                        crate::hardware::power::normalize_guid_str(subgroup_guid),
                        crate::hardware::power::normalize_guid_str(setting_guid)
                    );
                    let id = ledger.begin_change(
                        &tweak.id,
                        &action_key,
                        &format_power_indices(*scope, prev_ac, prev_dc),
                        &format_power_indices(*scope, *optimized_ac, *optimized_dc),
                    )?;
                    recorded_ids.push(id);
                    if let Err(e) = backend.write_power_setting_index(
                        Some(&resolved_scheme),
                        subgroup_guid,
                        setting_guid,
                        ac_opt,
                        dc_opt,
                        *unhide_attribute,
                    ) {
                        if is_unsupported_power_guid_error(&e) {
                            tracing::warn!(
                                "Skipping unsupported power setting write {}/{}: {}",
                                subgroup_guid,
                                setting_guid,
                                e
                            );
                            recorded_ids.pop();
                            ledger.discard_change(id)?;
                            skipped_power += 1;
                            last_power_err = Some(e);
                            continue;
                        }
                        return Err(e);
                    }
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::ServiceState {
                    service_name,
                    target_start_type,
                    ..
                } => {
                    let prev = backend.get_service_start_type(service_name);
                    let prev_delayed = if prev == Some(2) {
                        backend
                            .reg_get_dword(
                                "HKLM",
                                &format!(
                                    "SYSTEM\\CurrentControlSet\\Services\\{}",
                                    service_name.trim()
                                ),
                                "DelayedAutostart",
                            )
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    if prev == Some(*target_start_type) {
                        continue; // already at target (F-15)
                    }
                    let action_key = format!("ServiceState:{}", service_name);
                    let orig_str = match prev {
                        Some(2) if prev_delayed == 1 => "2|delayed=1".to_string(),
                        Some(v) => v.to_string(),
                        None => ABSENT_SENTINEL.to_string(),
                    };
                    let id = ledger.begin_change(
                        &tweak.id,
                        &action_key,
                        &orig_str,
                        &target_start_type.to_string(),
                    )?;
                    recorded_ids.push(id);
                    backend.set_service_start_type(service_name, *target_start_type)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    let prev = backend.get_task_enabled(task_path);
                    if prev == Some(*enable) {
                        continue; // already at target (F-15)
                    }
                    let action_key = format!("ScheduledTask:{}", task_path);
                    let orig_str = prev
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    let id = ledger.begin_change(
                        &tweak.id,
                        &action_key,
                        &orig_str,
                        &enable.to_string(),
                    )?;
                    recorded_ids.push(id);
                    backend.set_scheduled_task_enabled(task_path, *enable)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::AppXPackage { package_family } => {
                    let action_key = format!("AppXPackage:{}", package_family);
                    let id =
                        ledger.begin_change(&tweak.id, &action_key, "installed", "removed")?;
                    recorded_ids.push(id);
                    backend.remove_appx_package(package_family)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
                TweakAction::WindowsFeature {
                    feature_name,
                    kind,
                    enable,
                } => {
                    let feats = backend.list_optional_features()?;
                    let req_prefix = feature_name
                        .split_once("~~~~")
                        .map(|(p, _)| p)
                        .unwrap_or(feature_name);
                    let prev_enabled = feats
                        .iter()
                        .find(|f| {
                            f.name.eq_ignore_ascii_case(feature_name)
                                || f.name
                                    .split_once("~~~~")
                                    .map(|(p, _)| p)
                                    .unwrap_or(&f.name)
                                    .eq_ignore_ascii_case(req_prefix)
                        })
                        .map(|f| {
                            matches!(
                                f.state,
                                crate::core::risk::FeatureInstallState::Enabled
                                    | crate::core::risk::FeatureInstallState::EnablePending
                            )
                        })
                        .unwrap_or(!*enable);
                    if prev_enabled == *enable {
                        continue; // already at target (F-15)
                    }
                    let action_key =
                        format!("WindowsFeature:{:?}|{}", kind, escape_pipe(feature_name));
                    let id = ledger.begin_change(
                        &tweak.id,
                        &action_key,
                        &prev_enabled.to_string(),
                        &enable.to_string(),
                    )?;
                    recorded_ids.push(id);
                    backend.set_optional_feature(feature_name, *kind, *enable)?;
                    ledger.commit_change(id)?;
                    applied += 1;
                }
            }
        }
        if applied == 0 && skipped_power > 0 {
            if let Some(err) = last_power_err {
                return Err(err);
            }
        }
        Ok(applied)
    })();

    if let Err(err) = res {
        rollback_recorded_ids(recorded_ids, ledger, backend);
        return Err(err);
    }

    res
}

/// Apply a single tweak definition using an explicit `SystemBackend`, recording exact pre-change
/// states in the ledger. Supports multi-strategy execution: if the primary actions fail,
/// it rolls back partial mutations and sequentially attempts each defined fallback strategy.
pub fn apply_tweak_with(
    tweak: &TweakDefinition,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let _lock = mutation_guard();
    ensure_tweak_compatible(tweak, backend)?;

    // 1. Attempt Primary strategy
    match apply_action_slice_with(tweak, &tweak.actions, ledger, backend) {
        Ok(count) => Ok(count),
        Err(primary_err) => {
            if tweak.fallbacks.is_empty() {
                return Err(primary_err);
            }

            tracing::warn!(
                "Primary actions for tweak '{}' failed: {}. Attempting {} fallback strategies...",
                tweak.id,
                primary_err,
                tweak.fallbacks.len()
            );

            // 2. Attempt Fallbacks sequentially
            for (idx, fallback_actions) in tweak.fallbacks.iter().enumerate() {
                match apply_action_slice_with(tweak, fallback_actions, ledger, backend) {
                    Ok(count) => {
                        tracing::info!(
                            "Fallback strategy #{} succeeded for tweak '{}' ({} actions applied).",
                            idx + 1,
                            tweak.id,
                            count
                        );
                        return Ok(count);
                    }
                    Err(fallback_err) => {
                        tracing::warn!(
                            "Fallback strategy #{} failed for tweak '{}': {}",
                            idx + 1,
                            tweak.id,
                            fallback_err
                        );
                    }
                }
            }

            // All fallback strategies exhausted; return the original primary error
            Err(primary_err)
        }
    }
}

/// Apply multiple tweaks by ID using an explicit `SystemBackend`.
/// Uses `DijkstraScheduler` to resolve prerequisite dependencies and sequence execution
/// to minimize service disruptions and reboot requirements, and checks `RavenEngine` for conflict filtering.
pub fn apply_tweaks_by_id_with(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let _lock = mutation_guard();
    if tweak_ids.is_empty() {
        return Ok(0);
    }

    // Automated pre-flight VSS restore point check (within last 24 hours)
    if let Err(e) = backend.ensure_preflight_restore_point("WinBlast Pre-Optimization Checkpoint") {
        tracing::warn!(
            "Pre-flight system restore point check/creation skipped or failed: {}",
            e
        );
    }

    // 1. Check for mutual conflicts via Raven engine
    let raven = crate::graph::raven::RavenEngine::default();
    let candidates: Vec<crate::graph::raven::TweakCriteria> = all_tweaks
        .iter()
        .filter(|t| tweak_ids.contains(&t.id))
        .map(|t| {
            let stab_score = match t.valuation.stability_risk {
                crate::core::risk::RiskLevel::Safe => 0.0,
                crate::core::risk::RiskLevel::Low => 2.5,
                crate::core::risk::RiskLevel::Moderate => 5.0,
                crate::core::risk::RiskLevel::High => 8.5,
                crate::core::risk::RiskLevel::Extreme => 10.0,
            };
            let rev_score = match t.valuation.reversibility {
                crate::core::risk::Reversibility::Instant => 10.0,
                crate::core::risk::Reversibility::RequiresLogoff => 7.0,
                crate::core::risk::Reversibility::RequiresReboot => 4.0,
                crate::core::risk::Reversibility::RequiresPackageReinstall => 2.0,
            };
            crate::graph::raven::TweakCriteria {
                id: t.id.clone(),
                title: t.title.clone(),
                performance_gain: t.valuation.performance_gain as f32,
                privacy_gain: t.valuation.privacy_gain as f32,
                stability_risk: stab_score,
                reversibility_score: rev_score,
                conflicts_with: t.conflicts_with.clone(),
            }
        })
        .collect();

    let (active_ids, _conflicts) = raven.resolve_conflicts(&candidates);

    // 2. Schedule execution sequence via Dijkstra shortest-path / topological scheduler
    let scheduler = crate::graph::dijkstra::DijkstraScheduler::new();
    let tweak_tuples = all_tweaks.iter().map(|t| {
        let requires_reboot =
            t.valuation.reversibility == crate::core::risk::Reversibility::RequiresReboot;
        let touches_services = t
            .actions
            .iter()
            .any(|a| matches!(a, TweakAction::ServiceState { .. }));
        (
            t.id.as_str(),
            t.prerequisites.as_slice(),
            requires_reboot,
            touches_services,
        )
    });

    let execution_order = scheduler.schedule_execution(tweak_tuples, &active_ids);

    // 3. Sequentially apply tweaks in scheduled order
    let mut total = 0usize;
    let mut applied_set = std::collections::HashSet::new();

    for step in &execution_order.steps {
        if let Some(tweak) = all_tweaks.iter().find(|t| t.id == step.tweak_id) {
            total += apply_tweak_with(tweak, ledger, backend)?;
            applied_set.insert(step.tweak_id.clone());
        }
    }

    // Apply any unmapped IDs remaining
    for id in &active_ids {
        if !applied_set.contains(id) {
            if let Some(tweak) = all_tweaks.iter().find(|t| t.id == *id) {
                total += apply_tweak_with(tweak, ledger, backend)?;
                applied_set.insert(id.clone());
            }
        }
    }

    Ok(total)
}

/// Revert a single tweak definition back to Windows defaults using an explicit `SystemBackend`.
/// If action `k > 0` fails, already-committed revert mutations `0..k` are rolled back atomically.
pub fn revert_tweak_with(
    tweak: &TweakDefinition,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let _lock = mutation_guard();
    ensure_tweak_compatible(tweak, backend)?;
    let mut recorded_ids: Vec<i64> = Vec::with_capacity(tweak.actions.len());
    let mut consumed_prior_ids: Vec<i64> = Vec::new();
    let vendor_ref = tweak.gpu_vendor.as_deref();

    let res: Result<usize> = (|| {
        let mut reverted = 0usize;
        let mut skipped_power = 0usize;
        let mut last_power_err: Option<WinBlastError> = None;

        for action in &tweak.actions {
            match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    default_value,
                    ..
                } => {
                    // An absent value already reads as the Windows default (scanner rule), so
                    // a reset never creates keys or policies that did not exist (F-6).
                    reverted += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "Revert:", hive, path, name, vendor: vendor_ref },
                        RegDesired::Dword(*default_value),
                        true,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    default_value,
                    expand_sz,
                    absent_is_default,
                    ..
                } => {
                    let desired = if *absent_is_default || default_value == ABSENT_SENTINEL {
                        RegDesired::Delete
                    } else {
                        RegDesired::Sz { value: default_value, expand: *expand_sz }
                    };
                    reverted += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "Revert:", hive, path, name, vendor: vendor_ref },
                        desired,
                        *absent_is_default,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::RegistryDelete { hive, path, name } => {
                    let action_key = format!(
                        "RegistryDelete:{}",
                        format_pipe_reg_target(hive, path, name)
                    );
                    let history = ledger.get_history()?;
                    if let Some(prior) = history.iter().find(|e| {
                        e.tweak_id == tweak.id && e.action_type == action_key && !e.rolled_back
                    }) {
                        if prior.original_value != ABSENT_SENTINEL {
                            let snap: RegRawSnapshot = serde_json::from_str(&prior.original_value)?;
                            backend.reg_set_raw(hive, path, name, &snap)?;
                        }
                        let id = ledger.record_change(
                            &tweak.id,
                            &format!("Revert:{}", action_key),
                            ABSENT_SENTINEL,
                            &prior.original_value,
                        )?;
                        recorded_ids.push(id);
                        consumed_prior_ids.push(prior.id);
                        reverted += 1;
                    }
                }
                TweakAction::RegistryDeleteTree { hive, path } => {
                    let action_key =
                        format!("RegistryDeleteTree:{}\\{}", hive.trim(), escape_pipe(path));
                    let history = ledger.get_history()?;
                    if let Some(prior) = history.iter().find(|e| {
                        e.tweak_id == tweak.id && e.action_type == action_key && !e.rolled_back
                    }) {
                        if prior.original_value != ABSENT_SENTINEL {
                            let tree: RegTreeSnapshot =
                                serde_json::from_str(&prior.original_value)?;
                            backend.reg_restore_tree(hive, path, &tree)?;
                        }
                        let id = ledger.record_change(
                            &tweak.id,
                            &format!("Revert:{}", action_key),
                            ABSENT_SENTINEL,
                            &prior.original_value,
                        )?;
                        recorded_ids.push(id);
                        consumed_prior_ids.push(prior.id);
                        reverted += 1;
                    }
                }
                TweakAction::RegistryBinaryBit {
                    hive,
                    path,
                    name,
                    byte_index,
                    min_length,
                    bit_mask,
                    default_bits,
                    create_if_missing,
                    default_full_hex,
                    ..
                } => {
                    let req = (*min_length).max(*byte_index + 1);
                    let (cur_masked, mut bytes) = match backend.reg_get_binary(hive, path, name) {
                        Some(b) => {
                            if b.len() < req {
                                return Err(WinBlastError::TweakFailed(format!(
                                    "Binary value {}\\{}\\{} shorter than {} bytes (len={})",
                                    hive,
                                    path,
                                    name,
                                    req,
                                    b.len()
                                )));
                            }
                            let cur = format!("{}|{}", bit_mask, b[*byte_index] & *bit_mask);
                            (cur, b)
                        }
                        None => {
                            if *create_if_missing {
                                if let Some(hex_tpl) = default_full_hex {
                                    let decoded = decode_hex(hex_tpl)?;
                                    if decoded.len() < req {
                                        return Err(WinBlastError::invalid_input(
                                            "default_full_hex shorter than min_length",
                                        ));
                                    }
                                    (ABSENT_SENTINEL.to_string(), decoded)
                                } else {
                                    return Err(WinBlastError::invalid_input(
                                        "Missing default_full_hex template for absent binary value",
                                    ));
                                }
                            } else {
                                return Err(WinBlastError::TweakFailed(format!(
                                    "Binary value {}\\{}\\{} absent; length guard prevented corruption",
                                    hive, path, name
                                )));
                            }
                        }
                    };
                    bytes[*byte_index] =
                        (bytes[*byte_index] & !bit_mask) | (default_bits & bit_mask);
                    let new_masked = format!("{}|{}", bit_mask, bytes[*byte_index] & *bit_mask);
                    backend.reg_set_binary(hive, path, name, &bytes)?;
                    let action_key = format!(
                        "RegistryBinaryBit:{}|{}",
                        format_pipe_reg_target(hive, path, name),
                        byte_index
                    );
                    let id = ledger.record_change(
                        &tweak.id,
                        &format!("Revert:{}", action_key),
                        &cur_masked,
                        &new_masked,
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::CompositeStringKey {
                    hive,
                    path,
                    name,
                    entry_key,
                    default_entry_value,
                    pair_separator,
                    kv_separator,
                    trailing_separator,
                    ..
                } => {
                    let existing = backend.reg_get_string(hive, path, name, vendor_ref);
                    let (merged, prev_entry) = merge_composite_pair(
                        existing.as_ref().map(|s| s.value.as_str()),
                        entry_key,
                        default_entry_value,
                        pair_separator,
                        kv_separator,
                        *trailing_separator,
                    );
                    let orig_str = match (&existing, &prev_entry) {
                        (None, _) => VAL_ABSENT_SENTINEL.to_string(),
                        (Some(_), None) => SUBKEY_ABSENT_SENTINEL.to_string(),
                        (Some(_), Some(v)) => format!("entry:{}", v),
                    };
                    if merged.is_empty() && default_entry_value == ABSENT_SENTINEL {
                        backend.reg_delete_value_for_vendor(hive, path, name, vendor_ref)?;
                    } else {
                        backend.reg_set_string(hive, path, name, &merged, false, vendor_ref)?;
                    }
                    let new_str = if default_entry_value == ABSENT_SENTINEL {
                        SUBKEY_ABSENT_SENTINEL.to_string()
                    } else {
                        format!("entry:{}", default_entry_value)
                    };
                    let action_key = format_composite_action_key(
                        "Revert:",
                        CompositeKeyMeta {
                            hive,
                            path,
                            name,
                            entry_key,
                            pair_separator,
                            kv_separator,
                            trailing_separator: *trailing_separator,
                            vendor_ref,
                        },
                    );
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, &new_str)?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::RegistryDynamicDword {
                    hive,
                    path,
                    name,
                    default_value,
                    ..
                } => {
                    reverted += apply_registry_value(
                        RegChange { tweak_id: &tweak.id, prefix: "Revert:", hive, path, name, vendor: None },
                        RegDesired::Dword(*default_value),
                        true,
                        ledger,
                        backend,
                        &mut recorded_ids,
                    )?;
                }
                TweakAction::PowerCfgSetting {
                    subgroup_guid,
                    setting_guid,
                    scheme_guid,
                    default_ac,
                    default_dc,
                    scope,
                    ..
                } => {
                    let resolved_scheme = match scheme_guid {
                        Some(s) if !s.trim().is_empty() => {
                            crate::hardware::power::normalize_guid_str(s)
                        }
                        _ => backend.get_active_power_scheme()?,
                    };
                    let (prev_ac, prev_dc) = match backend.read_power_setting_index(
                        Some(&resolved_scheme),
                        subgroup_guid,
                        setting_guid,
                    ) {
                        Ok(pair) => pair,
                        Err(e) if is_unsupported_power_guid_error(&e) => {
                            tracing::warn!(
                                "Skipping unsupported power setting revert {}/{}: {}",
                                subgroup_guid,
                                setting_guid,
                                e
                            );
                            skipped_power += 1;
                            last_power_err = Some(e);
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    let ac_opt =
                        if matches!(scope, PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc) {
                            Some(*default_ac)
                        } else {
                            None
                        };
                    let dc_opt =
                        if matches!(scope, PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc) {
                            Some(*default_dc)
                        } else {
                            None
                        };
                    if let Err(e) = backend.write_power_setting_index(
                        Some(&resolved_scheme),
                        subgroup_guid,
                        setting_guid,
                        ac_opt,
                        dc_opt,
                        false,
                    ) {
                        if is_unsupported_power_guid_error(&e) {
                            tracing::warn!(
                                "Skipping unsupported power setting revert write {}/{}: {}",
                                subgroup_guid,
                                setting_guid,
                                e
                            );
                            skipped_power += 1;
                            last_power_err = Some(e);
                            continue;
                        }
                        return Err(e);
                    }
                    let action_key = format!(
                        "PowerCfgSetting:{}|{}|{}",
                        resolved_scheme,
                        crate::hardware::power::normalize_guid_str(subgroup_guid),
                        crate::hardware::power::normalize_guid_str(setting_guid)
                    );
                    let id = ledger.record_change(
                        &tweak.id,
                        &format!("Revert:{}", action_key),
                        &format_power_indices(*scope, prev_ac, prev_dc),
                        &format_power_indices(*scope, *default_ac, *default_dc),
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::ServiceState {
                    service_name,
                    default_start_type,
                    ..
                } => {
                    let Some(current_start) = backend.get_service_start_type(service_name) else {
                        continue;
                    };
                    let prev_delayed = if current_start == 2 {
                        backend
                            .reg_get_dword(
                                "HKLM",
                                &format!(
                                    "SYSTEM\\CurrentControlSet\\Services\\{}",
                                    service_name.trim()
                                ),
                                "DelayedAutostart",
                            )
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    backend.set_service_start_type(service_name, *default_start_type)?;

                    let orig_str = if current_start == 2 && prev_delayed == 1 {
                        "2|delayed=1".to_string()
                    } else {
                        current_start.to_string()
                    };
                    let action_key = format!("ServiceState:{}", service_name);
                    let id = ledger.record_change(
                        &tweak.id,
                        &format!("Revert:{}", action_key),
                        &orig_str,
                        &default_start_type.to_string(),
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    let default_state = !*enable;
                    let prev = backend.get_task_enabled(task_path);
                    if prev.is_none() && default_state {
                        continue;
                    }
                    let current_str = prev
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    backend.set_scheduled_task_enabled(task_path, default_state)?;

                    let action_key = format!("ScheduledTask:{}", task_path);
                    let id = ledger.record_change(
                        &tweak.id,
                        &format!("Revert:{}", action_key),
                        &current_str,
                        &default_state.to_string(),
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::AppXPackage { .. } => {}
                TweakAction::WindowsFeature {
                    feature_name,
                    kind,
                    enable,
                } => {
                    let feats = backend.list_optional_features()?;
                    let req_prefix = feature_name
                        .split_once("~~~~")
                        .map(|(p, _)| p)
                        .unwrap_or(feature_name);
                    let Some(feat) = feats.iter().find(|f| {
                        f.name.eq_ignore_ascii_case(feature_name)
                            || f.name
                                .split_once("~~~~")
                                .map(|(p, _)| p)
                                .unwrap_or(&f.name)
                                .eq_ignore_ascii_case(req_prefix)
                    }) else {
                        return Err(WinBlastError::TweakFailed(format!(
                            "Windows feature '{}' is not registered on this Windows edition (NotFound)",
                            feature_name
                        )));
                    };
                    let prev_enabled = matches!(
                        feat.state,
                        crate::core::risk::FeatureInstallState::Enabled
                            | crate::core::risk::FeatureInstallState::EnablePending
                    );
                    let revert_to = !*enable;
                    backend.set_optional_feature(feature_name, *kind, revert_to)?;
                    let action_key =
                        format!("WindowsFeature:{:?}|{}", kind, escape_pipe(feature_name));
                    let id = ledger.record_change(
                        &tweak.id,
                        &format!("Revert:{}", action_key),
                        &prev_enabled.to_string(),
                        &revert_to.to_string(),
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
            }
        }

        if reverted == 0 && skipped_power > 0 {
            if let Some(err) = last_power_err {
                return Err(err);
            }
        }

        Ok(reverted)
    })();

    if let Err(err) = res {
        rollback_recorded_ids(recorded_ids, ledger, backend);
        return Err(err);
    }

    for pid in consumed_prior_ids {
        ledger.rollback_entry(pid)?;
    }

    res
}

/// Revert multiple tweaks by ID back to default Windows states using an explicit `SystemBackend`.
/// Sequences revert operations in reverse topological order using `DijkstraScheduler`.
pub fn revert_tweaks_by_id_with(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let _lock = mutation_guard();
    if tweak_ids.is_empty() {
        return Ok(0);
    }

    // Automated pre-flight VSS restore point check (within last 24 hours)
    if let Err(e) = backend.ensure_preflight_restore_point("WinBlast Pre-Optimization Checkpoint") {
        tracing::warn!(
            "Pre-flight system restore point check/creation skipped or failed: {}",
            e
        );
    }

    let scheduler = crate::graph::dijkstra::DijkstraScheduler::new();
    let tweak_tuples = all_tweaks.iter().map(|t| {
        let requires_reboot =
            t.valuation.reversibility == crate::core::risk::Reversibility::RequiresReboot;
        let touches_services = t
            .actions
            .iter()
            .any(|a| matches!(a, TweakAction::ServiceState { .. }));
        (
            t.id.as_str(),
            t.prerequisites.as_slice(),
            requires_reboot,
            touches_services,
        )
    });

    let execution_order = scheduler.schedule_execution(tweak_tuples, tweak_ids);

    let mut total = 0usize;
    let mut reverted_set = std::collections::HashSet::new();

    // Revert in reverse order of application to honor reverse dependencies
    for step in execution_order.steps.iter().rev() {
        if let Some(tweak) = all_tweaks.iter().find(|t| t.id == step.tweak_id) {
            total += revert_tweak_with(tweak, ledger, backend)?;
            reverted_set.insert(step.tweak_id.clone());
        }
    }

    // Revert any unmapped IDs remaining
    for id in tweak_ids {
        if !reverted_set.contains(id) {
            if let Some(tweak) = all_tweaks.iter().find(|t| t.id == *id) {
                total += revert_tweak_with(tweak, ledger, backend)?;
                reverted_set.insert(id.clone());
            }
        }
    }

    Ok(total)
}

/// Roll back a single ledger entry using an explicit `SystemBackend`.
pub fn rollback_entry_with(
    entry: &LedgerEntry,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<()> {
    let _lock = mutation_guard();
    if entry.rolled_back {
        return Ok(());
    }

    let raw_action = entry
        .action_type
        .strip_prefix("Revert:")
        .unwrap_or(&entry.action_type);
    let parts: Vec<&str> = raw_action.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(WinBlastError::invalid_input(
            "Empty action_type in ledger entry",
        ));
    }

    let kind = parts[0];
    let target = parts.get(1).copied().unwrap_or_default();

    match kind {
        "RegistryValue" => {
            let (hive, path, name) = parse_pipe_reg_target(target)?;
            let json = entry.original_value.strip_prefix(PRIOR_PREFIX).ok_or_else(|| {
                WinBlastError::invalid_input(format!(
                    "RegistryValue entry {} has no recorded prior state",
                    entry.id
                ))
            })?;
            let prior: RegValuePrior = serde_json::from_str(json)?;
            match &prior.raw {
                Some(raw) => backend.reg_set_raw(&hive, &path, &name, raw)?,
                None => backend.reg_delete_value_literal(&hive, &path, &name)?,
            }
            if let Some(top) = &prior.created_key {
                delete_created_keys(backend, &hive, &path, top)?;
            }
        }
        "RegistryDword" => {
            let (raw_tgt, vendor_owned) = match target.rsplit_once("|vendor=") {
                Some((left, esc_v)) => (left, Some(unescape_pipe(esc_v))),
                None => (target, None),
            };
            let vendor_ref = vendor_owned.as_deref();
            let (hive, path, name) = if raw_tgt.contains('|') {
                parse_pipe_reg_target(raw_tgt)?
            } else {
                let first_slash = raw_tgt.find('\\').ok_or_else(|| {
                    WinBlastError::invalid_input(format!("Invalid registry target: {}", raw_tgt))
                })?;
                let last_slash = raw_tgt.rfind('\\').ok_or_else(|| {
                    WinBlastError::invalid_input(format!("Invalid registry target: {}", raw_tgt))
                })?;
                if first_slash == last_slash {
                    (
                        raw_tgt[..first_slash].to_string(),
                        String::new(),
                        raw_tgt[first_slash + 1..].to_string(),
                    )
                } else {
                    (
                        raw_tgt[..first_slash].to_string(),
                        raw_tgt[first_slash + 1..last_slash].to_string(),
                        raw_tgt[last_slash + 1..].to_string(),
                    )
                }
            };

            if entry.original_value == ABSENT_SENTINEL {
                backend.reg_delete_value_for_vendor(&hive, &path, &name, vendor_ref)?;
            } else {
                let original_val: u32 = entry.original_value.parse().map_err(|_| {
                    WinBlastError::invalid_input(format!(
                        "Invalid original value: {}",
                        entry.original_value
                    ))
                })?;
                backend.reg_set_dword_for_vendor(&hive, &path, &name, original_val, vendor_ref)?;
            }
        }
        "RegistryString" => {
            let (raw_tgt, vendor_owned) = match target.rsplit_once("|vendor=") {
                Some((left, esc_v)) => (left, Some(unescape_pipe(esc_v))),
                None => (target, None),
            };
            let vendor_ref = vendor_owned.as_deref();
            let (hive, path, name) = parse_pipe_reg_target(raw_tgt)?;
            if entry.original_value == ABSENT_SENTINEL {
                backend.reg_delete_value_for_vendor(&hive, &path, &name, vendor_ref)?;
            } else if let Some(s) = entry.original_value.strip_prefix("expand_sz:") {
                backend.reg_set_string(&hive, &path, &name, s, true, vendor_ref)?;
            } else if let Some(s) = entry.original_value.strip_prefix("sz:") {
                backend.reg_set_string(&hive, &path, &name, s, false, vendor_ref)?;
            } else {
                backend.reg_set_string(
                    &hive,
                    &path,
                    &name,
                    &entry.original_value,
                    false,
                    vendor_ref,
                )?;
            }
        }
        "RegistryDelete" => {
            let (hive, path, name) = parse_pipe_reg_target(target)?;
            if entry.action_type.starts_with("Revert:") && entry.original_value == ABSENT_SENTINEL {
                backend.reg_delete_value(&hive, &path, &name)?;
            } else if entry.original_value != ABSENT_SENTINEL {
                let snap: RegRawSnapshot = serde_json::from_str(&entry.original_value)?;
                backend.reg_set_raw(&hive, &path, &name, &snap)?;
            }
        }
        "RegistryDeleteTree" => {
            let (hive, esc_path) = target.split_once('\\').ok_or_else(|| {
                WinBlastError::invalid_input(format!("Invalid tree target: {}", target))
            })?;
            let path = unescape_pipe(esc_path);
            if entry.action_type.starts_with("Revert:") && entry.original_value == ABSENT_SENTINEL {
                backend.reg_delete_tree(hive, &path)?;
            } else if entry.original_value != ABSENT_SENTINEL {
                let tree: RegTreeSnapshot = serde_json::from_str(&entry.original_value)?;
                backend.reg_restore_tree(hive, &path, &tree)?;
            }
        }
        "RegistryBinaryBit" => {
            let segs: Vec<&str> = target.split('|').collect();
            if segs.len() == 3 {
                let base = format!("{}|{}", segs[0], segs[1]);
                let (hive, path, name) = parse_pipe_reg_target(&base)?;
                let byte_index: usize = segs[2].parse().map_err(|_| {
                    WinBlastError::invalid_input(format!(
                        "Invalid byte_index in RegistryBinaryBit target: {}",
                        target
                    ))
                })?;
                if entry.original_value == ABSENT_SENTINEL {
                    backend.reg_delete_value(&hive, &path, &name)?;
                } else if let Some((mask_s, bits_s)) = entry.original_value.split_once('|') {
                    let bit_mask: u8 = mask_s.parse().map_err(|_| {
                        WinBlastError::TweakFailed(format!(
                            "Invalid bit_mask in RegistryBinaryBit: {}",
                            entry.original_value
                        ))
                    })?;
                    let orig_bits: u8 = bits_s.parse().map_err(|_| {
                        WinBlastError::TweakFailed(format!(
                            "Invalid orig_bits in RegistryBinaryBit: {}",
                            entry.original_value
                        ))
                    })?;
                    let Some(mut live_bytes) = backend.reg_get_binary(&hive, &path, &name) else {
                        // Tolerate out-of-order Backup Center rollback when another entry already deleted the value
                        tracing::warn!(
                            "Binary value {}\\{}\\{} already absent during bit rollback; skipping bit restore",
                            hive,
                            path,
                            name
                        );
                        ledger.rollback_entry(entry.id)?;
                        return Ok(());
                    };
                    if live_bytes.len() <= byte_index {
                        return Err(WinBlastError::TweakFailed(format!(
                            "Binary value {}\\{}\\{} shorter than byte_index {} (len={})",
                            hive,
                            path,
                            name,
                            byte_index,
                            live_bytes.len()
                        )));
                    }
                    live_bytes[byte_index] =
                        (live_bytes[byte_index] & !bit_mask) | (orig_bits & bit_mask);
                    backend.reg_set_binary(&hive, &path, &name, &live_bytes)?;
                } else if let Some(hex) = entry.original_value.strip_prefix("hex:") {
                    let orig_bytes = decode_hex(hex)?;
                    backend.reg_set_binary(&hive, &path, &name, &orig_bytes)?;
                } else {
                    return Err(WinBlastError::TweakFailed(format!(
                        "Malformed binary rollback payload: {}",
                        entry.original_value
                    )));
                }
            } else {
                let (hive, path, name) = parse_pipe_reg_target(target)?;
                if entry.original_value == ABSENT_SENTINEL {
                    backend.reg_delete_value(&hive, &path, &name)?;
                } else if let Some(hex) = entry.original_value.strip_prefix("hex:") {
                    let orig_bytes = decode_hex(hex)?;
                    backend.reg_set_binary(&hive, &path, &name, &orig_bytes)?;
                } else {
                    return Err(WinBlastError::TweakFailed(format!(
                        "Malformed binary rollback payload: {}",
                        entry.original_value
                    )));
                }
            }
        }
        "CompositeStringKey" => {
            // Format: {hive}\{esc_path}|{esc_name}#{esc_entry_key}|sep={...}|kv={...}|trail={0|1}[|vendor={...}]
            let parts_meta: Vec<&str> = target.split('|').collect();
            if parts_meta.len() < 5 {
                return Err(WinBlastError::TweakFailed(format!(
                    "Malformed CompositeStringKey target: {}",
                    target
                )));
            }
            let pipe_left = format!("{}|{}", parts_meta[0], parts_meta[1]);
            let (base_pipe, esc_entry) = pipe_left.split_once('#').ok_or_else(|| {
                WinBlastError::TweakFailed(format!("Missing composite entry key in {}", target))
            })?;
            let (hive, path, name) = parse_pipe_reg_target(base_pipe)?;
            let entry_key = unescape_pipe(esc_entry);
            let pair_sep = parts_meta[2]
                .strip_prefix("sep=")
                .map(unescape_pipe)
                .unwrap_or_else(|| ";".to_string());
            let kv_sep = parts_meta[3]
                .strip_prefix("kv=")
                .map(unescape_pipe)
                .unwrap_or_else(|| "=".to_string());
            let trail = parts_meta[4].strip_prefix("trail=") == Some("1");
            let vendor_owned = parts_meta[5..]
                .iter()
                .find_map(|p| p.strip_prefix("vendor=").map(unescape_pipe));
            let vendor_ref = vendor_owned.as_deref();

            let live = backend.reg_get_string(&hive, &path, &name, vendor_ref);
            let restore_val = if entry.original_value == VAL_ABSENT_SENTINEL
                || entry.original_value == SUBKEY_ABSENT_SENTINEL
            {
                ABSENT_SENTINEL
            } else if let Some(v) = entry.original_value.strip_prefix("entry:") {
                v
            } else {
                &entry.original_value
            };
            let (merged, _) = merge_composite_pair(
                live.as_ref().map(|s| s.value.as_str()),
                &entry_key,
                restore_val,
                &pair_sep,
                &kv_sep,
                trail,
            );
            if merged.is_empty() && entry.original_value == VAL_ABSENT_SENTINEL {
                backend.reg_delete_value_for_vendor(&hive, &path, &name, vendor_ref)?;
            } else {
                backend.reg_set_string(&hive, &path, &name, &merged, false, vendor_ref)?;
            }
        }
        "RegistryDynamicDword" => {
            let (hive, path, name) = parse_pipe_reg_target(target)?;
            if entry.original_value == ABSENT_SENTINEL {
                backend.reg_delete_value(&hive, &path, &name)?;
            } else {
                let original_val: u32 = entry.original_value.parse().map_err(|_| {
                    WinBlastError::TweakFailed(format!(
                        "Invalid original value: {}",
                        entry.original_value
                    ))
                })?;
                backend.reg_set_dword(&hive, &path, &name, original_val)?;
            }
        }
        "PowerCfgSetting" => {
            let mut segs = target.splitn(3, '|');
            let scheme = segs.next().unwrap_or_default();
            let subgroup = segs.next().unwrap_or_default();
            let setting = segs.next().unwrap_or_default();
            if scheme.is_empty() || subgroup.is_empty() || setting.is_empty() {
                return Err(WinBlastError::TweakFailed(format!(
                    "Invalid PowerCfg target: {}",
                    target
                )));
            }
            let (ac, dc) = parse_power_indices(&entry.original_value);
            if ac.is_none() && dc.is_none() {
                return Err(WinBlastError::Rollback(format!(
                    "Malformed PowerCfgSetting original_value '{}'",
                    entry.original_value
                )));
            }
            backend.write_power_setting_index(Some(scheme), subgroup, setting, ac, dc, false)?;
        }
        "WindowsFeature" => {
            let (feat_name, feat_kind) = if let Some((kind_s, esc_name)) = target.split_once('|') {
                (unescape_pipe(esc_name), parse_feature_kind(kind_s))
            } else if let Some((name_s, kind_s)) = target.rsplit_once(':') {
                (name_s.to_string(), parse_feature_kind(kind_s))
            } else {
                return Err(WinBlastError::Rollback(format!(
                    "Malformed WindowsFeature rollback target '{}'",
                    target
                )));
            };
            let orig_en = if let Ok(b) = entry.original_value.parse::<bool>() {
                b
            } else if entry.original_value.eq_ignore_ascii_case("Enabled")
                || entry.original_value.eq_ignore_ascii_case("EnablePending")
            {
                true
            } else if entry.original_value.eq_ignore_ascii_case("Disabled")
                || entry.original_value.eq_ignore_ascii_case("DisablePending")
            {
                false
            } else {
                return Err(WinBlastError::Rollback(format!(
                    "Malformed WindowsFeature original_value '{}'",
                    entry.original_value
                )));
            };
            backend.set_optional_feature(&feat_name, feat_kind, orig_en)?;
        }
        "ServiceState" => {
            let service_name = target;
            if entry.original_value != ABSENT_SENTINEL {
                let (start_str, was_delayed) =
                    if let Some((s, d)) = entry.original_value.split_once("|delayed=") {
                        (s.trim(), d.trim() == "1")
                    } else {
                        (entry.original_value.trim(), false)
                    };
                let original_start: u32 = start_str.parse().map_err(|_| {
                    WinBlastError::TweakFailed(format!(
                        "Invalid original service start: {}",
                        entry.original_value
                    ))
                })?;
                if was_delayed && original_start == 2 {
                    let svc_path = format!(
                        "SYSTEM\\CurrentControlSet\\Services\\{}",
                        service_name.trim()
                    );
                    if let Err(e) = backend.reg_set_dword("HKLM", &svc_path, "DelayedAutostart", 1)
                    {
                        tracing::warn!(
                            "Failed to restore DelayedAutostart for {}: {}",
                            service_name,
                            e
                        );
                    }
                }
                backend.set_service_start_type(service_name, original_start)?;
            }
        }
        "ScheduledTask" => {
            let task_path = target;
            if entry.original_value != ABSENT_SENTINEL {
                let original_enabled = if let Ok(b) = entry.original_value.parse::<bool>() {
                    b
                } else if entry.original_value.eq_ignore_ascii_case("enabled") {
                    true
                } else if entry.original_value.eq_ignore_ascii_case("disabled") {
                    false
                } else {
                    return Err(WinBlastError::Rollback(format!(
                        "Malformed ScheduledTask original_value '{}'",
                        entry.original_value
                    )));
                };
                backend.set_scheduled_task_enabled(task_path, original_enabled)?;
            }
        }
        "AppXPackage" => {
            tracing::warn!("{} '{}' cannot be automatically restored.", kind, target);
        }
        "CleanTargets" | "DiskCleanup" | "AppXBatch" | "App" => {
            return Err(WinBlastError::Rollback(format!(
                "{} '{}' is non-reversible",
                kind, target
            )));
        }
        "StartupApproved" => {
            let entry_id = unescape_pipe(target);
            if entry_id.is_empty() {
                return Err(WinBlastError::Rollback(
                    "Missing startup entry ID".to_string(),
                ));
            }
            let orig_en = if let Ok(b) = entry.original_value.parse::<bool>() {
                b
            } else if entry.original_value.eq_ignore_ascii_case("enabled") {
                true
            } else if entry.original_value.eq_ignore_ascii_case("disabled") {
                false
            } else {
                return Err(WinBlastError::Rollback(format!(
                    "Malformed StartupApproved original_value '{}'",
                    entry.original_value
                )));
            };
            backend.set_startup_entry_enabled_by_id(&entry_id, orig_en)?;
        }
        "Startup" => {
            return Err(WinBlastError::Rollback(format!(
                "Legacy startup entry '{}' cannot be rolled back without an entry ID",
                target
            )));
        }
        "PowerCfg" => match target {
            "ActiveScheme" | "UltimateScheme" => {
                let prev_guid = entry.original_value.trim();
                if prev_guid.is_empty() {
                    return Err(WinBlastError::Rollback(
                        "Missing previous power scheme GUID".to_string(),
                    ));
                }
                backend.set_active_power_scheme(prev_guid)?;
            }
            "Hibernation" => {
                let orig_en = entry.original_value.eq_ignore_ascii_case("enabled")
                    || entry.original_value.eq_ignore_ascii_case("true");
                backend.set_hibernation_enabled(orig_en)?;
            }
            "UsbPowerSaving" => {
                let orig_en = entry.original_value.eq_ignore_ascii_case("enabled")
                    || entry.original_value.eq_ignore_ascii_case("true");
                backend.set_usb_power_saving_enabled(orig_en)?;
            }
            _ => {
                return Err(WinBlastError::Rollback(format!(
                    "Unsupported PowerCfg rollback target '{}'",
                    target
                )));
            }
        },
        "HostsBlock" | "GpuHags" | "GpuMpo" | "GpuMsi" | "CoreParking" => match kind {
            "HostsBlock" => match entry.new_value.strip_prefix("hosts:") {
                Some(list) => {
                    let hosts: Vec<String> = list
                        .split(',')
                        .map(str::trim)
                        .filter(|h| !h.is_empty())
                        .map(String::from)
                        .collect();
                    if let Some(existing) = backend.read_hosts_file() {
                        let (updated, changed) =
                            crate::executor::ps_runner::unblock_hosts_in_content(&existing, &hosts);
                        if changed {
                            backend.write_hosts_file(&updated)?;
                        }
                    }
                }
                None => {
                    crate::executor::ps_runner::unblock_telemetry_hosts_with(backend)?;
                }
            },
            "GpuHags" => {
                let orig_hags = entry.original_value == "true";
                backend.set_hags(orig_hags).map_err(WinBlastError::Io)?;
            }
            "GpuMpo" => {
                let orig_mpo = entry.original_value == "true";
                backend
                    .set_mpo_disabled(orig_mpo)
                    .map_err(WinBlastError::Io)?;
            }
            "GpuMsi" => {
                if target.is_empty() {
                    return Err(WinBlastError::Rollback(
                        "Missing GPU PnP device ID".to_string(),
                    ));
                }
                let (msi, prio) = if let Some((msi_s, prio_s)) =
                    entry.original_value.split_once('|')
                {
                    (
                        msi_s.eq_ignore_ascii_case("true") || msi_s.eq_ignore_ascii_case("enabled"),
                        prio_s.parse().unwrap_or(2),
                    )
                } else {
                    (
                        entry.original_value.eq_ignore_ascii_case("enabled")
                            || entry.original_value.eq_ignore_ascii_case("true"),
                        2,
                    )
                };
                backend
                    .set_gpu_msi_mode(target, msi, prio)
                    .map_err(WinBlastError::Io)?;
            }
            "CoreParking" => {
                if let Some(snap) =
                    crate::hardware::parking::ParkingSnapshot::deserialize(&entry.original_value)
                {
                    backend.apply_parking_snapshot(&snap)?;
                } else if entry.original_value == "unparked" {
                    backend.unpark_all_cores()?;
                } else {
                    backend.restore_stock_parking()?;
                }
            }
            _ => {}
        },
        _ => {
            return Err(WinBlastError::TweakFailed(format!(
                "Unknown action type in ledger: {}",
                kind
            )));
        }
    }

    ledger.rollback_entry(entry.id)?;
    Ok(())
}

// ── Public Backward-Compatible Wrappers (Delegate to `RealBackend`) ──────────

#[allow(dead_code)]
pub fn apply_tweak(tweak: &TweakDefinition, ledger: &StateLedger) -> Result<usize> {
    apply_tweak_with(tweak, ledger, &RealBackend)
}

#[allow(dead_code)]
pub fn apply_tweaks_by_id(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
) -> Result<usize> {
    apply_tweaks_by_id_with(tweak_ids, all_tweaks, ledger, &RealBackend)
}

#[allow(dead_code)]
pub fn revert_tweak(tweak: &TweakDefinition, ledger: &StateLedger) -> Result<usize> {
    revert_tweak_with(tweak, ledger, &RealBackend)
}

#[allow(dead_code)]
pub fn revert_tweaks_by_id(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
) -> Result<usize> {
    revert_tweaks_by_id_with(tweak_ids, all_tweaks, ledger, &RealBackend)
}

#[allow(dead_code)]
pub fn rollback_entry(entry: &LedgerEntry, ledger: &StateLedger) -> Result<()> {
    rollback_entry_with(entry, ledger, &RealBackend)
}

#[allow(dead_code)]
pub fn rollback_by_id(entry_id: i64, ledger: &StateLedger) -> Result<()> {
    let history = ledger.get_history()?;
    if let Some(entry) = history.iter().find(|e| e.id == entry_id) {
        rollback_entry_with(entry, ledger, &RealBackend)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::backend::SandboxEnv;
    use crate::core::risk::{OsRequirement, Reversibility, RiskLevel, RiskValuation};

    #[test]
    fn test_rollback_revert_prefix_in_sandbox() {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("Failed to create memory ledger");

        let _entry_id = ledger
            .record_change(
                "test_tweak",
                "Revert:RegistryDword:HKCU\\Software\\WinBlast0_Test\\TestVal",
                "1",
                "0",
            )
            .expect("Failed to record change");

        let history = ledger.get_history().expect("Failed to get history");
        assert_eq!(history.len(), 1);

        rollback_entry_with(&history[0], &ledger, &sbx).expect("Failed to roll back revert entry");

        let updated = ledger.get_history().expect("Failed to get updated history");
        assert!(updated[0].rolled_back);
        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TestVal"),
            Some(1)
        );
    }

    #[test]
    fn test_absent_key_rolls_back_by_deleting() {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("Failed to create memory ledger");
        let tweak = TweakDefinition {
            id: "test_absent".into(),
            title: "Test".into(),
            category: "OS Settings".into(),
            description: "Desc".into(),
            easy_mode_included: true,
            os_req: crate::core::risk::OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec![],
            },
            valuation: crate::core::risk::RiskValuation {
                stability_risk: crate::core::risk::RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 0,
                reversibility: crate::core::risk::Reversibility::Instant,
                broken_features: vec![],
            },
            actions: vec![TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: "Software\\WinBlast0_Test".into(),
                name: "TransientKey".into(),
                optimized_value: 1,
                default_value: 0,
            }],
            gpu_vendor: None,
            sub_category: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        };

        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"),
            None
        );
        apply_tweak_with(&tweak, &ledger, &sbx).unwrap();
        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"),
            Some(1)
        );

        let hist = ledger.get_history().unwrap();
        rollback_entry_with(&hist[0], &ledger, &sbx).unwrap();
        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"),
            None
        );
    }

    #[test]
    fn test_batch1_engine_edge_cases_and_atomic_revert() {
        use crate::core::risk::{
            OptionalFeatureKind, OsRequirement, Reversibility, RiskLevel, RiskValuation,
        };

        let mk_tweak = |id: &str,
                        min_build: u32,
                        gpu_vendor: Option<&str>,
                        actions: Vec<TweakAction>| TweakDefinition {
            id: id.into(),
            title: id.into(),
            category: "OS Settings".into(),
            description: String::new(),
            easy_mode_included: false,
            os_req: OsRequirement {
                min_build,
                max_build: None,
                editions: vec![],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 0,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions,
            gpu_vendor: gpu_vendor.map(str::to_string),
            sub_category: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        };

        // 1. OS + GPU vendor guard on apply/revert
        let sbx = SandboxEnv::new();
        {
            let mut st = sbx.inner.write().unwrap();
            st.os_info.build_number = 19041;
            st.gpus
                .retain(|g| g.vendor == crate::hardware::gpu::GpuVendor::Nvidia);
        }
        let ledger = StateLedger::in_memory().unwrap();
        let win11_tweak = mk_tweak(
            "win11_only",
            22621,
            None,
            vec![TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: "Software\\WinBlast0_Test".into(),
                name: "Val".into(),
                optimized_value: 1,
                default_value: 0,
            }],
        );
        assert!(matches!(
            apply_tweak_with(&win11_tweak, &ledger, &sbx),
            Err(WinBlastError::OsIncompatible(_))
        ));
        let amd_tweak = mk_tweak(
            "amd_only",
            19041,
            Some("AMD"),
            vec![TweakAction::RegistryDword {
                hive: "HKLM".into(),
                path: "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000".into(),
                name: "EnableUlps".into(),
                optimized_value: 0,
                default_value: 1,
            }],
        );
        assert!(matches!(
            revert_tweak_with(&amd_tweak, &ledger, &sbx),
            Err(WinBlastError::Hardware(_))
        ));

        // 2. Atomic reverse rollback on mid-revert_tweak_with failure
        let rev_fail_tweak = mk_tweak(
            "rev_atomic",
            19041,
            None,
            vec![
                TweakAction::RegistryDword {
                    hive: "HKCU".into(),
                    path: "Software\\WinBlast0_Test".into(),
                    name: "Step0".into(),
                    optimized_value: 1,
                    default_value: 0,
                },
                TweakAction::WindowsFeature {
                    feature_name: "NonExistentFeature".into(),
                    kind: OptionalFeatureKind::OptionalFeature,
                    enable: false,
                },
            ],
        );
        sbx.reg_set_dword("HKCU", "Software\\WinBlast0_Test", "Step0", 1)
            .unwrap();
        assert!(revert_tweak_with(&rev_fail_tweak, &ledger, &sbx).is_err());
        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "Step0"),
            Some(1)
        );
        assert!(ledger.get_history().unwrap().iter().all(|e| e.rolled_back));

        // 3. RegistryDelete revert marks consumed prior snapshot rolled_back = 1
        let del_tweak = mk_tweak(
            "del_consumed",
            19041,
            None,
            vec![TweakAction::RegistryDelete {
                hive: "HKCU".into(),
                path: "Software\\WinBlast0_Test".into(),
                name: "DelMe".into(),
            }],
        );
        sbx.reg_set_raw(
            "HKCU",
            "Software\\WinBlast0_Test",
            "DelMe",
            &RegRawSnapshot {
                reg_type: 4,
                hex_bytes: "05000000".into(),
            },
        )
        .unwrap();
        apply_tweak_with(&del_tweak, &ledger, &sbx).unwrap();
        revert_tweak_with(&del_tweak, &ledger, &sbx).unwrap();
        let del_prior = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.tweak_id == "del_consumed" && !e.action_type.starts_with("Revert:"))
            .unwrap();
        assert!(del_prior.rolled_back);

        // 4. RegistryBinaryBit: revert seeds from default_full_hex when absent, and out-of-order rollback tolerates __ABSENT__
        let bin_tweak = mk_tweak(
            "bin_seed_rev",
            19041,
            None,
            vec![
                TweakAction::RegistryBinaryBit {
                    hive: "HKCU".into(),
                    path: "Software\\WinBlast0_Test".into(),
                    name: "MaskVal".into(),
                    byte_index: 0,
                    min_length: 4,
                    bit_mask: 0x02,
                    optimized_bits: 0x00,
                    default_bits: 0x02,
                    create_if_missing: true,
                    default_full_hex: Some("9e3e0780".into()),
                },
                TweakAction::RegistryBinaryBit {
                    hive: "HKCU".into(),
                    path: "Software\\WinBlast0_Test".into(),
                    name: "MaskVal".into(),
                    byte_index: 0,
                    min_length: 4,
                    bit_mask: 0x04,
                    optimized_bits: 0x00,
                    default_bits: 0x04,
                    create_if_missing: true,
                    default_full_hex: Some("9e3e0780".into()),
                },
            ],
        );
        revert_tweak_with(&bin_tweak, &ledger, &sbx).unwrap();
        assert_eq!(
            sbx.reg_get_binary("HKCU", "Software\\WinBlast0_Test", "MaskVal")
                .unwrap()[0],
            0x9e
        );
        // Now delete MaskVal via rolling back first entry (__ABSENT__), then roll back second entry out-of-order
        let bin_entries: Vec<_> = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .filter(|e| e.tweak_id == "bin_seed_rev" && !e.rolled_back)
            .collect();
        // Reverse order (oldest first -> original_value == __ABSENT__ deletes MaskVal)
        rollback_entry_with(&bin_entries[1], &ledger, &sbx).unwrap();
        assert!(sbx
            .reg_get_binary("HKCU", "Software\\WinBlast0_Test", "MaskVal")
            .is_none());
        rollback_entry_with(&bin_entries[0], &ledger, &sbx).unwrap();

        // 5. CompositeStringKey with `#` in path/name/entry_key + existing empty "" preservation + pipe RegistryDword
        sbx.reg_set_string(
            "HKCU",
            "Software\\Test#Hash",
            "Val#Pipe|Name",
            "",
            false,
            None,
        )
        .unwrap();
        let comp_tweak = mk_tweak(
            "comp_hash_empty",
            19041,
            None,
            vec![
                TweakAction::CompositeStringKey {
                    hive: "HKCU".into(),
                    path: "Software\\Test#Hash".into(),
                    name: "Val#Pipe|Name".into(),
                    entry_key: "Key#1".into(),
                    optimized_entry_value: "1".into(),
                    default_entry_value: "0".into(),
                    pair_separator: ";".into(),
                    kv_separator: "=".into(),
                    trailing_separator: true,
                },
                TweakAction::RegistryDword {
                    hive: "HKCU".into(),
                    path: "Software\\Test#Hash".into(),
                    name: "Pipe|Dword#Name".into(),
                    optimized_value: 7,
                    default_value: 3,
                },
            ],
        );
        sbx.reg_set_dword("HKCU", "Software\\Test#Hash", "Pipe|Dword#Name", 3)
            .unwrap();
        apply_tweak_with(&comp_tweak, &ledger, &sbx).unwrap();
        for e in ledger
            .get_history()
            .unwrap()
            .into_iter()
            .filter(|e| e.tweak_id == "comp_hash_empty" && !e.rolled_back)
        {
            rollback_entry_with(&e, &ledger, &sbx).unwrap();
        }
        assert_eq!(
            sbx.reg_get_string("HKCU", "Software\\Test#Hash", "Val#Pipe|Name", None)
                .map(|s| s.value),
            Some(String::new())
        );
        assert_eq!(
            sbx.reg_get_dword("HKCU", "Software\\Test#Hash", "Pipe|Dword#Name"),
            Some(3)
        );

        // 6. Malformed PowerCfgSetting original_value must error and stay un-rolled-back
        let bad_pwr_id = ledger
            .record_change(
                "bad_pwr",
                "PowerCfgSetting:381b4222-f694-41f0-9685-ff5bb260df2e|54533251-82be-4824-96c1-47b60b740d00|0cc5b647-c1df-4637-891a-dec35c318583",
                "corrupted",
                "ac=100",
            )
            .unwrap();
        let bad_pwr_entry = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.id == bad_pwr_id)
            .unwrap();
        assert!(rollback_entry_with(&bad_pwr_entry, &ledger, &sbx).is_err());
        assert!(
            !ledger
                .get_history()
                .unwrap()
                .into_iter()
                .find(|e| e.id == bad_pwr_id)
                .unwrap()
                .rolled_back
        );

        // 7. DelayedAutostart snapshot & restore via ServiceState
        {
            let mut st = sbx.inner.write().unwrap();
            st.services.insert("mapsbroker".into(), 2);
            st.registry.insert(
                (
                    "HKLM".into(),
                    "system\\currentcontrolset\\services\\mapsbroker".into(),
                    "delayedautostart".into(),
                ),
                1,
            );
        }
        let svc_tweak = mk_tweak(
            "svc_delayed",
            19041,
            None,
            vec![TweakAction::ServiceState {
                service_name: "MapsBroker".into(),
                target_start_type: 4,
                default_start_type: 2,
            }],
        );
        apply_tweak_with(&svc_tweak, &ledger, &sbx).unwrap();
        let svc_entry = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.tweak_id == "svc_delayed")
            .unwrap();
        assert_eq!(svc_entry.original_value, "2|delayed=1");
        sbx.reg_delete_value(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services\\MapsBroker",
            "DelayedAutostart",
        )
        .unwrap();
        rollback_entry_with(&svc_entry, &ledger, &sbx).unwrap();
        assert_eq!(sbx.get_service_start_type("MapsBroker"), Some(2));
        assert_eq!(
            sbx.reg_get_dword(
                "HKLM",
                "SYSTEM\\CurrentControlSet\\Services\\MapsBroker",
                "DelayedAutostart"
            ),
            Some(1)
        );
    }

    #[test]
    fn test_core_parking_snapshot_and_legacy_rollback() {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("Failed to create memory ledger");

        // 1. Exact ParkingSnapshot JSON rollback
        let orig_snap = crate::hardware::parking::ParkingSnapshot {
            min_ac: 0,
            min_dc: 100,
            max_ac: 75,
            max_dc: 50,
            min1: Some((25, 10)),
            max1: Some((100, 100)),
        };
        sbx.unpark_all_cores().unwrap();
        assert!(sbx.detect_core_parking().is_fully_unparked);

        let snap_id = ledger
            .record_change(
                "cpu_unpark_all",
                "CoreParking:AllLogicalCores",
                &orig_snap.serialize(),
                "unparked",
            )
            .unwrap();
        let snap_entry = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.id == snap_id)
            .unwrap();
        rollback_entry_with(&snap_entry, &ledger, &sbx).unwrap();

        let cp_after = sbx.detect_core_parking();
        assert_eq!(cp_after.min_cores_ac_pct, 0);
        assert_eq!(cp_after.min_cores_dc_pct, 100);
        assert_eq!(cp_after.max_cores_ac_pct, 75);
        assert_eq!(cp_after.max_cores_dc_pct, 50);
        assert_eq!(cp_after.pcore_min_ac_pct, Some(25));
        assert_eq!(cp_after.pcore_min_dc_pct, Some(10));
        assert!(!cp_after.is_fully_unparked);

        // 2. Legacy string fallback ("unparked" / "parked")
        let legacy_unpark_id = ledger
            .record_change(
                "cpu_parking_stock",
                "CoreParking:AllLogicalCores",
                "unparked",
                "parked",
            )
            .unwrap();
        let legacy_unpark_entry = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.id == legacy_unpark_id)
            .unwrap();
        rollback_entry_with(&legacy_unpark_entry, &ledger, &sbx).unwrap();
        assert!(sbx.detect_core_parking().is_fully_unparked);

        let legacy_park_id = ledger
            .record_change(
                "cpu_unpark_all",
                "CoreParking:AllLogicalCores",
                "parked",
                "unparked",
            )
            .unwrap();
        let legacy_park_entry = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .find(|e| e.id == legacy_park_id)
            .unwrap();
        rollback_entry_with(&legacy_park_entry, &ledger, &sbx).unwrap();
        let cp_stock = sbx.detect_core_parking();
        assert!(!cp_stock.is_fully_unparked);
        assert_eq!(cp_stock.min_cores_ac_pct, 10);
        assert_eq!(cp_stock.min_cores_dc_pct, 5);
    }

    #[test]
    fn test_apply_tweak_fallback_execution() {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("Failed to create memory ledger");

        // Primary action attempts to update a non-existent binary key with create_if_missing: false (fails)
        let primary_failing_action = TweakAction::RegistryBinaryBit {
            hive: "HKLM".to_string(),
            path: r"SOFTWARE\WinBlastTest\NonExistent".to_string(),
            name: "NonExistentBinary".to_string(),
            byte_index: 0,
            min_length: 4,
            bit_mask: 0x01,
            optimized_bits: 0x01,
            default_bits: 0x00,
            create_if_missing: false,
            default_full_hex: None,
        };

        // Fallback strategy attempts standard DWORD registry write (succeeds)
        let fallback_working_action = TweakAction::RegistryDword {
            hive: "HKLM".to_string(),
            path: r"SOFTWARE\WinBlastTest\FallbackWorking".to_string(),
            name: "FallbackApplied".to_string(),
            optimized_value: 42,
            default_value: 0,
        };

        let tweak = TweakDefinition {
            id: "test_fallback_tweak".to_string(),
            title: "Test Fallback Tweak".to_string(),
            category: "Testing".to_string(),
            sub_category: None,
            description: "Verifies fallback execution when primary strategy fails".to_string(),
            easy_mode_included: false,
            os_req: OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec![],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 0,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions: vec![primary_failing_action],
            gpu_vendor: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: vec![vec![fallback_working_action]],
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        };

        // Execution should fail primary, attempt fallback #1, and succeed!
        let applied_count = apply_tweak_with(&tweak, &ledger, &sbx)
            .expect("Tweak application should succeed via fallback strategy");
        assert_eq!(applied_count, 1);

        // Verify fallback action took effect in sandbox backend
        let val = sbx.reg_get_dword_for_vendor(
            "HKLM",
            r"SOFTWARE\WinBlastTest\FallbackWorking",
            "FallbackApplied",
            None,
        );
        assert_eq!(val, Some(42));

        // Verify ledger recorded the fallback mutation
        let history = ledger.get_history().expect("History fetch failed");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].tweak_id, "test_fallback_tweak");
        assert_eq!(history[0].new_value, "42");
    }
}
