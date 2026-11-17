# Hardware Diagnostics & GPU Silicon Tuning

> 14 nodes · cohesion 0.19

## Key Concepts

- **PlatformSecurityInfo** (16 connections) — `src/hardware/sys_inventory.rs`
- **detect_platform_security_info()** (8 connections) — `src/hardware/sys_inventory.rs`
- **total_physical_ram_kb()** (4 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/sys_inventory.rs`
- **query_live_code_integrity_options()** (3 connections) — `src/hardware/sys_inventory.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.total_physical_ram_kb()** (2 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (2 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (2 connections) — `src/core/backend.rs`
- **c_void** (1 connections)

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (9 shared connections)
- [RealBackend](RealBackend.md) (2 shared connections)
- [Error.Rs Winblasterror Appx](Error.Rs_Winblasterror_Appx.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/sys_inventory.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 40 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*