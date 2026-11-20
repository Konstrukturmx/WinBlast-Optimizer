# Integration Suite: Tweak Sandbox E2E

> 56 nodes · cohesion 0.04

## Key Concepts

- **RealBackend** (84 connections) — `src/core/backend.rs`
- **resolve_persistent_app_dir()** (4 connections) — `src/core/backend.rs`
- **.is_elevated()** (3 connections) — `src/core/backend.rs`
- **.list_optional_features()** (3 connections) — `src/core/backend.rs`
- **.default_ledger_path()** (2 connections) — `src/core/backend.rs`
- **.default_profile_path()** (2 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (2 connections) — `src/core/backend.rs`
- **.reg_snapshot_tree()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
- **.drop()** (2 connections) — `src/core/backend.rs`
- **.block_telemetry_hosts()** (1 connections) — `src/core/backend.rs`
- **.broadcast_shell_settings_changed()** (1 connections) — `src/core/backend.rs`
- **.check_onedrive_kfm_active()** (1 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (1 connections) — `src/core/backend.rs`
- **.create_restore_point()** (1 connections) — `src/core/backend.rs`
- **.get_active_power_scheme()** (1 connections) — `src/core/backend.rs`
- **.get_service_start_type()** (1 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (1 connections) — `src/core/backend.rs`
- **.is_hags_enabled()** (1 connections) — `src/core/backend.rs`
- **.is_mpo_disabled()** (1 connections) — `src/core/backend.rs`
- **.logical_core_count()** (1 connections) — `src/core/backend.rs`
- **.read_hosts_file()** (1 connections) — `src/core/backend.rs`
- **.read_power_setting_index()** (1 connections) — `src/core/backend.rs`
- **.reg_delete_tree()** (1 connections) — `src/core/backend.rs`
- *... and 31 more nodes in this community*

## Relationships

- [deserialize](deserialize.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (2 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (2 shared connections)
- [features_apps.rs](features_apps.rs.md) (2 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (2 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [CoreParkingState](CoreParkingState.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 95 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*