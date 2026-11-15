# WinBlast0 Architectural Specifications

> 12 nodes · cohesion 0.21

## Key Concepts

- **deserialize** (17 connections)
- **serialize** (16 connections)
- **profile_io.rs** (8 connections) — `src/backup/profile_io.rs`
- **path** (6 connections)
- **restore_point.rs** (5 connections) — `src/backup/restore_point.rs`
- **export_profile()** (4 connections) — `src/backup/profile_io.rs`
- **import_profile()** (3 connections) — `src/backup/profile_io.rs`
- **test_profile_export_import()** (3 connections) — `src/backup/profile_io.rs`
- **create_restore_point()** (3 connections) — `src/backup/restore_point.rs`
- **TweakProfile** (2 connections) — `src/backup/profile_io.rs`
- **.new()** (2 connections) — `src/backup/profile_io.rs`
- **.create_restore_point()** (2 connections) — `src/core/backend.rs`

## Relationships

- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (7 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (6 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (4 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (3 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (3 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (2 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (2 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [View() Card() Switch](View_Card_Switch.md) (2 shared connections)

## Source Files

- `src/backup/profile_io.rs`
- `src/backup/restore_point.rs`
- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 57 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*