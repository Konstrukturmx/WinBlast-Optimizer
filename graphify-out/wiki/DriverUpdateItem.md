# DriverUpdateItem

> 15 nodes · cohesion 0.18

## Key Concepts

- **CoreParkingState** (21 connections) — `src/hardware/parking.rs`
- **.detect_internal()** (7 connections) — `src/hardware/parking.rs`
- **.detect_core_parking()** (5 connections) — `src/core/backend.rs`
- **.detect_core_parking()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_core_parking_sampled()** (3 connections) — `src/core/backend.rs`
- **.detect()** (3 connections) — `src/hardware/parking.rs`
- **CoreStatus** (3 connections) — `src/hardware/parking.rs`
- **is_on_ac_power()** (3 connections) — `src/hardware/power.rs`
- **.detect_core_parking()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_core_parking()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_core_parking()** (2 connections) — `src/core/backend.rs`
- **.detect_core_parking_sampled()** (2 connections) — `src/core/backend.rs`
- **.detect_core_parking()** (2 connections) — `src/core/backend.rs`
- **.detect_sampled()** (2 connections) — `src/hardware/parking.rs`
- **test_core_parking_detect()** (2 connections) — `src/hardware/parking.rs`

## Relationships

- [task_scanner.rs](task_scanner.rs.md) (4 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [Any](Any.md) (2 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 46 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*