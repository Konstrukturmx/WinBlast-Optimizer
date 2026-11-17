# Hardware Diagnostics & GPU Silicon Tuning

> 12 nodes · cohesion 0.20

## Key Concepts

- **InstalledAppxPackage** (15 connections) — `src/tools/features_apps.rs`
- **AppxSafetyTier** (10 connections) — `src/tools/features_apps.rs`
- **.list_installed_appx_detailed()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (2 connections) — `src/core/backend.rs`
- **.list_installed_appx_detailed()** (2 connections) — `src/core/backend.rs`
- **AiBloat** (1 connections) — `src/tools/features_apps.rs`
- **.as_label()** (1 connections) — `src/tools/features_apps.rs`
- **Caution** (1 connections) — `src/tools/features_apps.rs`
- **ProtectedCore** (1 connections) — `src/tools/features_apps.rs`
- **SafeBloat** (1 connections) — `src/tools/features_apps.rs`

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (5 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [action_button_guarded](action_button_guarded.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/features_apps.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 31 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*