# registry_scanner.rs

> 84 nodes · cohesion 0.05

## Key Concepts

- **WinBlastError** (38 connections) — `src/core/error.rs`
- **registry_scanner.rs** (37 connections) — `src/scanner/registry_scanner.rs`
- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **normalize_reg_value_name()** (21 connections) — `src/core/risk.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **Result** (14 connections) — `src/core/error.rs`
- **open_subkey_read_64()** (13 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (8 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_flags_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **RegKey** (8 connections)
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **get_raw()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_string_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **resolve_target_paths()** (7 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_key_recursive()** (7 connections) — `src/scanner/registry_scanner.rs`
- **test_error_formatting()** (6 connections) — `src/core/error.rs`
- **set_binary()** (6 connections) — `src/executor/reg_mutator.rs`
- **get_dword_for_vendor()** (6 connections) — `src/scanner/registry_scanner.rs`
- **test_strict_reg_type_dispatch_and_negative_numeric_sz()** (6 connections) — `src/scanner/registry_scanner.rs`
- **error.rs** (5 connections) — `src/core/error.rs`
- *... and 59 more nodes in this community*

## Relationships

- [full_telemetry_audit_with](full_telemetry_audit_with.md) (14 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (9 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (5 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [.setup](setup.md) (4 shared connections)
- [power.rs](power.rs.md) (4 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (4 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (4 shared connections)
- [theme.rs](theme.rs.md) (3 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (3 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)

## Source Files

- `src/core/error.rs`
- `src/core/reg_helpers.rs`
- `src/core/risk.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`
- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 213 (81%)
- INFERRED: 50 (19%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*