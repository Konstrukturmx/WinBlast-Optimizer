# DiskSubsystemReport

> 6 nodes · cohesion 0.40

## Key Concepts

- **DiskSubsystemReport** (8 connections) — `src/hardware/disks.rs`
- **.probe()** (4 connections) — `src/hardware/disks.rs`
- **PhysicalDriveHealth** (4 connections) — `src/hardware/disks.rs`
- **probe_volume_fallback()** (3 connections) — `src/hardware/disks.rs`
- **test_disk_subsystem_report_probe_no_panic()** (2 connections) — `src/hardware/disks.rs`
- **.run_trim_all()** (1 connections) — `src/hardware/disks.rs`

## Relationships

- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (4 shared connections)
- [probe_physical_drive](probe_physical_drive.md) (2 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (1 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)

## Source Files

- `src/hardware/disks.rs`

## Audit Trail

- EXTRACTED: 14 (88%)
- INFERRED: 2 (12%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*