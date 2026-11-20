# HibernationStatus

> 10 nodes · cohesion 0.24

## Key Concepts

- **LiveServiceInfo** (14 connections) — `src/tools/startup.rs`
- **.list_live_services()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_services()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_services()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_services()** (2 connections) — `src/core/backend.rs`
- **.list_live_services()** (2 connections) — `src/core/backend.rs`

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (5 shared connections)
- [super](super.md) (3 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [TaskCache](TaskCache.md) (3 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 35 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*