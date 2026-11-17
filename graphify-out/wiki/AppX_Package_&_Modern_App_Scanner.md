# AppX Package & Modern App Scanner

> 19 nodes · cohesion 0.19

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
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **PING_TARGETS** (2 connections) — `src/hardware/net_latency.rs`
- **test_dpc_jitter_measurement()** (2 connections) — `src/hardware/net_latency.rs`
- **test_probe_network_zero_probes()** (2 connections) — `src/hardware/net_latency.rs`
- **test_targets_structure()** (1 connections) — `src/hardware/net_latency.rs`

## Relationships

- [backend.rs](backend.rs.md) (4 shared connections)
- [SilentCommand](SilentCommand.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [WinBlastScrollable](WinBlastScrollable.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/net_latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 58 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*