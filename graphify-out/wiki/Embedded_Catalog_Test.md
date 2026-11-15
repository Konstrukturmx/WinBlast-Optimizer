# Embedded Catalog Test

> 23 nodes · cohesion 0.13

## Key Concepts

- **parking.rs** (24 connections) — `src/hardware/parking.rs`
- **is_unsupported_power_guid_error()** (12 connections) — `src/core/backend.rs`
- **ParkingSnapshot** (10 connections) — `src/hardware/parking.rs`
- **write_parking_thresholds()** (8 connections) — `src/hardware/parking.rs`
- **.apply_parking_snapshot()** (4 connections) — `src/core/backend.rs`
- **apply_parking_snapshot()** (4 connections) — `src/hardware/parking.rs`
- **compute_is_fully_unparked()** (4 connections) — `src/hardware/parking.rs`
- **test_parking_snapshot_roundtrip_and_active_rail_logic()** (4 connections) — `src/hardware/parking.rs`
- **set_usb_root_hub_power_saving()** (4 connections) — `src/hardware/power.rs`
- **.apply_parking_snapshot()** (3 connections) — `src/core/backend.rs`
- **capture_parking_snapshot()** (3 connections) — `src/hardware/parking.rs`
- **restore_stock_parking()** (3 connections) — `src/hardware/parking.rs`
- **unpark_all_cores()** (3 connections) — `src/hardware/parking.rs`
- **.restore_stock_parking()** (2 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (2 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (2 connections) — `src/core/backend.rs`
- **.deserialize()** (2 connections) — `src/hardware/parking.rs`
- **.serialize()** (2 connections) — `src/hardware/parking.rs`
- **CPMAXCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMAXCORES1** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES1** (1 connections) — `src/hardware/parking.rs`
- **SUB_PROCESSOR** (1 connections) — `src/hardware/parking.rs`

## Relationships

- [Rollback Panic (`Src/Backup/State](Rollback_Panic_`Src-Backup-State.md) (6 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (4 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (4 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (3 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`

## Audit Trail

- EXTRACTED: 58 (87%)
- INFERRED: 9 (13%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*