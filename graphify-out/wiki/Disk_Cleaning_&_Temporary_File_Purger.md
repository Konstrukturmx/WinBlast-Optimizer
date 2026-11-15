# Disk Cleaning & Temporary File Purger

> 52 nodes · cohesion 0.04

## Key Concepts

- **SystemBackend** (110 connections) — `src/core/backend.rs`
- **CachedScanBackend<'a>** (6 connections) — `src/core/backend.rs`
- **.inner()** (2 connections) — `src/core/backend.rs`
- **.new()** (2 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_get_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
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
- *... and 27 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (10 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (7 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (7 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (6 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (4 shared connections)
- [Themed Interactive Scrollbar Widget](Themed_Interactive_Scrollbar_Widget.md) (2 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (2 shared connections)
- [Startup Entries() .List](Startup_Entries_.List.md) (2 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 118 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*