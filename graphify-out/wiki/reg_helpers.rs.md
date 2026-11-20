# reg_helpers.rs

> 12 nodes · cohesion 0.24

## Key Concepts

- **reg_helpers.rs** (24 connections) — `src/core/reg_helpers.rs`
- **Result** (14 connections) — `src/core/error.rs`
- **parse_pipe_reg_target()** (6 connections) — `src/core/reg_helpers.rs`
- **is_known_numeric_reg_sz()** (5 connections) — `src/core/reg_helpers.rs`
- **read_composite_entry()** (4 connections) — `src/core/reg_helpers.rs`
- **unescape_pipe()** (4 connections) — `src/core/reg_helpers.rs`
- **parse_composite_pairs()** (3 connections) — `src/core/reg_helpers.rs`
- **test_composite_string_merge_preserves_siblings()** (3 connections) — `src/core/reg_helpers.rs`
- **test_pipe_escape_handles_default_and_backslashes()** (3 connections) — `src/core/reg_helpers.rs`
- **test_hkcr_and_shallow_tree_delete_blocked()** (2 connections) — `src/core/reg_helpers.rs`
- **test_known_numeric_reg_sz()** (2 connections) — `src/core/reg_helpers.rs`
- **test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()** (2 connections) — `tests/real_windows_env.rs`

## Relationships

- [RealBackend](RealBackend.md) (12 shared connections)
- [updater.rs](updater.rs.md) (7 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (3 shared connections)
- [DriverUpdateState](DriverUpdateState.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [elevation.rs](elevation.rs.md) (1 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)

## Source Files

- `src/core/error.rs`
- `src/core/reg_helpers.rs`
- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 48 (86%)
- INFERRED: 8 (14%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*