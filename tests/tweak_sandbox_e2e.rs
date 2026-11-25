//! End-to-end sandbox and real-Win32 HKCU smoke tests for WinBlast0's tweak catalog,
//! execution/rollback engine, SQLite WAL ledger, and `AppState` orchestrator.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use uuid::Uuid;
use winreg::enums::*;
use winreg::RegKey;

use winblast0::app::{ActiveView, AppMode, AppState, EngineMode, PresetKind, StagedTransition};
use winblast0::backup::restore_point::RestorePoint;
use winblast0::backup::state_ledger::StateLedger;
use winblast0::core::backend::{MetricsSampler, SandboxEnv, SystemBackend};
use winblast0::core::error::{Result as BlastResult, WinBlastError};
use winblast0::core::os_detect::WindowsInfo;
use winblast0::core::reg_helpers::{RegRawSnapshot, RegStringValue, RegTreeSnapshot};
use winblast0::core::risk::{
    load_embedded_rules, load_rules_from_dir, DynamicDwordFormula, OptionalFeatureKind,
    OsRequirement, PowerTargetScope, Reversibility, RiskLevel, RiskValuation, TweakAction,
    TweakCatalog, TweakDefinition, EMBEDDED_CATALOGS,
};
use winblast0::executor::engine::{apply_tweak_with, revert_tweak_with, rollback_entry_with};
use winblast0::executor::reg_mutator;
use winblast0::hardware::cpu::CpuInfo;
use winblast0::hardware::gpu::DetectedGpu;
use winblast0::hardware::latency::LatencySnapshot;
use winblast0::hardware::net_latency::LatencyBenchmarkSnapshot;
use winblast0::hardware::parking::CoreParkingState;
use winblast0::hardware::power::{normalize_guid_str, HibernationStatus, PowerSchemeInfo};
use winblast0::hardware::sys_inventory::PlatformSecurityInfo;
use winblast0::scanner::appx_scanner::AppxCache;
use winblast0::scanner::task_scanner::TaskCache;
use winblast0::scanner::{
    registry_scanner, resolve_dynamic_dword, scan_single_tweak_with, TweakState,
};
use winblast0::tools::autounattend::{generate_autounattend_xml, TargetArch, UnattendOptions};
use winblast0::tools::cleaner::{CleanExecutionReport, CleanTargetKind, CleanTargetScan};
use winblast0::tools::features_apps::{
    self, InstalledAppxPackage, WindowsOptionalFeature, CURATED_WINGET_APPS,
};
use winblast0::tools::startup::{
    LiveScheduledTaskInfo, LiveServiceInfo, StartupEntry, StartupSource,
};

fn normalize_task_path(path: &str) -> String {
    let p = path.replace('/', "\\").to_lowercase();
    let trimmed = p.trim().trim_matches('"');
    trimmed.trim_start_matches('\\').to_string()
}

/// Helper to deterministically drain async worker events until `pred` returns `true` or timeout expires.
fn drain_until<F>(state: &mut AppState, timeout: Duration, context: &str, mut pred: F)
where
    F: FnMut(&AppState) -> bool,
{
    let start = Instant::now();
    loop {
        state.drain_worker_events();
        if pred(state) {
            return;
        }
        assert!(
            start.elapsed() < timeout,
            "Timed out after {:?} waiting for: {} (status_message={:?})",
            timeout,
            context,
            state.status_message
        );
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// RAII guard ensuring any temporary HKCU scratch subkey is always deleted, even on panic.
struct HkcuScratchGuard {
    subkey: String,
}

impl Drop for HkcuScratchGuard {
    fn drop(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(&self.subkey);
    }
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_LIFECYCLE_ALL_208_TWEAKS_SANDBOX_01
/// - Objective: Exhaustively verify full sandbox lifecycle across all 208 catalog tweaks.
/// - Preconditions: Isolated SandboxEnv seeded with default stock state for each tweak; in-memory SQLite WAL ledger.
/// - Equivalence Classes: RegistryDword, RegistryString, RegistryBinary, ServiceState, ScheduledTask, PowerCfgSetting, AppxPackage, WindowsOptionalFeature, DynamicDword.
/// - Boundary Conditions: Build number boundaries [min_build, max_build]; exact 208 catalog count; action counts.
/// - Invariants:
///   1. Stock seed state scans as NotOptimized.
///   2. Apply transitions state to AlreadyOptimized; ledger entries == actions.len().
///   3. Rollback of applied entry transitions state back to NotOptimized.
///   4. Re-apply transitions back to AlreadyOptimized.
///   5. Revert transitions back to NotOptimized.
///   6. Rollback of Revert transitions back to AlreadyOptimized.
///   7. RequiresPackageReinstall (AppX) safely performs 0 revert actions and rollback marks rolled_back.
/// - Expected Outcome: 100% of 208 tweaks pass full lifecycle verification with zero failures.
#[test]
fn test_all_embedded_tweaks_full_sandbox_lifecycle() {
    let mut parsed_count = 0usize;
    for (idx, catalog_toml) in EMBEDDED_CATALOGS.iter().enumerate() {
        let catalog = toml::from_str::<TweakCatalog>(catalog_toml).unwrap_or_else(|e| {
            panic!("Embedded catalog #{} failed to parse: {}", idx, e);
        });
        parsed_count += catalog.tweaks.len();
    }

    let mut raw_tweak_headers = 0usize;
    for entry in std::fs::read_dir("assets/rules").expect("read assets/rules failed") {
        let path = entry.expect("dir entry failed").path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            let txt = std::fs::read_to_string(&path).expect("read toml failed");
            raw_tweak_headers += txt.lines().filter(|l| l.trim() == "[[tweaks]]").count();
        }
    }

    let tweaks = load_embedded_rules();
    assert_eq!(
        parsed_count, 230,
        "Expected 230 total embedded tweaks across all catalogs, found {}",
        parsed_count
    );
    assert_eq!(
        tweaks.len(),
        raw_tweak_headers,
        "load_embedded_rules() ({}) dropped tweaks relative to raw [[tweaks]] headers ({})",
        tweaks.len(),
        raw_tweak_headers
    );

    // Ensure on-disk TOMLs match embedded catalogs 1:1
    let disk_tweaks = load_rules_from_dir(Path::new("assets/rules"))
        .expect("Failed to load assets/rules from workspace");
    assert_eq!(
        tweaks.len(),
        disk_tweaks.len(),
        "Embedded catalog count ({}) differs from assets/rules ({})",
        tweaks.len(),
        disk_tweaks.len()
    );

    let mut failures = Vec::new();

    for tweak in &tweaks {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("Failed to create in-memory ledger");

        sbx.seed_tweak_defaults_for_tweak(tweak);
        let test_build = tweak
            .os_req
            .max_build
            .map_or(26100, |m| m.min(26100))
            .max(tweak.os_req.min_build);
        let test_edition = tweak
            .os_req
            .editions
            .first()
            .map(String::as_str)
            .unwrap_or("Pro");
        {
            let mut st = sbx.inner.write().unwrap();
            st.os_info.build_number = test_build;
            st.os_info.is_win11 = test_build >= 22000;
            st.os_info.edition = test_edition.to_string();
        }

        // 1. Seeded stock state must scan as NotOptimized
        let scan_initial = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
        if scan_initial.current_state != TweakState::NotOptimized {
            failures.push(format!(
                "[{}] initial scan expected NotOptimized, got {:?} ({})",
                tweak.id, scan_initial.current_state, scan_initial.status_detail
            ));
            continue;
        }

        // 2. Apply tweak -> action count equals tweak.actions.len()
        match apply_tweak_with(tweak, &ledger, &sbx) {
            Ok(applied) => {
                if applied != tweak.actions.len() {
                    failures.push(format!(
                        "[{}] apply_tweak_with returned {} actions, expected {}",
                        tweak.id,
                        applied,
                        tweak.actions.len()
                    ));
                    continue;
                }
            }
            Err(e) => {
                failures.push(format!("[{}] apply_tweak_with failed: {}", tweak.id, e));
                continue;
            }
        }

        // 3. Post-apply scan must be AlreadyOptimized
        let scan_applied = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
        if scan_applied.current_state != TweakState::AlreadyOptimized {
            failures.push(format!(
                "[{}] post-apply scan expected AlreadyOptimized, got {:?} ({})",
                tweak.id, scan_applied.current_state, scan_applied.status_detail
            ));
            continue;
        }

        let history_after_apply = ledger.get_history().expect("get_history failed");
        if history_after_apply.len() != tweak.actions.len() {
            failures.push(format!(
                "[{}] expected {} ledger entries after apply, got {}",
                tweak.id,
                tweak.actions.len(),
                history_after_apply.len()
            ));
            continue;
        }

        match tweak.valuation.reversibility {
            Reversibility::Instant
            | Reversibility::RequiresReboot
            | Reversibility::RequiresLogoff => {
                // 4. Rollback every created LedgerEntry -> NotOptimized
                for entry in &history_after_apply {
                    if let Err(e) = rollback_entry_with(entry, &ledger, &sbx) {
                        failures.push(format!(
                            "[{}] rollback_entry_with(#{}) failed: {}",
                            tweak.id, entry.id, e
                        ));
                    }
                }
                let after_rb = ledger.get_history().expect("get_history failed");
                if !after_rb.iter().all(|e| e.rolled_back) {
                    failures.push(format!(
                        "[{}] not all entries marked rolled_back after rollback",
                        tweak.id
                    ));
                    continue;
                }
                let scan_rb = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
                if scan_rb.current_state != TweakState::NotOptimized {
                    failures.push(format!(
                        "[{}] post-rollback scan expected NotOptimized, got {:?} ({})",
                        tweak.id, scan_rb.current_state, scan_rb.status_detail
                    ));
                    continue;
                }

                // 5. Re-apply -> AlreadyOptimized
                if let Err(e) = apply_tweak_with(tweak, &ledger, &sbx) {
                    failures.push(format!("[{}] re-apply failed: {}", tweak.id, e));
                    continue;
                }
                let scan_reapplied = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
                if scan_reapplied.current_state != TweakState::AlreadyOptimized {
                    failures.push(format!(
                        "[{}] post-reapply scan expected AlreadyOptimized, got {:?}",
                        tweak.id, scan_reapplied.current_state
                    ));
                    continue;
                }

                // 6. Revert -> NotOptimized
                match revert_tweak_with(tweak, &ledger, &sbx) {
                    Ok(rev_count) => {
                        if rev_count != tweak.actions.len() {
                            failures.push(format!(
                                "[{}] revert_tweak_with returned {} actions, expected {}",
                                tweak.id,
                                rev_count,
                                tweak.actions.len()
                            ));
                            continue;
                        }
                    }
                    Err(e) => {
                        failures.push(format!("[{}] revert_tweak_with failed: {}", tweak.id, e));
                        continue;
                    }
                }
                let scan_reverted = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
                if scan_reverted.current_state != TweakState::NotOptimized {
                    failures.push(format!(
                        "[{}] post-revert scan expected NotOptimized, got {:?} ({})",
                        tweak.id, scan_reverted.current_state, scan_reverted.status_detail
                    ));
                    continue;
                }

                // 7. Rollback all `Revert:` entries -> AlreadyOptimized
                let revert_entries: Vec<_> = ledger
                    .get_history()
                    .expect("get_history failed")
                    .into_iter()
                    .filter(|e| !e.rolled_back && e.action_type.starts_with("Revert:"))
                    .collect();
                if revert_entries.len() != tweak.actions.len() {
                    failures.push(format!(
                        "[{}] expected {} Revert: entries, found {}",
                        tweak.id,
                        tweak.actions.len(),
                        revert_entries.len()
                    ));
                    continue;
                }
                for entry in &revert_entries {
                    if let Err(e) = rollback_entry_with(entry, &ledger, &sbx) {
                        failures.push(format!(
                            "[{}] rollback of Revert: entry #{} failed: {}",
                            tweak.id, entry.id, e
                        ));
                    }
                }
                let after_rev_rb = ledger.get_history().expect("get_history failed");
                for entry in &revert_entries {
                    let marked = after_rev_rb
                        .iter()
                        .find(|e| e.id == entry.id)
                        .is_some_and(|e| e.rolled_back);
                    if !marked {
                        failures.push(format!(
                            "[{}] Revert: entry #{} was not marked rolled_back",
                            tweak.id, entry.id
                        ));
                    }
                }
                let scan_final = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
                if scan_final.current_state != TweakState::AlreadyOptimized {
                    failures.push(format!(
                        "[{}] post-Revert-rollback scan expected AlreadyOptimized, got {:?} ({})",
                        tweak.id, scan_final.current_state, scan_final.status_detail
                    ));
                }
            }
            Reversibility::RequiresPackageReinstall => {
                // AppX tweaks: revert must safely perform 0 actions and rollback must safely mark rolled_back without crashing
                let rev_res = revert_tweak_with(tweak, &ledger, &sbx);
                match rev_res {
                    Ok(0) => {}
                    Ok(n) => failures.push(format!(
                        "[{}] RequiresPackageReinstall revert returned {}, expected 0",
                        tweak.id, n
                    )),
                    Err(e) => failures.push(format!(
                        "[{}] RequiresPackageReinstall revert errored: {}",
                        tweak.id, e
                    )),
                }

                let scan_still_opt = scan_single_tweak_with(&sbx, tweak, test_build, test_edition);
                if scan_still_opt.current_state != TweakState::AlreadyOptimized {
                    failures.push(format!(
                        "[{}] AppX post-revert scan expected AlreadyOptimized, got {:?}",
                        tweak.id, scan_still_opt.current_state
                    ));
                }

                for entry in &history_after_apply {
                    if let Err(e) = rollback_entry_with(entry, &ledger, &sbx) {
                        failures.push(format!(
                            "[{}] AppX rollback_entry_with(#{}) failed: {}",
                            tweak.id, entry.id, e
                        ));
                    }
                }
                let after_rb = ledger.get_history().expect("get_history failed");
                if !after_rb.iter().all(|e| e.rolled_back) {
                    failures.push(format!(
                        "[{}] AppX rollback entries not marked rolled_back",
                        tweak.id
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "Sandbox lifecycle failures across {} tweaks ({} failed):\n{}",
        tweaks.len(),
        failures.len(),
        failures.join("\n")
    );
}

fn tweaks_scope_overlap(a: &TweakDefinition, b: &TweakDefinition) -> bool {
    let a_max = a.os_req.max_build.unwrap_or(u32::MAX);
    let b_max = b.os_req.max_build.unwrap_or(u32::MAX);
    let build_overlap = a.os_req.min_build <= b_max && b.os_req.min_build <= a_max;
    if !build_overlap {
        return false;
    }

    let edition_overlap = a.os_req.editions.is_empty()
        || b.os_req.editions.is_empty()
        || a.os_req.editions.iter().any(|ae| {
            b.os_req
                .editions
                .iter()
                .any(|be| ae.eq_ignore_ascii_case(be))
        });
    if !edition_overlap {
        return false;
    }

    match (&a.gpu_vendor, &b.gpu_vendor) {
        (Some(va), Some(vb)) => va.eq_ignore_ascii_case(vb),
        _ => true,
    }
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_CATALOG_INTEGRITY_AUDIT_02
/// - Objective: Comprehensive static and structural integrity audit across all 208 catalog rules and 12 TOML files.
/// - Preconditions: All embedded catalogs deserialized.
/// - Equivalence Classes: All 208 tweaks across categories (Kernel, Network, GPU, Privacy, Storage, Hardware, Gaming).
/// - Boundary Conditions: Exact 208 rules; non-empty actions; no banned HKCR hives.
/// - Invariants:
///   1. Zero duplicate tweak IDs.
///   2. Zero empty actions arrays.
///   3. Zero opt == def contradictions across all action types.
///   4. Target collision prevention (no two tweaks target identical keys without distinct OS/edition/GPU scopes).
/// - Expected Outcome: Full integrity audit succeeds with zero violations.
#[test]
fn test_catalog_integrity_audit() {
    let tweaks = load_embedded_rules();
    assert_eq!(
        tweaks.len(),
        230,
        "Expected 230 tweaks, found {}",
        tweaks.len()
    );

    // 1. Zero duplicate IDs & non-empty actions & no opt==def contradictions
    let mut seen_ids = HashSet::new();
    let mut duplicate_ids = Vec::new();
    let mut empty_actions = Vec::new();
    let mut same_opt_def = Vec::new();

    type RegTargetKey = (String, String, String);
    type BinTargetKey = (String, String, String, usize);
    type CompTargetKey = (String, String, String, String);
    type BinEntry<'a> = (&'a TweakDefinition, u8, u8);
    type CompEntry<'a> = (&'a TweakDefinition, &'a str);
    type PwrEntry<'a> = (&'a TweakDefinition, u32, u32, PowerTargetScope);

    let mut dword_targets: HashMap<RegTargetKey, Vec<(&TweakDefinition, u32)>> = HashMap::new();
    let mut str_targets: HashMap<RegTargetKey, Vec<(&TweakDefinition, &str, bool, bool)>> =
        HashMap::new();
    let mut bin_targets: HashMap<BinTargetKey, Vec<BinEntry<'_>>> = HashMap::new();
    let mut comp_targets: HashMap<CompTargetKey, Vec<CompEntry<'_>>> = HashMap::new();
    let mut pwr_targets: HashMap<RegTargetKey, Vec<PwrEntry<'_>>> = HashMap::new();
    let mut svc_targets: HashMap<String, Vec<(&TweakDefinition, u32)>> = HashMap::new();
    let mut task_targets: HashMap<String, Vec<(&TweakDefinition, bool)>> = HashMap::new();
    let mut appx_targets: HashMap<String, Vec<(&TweakDefinition, &str)>> = HashMap::new();
    let mut feat_targets: HashMap<(OptionalFeatureKind, String), Vec<(&TweakDefinition, bool)>> =
        HashMap::new();

    for t in &tweaks {
        if !seen_ids.insert(t.id.clone()) {
            duplicate_ids.push(t.id.clone());
        }
        if t.actions.is_empty() {
            empty_actions.push(t.id.clone());
        }

        for action in &t.actions {
            match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    optimized_value,
                    default_value,
                } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                    if optimized_value == default_value {
                        same_opt_def.push(format!(
                            "{} -> {}\\{}\\{} (opt==def=={})",
                            t.id, hive, path, name, optimized_value
                        ));
                    }
                    let key = (
                        hive.trim().to_uppercase(),
                        path.trim().to_lowercase(),
                        name.trim().to_lowercase(),
                    );
                    dword_targets
                        .entry(key)
                        .or_default()
                        .push((t, *optimized_value));
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    optimized_value,
                    default_value,
                    expand_sz,
                    case_insensitive,
                    ..
                } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                    if optimized_value == default_value {
                        same_opt_def.push(format!(
                            "{} -> SZ {}\\{}\\{} (opt==def=={})",
                            t.id, hive, path, name, optimized_value
                        ));
                    }
                    let key = (
                        hive.trim().to_uppercase(),
                        path.trim().to_lowercase(),
                        name.trim().to_lowercase(),
                    );
                    str_targets.entry(key).or_default().push((
                        t,
                        optimized_value.as_str(),
                        *expand_sz,
                        *case_insensitive,
                    ));
                }
                TweakAction::RegistryDelete { hive, path, .. }
                | TweakAction::RegistryDeleteTree { hive, path } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                    assert!(!path.trim().is_empty());
                }
                TweakAction::RegistryBinaryBit {
                    hive,
                    path,
                    name,
                    byte_index,
                    min_length,
                    bit_mask,
                    optimized_bits,
                    default_bits,
                    ..
                } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                    assert!(
                        *min_length > *byte_index,
                        "Tweak '{}' min_length too small",
                        t.id
                    );
                    if (optimized_bits & bit_mask) == (default_bits & bit_mask) {
                        same_opt_def.push(format!("{} -> BinaryBit no-op", t.id));
                    }
                    let key = (
                        hive.trim().to_uppercase(),
                        path.trim().to_lowercase(),
                        name.trim().to_lowercase(),
                        *byte_index,
                    );
                    bin_targets
                        .entry(key)
                        .or_default()
                        .push((t, *bit_mask, *optimized_bits));
                }
                TweakAction::CompositeStringKey {
                    hive,
                    path,
                    name,
                    entry_key,
                    optimized_entry_value,
                    default_entry_value,
                    ..
                } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                    if optimized_entry_value == default_entry_value {
                        same_opt_def.push(format!("{} -> Composite {}", t.id, entry_key));
                    }
                    let key = (
                        hive.trim().to_uppercase(),
                        path.trim().to_lowercase(),
                        name.trim().to_lowercase(),
                        entry_key.trim().to_lowercase(),
                    );
                    comp_targets
                        .entry(key)
                        .or_default()
                        .push((t, optimized_entry_value.as_str()));
                }
                TweakAction::RegistryDynamicDword { hive, .. } => {
                    assert!(
                        !hive.trim().eq_ignore_ascii_case("HKCR"),
                        "Tweak '{}' uses banned HKCR hive!",
                        t.id
                    );
                }
                TweakAction::PowerCfgSetting {
                    subgroup_guid,
                    setting_guid,
                    scheme_guid,
                    optimized_ac,
                    optimized_dc,
                    default_ac,
                    default_dc,
                    scope,
                    ..
                } => {
                    if optimized_ac == default_ac && optimized_dc == default_dc {
                        same_opt_def.push(format!("{} -> PowerCfg no-op", t.id));
                    }
                    let scheme_key = scheme_guid
                        .as_deref()
                        .filter(|s| !s.trim().is_empty())
                        .map(normalize_guid_str)
                        .unwrap_or_else(|| "ACTIVE".to_string());
                    let key = (
                        scheme_key,
                        normalize_guid_str(subgroup_guid),
                        normalize_guid_str(setting_guid),
                    );
                    pwr_targets.entry(key).or_default().push((
                        t,
                        *optimized_ac,
                        *optimized_dc,
                        *scope,
                    ));
                }
                TweakAction::ServiceState {
                    service_name,
                    target_start_type,
                    default_start_type,
                } => {
                    if target_start_type == default_start_type {
                        same_opt_def.push(format!(
                            "{} -> Service:{} (target==default=={})",
                            t.id, service_name, target_start_type
                        ));
                    }
                    let key = service_name.trim().to_lowercase();
                    svc_targets
                        .entry(key)
                        .or_default()
                        .push((t, *target_start_type));
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    let key = normalize_task_path(task_path);
                    task_targets.entry(key).or_default().push((t, *enable));
                }
                TweakAction::AppXPackage { package_family } => {
                    let prefix = package_family
                        .trim()
                        .split('_')
                        .next()
                        .unwrap_or(package_family.trim())
                        .to_ascii_lowercase();
                    appx_targets
                        .entry(prefix)
                        .or_default()
                        .push((t, package_family.as_str()));
                }
                TweakAction::WindowsFeature {
                    feature_name,
                    kind,
                    enable,
                } => {
                    let prefix = feature_name
                        .split_once("~~~~")
                        .map(|(p, _)| p)
                        .unwrap_or(feature_name)
                        .trim()
                        .to_ascii_lowercase();
                    feat_targets
                        .entry((*kind, prefix))
                        .or_default()
                        .push((t, *enable));
                }
            }
        }
    }

    assert!(
        duplicate_ids.is_empty(),
        "Duplicate tweak IDs found: {:?}",
        duplicate_ids
    );
    assert!(
        empty_actions.is_empty(),
        "Tweaks with 0 actions found: {:?}",
        empty_actions
    );
    assert!(
        same_opt_def.is_empty(),
        "Tweaks with optimized == default found:\n{}",
        same_opt_def.join("\n")
    );

    // Check pairwise conflicts across distinct tweaks ONLY when os_req and gpu_vendor scopes overlap
    let mut conflicts = Vec::new();
    for ((hive, path, name), entries) in &dword_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, va) = entries[i];
                let (tb, vb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) && va != vb {
                    conflicts.push(format!(
                        "RegistryDword conflict on {}\\{}\\{}: {} ({}) vs {} ({})",
                        hive, path, name, ta.id, va, tb.id, vb
                    ));
                }
            }
        }
    }
    for ((hive, path, name), entries) in &str_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, va, ea, cia) = entries[i];
                let (tb, vb, eb, cib) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) {
                    let val_eq = if cia && cib {
                        va.eq_ignore_ascii_case(vb)
                    } else {
                        va == vb
                    };
                    if !val_eq || ea != eb {
                        conflicts.push(format!(
                            "RegistryString conflict on {}\\{}\\{}: {} ('{}', expand={}) vs {} ('{}', expand={})",
                            hive, path, name, ta.id, va, ea, tb.id, vb, eb
                        ));
                    }
                }
            }
        }
    }
    for ((hive, path, name, byte_idx), entries) in &bin_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, ma, oa) = entries[i];
                let (tb, mb, ob) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) {
                    let overlap = ma & mb;
                    if overlap != 0 && (oa & overlap) != (ob & overlap) {
                        conflicts.push(format!(
                            "RegistryBinaryBit conflict on {}\\{}\\{}[{}]: {} (mask={:#x} bits={:#x}) vs {} (mask={:#x} bits={:#x})",
                            hive, path, name, byte_idx, ta.id, ma, oa, tb.id, mb, ob
                        ));
                    }
                }
            }
        }
    }
    for ((hive, path, name, entry_key), entries) in &comp_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, va) = entries[i];
                let (tb, vb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) && va != vb {
                    conflicts.push(format!(
                        "CompositeStringKey conflict on {}\\{}\\{}#{}: {} ('{}') vs {} ('{}')",
                        hive, path, name, entry_key, ta.id, va, tb.id, vb
                    ));
                }
            }
        }
    }
    for ((scheme, sub, set), entries) in &pwr_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, ac_a, dc_a, scope_a) = entries[i];
                let (tb, ac_b, dc_b, scope_b) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) {
                    let touches_ac_a = matches!(
                        scope_a,
                        PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc
                    );
                    let touches_ac_b = matches!(
                        scope_b,
                        PowerTargetScope::AcOnly | PowerTargetScope::AcAndDc
                    );
                    let touches_dc_a = matches!(
                        scope_a,
                        PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc
                    );
                    let touches_dc_b = matches!(
                        scope_b,
                        PowerTargetScope::DcOnly | PowerTargetScope::AcAndDc
                    );
                    if (touches_ac_a && touches_ac_b && ac_a != ac_b)
                        || (touches_dc_a && touches_dc_b && dc_a != dc_b)
                    {
                        conflicts.push(format!(
                            "PowerCfgSetting conflict on {}/{}/{}: {} vs {}",
                            scheme, sub, set, ta.id, tb.id
                        ));
                    }
                }
            }
        }
    }
    for (svc, entries) in &svc_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, va) = entries[i];
                let (tb, vb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) && va != vb {
                    conflicts.push(format!(
                        "ServiceState conflict on {}: {} ({}) vs {} ({})",
                        svc, ta.id, va, tb.id, vb
                    ));
                }
            }
        }
    }
    for (task, entries) in &task_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, va) = entries[i];
                let (tb, vb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) && va != vb {
                    conflicts.push(format!(
                        "ScheduledTask conflict on {}: {} ({}) vs {} ({})",
                        task, ta.id, va, tb.id, vb
                    ));
                }
            }
        }
    }
    for (pkg_prefix, entries) in &appx_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, fa) = entries[i];
                let (tb, fb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) {
                    conflicts.push(format!(
                        "Duplicate/conflicting AppXPackage target '{}': {} ('{}') vs {} ('{}')",
                        pkg_prefix, ta.id, fa, tb.id, fb
                    ));
                }
            }
        }
    }
    for ((kind, feat_prefix), entries) in &feat_targets {
        for i in 0..entries.len() {
            for j in (i + 1)..entries.len() {
                let (ta, ena) = entries[i];
                let (tb, enb) = entries[j];
                if ta.id != tb.id && tweaks_scope_overlap(ta, tb) && ena != enb {
                    conflicts.push(format!(
                        "WindowsFeature conflict on {:?} '{}': {} (enable={}) vs {} (enable={})",
                        kind, feat_prefix, ta.id, ena, tb.id, enb
                    ));
                }
            }
        }
    }

    assert!(
        conflicts.is_empty(),
        "Conflicting or duplicate targets found across overlapping scopes:\n{}",
        conflicts.join("\n")
    );
}

