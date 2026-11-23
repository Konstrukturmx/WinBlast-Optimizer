//! System scan orchestrator.
//!
//! Evaluates system state against loaded tweak definitions with zero duplicate action logic,
//! supporting both the live Windows OS (`RealBackend`) and isolated `SandboxEnv` testing.

pub mod appx_scanner;
pub mod registry_scanner;
pub mod service_scanner;
pub mod task_scanner;
pub mod telemetry_scanner;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::backend::{
    is_unsupported_power_guid_error, CachedScanBackend, RealBackend, SystemBackend,
};
use crate::core::risk::{RiskValuation, TweakAction, TweakDefinition};

/// Current optimization state of a single tweak.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TweakState {
    /// All actions in the tweak are already at their optimized values.
    AlreadyOptimized,
    /// None of the actions are at their optimized values.
    NotOptimized,
    /// Some actions are optimized and some are not.
    PartiallyOptimized,
    /// The tweak does not apply to this OS build/edition.
    NotApplicable,
}

/// Structured scan status detail payload for dynamic multi-language UI rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanDetailKind {
    NotApplicableBuild {
        current: u32,
        min_build: u32,
    },
    NotApplicableMaxBuild {
        current: u32,
        max_build: u32,
    },
    NotApplicableEdition {
        edition: String,
        required: String,
    },
    NotApplicableGpu {
        vendor: String,
    },
    RegStringValue {
        value: String,
    },
    RegNotConfiguredStock,
    RegValueRemoved,
    RegValuePresent,
    SubtreeRemoved,
    SubtreePresent,
    BinaryMatch {
        index: usize,
        byte_val: u8,
        mask: u8,
    },
    BinaryTooShort {
        required: usize,
        actual: usize,
    },
    BinaryNotConfigured,
    CompositeSet {
        key: String,
        value: String,
    },
    CompositeNotSet {
        key: String,
    },
    DynCurrent {
        current: u32,
        target: u32,
    },
    DynNotConfigured {
        target: u32,
    },
    PowerIndex {
        ac: u32,
        dc: u32,
    },
    PowerNotExposed {
        err: String,
    },
    PowerUnavailable {
        err: String,
    },
    PowerUnavailableHardware,
    FeatureState {
        state: crate::core::risk::FeatureInstallState,
    },
    FeatureNotRegistered,
    FeatureRequiresAdmin,
    CompoundOptimized {
        optimized: usize,
        total: usize,
    },
    // From registry_scanner:
    DwordOptimized {
        value: u32,
    },
    DwordDefault {
        value: u32,
    },
    DwordCustom {
        value: u32,
        expected: u32,
    },
    DwordDefaultNotConfigured,
    DwordNotConfigured {
        default_val: u32,
        expected: u32,
    },
    DwordUnknownHive {
        hive: String,
    },
    NoGpuAdapter {
        vendor: String,
    },
    // From service_scanner:
    ServiceOptimized {
        mode: u32,
    },
    ServiceCurrent {
        current: u32,
        target: u32,
    },
    ServiceNotInstalled,
    ServiceAbsent,
    // From task_scanner:
    TaskFound {
        active: bool,
    },
    TaskMatched {
        leaf: String,
        active: bool,
    },
    TaskDirect {
        leaf: String,
        active: bool,
    },
    TaskRequiresAdmin,
    TaskQueryError {
        err: String,
    },
    TaskAbsentOptimized,
    TaskAbsentStock,
    // From appx_scanner:
    AppxNoPackage,
    AppxRemovedPs,
    AppxInstalledPs,
    AppxInstalled {
        package: String,
    },
    AppxRemovedOptimized,
}

/// Result of scanning a single tweak against the current system state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub tweak_id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub easy_mode_included: bool,
    pub current_state: TweakState,
    pub risk: RiskValuation,
    pub status_detail: String,
    #[serde(default)]
    pub sub_category: Option<String>,
    #[serde(default)]
    pub detail_kind: Option<ScanDetailKind>,
}

/// Resolve dynamic DWORD target consistently across scanner, executor, and diff modal.
pub fn resolve_dynamic_dword(
    backend: &dyn SystemBackend,
    formula: crate::core::risk::DynamicDwordFormula,
) -> u32 {
    match formula {
        crate::core::risk::DynamicDwordFormula::TotalRamInKb => {
            backend.total_physical_ram_kb().clamp(1, u32::MAX as u64) as u32
        }
        crate::core::risk::DynamicDwordFormula::TotalRamInKbPlusMargin => backend
            .total_physical_ram_kb()
            .saturating_add(1_048_576)
            .clamp(1, u32::MAX as u64)
            as u32,
        crate::core::risk::DynamicDwordFormula::LogicalCoreCount => {
            backend.logical_core_count().max(1)
        }
    }
}

/// True when the hardware value a dynamic DWORD formula depends on could actually be read.
/// The executor refuses to write a value derived from an unknown (0) RAM size or core count.
pub fn dynamic_dword_input_available(
    backend: &dyn SystemBackend,
    formula: crate::core::risk::DynamicDwordFormula,
) -> bool {
    use crate::core::risk::DynamicDwordFormula as F;
    match formula {
        F::TotalRamInKb | F::TotalRamInKbPlusMargin => backend.total_physical_ram_kb() > 0,
        F::LogicalCoreCount => backend.logical_core_count() > 0,
    }
}

enum ActionEval {
    Evaluated {
        optimized: bool,
        detail: String,
        kind: ScanDetailKind,
    },
    SkippedPower {
        detail: String,
        kind: ScanDetailKind,
    },
    FeatureAuthRequired(String),
}

