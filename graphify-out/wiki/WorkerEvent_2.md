# WorkerEvent

> 19 nodes · cohesion 0.21

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **list_live_services()** (7 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (6 connections) — `src/tools/startup.rs`
- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **.drop()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- **CRITICAL_CORE_SERVICES** (1 connections) — `src/tools/startup.rs`

## Relationships

- [restart_explorer_safely](restart_explorer_safely.md) (9 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (3 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (2 shared connections)
- [parking.rs](parking.rs.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [Item](Item.md) (2 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (2 shared connections)
- [LazyLoader](LazyLoader.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 71 (97%)
- INFERRED: 2 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*