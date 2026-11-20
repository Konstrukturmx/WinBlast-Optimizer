# GPU Silicon & Hardware Tuning Policies

> 35 nodes · cohesion 0.07

## Key Concepts

- **spawn_guarded_worker()** (48 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
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
- **Sender** (3 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- **.set_active_power_plan()** (2 connections) — `src/app.rs`
- **.toggle_gpu_msi()** (2 connections) — `src/app.rs`
- **.toggle_hags()** (2 connections) — `src/app.rs`
- **.toggle_hibernation()** (2 connections) — `src/app.rs`
- **.toggle_mpo()** (2 connections) — `src/app.rs`
- *... and 10 more nodes in this community*

## Relationships

- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (24 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (16 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (7 shared connections)
- [AppxCache](AppxCache.md) (6 shared connections)
- [WinBlastScrollable](WinBlastScrollable.md) (5 shared connections)
- [power_governor.rs](power_governor.rs.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 109 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*