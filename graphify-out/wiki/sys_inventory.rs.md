# sys_inventory.rs

> 60 nodes · cohesion 0.05

## Key Concepts

- **SlowScanBackend** (85 connections) — `tests/tweak_sandbox_e2e.rs`
- **PanickingBackend** (81 connections) — `tests/tweak_sandbox_e2e.rs`
- **AtomicBool** (7 connections)
- **.reg_set_raw()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_string()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **test_epoch_and_stale_scan_fencing_preserves_instant_mutation()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_raw()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_tree()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_value()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_binary()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_tree()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_delete_value()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_dword()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_set_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.write_power_setting_index()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.broadcast_shell_settings_changed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.check_onedrive_kfm_active()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.get_service_start_type()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.get_task_enabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_hags_enabled()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.logical_core_count()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- *... and 35 more nodes in this community*

## Relationships

- [WinBlast Optimizer](WinBlast_Optimizer.md) (48 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (25 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (18 shared connections)
- [theme.rs](theme.rs.md) (10 shared connections)
- [deserialize](deserialize.md) (4 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (4 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (4 shared connections)
- [HibernationStatus](HibernationStatus.md) (4 shared connections)
- [.poll](poll.md) (4 shared connections)
- [features_apps.rs](features_apps.rs.md) (4 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)

## Source Files

- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 248 (100%)
- INFERRED: 1 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*