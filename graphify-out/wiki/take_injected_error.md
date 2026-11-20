# .take_injected_error

> 35 nodes · cohesion 0.11

## Key Concepts

- **.take_injected_error()** (37 connections) — `src/core/backend.rs`
- **.state_write()** (36 connections) — `src/core/backend.rs`
- **.sync_parking_in_state()** (7 connections) — `src/core/backend.rs`
- **test_sandbox_core_parking_labels_metrics_and_power_sync()** (7 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (6 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (6 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (6 connections) — `src/core/backend.rs`
- **.apply_parking_snapshot()** (5 connections) — `src/core/backend.rs`
- **.execute_clean_targets()** (5 connections) — `src/core/backend.rs`
- **.restore_stock_parking()** (5 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (5 connections) — `src/core/backend.rs`
- **.write_power_setting_index()** (5 connections) — `src/core/backend.rs`
- **.remove_appx_batch()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_family_in_state()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (4 connections) — `src/core/backend.rs`
- **.set_hags()** (4 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_mpo_disabled()** (4 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (4 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (4 connections) — `src/core/backend.rs`
- **optional_feature_matches()** (3 connections) — `src/core/backend.rs`
- **.block_telemetry_hosts()** (3 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (3 connections) — `src/core/backend.rs`
- **.request_system_reboot()** (3 connections) — `src/core/backend.rs`
- **.restart_explorer_safely()** (3 connections) — `src/core/backend.rs`
- *... and 10 more nodes in this community*

## Relationships

- [SandboxEnv](SandboxEnv.md) (35 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (17 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (4 shared connections)
- [parking.rs](parking.rs.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (1 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (1 shared connections)
- [HibernationStatus](HibernationStatus.md) (1 shared connections)
- [StartupEntry](StartupEntry.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 134 (97%)
- INFERRED: 4 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*