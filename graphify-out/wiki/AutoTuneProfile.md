# AutoTuneProfile

> 25 nodes · cohesion 0.12

## Key Concepts

- **parking.rs** (24 connections) — `src/hardware/parking.rs`
- **is_unsupported_power_guid_error()** (12 connections) — `src/core/backend.rs`
- **ParkingSnapshot** (10 connections) — `src/hardware/parking.rs`
- **read_power_setting_index()** (10 connections) — `src/hardware/power.rs`
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
- **.read_power_setting_index()** (2 connections) — `src/core/backend.rs`
- **.restore_stock_parking()** (2 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (2 connections) — `src/core/backend.rs`
- **.deserialize()** (2 connections) — `src/hardware/parking.rs`
- **.serialize()** (2 connections) — `src/hardware/parking.rs`
- **test_core_parking_detect()** (2 connections) — `src/hardware/parking.rs`
- **CPMAXCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMAXCORES1** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES** (1 connections) — `src/hardware/parking.rs`
- **CPMINCORES1** (1 connections) — `src/hardware/parking.rs`
- **SUB_PROCESSOR** (1 connections) — `src/hardware/parking.rs`

## Relationships

- [normalize_guid_str](normalize_guid_str.md) (6 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (5 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [driver_update.rs](driver_update.rs.md) (4 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (4 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (3 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (2 shared connections)
- [set_usb_root_hub_power_saving](set_usb_root_hub_power_saving.md) (1 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`

## Audit Trail

- EXTRACTED: 65 (87%)
- INFERRED: 10 (13%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*