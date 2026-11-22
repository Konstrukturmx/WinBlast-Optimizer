# Changelog

> 22 nodes · cohesion 0.23

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **normalize_reg_value_name()** (21 connections) — `src/core/risk.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (9 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree()** (6 connections) — `src/executor/reg_mutator.rs`
- **delete_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **set_dword()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **delete_value()** (3 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)

## Relationships

- [reg_mutator.rs](reg_mutator.rs.md) (15 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (7 shared connections)
- [driver_update.rs](driver_update.rs.md) (7 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (4 shared connections)
- [.all_physical](all_physical.md) (4 shared connections)
- [dijkstra.rs](dijkstra.rs.md) (4 shared connections)
- [AppxCache](AppxCache.md) (4 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [adaptive_tuning.rs](adaptive_tuning.rs.md) (3 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (2 shared connections)
- [RegStringValue](RegStringValue.md) (2 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/core/risk.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 81 (71%)
- INFERRED: 33 (29%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*