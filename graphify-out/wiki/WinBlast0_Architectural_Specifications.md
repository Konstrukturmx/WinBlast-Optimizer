# WinBlast0 Architectural Specifications

> 23 nodes · cohesion 0.14

## Key Concepts

- **FeatureInstallState** (26 connections) — `src/core/risk.rs`
- **OptionalFeatureKind** (21 connections) — `src/core/risk.rs`
- **curated_windows_features_meta()** (7 connections) — `src/tools/features_apps.rs`
- **dism_error_from_output()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_capabilities_locked()** (7 connections) — `src/tools/features_apps.rs`
- **set_optional_feature()** (7 connections) — `src/tools/features_apps.rs`
- **.set_optional_feature()** (6 connections) — `src/core/backend.rs`
- **list_optional_features()** (6 connections) — `src/tools/features_apps.rs`
- **query_dism_features_locked()** (6 connections) — `src/tools/features_apps.rs`
- **.set_optional_feature()** (4 connections) — `src/core/backend.rs`
- **parse_dism_english_state()** (4 connections) — `src/tools/features_apps.rs`
- **resolve_capability_identity_locked()** (4 connections) — `src/tools/features_apps.rs`
- **optional_feature_matches()** (3 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (3 connections) — `src/core/backend.rs`
- **validate_feature_name()** (3 connections) — `src/tools/features_apps.rs`
- **Disabled** (1 connections) — `src/core/risk.rs`
- **DisablePending** (1 connections) — `src/core/risk.rs`
- **Enabled** (1 connections) — `src/core/risk.rs`
- **EnablePending** (1 connections) — `src/core/risk.rs`
- **Removed** (1 connections) — `src/core/risk.rs`
- **Capability** (1 connections) — `src/core/risk.rs`
- **OptionalFeature** (1 connections) — `src/core/risk.rs`
- **Output** (1 connections)

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (11 shared connections)
- [Integration Suite: Real Windows Env](Integration_Suite-_Real_Windows_Env.md) (6 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [Startup Entries() .List](Startup_Entries_.List.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (3 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (2 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/tools/features_apps.rs`

## Audit Trail

- EXTRACTED: 85 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*