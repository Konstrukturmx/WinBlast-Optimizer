# State Ledger & Restore Point Engine

> 17 nodes · cohesion 0.17

## Key Concepts

- **thermal.rs** (17 connections) — `src/hardware/thermal.rs`
- **ThermalHealth** (9 connections) — `src/hardware/thermal.rs`
- **SystemThermalSnapshot** (8 connections) — `src/hardware/thermal.rs`
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

- [super](super.md) (5 shared connections)
- [LazyLoader](LazyLoader.md) (2 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)

## Source Files

- `src/hardware/thermal.rs`

## Audit Trail

- EXTRACTED: 35 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*