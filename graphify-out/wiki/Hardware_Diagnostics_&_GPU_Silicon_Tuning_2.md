# Hardware Diagnostics & GPU Silicon Tuning

> 28 nodes · cohesion 0.09

## Key Concepts

- **monitor.rs** (52 connections) — `src/hardware/monitor.rs`
- **query_nt_timer_resolution()** (7 connections) — `src/hardware/monitor.rs`
- **query_system_process_and_thread_counts()** (4 connections) — `src/hardware/monitor.rs`
- **query_system_thread_count()** (4 connections) — `src/hardware/monitor.rs`
- **.capture()** (3 connections) — `src/hardware/latency.rs`
- **GetPerformanceInfo()** (3 connections) — `src/hardware/monitor.rs`
- **NtQueryTimerResolution()** (3 connections) — `src/hardware/monitor.rs`
- **PerformanceInformation** (2 connections) — `src/hardware/monitor.rs`
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
- **refreshkind** (1 connections)
- *... and 3 more nodes in this community*

## Relationships

- [Restore Points() .List](Restore_Points_.List.md) (8 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (6 shared connections)
- [PdhParkingCollector](PdhParkingCollector.md) (5 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (2 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [SilentCommand](SilentCommand.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)

## Source Files

- `src/hardware/latency.rs`
- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 65 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*