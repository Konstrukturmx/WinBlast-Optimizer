# spawn_guarded_worker

> 37 nodes · cohesion 0.07

## Key Concepts

- **spawn_guarded_worker()** (48 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.rollback_entry()** (4 connections) — `src/app.rs`
- **.start_latency_benchmark()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **Waker** (4 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **Sender** (3 connections)
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- **.set_active_power_plan()** (2 connections) — `src/app.rs`
- **.toggle_gpu_msi()** (2 connections) — `src/app.rs`
- **.toggle_hags()** (2 connections) — `src/app.rs`
- **.toggle_hibernation()** (2 connections) — `src/app.rs`
- *... and 12 more nodes in this community*

## Relationships

- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (25 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (16 shared connections)
- [.set_status_spec](set_status_spec.md) (7 shared connections)
- [.navigate](navigate.md) (7 shared connections)
- [Installed Appx .List](Installed_Appx_.List.md) (5 shared connections)
- [.sync_startup_tasks_busy](sync_startup_tasks_busy.md) (4 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 114 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*