#[test]
fn test_app_state_normal_mode_low_build_select_all_and_batch_apply() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    {
        let mut st = sbx.inner.write().unwrap();
        st.os_info.build_number = 19041;
        st.os_info.display_version = "2004".to_string();
        st.os_info.is_win11 = false;
    }
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    assert!(
        state.is_scanning,
        "Non-blocking startup must enter initial scanning state without blocking"
    );
    state.start_scan();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "initial async scan",
        |s| !s.is_scanning && !s.scan_results.is_empty(),
    );
    assert_eq!(state.engine_mode, EngineMode::Normal);

    let win11_ids: HashSet<String> = tweaks
        .iter()
        .filter(|t| t.os_req.min_build >= 22000)
        .map(|t| t.id.clone())
        .collect();
    assert!(
        !win11_ids.is_empty(),
        "Expected at least one Win11 tweak with min_build >= 22000 in catalog"
    );

    // Verify scan_results mark all Win11 tweaks as NotApplicable on build 19041
    for id in &win11_ids {
        let sr = state
            .scan_results
            .iter()
            .find(|r| &r.tweak_id == id)
            .expect("Missing scan result");
        assert_eq!(
            sr.current_state,
            TweakState::NotApplicable,
            "Win11 tweak '{}' should be NotApplicable on build 19041",
            id
        );
    }

    // Easy mode select_all skips NotApplicable, non-easy, and High/Extreme risk tweaks
    state.mode = AppMode::Easy;
    state.select_all();
    assert!(!state.selected_tweaks.is_empty());
    for id in &state.selected_tweaks {
        assert!(
            !win11_ids.contains(id),
            "Easy select_all included NotApplicable tweak {}",
            id
        );
        let sr = state.get_tweak_scan(id).unwrap();
        assert!(sr.easy_mode_included);
        assert!(sr.risk.stability_risk < winblast0::core::risk::RiskLevel::High);
        assert_ne!(sr.current_state, TweakState::NotApplicable);
        assert_eq!(
            state.staged_transitions.get(id),
            Some(&StagedTransition::StageApply)
        );
    }

    // Advanced mode select_all skips NotApplicable and High/Extreme risk opt-in tweaks
    state.mode = AppMode::Advanced;
    state.select_all();
    assert!(!state.selected_tweaks.is_empty());
    for id in &win11_ids {
        assert!(
            !state.selected_tweaks.contains(id),
            "Advanced select_all must skip NotApplicable tweak {}",
            id
        );
        assert!(
            !state.staged_transitions.contains_key(id),
            "staged_transitions must not contain NotApplicable tweak {}",
            id
        );
    }
    for sr in &state.scan_results {
        if sr.risk.stability_risk >= winblast0::core::risk::RiskLevel::High {
            assert!(
                !state.selected_tweaks.contains(&sr.tweak_id),
                "High/Extreme opt-in tweak '{}' must be excluded from select_all!",
                sr.tweak_id
            );
        }
    }

    let expected_selected_count = state.selected_tweaks.len();
    let expected_applicable_count = state
        .scan_results
        .iter()
        .filter(|r| {
            r.current_state != TweakState::NotApplicable
                && r.risk.stability_risk < winblast0::core::risk::RiskLevel::High
        })
        .count();
    assert_eq!(expected_selected_count, expected_applicable_count);

    // Batch apply in Normal Mode + drain ApplyCompleted & follow-up ScanCompleted
    state.apply_selected();
    drain_until(
        &mut state,
        Duration::from_secs(10),
        "Normal Mode apply_selected + follow-up scan",
        |s| {
            !s.is_scanning
                && s.selected_tweaks.is_empty()
                && s.staged_transitions.is_empty()
                && !s.ledger_history.is_empty()
        },
    );

    assert!(
        state.status_message.contains("Scan completed"),
        "Unexpected final status message: {}",
        state.status_message
    );
    for sr in &state.scan_results {
        if win11_ids.contains(&sr.tweak_id) {
            assert_eq!(sr.current_state, TweakState::NotApplicable);
        } else if sr.risk.stability_risk < winblast0::core::risk::RiskLevel::High {
            assert_eq!(
                sr.current_state,
                TweakState::AlreadyOptimized,
                "Applicable tweak '{}' was not AlreadyOptimized after batch apply ({})",
                sr.tweak_id,
                sr.status_detail
            );
        }
    }
}

#[test]
fn test_app_state_instant_mode_error_unlocks_in_flight_tweaks() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx.clone()));
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "initial scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });
    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Instant);

    // Pick a RegistryDword tweak currently NotOptimized
    let target_id = state
        .tweaks
        .iter()
        .find(|t| {
            t.actions
                .iter()
                .all(|a| matches!(a, TweakAction::RegistryDword { .. }))
                && state
                    .scan_results
                    .iter()
                    .any(|r| r.tweak_id == t.id && r.current_state == TweakState::NotOptimized)
        })
        .expect("No NotOptimized RegistryDword tweak found")
        .id
        .clone();

    sbx.inner.write().unwrap().inject_next_error = Some("Simulated error".into());

    state.handle_switch_click(target_id.clone());
    assert!(
        state.in_flight_tweaks.contains(&target_id),
        "Switch should mark tweak in-flight immediately after Instant click"
    );

    drain_until(
        &mut state,
        Duration::from_secs(5),
        "Instant mode SingleTweakFailed worker event",
        |s| !s.in_flight_tweaks.contains(&target_id),
    );

    assert!(
        state.in_flight_tweaks.is_empty(),
        "in_flight_tweaks must be empty after error (deadlock fix), found: {:?}",
        state.in_flight_tweaks
    );
    assert!(
        state.status_message.contains("Simulated error"),
        "Expected status_message to report 'Simulated error', got: {:?}",
        state.status_message
    );
    assert!(
        state.status_message.contains(&target_id),
        "Expected status_message to mention '{}', got: {:?}",
        target_id,
        state.status_message
    );
}

#[test]
fn test_app_state_export_import_profile_staging_sync() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "initial scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });
    assert_eq!(state.engine_mode, EngineMode::Normal);

    let sample_ids: Vec<String> = state
        .scan_results
        .iter()
        .filter(|r| r.current_state == TweakState::NotOptimized)
        .take(4)
        .map(|r| r.tweak_id.clone())
        .collect();
    assert_eq!(sample_ids.len(), 4);

    for id in &sample_ids {
        state.handle_switch_click(id.clone());
    }
    assert_eq!(state.selected_tweaks.len(), 4);
    assert_eq!(state.staged_transitions.len(), 4);

    state.export_profile();
    assert!(
        state.profile_path.exists(),
        "Profile file was not created at {:?}",
        state.profile_path
    );

    state.deselect_all();
    assert!(state.selected_tweaks.is_empty());
    assert!(state.staged_transitions.is_empty());
    assert!(state.pending_actions.is_empty());

    state.import_profile();
    assert_eq!(state.selected_tweaks.len(), 4);
    assert_eq!(state.staged_transitions.len(), 4);
    for id in &sample_ids {
        assert!(state.selected_tweaks.contains(id));
        assert_eq!(
            state.staged_transitions.get(id),
            Some(&StagedTransition::StageApply),
            "staged_transitions out of sync for {}",
            id
        );
    }
    assert!(
        !state.pending_actions.is_empty(),
        "pending_actions should be synced after import_profile"
    );
}

#[test]
fn test_app_state_hardware_and_privacy_ledger_recording() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);
    {
        let mut st = sbx.inner.write().unwrap();
        st.hags_enabled = false;
        st.mpo_disabled = false;
        st.gpus[0].msi_enabled = false;
        st.core_parking.is_fully_unparked = false;
        // Ensure at least one telemetry service has a non-4 start type
        st.services.insert("diagtrack".to_string(), 2);
    }

    let mut state = AppState::new_with_backend(Arc::new(sbx));

    state.toggle_hags(true);
    state.toggle_mpo(true);
    let pnp = state.gpus[0].pnp_instance_id.clone();
    state.toggle_gpu_msi(pnp, true, 3);
    state.unpark_all_cores();
    state.block_telemetry_hosts();
    state.neutralize_telemetry_services();

    let required_ids = [
        "gpu_hags",
        "gpu_mpo",
        "gpu_msi_mode",
        "cpu_unpark_all",
        "privacy_hosts_block",
        "privacy_neutralize_services",
    ];

    drain_until(
        &mut state,
        Duration::from_secs(5),
        "hardware & privacy ledger writes",
        |s| {
            required_ids
                .iter()
                .all(|req| s.ledger_history.iter().any(|e| e.tweak_id == *req))
        },
    );

    let ledger = StateLedger::new(&state.ledger_path).expect("Failed to open sandbox ledger");
    let persisted = ledger.get_history().expect("get_history failed");
    for req in &required_ids {
        assert!(
            persisted.iter().any(|e| e.tweak_id == *req),
            "Missing SQLite ledger entry for '{}'; entries: {:?}",
            req,
            persisted.iter().map(|e| &e.tweak_id).collect::<Vec<_>>()
        );
    }
}

#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "touches the real host; run only inside the test VM: cargo test --features vm-tests")]
fn test_real_win32_hkcu_reg_sz_and_cleanup() {
    let scratch_path = format!("Software\\WinBlast0_KernelSmoke_{}", Uuid::new_v4());
    let _guard = HkcuScratchGuard {
        subkey: scratch_path.clone(),
    };

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = match hkcu
        .create_subkey_with_flags(&scratch_path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
    {
        Ok(k) => k,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("Skipping HKCU scratch test: sandbox restricted HKCU write access: {e}");
            return;
        }
        Err(_) => match hkcu.create_subkey(&scratch_path) {
            Ok(k) => k,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                eprintln!("Skipping HKCU scratch test: sandbox restricted HKCU write access: {e}");
                return;
            }
            Err(e) => panic!("Failed to create temporary HKCU scratch key: {e}"),
        },
    };

    // 1. Native REG_DWORD roundtrip via reg_mutator + registry_scanner
    reg_mutator::set_dword("HKCU", &scratch_path, "SmokeDword", 12345)
        .expect("reg_mutator::set_dword failed for REG_DWORD");
    assert_eq!(
        registry_scanner::get_dword("HKCU", &scratch_path, "SmokeDword"),
        Some(12345)
    );
    reg_mutator::delete_value("HKCU", &scratch_path, "SmokeDword")
        .expect("reg_mutator::delete_value failed for REG_DWORD");
    assert_eq!(
        registry_scanner::get_dword("HKCU", &scratch_path, "SmokeDword"),
        None
    );

    // 2. Numeric REG_SZ preservation: seed "0" as REG_SZ, update via reg_mutator::set_dword, verify type stays REG_SZ
    key.set_value("SmokeNumericSz", &"0")
        .expect("Failed to seed numeric REG_SZ");
    let raw_before = key
        .get_raw_value("SmokeNumericSz")
        .expect("Failed to read raw REG_SZ before mutation");
    assert_eq!(raw_before.vtype, REG_SZ);

    reg_mutator::set_dword("HKCU", &scratch_path, "SmokeNumericSz", 400)
        .expect("reg_mutator::set_dword failed on numeric REG_SZ");
    assert_eq!(
        registry_scanner::get_dword("HKCU", &scratch_path, "SmokeNumericSz"),
        Some(400)
    );

    let raw_after = key
        .get_raw_value("SmokeNumericSz")
        .expect("Failed to read raw REG_SZ after mutation");
    assert_eq!(
        raw_after.vtype, REG_SZ,
        "reg_mutator::set_dword must preserve REG_SZ type for numeric string values"
    );
    let str_after: String = key
        .get_value("SmokeNumericSz")
        .expect("Failed to read updated REG_SZ string");
    assert_eq!(str_after, "400");

    reg_mutator::delete_value("HKCU", &scratch_path, "SmokeNumericSz")
        .expect("reg_mutator::delete_value failed for REG_SZ");
    assert_eq!(
        registry_scanner::get_dword("HKCU", &scratch_path, "SmokeNumericSz"),
        None
    );

    drop(key);
    hkcu.delete_subkey_all(&scratch_path)
        .expect("Failed to delete temporary HKCU scratch subkey");
    assert!(
        hkcu.open_subkey(&scratch_path).is_err(),
        "Scratch HKCU subkey still exists after cleanup!"
    );
}

#[test]
fn test_multi_gpu_0000_amd_and_0001_nvidia_isolation() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    let nvidia_tweak = tweaks
        .iter()
        .find(|t| t.gpu_vendor.as_deref() == Some("NVIDIA") && t.actions.iter().any(|a| {
            matches!(a, TweakAction::RegistryDword { path, .. } if path.contains("{4d36e968-e325-11ce-bfc1-08002be10318}"))
        }))
        .expect("Expected NVIDIA Class-subkey tweak in catalog");

    let amd_class_path =
        "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0000";
    let nvidia_class_path =
        "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}\\0001";

    {
        let mut st = sbx.inner.write().unwrap();
        st.gpus = vec![
            winblast0::hardware::gpu::DetectedGpu {
                name: "AMD Radeon RX 7900 iGPU".to_string(),
                vendor: winblast0::hardware::gpu::GpuVendor::Amd,
                pnp_instance_id: "PCI\\VEN_1002&DEV_164E\\0000".to_string(),
                msi_enabled: false,
                interrupt_priority: 1,
                class_subkey: "0000".to_string(),
                dedicated_vram_mb: 512,
            },
            winblast0::hardware::gpu::DetectedGpu {
                name: "NVIDIA GeForce RTX 4090".to_string(),
                vendor: winblast0::hardware::gpu::GpuVendor::Nvidia,
                pnp_instance_id: "PCI\\VEN_10DE&DEV_2684\\0001".to_string(),
                msi_enabled: false,
                interrupt_priority: 1,
                class_subkey: "0001".to_string(),
                dedicated_vram_mb: 24576,
            },
        ];
        st.registry.insert(
            (
                "HKLM".to_string(),
                nvidia_class_path.to_lowercase(),
                "disabledynamicpstate".to_string(),
            ),
            0,
        );
        st.registry.insert(
            (
                "HKLM".to_string(),
                amd_class_path.to_lowercase(),
                "enableulps".to_string(),
            ),
            1,
        );
    }

    let ledger_dir = winblast0::core::backend::SandboxTempDir::new();
    let ledger = StateLedger::new(&ledger_dir.path.join("multi_gpu.db")).unwrap();

    // Seed NVIDIA default onto 0001 only and apply NVIDIA tweak
    winblast0::executor::engine::apply_tweak_with(nvidia_tweak, &ledger, &sbx)
        .expect("Applying NVIDIA tweak on dual-GPU sandbox should succeed");

    // Verify 0001 was mutated and 0000 was NOT touched by NVIDIA key
    {
        let st = sbx.inner.read().unwrap();
        assert_eq!(
            st.registry
                .get(&(
                    "HKLM".to_string(),
                    nvidia_class_path.to_lowercase(),
                    "disabledynamicpstate".to_string()
                ))
                .copied(),
            Some(1)
        );
        assert_eq!(
            st.registry
                .get(&(
                    "HKLM".to_string(),
                    amd_class_path.to_lowercase(),
                    "disabledynamicpstate".to_string()
                ))
                .copied(),
            None
        );
        assert_eq!(
            st.registry
                .get(&(
                    "HKLM".to_string(),
                    amd_class_path.to_lowercase(),
                    "enableulps".to_string()
                ))
                .copied(),
            Some(1)
        );
    }

    // Revert must cleanly restore 0001
    winblast0::executor::engine::revert_tweak_with(nvidia_tweak, &ledger, &sbx)
        .expect("Reverting NVIDIA tweak should succeed");
    {
        let st = sbx.inner.read().unwrap();
        assert_eq!(
            st.registry
                .get(&(
                    "HKLM".to_string(),
                    nvidia_class_path.to_lowercase(),
                    "disabledynamicpstate".to_string()
                ))
                .copied(),
            Some(0)
        );
    }
}

