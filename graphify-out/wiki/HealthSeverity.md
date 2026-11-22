# HealthSeverity

> 7 nodes · cohesion 0.43

## Key Concepts

- **LiveScheduledTaskInfo** (13 connections) — `src/tools/startup.rs`
- **list_live_tasks_from_cache()** (5 connections) — `src/tools/startup.rs`
- **.list_live_tasks()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (3 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (2 connections) — `src/core/backend.rs`

## Relationships

- [app.rs](app.rs.md) (3 shared connections)
- [CachedScanBackend<'a>](CachedScanBackend_'a_.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [HibernationStatus](HibernationStatus.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 25 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*