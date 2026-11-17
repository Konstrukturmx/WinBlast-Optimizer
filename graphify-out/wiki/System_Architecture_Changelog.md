# System Architecture Changelog

> 14 nodes · cohesion 0.19

## Key Concepts

- **AppxCache** (20 connections) — `src/scanner/appx_scanner.rs`
- **.list_installed_appx_detailed()** (4 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (4 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.appx()** (3 connections) — `src/core/backend.rs`
- **.is_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **check_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.iter()** (2 connections) — `src/scanner/appx_scanner.rs`
- **.load()** (2 connections) — `src/scanner/appx_scanner.rs`
- **Item** (1 connections)

## Relationships

- [appx_scanner.rs](appx_scanner.rs.md) (6 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/appx_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 39 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*