# PlatformSecurityInfo

> 15 nodes · cohesion 0.16

## Key Concepts

- **.setup()** (11 connections) — `src/app.rs`
- **.run_telemetry_audit()** (7 connections) — `src/app.rs`
- **.refresh_restore_points()** (6 connections) — `src/app.rs`
- **StatusUpdate** (6 connections) — `src/app.rs`
- **.refresh_core_parking()** (5 connections) — `src/app.rs`
- **.refresh_latency()** (5 connections) — `src/app.rs`
- **.refresh_platform_inventory()** (5 connections) — `src/app.rs`
- **.check_for_updates()** (3 connections) — `src/app.rs`
- **.setup_slint()** (2 connections) — `src/app.rs`
- **CoreParkingUpdated** (2 connections) — `src/app.rs`
- **LatencyUpdated** (2 connections) — `src/app.rs`
- **MetricsSnapshot** (2 connections) — `src/app.rs`
- **RestorePointsLoaded** (2 connections) — `src/app.rs`
- **TelemetryAuditCompleted** (2 connections) — `src/app.rs`
- **UpdateCheckCompleted** (2 connections) — `src/app.rs`

## Relationships

- [startup.rs](startup.rs.md) (10 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (8 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (6 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (2 shared connections)
- [parse_guid](parse_guid.md) (1 shared connections)
- [.run_scan_internal](run_scan_internal.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 44 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*