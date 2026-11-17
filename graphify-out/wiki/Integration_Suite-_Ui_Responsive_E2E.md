# Integration Suite: Ui Responsive E2E

> 29 nodes · cohesion 0.12

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (9 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree()** (6 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (6 connections) — `src/executor/reg_mutator.rs`
- **delete_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **.reg_restore_tree()** (3 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (3 connections) — `src/core/backend.rs`
- **delete_value()** (3 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **set_dword()** (3 connections) — `src/executor/reg_mutator.rs`
- **.reg_delete_tree()** (2 connections) — `src/core/backend.rs`
- **.reg_delete_value()** (2 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_set_binary()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_set_string()** (2 connections) — `src/core/backend.rs`
- *... and 4 more nodes in this community*

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (10 shared connections)
- [RealBackend](RealBackend.md) (9 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (7 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (5 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (3 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [engine.rs](engine.rs.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 79 (81%)
- INFERRED: 18 (19%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*