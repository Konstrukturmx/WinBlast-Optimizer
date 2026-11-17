# Win32 Registry Scanner & Mutator

> 23 nodes · cohesion 0.25

## Key Concepts

- **SandboxState** (32 connections) — `src/core/backend.rs`
- **.seed_actions_with_vendor()** (18 connections) — `src/core/backend.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **.reg_key()** (14 connections) — `src/core/backend.rs`
- **.normalize_hive()** (12 connections) — `src/core/backend.rs`
- **.ensure_tree_path_in_state()** (10 connections) — `src/core/backend.rs`
- **.resolve_target_paths_in_state()** (10 connections) — `src/core/backend.rs`
- **.write_dword_in_state()** (9 connections) — `src/core/backend.rs`
- **.write_raw_in_state()** (9 connections) — `src/core/backend.rs`
- **.clear_reg_value_in_state()** (8 connections) — `src/core/backend.rs`
- **.normalize_reg_path()** (8 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (8 connections) — `src/core/backend.rs`
- **.reg_set_string()** (8 connections) — `src/core/backend.rs`
- **.restore_tree_into_state()** (8 connections) — `src/core/backend.rs`
- **.reg_set_binary()** (7 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (7 connections) — `src/core/backend.rs`
- **.reg_delete_tree()** (5 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (5 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (5 connections) — `src/core/backend.rs`
- **.reg_get_string()** (4 connections) — `src/core/backend.rs`
- **.reg_snapshot_tree()** (4 connections) — `src/core/backend.rs`
- **.resolve_net_interface_paths_in_state()** (4 connections) — `src/core/backend.rs`
- **.reg_key_exists()** (3 connections) — `src/core/backend.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (43 shared connections)
- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (7 shared connections)
- [engine.rs](engine.rs.md) (5 shared connections)
- [backend.rs](backend.rs.md) (5 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (3 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (1 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`

## Audit Trail

- EXTRACTED: 142 (93%)
- INFERRED: 11 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*