# Core UI Internationalization & Text Catalog

> 37 nodes · cohesion 0.10

## Key Concepts

- **registry_scanner.rs** (37 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_read_64()** (13 connections) — `src/scanner/registry_scanner.rs`
- **get_string_for_vendor()** (8 connections) — `src/scanner/registry_scanner.rs`
- **RegKey** (8 connections)
- **get_dword_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_raw()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_binary()** (6 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_key_recursive()** (6 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_tree()** (6 connections) — `src/scanner/registry_scanner.rs`
- **get_dword()** (5 connections) — `src/scanner/registry_scanner.rs`
- **resolve_net_interface_paths()** (5 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value_with_kind()** (4 connections) — `src/scanner/registry_scanner.rs`
- **key_exists()** (4 connections) — `src/scanner/registry_scanner.rs`
- **read_dword_from_key()** (4 connections) — `src/scanner/registry_scanner.rs`
- **reg_type_to_u32()** (4 connections) — `src/scanner/registry_scanner.rs`
- **resolve_target_paths()** (4 connections) — `src/scanner/registry_scanner.rs`
- **test_strict_reg_type_dispatch_and_negative_numeric_sz()** (4 connections) — `src/scanner/registry_scanner.rs`
- **.reg_get_raw()** (3 connections) — `src/core/backend.rs`
- **.reg_get_string()** (3 connections) — `src/core/backend.rs`
- **.reg_snapshot_tree()** (3 connections) — `src/core/backend.rs`
- **has_active_ip()** (3 connections) — `src/scanner/registry_scanner.rs`
- **replace_placeholder_ci()** (3 connections) — `src/scanner/registry_scanner.rs`
- **ScratchCleanup** (3 connections) — `src/scanner/registry_scanner.rs`
- *... and 12 more nodes in this community*

## Relationships

- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (10 shared connections)
- [RealBackend](RealBackend.md) (7 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (4 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (3 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [super](super.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 100 (91%)
- INFERRED: 10 (9%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*