//! Core execution engine — applies tweaks atomically with ledger tracking,
//! handles missing pre-mutation sentinel rollback (`__ABSENT__`), and supports `SystemBackend`.

use crate::backup::state_ledger::{LedgerEntry, StateLedger};
use crate::core::backend::{is_unsupported_power_guid_error, RealBackend, SystemBackend};
use crate::core::error::{Result, WinBlastError};
use crate::core::reg_helpers::{
    decode_hex, escape_pipe, format_pipe_reg_target, merge_composite_pair,
    parse_pipe_reg_target, unescape_pipe, RegRawSnapshot, RegTreeSnapshot,
};
use crate::core::risk::{OptionalFeatureKind, PowerTargetScope, TweakAction, TweakDefinition};

/// Sentinel persisted in `StateLedger` when a registry value, absent service, or absent task
/// did not exist before mutation so rollback cleans up accurately instead of inventing keys/crashing.
pub const ABSENT_SENTINEL: &str = "__ABSENT__";
const VAL_ABSENT_SENTINEL: &str = "__VAL_ABSENT__";
const SUBKEY_ABSENT_SENTINEL: &str = "__KEY_ABSENT__";

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
        let vendors: Vec<_> = backend.detect_gpus().into_iter().map(|g| g.vendor).collect();
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

