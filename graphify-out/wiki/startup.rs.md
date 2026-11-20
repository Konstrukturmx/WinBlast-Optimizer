# startup.rs

> 24 nodes · cohesion 0.16

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **list_live_services()** (7 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (6 connections) — `src/tools/startup.rs`
- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **set_scheduled_task_enabled()** (4 connections) — `src/executor/ps_runner.rs`
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **.drop()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- **CRITICAL_CORE_SERVICES** (1 connections) — `src/tools/startup.rs`

## Relationships

- [StartupEntry](StartupEntry.md) (11 shared connections)
- [super](super.md) (5 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (2 shared connections)
- [path](path.md) (2 shared connections)
- [LiveScheduledTaskInfo](LiveScheduledTaskInfo.md) (2 shared connections)
- [.setup](setup.md) (2 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)
- [TaskCache](TaskCache.md) (1 shared connections)

## Source Files

- `src/executor/ps_runner.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 80 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*