# clean_apps.rs

> 40 nodes · cohesion 0.13

## Key Concepts

- **engine.rs** (53 connections) — `src/executor/engine.rs`
- **reg_helpers.rs** (24 connections) — `src/core/reg_helpers.rs`
- **rollback_entry_with()** (22 connections) — `src/executor/engine.rs`
- **apply_tweak_with()** (20 connections) — `src/executor/engine.rs`
- **revert_tweak_with()** (19 connections) — `src/executor/engine.rs`
- **apply_action_slice_with()** (17 connections) — `src/executor/engine.rs`
- **Result** (14 connections) — `src/core/error.rs`
- **is_unsupported_power_guid_error()** (12 connections) — `src/core/backend.rs`
- **format_pipe_reg_target()** (10 connections) — `src/core/reg_helpers.rs`
- **decode_hex()** (8 connections) — `src/core/reg_helpers.rs`
- **escape_pipe()** (7 connections) — `src/core/reg_helpers.rs`
- **merge_composite_pair()** (7 connections) — `src/core/reg_helpers.rs`
- **parse_pipe_reg_target()** (6 connections) — `src/core/reg_helpers.rs`
- **apply_tweaks_by_id_with()** (6 connections) — `src/executor/engine.rs`
- **format_composite_action_key()** (6 connections) — `src/executor/engine.rs`
- **revert_tweaks_by_id_with()** (6 connections) — `src/executor/engine.rs`
- **rollback_recorded_ids()** (6 connections) — `src/executor/engine.rs`
- **ensure_tweak_compatible()** (5 connections) — `src/executor/engine.rs`
- **format_dword_action_key()** (5 connections) — `src/executor/engine.rs`
- **read_composite_entry()** (4 connections) — `src/core/reg_helpers.rs`
- **unescape_pipe()** (4 connections) — `src/core/reg_helpers.rs`
- **apply_tweak()** (4 connections) — `src/executor/engine.rs`
- **apply_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **format_power_indices()** (4 connections) — `src/executor/engine.rs`
- **revert_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- *... and 15 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (27 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (14 shared connections)
- [startup.rs](startup.rs.md) (10 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (8 shared connections)
- [DriverError](DriverError.md) (7 shared connections)
- [parking.rs](parking.rs.md) (6 shared connections)
- [WorkerEvent](WorkerEvent.md) (5 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (4 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (4 shared connections)
- [theme.rs](theme.rs.md) (4 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [TaskCache](TaskCache.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/error.rs`
- `src/core/reg_helpers.rs`
- `src/executor/engine.rs`
- `tests/fuzz_mutation_properties.rs`

## Audit Trail

- EXTRACTED: 177 (83%)
- INFERRED: 35 (17%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*