fn rollback_recorded_ids(recorded_ids: Vec<i64>, ledger: &StateLedger, backend: &dyn SystemBackend) {
    if recorded_ids.is_empty() {
        return;
    }
    if let Ok(history) = ledger.get_history() {
        for id in recorded_ids.into_iter().rev() {
            if let Some(entry) = history.iter().find(|e| e.id == id && !e.rolled_back) {
                let _ = rollback_entry_with(entry, ledger, backend);
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

fn format_dword_action_key(prefix: &str, hive: &str, path: &str, name: &str, vendor_ref: Option<&str>) -> String {
    let base_tgt = format_pipe_reg_target(hive, path, name);
    match vendor_ref {
        Some(v) if !v.trim().is_empty() => format!("{}RegistryDword:{}|vendor={}", prefix, base_tgt, escape_pipe(v)),
        _ => format!("{}RegistryDword:{}", prefix, base_tgt),
    }
}

/// Apply a single tweak definition using an explicit `SystemBackend`, recording exact pre-change
/// states in the ledger. If action `k > 0` fails, already-committed actions `0..k` are rolled back.
pub fn apply_tweak_with(
    tweak: &TweakDefinition,
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    ensure_tweak_compatible(tweak, backend)?;
    let mut recorded_ids: Vec<i64> = Vec::with_capacity(tweak.actions.len());
    let vendor_ref = tweak.gpu_vendor.as_deref();

    let res: Result<usize> = (|| {
        let mut applied = 0usize;
        let mut skipped_power = 0usize;
        let mut last_power_err: Option<WinBlastError> = None;
        for action in &tweak.actions {
            match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    optimized_value,
                    ..
                } => {
                    let prev = backend.reg_get_dword_for_vendor(hive, path, name, vendor_ref);
                    backend.reg_set_dword_for_vendor(hive, path, name, *optimized_value, vendor_ref)?;
                    let action_key = format_dword_action_key("", hive, path, name, vendor_ref);
                    let orig_str = prev
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    let id = ledger.record_change(
                        &tweak.id,
                        &action_key,
                        &orig_str,
                        &optimized_value.to_string(),
                    )?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    optimized_value,
                    expand_sz,
                    ..
                } => {
                    let prev = backend.reg_get_string(hive, path, name, vendor_ref);
                    let orig_str = match &prev {
                        None => ABSENT_SENTINEL.to_string(),
                        Some(s) if s.expand_sz => format!("expand_sz:{}", s.value),
                        Some(s) => format!("sz:{}", s.value),
                    };
                    let new_str = if optimized_value == ABSENT_SENTINEL {
                        backend.reg_delete_value_for_vendor(hive, path, name, vendor_ref)?;
                        ABSENT_SENTINEL.to_string()
                    } else {
                        backend.reg_set_string(hive, path, name, optimized_value, *expand_sz, vendor_ref)?;
                        if *expand_sz {
                            format!("expand_sz:{}", optimized_value)
                        } else {
                            format!("sz:{}", optimized_value)
                        }
                    };
                    let base_tgt = format_pipe_reg_target(hive, path, name);
                    let action_key = match vendor_ref {
                        Some(v) if !v.trim().is_empty() => format!("RegistryString:{}|vendor={}", base_tgt, escape_pipe(v)),
                        _ => format!("RegistryString:{}", base_tgt),
                    };
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, &new_str)?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::RegistryDelete { hive, path, name } => {
                    let prev = backend.reg_get_raw(hive, path, name);
                    backend.reg_delete_value(hive, path, name)?;
                    let orig_str = match prev {
                        Some(snap) => serde_json::to_string(&snap)?,
                        None => ABSENT_SENTINEL.to_string(),
                    };
                    let action_key = format!("RegistryDelete:{}", format_pipe_reg_target(hive, path, name));
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, ABSENT_SENTINEL)?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::RegistryDeleteTree { hive, path } => {
                    let prev = backend.reg_snapshot_tree(hive, path)?;
                    backend.reg_delete_tree(hive, path)?;
                    let orig_str = match prev {
                        Some(tree) => serde_json::to_string(&tree)?,
                        None => ABSENT_SENTINEL.to_string(),
                    };
                    let action_key = format!("RegistryDeleteTree:{}\\{}", hive.trim(), escape_pipe(path));
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, ABSENT_SENTINEL)?;
                    recorded_ids.push(id);
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
                                    hive, path, name, req, b.len()
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
                                        return Err(WinBlastError::TweakFailed(
                                            "default_full_hex shorter than min_length".to_string(),
                                        ));
                                    }
                                    (ABSENT_SENTINEL.to_string(), decoded)
                                } else {
                                    return Err(WinBlastError::TweakFailed(
                                        "Missing default_full_hex template for absent binary value".to_string(),
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
                    bytes[*byte_index] = (bytes[*byte_index] & !bit_mask) | (optimized_bits & bit_mask);
                    let new_masked = format!("{}|{}", bit_mask, bytes[*byte_index] & *bit_mask);
                    backend.reg_set_binary(hive, path, name, &bytes)?;
                    let action_key = format!(
                        "RegistryBinaryBit:{}|{}",
                        format_pipe_reg_target(hive, path, name),
                        byte_index
                    );
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_masked, &new_masked)?;
                    recorded_ids.push(id);
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
                    if merged.is_empty() && optimized_entry_value == ABSENT_SENTINEL {
                        backend.reg_delete_value_for_vendor(hive, path, name, vendor_ref)?;
                    } else {
                        backend.reg_set_string(hive, path, name, &merged, false, vendor_ref)?;
                    }
                    let new_str = if optimized_entry_value == ABSENT_SENTINEL {
                        SUBKEY_ABSENT_SENTINEL.to_string()
                    } else {
                        format!("entry:{}", optimized_entry_value)
                    };
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
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, &new_str)?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::RegistryDynamicDword {
                    hive,
                    path,
                    name,
                    formula,
                    ..
                } => {
                    let prev = backend.reg_get_dword(hive, path, name);
                    let target = crate::scanner::resolve_dynamic_dword(backend, *formula);
                    backend.reg_set_dword(hive, path, name, target)?;
                    let orig_str = prev
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    let action_key = format!("RegistryDynamicDword:{}", format_pipe_reg_target(hive, path, name));
                    let id = ledger.record_change(&tweak.id, &action_key, &orig_str, &target.to_string())?;
                    recorded_ids.push(id);
                    applied += 1;
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
                        Some(s) if !s.trim().is_empty() => crate::hardware::power::normalize_guid_str(s),
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
                    let ac_opt = if matches!(scope, PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc) {
                        Some(*optimized_ac)
                    } else {
                        None
                    };
                    let dc_opt = if matches!(scope, PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc) {
                        Some(*optimized_dc)
                    } else {
                        None
                    };
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
                        &action_key,
                        &format_power_indices(*scope, prev_ac, prev_dc),
                        &format_power_indices(*scope, *optimized_ac, *optimized_dc),
                    )?;
                    recorded_ids.push(id);
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
                                &format!("SYSTEM\\CurrentControlSet\\Services\\{}", service_name.trim()),
                                "DelayedAutostart",
                            )
                            .unwrap_or(0)
                    } else {
                        0
                    };
                    backend.set_service_start_type(service_name, *target_start_type)?;
                    let action_key = format!("ServiceState:{}", service_name);
                    let orig_str = match prev {
                        Some(2) if prev_delayed == 1 => "2|delayed=1".to_string(),
                        Some(v) => v.to_string(),
                        None => ABSENT_SENTINEL.to_string(),
                    };
                    let id = ledger.record_change(
                        &tweak.id,
                        &action_key,
                        &orig_str,
                        &target_start_type.to_string(),
                    )?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    let prev = backend.get_task_enabled(task_path);
                    backend.set_scheduled_task_enabled(task_path, *enable)?;
                    let action_key = format!("ScheduledTask:{}", task_path);
                    let orig_str = prev
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    let id = ledger.record_change(
                        &tweak.id,
                        &action_key,
                        &orig_str,
                        &enable.to_string(),
                    )?;
                    recorded_ids.push(id);
                    applied += 1;
                }
                TweakAction::AppXPackage { package_family } => {
                    backend.remove_appx_package(package_family)?;
                    let action_key = format!("AppXPackage:{}", package_family);
                    let id = ledger.record_change(&tweak.id, &action_key, "installed", "removed")?;
                    recorded_ids.push(id);
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
                    backend.set_optional_feature(feature_name, *kind, *enable)?;
                    let action_key = format!("WindowsFeature:{:?}|{}", kind, escape_pipe(feature_name));
                    let id = ledger.record_change(
                        &tweak.id,
                        &action_key,
                        &prev_enabled.to_string(),
                        &enable.to_string(),
                    )?;
                    recorded_ids.push(id);
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

/// Apply multiple tweaks by ID using an explicit `SystemBackend`.
pub fn apply_tweaks_by_id_with(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let mut total = 0usize;
    for id in tweak_ids {
        if let Some(tweak) = all_tweaks.iter().find(|t| t.id == *id) {
            total += apply_tweak_with(tweak, ledger, backend)?;
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
                    let current_val = backend
                        .reg_get_dword_for_vendor(hive, path, name, vendor_ref)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());

                    backend.reg_set_dword_for_vendor(hive, path, name, *default_value, vendor_ref)?;

                    let action_key = format_dword_action_key("Revert:", hive, path, name, vendor_ref);
                    let id = ledger.record_change(
                        &tweak.id,
                        &action_key,
                        &current_val,
                        &default_value.to_string(),
                    )?;
                    recorded_ids.push(id);
                    reverted += 1;
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
                    let current = backend.reg_get_string(hive, path, name, vendor_ref);
                    let orig_str = match &current {
                        None => ABSENT_SENTINEL.to_string(),
                        Some(s) if s.expand_sz => format!("expand_sz:{}", s.value),
                        Some(s) => format!("sz:{}", s.value),
                    };
                    let new_str = if *absent_is_default || default_value == ABSENT_SENTINEL {
                        backend.reg_delete_value_for_vendor(hive, path, name, vendor_ref)?;
                        ABSENT_SENTINEL.to_string()
                    } else {
                        backend.reg_set_string(hive, path, name, default_value, *expand_sz, vendor_ref)?;
                        if *expand_sz {
                            format!("expand_sz:{}", default_value)
                        } else {
                            format!("sz:{}", default_value)
                        }
                    };
                    let base_tgt = format_pipe_reg_target(hive, path, name);
                    let action_key = match vendor_ref {
                        Some(v) if !v.trim().is_empty() => format!("RegistryString:{}|vendor={}", base_tgt, escape_pipe(v)),
                        _ => format!("RegistryString:{}", base_tgt),
                    };
                    let id = ledger.record_change(&tweak.id, &format!("Revert:{}", action_key), &orig_str, &new_str)?;
                    recorded_ids.push(id);
                    reverted += 1;
                }
                TweakAction::RegistryDelete { hive, path, name } => {
                    let action_key = format!("RegistryDelete:{}", format_pipe_reg_target(hive, path, name));
                    let history = ledger.get_history()?;
                    if let Some(prior) = history.iter().find(|e| e.tweak_id == tweak.id && e.action_type == action_key && !e.rolled_back) {
                        if prior.original_value != ABSENT_SENTINEL {
                            let snap: RegRawSnapshot = serde_json::from_str(&prior.original_value)?;
                            backend.reg_set_raw(hive, path, name, &snap)?;
                        }
                        let id = ledger.record_change(&tweak.id, &format!("Revert:{}", action_key), ABSENT_SENTINEL, &prior.original_value)?;
                        recorded_ids.push(id);
                        consumed_prior_ids.push(prior.id);
                        reverted += 1;
                    }
                }
                TweakAction::RegistryDeleteTree { hive, path } => {
                    let action_key = format!("RegistryDeleteTree:{}\\{}", hive.trim(), escape_pipe(path));
                    let history = ledger.get_history()?;
                    if let Some(prior) = history.iter().find(|e| e.tweak_id == tweak.id && e.action_type == action_key && !e.rolled_back) {
                        if prior.original_value != ABSENT_SENTINEL {
                            let tree: RegTreeSnapshot = serde_json::from_str(&prior.original_value)?;
                            backend.reg_restore_tree(hive, path, &tree)?;
                        }
                        let id = ledger.record_change(&tweak.id, &format!("Revert:{}", action_key), ABSENT_SENTINEL, &prior.original_value)?;
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
                                    hive, path, name, req, b.len()
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
                                        return Err(WinBlastError::TweakFailed(
                                            "default_full_hex shorter than min_length".to_string(),
                                        ));
                                    }
                                    (ABSENT_SENTINEL.to_string(), decoded)
                                } else {
                                    return Err(WinBlastError::TweakFailed(
                                        "Missing default_full_hex template for absent binary value".to_string(),
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
                    bytes[*byte_index] = (bytes[*byte_index] & !bit_mask) | (default_bits & bit_mask);
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
                    let current_val = backend
                        .reg_get_dword(hive, path, name)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| ABSENT_SENTINEL.to_string());
                    backend.reg_set_dword(hive, path, name, *default_value)?;
                    let action_key = format!("RegistryDynamicDword:{}", format_pipe_reg_target(hive, path, name));
                    let id = ledger.record_change(&tweak.id, &format!("Revert:{}", action_key), &current_val, &default_value.to_string())?;
                    recorded_ids.push(id);
                    reverted += 1;
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
                        Some(s) if !s.trim().is_empty() => crate::hardware::power::normalize_guid_str(s),
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
                    let ac_opt = if matches!(scope, PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc) {
                        Some(*default_ac)
                    } else {
                        None
                    };
                    let dc_opt = if matches!(scope, PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc) {
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
                    let prev = backend.get_service_start_type(service_name);
                    if prev.is_none() {
                        continue;
                    }
                    let current_start = prev.unwrap();
                    let prev_delayed = if current_start == 2 {
                        backend
                            .reg_get_dword(
                                "HKLM",
                                &format!("SYSTEM\\CurrentControlSet\\Services\\{}", service_name.trim()),
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
                    let action_key = format!("WindowsFeature:{:?}|{}", kind, escape_pipe(feature_name));
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
pub fn revert_tweaks_by_id_with(
    tweak_ids: &[String],
    all_tweaks: &[TweakDefinition],
    ledger: &StateLedger,
    backend: &dyn SystemBackend,
) -> Result<usize> {
    let mut total = 0usize;
    for id in tweak_ids {
        if let Some(tweak) = all_tweaks.iter().find(|t| t.id == *id) {
            total += revert_tweak_with(tweak, ledger, backend)?;
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
    if entry.rolled_back {
        return Ok(());
    }

    let raw_action = entry.action_type.strip_prefix("Revert:").unwrap_or(&entry.action_type);
    let parts: Vec<&str> = raw_action.splitn(2, ':').collect();
    if parts.is_empty() {
        return Err(WinBlastError::TweakFailed("Empty action_type in ledger entry".into()));
    }

    let kind = parts[0];
    let target = parts.get(1).copied().unwrap_or_default();

    match kind {
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
                    WinBlastError::TweakFailed(format!("Invalid registry target: {}", raw_tgt))
                })?;
                let last_slash = raw_tgt.rfind('\\').ok_or_else(|| {
                    WinBlastError::TweakFailed(format!("Invalid registry target: {}", raw_tgt))
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
                    WinBlastError::TweakFailed(format!("Invalid original value: {}", entry.original_value))
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
                backend.reg_set_string(&hive, &path, &name, &entry.original_value, false, vendor_ref)?;
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
                WinBlastError::TweakFailed(format!("Invalid tree target: {}", target))
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
                    WinBlastError::TweakFailed(format!("Invalid byte_index in RegistryBinaryBit target: {}", target))
                })?;
                if entry.original_value == ABSENT_SENTINEL {
                    backend.reg_delete_value(&hive, &path, &name)?;
                } else if let Some((mask_s, bits_s)) = entry.original_value.split_once('|') {
                    let bit_mask: u8 = mask_s.parse().map_err(|_| {
                        WinBlastError::TweakFailed(format!("Invalid bit_mask in RegistryBinaryBit: {}", entry.original_value))
                    })?;
                    let orig_bits: u8 = bits_s.parse().map_err(|_| {
                        WinBlastError::TweakFailed(format!("Invalid orig_bits in RegistryBinaryBit: {}", entry.original_value))
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
                    live_bytes[byte_index] = (live_bytes[byte_index] & !bit_mask) | (orig_bits & bit_mask);
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
                    WinBlastError::TweakFailed(format!("Invalid original value: {}", entry.original_value))
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
                return Err(WinBlastError::TweakFailed(format!("Invalid PowerCfg target: {}", target)));
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
                    WinBlastError::TweakFailed(format!("Invalid original service start: {}", entry.original_value))
                })?;
                if was_delayed && original_start == 2 {
                    let svc_path = format!("SYSTEM\\CurrentControlSet\\Services\\{}", service_name.trim());
                    let _ = backend.reg_set_dword("HKLM", &svc_path, "DelayedAutostart", 1);
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
                return Err(WinBlastError::Rollback("Missing startup entry ID".to_string()));
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
        "HostsBlock" | "GpuHags" | "GpuMpo" | "GpuMsi" | "CoreParking" => {
            match kind {
                "HostsBlock" => {
                    crate::executor::ps_runner::unblock_telemetry_hosts_with(backend)?;
                }
                "GpuHags" => {
                    let orig_hags = entry.original_value == "true";
                    backend.set_hags(orig_hags).map_err(WinBlastError::Registry)?;
                }
                "GpuMpo" => {
                    let orig_mpo = entry.original_value == "true";
                    backend.set_mpo_disabled(orig_mpo).map_err(WinBlastError::Registry)?;
                }
                "GpuMsi" => {
                    if target.is_empty() {
                        return Err(WinBlastError::Rollback("Missing GPU PnP device ID".to_string()));
                    }
                    let (msi, prio) = if let Some((msi_s, prio_s)) = entry.original_value.split_once('|') {
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
                    backend.set_gpu_msi_mode(target, msi, prio).map_err(WinBlastError::Registry)?;
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
            }
        }
        _ => {
            return Err(WinBlastError::TweakFailed(format!("Unknown action type in ledger: {}", kind)));
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
        };

        assert_eq!(sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"), None);
        apply_tweak_with(&tweak, &ledger, &sbx).unwrap();
        assert_eq!(sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"), Some(1));

        let hist = ledger.get_history().unwrap();
        rollback_entry_with(&hist[0], &ledger, &sbx).unwrap();
        assert_eq!(sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "TransientKey"), None);
    }

    #[test]
    fn test_batch1_engine_edge_cases_and_atomic_revert() {
        use crate::core::risk::{OptionalFeatureKind, OsRequirement, Reversibility, RiskLevel, RiskValuation};

        let mk_tweak = |id: &str, min_build: u32, gpu_vendor: Option<&str>, actions: Vec<TweakAction>| TweakDefinition {
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
        };

        // 1. OS + GPU vendor guard on apply/revert
        let sbx = SandboxEnv::new();
        {
            let mut st = sbx.inner.write().unwrap();
            st.os_info.build_number = 19041;
            st.gpus.retain(|g| g.vendor == crate::hardware::gpu::GpuVendor::Nvidia);
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
        sbx.reg_set_dword("HKCU", "Software\\WinBlast0_Test", "Step0", 1).unwrap();
        assert!(revert_tweak_with(&rev_fail_tweak, &ledger, &sbx).is_err());
        assert_eq!(sbx.reg_get_dword("HKCU", "Software\\WinBlast0_Test", "Step0"), Some(1));
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
        assert_eq!(sbx.reg_get_binary("HKCU", "Software\\WinBlast0_Test", "MaskVal").unwrap()[0], 0x9e);
        // Now delete MaskVal via rolling back first entry (__ABSENT__), then roll back second entry out-of-order
        let bin_entries: Vec<_> = ledger
            .get_history()
            .unwrap()
            .into_iter()
            .filter(|e| e.tweak_id == "bin_seed_rev" && !e.rolled_back)
            .collect();
        // Reverse order (oldest first -> original_value == __ABSENT__ deletes MaskVal)
        rollback_entry_with(&bin_entries[1], &ledger, &sbx).unwrap();
        assert!(sbx.reg_get_binary("HKCU", "Software\\WinBlast0_Test", "MaskVal").is_none());
        rollback_entry_with(&bin_entries[0], &ledger, &sbx).unwrap();

        // 5. CompositeStringKey with `#` in path/name/entry_key + existing empty "" preservation + pipe RegistryDword
        sbx.reg_set_string("HKCU", "Software\\Test#Hash", "Val#Pipe|Name", "", false, None)
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
                ("HKLM".into(), "system\\currentcontrolset\\services\\mapsbroker".into(), "delayedautostart".into()),
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
        sbx.reg_delete_value("HKLM", "SYSTEM\\CurrentControlSet\\Services\\MapsBroker", "DelayedAutostart")
            .unwrap();
        rollback_entry_with(&svc_entry, &ledger, &sbx).unwrap();
        assert_eq!(sbx.get_service_start_type("MapsBroker"), Some(2));
        assert_eq!(
            sbx.reg_get_dword("HKLM", "SYSTEM\\CurrentControlSet\\Services\\MapsBroker", "DelayedAutostart"),
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
}