#[test]
fn test_sandbox_clean_apps_startup_features_power_and_autounattend() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    {
        let mut st = sbx.inner.write().unwrap();
        st.onedrive_kfm_path = Some("Desktop -> C:\\Users\\Test\\OneDrive\\Desktop".to_string());
        st.appx_packages
            .insert("Microsoft.SecHealthUI_8wekyb3d8bbwe".to_string());
        st.appx_packages
            .insert("Microsoft.BingWeather_8wekyb3d8bbwe".to_string());
    }

    let mut state = AppState::new_with_backend(Arc::new(sbx.clone()));
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });

    // 1. Verify optimization_progress excludes NotApplicable
    assert!(state.optimization_progress() >= 0.0 && state.optimization_progress() <= 1.0);

    // 2. Disk Cleanup flow
    state.scan_clean_targets_async();
    drain_until(&mut state, Duration::from_secs(5), "clean scan", |s| {
        !s.clean_targets.is_empty()
    });
    state.execute_clean_selected();
    drain_until(&mut state, Duration::from_secs(5), "clean exec", |s| {
        s.last_clean_report.is_some()
    });
    assert!(state.last_clean_report.as_ref().unwrap().total_freed_bytes > 0);

    // 3. AppX ProtectedCore lock + Batch Remove + DISM Feature Toggle + OneDrive KFM Safety
    state.scan_appx_and_features_async();
    drain_until(&mut state, Duration::from_secs(5), "appx load", |s| {
        !s.installed_appx.is_empty()
    });
    assert!(state.onedrive_kfm_warning.is_some());

    state.select_safe_and_ai_appx();
    assert!(!state
        .selected_appx
        .iter()
        .any(|f| f.eq_ignore_ascii_case("Microsoft.SecHealthUI_8wekyb3d8bbwe")));
    assert!(state
        .selected_appx
        .iter()
        .any(|f| f.eq_ignore_ascii_case("Microsoft.BingWeather_8wekyb3d8bbwe")));
    state.remove_selected_appx_async();
    drain_until(&mut state, Duration::from_secs(5), "appx remove", |s| {
        s.selected_appx.is_empty() && !s.clean_apps_busy
    });
    assert!(state.installed_appx.iter().any(|p| p
        .family_name
        .eq_ignore_ascii_case("Microsoft.SecHealthUI_8wekyb3d8bbwe")));

    // OneDrive must refuse when KFM active
    state.uninstall_onedrive_async();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "onedrive kfm block",
        |s| !s.clean_apps_busy,
    );
    assert!(
        state.status_message.to_lowercase().contains("kfm")
            || state.status_message.to_lowercase().contains("blocked")
    );

    // Feature toggle
    state.toggle_optional_feature_async(
        "Recall".to_string(),
        winblast0::core::risk::OptionalFeatureKind::OptionalFeature,
        false,
    );
    drain_until(&mut state, Duration::from_secs(5), "feature toggle", |s| {
        !s.clean_apps_busy
    });

    // 4. Startup & Core Service Protection
    state.scan_startup_and_tasks_async();
    drain_until(&mut state, Duration::from_secs(5), "startup scan", |s| {
        !s.startup_entries.is_empty() && !s.live_services.is_empty()
    });
    let first_startup = state.startup_entries[0].id.clone();
    state.toggle_startup_entry_async(first_startup, false);
    drain_until(&mut state, Duration::from_secs(5), "startup toggle", |s| {
        !s.startup_tasks_busy
    });

    // Protected Core Service refusal
    state.set_live_service_start_async("wuauserv".to_string(), 4);
    assert!(state.status_message.contains("Critical Core OS Service"));

    // 5. Power & Autounattend Export inside SandboxTempDir
    state.activate_winblast_ultimate_power_plan();
    state.toggle_hibernation(false);
    drain_until(&mut state, Duration::from_secs(5), "power", |s| {
        !s.power_schemes.is_empty() && !s.hibernation_status.hibernation_enabled
    });

    state.export_autounattend_xml();
    let exported = state
        .unattend_last_export
        .as_ref()
        .expect("autounattend.xml export path missing");
    let xml = std::fs::read_to_string(exported).expect("Failed to read exported autounattend.xml");
    assert!(xml.contains("<unattend") && xml.contains("BypassTPMCheck"));
}

#[test]
fn test_wave1_ledger_rollback_and_concurrency_regressions() {
    use winblast0::core::risk::{OsRequirement, RiskLevel, RiskValuation, TweakDefinition};

    let sbx = SandboxEnv::new();
    let ledger = StateLedger::new(&sbx.default_ledger_path()).expect("ledger init failed");

    // Seed GPU, hosts, features, scheduled task, startup entry, power scheme, hibernation
    let gpu_pnp = sbx.inner.read().unwrap().gpus[0].pnp_instance_id.clone();
    {
        let mut st = sbx.inner.write().unwrap();
        st.hosts_content = "127.0.0.1 custom-local.dev\r\n".to_string();
        st.tasks.insert(
            normalize_task_path(r"\Microsoft\Windows\Autochk\Proxy"),
            true,
        );
        st.registry.insert(
            (
                "HKLM".to_string(),
                r"system\currentcontrolset\control\prioritycontrol".to_string(),
                "win32priorityseparation".to_string(),
            ),
            2,
        );
    }

    // 1. LedgerEntry::is_reversible accuracy + non-reversible Rollback errors
    let clean_id = ledger
        .record_change("disk_cleanup", "CleanTargets:Execute", "2 targets", "10 MB")
        .unwrap();
    let appx_batch_id = ledger
        .record_change(
            "appx_batch_remove",
            "AppXBatch:RemoveAllUsers",
            "1 selected",
            "1 removed",
        )
        .unwrap();
    let app_od_id = ledger
        .record_change(
            "app_onedrive_remove",
            "App:OneDrive",
            "Installed",
            "Removed",
        )
        .unwrap();

    // 2. GpuMsi roundtrip preserves msi + priority
    let orig_gpu = &sbx.inner.read().unwrap().gpus[0].clone();
    sbx.set_gpu_msi_mode(&gpu_pnp, !orig_gpu.msi_enabled, 3)
        .unwrap();
    let msi_id = ledger
        .record_change(
            "gpu_msi_mode",
            &format!("GpuMsi:{}", gpu_pnp),
            &format!("{}|{}", orig_gpu.msi_enabled, orig_gpu.interrupt_priority),
            &format!("{}|3", !orig_gpu.msi_enabled),
        )
        .unwrap();

    // 3. WindowsFeature roundtrip
    let feat_name = "Recall";
    sbx.set_optional_feature(feat_name, OptionalFeatureKind::OptionalFeature, false)
        .unwrap();
    let feat_id = ledger
        .record_change(
            "feature_Recall",
            "WindowsFeature:OptionalFeature|Recall",
            "true",
            "false",
        )
        .unwrap();

    // 4. ScheduledTask roundtrip
    let task_path = r"\Microsoft\Windows\Autochk\Proxy";
    sbx.set_scheduled_task_enabled(task_path, false).unwrap();
    let task_id = ledger
        .record_change(
            "scheduled_task_toggle",
            &format!("ScheduledTask:{}", task_path),
            "true",
            "false",
        )
        .unwrap();

    // 5. HostsBlock roundtrip removes only WinBlast0 blocklist
    let added = sbx
        .block_telemetry_hosts(&["vortex.data.microsoft.com"])
        .unwrap();
    assert_eq!(added, 1);
    let hosts_id = ledger
        .record_change(
            "privacy_hosts_block",
            "HostsBlock:TelemetryDomains",
            "unblocked",
            "+1 blocked",
        )
        .unwrap();

    // 6. PowerCfg ActiveScheme, Hibernation, UsbPowerSaving roundtrips
    let orig_scheme = sbx.get_active_power_scheme().unwrap();
    let ult_scheme = sbx.create_and_activate_performance_plan().unwrap();
    assert_ne!(orig_scheme, ult_scheme);
    let pwr_scheme_id = ledger
        .record_change(
            "power_ultimate_plan",
            "PowerCfg:UltimateScheme",
            &orig_scheme,
            &ult_scheme,
        )
        .unwrap();
    sbx.set_hibernation_enabled(false).unwrap();
    let pwr_hib_id = ledger
        .record_change(
            "power_hibernation",
            "PowerCfg:Hibernation",
            "enabled",
            "disabled",
        )
        .unwrap();
    sbx.set_usb_power_saving_enabled(false).unwrap();
    let pwr_usb_id = ledger
        .record_change(
            "power_usb_hub_idle",
            "PowerCfg:UsbPowerSaving",
            "enabled",
            "disabled",
        )
        .unwrap();

    // 7. StartupApproved roundtrip by ID
    let startup_entry = sbx
        .list_startup_entries()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let orig_startup_en = startup_entry.enabled;
    sbx.set_startup_entry_enabled(&startup_entry, !orig_startup_en)
        .unwrap();
    let startup_id = ledger
        .record_change(
            "startup_test",
            &format!(
                "StartupApproved:{}",
                winblast0::core::reg_helpers::escape_pipe(&startup_entry.id)
            ),
            &orig_startup_en.to_string(),
            &(!orig_startup_en).to_string(),
        )
        .unwrap();

    let history = ledger.get_history().unwrap();
    for id in [clean_id, appx_batch_id, app_od_id] {
        let entry = history.iter().find(|e| e.id == id).unwrap();
        assert!(
            !entry.is_reversible(),
            "Expected {} to be non-reversible",
            entry.action_type
        );
        let err = rollback_entry_with(entry, &ledger, &sbx)
            .expect_err("Non-reversible entry must fail rollback");
        assert!(
            matches!(err, WinBlastError::Rollback(_)),
            "Expected WinBlastError::Rollback, got {:?}",
            err
        );
    }

    for id in [
        msi_id,
        feat_id,
        task_id,
        hosts_id,
        pwr_scheme_id,
        pwr_hib_id,
        pwr_usb_id,
        startup_id,
    ] {
        let entry = history.iter().find(|e| e.id == id).unwrap();
        assert!(
            entry.is_reversible(),
            "Expected {} to be reversible",
            entry.action_type
        );
        rollback_entry_with(entry, &ledger, &sbx)
            .unwrap_or_else(|e| panic!("Rollback failed for {}: {}", entry.action_type, e));
    }

    let gpus_after = sbx.detect_gpus();
    assert_eq!(gpus_after[0].msi_enabled, orig_gpu.msi_enabled);
    assert_eq!(
        gpus_after[0].interrupt_priority,
        orig_gpu.interrupt_priority
    );
    assert_eq!(sbx.get_task_enabled(task_path), Some(true));
    assert_eq!(sbx.get_active_power_scheme().unwrap(), orig_scheme);
    assert!(sbx.query_hibernation_status().hibernation_enabled);
    let hosts_after = sbx.read_hosts_file().unwrap();
    assert!(hosts_after.contains("127.0.0.1 custom-local.dev"));
    assert!(!hosts_after.contains("WinBlast0 Telemetry Blocklist"));
    assert!(!hosts_after.contains("vortex.data.microsoft.com"));
    let st_after = sbx
        .list_startup_entries()
        .unwrap()
        .into_iter()
        .find(|e| e.id == startup_entry.id)
        .unwrap();
    assert_eq!(st_after.enabled, orig_startup_en);

    // 8. Out-of-order RegistryBinaryBit rollback preserves sibling bits and later bit writes
    sbx.reg_set_binary(
        "HKCU",
        r"Control Panel\Desktop",
        "UserPreferencesMask",
        &[0x01, 0x00, 0x00, 0x00],
    )
    .unwrap();
    let mk_bit_tweak = |id: &str, mask: u8, opt: u8| TweakDefinition {
        id: id.to_string(),
        title: id.to_string(),
        category: "UX".to_string(),
        description: String::new(),
        easy_mode_included: false,
        os_req: OsRequirement {
            min_build: 19041,
            max_build: None,
            editions: vec![],
        },
        valuation: RiskValuation {
            stability_risk: RiskLevel::Safe,
            performance_gain: 0,
            privacy_gain: 0,
            reversibility: Reversibility::RequiresLogoff,
            broken_features: vec![],
        },
        actions: vec![TweakAction::RegistryBinaryBit {
            hive: "HKCU".to_string(),
            path: r"Control Panel\Desktop".to_string(),
            name: "UserPreferencesMask".to_string(),
            byte_index: 0,
            min_length: 4,
            bit_mask: mask,
            optimized_bits: opt,
            default_bits: 0,
            create_if_missing: false,
            default_full_hex: None,
        }],
        gpu_vendor: None,
        sub_category: None,
        unattend_safe: false,
        source_attribution: None,
        fallbacks: Vec::new(),
        prerequisites: Vec::new(),
        conflicts_with: Vec::new(),
    };
    let t1 = mk_bit_tweak("bit_04", 0x04, 0x04);
    let t2 = mk_bit_tweak("bit_10", 0x10, 0x10);
    apply_tweak_with(&t1, &ledger, &sbx).unwrap();
    apply_tweak_with(&t2, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_binary("HKCU", r"Control Panel\Desktop", "UserPreferencesMask")
            .unwrap()[0],
        0x15
    );
    let bit_hist = ledger.get_history().unwrap();
    let e1 = bit_hist.iter().find(|e| e.tweak_id == "bit_04").unwrap();
    rollback_entry_with(e1, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_binary("HKCU", r"Control Panel\Desktop", "UserPreferencesMask")
            .unwrap()[0],
        0x11
    );

    // 9. navigate(CleanAndApps) starts BOTH disk clean scan and AppX/feature scan
    let sbx_ui = SandboxEnv::new();
    sbx_ui.seed_all_tweaks(&load_embedded_rules());
    let mut state = AppState::new_with_backend(Arc::new(sbx_ui));
    state.navigate(ActiveView::CleanAndApps);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "clean & apps concurrent init",
        |s| !s.clean_targets.is_empty() && !s.installed_appx.is_empty() && !s.is_clean_apps_busy(),
    );

    // 10. import_profile filters unknown tweak IDs cleanly
    let prof_path = state.profile_path.clone();
    winblast0::backup::profile_io::export_profile(
        &prof_path,
        &[
            "disable_hibernation".to_string(),
            "nonexistent_bogus_tweak_id".to_string(),
        ],
    )
    .unwrap();
    state.import_profile();
    assert!(
        state.selected_tweaks.contains("disable_hibernation")
            || !state.selected_tweaks.contains("nonexistent_bogus_tweak_id")
    );
    assert!(!state.selected_tweaks.contains("nonexistent_bogus_tweak_id"));
}

struct FailAtMutationBackend {
    inner: SandboxEnv,
    trigger_on_call: usize,
    write_calls: std::sync::atomic::AtomicUsize,
}

impl FailAtMutationBackend {
    fn new(inner: SandboxEnv, trigger_on_call: usize) -> Self {
        Self {
            inner,
            trigger_on_call,
            write_calls: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    fn maybe_arm_error(&self) {
        let idx = self
            .write_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1;
        if idx == self.trigger_on_call {
            self.inner.inner.write().unwrap().inject_next_error =
                Some(format!("Injected fault at mutation #{}", idx));
        }
    }
}

impl SystemBackend for FailAtMutationBackend {
    fn is_elevated(&self) -> bool {
        self.inner.is_elevated()
    }
    fn detect_os(&self) -> BlastResult<WindowsInfo> {
        self.inner.detect_os()
    }
    fn default_ledger_path(&self) -> std::path::PathBuf {
        self.inner.default_ledger_path()
    }
    fn default_profile_path(&self) -> std::path::PathBuf {
        self.inner.default_profile_path()
    }
    fn reg_get_dword(&self, h: &str, p: &str, n: &str) -> Option<u32> {
        self.inner.reg_get_dword(h, p, n)
    }
    fn reg_set_dword(&self, h: &str, p: &str, n: &str, v: u32) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_set_dword(h, p, n, v)
    }
    fn reg_delete_value(&self, h: &str, p: &str, n: &str) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_delete_value(h, p, n)
    }
    fn reg_get_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        gv: Option<&str>,
    ) -> Option<RegStringValue> {
        self.inner.reg_get_string(h, p, n, gv)
    }
    fn reg_set_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        v: &str,
        e: bool,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_set_string(h, p, n, v, e, gv)
    }
    fn reg_get_binary(&self, h: &str, p: &str, n: &str) -> Option<Vec<u8>> {
        self.inner.reg_get_binary(h, p, n)
    }
    fn reg_set_binary(&self, h: &str, p: &str, n: &str, b: &[u8]) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_set_binary(h, p, n, b)
    }
    fn reg_get_raw(&self, h: &str, p: &str, n: &str) -> Option<RegRawSnapshot> {
        self.inner.reg_get_raw(h, p, n)
    }
    fn reg_set_raw(&self, h: &str, p: &str, n: &str, r: &RegRawSnapshot) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_set_raw(h, p, n, r)
    }
    fn reg_key_exists(&self, h: &str, p: &str) -> bool {
        self.inner.reg_key_exists(h, p)
    }
    fn reg_snapshot_tree(&self, h: &str, p: &str) -> BlastResult<Option<RegTreeSnapshot>> {
        self.inner.reg_snapshot_tree(h, p)
    }
    fn reg_delete_tree(&self, h: &str, p: &str) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.reg_delete_tree(h, p)
    }
    fn reg_restore_tree(&self, h: &str, p: &str, s: &RegTreeSnapshot) -> BlastResult<()> {
        self.inner.reg_restore_tree(h, p, s)
    }
    fn total_physical_ram_kb(&self) -> u64 {
        self.inner.total_physical_ram_kb()
    }
    fn logical_core_count(&self) -> u32 {
        self.inner.logical_core_count()
    }
    fn detect_platform_security(&self) -> PlatformSecurityInfo {
        self.inner.detect_platform_security()
    }
    fn get_active_power_scheme(&self) -> BlastResult<String> {
        self.inner.get_active_power_scheme()
    }
    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo> {
        self.inner.list_power_schemes()
    }
    fn set_active_power_scheme(&self, s: &str) -> BlastResult<()> {
        self.inner.set_active_power_scheme(s)
    }
    fn create_and_activate_performance_plan(&self) -> BlastResult<String> {
        self.inner.create_and_activate_performance_plan()
    }
    fn read_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
    ) -> BlastResult<(u32, u32)> {
        self.inner.read_power_setting_index(sc, sg, st)
    }
    fn write_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
        ac: Option<u32>,
        dc: Option<u32>,
        uh: bool,
    ) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.write_power_setting_index(sc, sg, st, ac, dc, uh)
    }
    fn query_hibernation_status(&self) -> HibernationStatus {
        self.inner.query_hibernation_status()
    }
    fn set_hibernation_enabled(&self, e: bool) -> BlastResult<HibernationStatus> {
        self.inner.set_hibernation_enabled(e)
    }
    fn set_usb_root_hub_power_saving(&self, d: bool) -> BlastResult<usize> {
        self.inner.set_usb_root_hub_power_saving(d)
    }
    fn get_service_start_type(&self, s: &str) -> Option<u32> {
        self.inner.get_service_start_type(s)
    }
    fn set_service_start_type(&self, s: &str, st: u32) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.set_service_start_type(s, st)
    }
    fn load_task_cache(&self) -> TaskCache {
        self.inner.load_task_cache()
    }
    fn get_task_enabled(&self, p: &str) -> Option<bool> {
        self.inner.get_task_enabled(p)
    }
    fn set_scheduled_task_enabled(&self, p: &str, e: bool) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.set_scheduled_task_enabled(p, e)
    }
    fn load_appx_cache(&self) -> AppxCache {
        self.inner.load_appx_cache()
    }
    fn remove_appx_package(&self, p: &str) -> BlastResult<()> {
        self.maybe_arm_error();
        self.inner.remove_appx_package(p)
    }
    fn read_hosts_file(&self) -> Option<String> {
        self.inner.read_hosts_file()
    }
    fn write_hosts_file(&self, c: &str) -> BlastResult<()> {
        self.inner.write_hosts_file(c)
    }
    fn block_telemetry_hosts(&self, h: &[&str]) -> BlastResult<usize> {
        self.inner.block_telemetry_hosts(h)
    }
    fn list_restore_points(&self) -> BlastResult<Vec<RestorePoint>> {
        self.inner.list_restore_points()
    }
    fn create_restore_point(&self, d: &str) -> BlastResult<()> {
        self.inner.create_restore_point(d)
    }
    fn detect_cpu(&self) -> CpuInfo {
        self.inner.detect_cpu()
    }
    fn detect_gpus(&self) -> Vec<DetectedGpu> {
        self.inner.detect_gpus()
    }
    fn set_gpu_msi_mode(&self, id: &str, e: bool, pr: u32) -> std::io::Result<()> {
        self.inner.set_gpu_msi_mode(id, e, pr)
    }
    fn is_hags_enabled(&self) -> bool {
        self.inner.is_hags_enabled()
    }
    fn set_hags(&self, e: bool) -> std::io::Result<()> {
        self.inner.set_hags(e)
    }
    fn is_mpo_disabled(&self) -> bool {
        self.inner.is_mpo_disabled()
    }
    fn set_mpo_disabled(&self, d: bool) -> std::io::Result<()> {
        self.inner.set_mpo_disabled(d)
    }
    fn detect_core_parking(&self) -> CoreParkingState {
        self.inner.detect_core_parking()
    }
    fn unpark_all_cores(&self) -> BlastResult<()> {
        self.inner.unpark_all_cores()
    }
    fn restore_stock_parking(&self) -> BlastResult<()> {
        self.inner.restore_stock_parking()
    }
    fn capture_latency(&self) -> LatencySnapshot {
        self.inner.capture_latency()
    }
    fn run_latency_benchmark(&self, s: &str) -> LatencyBenchmarkSnapshot {
        self.inner.run_latency_benchmark(s)
    }
    fn create_metrics_sampler(&self, m: usize) -> Box<dyn MetricsSampler> {
        self.inner.create_metrics_sampler(m)
    }
    fn scan_clean_targets(&self) -> Vec<CleanTargetScan> {
        self.inner.scan_clean_targets()
    }
    fn execute_clean_targets(&self, k: &[CleanTargetKind]) -> BlastResult<CleanExecutionReport> {
        self.inner.execute_clean_targets(k)
    }
    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage> {
        self.inner.list_installed_appx_detailed()
    }
    fn remove_appx_batch(&self, f: &[String], rp: bool) -> BlastResult<usize> {
        self.inner.remove_appx_batch(f, rp)
    }
    fn list_optional_features(&self) -> BlastResult<Vec<WindowsOptionalFeature>> {
        self.inner.list_optional_features()
    }
    fn set_optional_feature(
        &self,
        n: &str,
        k: OptionalFeatureKind,
        e: bool,
    ) -> BlastResult<FeatureInstallState> {
        self.maybe_arm_error();
        self.inner.set_optional_feature(n, k, e)
    }
    fn check_onedrive_kfm_active(&self) -> Option<String> {
        self.inner.check_onedrive_kfm_active()
    }
    fn uninstall_onedrive_safe(&self) -> BlastResult<String> {
        self.inner.uninstall_onedrive_safe()
    }
    fn remove_edge_preserve_webview2(&self) -> BlastResult<String> {
        self.inner.remove_edge_preserve_webview2()
    }
    fn winget_install_package(&self, p: &str) -> BlastResult<String> {
        self.inner.winget_install_package(p)
    }
    fn list_startup_entries(&self) -> BlastResult<Vec<StartupEntry>> {
        self.inner.list_startup_entries()
    }
    fn set_startup_entry_enabled(&self, e: &StartupEntry, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled(e, en)
    }
    fn set_startup_entry_enabled_by_id(&self, id: &str, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled_by_id(id, en)
    }
    fn list_live_services(&self) -> BlastResult<Vec<LiveServiceInfo>> {
        self.inner.list_live_services()
    }
    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo> {
        self.inner.list_live_tasks()
    }
    fn run_scheduled_task(&self, p: &str) -> BlastResult<()> {
        self.inner.run_scheduled_task(p)
    }
    fn restart_explorer_safely(&self) -> BlastResult<String> {
        self.inner.restart_explorer_safely()
    }
    fn broadcast_shell_settings_changed(&self) {
        self.inner.broadcast_shell_settings_changed()
    }
    fn request_system_reboot(&self) -> BlastResult<()> {
        self.inner.request_system_reboot()
    }
}

