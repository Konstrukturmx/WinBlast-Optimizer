# Embedded Catalog Test

> 29 nodes · cohesion 0.09

## Key Concepts

- **WorkerEvent** (81 connections) — `src/app.rs`
- **.apply_worker_event()** (31 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
- **CleanTargetsScanned** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **HagsUpdated** (2 connections) — `src/app.rs`
- **LatencyBenchmarkFailed** (2 connections) — `src/app.rs`
- **MpoUpdated** (2 connections) — `src/app.rs`
- **ScanCompleted** (2 connections) — `src/app.rs`
- **ScanFailed** (2 connections) — `src/app.rs`
- **StartupScanFailed** (2 connections) — `src/app.rs`
- **AppxAndFeaturesLoaded** (1 connections) — `src/app.rs`
- **AppxScanFailed** (1 connections) — `src/app.rs`
- **CleanScanFailed** (1 connections) — `src/app.rs`
- **CoreParkingActionCompleted** (1 connections) — `src/app.rs`
- **FeatureToggled** (1 connections) — `src/app.rs`
- **GpuMsiUpdated** (1 connections) — `src/app.rs`
- **PlatformInventoryLoaded** (1 connections) — `src/app.rs`
- *... and 4 more nodes in this community*

## Relationships

- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (16 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (13 shared connections)
- [WinBlastScrollable](WinBlastScrollable.md) (9 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (6 shared connections)
- [AppxCache](AppxCache.md) (6 shared connections)
- [Appx Package() Prefixes](Appx_Package_Prefixes.md) (6 shared connections)
- [power_governor.rs](power_governor.rs.md) (4 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [.set_status_spec](set_status_spec.md) (2 shared connections)
- [features_apps.rs](features_apps.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 121 (95%)
- INFERRED: 6 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*