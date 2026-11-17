# Test Composite With()

> 25 nodes · cohesion 0.10

## Key Concepts

- **WorkerEvent** (73 connections) — `src/app.rs`
- **.apply_worker_event()** (30 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **MpoUpdated** (2 connections) — `src/app.rs`
- **ScanCompleted** (2 connections) — `src/app.rs`
- **ScanFailed** (2 connections) — `src/app.rs`
- **StartupScanFailed** (2 connections) — `src/app.rs`
- **AppxAndFeaturesLoaded** (1 connections) — `src/app.rs`
- **AppxScanFailed** (1 connections) — `src/app.rs`
- **CleanScanFailed** (1 connections) — `src/app.rs`
- **CoreParkingActionCompleted** (1 connections) — `src/app.rs`
- **FeatureToggled** (1 connections) — `src/app.rs`
- **GpuMsiUpdated** (1 connections) — `src/app.rs`
- **PlatformInventoryLoaded** (1 connections) — `src/app.rs`
- **PowerActionCompleted** (1 connections) — `src/app.rs`
- **SingleTweakFailed** (1 connections) — `src/app.rs`
- **SingleTweakUpdated** (1 connections) — `src/app.rs`
- **StartupAndTasksLoaded** (1 connections) — `src/app.rs`

## Relationships

- [AppState](AppState.md) (19 shared connections)
- [.setup](setup.md) (10 shared connections)
- [.sync_clean_apps_busy](sync_clean_apps_busy.md) (8 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (6 shared connections)
- [.sync_startup_tasks_busy](sync_startup_tasks_busy.md) (4 shared connections)
- [UpdateStatus](UpdateStatus.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (2 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (1 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 107 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*