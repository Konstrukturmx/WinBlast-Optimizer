# SlowScanBackend

> 27 nodes · cohesion 0.15

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **StartupEntry** (25 connections) — `src/tools/startup.rs`
- **list_live_services()** (7 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (6 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (6 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (5 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **set_scheduled_task_enabled()** (4 connections) — `src/executor/ps_runner.rs`
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (3 connections) — `src/tools/startup.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled()** (2 connections) — `src/core/backend.rs`
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- *... and 2 more nodes in this community*

## Relationships

- [UiThemeStyle](UiThemeStyle.md) (7 shared connections)
- [deserialize](deserialize.md) (4 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (4 shared connections)
- [TaskCache](TaskCache.md) (3 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [diff_modal.rs](diff_modal.rs.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/ps_runner.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 106 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*