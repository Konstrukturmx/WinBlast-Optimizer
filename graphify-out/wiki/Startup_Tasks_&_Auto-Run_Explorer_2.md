# Startup Tasks & Auto-Run Explorer

> 6 nodes · cohesion 0.47

## Key Concepts

- **RegStringValue** (11 connections) — `src/core/reg_helpers.rs`
- **.reg_get_string()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `src/core/backend.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (2 connections) — `src/core/backend.rs`

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (1 shared connections)
- [Windows Unattend Deployment Generator](Windows_Unattend_Deployment_Generator.md) (1 shared connections)
- [Integration Suite: Real Windows Env](Integration_Suite-_Real_Windows_Env.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 19 (95%)
- INFERRED: 1 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*