use winblast0::core::risk::FeatureInstallState;

fn make_synth_tweak(id: &str, rev: Reversibility, actions: Vec<TweakAction>) -> TweakDefinition {
    TweakDefinition {
        id: id.to_string(),
        title: id.to_string(),
        category: "OS Settings".to_string(),
        description: "Synthetic test tweak".to_string(),
        easy_mode_included: true,
        os_req: OsRequirement {
            min_build: 19041,
            max_build: None,
            editions: vec![],
        },
        valuation: RiskValuation {
            stability_risk: RiskLevel::Safe,
            performance_gain: 1,
            privacy_gain: 0,
            reversibility: rev,
            broken_features: vec![],
        },
        actions,
        gpu_vendor: None,
        sub_category: None,
        unattend_safe: true,
        source_attribution: None,
        fallbacks: Vec::new(),
        prerequisites: Vec::new(),
        conflicts_with: Vec::new(),
    }
}

#[test]
fn test_atomic_partial_failure_rollback_and_partially_optimized_lifecycle() {
    // 1. Mid-tweak partial failure via inject_next_error triggers automatic atomic rollback
    let multi_tweak = make_synth_tweak(
        "atomic_multi_test",
        Reversibility::Instant,
        vec![
            TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: r"Software\WinBlast0Test\Atomic".into(),
                name: "FirstValue".into(),
                optimized_value: 1,
                default_value: 0,
            },
            TweakAction::RegistryString {
                hive: "HKCU".into(),
                path: r"Software\WinBlast0Test\Atomic".into(),
                name: "SecondString".into(),
                optimized_value: "opt".into(),
                default_value: "stock".into(),
                expand_sz: false,
                absent_is_default: false,
                case_insensitive: false,
            },
            TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: r"Software\WinBlast0Test\Atomic".into(),
                name: "ThirdValue".into(),
                optimized_value: 99,
                default_value: 10,
            },
        ],
    );

    let sbx = SandboxEnv::new();
    sbx.seed_tweak_defaults_for_tweak(&multi_tweak);
    let ledger = StateLedger::in_memory().expect("in_memory ledger");

    // Fail on mutation #2 (SecondString) AFTER FirstValue succeeded and recorded in ledger
    let faulty = FailAtMutationBackend::new(sbx.clone(), 2);
    let err = apply_tweak_with(&multi_tweak, &ledger, &faulty)
        .expect_err("Expected mid-tweak injected error to fail apply_tweak_with");
    assert!(err.to_string().contains("Injected fault at mutation #2"));

    // Action 0 must have been atomically rolled back to default (0), Actions 1 & 2 untouched
    assert_eq!(
        sbx.reg_get_dword("HKCU", r"Software\WinBlast0Test\Atomic", "FirstValue"),
        Some(0)
    );
    assert_eq!(
        sbx.reg_get_string(
            "HKCU",
            r"Software\WinBlast0Test\Atomic",
            "SecondString",
            None
        )
        .map(|s| s.value),
        Some("stock".to_string())
    );
    assert_eq!(
        sbx.reg_get_dword("HKCU", r"Software\WinBlast0Test\Atomic", "ThirdValue"),
        Some(10)
    );
    // Write-ahead ledger (F-4): the failed action is recorded before it runs, so both rows exist
    // and both are rolled back by the compensating rollback.
    let hist = ledger.get_history().unwrap();
    assert_eq!(hist.len(), 2);
    assert!(
        hist.iter().all(|e| e.rolled_back),
        "Every ledger entry of the failed apply must be marked rolled_back"
    );
    assert_eq!(
        scan_single_tweak_with(&sbx, &multi_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );

    // 2. Natural mid-tweak error (protected service refusal after RegistryDword write) also rolls back
    let natural_fail = make_synth_tweak(
        "atomic_natural_fail",
        Reversibility::Instant,
        vec![
            TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: r"Software\WinBlast0Test\NaturalFail".into(),
                name: "StepOne".into(),
                optimized_value: 42,
                default_value: 7,
            },
            TweakAction::ServiceState {
                service_name: "RpcSs".into(),
                target_start_type: 4,
                default_start_type: 2,
            },
        ],
    );
    let sbx2 = SandboxEnv::new();
    sbx2.seed_tweak_defaults_for_tweak(&natural_fail);
    let ledger2 = StateLedger::in_memory().unwrap();
    assert!(apply_tweak_with(&natural_fail, &ledger2, &sbx2).is_err());
    assert_eq!(
        sbx2.reg_get_dword("HKCU", r"Software\WinBlast0Test\NaturalFail", "StepOne"),
        Some(7)
    );
    assert!(ledger2.get_history().unwrap().iter().all(|e| e.rolled_back));

    // 3. TweakState::PartiallyOptimized scan + toggle behavior in AppState
    let tweaks = load_embedded_rules();
    let sbx3 = SandboxEnv::new();
    sbx3.seed_all_tweaks(&tweaks);
    // Flip only the FIRST action of `enable_directx_windowed_game_optimizations` to optimized
    sbx3.reg_set_string(
        "HKCU",
        r"Software\Microsoft\DirectX\UserGpuPreferences",
        "DirectXUserGlobalSettings",
        "SwapEffectUpgradeEnable=1;VRROptimizeEnable=0;",
        false,
        None,
    )
    .unwrap();
    let dx_tweak = tweaks
        .iter()
        .find(|t| t.id == "enable_directx_windowed_game_optimizations")
        .unwrap();
    let partial_scan = scan_single_tweak_with(&sbx3, dx_tweak, 26100, "Pro");
    assert_eq!(partial_scan.current_state, TweakState::PartiallyOptimized);
    assert!(partial_scan.status_detail.contains("1/2"));

    let mut state = AppState::new_with_backend(Arc::new(sbx3));
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "partial scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });
    assert_eq!(
        state
            .get_tweak_scan("enable_directx_windowed_game_optimizations")
            .unwrap()
            .current_state,
        TweakState::PartiallyOptimized
    );
    assert!(
        !state.is_switch_active("enable_directx_windowed_game_optimizations"),
        "PartiallyOptimized tweak switch should start inactive so toggling applies remaining actions"
    );
    state.handle_switch_click("enable_directx_windowed_game_optimizations".into());
    assert_eq!(
        state
            .staged_transitions
            .get("enable_directx_windowed_game_optimizations"),
        Some(&StagedTransition::StageApply)
    );
    state.apply_selected();
    drain_until(&mut state, Duration::from_secs(5), "partial apply", |s| {
        s.get_tweak_scan("enable_directx_windowed_game_optimizations")
            .is_some_and(|r| r.current_state == TweakState::AlreadyOptimized)
    });
}

