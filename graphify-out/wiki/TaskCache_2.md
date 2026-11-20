# TaskCache

> 14 nodes · cohesion 0.18

## Key Concepts

- **TaskCache** (19 connections) — `src/scanner/task_scanner.rs`
- **LiveScheduledTaskInfo** (13 connections) — `src/tools/startup.rs`
- **.load_task_cache()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.evaluate_task()** (3 connections) — `src/scanner/task_scanner.rs`
- **list_live_tasks_from_cache()** (3 connections) — `src/tools/startup.rs`
- **.load_task_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_task_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.list_live_tasks()** (2 connections) — `src/core/backend.rs`
- **.load_task_cache()** (2 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (2 connections) — `src/core/backend.rs`
- **.load_task_cache()** (2 connections) — `src/core/backend.rs`
- **.all_entries()** (2 connections) — `src/scanner/task_scanner.rs`
- **.is_task_enabled()** (2 connections) — `src/scanner/task_scanner.rs`
- **Item** (1 connections)

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (5 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (5 shared connections)
- [super](super.md) (3 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [parse_guid](parse_guid.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 45 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*