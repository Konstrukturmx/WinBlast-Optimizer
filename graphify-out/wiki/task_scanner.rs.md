# task_scanner.rs

> 23 nodes · cohesion 0.22

## Key Concepts

- **SandboxState** (34 connections) — `src/core/backend.rs`
- **.reg_key()** (14 connections) — `src/core/backend.rs`
- **.seed_actions_with_vendor()** (14 connections) — `src/core/backend.rs`
- **.normalize_hive()** (12 connections) — `src/core/backend.rs`
- **.ensure_tree_path_in_state()** (10 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (9 connections) — `src/core/backend.rs`
- **.reg_set_string()** (9 connections) — `src/core/backend.rs`
- **.resolve_target_paths_in_state()** (9 connections) — `src/core/backend.rs`
- **.clear_reg_value_in_state()** (8 connections) — `src/core/backend.rs`
- **.normalize_reg_path()** (8 connections) — `src/core/backend.rs`
- **.reg_set_binary()** (8 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (8 connections) — `src/core/backend.rs`
- **.restore_tree_into_state()** (8 connections) — `src/core/backend.rs`
- **.write_dword_in_state()** (8 connections) — `src/core/backend.rs`
- **.write_raw_in_state()** (8 connections) — `src/core/backend.rs`
- **.reg_delete_tree()** (6 connections) — `src/core/backend.rs`
- **.reg_get_string()** (5 connections) — `src/core/backend.rs`
- **.reg_snapshot_tree()** (5 connections) — `src/core/backend.rs`
- **.reg_key_exists()** (4 connections) — `src/core/backend.rs`
- **.reg_delete_value()** (3 connections) — `src/core/backend.rs`
- **.reg_set_dword()** (3 connections) — `src/core/backend.rs`
- **.resolve_net_interface_paths_in_state()** (3 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults()** (3 connections) — `src/core/backend.rs`

## Relationships

- [SandboxEnv](SandboxEnv.md) (33 shared connections)
- [.take_injected_error](take_injected_error.md) (17 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (5 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (2 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (2 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (2 shared connections)
- [Error.Rs Winblasterror Appx](Error.Rs_Winblasterror_Appx.md) (1 shared connections)
- [WindowsInfo](WindowsInfo.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 135 (94%)
- INFERRED: 8 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*