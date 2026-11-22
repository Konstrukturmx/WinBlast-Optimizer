# startup.rs

> 7 nodes · cohesion 0.38

## Key Concepts

- **PowerSchemeInfo** (15 connections) — `src/hardware/power.rs`
- **RollbackRefreshData** (6 connections) — `src/app.rs`
- **.list_power_schemes()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_power_schemes()** (3 connections) — `src/core/backend.rs`
- **.list_power_schemes()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_power_schemes()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_power_schemes()** (2 connections) — `src/core/backend.rs`

## Relationships

- [PlatformSecurityInfo](PlatformSecurityInfo.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [WinBlastError](WinBlastError.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 28 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*