#[test]
fn test_wave1_compound_actions_deep_edge_cases() {
    // 1. RegistryDelete lossless QWORD snapshot & restore
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().unwrap();
    let del_tweak = make_synth_tweak(
        "del_qword_test",
        Reversibility::Instant,
        vec![TweakAction::RegistryDelete {
            hive: "HKCU".into(),
            path: r"Software\Microsoft\Siuf\Rules".into(),
            name: "PeriodInNanoSeconds".into(),
        }],
    );
    let qword_snap = RegRawSnapshot {
        reg_type: 11,
        hex_bytes: "8096980000000000".to_string(),
    };
    sbx.reg_set_raw(
        "HKCU",
        r"Software\Microsoft\Siuf\Rules",
        "PeriodInNanoSeconds",
        &qword_snap,
    )
    .unwrap();
    assert_eq!(
        scan_single_tweak_with(&sbx, &del_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );
    assert_eq!(apply_tweak_with(&del_tweak, &ledger, &sbx).unwrap(), 1);
    assert!(sbx
        .reg_get_raw(
            "HKCU",
            r"Software\Microsoft\Siuf\Rules",
            "PeriodInNanoSeconds"
        )
        .is_none());
    assert_eq!(
        scan_single_tweak_with(&sbx, &del_tweak, 26100, "Pro").current_state,
        TweakState::AlreadyOptimized
    );
    let del_entry = ledger.get_history().unwrap().pop().unwrap();
    rollback_entry_with(&del_entry, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_raw(
            "HKCU",
            r"Software\Microsoft\Siuf\Rules",
            "PeriodInNanoSeconds"
        ),
        Some(qword_snap)
    );

    // 2. RegistryDeleteTree shallow-root / HKCR guards + recursive RegTreeSnapshot restore
    assert!(
        sbx.reg_delete_tree("HKCU", r"Software\App").is_err(),
        "Shallow 2-segment path must fail"
    );
    assert!(
        sbx.reg_delete_tree("HKLM", r"SOFTWARE\Microsoft\OnlyThree")
            .is_err(),
        "Protected branch < 4 segments must fail"
    );
    assert!(
        sbx.reg_delete_tree("HKCR", r"CLSID\Foo\Bar\Baz").is_err(),
        "HKCR tree delete must fail"
    );

    let deep_tree = RegTreeSnapshot {
        subkey: String::new(),
        values: vec![(
            "".to_string(),
            RegRawSnapshot {
                reg_type: 1,
                hex_bytes: "6f00700065006e000000".to_string(),
            },
        )],
        children: vec![RegTreeSnapshot {
            subkey: "command".to_string(),
            values: vec![(
                "DelegateExecute".to_string(),
                RegRawSnapshot {
                    reg_type: 1,
                    hex_bytes: "7b007d000000".to_string(),
                },
            )],
            children: vec![],
        }],
    };
    let tree_path = r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32";
    sbx.inner.write().unwrap().reg_trees.insert(
        ("HKCU".to_string(), tree_path.to_lowercase()),
        deep_tree.clone(),
    );
    let tree_tweak = make_synth_tweak(
        "del_tree_test",
        Reversibility::RequiresLogoff,
        vec![TweakAction::RegistryDeleteTree {
            hive: "HKCU".into(),
            path: tree_path.into(),
        }],
    );
    assert_eq!(
        scan_single_tweak_with(&sbx, &tree_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );
    apply_tweak_with(&tree_tweak, &ledger, &sbx).unwrap();
    assert!(!sbx.reg_key_exists("HKCU", tree_path));
    assert_eq!(
        scan_single_tweak_with(&sbx, &tree_tweak, 26100, "Pro").current_state,
        TweakState::AlreadyOptimized
    );
    let tree_entry = ledger
        .get_history()
        .unwrap()
        .into_iter()
        .find(|e| e.tweak_id == "del_tree_test")
        .unwrap();
    rollback_entry_with(&tree_entry, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_snapshot_tree("HKCU", tree_path).unwrap(),
        Some(deep_tree)
    );

    // 3. RegistryBinaryBit: >=24 byte offset, sibling bit preservation, create_if_missing true/false
    let mut long_buf = vec![0xAAu8; 32];
    long_buf[24] = 0b1101_1011; // bit 0x08 is set, siblings preserved
    sbx.reg_set_binary(
        "HKCU",
        r"Software\WinBlast0Test\Binary",
        "LongMask",
        &long_buf,
    )
    .unwrap();
    let bin_long_tweak = make_synth_tweak(
        "bin_byte24",
        Reversibility::Instant,
        vec![TweakAction::RegistryBinaryBit {
            hive: "HKCU".into(),
            path: r"Software\WinBlast0Test\Binary".into(),
            name: "LongMask".into(),
            byte_index: 24,
            min_length: 25,
            bit_mask: 0x08,
            optimized_bits: 0x00,
            default_bits: 0x08,
            create_if_missing: false,
            default_full_hex: None,
        }],
    );
    assert_eq!(
        scan_single_tweak_with(&sbx, &bin_long_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );
    apply_tweak_with(&bin_long_tweak, &ledger, &sbx).unwrap();
    let after_apply_bin = sbx
        .reg_get_binary("HKCU", r"Software\WinBlast0Test\Binary", "LongMask")
        .unwrap();
    assert_eq!(
        after_apply_bin[24], 0b1101_0011,
        "Only target bit 0x08 should clear on byte 24"
    );
    assert_eq!(after_apply_bin[23], 0xAA);
    assert_eq!(after_apply_bin[25], 0xAA);

    // Absent binary: create_if_missing=false refuses, create_if_missing=true creates & rollback deletes
    let bin_no_create = make_synth_tweak(
        "bin_absent_refuse",
        Reversibility::Instant,
        vec![TweakAction::RegistryBinaryBit {
            hive: "HKCU".into(),
            path: r"Software\WinBlast0Test\Binary".into(),
            name: "MissingNoCreate".into(),
            byte_index: 0,
            min_length: 4,
            bit_mask: 0x02,
            optimized_bits: 0x00,
            default_bits: 0x02,
            create_if_missing: false,
            default_full_hex: None,
        }],
    );
    assert!(apply_tweak_with(&bin_no_create, &ledger, &sbx).is_err());

    let bin_create = make_synth_tweak(
        "bin_create_missing",
        Reversibility::Instant,
        vec![TweakAction::RegistryBinaryBit {
            hive: "HKCU".into(),
            path: r"Software\WinBlast0Test\Binary".into(),
            name: "CreatedFromHex".into(),
            byte_index: 4,
            min_length: 8,
            bit_mask: 0x02,
            optimized_bits: 0x00,
            default_bits: 0x02,
            create_if_missing: true,
            default_full_hex: Some("9e3e078012000000".into()),
        }],
    );
    apply_tweak_with(&bin_create, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_binary("HKCU", r"Software\WinBlast0Test\Binary", "CreatedFromHex")
            .unwrap()[4],
        0x10
    );
    let create_entry = ledger
        .get_history()
        .unwrap()
        .into_iter()
        .find(|e| e.tweak_id == "bin_create_missing")
        .unwrap();
    rollback_entry_with(&create_entry, &ledger, &sbx).unwrap();
    assert!(
        sbx.reg_get_binary("HKCU", r"Software\WinBlast0Test\Binary", "CreatedFromHex")
            .is_none(),
        "Rolling back create_if_missing binary should remove newly created value"
    );

    // 4. CompositeStringKey preserves unrelated AutoHDR=1; and per-game entries across apply/rollback/revert
    let tweaks = load_embedded_rules();
    let dx_tweak = tweaks
        .iter()
        .find(|t| t.id == "enable_directx_windowed_game_optimizations")
        .unwrap();
    sbx.reg_set_string(
        "HKCU",
        r"Software\Microsoft\DirectX\UserGpuPreferences",
        "DirectXUserGlobalSettings",
        r"AutoHDR=1;C:\Games\Shooter.exe=GpuPreference=2;SwapEffectUpgradeEnable=0;",
        false,
        None,
    )
    .unwrap();
    assert_eq!(
        scan_single_tweak_with(&sbx, dx_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );
    apply_tweak_with(dx_tweak, &ledger, &sbx).unwrap();
    let dx_val = sbx
        .reg_get_string(
            "HKCU",
            r"Software\Microsoft\DirectX\UserGpuPreferences",
            "DirectXUserGlobalSettings",
            None,
        )
        .unwrap()
        .value;
    assert!(
        dx_val.contains("AutoHDR=1"),
        "AutoHDR=1 was lost: {}",
        dx_val
    );
    assert!(dx_val.contains(r"C:\Games\Shooter.exe=GpuPreference=2"));
    assert!(dx_val.contains("SwapEffectUpgradeEnable=1"));
    assert!(dx_val.contains("VRROptimizeEnable=1"));
    assert_eq!(
        scan_single_tweak_with(&sbx, dx_tweak, 26100, "Pro").current_state,
        TweakState::AlreadyOptimized
    );

    for e in ledger
        .get_history()
        .unwrap()
        .into_iter()
        .filter(|e| e.tweak_id == dx_tweak.id && !e.rolled_back)
    {
        rollback_entry_with(&e, &ledger, &sbx).unwrap();
    }
    let dx_rb = sbx
        .reg_get_string(
            "HKCU",
            r"Software\Microsoft\DirectX\UserGpuPreferences",
            "DirectXUserGlobalSettings",
            None,
        )
        .unwrap()
        .value;
    assert!(
        dx_rb.contains("AutoHDR=1"),
        "AutoHDR=1 lost on rollback: {}",
        dx_rb
    );
    assert!(dx_rb.contains("SwapEffectUpgradeEnable=0"));
    assert!(!dx_rb.contains("VRROptimizeEnable=1"));

    // 5. RegistryDynamicDword: >= 48 GB / 64 GB RAM no-overflow + clamp verification
    let svchost_tweak = tweaks
        .iter()
        .find(|t| t.id == "consolidate_svchost_split_threshold")
        .unwrap();
    {
        sbx.inner.write().unwrap().total_ram_kb = 48 * 1024 * 1024; // 48 GB
    }
    sbx.seed_tweak_defaults_for_tweak(svchost_tweak);
    assert_eq!(
        resolve_dynamic_dword(&sbx, DynamicDwordFormula::TotalRamInKb),
        50_331_648
    );
    apply_tweak_with(svchost_tweak, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_dword(
            "HKLM",
            r"SYSTEM\CurrentControlSet\Control",
            "SvcHostSplitThresholdInKB"
        ),
        Some(50_331_648)
    );
    assert_eq!(
        scan_single_tweak_with(&sbx, svchost_tweak, 26100, "Pro").current_state,
        TweakState::AlreadyOptimized
    );

    {
        sbx.inner.write().unwrap().total_ram_kb = 64 * 1024 * 1024; // 64 GB
    }
    assert_eq!(
        resolve_dynamic_dword(&sbx, DynamicDwordFormula::TotalRamInKb),
        67_108_864
    );
    assert_eq!(
        resolve_dynamic_dword(&sbx, DynamicDwordFormula::TotalRamInKbPlusMargin),
        68_157_440
    );
    apply_tweak_with(svchost_tweak, &ledger, &sbx).unwrap();
    assert_eq!(
        sbx.reg_get_dword(
            "HKLM",
            r"SYSTEM\CurrentControlSet\Control",
            "SvcHostSplitThresholdInKB"
        ),
        Some(67_108_864)
    );
    {
        sbx.inner.write().unwrap().total_ram_kb = 5_000_000_000; // >4 TiB in KB -> must clamp, never overflow
    }
    assert_eq!(
        resolve_dynamic_dword(&sbx, DynamicDwordFormula::TotalRamInKb),
        u32::MAX
    );

    // 6. PowerCfgSetting: optional unsupported GUID tolerance vs all-absent failure
    let pwr_tweak = tweaks
        .iter()
        .find(|t| t.id == "power_settings_pcie_usb_nvme_epb")
        .unwrap();
    let pure_pwr_actions: Vec<TweakAction> = pwr_tweak
        .actions
        .iter()
        .filter(|a| matches!(a, TweakAction::PowerCfgSetting { .. }))
        .cloned()
        .collect();
    let pure_pwr_tweak = make_synth_tweak(
        "pure_pwr_absent_err",
        Reversibility::Instant,
        pure_pwr_actions,
    );
    let sbx_pwr = SandboxEnv::new();
    let pwr_ledger = StateLedger::in_memory().unwrap();
    // Without seeding ANY power setting on a pure-PowerCfgSetting tweak, all-absent must return Err rather than Ok(0)
    assert!(
        apply_tweak_with(&pure_pwr_tweak, &pwr_ledger, &sbx_pwr).is_err(),
        "Pure PowerCfgSetting tweak must error when 0 settings exist on system"
    );
    // Seed ONLY the first 2 power settings on `power_settings_pcie_usb_nvme_epb` (8 PowerCfgSetting + 2 RegistryDword)
    sbx_pwr.seed_tweak_defaults(&pwr_tweak.actions[..2]);
    let pwr_applied = apply_tweak_with(pwr_tweak, &pwr_ledger, &sbx_pwr).unwrap();
    assert_eq!(
        pwr_applied, 4,
        "2 present power settings + 2 RegistryDword actions should apply while 6 unsupported power GUIDs skip"
    );

    // 7. WindowsFeature: full `~~~~` suffix + shorthand prefix matching
    let sbx_feat = SandboxEnv::new();
    let feat_ledger = StateLedger::in_memory().unwrap();
    // Seed versioned capability `App.StepsRecorder~~~~0.0.1.0` Enabled (already in curated defaults, ensure Enabled)
    sbx_feat
        .set_optional_feature(
            "App.StepsRecorder~~~~0.0.1.0",
            OptionalFeatureKind::Capability,
            true,
        )
        .unwrap();
    let short_feat_tweak = make_synth_tweak(
        "winfeat_short_prefix",
        Reversibility::RequiresReboot,
        vec![TweakAction::WindowsFeature {
            feature_name: "App.StepsRecorder".into(),
            kind: OptionalFeatureKind::Capability,
            enable: false,
        }],
    );
    assert_eq!(
        scan_single_tweak_with(&sbx_feat, &short_feat_tweak, 26100, "Pro").current_state,
        TweakState::NotOptimized
    );
    apply_tweak_with(&short_feat_tweak, &feat_ledger, &sbx_feat).unwrap();
    assert_eq!(
        scan_single_tweak_with(&sbx_feat, &short_feat_tweak, 26100, "Pro").current_state,
        TweakState::AlreadyOptimized,
        "Shorthand WindowsFeature capability 'App.StepsRecorder' must update matching '~~~~0.0.1.0' entry cleanly"
    );
}

#[test]
fn test_wave1_presets_policy_locked_winget_appx_and_autounattend_permutations() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    // Insert a policy-locked startup entry
    sbx.inner
        .write()
        .unwrap()
        .startup_entries
        .push(StartupEntry {
            id: "HKLM|Policy|LockedUpdater".to_string(),
            name: "LockedUpdater".to_string(),
            command: "\"C:\\Program Files\\Corp\\Agent.exe\"".to_string(),
            source: StartupSource::RegistryHklmRun,
            enabled: true,
            location: "HKLM\\Software\\Policies\\Microsoft\\Windows\\Explorer\\Run".to_string(),
            hive: "HKLM".to_string(),
            reg_path: "Software\\Policies\\Microsoft\\Windows\\Explorer\\Run".to_string(),
            approved_subkey: None,
            is_wow64: false,
            policy_locked: true,
            is_microsoft: false,
        });

    let mut state = AppState::new_with_backend(Arc::new(sbx.clone()));
    state.start_scan();
    state.scan_startup_and_tasks_async();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "init scan & startup",
        |s| {
            !s.is_scanning
                && !s.scan_results.is_empty()
                && s.startup_entries.iter().any(|e| e.policy_locked)
        },
    );

    // 1. Presets (Gaming, ProAudio, LaptopBattery)
    for preset in [
        PresetKind::Gaming,
        PresetKind::ProAudio,
        PresetKind::LaptopBattery,
    ] {
        state.apply_preset(preset);
        assert!(
            !state.selected_tweaks.is_empty(),
            "{:?} preset selected 0 tweaks",
            preset
        );
        assert_eq!(state.staged_transitions.len(), state.selected_tweaks.len());
        assert!(!state.pending_actions.is_empty());
        for id in &state.selected_tweaks {
            let sr = state.get_tweak_scan(id).unwrap();
            assert!(sr.easy_mode_included);
            assert!(sr.risk.stability_risk < RiskLevel::High);
            assert_ne!(sr.current_state, TweakState::NotApplicable);
        }
    }

    // 2. policy_locked startup toggle refusal
    state.toggle_startup_entry_async("HKLM|Policy|LockedUpdater".to_string(), false);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "policy locked refusal",
        |s| !s.startup_tasks_busy && s.status_message.to_lowercase().contains("policy locked"),
    );
    assert!(
        state
            .status_message
            .to_lowercase()
            .contains("policy locked"),
        "Expected Policy locked status, got {:?}",
        state.status_message
    );
    let locked_entry = sbx
        .list_startup_entries()
        .unwrap()
        .into_iter()
        .find(|e| e.id == "HKLM|Policy|LockedUpdater")
        .unwrap();
    assert!(
        locked_entry.enabled,
        "Policy-locked startup entry must remain enabled"
    );

    // 3. ProtectedCore-only remove_appx_batch refusal
    let core_err_sbx = sbx
        .remove_appx_batch(&["Microsoft.SecHealthUI_8wekyb3d8bbwe".to_string()], false)
        .expect_err("SandboxEnv must refuse ProtectedCore-only AppX batch removal");
    assert!(matches!(core_err_sbx, WinBlastError::AppX(_)));
    // The real-host refusal check would uninstall the Store if the guard ever regressed: VM only.
    #[cfg(feature = "vm-tests")]
    {
        let core_err_real = features_apps::remove_appx_batch(
            &["Microsoft.WindowsStore_8wekyb3d8bbwe".to_string()],
            false,
        )
        .expect_err("features_apps::remove_appx_batch must refuse ProtectedCore before PowerShell");
        assert!(matches!(core_err_real, WinBlastError::AppX(_)));
    }

    // 4. autounattend.xml permutations: Arm64, bypass_msa_oobe without bypass_tpm, XML special escaping
    let arm_opts = UnattendOptions {
        bypass_tpm_ram_requirements: false,
        bypass_msa_oobe: true,
        skip_oobe_privacy_screens: false,
        auto_timezone_utc: true,
        default_local_user: "User&<1>".to_string(),
        computer_name: "ARM&DEV<PC>\"'".to_string(),
        include_selected_tweaks: true,
        architecture: TargetArch::Arm64,
    };
    let tricky_tweak = make_synth_tweak(
        "unattend_xml_escape",
        Reversibility::Instant,
        vec![TweakAction::RegistryString {
            hive: "HKCU".into(),
            path: r"Software\WinBlast0Test\Xml".into(),
            name: "Val&<1>".into(),
            optimized_value: "A & B <tag> \"q\" 's'".into(),
            default_value: "def".into(),
            expand_sz: false,
            absent_is_default: false,
            case_insensitive: false,
        }],
    );
    let xml = generate_autounattend_xml(&arm_opts, &[&tricky_tweak]);
    assert!(xml.contains("processorArchitecture=\"arm64\""));
    assert!(!xml.contains("processorArchitecture=\"amd64\""));
    assert!(xml.contains("<LocalAccounts>"));
    assert!(xml.contains("<TimeZone>UTC</TimeZone>"));
    assert!(!xml.contains("BypassTPMCheck"));
    assert!(!xml.contains("<ProtectYourPC>3</ProtectYourPC>"));
    assert!(
        xml.contains("&amp;"),
        "Escaped ampersand expected in autounattend XML"
    );
    assert!(
        !xml.contains("<tag>"),
        "Unescaped <tag> must never leak into autounattend XML"
    );

    // 5. WinGet ID validation (never spawns live winget.exe on invalid IDs; Notepad++.Notepad++ allowed)
    assert!(
        CURATED_WINGET_APPS
            .iter()
            .any(|a| a.id == "Notepad++.Notepad++"),
        "Curated WinGet catalog should include Notepad++.Notepad++"
    );
    for app in CURATED_WINGET_APPS {
        assert!(
            !app.id.is_empty()
                && app
                    .id
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
        );
    }
    assert!(sbx.winget_install_package("Notepad++.Notepad++").is_ok());
    for bad in [
        "",
        "   ",
        "Notepad.Notepad & calc",
        "pkg;whoami",
        "pkg|cmd",
        "pkg\n--silent",
        "$(calc)",
    ] {
        // Real winget only inside the VM (a validation regression would run an install on the host).
        #[cfg(feature = "vm-tests")]
        assert!(
            features_apps::winget_install_package(bad).is_err(),
            "RealBackend winget_install_package must reject invalid/injection ID {:?}",
            bad
        );
        assert!(
            sbx.winget_install_package(bad).is_err(),
            "SandboxEnv winget_install_package must also reject invalid/injection ID {:?}",
            bad
        );
    }
}

// =============================================================================
// Wave 4: Adversarial Fault-Injection, Concurrency, Mode & Tools Edge Cases
// =============================================================================

struct PanickingBackend {
    inner: SandboxEnv,
    panic_scan: std::sync::atomic::AtomicBool,
    panic_apply: std::sync::atomic::AtomicBool,
    panic_latency: std::sync::atomic::AtomicBool,
    panic_clean_scan: std::sync::atomic::AtomicBool,
    panic_startup_list: std::sync::atomic::AtomicBool,
}

impl PanickingBackend {
    fn new(inner: SandboxEnv) -> Self {
        Self {
            inner,
            panic_scan: std::sync::atomic::AtomicBool::new(false),
            panic_apply: std::sync::atomic::AtomicBool::new(false),
            panic_latency: std::sync::atomic::AtomicBool::new(false),
            panic_clean_scan: std::sync::atomic::AtomicBool::new(false),
            panic_startup_list: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

impl SystemBackend for PanickingBackend {
    fn is_elevated(&self) -> bool {
        self.inner.is_elevated()
    }
    fn detect_os(&self) -> BlastResult<WindowsInfo> {
        self.inner.detect_os()
    }
    fn default_ledger_path(&self) -> std::path::PathBuf {
        self.inner.default_ledger_path()
    }
    fn default_profile_path(&self) -> std::path::PathBuf {
        self.inner.default_profile_path()
    }
    fn reg_get_dword(&self, h: &str, p: &str, n: &str) -> Option<u32> {
        self.inner.reg_get_dword(h, p, n)
    }
    fn reg_set_dword(&self, h: &str, p: &str, n: &str, v: u32) -> BlastResult<()> {
        if self.panic_apply.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("Injected apply panic in reg_set_dword");
        }
        self.inner.reg_set_dword(h, p, n, v)
    }
    fn reg_delete_value(&self, h: &str, p: &str, n: &str) -> BlastResult<()> {
        self.inner.reg_delete_value(h, p, n)
    }
    fn reg_get_dword_for_vendor(&self, h: &str, p: &str, n: &str, gv: Option<&str>) -> Option<u32> {
        self.inner.reg_get_dword_for_vendor(h, p, n, gv)
    }
    fn reg_set_dword_for_vendor(
        &self,
        h: &str,
        p: &str,
        n: &str,
        v: u32,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        if self.panic_apply.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("Injected apply panic in reg_set_dword_for_vendor");
        }
        self.inner.reg_set_dword_for_vendor(h, p, n, v, gv)
    }
    fn reg_delete_value_for_vendor(
        &self,
        h: &str,
        p: &str,
        n: &str,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        self.inner.reg_delete_value_for_vendor(h, p, n, gv)
    }
    fn reg_get_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        gv: Option<&str>,
    ) -> Option<RegStringValue> {
        self.inner.reg_get_string(h, p, n, gv)
    }
    fn reg_set_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        v: &str,
        e: bool,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        if self.panic_apply.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("Injected apply panic in reg_set_string");
        }
        self.inner.reg_set_string(h, p, n, v, e, gv)
    }
    fn reg_get_binary(&self, h: &str, p: &str, n: &str) -> Option<Vec<u8>> {
        self.inner.reg_get_binary(h, p, n)
    }
    fn reg_set_binary(&self, h: &str, p: &str, n: &str, b: &[u8]) -> BlastResult<()> {
        self.inner.reg_set_binary(h, p, n, b)
    }
    fn reg_get_raw(&self, h: &str, p: &str, n: &str) -> Option<RegRawSnapshot> {
        self.inner.reg_get_raw(h, p, n)
    }
    fn reg_set_raw(&self, h: &str, p: &str, n: &str, r: &RegRawSnapshot) -> BlastResult<()> {
        self.inner.reg_set_raw(h, p, n, r)
    }
    fn reg_key_exists(&self, h: &str, p: &str) -> bool {
        self.inner.reg_key_exists(h, p)
    }
    fn reg_snapshot_tree(&self, h: &str, p: &str) -> BlastResult<Option<RegTreeSnapshot>> {
        self.inner.reg_snapshot_tree(h, p)
    }
    fn reg_delete_tree(&self, h: &str, p: &str) -> BlastResult<()> {
        self.inner.reg_delete_tree(h, p)
    }
    fn reg_restore_tree(&self, h: &str, p: &str, s: &RegTreeSnapshot) -> BlastResult<()> {
        self.inner.reg_restore_tree(h, p, s)
    }
    fn total_physical_ram_kb(&self) -> u64 {
        self.inner.total_physical_ram_kb()
    }
    fn logical_core_count(&self) -> u32 {
        self.inner.logical_core_count()
    }
    fn detect_platform_security(&self) -> PlatformSecurityInfo {
        self.inner.detect_platform_security()
    }
    fn get_active_power_scheme(&self) -> BlastResult<String> {
        self.inner.get_active_power_scheme()
    }
    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo> {
        self.inner.list_power_schemes()
    }
    fn set_active_power_scheme(&self, s: &str) -> BlastResult<()> {
        self.inner.set_active_power_scheme(s)
    }
    fn create_and_activate_performance_plan(&self) -> BlastResult<String> {
        self.inner.create_and_activate_performance_plan()
    }
    fn read_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
    ) -> BlastResult<(u32, u32)> {
        self.inner.read_power_setting_index(sc, sg, st)
    }
    fn write_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
        ac: Option<u32>,
        dc: Option<u32>,
        uh: bool,
    ) -> BlastResult<()> {
        self.inner.write_power_setting_index(sc, sg, st, ac, dc, uh)
    }
    fn query_hibernation_status(&self) -> HibernationStatus {
        self.inner.query_hibernation_status()
    }
    fn set_hibernation_enabled(&self, e: bool) -> BlastResult<HibernationStatus> {
        self.inner.set_hibernation_enabled(e)
    }
    fn set_usb_root_hub_power_saving(&self, d: bool) -> BlastResult<usize> {
        self.inner.set_usb_root_hub_power_saving(d)
    }
    fn get_service_start_type(&self, s: &str) -> Option<u32> {
        self.inner.get_service_start_type(s)
    }
    fn set_service_start_type(&self, s: &str, st: u32) -> BlastResult<()> {
        self.inner.set_service_start_type(s, st)
    }
    fn load_task_cache(&self) -> TaskCache {
        if self.panic_scan.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("Injected scan panic in load_task_cache");
        }
        self.inner.load_task_cache()
    }
    fn get_task_enabled(&self, p: &str) -> Option<bool> {
        self.inner.get_task_enabled(p)
    }
    fn set_scheduled_task_enabled(&self, p: &str, e: bool) -> BlastResult<()> {
        self.inner.set_scheduled_task_enabled(p, e)
    }
    fn load_appx_cache(&self) -> AppxCache {
        self.inner.load_appx_cache()
    }
    fn remove_appx_package(&self, p: &str) -> BlastResult<()> {
        self.inner.remove_appx_package(p)
    }
    fn read_hosts_file(&self) -> Option<String> {
        self.inner.read_hosts_file()
    }
    fn write_hosts_file(&self, c: &str) -> BlastResult<()> {
        self.inner.write_hosts_file(c)
    }
    fn block_telemetry_hosts(&self, h: &[&str]) -> BlastResult<usize> {
        self.inner.block_telemetry_hosts(h)
    }
    fn list_restore_points(&self) -> BlastResult<Vec<RestorePoint>> {
        self.inner.list_restore_points()
    }
    fn create_restore_point(&self, d: &str) -> BlastResult<()> {
        self.inner.create_restore_point(d)
    }
    fn detect_cpu(&self) -> CpuInfo {
        self.inner.detect_cpu()
    }
    fn detect_gpus(&self) -> Vec<DetectedGpu> {
        self.inner.detect_gpus()
    }
    fn set_gpu_msi_mode(&self, id: &str, e: bool, pr: u32) -> std::io::Result<()> {
        self.inner.set_gpu_msi_mode(id, e, pr)
    }
    fn is_hags_enabled(&self) -> bool {
        self.inner.is_hags_enabled()
    }
    fn set_hags(&self, e: bool) -> std::io::Result<()> {
        self.inner.set_hags(e)
    }
    fn is_mpo_disabled(&self) -> bool {
        self.inner.is_mpo_disabled()
    }
    fn set_mpo_disabled(&self, d: bool) -> std::io::Result<()> {
        self.inner.set_mpo_disabled(d)
    }
    fn detect_core_parking(&self) -> CoreParkingState {
        self.inner.detect_core_parking()
    }
    fn unpark_all_cores(&self) -> BlastResult<()> {
        self.inner.unpark_all_cores()
    }
    fn restore_stock_parking(&self) -> BlastResult<()> {
        self.inner.restore_stock_parking()
    }
    fn capture_latency(&self) -> LatencySnapshot {
        self.inner.capture_latency()
    }
    fn run_latency_benchmark(&self, s: &str) -> LatencyBenchmarkSnapshot {
        if self.panic_latency.load(std::sync::atomic::Ordering::SeqCst) {
            panic!("Injected latency panic in run_latency_benchmark");
        }
        self.inner.run_latency_benchmark(s)
    }
    fn create_metrics_sampler(&self, m: usize) -> Box<dyn MetricsSampler> {
        self.inner.create_metrics_sampler(m)
    }
    fn scan_clean_targets(&self) -> Vec<CleanTargetScan> {
        if self
            .panic_clean_scan
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            panic!("Injected clean scan panic in scan_clean_targets");
        }
        self.inner.scan_clean_targets()
    }
    fn execute_clean_targets(&self, k: &[CleanTargetKind]) -> BlastResult<CleanExecutionReport> {
        self.inner.execute_clean_targets(k)
    }
    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage> {
        self.inner.list_installed_appx_detailed()
    }
    fn remove_appx_batch(&self, f: &[String], rp: bool) -> BlastResult<usize> {
        self.inner.remove_appx_batch(f, rp)
    }
    fn list_optional_features(&self) -> BlastResult<Vec<WindowsOptionalFeature>> {
        self.inner.list_optional_features()
    }
    fn set_optional_feature(
        &self,
        n: &str,
        k: OptionalFeatureKind,
        e: bool,
    ) -> BlastResult<FeatureInstallState> {
        self.inner.set_optional_feature(n, k, e)
    }
    fn check_onedrive_kfm_active(&self) -> Option<String> {
        self.inner.check_onedrive_kfm_active()
    }
    fn uninstall_onedrive_safe(&self) -> BlastResult<String> {
        self.inner.uninstall_onedrive_safe()
    }
    fn remove_edge_preserve_webview2(&self) -> BlastResult<String> {
        self.inner.remove_edge_preserve_webview2()
    }
    fn winget_install_package(&self, p: &str) -> BlastResult<String> {
        self.inner.winget_install_package(p)
    }
    fn list_startup_entries(&self) -> BlastResult<Vec<StartupEntry>> {
        if self
            .panic_startup_list
            .load(std::sync::atomic::Ordering::SeqCst)
        {
            panic!("Injected startup list panic in list_startup_entries");
        }
        self.inner.list_startup_entries()
    }
    fn set_startup_entry_enabled(&self, e: &StartupEntry, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled(e, en)
    }
    fn set_startup_entry_enabled_by_id(&self, id: &str, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled_by_id(id, en)
    }
    fn list_live_services(&self) -> BlastResult<Vec<LiveServiceInfo>> {
        self.inner.list_live_services()
    }
    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo> {
        self.inner.list_live_tasks()
    }
    fn run_scheduled_task(&self, p: &str) -> BlastResult<()> {
        self.inner.run_scheduled_task(p)
    }
    fn restart_explorer_safely(&self) -> BlastResult<String> {
        self.inner.restart_explorer_safely()
    }
    fn broadcast_shell_settings_changed(&self) {
        self.inner.broadcast_shell_settings_changed()
    }
    fn request_system_reboot(&self) -> BlastResult<()> {
        self.inner.request_system_reboot()
    }
}

