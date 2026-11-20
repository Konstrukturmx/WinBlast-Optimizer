# restart_explorer_safely

> 15 nodes · cohesion 0.17

## Key Concepts

- **StartupSource** (15 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (6 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (3 connections) — `src/tools/startup.rs`
- **.as_label()** (1 connections) — `src/tools/startup.rs`
- **RegistryHkcuRun** (1 connections) — `src/tools/startup.rs`
- **RegistryHklmRun** (1 connections) — `src/tools/startup.rs`
- **ScheduledTaskLogon** (1 connections) — `src/tools/startup.rs`
- **StartupFolderAllUsers** (1 connections) — `src/tools/startup.rs`
- **StartupFolderCurrentUser** (1 connections) — `src/tools/startup.rs`
- **UwpStartupTask** (1 connections) — `src/tools/startup.rs`

## Relationships

- [WorkerEvent](WorkerEvent.md) (9 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (5 shared connections)
- [elevation.rs](elevation.rs.md) (2 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 37 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*