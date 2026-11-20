# spawn_guarded_worker

> 57 nodes · cohesion 0.05

## Key Concepts

- **spawn_guarded_worker()** (48 connections) — `src/app.rs`
- **.navigate()** (15 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.sync_startup_tasks_busy()** (7 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **.scan_online_drivers()** (5 connections) — `src/app.rs`
- **.scan_startup_and_tasks_async()** (5 connections) — `src/app.rs`
- **.set_live_service_start_async()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **.create_restore_point()** (4 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.reset_baseline()** (4 connections) — `src/app.rs`
- **.run_live_task_async()** (4 connections) — `src/app.rs`
- **.scan_bios_info_async()** (4 connections) — `src/app.rs`
- **.scan_disk_health_async()** (4 connections) — `src/app.rs`
- **.toggle_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.toggle_startup_entry_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **Waker** (4 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- *... and 32 more nodes in this community*

## Relationships

- [AppState](AppState.md) (47 shared connections)
- [WorkerEvent](WorkerEvent.md) (32 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [theme.rs](theme.rs.md) (2 shared connections)
- [.poll](poll.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [Language](Language.md) (1 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 164 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*