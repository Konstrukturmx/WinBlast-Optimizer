# FeatureInstallState

> 25 nodes · cohesion 0.12

## Key Concepts

- **FeatureInstallState** (26 connections) — `src/core/risk.rs`
- **OptionalFeatureKind** (21 connections) — `src/core/risk.rs`
- **decode_oem_bytes()** (18 connections) — `src/scanner/task_scanner.rs`
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
- **run_scheduled_task()** (2 connections) — `src/tools/startup.rs`
- **Disabled** (1 connections) — `src/core/risk.rs`
- **DisablePending** (1 connections) — `src/core/risk.rs`
- **Enabled** (1 connections) — `src/core/risk.rs`
- **EnablePending** (1 connections) — `src/core/risk.rs`
- **Removed** (1 connections) — `src/core/risk.rs`
- **Capability** (1 connections) — `src/core/risk.rs`
- **OptionalFeature** (1 connections) — `src/core/risk.rs`
- **Output** (1 connections)

## Relationships

- [features_apps.rs](features_apps.rs.md) (15 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (5 shared connections)
- [.set_optional_feature](set_optional_feature.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (4 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (4 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (3 shared connections)
- [driver_update.rs](driver_update.rs.md) (3 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [super](super.md) (2 shared connections)
- [RealBackend](RealBackend.md) (2 shared connections)
- [.set_status_spec](set_status_spec.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/explorer_ctrl.rs`
- `src/tools/features_apps.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 92 (96%)
- INFERRED: 4 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*