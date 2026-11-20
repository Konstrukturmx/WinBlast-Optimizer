# CoreParkingState

> 13 nodes · cohesion 0.21

## Key Concepts

- **CoreParkingState** (20 connections) — `src/hardware/parking.rs`
- **.detect_internal()** (7 connections) — `src/hardware/parking.rs`
- **.detect_core_parking()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
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

- [parking.rs](parking.rs.md) (4 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [parse_guid](parse_guid.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 40 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*