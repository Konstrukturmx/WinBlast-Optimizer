# .Create Restore Point()

> 24 nodes · cohesion 0.12

## Key Concepts

- **StartupEntry** (25 connections) — `src/tools/startup.rs`
- **StartupSource** (15 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (7 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (6 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (4 connections) — `src/tools/startup.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (3 connections) — `src/core/backend.rs`
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
- **UwpStartupTask** (1 connections) — `src/tools/startup.rs`

## Relationships

- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (13 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (9 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [View() Color Scrollspec](View_Color_Scrollspec.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 71 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*