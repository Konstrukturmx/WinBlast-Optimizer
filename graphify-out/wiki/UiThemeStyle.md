# UiThemeStyle

> 8 nodes · cohesion 0.39

## Key Concepts

- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **.drop()** (2 connections) — `src/tools/startup.rs`

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (7 shared connections)

## Source Files

- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 17 (94%)
- INFERRED: 1 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*