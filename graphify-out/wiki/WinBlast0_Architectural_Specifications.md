# WinBlast0 Architectural Specifications

> 21 nodes · cohesion 0.14

## Key Concepts

- **FeatureInstallState** (26 connections) — `src/core/risk.rs`
- **WindowsOptionalFeature** (18 connections) — `src/tools/features_apps.rs`
- **dism_error_from_output()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_capabilities_locked()** (7 connections) — `src/tools/features_apps.rs`
- **set_optional_feature()** (7 connections) — `src/tools/features_apps.rs`
- **list_optional_features()** (6 connections) — `src/tools/features_apps.rs`
- **query_dism_features_locked()** (6 connections) — `src/tools/features_apps.rs`
- **.list_optional_features()** (4 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (4 connections) — `src/core/backend.rs`
- **parse_dism_english_state()** (4 connections) — `src/tools/features_apps.rs`
- **resolve_capability_identity_locked()** (4 connections) — `src/tools/features_apps.rs`
- **.optional_features()** (3 connections) — `src/core/backend.rs`
- **.list_optional_features()** (3 connections) — `src/core/backend.rs`
- **validate_feature_name()** (3 connections) — `src/tools/features_apps.rs`
- **.list_optional_features()** (2 connections) — `src/core/backend.rs`
- **Disabled** (1 connections) — `src/core/risk.rs`
- **DisablePending** (1 connections) — `src/core/risk.rs`
- **Enabled** (1 connections) — `src/core/risk.rs`
- **EnablePending** (1 connections) — `src/core/risk.rs`
- **Removed** (1 connections) — `src/core/risk.rs`
- **Output** (1 connections)

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (10 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (3 shared connections)
- [BlastResult](BlastResult.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [RealBackend](RealBackend.md) (2 shared connections)
- [Test Composite With()](Test_Composite_With.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/tools/features_apps.rs`

## Audit Trail

- EXTRACTED: 79 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*