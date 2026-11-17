# LiveScheduledTaskInfo

> 10 nodes · cohesion 0.29

## Key Concepts

- **LiveScheduledTaskInfo** (13 connections) — `src/tools/startup.rs`
- **list_live_tasks_from_cache()** (5 connections) — `src/tools/startup.rs`
- **.list_live_tasks()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (4 connections) — `src/core/backend.rs`
- **.load_task_cache()** (4 connections) — `src/core/backend.rs`
- **.tasks()** (3 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (3 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (2 connections) — `src/core/backend.rs`

## Relationships

- [TaskCache](TaskCache.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 32 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*