# backend.rs

> 23 nodes · cohesion 0.09

## Key Concepts

- **TweakAction** (36 connections) — `src/core/risk.rs`
- **PowerTargetScope** (8 connections) — `src/core/risk.rs`
- **DynamicDwordFormula** (7 connections) — `src/core/risk.rs`
- **resolve_dynamic_dword()** (7 connections) — `src/scanner/mod.rs`
- **.seed_tweak_defaults()** (3 connections) — `src/core/backend.rs`
- **LogicalCoreCount** (1 connections) — `src/core/risk.rs`
- **TotalRamInKb** (1 connections) — `src/core/risk.rs`
- **TotalRamInKbPlusMargin** (1 connections) — `src/core/risk.rs`
- **AcAndDc** (1 connections) — `src/core/risk.rs`
- **AcOnly** (1 connections) — `src/core/risk.rs`
- **DcOnly** (1 connections) — `src/core/risk.rs`
- **AppXPackage** (1 connections) — `src/core/risk.rs`
- **CompositeStringKey** (1 connections) — `src/core/risk.rs`
- **PowerCfgSetting** (1 connections) — `src/core/risk.rs`
- **RegistryBinaryBit** (1 connections) — `src/core/risk.rs`
- **RegistryDelete** (1 connections) — `src/core/risk.rs`
- **RegistryDeleteTree** (1 connections) — `src/core/risk.rs`
- **RegistryDword** (1 connections) — `src/core/risk.rs`
- **RegistryDynamicDword** (1 connections) — `src/core/risk.rs`
- **RegistryString** (1 connections) — `src/core/risk.rs`
- **ScheduledTask** (1 connections) — `src/core/risk.rs`
- **ServiceState** (1 connections) — `src/core/risk.rs`
- **WindowsFeature** (1 connections) — `src/core/risk.rs`

## Relationships

- [registry_scanner.rs](registry_scanner.rs.md) (6 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (5 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (5 shared connections)
- [boot_config.rs](boot_config.rs.md) (4 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 54 (95%)
- INFERRED: 3 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*