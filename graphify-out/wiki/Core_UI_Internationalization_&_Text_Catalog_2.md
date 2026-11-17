# Core UI Internationalization & Text Catalog

> 31 nodes · cohesion 0.15

## Key Concepts

- **registry_scanner.rs** (37 connections) — `src/scanner/registry_scanner.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_read_64()** (13 connections) — `src/scanner/registry_scanner.rs`
- **RegKey** (8 connections)
- **get_dword_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_string_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **resolve_target_paths()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_raw()** (6 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_key_recursive()** (6 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_tree()** (6 connections) — `src/scanner/registry_scanner.rs`
- **test_strict_reg_type_dispatch_and_negative_numeric_sz()** (6 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value_with_kind()** (5 connections) — `src/scanner/registry_scanner.rs`
- **get_binary()** (5 connections) — `src/scanner/registry_scanner.rs`
- **get_dword()** (5 connections) — `src/scanner/registry_scanner.rs`
- **resolve_net_interface_paths()** (5 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value()** (4 connections) — `src/scanner/registry_scanner.rs`
- **key_exists()** (4 connections) — `src/scanner/registry_scanner.rs`
- **reg_type_to_u32()** (4 connections) — `src/scanner/registry_scanner.rs`
- **has_active_ip()** (3 connections) — `src/scanner/registry_scanner.rs`
- **read_dword_from_key()** (3 connections) — `src/scanner/registry_scanner.rs`
- **replace_placeholder_ci()** (3 connections) — `src/scanner/registry_scanner.rs`
- **ScratchCleanup** (3 connections) — `src/scanner/registry_scanner.rs`
- **test_evaluate_dword_value_with_kind_all_branches()** (3 connections) — `src/scanner/registry_scanner.rs`
- **check_dword()** (2 connections) — `src/scanner/registry_scanner.rs`
- *... and 6 more nodes in this community*

## Relationships

- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (12 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (5 shared connections)
- [RealBackend](RealBackend.md) (5 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [RegStringValue](RegStringValue.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [delete_value_for_vendor](delete_value_for_vendor.md) (2 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)

## Source Files

- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 90 (82%)
- INFERRED: 20 (18%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*