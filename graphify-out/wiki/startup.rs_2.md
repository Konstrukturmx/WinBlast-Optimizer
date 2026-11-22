# startup.rs

> 31 nodes · cohesion 0.09

## Key Concepts

- **WorkerEvent** (85 connections) — `src/app.rs`
- **.apply_worker_event()** (33 connections) — `src/app.rs`
- **.download_and_install_update()** (4 connections) — `src/app.rs`
- **.refresh_system_health()** (4 connections) — `src/app.rs`
- **Failed** (4 connections) — `src/app.rs`
- **.assess_system_health()** (3 connections) — `src/app.rs`
- **.neutralize_telemetry_services()** (3 connections) — `src/app.rs`
- **TelemetryServicesNeutralized** (3 connections) — `src/app.rs`
- **.apply_and_restart()** (2 connections) — `src/app.rs`
- **.optimization_progress()** (2 connections) — `src/app.rs`
- **.rebuild_scan_index()** (2 connections) — `src/app.rs`
- **.record_current_baseline()** (2 connections) — `src/app.rs`
- **.refresh_ledger()** (2 connections) — `src/app.rs`
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
- *... and 6 more nodes in this community*

## Relationships

- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (12 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (12 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (11 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (7 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (7 shared connections)
- [super](super.md) (6 shared connections)
- [.run_scan_internal](run_scan_internal.md) (2 shared connections)
- [.set_status](set_status.md) (2 shared connections)
- [UpdateStatus](UpdateStatus.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [UiThemeStyle](UiThemeStyle.md) (2 shared connections)
- [power.rs](power.rs.md) (2 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 127 (95%)
- INFERRED: 7 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*