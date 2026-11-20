# Hardware Diagnostics & GPU Silicon Tuning

> 11 nodes · cohesion 0.25

## Key Concepts

- **LatencySnapshot** (18 connections) — `src/hardware/latency.rs`
- **latency.rs** (4 connections) — `src/hardware/latency.rs`
- **.capture_latency()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (2 connections) — `src/core/backend.rs`
- **.capture_latency()** (2 connections) — `src/core/backend.rs`
- **DateTime** (2 connections)
- **Utc** (2 connections)
- **.ram_display()** (1 connections) — `src/hardware/latency.rs`
- **.timer_resolution_display()** (1 connections) — `src/hardware/latency.rs`

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 28 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*