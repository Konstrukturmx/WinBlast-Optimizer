# .navigate

> 14 nodes · cohesion 0.14

## Key Concepts

- **.navigate()** (15 connections) — `src/app.rs`
- **.scan_startup_and_tasks_async()** (5 connections) — `src/app.rs`
- **.scan_bios_info_async()** (4 connections) — `src/app.rs`
- **.scan_disk_health_async()** (4 connections) — `src/app.rs`
- **.scan_online_drivers()** (4 connections) — `src/app.rs`
- **.scan_thermal_snapshot_async()** (4 connections) — `src/app.rs`
- **.nav_prev_view()** (3 connections) — `src/app.rs`
- **ThermalSnapshot** (3 connections) — `src/app.rs`
- **.prev()** (2 connections) — `src/app.rs`
- **.nav_next_view()** (2 connections) — `src/app.rs`
- **.scan_game_prereqs()** (2 connections) — `src/app.rs`
- **BiosInfoLoaded** (2 connections) — `src/app.rs`
- **DiskReportLoaded** (2 connections) — `src/app.rs`
- **DriverScanCompleted** (2 connections) — `src/app.rs`

## Relationships

- [AppState](AppState.md) (10 shared connections)
- [WorkerEvent](WorkerEvent.md) (7 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (6 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (2 shared connections)
- [.setup](setup.md) (2 shared connections)
- [.poll](poll.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 41 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*