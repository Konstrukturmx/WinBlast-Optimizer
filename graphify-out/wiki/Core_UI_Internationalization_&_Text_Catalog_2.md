# Core UI Internationalization & Text Catalog

> 29 nodes · cohesion 0.09

## Key Concepts

- **WorkerEvent** (81 connections) — `src/app.rs`
- **.apply_worker_event()** (30 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.download_and_install_update()** (4 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **MpoUpdated** (2 connections) — `src/app.rs`
- **ScanCompleted** (2 connections) — `src/app.rs`
- **ScanFailed** (2 connections) — `src/app.rs`
- **StartupScanFailed** (2 connections) — `src/app.rs`
- **UpdateDownloadCompleted** (2 connections) — `src/app.rs`
- **AppxAndFeaturesLoaded** (1 connections) — `src/app.rs`
- **AppxScanFailed** (1 connections) — `src/app.rs`
- **CleanScanFailed** (1 connections) — `src/app.rs`
- **CoreParkingActionCompleted** (1 connections) — `src/app.rs`
- **FeatureToggled** (1 connections) — `src/app.rs`
- **GpuMsiUpdated** (1 connections) — `src/app.rs`
- **PlatformInventoryLoaded** (1 connections) — `src/app.rs`
- *... and 4 more nodes in this community*

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (17 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (12 shared connections)
- [Installed Appx .List](Installed_Appx_.List.md) (9 shared connections)
- [.set_status_spec](set_status_spec.md) (6 shared connections)
- [.navigate](navigate.md) (5 shared connections)
- [.sync_startup_tasks_busy](sync_startup_tasks_busy.md) (4 shared connections)
- [ColorScheme](ColorScheme.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (3 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [.poll](poll.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 118 (95%)
- INFERRED: 6 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*