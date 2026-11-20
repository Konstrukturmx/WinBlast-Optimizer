# updater.rs

> 22 nodes · cohesion 0.23

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **normalize_reg_value_name()** (21 connections) — `src/core/risk.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (8 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (6 connections) — `src/executor/reg_mutator.rs`
- **restore_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **delete_tree()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **set_dword()** (3 connections) — `src/executor/reg_mutator.rs`
- **delete_value()** (2 connections) — `src/executor/reg_mutator.rs`
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)

## Relationships

- [registry_scanner.rs](registry_scanner.rs.md) (20 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (7 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (7 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [DriverUpdateState](DriverUpdateState.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [enums](enums.md) (2 shared connections)
- [RealBackend](RealBackend.md) (2 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/core/risk.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 72 (67%)
- INFERRED: 36 (33%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*