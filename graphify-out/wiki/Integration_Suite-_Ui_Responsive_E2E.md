# Integration Suite: Ui Responsive E2E

> 24 nodes · cohesion 0.13

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (6 connections) — `src/executor/reg_mutator.rs`
- **set_raw()** (6 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (6 connections) — `src/executor/reg_mutator.rs`
- **is_known_numeric_reg_sz()** (5 connections) — `src/core/reg_helpers.rs`
- **restore_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (5 connections) — `src/executor/reg_mutator.rs`
- **set_dword()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (4 connections) — `src/executor/reg_mutator.rs`
- **.reg_restore_tree()** (3 connections) — `src/core/backend.rs`
- **RegKey** (3 connections)
- **.reg_set_binary()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_set_string()** (2 connections) — `src/core/backend.rs`
- **test_known_numeric_reg_sz()** (2 connections) — `src/core/reg_helpers.rs`
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()** (2 connections) — `tests/real_windows_env.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (12 shared connections)
- [RealBackend](RealBackend.md) (5 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (3 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (3 shared connections)
- [engine.rs](engine.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [delete_value_for_vendor](delete_value_for_vendor.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`
- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 53 (75%)
- INFERRED: 18 (25%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*