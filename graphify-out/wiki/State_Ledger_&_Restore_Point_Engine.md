# State Ledger & Restore Point Engine

> 17 nodes · cohesion 0.17

## Key Concepts

- **thermal.rs** (17 connections) — `src/hardware/thermal.rs`
- **ThermalHealth** (9 connections) — `src/hardware/thermal.rs`
- **SystemThermalSnapshot** (7 connections) — `src/hardware/thermal.rs`
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

- [deserialize](deserialize.md) (4 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (1 shared connections)
- [LazyLoader](LazyLoader.md) (1 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (1 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (1 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (1 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (1 shared connections)

## Source Files

- `src/hardware/thermal.rs`

## Audit Trail

- EXTRACTED: 34 (94%)
- INFERRED: 2 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*