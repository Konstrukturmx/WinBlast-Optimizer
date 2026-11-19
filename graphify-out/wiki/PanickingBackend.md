# PanickingBackend

> 55 nodes · cohesion 0.05

## Key Concepts

- **SlowScanBackend** (85 connections) — `tests/tweak_sandbox_e2e.rs`
- **PanickingBackend** (81 connections) — `tests/tweak_sandbox_e2e.rs`
- **AtomicBool** (7 connections)
- **.reg_delete_tree()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.remove_appx_package()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_tree()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.remove_appx_package()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.broadcast_shell_settings_changed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.check_onedrive_kfm_active()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.default_profile_path()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.get_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.get_task_enabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_elevated()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_hags_enabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_mpo_disabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_dword()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_key_exists()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_hags()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_mpo_disabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- *... and 30 more nodes in this community*

## Relationships

- [BlastResult](BlastResult.md) (44 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (22 shared connections)
- [test_wave1_ledger_rollback_and_concurrency_regressions](test_wave1_ledger_rollback_and_concurrency_regressions.md) (20 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (12 shared connections)
- [theme.rs](theme.rs.md) (6 shared connections)
- [LazyLoader](LazyLoader.md) (4 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (4 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (4 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [.detect_os](detect_os.md) (2 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (2 shared connections)

## Source Files

- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 231 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*