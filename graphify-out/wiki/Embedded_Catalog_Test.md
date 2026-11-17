# Embedded Catalog Test

> 21 nodes · cohesion 0.14

## Key Concepts

- **parking.rs** (24 connections) — `src/hardware/parking.rs`
- **ParkingSnapshot** (10 connections) — `src/hardware/parking.rs`
- **write_parking_thresholds()** (8 connections) — `src/hardware/parking.rs`
- **.apply_parking_snapshot()** (4 connections) — `src/core/backend.rs`
- **apply_parking_snapshot()** (4 connections) — `src/hardware/parking.rs`
- **compute_is_fully_unparked()** (4 connections) — `src/hardware/parking.rs`
- **test_parking_snapshot_roundtrip_and_active_rail_logic()** (4 connections) — `src/hardware/parking.rs`
- **.apply_parking_snapshot()** (3 connections) — `src/core/backend.rs`
- **capture_parking_snapshot()** (3 connections) — `src/hardware/parking.rs`
- **CoreStatus** (3 connections) — `src/hardware/parking.rs`
- **restore_stock_parking()** (3 connections) — `src/hardware/parking.rs`
- **unpark_all_cores()** (3 connections) — `src/hardware/parking.rs`
- **.restore_stock_parking()** (2 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (2 connections) — `src/core/backend.rs`
- **.deserialize()** (2 connections) — `src/hardware/parking.rs`
- **.serialize()** (2 connections) — `src/hardware/parking.rs`
- **CPMAXCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMAXCORES1** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES1** (1 connections) — `src/hardware/parking.rs`
- **SUB_PROCESSOR** (1 connections) — `src/hardware/parking.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (4 shared connections)
- [RealBackend](RealBackend.md) (3 shared connections)
- [engine.rs](engine.rs.md) (3 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (3 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (1 shared connections)
- [super](super.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`

## Audit Trail

- EXTRACTED: 50 (89%)
- INFERRED: 6 (11%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*