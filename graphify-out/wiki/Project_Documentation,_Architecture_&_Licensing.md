# Project Documentation, Architecture & Licensing

> 18 nodes · cohesion 0.21

## Key Concepts

- **net_latency.rs** (20 connections) — `src/hardware/net_latency.rs`
- **LatencyBenchmarkSnapshot** (18 connections) — `src/hardware/net_latency.rs`
- **measure_dpc_jitter()** (5 connections) — `src/hardware/net_latency.rs`
- **probe_network_target()** (5 connections) — `src/hardware/net_latency.rs`
- **run_full_latency_benchmark()** (5 connections) — `src/hardware/net_latency.rs`
- **DpcJitterResult** (4 connections) — `src/hardware/net_latency.rs`
- **PingTarget** (4 connections) — `src/hardware/net_latency.rs`
- **TargetPingResult** (4 connections) — `src/hardware/net_latency.rs`
- **.run_latency_benchmark()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **SocketAddr** (3 connections)
- **.run_latency_benchmark()** (3 connections) — `src/core/backend.rs`
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **PING_TARGETS** (2 connections) — `src/hardware/net_latency.rs`
- **test_dpc_jitter_measurement()** (2 connections) — `src/hardware/net_latency.rs`
- **test_probe_network_zero_probes()** (2 connections) — `src/hardware/net_latency.rs`
- **test_targets_structure()** (1 connections) — `src/hardware/net_latency.rs`

## Relationships

- [backend.rs](backend.rs.md) (4 shared connections)
- [appx_scanner.rs](appx_scanner.rs.md) (2 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (2 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (2 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [probe_physical_drive](probe_physical_drive.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/net_latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 57 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*