/// Build a [`ScanResult`] with consistent metadata and structured detail kind.
fn make_scan_result(
    tweak: &TweakDefinition,
    current_state: TweakState,
    status_detail: String,
    detail_kind: ScanDetailKind,
) -> ScanResult {
    ScanResult {
        tweak_id: tweak.id.clone(),
        title: tweak.title.clone(),
        category: tweak.category.clone(),
        description: tweak.description.clone(),
        easy_mode_included: tweak.easy_mode_included,
        current_state,
        risk: tweak.valuation.clone(),
        status_detail,
        sub_category: tweak.sub_category.clone(),
        detail_kind: Some(detail_kind),
    }
}

/// Evaluate a single [`TweakAction`] returning its optimization state and structured diagnostic.
fn evaluate_single_action(
    cached: &CachedScanBackend<'_>,
    action: &TweakAction,
    vendor_ref: Option<&str>,
) -> ActionEval {
    match action {
        TweakAction::RegistryDword {
            hive,
            path,
            name,
            optimized_value,
            default_value,
        } => {
            let (optimized, detail, kind) = registry_scanner::evaluate_dword_value_with_kind(
                cached
                    .inner()
                    .reg_get_dword_for_vendor(hive, path, name, vendor_ref),
                *optimized_value,
                *default_value,
            );
            ActionEval::Evaluated {
                optimized,
                detail,
                kind,
            }
        }
        TweakAction::RegistryString {
            hive,
            path,
            name,
            optimized_value,
            default_value,
            expand_sz,
            absent_is_default,
            case_insensitive,
        } => match cached.inner().reg_get_string(hive, path, name, vendor_ref) {
            Some(cur) => {
                let text_eq = if *case_insensitive {
                    cur.value.eq_ignore_ascii_case(optimized_value)
                } else {
                    cur.value == *optimized_value
                };
                let is_opt =
                    optimized_value != "__ABSENT__" && text_eq && cur.expand_sz == *expand_sz;
                let detail = format!("String value: '{}'", cur.value);
                ActionEval::Evaluated {
                    optimized: is_opt,
                    detail,
                    kind: ScanDetailKind::RegStringValue { value: cur.value },
                }
            }
            None => {
                let is_opt = optimized_value == "__ABSENT__"
                    || (*absent_is_default && optimized_value == default_value);
                ActionEval::Evaluated {
                    optimized: is_opt,
                    detail: "Value not configured (Stock)".to_string(),
                    kind: ScanDetailKind::RegNotConfiguredStock,
                }
            }
        },
        TweakAction::RegistryDelete { hive, path, name } => {
            match cached.inner().reg_get_raw(hive, path, name) {
                None => ActionEval::Evaluated {
                    optimized: true,
                    detail: "Value removed (Optimized)".to_string(),
                    kind: ScanDetailKind::RegValueRemoved,
                },
                Some(_) => ActionEval::Evaluated {
                    optimized: false,
                    detail: "Value present (Stock)".to_string(),
                    kind: ScanDetailKind::RegValuePresent,
                },
            }
        }
        TweakAction::RegistryDeleteTree { hive, path } => {
            if !cached.inner().reg_key_exists(hive, path) {
                ActionEval::Evaluated {
                    optimized: true,
                    detail: "Subtree removed (Optimized)".to_string(),
                    kind: ScanDetailKind::SubtreeRemoved,
                }
            } else {
                ActionEval::Evaluated {
                    optimized: false,
                    detail: "Subtree present (Stock)".to_string(),
                    kind: ScanDetailKind::SubtreePresent,
                }
            }
        }
        TweakAction::RegistryBinaryBit {
            hive,
            path,
            name,
            byte_index,
            min_length,
            bit_mask,
            optimized_bits,
            ..
        } => {
            let req = (*min_length).max(byte_index.saturating_add(1));
            match cached.inner().reg_get_binary(hive, path, name) {
                Some(bytes) if bytes.len() >= req => {
                    if let Some(&byte_val) = bytes.get(*byte_index) {
                        let cur_bits = byte_val & bit_mask;
                        let tgt_bits = optimized_bits & bit_mask;
                        ActionEval::Evaluated {
                            optimized: cur_bits == tgt_bits,
                            detail: format!(
                                "Binary[{}]=0x{:02X} (mask 0x{:02X})",
                                byte_index, byte_val, bit_mask
                            ),
                            kind: ScanDetailKind::BinaryMatch {
                                index: *byte_index,
                                byte_val,
                                mask: *bit_mask,
                            },
                        }
                    } else {
                        ActionEval::Evaluated {
                            optimized: false,
                            detail: format!(
                                "Binary shorter than {} bytes (len={})",
                                req,
                                bytes.len()
                            ),
                            kind: ScanDetailKind::BinaryTooShort {
                                required: req,
                                actual: bytes.len(),
                            },
                        }
                    }
                }
                Some(bytes) => ActionEval::Evaluated {
                    optimized: false,
                    detail: format!("Binary shorter than {} bytes (len={})", req, bytes.len()),
                    kind: ScanDetailKind::BinaryTooShort {
                        required: req,
                        actual: bytes.len(),
                    },
                },
                None => ActionEval::Evaluated {
                    optimized: false,
                    detail: "Binary value not configured".to_string(),
                    kind: ScanDetailKind::BinaryNotConfigured,
                },
            }
        }
        TweakAction::CompositeStringKey {
            hive,
            path,
            name,
            entry_key,
            optimized_entry_value,
            pair_separator,
            kv_separator,
            ..
        } => {
            let raw = cached
                .inner()
                .reg_get_string(hive, path, name, vendor_ref)
                .map(|s| s.value);
            let found = raw.as_deref().and_then(|r| {
                crate::core::reg_helpers::read_composite_entry(
                    r,
                    entry_key,
                    pair_separator,
                    kv_separator,
                )
            });
            let is_opt = if optimized_entry_value == "__ABSENT__" {
                found.is_none()
            } else {
                found.as_deref() == Some(optimized_entry_value.as_str())
            };
            let (detail, kind) = match found {
                Some(v) => (
                    format!("{}={}", entry_key, v),
                    ScanDetailKind::CompositeSet {
                        key: entry_key.clone(),
                        value: v,
                    },
                ),
                None => (
                    format!("{} not set", entry_key),
                    ScanDetailKind::CompositeNotSet {
                        key: entry_key.clone(),
                    },
                ),
            };
            ActionEval::Evaluated {
                optimized: is_opt,
                detail,
                kind,
            }
        }
        TweakAction::RegistryDynamicDword {
            hive,
            path,
            name,
            formula,
            default_value,
            tolerance_pct,
        } => {
            let target = resolve_dynamic_dword(cached.inner(), *formula);
            match cached.inner().reg_get_dword(hive, path, name) {
                Some(val) => {
                    let min_ok = ((target as u64)
                        .saturating_mul((100u64).saturating_sub(*tolerance_pct as u64))
                        / 100) as u32;
                    let is_opt = if *tolerance_pct > 0 && target > *default_value {
                        val >= min_ok && val != *default_value
                    } else {
                        val == target
                    };
                    ActionEval::Evaluated {
                        optimized: is_opt,
                        detail: format!("Current: {} (Target: {})", val, target),
                        kind: ScanDetailKind::DynCurrent {
                            current: val,
                            target,
                        },
                    }
                }
                None => ActionEval::Evaluated {
                    optimized: false,
                    detail: format!("Not configured (Target: {})", target),
                    kind: ScanDetailKind::DynNotConfigured { target },
                },
            }
        }
        TweakAction::PowerCfgSetting {
            subgroup_guid,
            setting_guid,
            scheme_guid,
            optimized_ac,
            optimized_dc,
            scope,
            ..
        } => {
            match cached.inner().read_power_setting_index(
                scheme_guid.as_deref(),
                subgroup_guid,
                setting_guid,
            ) {
                Ok((ac, dc)) => {
                    let is_opt = match scope {
                        crate::core::risk::PowerTargetScope::AcOnly => ac == *optimized_ac,
                        crate::core::risk::PowerTargetScope::DcOnly => dc == *optimized_dc,
                        crate::core::risk::PowerTargetScope::AcAndDc => {
                            ac == *optimized_ac && dc == *optimized_dc
                        }
                    };
                    ActionEval::Evaluated {
                        optimized: is_opt,
                        detail: format!("Power index AC={} DC={}", ac, dc),
                        kind: ScanDetailKind::PowerIndex { ac, dc },
                    }
                }
                Err(e) if is_unsupported_power_guid_error(&e) => {
                    let err = e.to_string();
                    ActionEval::SkippedPower {
                        detail: format!(
                            "Power setting not exposed on this hardware/scheme: {}",
                            err
                        ),
                        kind: ScanDetailKind::PowerNotExposed { err },
                    }
                }
                Err(e) => {
                    let err = e.to_string();
                    ActionEval::Evaluated {
                        optimized: false,
                        detail: format!("Power setting unavailable: {}", err),
                        kind: ScanDetailKind::PowerUnavailable { err },
                    }
                }
            }
        }
        TweakAction::ServiceState {
            service_name,
            target_start_type,
            ..
        } => {
            let (optimized, detail, kind) = service_scanner::evaluate_service_value_with_kind(
                cached.inner().get_service_start_type(service_name),
                *target_start_type,
            );
            ActionEval::Evaluated {
                optimized,
                detail,
                kind,
            }
        }
        TweakAction::ScheduledTask { task_path, enable } => {
            let (optimized, detail, kind) =
                cached.tasks().evaluate_task_with_kind(task_path, *enable);
            ActionEval::Evaluated {
                optimized,
                detail,
                kind,
            }
        }
        TweakAction::AppXPackage { package_family } => {
            let (optimized, detail, kind) =
                cached.appx().evaluate_package_with_kind(package_family);
            ActionEval::Evaluated {
                optimized,
                detail,
                kind,
            }
        }
        TweakAction::WindowsFeature {
            feature_name,
            enable,
            ..
        } => match cached.optional_features() {
            Ok(feats) => {
                let req_prefix = feature_name
                    .split_once("~~~~")
                    .map(|(p, _)| p)
                    .unwrap_or(feature_name);
                if let Some(f) = feats.iter().find(|f| {
                    f.name.eq_ignore_ascii_case(feature_name)
                        || f.name
                            .split_once("~~~~")
                            .map(|(p, _)| p)
                            .unwrap_or(&f.name)
                            .eq_ignore_ascii_case(req_prefix)
                }) {
                    let is_enabled = matches!(
                        f.state,
                        crate::core::risk::FeatureInstallState::Enabled
                            | crate::core::risk::FeatureInstallState::EnablePending
                    );
                    ActionEval::Evaluated {
                        optimized: is_enabled == *enable,
                        detail: format!("Feature state: {:?}", f.state),
                        kind: ScanDetailKind::FeatureState { state: f.state },
                    }
                } else {
                    ActionEval::Evaluated {
                        optimized: !*enable,
                        detail: "Feature not present on this Windows edition (NotRegistered)"
                            .to_string(),
                        kind: ScanDetailKind::FeatureNotRegistered,
                    }
                }
            }
            Err(_) => ActionEval::FeatureAuthRequired(
                "Optional feature scan requires Administrator privileges".to_string(),
            ),
        },
    }
}

