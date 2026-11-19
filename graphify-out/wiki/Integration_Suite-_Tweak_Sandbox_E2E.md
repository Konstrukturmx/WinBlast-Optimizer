# Integration Suite: Tweak Sandbox E2E

> 51 nodes · cohesion 0.04

## Key Concepts

- **SystemBackend** (118 connections) — `src/core/backend.rs`
- **.apply_parking_snapshot()** (4 connections) — `src/core/backend.rs`
- **.reg_delete_value_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_get_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
- **.reg_set_dword_for_vendor()** (2 connections) — `src/core/backend.rs`
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
- *... and 26 more nodes in this community*

## Relationships

- [full_telemetry_audit_with](full_telemetry_audit_with.md) (10 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (7 shared connections)
- [app.rs](app.rs.md) (4 shared connections)
- [SandboxEnv](SandboxEnv.md) (4 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (3 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [power.rs](power.rs.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 123 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*