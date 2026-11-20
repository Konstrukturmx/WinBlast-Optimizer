# StartupEntry

> 20 nodes · cohesion 0.14

## Key Concepts

- **StartupEntry** (25 connections) — `src/tools/startup.rs`
- **StartupSource** (15 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (6 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
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

- [startup.rs](startup.rs.md) (11 shared connections)
- [test_wave1_ledger_rollback_and_concurrency_regressions](test_wave1_ledger_rollback_and_concurrency_regressions.md) (6 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [theme.rs](theme.rs.md) (2 shared connections)
- [action_button_guarded](action_button_guarded.md) (2 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 61 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*