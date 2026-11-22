# WinBlast Optimizer — Developer & Architecture Guide

> 10 nodes · cohesion 0.29

## Key Concepts

- **InstalledAppxPackage** (15 connections) — `src/tools/features_apps.rs`
- **list_installed_appx_from_cache_with_provisioned()** (10 connections) — `src/tools/features_apps.rs`
- **list_installed_appx_from_cache()** (5 connections) — `src/tools/features_apps.rs`
- **.list_installed_appx_detailed()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `src/core/backend.rs`
- **query_provisioned_appx_families()** (3 connections) — `src/tools/features_apps.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_installed_appx_detailed()** (2 connections) — `src/core/backend.rs`
- **is_microsoft_appx_publisher()** (2 connections) — `src/tools/features_apps.rs`

## Relationships

- [features_apps.rs](features_apps.rs.md) (5 shared connections)
- [AppxSafetyTier](AppxSafetyTier.md) (3 shared connections)
- [CachedScanBackend<'a>](CachedScanBackend_'a_.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [appx_scanner.rs](appx_scanner.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/features_apps.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 37 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*