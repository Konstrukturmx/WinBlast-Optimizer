# path

> 14 nodes · cohesion 0.24

## Key Concepts

- **sample_live_cpu_and_parking()** (7 connections) — `src/hardware/monitor.rs`
- **active_logical_core_count()** (6 connections) — `src/hardware/monitor.rs`
- **NtCpuSampler** (6 connections) — `src/hardware/monitor.rs`
- **query_nt_processor_performance_into()** (6 connections) — `src/hardware/monitor.rs`
- **SystemProcessorPerformanceInformation** (6 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **compute_nt_cpu_deltas_into()** (4 connections) — `src/hardware/monitor.rs`
- **.new()** (4 connections) — `src/hardware/monitor.rs`
- **query_nt_processor_performance()** (4 connections) — `src/hardware/monitor.rs`
- **.core_count()** (3 connections) — `src/hardware/monitor.rs`
- **.sample()** (3 connections) — `src/hardware/monitor.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/monitor.rs`
- **query_processor_group_bases()** (3 connections) — `src/hardware/monitor.rs`
- **c_void** (2 connections)

## Relationships

- [parking.rs](parking.rs.md) (9 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (4 shared connections)
- [SystemMetrics](SystemMetrics.md) (3 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (1 shared connections)

## Source Files

- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 39 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*