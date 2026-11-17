# engine.rs

> 39 nodes · cohesion 0.13

## Key Concepts

- **engine.rs** (53 connections) — `src/executor/engine.rs`
- **reg_helpers.rs** (24 connections) — `src/core/reg_helpers.rs`
- **rollback_entry_with()** (21 connections) — `src/executor/engine.rs`
- **revert_tweak_with()** (20 connections) — `src/executor/engine.rs`
- **apply_tweak_with()** (18 connections) — `src/executor/engine.rs`
- **apply_action_slice_with()** (16 connections) — `src/executor/engine.rs`
- **is_unsupported_power_guid_error()** (10 connections) — `src/core/backend.rs`
- **decode_hex()** (10 connections) — `src/core/reg_helpers.rs`
- **format_pipe_reg_target()** (8 connections) — `src/core/reg_helpers.rs`
- **merge_composite_pair()** (8 connections) — `src/core/reg_helpers.rs`
- **escape_pipe()** (7 connections) — `src/core/reg_helpers.rs`
- **revert_tweaks_by_id_with()** (7 connections) — `src/executor/engine.rs`
- **parse_pipe_reg_target()** (6 connections) — `src/core/reg_helpers.rs`
- **format_composite_action_key()** (6 connections) — `src/executor/engine.rs`
- **rollback_recorded_ids()** (6 connections) — `src/executor/engine.rs`
- **ensure_tweak_compatible()** (5 connections) — `src/executor/engine.rs`
- **format_dword_action_key()** (5 connections) — `src/executor/engine.rs`
- **unescape_pipe()** (4 connections) — `src/core/reg_helpers.rs`
- **apply_tweak()** (4 connections) — `src/executor/engine.rs`
- **format_power_indices()** (4 connections) — `src/executor/engine.rs`
- **revert_tweak()** (4 connections) — `src/executor/engine.rs`
- **revert_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **test_batch1_engine_edge_cases_and_atomic_revert()** (4 connections) — `src/executor/engine.rs`
- **parse_composite_pairs()** (3 connections) — `src/core/reg_helpers.rs`
- **read_composite_entry()** (3 connections) — `src/core/reg_helpers.rs`
- *... and 14 more nodes in this community*

## Relationships

- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (18 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (15 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (8 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (7 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (5 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (5 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (5 shared connections)
- [AppState](AppState.md) (4 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (3 shared connections)
- [super](super.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 161 (83%)
- INFERRED: 32 (17%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*