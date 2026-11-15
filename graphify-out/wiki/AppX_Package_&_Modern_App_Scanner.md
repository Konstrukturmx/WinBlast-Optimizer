# AppX Package & Modern App Scanner

> 25 nodes · cohesion 0.13

## Key Concepts

- **net_latency.rs** (20 connections) — `src/hardware/net_latency.rs`
- **LatencyBenchmarkSnapshot** (18 connections) — `src/hardware/net_latency.rs`
- **query_nt_timer_resolution()** (7 connections) — `src/hardware/monitor.rs`
- **measure_dpc_jitter()** (5 connections) — `src/hardware/net_latency.rs`
- **probe_network_target()** (5 connections) — `src/hardware/net_latency.rs`
- **run_full_latency_benchmark()** (5 connections) — `src/hardware/net_latency.rs`
- **DpcJitterResult** (4 connections) — `src/hardware/net_latency.rs`
- **PingTarget** (4 connections) — `src/hardware/net_latency.rs`
- **TargetPingResult** (4 connections) — `src/hardware/net_latency.rs`
- **.run_latency_benchmark()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **duration** (3 connections)
- **instant** (3 connections)
- **.run_latency_benchmark()** (3 connections) — `src/core/backend.rs`
- **NtQueryTimerResolution()** (3 connections) — `src/hardware/monitor.rs`
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **SocketAddr** (2 connections)
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **test_nt_timer_resolution()** (2 connections) — `src/hardware/monitor.rs`
- **PING_TARGETS** (2 connections) — `src/hardware/net_latency.rs`
- **test_dpc_jitter_measurement()** (2 connections) — `src/hardware/net_latency.rs`
- **test_probe_network_zero_probes()** (2 connections) — `src/hardware/net_latency.rs`
- **test_targets_structure()** (1 connections) — `src/hardware/net_latency.rs`
- **tcpstream** (1 connections)

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (7 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (4 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (3 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (3 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/monitor.rs`
- `src/hardware/net_latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 69 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*