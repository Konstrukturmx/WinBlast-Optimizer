# Error.Rs Winblasterror Appx

> 18 nodes · cohesion 0.20

## Key Concepts

- **net_latency.rs** (20 connections) — `src/hardware/net_latency.rs`
- **LatencyBenchmarkSnapshot** (18 connections) — `src/hardware/net_latency.rs`
- **measure_dpc_jitter()** (5 connections) — `src/hardware/net_latency.rs`
- **probe_network_target()** (5 connections) — `src/hardware/net_latency.rs`
- **DpcJitterResult** (4 connections) — `src/hardware/net_latency.rs`
- **PingTarget** (4 connections) — `src/hardware/net_latency.rs`
- **run_full_latency_benchmark()** (4 connections) — `src/hardware/net_latency.rs`
- **TargetPingResult** (4 connections) — `src/hardware/net_latency.rs`
- **.run_latency_benchmark()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **SocketAddr** (3 connections)
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.run_latency_benchmark()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **.run_latency_benchmark()** (2 connections) — `src/core/backend.rs`
- **PING_TARGETS** (2 connections) — `src/hardware/net_latency.rs`
- **test_dpc_jitter_measurement()** (2 connections) — `src/hardware/net_latency.rs`
- **test_probe_network_zero_probes()** (2 connections) — `src/hardware/net_latency.rs`
- **test_targets_structure()** (1 connections) — `src/hardware/net_latency.rs`

## Relationships

- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (4 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (3 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [parking.rs](parking.rs.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [WinBlastScrollable](WinBlastScrollable.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/net_latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 56 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*