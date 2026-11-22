# Integration Suite: Tweak Sandbox E2E

> 62 nodes · cohesion 0.04

## Key Concepts

- **SlowScanBackend** (85 connections) — `tests/tweak_sandbox_e2e.rs`
- **PanickingBackend** (81 connections) — `tests/tweak_sandbox_e2e.rs`
- **AtomicBool** (7 connections)
- **.reg_set_raw()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_string()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **test_epoch_and_stale_scan_fencing_preserves_instant_mutation()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_raw()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_tree()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_binary()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_tree()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.broadcast_shell_settings_changed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.check_onedrive_kfm_active()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.default_profile_path()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.get_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_elevated()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_hags_enabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_mpo_disabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.logical_core_count()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- *... and 37 more nodes in this community*

## Relationships

- [automation.rs](automation.rs.md) (44 shared connections)
- [HibernationStatus](HibernationStatus.md) (27 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (20 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (9 shared connections)
- [driver_update.rs](driver_update.rs.md) (4 shared connections)
- [WinBlastError](WinBlastError.md) (4 shared connections)
- [.all_physical](all_physical.md) (4 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (4 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [DiskSubsystemReport](DiskSubsystemReport.md) (2 shared connections)
- [RegStringValue](RegStringValue.md) (2 shared connections)
- [adaptive_tuning.rs](adaptive_tuning.rs.md) (2 shared connections)

## Source Files

- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 247 (100%)
- INFERRED: 1 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*