/// Shared single-tweak evaluator powered by a lazy-caching `CachedScanBackend`.
fn evaluate_tweak(
    cached: &CachedScanBackend<'_>,
    tweak: &TweakDefinition,
    build: u32,
    edition: &str,
) -> ScanResult {
    if !tweak.is_compatible(build, edition) {
        let (detail, kind) = if build < tweak.os_req.min_build {
            (
                format!(
                    "Not applicable on build {} (requires >= {})",
                    build, tweak.os_req.min_build
                ),
                ScanDetailKind::NotApplicableBuild {
                    current: build,
                    min_build: tweak.os_req.min_build,
                },
            )
        } else if let Some(max_build) = tweak.os_req.max_build.filter(|&max| build > max) {
            (
                format!(
                    "Not applicable on build {} (requires <= {})",
                    build, max_build
                ),
                ScanDetailKind::NotApplicableMaxBuild {
                    current: build,
                    max_build,
                },
            )
        } else {
            (
                format!("Not applicable on {} edition", edition),
                ScanDetailKind::NotApplicableEdition {
                    edition: edition.to_string(),
                    required: tweak.os_req.editions.join(", "),
                },
            )
        };
        return make_scan_result(tweak, TweakState::NotApplicable, detail, kind);
    }

    if tweak.gpu_vendor.is_some() && !tweak.is_gpu_vendor_compatible(cached.gpu_vendors()) {
        let vendor = tweak
            .gpu_vendor
            .as_deref()
            .unwrap_or("matching")
            .to_string();
        return make_scan_result(
            tweak,
            TweakState::NotApplicable,
            format!("Not applicable (requires {} GPU)", vendor),
            ScanDetailKind::NotApplicableGpu { vendor },
        );
    }

    let vendor_ref = tweak.gpu_vendor.as_deref();
    let mut optimized_count = 0usize;
    let mut total_count = 0usize;
    let mut skipped_power = 0usize;
    let mut last_power_skip: Option<(String, ScanDetailKind)> = None;
    let mut feature_scan_err: Option<String> = None;
    let mut action_diagnostics: Vec<(String, ScanDetailKind)> =
        Vec::with_capacity(tweak.actions.len());

    for action in &tweak.actions {
        match evaluate_single_action(cached, action, vendor_ref) {
            ActionEval::Evaluated {
                optimized,
                detail,
                kind,
            } => {
                total_count += 1;
                if optimized {
                    optimized_count += 1;
                }
                action_diagnostics.push((detail, kind));
            }
            ActionEval::SkippedPower { detail, kind } => {
                skipped_power += 1;
                last_power_skip = Some((detail, kind));
            }
            ActionEval::FeatureAuthRequired(err) => {
                feature_scan_err = Some(err);
            }
        }
    }

    if let Some(ref feat_err) = feature_scan_err {
        if total_count == 0 {
            return make_scan_result(
                tweak,
                TweakState::NotApplicable,
                feat_err.clone(),
                ScanDetailKind::FeatureRequiresAdmin,
            );
        }
        total_count += 1;
        action_diagnostics.push((feat_err.clone(), ScanDetailKind::FeatureRequiresAdmin));
    }

    let current_state = if total_count == 0 {
        TweakState::NotApplicable
    } else if optimized_count == total_count {
        TweakState::AlreadyOptimized
    } else if optimized_count == 0 {
        TweakState::NotOptimized
    } else {
        TweakState::PartiallyOptimized
    };

    let (status_detail, detail_kind) = if total_count == 0 && skipped_power > 0 {
        last_power_skip.unwrap_or_else(|| {
            (
                "Power setting unavailable on this hardware".to_string(),
                ScanDetailKind::PowerUnavailableHardware,
            )
        })
    } else if action_diagnostics.len() == 1 {
        action_diagnostics.into_iter().next().unwrap_or_else(|| {
            (
                String::new(),
                ScanDetailKind::CompoundOptimized {
                    optimized: 0,
                    total: 0,
                },
            )
        })
    } else {
        (
            format!("{}/{} actions optimized", optimized_count, total_count),
            ScanDetailKind::CompoundOptimized {
                optimized: optimized_count,
                total: total_count,
            },
        )
    };

    make_scan_result(tweak, current_state, status_detail, detail_kind)
}

