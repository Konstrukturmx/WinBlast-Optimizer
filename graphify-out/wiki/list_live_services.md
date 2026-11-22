# list_live_services

> 11 nodes · cohesion 0.27

## Key Concepts

- **list_live_services()** (8 connections) — `src/tools/startup.rs`
- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **.list_live_services()** (3 connections) — `src/core/backend.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **.drop()** (2 connections) — `src/tools/startup.rs`

## Relationships

- [app.rs](app.rs.md) (8 shared connections)
- [LiveServiceInfo](LiveServiceInfo.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 26 (96%)
- INFERRED: 1 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*