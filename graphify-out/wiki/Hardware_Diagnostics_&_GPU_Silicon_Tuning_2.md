# Hardware Diagnostics & GPU Silicon Tuning

> 32 nodes · cohesion 0.08

## Key Concepts

- **monitor.rs** (52 connections) — `src/hardware/monitor.rs`
- **PdhParkingCollector** (7 connections) — `src/hardware/monitor.rs`
- **query_system_process_and_thread_counts()** (4 connections) — `src/hardware/monitor.rs`
- **query_system_thread_count()** (4 connections) — `src/hardware/monitor.rs`
- **.capture()** (3 connections) — `src/hardware/latency.rs`
- **GetPerformanceInfo()** (3 connections) — `src/hardware/monitor.rs`
- **parse_pdh_processor_instance()** (3 connections) — `src/hardware/monitor.rs`
- **test_nt_cpu_delta_handles_idledisable_and_multigroup()** (3 connections) — `src/hardware/monitor.rs`
- **PDH_HCOUNTER** (2 connections)
- **PDH_HQUERY** (2 connections)
- **.sample()** (2 connections) — `src/hardware/monitor.rs`
- **PerformanceInformation** (2 connections) — `src/hardware/monitor.rs`
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
- *... and 7 more nodes in this community*

## Relationships

- [Restore Points() .List](Restore_Points_.List.md) (10 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (6 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (6 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)

## Source Files

- `src/hardware/latency.rs`
- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 68 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*