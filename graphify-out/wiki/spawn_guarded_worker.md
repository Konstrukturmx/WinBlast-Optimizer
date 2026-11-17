# spawn_guarded_worker

> 21 nodes · cohesion 0.10

## Key Concepts

- **spawn_guarded_worker()** (44 connections) — `src/app.rs`
- **.rollback_entry()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **Waker** (4 connections)
- **Sender** (3 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- **.set_active_power_plan()** (2 connections) — `src/app.rs`
- **.toggle_gpu_msi()** (2 connections) — `src/app.rs`
- **.toggle_hags()** (2 connections) — `src/app.rs`
- **.toggle_hibernation()** (2 connections) — `src/app.rs`
- **.toggle_mpo()** (2 connections) — `src/app.rs`
- **.toggle_usb_power_saving()** (2 connections) — `src/app.rs`
- **ExplorerRestarted** (2 connections) — `src/app.rs`
- **RollbackCompleted** (2 connections) — `src/app.rs`
- **TelemetryHostsBlocked** (2 connections) — `src/app.rs`
- **Any** (1 connections)
- **F** (1 connections)

## Relationships

- [AppState](AppState.md) (22 shared connections)
- [.sync_clean_apps_busy](sync_clean_apps_busy.md) (8 shared connections)
- [Test Composite With()](Test_Composite_With.md) (6 shared connections)
- [.setup](setup.md) (6 shared connections)
- [.sync_startup_tasks_busy](sync_startup_tasks_busy.md) (4 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [engine.rs](engine.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 72 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*