/// Evaluate the optimization status of a single tweak definition against the provided backend.
pub fn evaluate_tweak_status(
    backend: &dyn SystemBackend,
    tweak: &TweakDefinition,
    build: u32,
    edition: &str,
) -> ScanResult {
    scan_single_tweak_with(backend, tweak, build, edition)
}

/// Run a multi-threaded parallel system scan against the provided backend using Rayon.
pub fn scan_all_tweaks_parallel_with(
    backend: &dyn SystemBackend,
    tweaks: &[TweakDefinition],
    build: u32,
    edition: &str,
) -> Vec<ScanResult> {
    let cached = CachedScanBackend::new(backend);
    // Pre-warm GPU vendor detection to avoid initial contention across threads
    let _ = cached.gpu_vendors();
    tweaks
        .par_iter()
        .map(|t| evaluate_tweak(&cached, t, build, edition))
        .collect()
}

/// Run a multi-threaded parallel system scan against the live Windows OS using Rayon.
pub fn scan_all_tweaks_parallel(
    tweaks: &[TweakDefinition],
    build: u32,
    edition: &str,
) -> Vec<ScanResult> {
    scan_all_tweaks_parallel_with(&RealBackend, tweaks, build, edition)
}

/// Run a chunked multi-threaded parallel system scan against the provided backend using Rayon.
pub fn scan_tweaks_chunked_parallel_with(
    backend: &dyn SystemBackend,
    tweaks: &[TweakDefinition],
    build: u32,
    edition: &str,
    chunk_size: usize,
) -> Vec<ScanResult> {
    let cached = CachedScanBackend::new(backend);
    let _ = cached.gpu_vendors();
    let size = chunk_size.max(1);
    tweaks
        .par_chunks(size)
        .flat_map(|chunk| {
            chunk
                .iter()
                .map(|t| evaluate_tweak(&cached, t, build, edition))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Run a chunked multi-threaded parallel system scan against the live Windows OS using Rayon.
pub fn scan_tweaks_chunked_parallel(
    tweaks: &[TweakDefinition],
    build: u32,
    edition: &str,
    chunk_size: usize,
) -> Vec<ScanResult> {
    scan_tweaks_chunked_parallel_with(&RealBackend, tweaks, build, edition, chunk_size)
}

/// Run a full system scan against the provided backend.
pub fn run_full_scan_with(
    backend: &dyn SystemBackend,
    tweaks: &[TweakDefinition],
    build: u32,
    edition: &str,
) -> Vec<ScanResult> {
    scan_all_tweaks_parallel_with(backend, tweaks, build, edition)
}

/// Run an instant, targeted scan on a single tweak against the provided backend.
pub fn scan_single_tweak_with(
    backend: &dyn SystemBackend,
    tweak: &TweakDefinition,
    build: u32,
    edition: &str,
) -> ScanResult {
    let cached = CachedScanBackend::new(backend);
    evaluate_tweak(&cached, tweak, build, edition)
}

/// Run a full system scan against the live Windows OS.
pub fn run_full_scan(tweaks: &[TweakDefinition], build: u32, edition: &str) -> Vec<ScanResult> {
    run_full_scan_with(&RealBackend, tweaks, build, edition)
}

/// Run an instant, targeted scan on a single tweak against the live Windows OS.
pub fn scan_single_tweak(tweak: &TweakDefinition, build: u32, edition: &str) -> ScanResult {
    scan_single_tweak_with(&RealBackend, tweak, build, edition)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::backend::SandboxEnv;
    use crate::core::risk::load_embedded_rules;

    #[test]
    fn test_full_scan_execution_in_sandbox() {
        let tweaks = load_embedded_rules();
        assert!(!tweaks.is_empty(), "No embedded rules found");

        let sbx = SandboxEnv::new();
        sbx.seed_all_tweaks(&tweaks);

        let results = run_full_scan_with(&sbx, &tweaks, 26100, "Pro");
        assert_eq!(results.len(), tweaks.len());
    }

    #[test]
    fn test_scan_single_tweak_in_sandbox() {
        let tweaks = load_embedded_rules();
        let sbx = SandboxEnv::new();
        if let Some(first) = tweaks.first() {
            sbx.seed_tweak_defaults(&first.actions);
            let res = scan_single_tweak_with(&sbx, first, 26100, "Pro");
            assert_eq!(res.tweak_id, first.id);
            assert_eq!(res.current_state, TweakState::NotOptimized);
        }
    }

    #[test]
    fn test_parallel_scan_parity_with_sequential() {
        let tweaks = load_embedded_rules();
        let sbx = SandboxEnv::new();
        sbx.seed_all_tweaks(&tweaks);

        // Sequential baseline
        let cached = CachedScanBackend::new(&sbx);
        let seq_results: Vec<ScanResult> = tweaks
            .iter()
            .map(|t| evaluate_tweak(&cached, t, 26100, "Pro"))
            .collect();

        // Multi-threaded parallel scan
        let par_results = scan_all_tweaks_parallel_with(&sbx, &tweaks, 26100, "Pro");
        assert_eq!(seq_results.len(), par_results.len());

        for (seq, par) in seq_results.iter().zip(par_results.iter()) {
            assert_eq!(seq.tweak_id, par.tweak_id);
            assert_eq!(seq.current_state, par.current_state);
            assert_eq!(seq.status_detail, par.status_detail);
            assert_eq!(seq.detail_kind, par.detail_kind);
        }

        // Chunked parallel scan across multiple grain sizes
        for chunk_size in [1, 7, 16, 64] {
            let chunked =
                scan_tweaks_chunked_parallel_with(&sbx, &tweaks, 26100, "Pro", chunk_size);
            assert_eq!(seq_results.len(), chunked.len());
            for (seq, chk) in seq_results.iter().zip(chunked.iter()) {
                assert_eq!(seq.tweak_id, chk.tweak_id);
                assert_eq!(seq.current_state, chk.current_state);
            }
        }
    }

    #[test]
    fn test_evaluate_tweak_status_single() {
        let tweaks = load_embedded_rules();
        let sbx = SandboxEnv::new();
        if let Some(tweak) = tweaks.first() {
            sbx.seed_tweak_defaults(&tweak.actions);
            let res = evaluate_tweak_status(&sbx, tweak, 26100, "Pro");
            assert_eq!(res.tweak_id, tweak.id);
            assert_eq!(res.current_state, TweakState::NotOptimized);
        }
    }

    #[test]
    fn test_foreign_gpu_and_non_elevated_feature_and_partial_power_scan() {
        let tweaks = load_embedded_rules();
        let sbx = SandboxEnv::new();
        {
            let mut st = sbx.inner.write().unwrap();
            // NVIDIA only -> AMD & Intel must be NotApplicable
            st.gpus
                .retain(|g| g.vendor == crate::hardware::gpu::GpuVendor::Nvidia);
            st.elevated = false;
        }

        let amd_tweak = tweaks
            .iter()
            .find(|t| t.gpu_vendor.as_deref() == Some("AMD"))
            .expect("AMD tweak missing");
        let amd_res = scan_single_tweak_with(&sbx, amd_tweak, 26100, "Pro");
        assert_eq!(amd_res.current_state, TweakState::NotApplicable);

        let feat_tweak = tweaks
            .iter()
            .find(|t| t.id == "disable_recall_optional_feature")
            .expect("Recall feature tweak missing");
        let feat_res = scan_single_tweak_with(&sbx, feat_tweak, 26100, "Pro");
        assert_eq!(feat_res.current_state, TweakState::NotApplicable);
        assert!(feat_res.status_detail.contains("Administrator"));

        // PowerCfgSetting: sibling tuned settings tolerate hardware-absent power GUIDs
        let pwr_tweak = tweaks
            .iter()
            .find(|t| t.id == "power_settings_pcie_usb_nvme_epb")
            .unwrap();
        sbx.seed_tweak_defaults(&pwr_tweak.actions[..2]);
        let ledger = crate::backup::state_ledger::StateLedger::in_memory().unwrap();
        crate::executor::engine::apply_tweak_with(pwr_tweak, &ledger, &sbx).unwrap();
        let pwr_res = scan_single_tweak_with(&sbx, pwr_tweak, 26100, "Pro");
        assert_eq!(pwr_res.current_state, TweakState::AlreadyOptimized);
        assert!(matches!(
            pwr_res.detail_kind,
            Some(ScanDetailKind::CompoundOptimized { .. })
        ));
    }

    fn mk_test_tweak(id: &str, actions: Vec<TweakAction>) -> TweakDefinition {
        use crate::core::risk::{OsRequirement, Reversibility, RiskLevel};

        TweakDefinition {
            id: id.to_string(),
            title: format!("Title {}", id),
            category: "Test".to_string(),
            description: "Test tweak".to_string(),
            easy_mode_included: false,
            os_req: OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec!["Home".to_string(), "Pro".to_string()],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 1,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions,
            gpu_vendor: None,
            sub_category: Some("Power".to_string()),
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        }
    }

    #[test]
    fn test_run_full_scan_populates_detail_kind_and_sub_category() {
        let tweaks = load_embedded_rules();
        let sbx = SandboxEnv::new();
        sbx.seed_all_tweaks(&tweaks);

        let results = run_full_scan_with(&sbx, &tweaks, 26100, "Pro");
        assert_eq!(results.len(), tweaks.len());
        for (tweak, res) in tweaks.iter().zip(results.iter()) {
            assert!(
                res.detail_kind.is_some(),
                "detail_kind missing for tweak '{}' (state={:?}, status={})",
                tweak.id,
                res.current_state,
                res.status_detail
            );
            assert_eq!(res.sub_category, tweak.sub_category);
        }

        // Regression: max_build-incompatible tweaks report NotApplicableMaxBuild accurately on 26100
        let mpo = results
            .iter()
            .find(|r| r.tweak_id == "disable_mpo")
            .expect("disable_mpo missing");
        assert_eq!(mpo.current_state, TweakState::NotApplicable);
        assert_eq!(
            mpo.detail_kind,
            Some(ScanDetailKind::NotApplicableMaxBuild {
                current: 26100,
                max_build: 25999,
            })
        );
    }

    #[test]
    fn test_scan_single_tweak_detail_kind_every_action_and_na_branch() {
        use crate::core::backend::SystemBackend;
        use crate::core::risk::{DynamicDwordFormula, OptionalFeatureKind, PowerTargetScope};

        let sbx = SandboxEnv::new();
        sbx.reg_set_dword("HKCU", "Software\\Test", "DwordOpt", 1)
            .unwrap();
        sbx.reg_set_string("HKCU", "Software\\Test", "StrVal", "hello", false, None)
            .unwrap();
        sbx.reg_set_string(
            "HKCU",
            "Software\\Test",
            "CompVal",
            "Alpha=1;Beta=2;",
            false,
            None,
        )
        .unwrap();
        sbx.reg_set_dword("HKCU", "Software\\Test", "KeepMe", 1)
            .unwrap();
        sbx.reg_set_dword("HKCU", "Software\\Test\\Tree", "Leaf", 1)
            .unwrap();
        sbx.reg_set_binary("HKCU", "Software\\Test", "BinOk", &[0x00, 0xA4, 0xFF])
            .unwrap();
        sbx.reg_set_binary("HKCU", "Software\\Test", "BinShort", &[0x01])
            .unwrap();
        sbx.reg_set_dword("HKLM", "SYSTEM\\Test", "DynMem", 33_554_432)
            .unwrap();
        sbx.write_power_setting_index(
            None,
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-dec35c318583",
            Some(100),
            Some(50),
            false,
        )
        .unwrap();
        {
            let mut st = sbx.inner.write().unwrap();
            st.gpus
                .retain(|g| g.vendor == crate::hardware::gpu::GpuVendor::Nvidia);
            st.services.insert("diagtrack".to_string(), 4);
            st.tasks.insert("\\Microsoft\\TestTask".to_string(), false);
            st.appx_packages
                .insert("microsoft.bingweather_4.0_x64__8wekyb3d8bbwe".to_string());
        }

        // 1. NotApplicable branches: min_build, max_build, edition, gpu
        let min_build_tweak = mk_test_tweak("min_build", vec![]);
        let r_min = scan_single_tweak_with(&sbx, &min_build_tweak, 18363, "Pro");
        assert_eq!(r_min.current_state, TweakState::NotApplicable);
        assert_eq!(
            r_min.detail_kind,
            Some(ScanDetailKind::NotApplicableBuild {
                current: 18363,
                min_build: 19041,
            })
        );
        assert_eq!(r_min.sub_category.as_deref(), Some("Power"));

        let mut max_build_tweak = mk_test_tweak("max_build", vec![]);
        max_build_tweak.os_req.max_build = Some(22631);
        let r_max = scan_single_tweak_with(&sbx, &max_build_tweak, 26100, "Pro");
        assert_eq!(
            r_max.detail_kind,
            Some(ScanDetailKind::NotApplicableMaxBuild {
                current: 26100,
                max_build: 22631,
            })
        );

        let mut edition_tweak = mk_test_tweak("edition", vec![]);
        edition_tweak.os_req.editions = vec!["Enterprise".to_string()];
        let r_ed = scan_single_tweak_with(&sbx, &edition_tweak, 26100, "Pro");
        assert_eq!(
            r_ed.detail_kind,
            Some(ScanDetailKind::NotApplicableEdition {
                edition: "Pro".to_string(),
                required: "Enterprise".to_string(),
            })
        );

        let mut gpu_tweak = mk_test_tweak("gpu", vec![]);
        gpu_tweak.gpu_vendor = Some("Intel".to_string());
        let r_gpu = scan_single_tweak_with(&sbx, &gpu_tweak, 26100, "Pro");
        assert_eq!(
            r_gpu.detail_kind,
            Some(ScanDetailKind::NotApplicableGpu {
                vendor: "Intel".to_string(),
            })
        );

        // 2. Empty actions tweak
        let empty_tweak = mk_test_tweak("empty", vec![]);
        let r_empty = scan_single_tweak_with(&sbx, &empty_tweak, 26100, "Pro");
        assert_eq!(r_empty.current_state, TweakState::NotApplicable);
        assert_eq!(
            r_empty.detail_kind,
            Some(ScanDetailKind::CompoundOptimized {
                optimized: 0,
                total: 0,
            })
        );

        // 3. Every TweakAction variant
        let cases: Vec<(TweakAction, ScanDetailKind)> = vec![
            (
                TweakAction::RegistryDword {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "DwordOpt".to_string(),
                    optimized_value: 1,
                    default_value: 0,
                },
                ScanDetailKind::DwordOptimized { value: 1 },
            ),
            (
                TweakAction::RegistryString {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "StrVal".to_string(),
                    optimized_value: "hello".to_string(),
                    default_value: "world".to_string(),
                    expand_sz: false,
                    absent_is_default: false,
                    case_insensitive: false,
                },
                ScanDetailKind::RegStringValue {
                    value: "hello".to_string(),
                },
            ),
            (
                TweakAction::RegistryString {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "MissingStr".to_string(),
                    optimized_value: "hello".to_string(),
                    default_value: "world".to_string(),
                    expand_sz: false,
                    absent_is_default: false,
                    case_insensitive: false,
                },
                ScanDetailKind::RegNotConfiguredStock,
            ),
            (
                TweakAction::RegistryDelete {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "Gone".to_string(),
                },
                ScanDetailKind::RegValueRemoved,
            ),
            (
                TweakAction::RegistryDelete {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "KeepMe".to_string(),
                },
                ScanDetailKind::RegValuePresent,
            ),
            (
                TweakAction::RegistryDeleteTree {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test\\GoneTree".to_string(),
                },
                ScanDetailKind::SubtreeRemoved,
            ),
            (
                TweakAction::RegistryDeleteTree {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test\\Tree".to_string(),
                },
                ScanDetailKind::SubtreePresent,
            ),
            (
                TweakAction::RegistryBinaryBit {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "BinOk".to_string(),
                    byte_index: 1,
                    min_length: 2,
                    bit_mask: 0x04,
                    optimized_bits: 0x04,
                    default_bits: 0x00,
                    create_if_missing: false,
                    default_full_hex: None,
                },
                ScanDetailKind::BinaryMatch {
                    index: 1,
                    byte_val: 0xA4,
                    mask: 0x04,
                },
            ),
            (
                TweakAction::RegistryBinaryBit {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "BinShort".to_string(),
                    byte_index: 2,
                    min_length: 3,
                    bit_mask: 0x01,
                    optimized_bits: 0x01,
                    default_bits: 0x00,
                    create_if_missing: false,
                    default_full_hex: None,
                },
                ScanDetailKind::BinaryTooShort {
                    required: 3,
                    actual: 1,
                },
            ),
            (
                TweakAction::RegistryBinaryBit {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "BinMissing".to_string(),
                    byte_index: 0,
                    min_length: 1,
                    bit_mask: 0x01,
                    optimized_bits: 0x01,
                    default_bits: 0x00,
                    create_if_missing: false,
                    default_full_hex: None,
                },
                ScanDetailKind::BinaryNotConfigured,
            ),
            (
                TweakAction::CompositeStringKey {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "CompVal".to_string(),
                    entry_key: "Alpha".to_string(),
                    optimized_entry_value: "1".to_string(),
                    default_entry_value: "0".to_string(),
                    pair_separator: ";".to_string(),
                    kv_separator: "=".to_string(),
                    trailing_separator: true,
                },
                ScanDetailKind::CompositeSet {
                    key: "Alpha".to_string(),
                    value: "1".to_string(),
                },
            ),
            (
                TweakAction::CompositeStringKey {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "CompVal".to_string(),
                    entry_key: "Gamma".to_string(),
                    optimized_entry_value: "1".to_string(),
                    default_entry_value: "0".to_string(),
                    pair_separator: ";".to_string(),
                    kv_separator: "=".to_string(),
                    trailing_separator: true,
                },
                ScanDetailKind::CompositeNotSet {
                    key: "Gamma".to_string(),
                },
            ),
            (
                TweakAction::RegistryDynamicDword {
                    hive: "HKLM".to_string(),
                    path: "SYSTEM\\Test".to_string(),
                    name: "DynMem".to_string(),
                    formula: DynamicDwordFormula::TotalRamInKb,
                    default_value: 0,
                    tolerance_pct: 0,
                },
                ScanDetailKind::DynCurrent {
                    current: 33_554_432,
                    target: 33_554_432,
                },
            ),
            (
                TweakAction::RegistryDynamicDword {
                    hive: "HKLM".to_string(),
                    path: "SYSTEM\\Test".to_string(),
                    name: "DynMissing".to_string(),
                    formula: DynamicDwordFormula::LogicalCoreCount,
                    default_value: 0,
                    tolerance_pct: 0,
                },
                ScanDetailKind::DynNotConfigured { target: 8 },
            ),
            (
                TweakAction::PowerCfgSetting {
                    subgroup_guid: "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                    setting_guid: "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                    scheme_guid: None,
                    optimized_ac: 100,
                    optimized_dc: 50,
                    default_ac: 5,
                    default_dc: 5,
                    scope: PowerTargetScope::AcAndDc,
                    unhide_attribute: false,
                },
                ScanDetailKind::PowerIndex { ac: 100, dc: 50 },
            ),
            (
                TweakAction::ServiceState {
                    service_name: "DiagTrack".to_string(),
                    target_start_type: 4,
                    default_start_type: 2,
                },
                ScanDetailKind::ServiceOptimized { mode: 4 },
            ),
            (
                TweakAction::ScheduledTask {
                    task_path: "\\Microsoft\\TestTask".to_string(),
                    enable: false,
                },
                ScanDetailKind::TaskFound { active: false },
            ),
            (
                TweakAction::AppXPackage {
                    package_family: "Microsoft.BingWeather_8wekyb3d8bbwe".to_string(),
                },
                ScanDetailKind::AppxInstalled {
                    package: "microsoft.bingweather_4.0_x64__8wekyb3d8bbwe".to_string(),
                },
            ),
            (
                TweakAction::WindowsFeature {
                    feature_name: "Recall".to_string(),
                    kind: OptionalFeatureKind::OptionalFeature,
                    enable: false,
                },
                ScanDetailKind::FeatureState {
                    state: crate::core::risk::FeatureInstallState::Enabled,
                },
            ),
            (
                TweakAction::WindowsFeature {
                    feature_name: "NonExistentFeatureXYZ".to_string(),
                    kind: OptionalFeatureKind::OptionalFeature,
                    enable: false,
                },
                ScanDetailKind::FeatureNotRegistered,
            ),
        ];

        for (idx, (action, expected_kind)) in cases.into_iter().enumerate() {
            let tw = mk_test_tweak(&format!("case_{}", idx), vec![action]);
            let res = scan_single_tweak_with(&sbx, &tw, 26100, "Pro");
            assert_eq!(
                res.detail_kind,
                Some(expected_kind),
                "Mismatch at action case {}",
                idx
            );
        }

        // PowerNotExposed when all actions are unsupported power GUIDs
        let pwr_absent = mk_test_tweak(
            "pwr_absent",
            vec![TweakAction::PowerCfgSetting {
                subgroup_guid: "54533251-82be-4824-96c1-47b60b740d00".to_string(),
                setting_guid: "ffffffff-ffff-ffff-ffff-ffffffffffff".to_string(),
                scheme_guid: None,
                optimized_ac: 0,
                optimized_dc: 0,
                default_ac: 1,
                default_dc: 1,
                scope: PowerTargetScope::AcAndDc,
                unhide_attribute: false,
            }],
        );
        let r_pwr_absent = scan_single_tweak_with(&sbx, &pwr_absent, 26100, "Pro");
        assert_eq!(r_pwr_absent.current_state, TweakState::NotApplicable);
        assert!(matches!(
            r_pwr_absent.detail_kind,
            Some(ScanDetailKind::PowerNotExposed { .. })
        ));

        // PowerUnavailable when GUID parsing fails
        let pwr_bad = mk_test_tweak(
            "pwr_bad",
            vec![TweakAction::PowerCfgSetting {
                subgroup_guid: "invalid-guid".to_string(),
                setting_guid: "0cc5b647-c1df-4637-891a-dec35c318583".to_string(),
                scheme_guid: None,
                optimized_ac: 0,
                optimized_dc: 0,
                default_ac: 1,
                default_dc: 1,
                scope: PowerTargetScope::AcAndDc,
                unhide_attribute: false,
            }],
        );
        let r_pwr_bad = scan_single_tweak_with(&sbx, &pwr_bad, 26100, "Pro");
        assert!(matches!(
            r_pwr_bad.detail_kind,
            Some(ScanDetailKind::PowerUnavailable { .. })
        ));

        // FeatureRequiresAdmin when not elevated
        sbx.inner.write().unwrap().elevated = false;
        let feat_admin = mk_test_tweak(
            "feat_admin",
            vec![TweakAction::WindowsFeature {
                feature_name: "Recall".to_string(),
                kind: OptionalFeatureKind::OptionalFeature,
                enable: false,
            }],
        );
        let r_feat_admin = scan_single_tweak_with(&sbx, &feat_admin, 26100, "Pro");
        assert_eq!(r_feat_admin.current_state, TweakState::NotApplicable);
        assert_eq!(
            r_feat_admin.detail_kind,
            Some(ScanDetailKind::FeatureRequiresAdmin)
        );

        // Compound tweak -> CompoundOptimized + serde backward compatibility
        let compound = mk_test_tweak(
            "compound",
            vec![
                TweakAction::RegistryDword {
                    hive: "HKCU".to_string(),
                    path: "Software\\Test".to_string(),
                    name: "DwordOpt".to_string(),
                    optimized_value: 1,
                    default_value: 0,
                },
                TweakAction::ServiceState {
                    service_name: "DiagTrack".to_string(),
                    target_start_type: 2,
                    default_start_type: 2,
                },
            ],
        );
        let r_compound = scan_single_tweak_with(&sbx, &compound, 26100, "Pro");
        assert_eq!(r_compound.current_state, TweakState::PartiallyOptimized);
        assert_eq!(
            r_compound.detail_kind,
            Some(ScanDetailKind::CompoundOptimized {
                optimized: 1,
                total: 2,
            })
        );

        // Serde backward compatibility when sub_category/detail_kind are omitted in legacy JSON
        let json = serde_json::to_string(&r_compound).unwrap();
        let mut val: serde_json::Value = serde_json::from_str(&json).unwrap();
        val.as_object_mut().unwrap().remove("sub_category");
        val.as_object_mut().unwrap().remove("detail_kind");
        let legacy: ScanResult = serde_json::from_value(val).unwrap();
        assert_eq!(legacy.sub_category, None);
        assert_eq!(legacy.detail_kind, None);
    }
}
