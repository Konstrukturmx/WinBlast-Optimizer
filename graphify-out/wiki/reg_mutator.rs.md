# reg_mutator.rs

> 24 nodes · cohesion 0.18

## Key Concepts

- **reg_mutator.rs** (25 connections) — `src/executor/reg_mutator.rs`
- **validate_registry_mutation_path()** (17 connections) — `src/core/reg_helpers.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **set_raw()** (8 connections) — `src/executor/reg_mutator.rs`
- **create_subkey_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **restore_tree_node()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_dword_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_string_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **set_binary()** (6 connections) — `src/executor/reg_mutator.rs`
- **is_known_numeric_reg_sz()** (5 connections) — `src/core/reg_helpers.rs`
- **restore_tree()** (5 connections) — `src/executor/reg_mutator.rs`
- **write_dword_or_numeric_sz()** (5 connections) — `src/executor/reg_mutator.rs`
- **delete_tree()** (4 connections) — `src/executor/reg_mutator.rs`
- **u32_to_reg_type()** (4 connections) — `src/executor/reg_mutator.rs`
- **RegKey** (3 connections)
- **set_dword()** (3 connections) — `src/executor/reg_mutator.rs`
- **test_known_numeric_reg_sz()** (2 connections) — `src/core/reg_helpers.rs`
- **delete_value()** (2 connections) — `src/executor/reg_mutator.rs`
- **set_string()** (2 connections) — `src/executor/reg_mutator.rs`
- **test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()** (2 connections) — `tests/real_windows_env.rs`
- **resolve_target_paths** (1 connections)
- **REG_DELETE_RIGHT** (1 connections) — `src/executor/reg_mutator.rs`
- **RegType** (1 connections)

## Relationships

- [registry_scanner.rs](registry_scanner.rs.md) (15 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (6 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (5 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (5 shared connections)
- [.setup](setup.md) (3 shared connections)
- [.take_injected_error](take_injected_error.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (2 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (2 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (2 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`
- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 66 (68%)
- INFERRED: 31 (32%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*