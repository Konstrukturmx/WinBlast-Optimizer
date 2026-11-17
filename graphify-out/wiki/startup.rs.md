# startup.rs

> 21 nodes · cohesion 0.19

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **list_live_services()** (8 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (7 connections) — `src/tools/startup.rs`
- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **.drop()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- **CRITICAL_CORE_SERVICES** (1 connections) — `src/tools/startup.rs`

## Relationships

- [.Create Restore Point()](Create_Restore_Point.md) (12 shared connections)
- [LiveServiceInfo](LiveServiceInfo.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [enums](enums.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [path](path.md) (2 shared connections)
- [LiveScheduledTaskInfo](LiveScheduledTaskInfo.md) (2 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (2 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)

## Source Files

- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 75 (97%)
- INFERRED: 2 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*