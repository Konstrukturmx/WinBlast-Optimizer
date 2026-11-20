# reg_mutator.rs

> 34 nodes · cohesion 0.11

## Key Concepts

- **.take_injected_error()** (37 connections) — `src/core/backend.rs`
- **.state_write()** (36 connections) — `src/core/backend.rs`
- **.sync_parking_in_state()** (7 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (6 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (6 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (6 connections) — `src/core/backend.rs`
- **.apply_parking_snapshot()** (5 connections) — `src/core/backend.rs`
- **.execute_clean_targets()** (5 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (5 connections) — `src/core/backend.rs`
- **.write_power_setting_index()** (5 connections) — `src/core/backend.rs`
- **.remove_appx_batch()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_family_in_state()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (4 connections) — `src/core/backend.rs`
- **.set_hags()** (4 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_mpo_disabled()** (4 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (4 connections) — `src/core/backend.rs`
- **optional_feature_matches()** (3 connections) — `src/core/backend.rs`
- **.block_telemetry_hosts()** (3 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (3 connections) — `src/core/backend.rs`
- **.request_system_reboot()** (3 connections) — `src/core/backend.rs`
- **.restart_explorer_safely()** (3 connections) — `src/core/backend.rs`
- **.set_active_power_scheme()** (3 connections) — `src/core/backend.rs`
- *... and 9 more nodes in this community*

## Relationships

- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (32 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (17 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (5 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (3 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [updater.rs](updater.rs.md) (2 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (2 shared connections)
- [.set_status_spec](set_status_spec.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (2 shared connections)
- [enums](enums.md) (1 shared connections)
- [DriverUpdateState](DriverUpdateState.md) (1 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 128 (96%)
- INFERRED: 5 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*