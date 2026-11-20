# .setup

> 12 nodes · cohesion 0.21

## Key Concepts

- **.setup()** (10 connections) — `src/app.rs`
- **.run_telemetry_audit()** (7 connections) — `src/app.rs`
- **.refresh_restore_points()** (6 connections) — `src/app.rs`
- **StatusUpdate** (6 connections) — `src/app.rs`
- **.refresh_latency()** (5 connections) — `src/app.rs`
- **.refresh_platform_inventory()** (5 connections) — `src/app.rs`
- **.check_for_updates()** (4 connections) — `src/app.rs`
- **LatencyUpdated** (2 connections) — `src/app.rs`
- **MetricsSnapshot** (2 connections) — `src/app.rs`
- **RestorePointsLoaded** (2 connections) — `src/app.rs`
- **TelemetryAuditCompleted** (2 connections) — `src/app.rs`
- **UpdateCheckCompleted** (2 connections) — `src/app.rs`

## Relationships

- [WorkerEvent](WorkerEvent.md) (9 shared connections)
- [AppState](AppState.md) (6 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (5 shared connections)
- [.navigate](navigate.md) (2 shared connections)
- [updater.rs](updater.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [.run_scan_internal](run_scan_internal.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 38 (97%)
- INFERRED: 1 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*