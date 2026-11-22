# .setup

> 18 nodes · cohesion 0.16

## Key Concepts

- **thermal.rs** (17 connections) — `src/hardware/thermal.rs`
- **SystemThermalSnapshot** (9 connections) — `src/hardware/thermal.rs`
- **ThermalHealth** (9 connections) — `src/hardware/thermal.rs`
- **ordering** (7 connections)
- **.read()** (4 connections) — `src/hardware/thermal.rs`
- **AtomicU8** (3 connections)
- **ComponentThermal** (3 connections) — `src/hardware/thermal.rs`
- **NVIDIA_SMI_STATUS** (2 connections) — `src/hardware/thermal.rs`
- **probe_acpi_wmi_and_perf()** (2 connections) — `src/hardware/thermal.rs`
- **probe_gpu_thermals()** (2 connections) — `src/hardware/thermal.rs`
- **test_thermal_snapshot_read_no_panic()** (2 connections) — `src/hardware/thermal.rs`
- **WMI_ACPI_STATUS** (2 connections) — `src/hardware/thermal.rs`
- **test_thermal_health_labels()** (1 connections) — `src/hardware/thermal.rs`
- **Cool** (1 connections) — `src/hardware/thermal.rs`
- **Hot** (1 connections) — `src/hardware/thermal.rs`
- **.label()** (1 connections) — `src/hardware/thermal.rs`
- **Optimal** (1 connections) — `src/hardware/thermal.rs`
- **Warm** (1 connections) — `src/hardware/thermal.rs`

## Relationships

- [startup.rs](startup.rs.md) (3 shared connections)
- [driver_update.rs](driver_update.rs.md) (3 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [probe_physical_drive](probe_physical_drive.md) (2 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [.maybe_arm_error](maybe_arm_error.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (1 shared connections)
- [AppxCache](AppxCache.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/hardware/thermal.rs`

## Audit Trail

- EXTRACTED: 42 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*