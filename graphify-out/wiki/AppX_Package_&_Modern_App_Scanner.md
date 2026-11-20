# AppX Package & Modern App Scanner

> 29 nodes · cohesion 0.09

## Key Concepts

- **monitor.rs** (55 connections) — `src/hardware/monitor.rs`
- **query_nt_timer_resolution()** (7 connections) — `src/hardware/monitor.rs`
- **query_system_perf_info()** (5 connections) — `src/hardware/monitor.rs`
- **query_system_process_and_thread_counts()** (4 connections) — `src/hardware/monitor.rs`
- **query_system_thread_count()** (4 connections) — `src/hardware/monitor.rs`
- **.capture()** (3 connections) — `src/hardware/latency.rs`
- **GetPerformanceInfo()** (3 connections) — `src/hardware/monitor.rs`
- **NtQueryTimerResolution()** (3 connections) — `src/hardware/monitor.rs`
- **PerformanceInformation** (3 connections) — `src/hardware/monitor.rs`
- **test_nt_timer_resolution()** (2 connections) — `src/hardware/monitor.rs`
- **test_system_thread_count()** (2 connections) — `src/hardware/monitor.rs`
- **all_processor_groups** (1 connections)
- **getactiveprocessorcount** (1 connections)
- **getactiveprocessorgroupcount** (1 connections)
- **memoryrefreshkind** (1 connections)
- **pdh_cstatus_new_data** (1 connections)
- **pdh_cstatus_valid_data** (1 connections)
- **pdh_fmt_countervalue_item_w** (1 connections)
- **pdh_fmt_double** (1 connections)
- **pdh_more_data** (1 connections)
- **pdhaddenglishcounterw** (1 connections)
- **pdhclosequery** (1 connections)
- **pdhcollectquerydata** (1 connections)
- **pdhgetformattedcounterarrayw** (1 connections)
- **pdhopenqueryw** (1 connections)
- *... and 4 more nodes in this community*

## Relationships

- [WindowsOptionalFeature](WindowsOptionalFeature.md) (9 shared connections)
- [DateTime](DateTime.md) (7 shared connections)
- [TaskCache](TaskCache.md) (6 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (2 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [.logical_core_count](logical_core_count.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [.take_injected_error](take_injected_error.md) (1 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)

## Source Files

- `src/hardware/latency.rs`
- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 71 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*