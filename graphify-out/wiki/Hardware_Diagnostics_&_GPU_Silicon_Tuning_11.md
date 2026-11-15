# Hardware Diagnostics & GPU Silicon Tuning

> 8 nodes · cohesion 0.29

## Key Concepts

- **detect_platform_security_info()** (8 connections) — `src/hardware/sys_inventory.rs`
- **total_physical_ram_kb()** (4 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/sys_inventory.rs`
- **query_live_code_integrity_options()** (3 connections) — `src/hardware/sys_inventory.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- **.total_physical_ram_kb()** (2 connections) — `src/core/backend.rs`
- **c_void** (1 connections)

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (2 shared connections)
- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (1 shared connections)
- [Error.Rs Winblasterror Appx](Error.Rs_Winblasterror_Appx.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/sys_inventory.rs`

## Audit Trail

- EXTRACTED: 19 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*