# DriverError

> 24 nodes · cohesion 0.18

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (8 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (6 connections) — `src/executor/reg_mutator.rs`
- **is_known_numeric_reg_sz()** (5 connections) — `src/core/reg_helpers.rs`
- **restore_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **delete_tree()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **set_dword()** (3 connections) — `src/executor/reg_mutator.rs`
- **test_hkcr_and_shallow_tree_delete_blocked()** (2 connections) — `src/core/reg_helpers.rs`
- **test_known_numeric_reg_sz()** (2 connections) — `src/core/reg_helpers.rs`
- **delete_value()** (2 connections) — `src/executor/reg_mutator.rs`
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (15 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (7 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (5 shared connections)
- [TaskCache](TaskCache.md) (5 shared connections)
- [.new](new.md) (3 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (2 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [power.rs](power.rs.md) (1 shared connections)
- [CoreParkingState](CoreParkingState.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 66 (68%)
- INFERRED: 31 (32%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*