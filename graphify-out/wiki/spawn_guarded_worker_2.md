# spawn_guarded_worker

> 34 nodes · cohesion 0.11

## Key Concepts

- **.take_injected_error()** (37 connections) — `src/core/backend.rs`
- **.state_write()** (36 connections) — `src/core/backend.rs`
- **.sync_parking_in_state()** (8 connections) — `src/core/backend.rs`
- **.write_power_setting_index()** (7 connections) — `src/core/backend.rs`
- **test_sandbox_core_parking_labels_metrics_and_power_sync()** (7 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (6 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (6 connections) — `src/core/backend.rs`
- **.apply_parking_snapshot()** (5 connections) — `src/core/backend.rs`
- **.execute_clean_targets()** (5 connections) — `src/core/backend.rs`
- **.remove_appx_batch()** (5 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (5 connections) — `src/core/backend.rs`
- **.restore_stock_parking()** (5 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (5 connections) — `src/core/backend.rs`
- **.block_telemetry_hosts()** (4 connections) — `src/core/backend.rs`
- **.set_hags()** (4 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_mpo_disabled()** (4 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_service_start_type()** (4 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (4 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (3 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (3 connections) — `src/core/backend.rs`
- **.request_system_reboot()** (3 connections) — `src/core/backend.rs`
- **.restart_explorer_safely()** (3 connections) — `src/core/backend.rs`
- *... and 9 more nodes in this community*

## Relationships

- [WinBlastError](WinBlastError.md) (31 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (16 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (6 shared connections)
- [AutoTuneProfile](AutoTuneProfile.md) (4 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (4 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [power.rs](power.rs.md) (2 shared connections)
- [Changelog](Changelog.md) (2 shared connections)
- [AppxSafetyTier](AppxSafetyTier.md) (2 shared connections)
- [appx_scanner.rs](appx_scanner.rs.md) (2 shared connections)
- [restore_point.rs](restore_point.rs.md) (2 shared connections)
- [adaptive_tuning.rs](adaptive_tuning.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 138 (97%)
- INFERRED: 5 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*