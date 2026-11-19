# parking.rs

> 18 nodes · cohesion 0.17

## Key Concepts

- **parking.rs** (24 connections) — `src/hardware/parking.rs`
- **ParkingSnapshot** (10 connections) — `src/hardware/parking.rs`
- **write_parking_thresholds()** (8 connections) — `src/hardware/parking.rs`
- **.apply_parking_snapshot()** (4 connections) — `src/core/backend.rs`
- **test_parking_snapshot_roundtrip_and_active_rail_logic()** (4 connections) — `src/hardware/parking.rs`
- **apply_parking_snapshot()** (3 connections) — `src/hardware/parking.rs`
- **capture_parking_snapshot()** (3 connections) — `src/hardware/parking.rs`
- **compute_is_fully_unparked()** (3 connections) — `src/hardware/parking.rs`
- **.apply_parking_snapshot()** (2 connections) — `src/core/backend.rs`
- **.deserialize()** (2 connections) — `src/hardware/parking.rs`
- **.serialize()** (2 connections) — `src/hardware/parking.rs`
- **restore_stock_parking()** (2 connections) — `src/hardware/parking.rs`
- **unpark_all_cores()** (2 connections) — `src/hardware/parking.rs`
- **CPMAXCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMAXCORES1** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES1** (1 connections) — `src/hardware/parking.rs`
- **SUB_PROCESSOR** (1 connections) — `src/hardware/parking.rs`

## Relationships

- [CoreParkingState](CoreParkingState.md) (4 shared connections)
- [.take_injected_error](take_injected_error.md) (3 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (3 shared connections)
- [parse_guid](parse_guid.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [WinBlastError](WinBlastError.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [power.rs](power.rs.md) (1 shared connections)
- [path](path.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`

## Audit Trail

- EXTRACTED: 42 (88%)
- INFERRED: 6 (12%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*