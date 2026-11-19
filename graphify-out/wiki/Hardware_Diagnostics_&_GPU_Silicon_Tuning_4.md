# Hardware Diagnostics & GPU Silicon Tuning

> 12 nodes · cohesion 0.30

## Key Concepts

- **.sample()** (8 connections) — `src/hardware/monitor.rs`
- **sample_live_cpu_and_parking()** (7 connections) — `src/hardware/monitor.rs`
- **SystemMonitor** (7 connections) — `src/hardware/monitor.rs`
- **active_logical_core_count()** (6 connections) — `src/hardware/monitor.rs`
- **NtCpuSampler** (6 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **.new()** (4 connections) — `src/hardware/monitor.rs`
- **.core_count()** (3 connections) — `src/hardware/monitor.rs`
- **query_processor_group_bases()** (3 connections) — `src/hardware/monitor.rs`
- **test_monitor_cold_main_thread_never_sleeps_or_spikes()** (3 connections) — `src/hardware/monitor.rs`
- **test_monitor_sample()** (3 connections) — `src/hardware/monitor.rs`

## Relationships

- [startup.rs](startup.rs.md) (10 shared connections)
- [query_nt_processor_performance_into](query_nt_processor_performance_into.md) (4 shared connections)
- [PdhParkingCollector](PdhParkingCollector.md) (2 shared connections)
- [power.rs](power.rs.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [SystemMetrics](SystemMetrics.md) (1 shared connections)

## Source Files

- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 39 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*