# WorkerEvent

> 40 nodes · cohesion 0.07

## Key Concepts

- **WorkerEvent** (81 connections) — `src/app.rs`
- **.apply_worker_event()** (31 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **AppxBatchRemoved** (2 connections) — `src/app.rs`
- **CleanExecuted** (2 connections) — `src/app.rs`
- **CleanTargetsScanned** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **MpoUpdated** (2 connections) — `src/app.rs`
- *... and 15 more nodes in this community*

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (21 shared connections)
- [AppState](AppState.md) (17 shared connections)
- [.setup](setup.md) (9 shared connections)
- [.navigate](navigate.md) (7 shared connections)
- [.set_status_spec](set_status_spec.md) (6 shared connections)
- [UpdateStatus](UpdateStatus.md) (4 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (3 shared connections)
- [.run_scan_internal](run_scan_internal.md) (2 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [elevation.rs](elevation.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 148 (96%)
- INFERRED: 6 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*