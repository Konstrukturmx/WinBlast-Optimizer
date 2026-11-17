# Disk Cleaning & Temporary File Purger

> 54 nodes · cohesion 0.04

## Key Concepts

- **SystemBackend** (117 connections) — `src/core/backend.rs`
- **CachedScanBackend<'a>** (6 connections) — `src/core/backend.rs`
- **.inner()** (2 connections) — `src/core/backend.rs`
- **.new()** (2 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_get_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_snapshot_tree()** (2 connections) — `src/core/backend.rs`
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
- *... and 29 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (14 shared connections)
- [engine.rs](engine.rs.md) (9 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (2 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (2 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 127 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*