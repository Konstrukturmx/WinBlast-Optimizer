# Startup Tasks & Auto-Run Explorer

> 12 nodes · cohesion 0.24

## Key Concepts

- **HibernationStatus** (22 connections) — `src/hardware/power.rs`
- **query_hibernation_and_standby()** (4 connections) — `src/hardware/power.rs`
- **set_hibernation_enabled()** (4 connections) — `src/hardware/power.rs`
- **.query_hibernation_status()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (3 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (3 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (3 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (2 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (2 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (2 connections) — `src/core/backend.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (6 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (3 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [.Detect Core Parking()](Detect_Core_Parking.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 39 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*