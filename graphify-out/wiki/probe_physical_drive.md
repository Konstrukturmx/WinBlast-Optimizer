# probe_physical_drive

> 16 nodes · cohesion 0.19

## Key Concepts

- **probe_physical_drive()** (10 connections) — `src/hardware/disks.rs`
- **HANDLE** (7 connections)
- **HandleGuard** (7 connections) — `src/hardware/disks.rs`
- **DriveSmartData** (5 connections) — `src/hardware/disks.rs`
- **query_ata_smart_ioctl()** (5 connections) — `src/hardware/disks.rs`
- **query_disk_performance_ioctl()** (5 connections) — `src/hardware/disks.rs`
- **query_nvme_smart_ioctl()** (5 connections) — `src/hardware/disks.rs`
- **parse_ata_smart_attributes()** (4 connections) — `src/hardware/disks.rs`
- **parse_nvme_smart_log()** (4 connections) — `src/hardware/disks.rs`
- **query_disk_capacity_ioctl()** (3 connections) — `src/hardware/disks.rs`
- **test_kernel_disk_performance_live_probe()** (3 connections) — `src/hardware/disks.rs`
- **DiskPerformanceSnapshot** (2 connections) — `src/hardware/disks.rs`
- **extract_null_terminated_str()** (2 connections) — `src/hardware/disks.rs`
- **test_parse_ata_smart_attributes_synthetic()** (2 connections) — `src/hardware/disks.rs`
- **test_parse_nvme_smart_log_synthetic()** (2 connections) — `src/hardware/disks.rs`
- **.drop()** (1 connections) — `src/hardware/disks.rs`

## Relationships

- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (16 shared connections)
- [DiskSubsystemReport](DiskSubsystemReport.md) (2 shared connections)
- [restart_explorer_safely](restart_explorer_safely.md) (1 shared connections)
- [sysprep_engine.rs](sysprep_engine.rs.md) (1 shared connections)
- [storage.rs](storage.rs.md) (1 shared connections)

## Source Files

- `src/hardware/disks.rs`

## Audit Trail

- EXTRACTED: 42 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*