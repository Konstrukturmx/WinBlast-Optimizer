# Integration Suite: Ui Responsive E2E

> 21 nodes · cohesion 0.19

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **set_raw()** (9 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree()** (6 connections) — `src/executor/reg_mutator.rs`
- **test_strict_reg_type_dispatch_and_negative_numeric_sz()** (6 connections) — `src/scanner/registry_scanner.rs`
- **delete_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **set_dword()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **ScratchCleanup** (3 connections) — `src/scanner/registry_scanner.rs`
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)
- **.drop()** (1 connections) — `src/scanner/registry_scanner.rs`

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (21 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (7 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (7 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (6 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 64 (71%)
- INFERRED: 26 (29%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*