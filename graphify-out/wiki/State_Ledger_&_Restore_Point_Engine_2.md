# State Ledger & Restore Point Engine

> 15 nodes · cohesion 0.18

## Key Concepts

- **CoreParkingState** (20 connections) — `src/hardware/parking.rs`
- **.detect_internal()** (8 connections) — `src/hardware/parking.rs`
- **.detect_core_parking()** (4 connections) — `src/core/backend.rs`
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

- [Embedded Catalog Test](Embedded_Catalog_Test.md) (4 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Rollback Panic (`Src/Backup/State](Rollback_Panic_`Src-Backup-State.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [.Detect Core Parking()](Detect_Core_Parking.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)

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