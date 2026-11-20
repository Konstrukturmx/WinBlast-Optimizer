# spawn_guarded_worker

> 32 nodes · cohesion 0.07

## Key Concepts

- **spawn_guarded_worker()** (48 connections) — `src/app.rs`
- **.sync_startup_tasks_busy()** (7 connections) — `src/app.rs`
- **Sender** (5 connections)
- **.set_live_service_start_async()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **.create_restore_point()** (4 connections) — `src/app.rs`
- **.rollback_entry()** (4 connections) — `src/app.rs`
- **.run_live_task_async()** (4 connections) — `src/app.rs`
- **.start_latency_benchmark()** (4 connections) — `src/app.rs`
- **.toggle_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.toggle_startup_entry_async()** (4 connections) — `src/app.rs`
- **Waker** (4 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **ScheduledTaskRunResult** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- **.set_active_power_plan()** (2 connections) — `src/app.rs`
- **.toggle_gpu_msi()** (2 connections) — `src/app.rs`
- **.toggle_hags()** (2 connections) — `src/app.rs`
- **.toggle_hibernation()** (2 connections) — `src/app.rs`
- **.toggle_mpo()** (2 connections) — `src/app.rs`
- **.toggle_usb_power_saving()** (2 connections) — `src/app.rs`
- **ExplorerRestarted** (2 connections) — `src/app.rs`
- *... and 7 more nodes in this community*

## Relationships

- [AppState](AppState.md) (21 shared connections)
- [WorkerEvent](WorkerEvent.md) (21 shared connections)
- [.set_status_spec](set_status_spec.md) (7 shared connections)
- [.navigate](navigate.md) (6 shared connections)
- [.setup](setup.md) (5 shared connections)
- [LazyLoader](LazyLoader.md) (3 shared connections)
- [.run_scan_internal](run_scan_internal.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 105 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*