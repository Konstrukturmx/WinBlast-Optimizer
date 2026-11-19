# sys_inventory.rs

> 63 nodes · cohesion 0.06

## Key Concepts

- **sys_inventory.rs** (50 connections) — `src/hardware/sys_inventory.rs`
- **super** (43 connections)
- **disks.rs** (36 connections) — `src/hardware/disks.rs`
- **DiskSubsystemReport** (8 connections) — `src/hardware/disks.rs`
- **probe_physical_drive()** (8 connections) — `src/hardware/disks.rs`
- **StorageVolumeInfo** (8 connections) — `src/hardware/sys_inventory.rs`
- **detect_platform_security_info()** (7 connections) — `src/hardware/sys_inventory.rs`
- **RamDimmInfo** (7 connections) — `src/hardware/sys_inventory.rs`
- **HANDLE** (5 connections)
- **pcwstr** (5 connections)
- **PhysicalDriveHealth** (5 connections) — `src/hardware/disks.rs`
- **query_ram_dimms()** (5 connections) — `src/hardware/sys_inventory.rs`
- **closehandle** (4 connections)
- **.probe()** (4 connections) — `src/hardware/disks.rs`
- **HandleGuard** (4 connections) — `src/hardware/disks.rs`
- **query_nvme_smart_ioctl()** (4 connections) — `src/hardware/disks.rs`
- **HandleGuard** (4 connections) — `src/hardware/sys_inventory.rs`
- **probe_drive_media_kind()** (4 connections) — `src/hardware/sys_inventory.rs`
- **query_storage_volumes()** (4 connections) — `src/hardware/sys_inventory.rs`
- **probe_volume_fallback()** (3 connections) — `src/hardware/disks.rs`
- **query_disk_capacity_ioctl()** (3 connections) — `src/hardware/disks.rs`
- **query_smart_health_wmi()** (3 connections) — `src/hardware/disks.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/sys_inventory.rs`
- **query_live_code_integrity_options()** (3 connections) — `src/hardware/sys_inventory.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- *... and 38 more nodes in this community*

## Relationships

- [deserialize](deserialize.md) (7 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [startup.rs](startup.rs.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (3 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (3 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (3 shared connections)
- [DriverError](DriverError.md) (3 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [elevation.rs](elevation.rs.md) (2 shared connections)

## Source Files

- `src/hardware/disks.rs`
- `src/hardware/sys_inventory.rs`

## Audit Trail

- EXTRACTED: 183 (99%)
- INFERRED: 2 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*