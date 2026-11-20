# edge_cases_unorthodox.rs

> 15 nodes · cohesion 0.18

## Key Concepts

- **CoreParkingState** (20 connections) — `src/hardware/parking.rs`
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

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [WorkerEvent](WorkerEvent.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)
- [HibernationStatus](HibernationStatus.md) (1 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 45 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*