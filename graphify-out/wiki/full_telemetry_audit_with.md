# full_telemetry_audit_with

> 48 nodes · cohesion 0.04

## Key Concepts

- **SystemBackend** (122 connections) — `src/core/backend.rs`
- **.reg_get_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.set_usb_power_saving_enabled()** (2 connections) — `src/core/backend.rs`
- **.block_telemetry_hosts()** (1 connections) — `src/core/backend.rs`
- **.broadcast_shell_settings_changed()** (1 connections) — `src/core/backend.rs`
- **.check_onedrive_kfm_active()** (1 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (1 connections) — `src/core/backend.rs`
- **.create_restore_point()** (1 connections) — `src/core/backend.rs`
- **.default_ledger_path()** (1 connections) — `src/core/backend.rs`
- **.default_profile_path()** (1 connections) — `src/core/backend.rs`
- **.get_active_power_scheme()** (1 connections) — `src/core/backend.rs`
- **.get_service_start_type()** (1 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (1 connections) — `src/core/backend.rs`
- **.is_elevated()** (1 connections) — `src/core/backend.rs`
- **.is_hags_enabled()** (1 connections) — `src/core/backend.rs`
- **.is_mpo_disabled()** (1 connections) — `src/core/backend.rs`
- **.logical_core_count()** (1 connections) — `src/core/backend.rs`
- **.read_hosts_file()** (1 connections) — `src/core/backend.rs`
- **.read_power_setting_index()** (1 connections) — `src/core/backend.rs`
- **.reg_delete_tree()** (1 connections) — `src/core/backend.rs`
- **.reg_delete_value()** (1 connections) — `src/core/backend.rs`
- **.reg_get_binary()** (1 connections) — `src/core/backend.rs`
- **.reg_get_dword()** (1 connections) — `src/core/backend.rs`
- **.reg_key_exists()** (1 connections) — `src/core/backend.rs`
- **.reg_set_binary()** (1 connections) — `src/core/backend.rs`
- *... and 23 more nodes in this community*

## Relationships

- [parse_guid](parse_guid.md) (8 shared connections)
- [RealBackend](RealBackend.md) (8 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (6 shared connections)
- [.set_status_spec](set_status_spec.md) (5 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (3 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (2 shared connections)
- [TaskCache](TaskCache.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 123 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*