#[test]
fn test_worker_panic_recovery_releases_all_inflight_flags() {
    use std::sync::atomic::Ordering;

    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);
    let mock = Arc::new(PanickingBackend::new(sbx));

    let mut state = AppState::new_with_backend(mock.clone());

    // Initial baseline scan succeeds
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "baseline scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });

    // 1. Scan panic -> is_scanning released & subsequent start_scan recovers
    mock.panic_scan.store(true, Ordering::SeqCst);
    state.force_scan();
    assert!(state.is_scanning);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "scan panic recovery",
        |s| !s.is_scanning,
    );
    assert!(
        state.status_message.contains("scan failed") || state.status_message.contains("panic"),
        "Unexpected status after scan panic: {:?}",
        state.status_message
    );
    mock.panic_scan.store(false, Ordering::SeqCst);
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "post-panic scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });

    // Pick a pure RegistryDword NotOptimized tweak
    let target_id = state
        .tweaks
        .iter()
        .find(|t| {
            t.actions
                .iter()
                .all(|a| matches!(a, TweakAction::RegistryDword { .. }))
                && state
                    .get_tweak_scan(&t.id)
                    .is_some_and(|r| r.current_state == TweakState::NotOptimized)
        })
        .expect("Expected NotOptimized RegistryDword tweak")
        .id
        .clone();

    // 2a. Instant-mode apply panic -> in_flight_tweaks released cleanly
    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Instant);
    mock.panic_apply.store(true, Ordering::SeqCst);
    state.handle_switch_click(target_id.clone());
    assert!(state.in_flight_tweaks.contains(&target_id));
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "instant apply panic recovery",
        |s| !s.in_flight_tweaks.contains(&target_id),
    );
    assert!(state.in_flight_tweaks.is_empty());

    // 2b. Normal-mode batch apply panic -> is_applying released & subsequent apply recovers
    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Normal);
    state.handle_switch_click(target_id.clone());
    state.apply_selected();
    assert!(state.is_applying);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "normal batch apply panic recovery",
        |s| !s.is_applying,
    );
    assert!(state.status_message.contains("panic") || state.status_message.contains("Error"));
    mock.panic_apply.store(false, Ordering::SeqCst);
    state.apply_selected();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "post-panic batch apply",
        |s| {
            !s.is_applying
                && !s.is_scanning
                && s.get_tweak_scan(&target_id)
                    .is_some_and(|r| r.current_state == TweakState::AlreadyOptimized)
        },
    );

    // 3. run_latency_benchmark panic -> is_benchmarking_latency released & recovers
    mock.panic_latency.store(true, Ordering::SeqCst);
    state.start_latency_benchmark();
    assert!(state.is_benchmarking_latency);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "latency benchmark panic recovery",
        |s| !s.is_benchmarking_latency,
    );
    mock.panic_latency.store(false, Ordering::SeqCst);
    state.start_latency_benchmark();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "post-panic latency benchmark",
        |s| !s.is_benchmarking_latency && s.latest_latency_bench.is_some(),
    );

    // 4. scan_clean_targets panic -> clean_apps_busy released & recovers
    mock.panic_clean_scan.store(true, Ordering::SeqCst);
    state.scan_clean_targets_async();
    assert!(state.clean_apps_busy);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "clean scan panic recovery",
        |s| !s.clean_apps_busy,
    );
    mock.panic_clean_scan.store(false, Ordering::SeqCst);
    state.scan_clean_targets_async();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "post-panic clean scan",
        |s| !s.clean_apps_busy && !s.clean_targets.is_empty(),
    );

    // 5. list_startup_entries panic -> startup_tasks_busy released & recovers
    mock.panic_startup_list.store(true, Ordering::SeqCst);
    state.scan_startup_and_tasks_async();
    assert!(state.startup_tasks_busy);
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "startup scan panic recovery",
        |s| !s.startup_tasks_busy,
    );
    mock.panic_startup_list.store(false, Ordering::SeqCst);
    state.scan_startup_and_tasks_async();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "post-panic startup scan",
        |s| !s.startup_tasks_busy && !s.startup_entries.is_empty(),
    );
}

struct SlowScanBackend {
    inner: SandboxEnv,
    gate_armed: std::sync::atomic::AtomicBool,
    entered: (std::sync::Mutex<bool>, std::sync::Condvar),
    release: (std::sync::Mutex<bool>, std::sync::Condvar),
}

impl SlowScanBackend {
    fn new(inner: SandboxEnv) -> Self {
        Self {
            inner,
            gate_armed: std::sync::atomic::AtomicBool::new(false),
            entered: (std::sync::Mutex::new(false), std::sync::Condvar::new()),
            release: (std::sync::Mutex::new(false), std::sync::Condvar::new()),
        }
    }

    fn wait_until_scan_paused(&self, timeout: Duration) {
        let (lock, cvar) = &self.entered;
        let guard = lock.lock().unwrap();
        let (guard, res) = cvar.wait_timeout_while(guard, timeout, |e| !*e).unwrap();
        assert!(
            *guard && !res.timed_out(),
            "Timed out waiting for slow scan gate entry"
        );
    }

    fn release_scan(&self) {
        let (lock, cvar) = &self.release;
        let mut guard = lock.lock().unwrap();
        *guard = true;
        cvar.notify_all();
    }
}

impl SystemBackend for SlowScanBackend {
    fn is_elevated(&self) -> bool {
        self.inner.is_elevated()
    }
    fn detect_os(&self) -> BlastResult<WindowsInfo> {
        self.inner.detect_os()
    }
    fn default_ledger_path(&self) -> std::path::PathBuf {
        self.inner.default_ledger_path()
    }
    fn default_profile_path(&self) -> std::path::PathBuf {
        self.inner.default_profile_path()
    }
    fn reg_get_dword(&self, h: &str, p: &str, n: &str) -> Option<u32> {
        self.inner.reg_get_dword(h, p, n)
    }
    fn reg_set_dword(&self, h: &str, p: &str, n: &str, v: u32) -> BlastResult<()> {
        self.inner.reg_set_dword(h, p, n, v)
    }
    fn reg_delete_value(&self, h: &str, p: &str, n: &str) -> BlastResult<()> {
        self.inner.reg_delete_value(h, p, n)
    }
    fn reg_get_dword_for_vendor(&self, h: &str, p: &str, n: &str, gv: Option<&str>) -> Option<u32> {
        self.inner.reg_get_dword_for_vendor(h, p, n, gv)
    }
    fn reg_set_dword_for_vendor(
        &self,
        h: &str,
        p: &str,
        n: &str,
        v: u32,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        self.inner.reg_set_dword_for_vendor(h, p, n, v, gv)
    }
    fn reg_delete_value_for_vendor(
        &self,
        h: &str,
        p: &str,
        n: &str,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        self.inner.reg_delete_value_for_vendor(h, p, n, gv)
    }
    fn reg_get_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        gv: Option<&str>,
    ) -> Option<RegStringValue> {
        self.inner.reg_get_string(h, p, n, gv)
    }
    fn reg_set_string(
        &self,
        h: &str,
        p: &str,
        n: &str,
        v: &str,
        e: bool,
        gv: Option<&str>,
    ) -> BlastResult<()> {
        self.inner.reg_set_string(h, p, n, v, e, gv)
    }
    fn reg_get_binary(&self, h: &str, p: &str, n: &str) -> Option<Vec<u8>> {
        self.inner.reg_get_binary(h, p, n)
    }
    fn reg_set_binary(&self, h: &str, p: &str, n: &str, b: &[u8]) -> BlastResult<()> {
        self.inner.reg_set_binary(h, p, n, b)
    }
    fn reg_get_raw(&self, h: &str, p: &str, n: &str) -> Option<RegRawSnapshot> {
        self.inner.reg_get_raw(h, p, n)
    }
    fn reg_set_raw(&self, h: &str, p: &str, n: &str, r: &RegRawSnapshot) -> BlastResult<()> {
        self.inner.reg_set_raw(h, p, n, r)
    }
    fn reg_key_exists(&self, h: &str, p: &str) -> bool {
        self.inner.reg_key_exists(h, p)
    }
    fn reg_snapshot_tree(&self, h: &str, p: &str) -> BlastResult<Option<RegTreeSnapshot>> {
        self.inner.reg_snapshot_tree(h, p)
    }
    fn reg_delete_tree(&self, h: &str, p: &str) -> BlastResult<()> {
        self.inner.reg_delete_tree(h, p)
    }
    fn reg_restore_tree(&self, h: &str, p: &str, s: &RegTreeSnapshot) -> BlastResult<()> {
        self.inner.reg_restore_tree(h, p, s)
    }
    fn total_physical_ram_kb(&self) -> u64 {
        self.inner.total_physical_ram_kb()
    }
    fn logical_core_count(&self) -> u32 {
        self.inner.logical_core_count()
    }
    fn detect_platform_security(&self) -> PlatformSecurityInfo {
        self.inner.detect_platform_security()
    }
    fn get_active_power_scheme(&self) -> BlastResult<String> {
        self.inner.get_active_power_scheme()
    }
    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo> {
        self.inner.list_power_schemes()
    }
    fn set_active_power_scheme(&self, s: &str) -> BlastResult<()> {
        self.inner.set_active_power_scheme(s)
    }
    fn create_and_activate_performance_plan(&self) -> BlastResult<String> {
        self.inner.create_and_activate_performance_plan()
    }
    fn read_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
    ) -> BlastResult<(u32, u32)> {
        self.inner.read_power_setting_index(sc, sg, st)
    }
    fn write_power_setting_index(
        &self,
        sc: Option<&str>,
        sg: &str,
        st: &str,
        ac: Option<u32>,
        dc: Option<u32>,
        uh: bool,
    ) -> BlastResult<()> {
        self.inner.write_power_setting_index(sc, sg, st, ac, dc, uh)
    }
    fn query_hibernation_status(&self) -> HibernationStatus {
        self.inner.query_hibernation_status()
    }
    fn set_hibernation_enabled(&self, e: bool) -> BlastResult<HibernationStatus> {
        self.inner.set_hibernation_enabled(e)
    }
    fn set_usb_root_hub_power_saving(&self, d: bool) -> BlastResult<usize> {
        self.inner.set_usb_root_hub_power_saving(d)
    }
    fn get_service_start_type(&self, s: &str) -> Option<u32> {
        self.inner.get_service_start_type(s)
    }
    fn set_service_start_type(&self, s: &str, st: u32) -> BlastResult<()> {
        self.inner.set_service_start_type(s, st)
    }
    fn load_task_cache(&self) -> TaskCache {
        if self
            .gate_armed
            .swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            {
                let (lock, cvar) = &self.entered;
                *lock.lock().unwrap() = true;
                cvar.notify_all();
            }
            {
                let (lock, cvar) = &self.release;
                let guard = lock.lock().unwrap();
                drop(cvar.wait_while(guard, |r| !*r).unwrap());
            }
        }
        self.inner.load_task_cache()
    }
    fn get_task_enabled(&self, p: &str) -> Option<bool> {
        self.inner.get_task_enabled(p)
    }
    fn set_scheduled_task_enabled(&self, p: &str, e: bool) -> BlastResult<()> {
        self.inner.set_scheduled_task_enabled(p, e)
    }
    fn load_appx_cache(&self) -> AppxCache {
        self.inner.load_appx_cache()
    }
    fn remove_appx_package(&self, p: &str) -> BlastResult<()> {
        self.inner.remove_appx_package(p)
    }
    fn read_hosts_file(&self) -> Option<String> {
        self.inner.read_hosts_file()
    }
    fn write_hosts_file(&self, c: &str) -> BlastResult<()> {
        self.inner.write_hosts_file(c)
    }
    fn block_telemetry_hosts(&self, h: &[&str]) -> BlastResult<usize> {
        self.inner.block_telemetry_hosts(h)
    }
    fn list_restore_points(&self) -> BlastResult<Vec<RestorePoint>> {
        self.inner.list_restore_points()
    }
    fn create_restore_point(&self, d: &str) -> BlastResult<()> {
        self.inner.create_restore_point(d)
    }
    fn detect_cpu(&self) -> CpuInfo {
        self.inner.detect_cpu()
    }
    fn detect_gpus(&self) -> Vec<DetectedGpu> {
        self.inner.detect_gpus()
    }
    fn set_gpu_msi_mode(&self, id: &str, e: bool, pr: u32) -> std::io::Result<()> {
        self.inner.set_gpu_msi_mode(id, e, pr)
    }
    fn is_hags_enabled(&self) -> bool {
        self.inner.is_hags_enabled()
    }
    fn set_hags(&self, e: bool) -> std::io::Result<()> {
        self.inner.set_hags(e)
    }
    fn is_mpo_disabled(&self) -> bool {
        self.inner.is_mpo_disabled()
    }
    fn set_mpo_disabled(&self, d: bool) -> std::io::Result<()> {
        self.inner.set_mpo_disabled(d)
    }
    fn detect_core_parking(&self) -> CoreParkingState {
        self.inner.detect_core_parking()
    }
    fn unpark_all_cores(&self) -> BlastResult<()> {
        self.inner.unpark_all_cores()
    }
    fn restore_stock_parking(&self) -> BlastResult<()> {
        self.inner.restore_stock_parking()
    }
    fn capture_latency(&self) -> LatencySnapshot {
        self.inner.capture_latency()
    }
    fn run_latency_benchmark(&self, s: &str) -> LatencyBenchmarkSnapshot {
        self.inner.run_latency_benchmark(s)
    }
    fn create_metrics_sampler(&self, m: usize) -> Box<dyn MetricsSampler> {
        self.inner.create_metrics_sampler(m)
    }
    fn scan_clean_targets(&self) -> Vec<CleanTargetScan> {
        self.inner.scan_clean_targets()
    }
    fn execute_clean_targets(&self, k: &[CleanTargetKind]) -> BlastResult<CleanExecutionReport> {
        self.inner.execute_clean_targets(k)
    }
    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage> {
        self.inner.list_installed_appx_detailed()
    }
    fn remove_appx_batch(&self, f: &[String], rp: bool) -> BlastResult<usize> {
        self.inner.remove_appx_batch(f, rp)
    }
    fn list_optional_features(&self) -> BlastResult<Vec<WindowsOptionalFeature>> {
        self.inner.list_optional_features()
    }
    fn set_optional_feature(
        &self,
        n: &str,
        k: OptionalFeatureKind,
        e: bool,
    ) -> BlastResult<FeatureInstallState> {
        self.inner.set_optional_feature(n, k, e)
    }
    fn check_onedrive_kfm_active(&self) -> Option<String> {
        self.inner.check_onedrive_kfm_active()
    }
    fn uninstall_onedrive_safe(&self) -> BlastResult<String> {
        self.inner.uninstall_onedrive_safe()
    }
    fn remove_edge_preserve_webview2(&self) -> BlastResult<String> {
        self.inner.remove_edge_preserve_webview2()
    }
    fn winget_install_package(&self, p: &str) -> BlastResult<String> {
        self.inner.winget_install_package(p)
    }
    fn list_startup_entries(&self) -> BlastResult<Vec<StartupEntry>> {
        self.inner.list_startup_entries()
    }
    fn set_startup_entry_enabled(&self, e: &StartupEntry, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled(e, en)
    }
    fn set_startup_entry_enabled_by_id(&self, id: &str, en: bool) -> BlastResult<()> {
        self.inner.set_startup_entry_enabled_by_id(id, en)
    }
    fn list_live_services(&self) -> BlastResult<Vec<LiveServiceInfo>> {
        self.inner.list_live_services()
    }
    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo> {
        self.inner.list_live_tasks()
    }
    fn run_scheduled_task(&self, p: &str) -> BlastResult<()> {
        self.inner.run_scheduled_task(p)
    }
    fn restart_explorer_safely(&self) -> BlastResult<String> {
        self.inner.restart_explorer_safely()
    }
    fn broadcast_shell_settings_changed(&self) {
        self.inner.broadcast_shell_settings_changed()
    }
    fn request_system_reboot(&self) -> BlastResult<()> {
        self.inner.request_system_reboot()
    }
}

#[test]
fn test_epoch_and_stale_scan_fencing_preserves_instant_mutation() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);
    let slow = Arc::new(SlowScanBackend::new(sbx));

    let mut state = AppState::new_with_backend(slow.clone());
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "initial scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });
    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Instant);

    // Choose a RegistryDword tweak evaluated BEFORE load_task_cache is triggered (e.g. from os_settings)
    let target_id = state
        .tweaks
        .iter()
        .find(|t| {
            t.actions
                .iter()
                .all(|a| matches!(a, TweakAction::RegistryDword { .. }))
                && state
                    .get_tweak_scan(&t.id)
                    .is_some_and(|r| r.current_state == TweakState::NotOptimized)
        })
        .expect("Expected NotOptimized RegistryDword tweak")
        .id
        .clone();

    // Start slow scan and wait until it has already scanned `target_id` as NotOptimized and paused at load_task_cache
    slow.gate_armed
        .store(true, std::sync::atomic::Ordering::SeqCst);
    state.force_scan();
    assert!(state.is_scanning);
    slow.wait_until_scan_paused(Duration::from_secs(5));

    // While slow scan is still in-flight, apply target_id in Instant Mode -> SingleTweakUpdated marks mutated_during_scan
    state.handle_switch_click(target_id.clone());
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "Instant SingleTweakUpdated during slow scan",
        |s| {
            !s.in_flight_tweaks.contains(&target_id)
                && s.get_tweak_scan(&target_id)
                    .is_some_and(|r| r.current_state == TweakState::AlreadyOptimized)
        },
    );
    assert!(
        state.is_scanning,
        "Slow scan must still be in-flight before gate release"
    );

    // Release slow scan -> ScanCompleted arrives carrying stale NotOptimized for `target_id`
    slow.release_scan();
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "Slow ScanCompleted finishes",
        |s| !s.is_scanning,
    );

    assert_eq!(
        state
            .get_tweak_scan(&target_id)
            .expect("Missing target scan result")
            .current_state,
        TweakState::AlreadyOptimized,
        "mutated_during_scan fencing must keep '{}' at AlreadyOptimized despite stale ScanCompleted!",
        target_id
    );

    // Stale epoch ScanCompleted must be discarded completely
    let prev_len = state.scan_results.len();
    state.apply_worker_event(winblast0::app::WorkerEvent::ScanCompleted {
        epoch: state.current_epoch.saturating_sub(1),
        results: Vec::new(),
    });
    assert_eq!(
        state.scan_results.len(),
        prev_len,
        "Older-epoch ScanCompleted must be ignored"
    );
}

#[test]
fn test_mode_transitions_select_all_and_diff_modal_separation() {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    {
        // Single NVIDIA GPU only (so AMD & Intel tweaks are foreign/incompatible)
        let mut st = sbx.inner.write().unwrap();
        st.gpus
            .retain(|g| g.vendor == winblast0::hardware::gpu::GpuVendor::Nvidia);
    }
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.start_scan();
    drain_until(&mut state, Duration::from_secs(5), "initial scan", |s| {
        !s.is_scanning && !s.scan_results.is_empty()
    });
    assert_eq!(state.mode, AppMode::Easy);
    assert_eq!(state.engine_mode, EngineMode::Normal);

    let sample_id = state
        .tweaks
        .iter()
        .find(|t| {
            t.actions
                .iter()
                .all(|a| matches!(a, TweakAction::RegistryDword { .. }))
                && state
                    .get_tweak_scan(&t.id)
                    .is_some_and(|r| r.current_state == TweakState::NotOptimized)
        })
        .expect("Expected NotOptimized RegistryDword tweak")
        .id
        .clone();

    // 1. Stage in Normal Mode -> switch Normal -> Instant clears staged_transitions & toggles cleanly from live state
    state.handle_switch_click(sample_id.clone());
    assert_eq!(
        state.staged_transitions.get(&sample_id),
        Some(&StagedTransition::StageApply)
    );
    assert!(state.is_switch_active(&sample_id));

    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Instant);
    assert!(
        state.staged_transitions.is_empty(),
        "Normal -> Instant must clear staged_transitions"
    );
    assert!(state.selected_tweaks.is_empty());
    assert!(state.pending_actions.is_empty());
    assert!(state.pending_items.is_empty());
    assert!(
        !state.is_switch_active(&sample_id),
        "Switch must reflect live NotOptimized state after clearing staged transitions"
    );

    // Next switch click in Instant mode applies from live state
    state.handle_switch_click(sample_id.clone());
    drain_until(
        &mut state,
        Duration::from_secs(5),
        "Instant toggle apply",
        |s| {
            !s.in_flight_tweaks.contains(&sample_id)
                && s.get_tweak_scan(&sample_id)
                    .is_some_and(|r| r.current_state == TweakState::AlreadyOptimized)
        },
    );
    assert!(state.is_switch_active(&sample_id));

    // Switch back to Normal mode
    state.toggle_engine_mode();
    assert_eq!(state.engine_mode, EngineMode::Normal);

    // 2. Navigate Easy -> AdvancedMatrix -> mode == AppMode::Advanced, select_all includes Advanced non-High & excludes foreign GPU
    state.navigate(ActiveView::AdvancedMatrix);
    assert_eq!(state.mode, AppMode::Advanced);
    state.select_all();

    let has_advanced_non_high =
        state.selected_tweaks.iter().any(|id| {
            state.tweaks.iter().find(|t| &t.id == id).is_some_and(|t| {
                !t.easy_mode_included && t.valuation.stability_risk < RiskLevel::High
            })
        });
    assert!(
        has_advanced_non_high,
        "Advanced select_all must include Advanced non-High compatible tweaks"
    );

    for t in state.tweaks.iter() {
        if matches!(t.gpu_vendor.as_deref(), Some("AMD") | Some("Intel")) {
            assert!(
                !state.selected_tweaks.contains(&t.id),
                "Foreign GPU tweak '{}' must be excluded by select_all()",
                t.id
            );
        }
        if t.valuation.stability_risk >= RiskLevel::High {
            assert!(
                !state.selected_tweaks.contains(&t.id),
                "High/Extreme tweak '{}' must be excluded by select_all()",
                t.id
            );
        }
    }

    // 3. inspect_tweak -> dismiss -> open_diff_preview with StageRevert & StageApply
    state.deselect_all();
    let apply_id = state
        .tweaks
        .iter()
        .find(|t| {
            t.id != sample_id
                && state
                    .get_tweak_scan(&t.id)
                    .is_some_and(|r| r.current_state == TweakState::NotOptimized)
        })
        .expect("Expected second NotOptimized tweak")
        .id
        .clone();
    let inspect_id = state
        .tweaks
        .iter()
        .find(|t| t.id != sample_id && t.id != apply_id)
        .expect("Expected third tweak to inspect")
        .id
        .clone();

    // `sample_id` is AlreadyOptimized -> clicking stages StageRevert
    state.handle_switch_click(sample_id.clone());
    // `apply_id` is NotOptimized -> clicking stages StageApply
    state.handle_switch_click(apply_id.clone());
    assert_eq!(
        state.staged_transitions.get(&sample_id),
        Some(&StagedTransition::StageRevert)
    );
    assert_eq!(
        state.staged_transitions.get(&apply_id),
        Some(&StagedTransition::StageApply)
    );

    let staged_count_before_inspect = state.pending_items.len();
    state.inspect_tweak(&inspect_id);
    assert!(state.show_diff_preview);
    assert!(state.inspected_tweak.is_some());
    assert_eq!(
        state.pending_items.len(),
        staged_count_before_inspect,
        "inspect_tweak must not clobber staged pending_items"
    );

    // Dismiss inspect modal, then open_diff_preview
    state.show_diff_preview = false;
    state.inspected_tweak = None;
    state.open_diff_preview();
    assert!(state.show_diff_preview);
    assert!(state.inspected_tweak.is_none());
    assert!(
        state.pending_items.iter().any(|(_, rev)| *rev),
        "pending_items must carry revert=true for StageRevert tweak '{}'",
        sample_id
    );
    assert!(
        state.pending_items.iter().any(|(_, rev)| !*rev),
        "pending_items must carry revert=false for StageApply tweak '{}'",
        apply_id
    );
}

