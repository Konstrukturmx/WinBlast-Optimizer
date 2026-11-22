# tools/game_overlay.rs

> 12 nodes · cohesion 0.36

## Key Concepts

- **svc_mutator.rs** (13 connections) — `src/executor/svc_mutator.rs`
- **set_service_start_type()** (9 connections) — `src/executor/svc_mutator.rs`
- **is_protected_core_service()** (7 connections) — `src/tools/startup.rs`
- **format_sc_output()** (6 connections) — `src/executor/svc_mutator.rs`
- **stop_service()** (6 connections) — `src/executor/svc_mutator.rs`
- **start_service()** (5 connections) — `src/executor/svc_mutator.rs`
- **output_has_sc_code()** (4 connections) — `src/executor/svc_mutator.rs`
- **validate_service_name()** (4 connections) — `src/executor/svc_mutator.rs`
- **.set_service_start_type()** (2 connections) — `src/core/backend.rs`
- **Output** (2 connections)
- **sc_start_mode()** (2 connections) — `src/executor/svc_mutator.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`

## Relationships

- [app.rs](app.rs.md) (3 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [decode_oem_bytes](decode_oem_bytes.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [AppxCache](AppxCache.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [list_live_services](list_live_services.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/svc_mutator.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 35 (92%)
- INFERRED: 3 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*