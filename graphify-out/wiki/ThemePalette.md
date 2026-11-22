# ThemePalette

> 15 nodes · cohesion 0.23

## Key Concepts

- **sample_live_cpu_and_parking()** (7 connections) — `src/hardware/monitor.rs`
- **active_logical_core_count()** (6 connections) — `src/hardware/monitor.rs`
- **NtCpuSampler** (6 connections) — `src/hardware/monitor.rs`
- **query_nt_processor_performance_into()** (6 connections) — `src/hardware/monitor.rs`
- **SystemProcessorPerformanceInformation** (6 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **compute_nt_cpu_deltas()** (4 connections) — `src/hardware/monitor.rs`
- **compute_nt_cpu_deltas_into()** (4 connections) — `src/hardware/monitor.rs`
- **.new()** (4 connections) — `src/hardware/monitor.rs`
- **query_nt_processor_performance()** (4 connections) — `src/hardware/monitor.rs`
- **.core_count()** (3 connections) — `src/hardware/monitor.rs`
- **.sample()** (3 connections) — `src/hardware/monitor.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/monitor.rs`
- **query_processor_group_bases()** (3 connections) — `src/hardware/monitor.rs`
- **c_void** (1 connections)

## Relationships

- [ps_runner.rs](ps_runner.rs.md) (10 shared connections)
- [clean_exe_path](clean_exe_path.md) (4 shared connections)
- [DateTime](DateTime.md) (2 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (1 shared connections)

## Source Files

- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 40 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*