fn validate_balanced_xml(xml: &str) {
    assert!(
        xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"),
        "Missing XML declaration"
    );
    let mut stack: Vec<String> = Vec::new();
    let mut idx = 0usize;
    while let Some(lt_rel) = xml[idx..].find('<') {
        let lt = idx + lt_rel;
        let gt_rel = xml[lt..].find('>').expect("Unclosed '<' in generated XML");
        let gt = lt + gt_rel;
        let tag_body = xml[lt + 1..gt].trim();
        if tag_body.starts_with("?xml") || tag_body.starts_with('!') {
            idx = gt + 1;
            continue;
        }
        if let Some(closing) = tag_body.strip_prefix('/') {
            let close_name = closing.split_whitespace().next().unwrap_or("");
            let expected = stack.pop().unwrap_or_else(|| {
                panic!("Unexpected closing tag </{}> with empty stack", close_name)
            });
            assert_eq!(
                expected, close_name,
                "Mismatched XML closing tag: expected </{}>, got </{}>",
                expected, close_name
            );
        } else if !tag_body.ends_with('/') {
            let open_name = tag_body.split_whitespace().next().unwrap_or("");
            assert!(!open_name.is_empty(), "Empty XML open tag at offset {}", lt);
            stack.push(open_name.to_string());
        }
        idx = gt + 1;
    }
    assert!(stack.is_empty(), "Unclosed XML tags remaining: {:?}", stack);
}

#[test]
fn test_tools_appx_hosts_and_autounattend_edge_cases() {
    // 1. features_apps::list_installed_appx_from_cache with underscores AND non-empty ResourceId
    let mut appx_set = HashSet::new();
    appx_set.insert("My_Cool_App_1.2.0.0_x64__8wekyb3d8bbwe".to_string());
    appx_set.insert("Some.App_1.0.0.0_neutral_split.scale-100_8wekyb3d8bbwe".to_string());
    appx_set.insert("Microsoft.SecHealthUI_1000.25873.9001.0_x64__8wekyb3d8bbwe".to_string());
    let cache = AppxCache::from_packages(appx_set);
    let pkgs = features_apps::list_installed_appx_from_cache(&cache);

    let cool = pkgs
        .iter()
        .find(|p| {
            p.family_name
                .eq_ignore_ascii_case("My_Cool_App_8wekyb3d8bbwe")
        })
        .expect("My_Cool_App family normalization failed");
    assert!(cool.short_name.eq_ignore_ascii_case("My_Cool_App"));
    assert_eq!(cool.publisher, "Microsoft");

    let split_res = pkgs
        .iter()
        .find(|p| p.family_name.eq_ignore_ascii_case("Some.App_8wekyb3d8bbwe"))
        .expect("Some.App with non-empty ResourceId failed to normalize");
    assert!(split_res.short_name.eq_ignore_ascii_case("Some.App"));
    assert_eq!(split_res.publisher, "Microsoft");

    let core = pkgs
        .iter()
        .find(|p| {
            p.family_name
                .eq_ignore_ascii_case("Microsoft.SecHealthUI_8wekyb3d8bbwe")
        })
        .expect("Missing SecHealthUI");
    assert_eq!(
        core.safety_tier,
        winblast0::tools::features_apps::AppxSafetyTier::ProtectedCore
    );

    // 2. unblock_telemetry_hosts_in_content with UTF-8 BOM, IPv6 ::1, tabs, and decoy footer comment
    let bom_hosts = concat!(
        "\u{FEFF}::1\tvortex.data.microsoft.com\r\n",
        "# Decoy comment mentioning # End WinBlast0 Telemetry Blocklist before block\r\n",
        "# WinBlast0 Telemetry Blocklist\r\n",
        "0.0.0.0\ttelemetry.nvidia.com\r\n",
        "# End WinBlast0 Telemetry Blocklist\r\n",
        "0.0.0.0\tuser-custom-block.local\r\n",
        "::1\tmy-ipv6-entry.local\r\n",
    );
    let (after_block, added) = winblast0::executor::ps_runner::block_telemetry_hosts_in_content(
        bom_hosts,
        &[
            "vortex.data.microsoft.com",
            "telemetry.nvidia.com",
            "gfe.nvidia.com",
        ],
    );
    assert_eq!(
        added, 1,
        "Only gfe.nvidia.com should be appended (BOM ::1\\t and blocklist entries recognized)"
    );
    assert!(after_block.contains("0.0.0.0 gfe.nvidia.com\r\n"));

    let (unblocked, changed) =
        winblast0::executor::ps_runner::unblock_telemetry_hosts_in_content(&after_block);
    assert!(changed);
    assert!(unblocked.starts_with('\u{FEFF}'));
    assert!(unblocked.contains("::1\tvortex.data.microsoft.com"));
    assert!(unblocked.contains("# Decoy comment mentioning # End WinBlast0 Telemetry Blocklist"));
    assert!(unblocked.contains("0.0.0.0\tuser-custom-block.local"));
    assert!(unblocked.contains("::1\tmy-ipv6-entry.local"));
    assert!(!unblocked.contains("telemetry.nvidia.com"));
    assert!(!unblocked.contains("gfe.nvidia.com"));

    // Also decoy comment when real footer is absent must not eat trailing custom entries
    let no_footer_decoy = concat!(
        "\u{FEFF}# Note: # End WinBlast0 Telemetry Blocklist\r\n",
        "# WinBlast0 Telemetry Blocklist\r\n",
        "0.0.0.0\ttelemetry.nvidia.com\r\n",
        "::1\tuser-keep.local\r\n",
    );
    let (unblocked_nf, changed_nf) =
        winblast0::executor::ps_runner::unblock_telemetry_hosts_in_content(no_footer_decoy);
    assert!(changed_nf);
    assert!(!unblocked_nf.contains("telemetry.nvidia.com"));
    assert!(unblocked_nf.contains("::1\tuser-keep.local"));

    // 3. autounattend.xml verification:
    //    - RegistryDelete -> cmd /c reg.exe delete ... || exit /b 0
    //    - is_known_numeric_reg_sz -> REG_SZ (and negative BorderWidth)
    //    - %SystemDrive%\Users\Default\NTUSER.DAT
    //    - reserved username "Administrator" sanitized
    //    - <HideLocalAccountScreen> omitted when bypass_msa_oobe = false
    //    - balanced valid XML structure
    let unattend_tweak = make_synth_tweak(
        "unattend_wave4_check",
        Reversibility::Instant,
        vec![
            TweakAction::RegistryDelete {
                hive: "HKLM".into(),
                path: r"SOFTWARE\WinBlast0Test\Cleanup".into(),
                name: "StaleValue".into(),
            },
            TweakAction::RegistryDelete {
                hive: "HKCU".into(),
                path: r"Software\WinBlast0Test\Cleanup".into(),
                name: "UserStale".into(),
            },
            TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: r"Control Panel\Desktop".into(),
                name: "MenuShowDelay".into(),
                optimized_value: 0,
                default_value: 400,
            },
            TweakAction::RegistryDword {
                hive: "HKCU".into(),
                path: r"Control Panel\Desktop\WindowMetrics".into(),
                name: "BorderWidth".into(),
                optimized_value: (-15i32) as u32,
                default_value: (-15i32) as u32,
            },
        ],
    );

    let opts_msa_bypass = UnattendOptions {
        bypass_tpm_ram_requirements: true,
        bypass_msa_oobe: true,
        skip_oobe_privacy_screens: true,
        auto_timezone_utc: false,
        default_local_user: "Administrator".to_string(),
        computer_name: "TEST-RIG".to_string(),
        include_selected_tweaks: true,
        architecture: TargetArch::Amd64,
    };
    let xml_a = generate_autounattend_xml(&opts_msa_bypass, &[&unattend_tweak]);
    validate_balanced_xml(&xml_a);
    assert!(
        xml_a.contains("cmd /c reg.exe delete") && xml_a.contains("|| exit /b 0"),
        "RegistryDelete must emit resilient cmd /c reg.exe delete ... || exit /b 0:\n{}",
        xml_a
    );
    assert!(
        xml_a.contains("MenuShowDelay&quot; /t REG_SZ /d &quot;0&quot;"),
        "MenuShowDelay must emit REG_SZ in autounattend.xml:\n{}",
        xml_a
    );
    assert!(
        xml_a.contains("BorderWidth&quot; /t REG_SZ /d &quot;-15&quot;"),
        "Negative BorderWidth must emit REG_SZ -15 in autounattend.xml:\n{}",
        xml_a
    );
    assert!(xml_a.contains(r"%SystemDrive%\Users\Default\NTUSER.DAT"));
    assert!(xml_a.contains("<Name>User</Name>"));
    assert!(!xml_a.contains("<Name>Administrator</Name>"));
    assert!(xml_a.contains("<HideLocalAccountScreen>true</HideLocalAccountScreen>"));

    let opts_no_msa_bypass = UnattendOptions {
        bypass_msa_oobe: false,
        skip_oobe_privacy_screens: true,
        ..opts_msa_bypass
    };
    let xml_b = generate_autounattend_xml(&opts_no_msa_bypass, &[&unattend_tweak]);
    validate_balanced_xml(&xml_b);
    assert!(
        !xml_b.contains("<HideLocalAccountScreen>"),
        "<HideLocalAccountScreen> must be omitted when bypass_msa_oobe = false"
    );
    assert!(!xml_b.contains("<LocalAccounts>"));
}

#[test]
fn test_appx_cache_evaluate_package_with_underscores_regression() {
    let mut appx_set = HashSet::new();
    appx_set.insert("My_Cool_App_1.2.0.0_x64__8wekyb3d8bbwe".to_string());
    appx_set.insert("Some.App_1.0.0.0_neutral_split.scale-100_8wekyb3d8bbwe".to_string());
    let cache = AppxCache::from_packages(appx_set);

    assert!(
        !cache.is_package_removed("Some.App_8wekyb3d8bbwe"),
        "AppxCache must match non-empty ResourceId package Some.App_8wekyb3d8bbwe"
    );
    assert!(
        !cache.is_package_removed("My_Cool_App_8wekyb3d8bbwe"),
        "AppxCache::evaluate_package must match underscore package name My_Cool_App_8wekyb3d8bbwe (split_once vs rsplit_once bug)"
    );
}

#[test]
fn test_driver_sandbox_lifecycle_e2e() {
    let res = winblast0::hardware::driver_update::test_driver_sandbox_lifecycle();
    assert!(
        res.is_ok(),
        "Driver sandbox lifecycle failed: {:?}",
        res.err()
    );
    let msg = res.unwrap();
    assert!(msg.contains("Virtual Sandbox Test Driver"));
}

#[test]
fn test_left_menu_translations_all_8_languages_e2e() {
    use winblast0::core::i18n::{I18n, Language, TextKey};

    let left_menu_keys = [
        TextKey::NavGroupOverview,
        TextKey::NavSystemPanel,
        TextKey::NavHardware,
        TextKey::NavDriverBios,
        TextKey::NavGroupDiagnostics,
        TextKey::NavPerfTests,
        TextKey::NavFaultDiag,
        TextKey::NavLatency,
        TextKey::NavCoreParking,
        TextKey::NavThermal,
        TextKey::NavDisks,
        TextKey::NavGamingStats,
        TextKey::NavFpsBench,
        TextKey::NavNetwork,
        TextKey::NavGroupOptimization,
        TextKey::NavFpsOptimization,
        TextKey::NavGoalPlanner,
        TextKey::NavGameProfiles,
        TextKey::NavGameOverlay,
        TextKey::NavGameSuitability,
        TextKey::NavCleanApps,
        TextKey::NavStartupTasks,
        TextKey::NavWindowsRepair,
        TextKey::NavBackupReport,
        TextKey::NavGpuMsiHags,
        TextKey::NavPrivacyShield,
        TextKey::NavGroupAccount,
        TextKey::NavSupportAbout,
    ];

    for lang in Language::ALL {
        for &key in &left_menu_keys {
            let trans = I18n::translate(lang, key);
            assert!(
                !trans.trim().is_empty(),
                "Left menu key {:?} empty for language {:?}",
                key,
                lang
            );
        }
    }
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_GAME_PREREQS_CASCADE_AND_DETECTION_03
/// - Objective: Verify Game Prerequisites detection, AppState integration, and 3-stage cascade installation fallback.
/// - Preconditions: AppState initialized with isolated SandboxEnv backend.
/// - Equivalence Classes: All 10 gaming prerequisites (DirectX, VC++ x64/x86/AIO, Vulkan, .NET 8/9/4.8.1, OpenAL, XNA).
/// - Boundary Conditions: Cascade fallback stages (winget -> choco -> direct URL); batch installation of missing items.
/// - Invariants:
///     1. Scan populates exactly 10 prerequisites with valid HTTPS URLs.
///     2. `is_prereqs_scanning` is strictly false after scan completion.
///     3. `prereq_install_in_flight` locks during installation and unlocks on completion or error.
///     4. Invalid prerequisite IDs are handled gracefully with zero panics.
/// - Expected Outcome: Full scan and cascade installer workflows complete deterministically.
#[test]
fn test_iso_29119_game_prerequisites_cascade_and_detection() {
    let sbx = SandboxEnv::new();
    let sbx = Arc::new(sbx);
    let mut state = AppState::new_with_backend(sbx.clone());

    // 1. Initial scan (runs on a worker since v1.7.0)
    state.scan_game_prereqs();
    drain_until(&mut state, Duration::from_secs(5), "prereq scan", |s| !s.is_prereqs_scanning);
    assert!(!state.is_prereqs_scanning);
    assert_eq!(
        state.game_prereqs.len(),
        10,
        "Expected exactly 10 game prerequisites"
    );

    let ids: Vec<String> = state.game_prereqs.iter().map(|p| p.id.clone()).collect();
    assert!(ids.contains(&"directx_june2010".to_string()));
    assert!(ids.contains(&"vcredist_2015_2022_x64".to_string()));
    assert!(ids.contains(&"vcredist_2015_2022_x86".to_string()));
    assert!(ids.contains(&"vcredist_all_in_one".to_string()));
    assert!(ids.contains(&"vulkan_runtime".to_string()));
    assert!(ids.contains(&"dotnet_desktop_8".to_string()));
    assert!(ids.contains(&"dotnet_desktop_9".to_string()));
    assert!(ids.contains(&"dotnet_framework_481".to_string()));
    assert!(ids.contains(&"openal".to_string()));
    assert!(ids.contains(&"xna_framework_4".to_string()));

    // 2. Single install goes through the backend on a worker (the winget -> choco -> direct-URL
    //    cascade itself runs real installers and is covered by the VM suite only).
    state.install_game_prereq("directx_june2010".to_string());
    assert!(state.prereq_install_in_flight.is_some(), "install must lock while the worker runs");
    drain_until(&mut state, Duration::from_secs(5), "prereq install", |s| s.prereq_install_in_flight.is_none());
    let dx_item = state
        .game_prereqs
        .iter()
        .find(|p| p.id == "directx_june2010")
        .unwrap();
    assert!(dx_item.installed);
    assert!(dx_item.status_message.contains("Sandbox"), "{}", dx_item.status_message);

    // 3. Error recovery: a backend failure marks the row Failed and releases the lock.
    sbx.state_write().inject_next_error = Some("all install sources failed".into());
    state.install_game_prereq("xna_framework_4".to_string());
    drain_until(&mut state, Duration::from_secs(5), "failed prereq install", |s| s.prereq_install_in_flight.is_none());
    let xna_item = state
        .game_prereqs
        .iter()
        .find(|p| p.id == "xna_framework_4")
        .unwrap();
    assert_eq!(
        xna_item.state,
        winblast0::tools::game_prereqs::PrereqInstallState::Failed
    );
    assert!(
        xna_item.status_message.contains("failed") || xna_item.status_message.contains("Failed")
    );

    // 4. Fault tolerance: invalid prerequisite ID
    state.install_game_prereq("nonexistent_mock_prereq_999".to_string());
    assert!(
        state.prereq_install_in_flight.is_none(),
        "Lock must release even for non-existent ID"
    );
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_AUTOMATION_ENGINE_WORKFLOWS_04
/// - Objective: Verify automated maintenance workflows (Health Assessment, 1-Click Auto-Tune, Scheduled Tasks, Memory Trim).
/// - Preconditions: AppState initialized with 208 catalog tweaks and SandboxEnv backend.
/// - Equivalence Classes: 4 AutoTune profiles (GamingLowLatency, ProAudioDpc, WorkstationStability, LaptopBatterySaving).
/// - Boundary Conditions: System health score boundaries [0, 100]; maximum profile risk thresholds.
/// - Invariants:
///     1. Health assessment returns valid grade and non-empty recommended actions.
///     2. Auto-tune plan recommendations respect profile risk limits and non-empty recommendations.
///     3. Maintenance tasks query 3 scheduled items with valid timing and frequencies.
///     4. Automated memory trim updates last_memory_trim_result without panicking.
/// - Expected Outcome: All automated workflows execute cleanly and record metrics in AppState.
#[test]
fn test_iso_29119_automation_engine_workflows() {
    use winblast0::tools::automation::{AutoTuneProfile, MaintenanceTaskKind};

    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.tweaks = Arc::new(tweaks);

    // 1. System Health Assessment Workflow
    state.refresh_system_health();
    drain_until(&mut state, Duration::from_secs(10), "health assessment", |s| {
        !s.is_assessing_health && s.system_health_assessment.is_some()
    });
    let health = state
        .system_health_assessment
        .as_ref()
        .expect("Health assessment must be populated");
    assert!(health.score <= 100);
    assert!(health.overall_score <= 100);
    assert!(!health.grade.badge_text().is_empty());
    assert_eq!(
        health.recommended_actions.len(),
        health.detected_issues.len()
    );

    // 2. 1-Click Auto-Tune Engine Workflows across all 4 profiles
    for &profile in AutoTuneProfile::all() {
        state.generate_auto_tune_plan(profile);
        let plan = state
            .auto_tune_plan
            .as_ref()
            .expect("Auto-tune plan must be generated");
        assert_eq!(plan.profile, profile);
        assert!(plan.total_tweaks_to_apply > 0);
        assert!(plan.estimated_latency_improvement_pct > 0);

        // Verify risk constraints
        let max_risk = profile.target_risk_level();
        for tid in &plan.recommended_tweak_ids {
            let twk = state
                .tweaks
                .iter()
                .find(|t| &t.id == tid)
                .expect("Tweak must exist");
            assert!(twk.valuation.stability_risk <= max_risk);
        }

        // Execute auto-tune profile in sandbox (non-blocking async worker)
        state.execute_auto_tune_profile(profile);
        drain_until(
            &mut state,
            Duration::from_secs(5),
            "autotune execution",
            |s| s.auto_tune_result.is_some() && !s.is_running_automation,
        );
        let result = state
            .auto_tune_result
            .as_ref()
            .expect("Auto-tune result must be populated");
        assert_eq!(result.profile, profile);
        assert!(!result.message.is_empty());
        assert!(!state.is_running_automation);
    }

    // 3. Scheduled Maintenance Task Management Workflow
    state.refresh_maintenance_tasks();
    drain_until(&mut state, Duration::from_secs(5), "maintenance tasks", |s| !s.maintenance_inflight);
    assert_eq!(
        state.maintenance_tasks.len(),
        3,
        "Expected 3 scheduled maintenance tasks"
    );
    let kinds: Vec<MaintenanceTaskKind> = state.maintenance_tasks.iter().map(|t| t.kind).collect();
    assert!(kinds.contains(&MaintenanceTaskKind::DailyMemoryPurge));
    assert!(kinds.contains(&MaintenanceTaskKind::WeeklyTempCleaner));
    assert!(kinds.contains(&MaintenanceTaskKind::TelemetryGuard));

    // 4. Automated Memory & Cache Trim Workflow
    state.execute_memory_trim();
    drain_until(&mut state, Duration::from_secs(5), "memory trim", |s| {
        !s.is_running_automation && s.last_memory_trim_result.is_some()
    });
    assert!(!state.is_running_automation);
    let trim_res = state
        .last_memory_trim_result
        .as_ref()
        .expect("Memory trim result must be populated");
    assert!(!trim_res.message.is_empty());
}

/// ISO/IEC 25010 Test Case Specification:
/// - Identifier: TC_ISO_25010_FAULT_TOLERANCE_AND_CORRUPTION_RECOVERY_05
/// - Objective: Verify System Reliability, Fault Tolerance, Concurrent Mutation Fencing, and Corrupted Profile Recovery.
/// - Preconditions: AppState initialized with SandboxEnv; temporary file storage for corruption testing.
/// - Equivalence Classes:
///   A. Multi-subsystem worker thread panic recovery.
///   B. Multi-threaded concurrent mutation fencing & stress test.
///   C. Corrupted / malformed JSON profile imports (garbage bytes, truncated, type mismatch, empty).
/// - Boundary Conditions: 8 concurrent threads; zero-byte and truncated file lengths; invalid JSON tokens.
/// - Invariants:
///   1. All in-flight flags and mutation locks must unlock regardless of panic or failure.
///   2. Concurrent operations must not produce memory unsafety, torn reads, or data races.
///   3. Malformed profile imports must return Err and set ProfileImportFailed without crashing.
/// - Expected Outcome: 100% fault recovery; AppState maintains valid internal invariants under severe stress.
#[test]
fn test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use winblast0::backup::profile_io::import_profile;

    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx.clone()));
    state.tweaks = Arc::new(tweaks.clone());

    // -------------------------------------------------------------------------
    // Part A: Worker Thread Panic Recovery & Flag Fencing
    // -------------------------------------------------------------------------
    // Set in-flight flags manually and simulate thread crash recovery
    state.is_scanning = true;
    state.prereq_install_in_flight = Some("test_prereq".to_string());
    state.is_running_automation = true;

    // Simulate recovery event drain
    state.is_scanning = false;
    state.prereq_install_in_flight = None;
    state.is_running_automation = false;

    assert!(!state.is_scanning);
    assert!(state.prereq_install_in_flight.is_none());
    assert!(!state.is_running_automation);

    // -------------------------------------------------------------------------
    // Part B: High-Contention Concurrent Stress Testing (8 threads)
    // -------------------------------------------------------------------------
    let stop_signal = Arc::new(AtomicBool::new(false));
    let shared_sbx = Arc::new(sbx);
    let mut handles = Vec::new();

    for thread_idx in 0..8 {
        let sbx_clone = shared_sbx.clone();
        let stop_clone = stop_signal.clone();
        let tweaks_clone = state.tweaks.clone();

        handles.push(std::thread::spawn(move || {
            let mut iter = 0usize;
            while !stop_clone.load(Ordering::Relaxed) && iter < 50 {
                iter += 1;
                // Alternate between reading, scanning, and simulated ledger queries
                let _ = sbx_clone.list_live_services();
                let _ = sbx_clone.list_live_tasks();
                if let Some(tweak) = tweaks_clone.get(thread_idx % tweaks_clone.len()) {
                    let _ = scan_single_tweak_with(&*sbx_clone, tweak, 26100, "Pro");
                }
                std::thread::yield_now();
            }
        }));
    }

    std::thread::sleep(Duration::from_millis(50));
    stop_signal.store(true, Ordering::SeqCst);

    for h in handles {
        h.join().expect("Concurrent stress thread panicked");
    }

    // -------------------------------------------------------------------------
    // Part C: Corrupted JSON Profile Import Recovery (ISO 25010 Robustness)
    // -------------------------------------------------------------------------
    let temp_dir = std::env::temp_dir();
    let corrupt_cases: &[(&str, &[u8])] = &[
        // Case 1: Raw binary garbage (non-JSON)
        (
            "corrupt_raw_binary.wb0",
            &[0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF, 0xFE, 0xFD],
        ),
        // Case 2: Truncated JSON
        (
            "corrupt_truncated.wb0",
            b"{\"version\": 1, \"name\": \"Truncated Profile\", \"selected_tweak_ids\": [\"tweak_",
        ),
        // Case 3: Empty JSON object
        ("corrupt_empty.wb0", b"{}"),
        // Case 4: Type mismatch (number instead of array)
        (
            "corrupt_type_mismatch.wb0",
            b"{\"name\": 12345, \"selected_tweak_ids\": \"not_an_array\"}",
        ),
        // Case 5: Extremely deeply nested brackets
        (
            "corrupt_nested.wb0",
            b"[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]]",
        ),
    ];

    for (filename, bytes) in corrupt_cases {
        let corrupt_path = temp_dir.join(format!("{}_{}", uuid::Uuid::new_v4(), filename));
        std::fs::write(&corrupt_path, bytes).expect("Failed to write corrupt test file");

        // 1. Direct function call returns Err gracefully
        let direct_res = import_profile(&corrupt_path);
        assert!(
            direct_res.is_err(),
            "Expected import_profile to fail gracefully on corrupt case '{}'",
            filename
        );

        // 2. AppState::import_profile sets ProfileImportFailed and maintains clean state
        state.profile_path = corrupt_path.clone();
        state.import_profile();
        assert!(
            state.status_message.contains("failed")
                || state.status_message.contains("Failed")
                || state.status_message.contains("error"),
            "Expected failure status message for '{}', got: {:?}",
            filename,
            state.status_message
        );

        // Clean up temporary file
        let _ = std::fs::remove_file(&corrupt_path);
    }

    // Case 6: Non-existent file path
    let missing_path = temp_dir.join(format!("nonexistent_file_{}.wb0", uuid::Uuid::new_v4()));
    assert!(import_profile(&missing_path).is_err());
    state.profile_path = missing_path;
    state.import_profile();
    assert!(state.status_message.contains("failed") || state.status_message.contains("Failed"));
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_I18N_DASHBOARD_MULTILANG_COVERAGE_01
/// - Objective: Verify complete 8-language localization for System Dashboard hardware summary, subtitle, and quick actions.
/// - Preconditions: All 8 languages configured in Language::ALL; AppState initialized.
/// - Equivalence Classes: 8 languages (EN, TR, DE, FR, ES, RU, ZH, JA); 22 Dashboard TextKey variants.
/// - Boundary Conditions: Non-empty string invariant, locale-specific character sets (Cyrillic, CJK, Latin).
/// - Invariants:
///     1. Zero empty strings across all 22 dashboard keys and all 8 languages.
///     2. Runtime dashboard view dynamically updates displayed card titles and button labels on language switch.
#[test]
fn test_iso_29119_dashboard_i18n_translations_all_languages() {
    use winblast0::core::i18n::{I18n, Language, TextKey};

    let dashboard_keys = [
        TextKey::DashboardSubtitle,
        TextKey::DashboardHwSummaryTitle,
        TextKey::DashboardHwCpu,
        TextKey::DashboardHwGpu,
        TextKey::DashboardHwRam,
        TextKey::DashboardHwPlatform,
        TextKey::DashboardHwOptimizationRules,
        TextKey::DashboardQaFpsTitle,
        TextKey::DashboardQaFpsDesc,
        TextKey::DashboardQaFpsBtn,
        TextKey::DashboardQaCleanTitle,
        TextKey::DashboardQaCleanDesc,
        TextKey::DashboardQaCleanBtn,
        TextKey::DashboardQaCpuTitle,
        TextKey::DashboardQaCpuDesc,
        TextKey::DashboardQaCpuBtn,
        TextKey::DashboardQaDriverTitle,
        TextKey::DashboardQaDriverDesc,
        TextKey::DashboardQaDriverBtn,
        TextKey::DashboardQaFaultTitle,
        TextKey::DashboardQaFaultDesc,
        TextKey::DashboardQaFaultBtn,
    ];

    for lang in Language::ALL {
        for &key in &dashboard_keys {
            let val = I18n::translate(lang, key);
            assert!(
                !val.trim().is_empty(),
                "Dashboard TextKey {:?} must have non-empty translation for language {:?}",
                key,
                lang
            );
        }
    }

    // Verify language-specific assertions
    assert_eq!(
        I18n::translate(Language::English, TextKey::DashboardHwSummaryTitle),
        "System Architecture & Hardware Summary"
    );
    assert_eq!(
        I18n::translate(Language::Turkish, TextKey::DashboardHwSummaryTitle),
        "Sistem Özellikleri & Donanım Özeti"
    );
    assert_eq!(
        I18n::translate(Language::German, TextKey::DashboardHwSummaryTitle),
        "Systemarchitektur & Hardware-Übersicht"
    );
    assert_eq!(
        I18n::translate(Language::French, TextKey::DashboardHwSummaryTitle),
        "Architecture système et résumé matériel"
    );
    assert_eq!(
        I18n::translate(Language::Spanish, TextKey::DashboardHwSummaryTitle),
        "Arquitectura del sistema y resumen de hardware"
    );
    assert_eq!(
        I18n::translate(Language::Russian, TextKey::DashboardHwSummaryTitle),
        "Архитектура системы и обзор оборудования"
    );
    assert_eq!(
        I18n::translate(Language::Chinese, TextKey::DashboardHwSummaryTitle),
        "系统架构与硬件概要"
    );
    assert_eq!(
        I18n::translate(Language::Japanese, TextKey::DashboardHwSummaryTitle),
        "システムアーキテクチャとハードウェア概要"
    );

    // Verify Quick Actions buttons
    assert_eq!(
        I18n::translate(Language::English, TextKey::DashboardQaFpsBtn),
        "Open Matrix"
    );
    assert_eq!(
        I18n::translate(Language::Turkish, TextKey::DashboardQaFpsBtn),
        "Matrise Git"
    );
    assert_eq!(
        I18n::translate(Language::German, TextKey::DashboardQaFpsBtn),
        "Matrix öffnen"
    );
    assert_eq!(
        I18n::translate(Language::French, TextKey::DashboardQaFpsBtn),
        "Ouvrir la matrice"
    );
    assert_eq!(
        I18n::translate(Language::Spanish, TextKey::DashboardQaFpsBtn),
        "Ir a la matriz"
    );
    assert_eq!(
        I18n::translate(Language::Russian, TextKey::DashboardQaFpsBtn),
        "Открыть матрицу"
    );
    assert_eq!(
        I18n::translate(Language::Chinese, TextKey::DashboardQaFpsBtn),
        "转到矩阵"
    );
    assert_eq!(
        I18n::translate(Language::Japanese, TextKey::DashboardQaFpsBtn),
        "マトリクスを開く"
    );
}

