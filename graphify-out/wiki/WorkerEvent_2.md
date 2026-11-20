# WorkerEvent

> 46 nodes · cohesion 0.07

## Key Concepts

- **WorkerEvent** (81 connections) — `src/app.rs`
- **.apply_worker_event()** (31 connections) — `src/app.rs`
- **.setup()** (11 connections) — `src/app.rs`
- **.run_telemetry_audit()** (7 connections) — `src/app.rs`
- **.refresh_restore_points()** (6 connections) — `src/app.rs`
- **StatusUpdate** (6 connections) — `src/app.rs`
- **.download_and_install_update()** (5 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.refresh_latency()** (5 connections) — `src/app.rs`
- **.refresh_platform_inventory()** (5 connections) — `src/app.rs`
- **.check_for_updates()** (4 connections) — `src/app.rs`
- **.scan_thermal_snapshot_async()** (4 connections) — `src/app.rs`
- **Failed** (4 connections) — `src/app.rs`
- **apply_and_restart()** (4 connections) — `src/core/updater.rs`
- **.apply_and_restart()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **ThermalSnapshot** (3 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **LatencyUpdated** (2 connections) — `src/app.rs`
- *... and 21 more nodes in this community*

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (32 shared connections)
- [AppState](AppState.md) (26 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (5 shared connections)
- [app.rs](app.rs.md) (4 shared connections)
- [.poll](poll.md) (2 shared connections)
- [UpdateStatus](UpdateStatus.md) (2 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [parse_guid](parse_guid.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 159 (96%)
- INFERRED: 7 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*