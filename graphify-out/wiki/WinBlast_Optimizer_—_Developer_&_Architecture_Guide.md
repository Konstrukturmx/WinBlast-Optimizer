# WinBlast Optimizer — Developer & Architecture Guide

> 6 nodes · cohesion 0.47

## Key Concepts

- **RestorePoint** (14 connections) — `src/backup/restore_point.rs`
- **.list_restore_points()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_restore_points()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_restore_points()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_restore_points()** (2 connections) — `src/core/backend.rs`
- **.list_restore_points()** (2 connections) — `src/core/backend.rs`

## Relationships

- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [ThemePalette](ThemePalette.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)

## Source Files

- `src/backup/restore_point.rs`
- `src/core/backend.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 24 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*