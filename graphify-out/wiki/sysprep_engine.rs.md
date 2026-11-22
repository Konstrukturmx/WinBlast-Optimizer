# sysprep_engine.rs

> 6 nodes · cohesion 0.40

## Key Concepts

- **DiskSubsystemReport** (10 connections) — `src/hardware/disks.rs`
- **.probe()** (4 connections) — `src/hardware/disks.rs`
- **PhysicalDriveHealth** (4 connections) — `src/hardware/disks.rs`
- **probe_volume_fallback()** (3 connections) — `src/hardware/disks.rs`
- **test_disk_subsystem_report_probe_no_panic()** (2 connections) — `src/hardware/disks.rs`
- **.run_trim_all()** (1 connections) — `src/hardware/disks.rs`

## Relationships

- [topological_scheduler.rs](topological_scheduler.rs.md) (4 shared connections)
- [probe_physical_drive](probe_physical_drive.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (2 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (2 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/hardware/disks.rs`

## Audit Trail

- EXTRACTED: 16 (89%)
- INFERRED: 2 (11%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*