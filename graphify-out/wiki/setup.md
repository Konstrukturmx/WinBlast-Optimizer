# .setup

> 14 nodes · cohesion 0.18

## Key Concepts

- **.setup()** (9 connections) — `src/app.rs`
- **.run_telemetry_audit()** (7 connections) — `src/app.rs`
- **.refresh_restore_points()** (6 connections) — `src/app.rs`
- **StatusUpdate** (6 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.refresh_latency()** (5 connections) — `src/app.rs`
- **.refresh_platform_inventory()** (5 connections) — `src/app.rs`
- **.check_for_updates()** (3 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **LatencyUpdated** (2 connections) — `src/app.rs`
- **MetricsSnapshot** (2 connections) — `src/app.rs`
- **RestorePointsLoaded** (2 connections) — `src/app.rs`
- **TelemetryAuditCompleted** (2 connections) — `src/app.rs`
- **UpdateCheckCompleted** (2 connections) — `src/app.rs`

## Relationships

- [Test Composite With()](Test_Composite_With.md) (10 shared connections)
- [AppState](AppState.md) (9 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (6 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 41 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*