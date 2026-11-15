# Task Scheduler Inspection Subsystem

> 21 nodes · cohesion 0.18

## Key Concepts

- **engine.rs** (53 connections) — `src/executor/engine.rs`
- **rollback_entry_with()** (21 connections) — `src/executor/engine.rs`
- **apply_tweak_with()** (17 connections) — `src/executor/engine.rs`
- **apply_tweaks_by_id_with()** (7 connections) — `src/executor/engine.rs`
- **parse_pipe_reg_target()** (6 connections) — `src/core/reg_helpers.rs`
- **ensure_tweak_compatible()** (5 connections) — `src/executor/engine.rs`
- **unescape_pipe()** (4 connections) — `src/core/reg_helpers.rs`
- **apply_tweak()** (4 connections) — `src/executor/engine.rs`
- **apply_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **rollback_entry()** (4 connections) — `src/executor/engine.rs`
- **test_batch1_engine_edge_cases_and_atomic_revert()** (4 connections) — `src/executor/engine.rs`
- **parse_feature_kind()** (3 connections) — `src/executor/engine.rs`
- **rollback_by_id()** (3 connections) — `src/executor/engine.rs`
- **test_absent_key_rolls_back_by_deleting()** (3 connections) — `src/executor/engine.rs`
- **parse_power_indices()** (2 connections) — `src/executor/engine.rs`
- **test_apply_tweak_fallback_execution()** (2 connections) — `src/executor/engine.rs`
- **test_core_parking_snapshot_and_legacy_rollback()** (2 connections) — `src/executor/engine.rs`
- **test_rollback_revert_prefix_in_sandbox()** (2 connections) — `src/executor/engine.rs`
- **ABSENT_SENTINEL** (1 connections) — `src/executor/engine.rs`
- **SUBKEY_ABSENT_SENTINEL** (1 connections) — `src/executor/engine.rs`
- **VAL_ABSENT_SENTINEL** (1 connections) — `src/executor/engine.rs`

## Relationships

- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (20 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (14 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (11 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (5 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (3 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [Integration Suite: Real Windows Env](Integration_Suite-_Real_Windows_Env.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 99 (89%)
- INFERRED: 12 (11%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*