# privacy_shield.rs

> 13 nodes · cohesion 0.17

## Key Concepts

- **.sync_startup_tasks_busy()** (9 connections) — `src/app.rs`
- **.scan_startup_and_tasks_async()** (5 connections) — `src/app.rs`
- **.set_live_service_start_async()** (5 connections) — `src/app.rs`
- **.batch_toggle_selected_startup_async()** (4 connections) — `src/app.rs`
- **.batch_toggle_selected_tasks_async()** (4 connections) — `src/app.rs`
- **.run_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_live_task_async()** (4 connections) — `src/app.rs`
- **.toggle_startup_entry_async()** (4 connections) — `src/app.rs`
- **ScheduledTaskRunResult** (3 connections) — `src/app.rs`
- **ServiceLiveUpdated** (2 connections) — `src/app.rs`
- **StartupBatchToggled** (2 connections) — `src/app.rs`
- **StartupEntryToggled** (2 connections) — `src/app.rs`
- **TasksBatchToggled** (2 connections) — `src/app.rs`

## Relationships

- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (8 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (7 shared connections)
- [startup.rs](startup.rs.md) (7 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (1 shared connections)
- [super](super.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 37 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*