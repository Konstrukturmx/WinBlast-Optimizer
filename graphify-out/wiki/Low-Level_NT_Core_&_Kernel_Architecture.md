# Low-Level NT Core & Kernel Architecture

> 12 nodes · cohesion 0.23

## Key Concepts

- **TaskCache** (19 connections) — `src/scanner/task_scanner.rs`
- **.evaluate_task_with_kind()** (7 connections) — `src/scanner/task_scanner.rs`
- **.load_task_cache()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.evaluate_task()** (3 connections) — `src/scanner/task_scanner.rs`
- **.load_task_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_task_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_task_cache()** (2 connections) — `src/core/backend.rs`
- **.load_task_cache()** (2 connections) — `src/core/backend.rs`
- **task_leaf_name()** (2 connections) — `src/scanner/task_scanner.rs`
- **.all_entries()** (2 connections) — `src/scanner/task_scanner.rs`
- **.is_task_enabled()** (2 connections) — `src/scanner/task_scanner.rs`
- **Item** (1 connections)

## Relationships

- [DriverUpdateState](DriverUpdateState.md) (7 shared connections)
- [theme.rs](theme.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/task_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 34 (94%)
- INFERRED: 2 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*