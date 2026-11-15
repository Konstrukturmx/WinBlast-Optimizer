# Hardware Diagnostics & GPU Silicon Tuning

> 13 nodes · cohesion 0.19

## Key Concepts

- **InstalledAppxPackage** (15 connections) — `src/tools/features_apps.rs`
- **AppxSafetyTier** (10 connections) — `src/tools/features_apps.rs`
- **list_installed_appx_from_cache()** (5 connections) — `src/tools/features_apps.rs`
- **.list_installed_appx_detailed()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `src/core/backend.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (2 connections) — `src/core/backend.rs`
- **AiBloat** (1 connections) — `src/tools/features_apps.rs`
- **.as_label()** (1 connections) — `src/tools/features_apps.rs`
- **Caution** (1 connections) — `src/tools/features_apps.rs`
- **ProtectedCore** (1 connections) — `src/tools/features_apps.rs`
- **SafeBloat** (1 connections) — `src/tools/features_apps.rs`

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (6 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Windows Unattend Deployment Generator](Windows_Unattend_Deployment_Generator.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/features_apps.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 35 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*