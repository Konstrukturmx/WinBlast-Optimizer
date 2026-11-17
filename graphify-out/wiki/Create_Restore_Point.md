# .Create Restore Point()

> 26 nodes · cohesion 0.12

## Key Concepts

- **StartupEntry** (25 connections) — `src/tools/startup.rs`
- **StartupSource** (16 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (7 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (6 connections) — `src/tools/startup.rs`
- **.set_startup_entry_enabled()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **HKEY** (4 connections)
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (4 connections) — `src/tools/startup.rs`
- **.set_startup_entry_enabled()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_startup_entry_enabled()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (3 connections) — `src/core/backend.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **.set_startup_entry_enabled_by_id()** (2 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
- **.as_label()** (1 connections) — `src/tools/startup.rs`
- **RegistryHkcuRun** (1 connections) — `src/tools/startup.rs`
- **RegistryHklmRun** (1 connections) — `src/tools/startup.rs`
- **ScheduledTaskLogon** (1 connections) — `src/tools/startup.rs`
- **StartupFolderAllUsers** (1 connections) — `src/tools/startup.rs`
- **StartupFolderCurrentUser** (1 connections) — `src/tools/startup.rs`
- *... and 1 more nodes in this community*

## Relationships

- [startup.rs](startup.rs.md) (12 shared connections)
- [test_wave1_ledger_rollback_and_concurrency_regressions](test_wave1_ledger_rollback_and_concurrency_regressions.md) (4 shared connections)
- [RealBackend](RealBackend.md) (3 shared connections)
- [BlastResult](BlastResult.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [action_button_guarded](action_button_guarded.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 79 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*