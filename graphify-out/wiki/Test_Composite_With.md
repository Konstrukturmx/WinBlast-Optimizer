# Test Composite With()

> 45 nodes · cohesion 0.07

## Key Concepts

- **WorkerEvent** (73 connections) — `src/app.rs`
- **.apply_worker_event()** (31 connections) — `src/app.rs`
- **.setup()** (9 connections) — `src/app.rs`
- **.run_telemetry_audit()** (7 connections) — `src/app.rs`
- **.refresh_restore_points()** (6 connections) — `src/app.rs`
- **StatusUpdate** (6 connections) — `src/app.rs`
- **.download_and_install_update()** (5 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.refresh_latency()** (5 connections) — `src/app.rs`
- **.refresh_platform_inventory()** (5 connections) — `src/app.rs`
- **.check_for_updates()** (4 connections) — `src/app.rs`
- **Failed** (4 connections) — `src/app.rs`
- **.apply_and_restart()** (3 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **apply_and_restart()** (3 connections) — `src/core/updater.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **LatencyUpdated** (2 connections) — `src/app.rs`
- **MetricsSnapshot** (2 connections) — `src/app.rs`
- *... and 20 more nodes in this community*

## Relationships

- [AppState](AppState.md) (31 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (23 shared connections)
- [updater.rs](updater.rs.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [app.rs](app.rs.md) (4 shared connections)
- [UpdateStatus](UpdateStatus.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 150 (98%)
- INFERRED: 3 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*