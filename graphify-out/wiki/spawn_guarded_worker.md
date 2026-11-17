# spawn_guarded_worker

> 41 nodes · cohesion 0.07

## Key Concepts

- **spawn_guarded_worker()** (44 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.sync_startup_tasks_busy()** (7 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **.set_live_service_start_async()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.run_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.toggle_startup_entry_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **Waker** (4 connections)
- **Sender** (3 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **ScheduledTaskRunResult** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- *... and 16 more nodes in this community*

## Relationships

- [AppState](AppState.md) (40 shared connections)
- [Test Composite With()](Test_Composite_With.md) (23 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 121 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*