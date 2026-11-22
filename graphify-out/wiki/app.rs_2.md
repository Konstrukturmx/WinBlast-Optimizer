# app.rs

> 24 nodes · cohesion 0.17

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **StartupEntry** (25 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (7 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (6 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (4 connections) — `src/tools/startup.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (3 connections) — `src/core/backend.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **run_scheduled_task()** (3 connections) — `src/tools/startup.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **.run_scheduled_task()** (2 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- **CRITICAL_CORE_SERVICES** (1 connections) — `src/tools/startup.rs`

## Relationships

- [list_live_services](list_live_services.md) (8 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (6 shared connections)
- [driver_update.rs](driver_update.rs.md) (4 shared connections)
- [Item](Item.md) (4 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (3 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [AppxCache](AppxCache.md) (2 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (2 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 103 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*