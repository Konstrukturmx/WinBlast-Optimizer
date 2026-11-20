# SlowScanBackend

> 28 nodes · cohesion 0.15

## Key Concepts

- **registry_scanner.rs** (37 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_read_64()** (13 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_flags_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **RegKey** (8 connections)
- **get_raw()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_string_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **resolve_target_paths()** (7 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_key_recursive()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_dword_for_vendor()** (6 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value_with_kind()** (5 connections) — `src/scanner/registry_scanner.rs`
- **resolve_net_interface_paths()** (5 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_tree()** (5 connections) — `src/scanner/registry_scanner.rs`
- **encode_hex()** (4 connections) — `src/core/reg_helpers.rs`
- **evaluate_dword()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value()** (4 connections) — `src/scanner/registry_scanner.rs`
- **get_dword()** (4 connections) — `src/scanner/registry_scanner.rs`
- **read_dword_from_key()** (4 connections) — `src/scanner/registry_scanner.rs`
- **reg_type_to_u32()** (4 connections) — `src/scanner/registry_scanner.rs`
- **has_active_ip()** (3 connections) — `src/scanner/registry_scanner.rs`
- **key_exists()** (3 connections) — `src/scanner/registry_scanner.rs`
- **test_evaluate_dword_value_with_kind_all_branches()** (3 connections) — `src/scanner/registry_scanner.rs`
- **check_dword()** (2 connections) — `src/scanner/registry_scanner.rs`
- **get_string()** (2 connections) — `src/scanner/registry_scanner.rs`
- **replace_placeholder_ci()** (2 connections) — `src/scanner/registry_scanner.rs`
- **DISPLAY_CLASS_LOWER** (1 connections) — `src/scanner/registry_scanner.rs`
- *... and 3 more nodes in this community*

## Relationships

- [DriverError](DriverError.md) (15 shared connections)
- [.new](new.md) (5 shared connections)
- [TaskCache](TaskCache.md) (3 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (2 shared connections)
- [parking.rs](parking.rs.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [RestorePoint](RestorePoint.md) (2 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [CoreParkingState](CoreParkingState.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 84 (84%)
- INFERRED: 16 (16%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*