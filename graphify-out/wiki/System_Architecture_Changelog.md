# System Architecture Changelog

> 29 nodes · cohesion 0.11

## Key Concepts

- **AppxCache** (20 connections) — `src/scanner/appx_scanner.rs`
- **appx_scanner.rs** (13 connections) — `src/scanner/appx_scanner.rs`
- **hashset** (9 connections)
- **.remove_appx_family_in_state()** (7 connections) — `src/core/backend.rs`
- **.evaluate_package_with_kind()** (7 connections) — `src/scanner/appx_scanner.rs`
- **split_appx_name_and_publisher()** (6 connections) — `src/scanner/appx_scanner.rs`
- **appx_package_matches_family()** (5 connections) — `src/scanner/appx_scanner.rs`
- **.from_packages()** (5 connections) — `src/scanner/appx_scanner.rs`
- **.list_installed_appx_detailed()** (4 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_batch()** (4 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (4 connections) — `src/core/backend.rs`
- **.evaluate_package()** (4 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.appx()** (3 connections) — `src/core/backend.rs`
- **.is_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **check_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **test_appx_cache_loaded_empty_no_fallback()** (3 connections) — `src/scanner/appx_scanner.rs`
- **test_split_appx_name_and_publisher_with_underscores_and_prefix()** (3 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.iter()** (2 connections) — `src/scanner/appx_scanner.rs`
- **.load()** (2 connections) — `src/scanner/appx_scanner.rs`
- *... and 4 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (10 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (9 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [View() Color Scrollspec](View_Color_Scrollspec.md) (1 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/appx_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 84 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*