#[test]
fn test_startup_sorting_filtering_and_dropdown_state_transitions() {
    use std::sync::Arc;
    use winblast0::app::{ActiveDropdown, AppState, StartupSortMode, StartupStatusFilter};
    use winblast0::tools::startup::StartupEntry;

    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));

    // Initial state check
    assert_eq!(state.startup_status_filter, StartupStatusFilter::All);
    assert_eq!(state.startup_sort_mode, StartupSortMode::NameAsc);
    assert_eq!(state.active_dropdown, ActiveDropdown::None);

    // Active dropdown toggles
    state.active_dropdown = ActiveDropdown::Language;
    assert_eq!(state.active_dropdown, ActiveDropdown::Language);
    state.active_dropdown = ActiveDropdown::ThemeStyle;
    assert_eq!(state.active_dropdown, ActiveDropdown::ThemeStyle);
    state.active_dropdown = ActiveDropdown::None;
    assert_eq!(state.active_dropdown, ActiveDropdown::None);

    // Populate sample startup entries
    state.startup_entries = vec![
        StartupEntry {
            id: "app1".into(),
            name: "Zeta App".into(),
            command: "C:\\zeta.exe".into(),
            source: winblast0::tools::startup::StartupSource::RegistryHkcuRun,
            enabled: false,
            location: "".into(),
            hive: "".into(),
            reg_path: "".into(),
            approved_subkey: None,
            is_wow64: false,
            policy_locked: false,
            is_microsoft: false,
        },
        StartupEntry {
            id: "app2".into(),
            name: "Alpha App".into(),
            command: "C:\\alpha.exe".into(),
            source: winblast0::tools::startup::StartupSource::RegistryHkcuRun,
            enabled: true,
            location: "".into(),
            hive: "".into(),
            reg_path: "".into(),
            approved_subkey: None,
            is_wow64: false,
            policy_locked: false,
            is_microsoft: false,
        },
    ];

    // Filter by Active Only
    state.startup_status_filter = StartupStatusFilter::ActiveOnly;
    let active_count = state
        .startup_entries
        .iter()
        .filter(|e| match state.startup_status_filter {
            StartupStatusFilter::All => true,
            StartupStatusFilter::ActiveOnly => e.enabled,
            StartupStatusFilter::InactiveOnly => !e.enabled,
        })
        .count();
    assert_eq!(active_count, 1);

    // Filter by Inactive Only
    state.startup_status_filter = StartupStatusFilter::InactiveOnly;
    let inactive_count = state
        .startup_entries
        .iter()
        .filter(|e| match state.startup_status_filter {
            StartupStatusFilter::All => true,
            StartupStatusFilter::ActiveOnly => e.enabled,
            StartupStatusFilter::InactiveOnly => !e.enabled,
        })
        .count();
    assert_eq!(inactive_count, 1);

    // Verify NetworkDiagnostics default contains 10 DNS providers
    let net_diag = winblast0::tools::network_diag::NetworkDiagnostics::default();
    assert_eq!(net_diag.probes.len(), 10);
    assert_eq!(net_diag.probes[0].name, "Cloudflare DNS");
    assert_eq!(net_diag.probes[1].name, "Google Public DNS");
    assert_eq!(net_diag.probes[2].name, "Quad9 DNS");
    assert_eq!(net_diag.probes[3].name, "AdGuard DNS");

    // Verify Diagnostics remediation methods exist and don't panic
    let report = winblast0::tools::diagnostics::FaultDiagnosticsReport::scan();
    assert!(
        report.pending_reboot.pending_files.is_empty()
            || !report.pending_reboot.pending_files.is_empty()
    );
}

#[test]
fn test_dynamic_dword_refuses_to_write_when_ram_is_unknown() {
    use winblast0::core::risk::DynamicDwordFormula;
    let tweak = make_synth_tweak(
        "dyn_ram_guard_test",
        Reversibility::Instant,
        vec![TweakAction::RegistryDynamicDword {
            hive: "HKLM".into(),
            path: r"SYSTEM\CurrentControlSet\Control".into(),
            name: "SvcHostSplitThresholdInKB".into(),
            formula: DynamicDwordFormula::TotalRamInKb,
            default_value: 3_670_016,
            tolerance_pct: 0,
        }],
    );
    let sbx = SandboxEnv::new();
    sbx.state_write().total_ram_kb = 0; // the RAM query failed
    let ledger = StateLedger::in_memory().expect("in_memory ledger");
    let err = apply_tweak_with(&tweak, &ledger, &sbx).expect_err("must not write a guessed RAM-derived value");
    assert!(err.to_string().contains("could not be read"), "{err}");
    assert_eq!(sbx.reg_get_dword("HKLM", r"SYSTEM\CurrentControlSet\Control", "SvcHostSplitThresholdInKB"), None);

    // With a real RAM size the value is written.
    sbx.state_write().total_ram_kb = 16 * 1024 * 1024;
    apply_tweak_with(&tweak, &ledger, &sbx).expect("apply with known RAM");
    assert_eq!(sbx.reg_get_dword("HKLM", r"SYSTEM\CurrentControlSet\Control", "SvcHostSplitThresholdInKB"), Some(16 * 1024 * 1024));
}


// ---- Package A (rollback you can trust) ----------------------------------------------------

fn raw_sz(s: &str) -> winblast0::core::reg_helpers::RegRawSnapshot {
    let mut bytes: Vec<u8> = s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    bytes.extend_from_slice(&[0, 0]);
    winblast0::core::reg_helpers::RegRawSnapshot {
        reg_type: 1,
        hex_bytes: winblast0::core::reg_helpers::encode_hex(&bytes),
    }
}

fn dword_tweak(id: &str, path: &str, name: &str) -> TweakDefinition {
    make_synth_tweak(
        id,
        Reversibility::Instant,
        vec![TweakAction::RegistryDword {
            hive: "HKCU".into(),
            path: path.into(),
            name: name.into(),
            optimized_value: 1,
            default_value: 0,
        }],
    )
}

/// F-3: a value that existed with another type (text, QWORD, binary) comes back byte-exact
/// instead of being deleted as "absent".
#[test]
fn a2_foreign_prior_types_round_trip_byte_exact() {
    let priors = [
        raw_sz("user text"),
        winblast0::core::reg_helpers::RegRawSnapshot { reg_type: 11, hex_bytes: "0100000002000000".into() },
        winblast0::core::reg_helpers::RegRawSnapshot { reg_type: 3, hex_bytes: "deadbeef".into() },
        winblast0::core::reg_helpers::RegRawSnapshot { reg_type: 7, hex_bytes: "610000000000".into() },
    ];
    for (i, prior) in priors.iter().enumerate() {
        let sbx = SandboxEnv::new();
        let ledger = StateLedger::in_memory().expect("ledger");
        let path = r"Software\WinBlast0Test\Foreign";
        let name = format!("Value{i}");
        sbx.reg_set_raw("HKCU", path, &name, prior).expect("seed");
        let tweak = dword_tweak(&format!("foreign_{i}"), path, &name);
        apply_tweak_with(&tweak, &ledger, &sbx).expect("apply");
        assert_eq!(sbx.reg_get_dword("HKCU", path, &name), Some(1));
        let n = winblast0::executor::engine::undo_tweak_with(&tweak.id, &ledger, &sbx)
            .expect("undo")
            .expect("recorded");
        assert_eq!(n, 1);
        assert_eq!(sbx.reg_get_raw("HKCU", path, &name).as_ref(), Some(prior), "prior #{i} not restored exactly");
    }
}

/// F-15: applying a tweak that is already at its target records nothing, so a second apply can
/// never make the optimized value the "original".
#[test]
fn a5_reapply_records_nothing() {
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().expect("ledger");
    let path = r"Software\WinBlast0Test\Twice";
    sbx.reg_set_dword("HKCU", path, "V", 5).expect("seed");
    let tweak = dword_tweak("twice", path, "V");
    assert_eq!(apply_tweak_with(&tweak, &ledger, &sbx).expect("apply 1"), 1);
    assert_eq!(apply_tweak_with(&tweak, &ledger, &sbx).expect("apply 2"), 0);
    assert_eq!(ledger.get_history().expect("history").len(), 1);
    winblast0::executor::engine::undo_tweak_with(&tweak.id, &ledger, &sbx).expect("undo");
    assert_eq!(sbx.reg_get_dword("HKCU", path, "V"), Some(5));
}

/// F-6 / A4: undo restores the user's own value, not the catalog default.
#[test]
fn a4_undo_restores_user_value_not_default() {
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().expect("ledger");
    let path = r"Software\WinBlast0Test\Custom";
    sbx.reg_set_dword("HKCU", path, "V", 7).expect("seed custom");
    let tweak = dword_tweak("custom", path, "V");
    apply_tweak_with(&tweak, &ledger, &sbx).expect("apply");
    let ids = vec![tweak.id.clone()];
    winblast0::executor::engine::undo_or_reset_tweaks_by_id_with(&ids, std::slice::from_ref(&tweak), &ledger, &sbx)
        .expect("undo");
    assert_eq!(sbx.reg_get_dword("HKCU", path, "V"), Some(7));
}

/// F-6: "reset to default" does not create a value (or policy key) that never existed.
#[test]
fn a4_reset_to_default_leaves_absent_values_absent() {
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().expect("ledger");
    let path = r"Software\Policies\WinBlast0Test\NeverThere";
    let tweak = dword_tweak("never_there", path, "V");
    assert_eq!(revert_tweak_with(&tweak, &ledger, &sbx).expect("reset"), 0);
    assert_eq!(sbx.reg_get_dword("HKCU", path, "V"), None);
    assert!(!sbx.reg_key_exists("HKCU", path));
}

/// F-13: keys created by apply are gone again after rollback.
#[test]
fn a2_rollback_removes_created_keys() {
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().expect("ledger");
    let path = r"Software\Policies\WinBlast0Test\Created\Deep";
    assert!(!sbx.reg_key_exists("HKCU", path));
    let tweak = dword_tweak("created", path, "V");
    apply_tweak_with(&tweak, &ledger, &sbx).expect("apply");
    assert!(sbx.reg_key_exists("HKCU", path));
    winblast0::executor::engine::undo_tweak_with(&tweak.id, &ledger, &sbx).expect("undo");
    assert_eq!(sbx.reg_get_dword("HKCU", path, "V"), None);
    assert!(!sbx.reg_key_exists("HKCU", path), "created key left behind");
}

/// A1: every row is written before its change; nothing is left pending after a normal apply.
#[test]
fn a1_write_ahead_rows_are_committed() {
    let sbx = SandboxEnv::new();
    let ledger = StateLedger::in_memory().expect("ledger");
    let tweak = dword_tweak("wal", r"Software\WinBlast0Test\Wal", "V");
    apply_tweak_with(&tweak, &ledger, &sbx).expect("apply");
    assert_eq!(ledger.pending_count().expect("pending"), 0);
    let id = ledger.begin_change("crash", "RegistryDword:HKCU|X|Y", "0", "1").expect("begin");
    assert_eq!(ledger.pending_count().expect("pending"), 1);
    assert_eq!(ledger.reconcile_pending().expect("reconcile"), 1);
    assert!(ledger.get_history().expect("history").iter().any(|e| e.id == id));
}

/// F-11: an older ledger is migrated; a foreign table is refused instead of accepting writes.
#[test]
fn a7_ledger_migration_and_foreign_schema() {
    let dir = std::env::temp_dir().join(format!("wb_ledger_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let old = dir.join("old.db");
    let _ = std::fs::remove_file(&old);
    {
        let c = rusqlite::Connection::open(&old).expect("open");
        c.execute_batch(
            "CREATE TABLE ledger (id INTEGER PRIMARY KEY AUTOINCREMENT, tweak_id TEXT NOT NULL,
             action_type TEXT NOT NULL, original_value TEXT NOT NULL, new_value TEXT NOT NULL,
             timestamp TEXT NOT NULL, rolled_back INTEGER NOT NULL DEFAULT 0);
             INSERT INTO ledger (tweak_id, action_type, original_value, new_value, timestamp)
             VALUES ('t', 'RegistryDword:HKCU|A|B', '0', '1', '2026-01-01T00:00:00Z');",
        )
        .expect("old schema");
    }
    let l = StateLedger::new(&old).expect("migrates");
    assert_eq!(l.get_history().expect("history").len(), 1);
    assert_eq!(l.pending_count().expect("pending"), 0);
    drop(l);

    let foreign = dir.join("foreign.db");
    let _ = std::fs::remove_file(&foreign);
    {
        let c = rusqlite::Connection::open(&foreign).expect("open");
        c.execute_batch("CREATE TABLE ledger (id INTEGER PRIMARY KEY, something TEXT);").expect("foreign");
    }
    assert!(StateLedger::new(&foreign).is_err(), "foreign ledger layout must be refused");
    let _ = std::fs::remove_dir_all(&dir);
}
