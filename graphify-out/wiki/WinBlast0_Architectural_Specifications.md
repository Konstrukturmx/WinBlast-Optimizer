# WinBlast0 Architectural Specifications

> 24 nodes · cohesion 0.13

## Key Concepts

- **FeatureInstallState** (26 connections) — `src/core/risk.rs`
- **OptionalFeatureKind** (21 connections) — `src/core/risk.rs`
- **decode_oem_bytes()** (14 connections) — `src/scanner/task_scanner.rs`
- **dism_error_from_output()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_capabilities_locked()** (7 connections) — `src/tools/features_apps.rs`
- **curated_windows_features_meta()** (6 connections) — `src/tools/features_apps.rs`
- **query_dism_features_locked()** (6 connections) — `src/tools/features_apps.rs`
- **set_optional_feature()** (6 connections) — `src/tools/features_apps.rs`
- **list_optional_features()** (5 connections) — `src/tools/features_apps.rs`
- **parse_dism_english_state()** (4 connections) — `src/tools/features_apps.rs`
- **resolve_capability_identity_locked()** (4 connections) — `src/tools/features_apps.rs`
- **.set_optional_feature()** (3 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (3 connections) — `src/core/backend.rs`
- **request_system_reboot()** (2 connections) — `src/tools/explorer_ctrl.rs`
- **remove_edge_preserve_webview2()** (2 connections) — `src/tools/features_apps.rs`
- **validate_feature_name()** (2 connections) — `src/tools/features_apps.rs`
- **Disabled** (1 connections) — `src/core/risk.rs`
- **DisablePending** (1 connections) — `src/core/risk.rs`
- **Enabled** (1 connections) — `src/core/risk.rs`
- **EnablePending** (1 connections) — `src/core/risk.rs`
- **Removed** (1 connections) — `src/core/risk.rs`
- **Capability** (1 connections) — `src/core/risk.rs`
- **OptionalFeature** (1 connections) — `src/core/risk.rs`
- **Output** (1 connections)

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (15 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (4 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (3 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (2 shared connections)
- [engine.rs](engine.rs.md) (2 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (2 shared connections)
- [Bloatware & AppX Deprovisioning Rules](Bloatware_&_AppX_Deprovisioning_Rules.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/explorer_ctrl.rs`
- `src/tools/features_apps.rs`

## Audit Trail

- EXTRACTED: 87 (96%)
- INFERRED: 4 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*