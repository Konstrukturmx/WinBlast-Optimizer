# WindowsOptionalFeature

> 14 nodes · cohesion 0.14

## Key Concepts

- **.navigate()** (16 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.scan_bios_info_async()** (4 connections) — `src/app.rs`
- **.scan_disk_health_async()** (4 connections) — `src/app.rs`
- **.scan_online_drivers()** (4 connections) — `src/app.rs`
- **.scan_thermal_snapshot_async()** (4 connections) — `src/app.rs`
- **ThermalSnapshot** (3 connections) — `src/app.rs`
- **.nav_next_view()** (2 connections) — `src/app.rs`
- **.record_user_interaction()** (2 connections) — `src/app.rs`
- **.scan_game_prereqs()** (2 connections) — `src/app.rs`
- **BiosInfoLoaded** (2 connections) — `src/app.rs`
- **CleanTargetsScanned** (2 connections) — `src/app.rs`
- **DiskReportLoaded** (2 connections) — `src/app.rs`
- **DriverScanCompleted** (2 connections) — `src/app.rs`

## Relationships

- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (10 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (7 shared connections)
- [startup.rs](startup.rs.md) (7 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (2 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (2 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 42 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*