# .poll

> 9 nodes · cohesion 0.25

## Key Concepts

- **.poll()** (6 connections) — `src/app.rs`
- **.drain_worker_events_bounded()** (5 connections) — `src/app.rs`
- **.sync_search_state()** (4 connections) — `src/app.rs`
- **attach_window_icon()** (3 connections) — `src/ui/window_icon.rs`
- **.drain_worker_events()** (2 connections) — `src/app.rs`
- **.is_in_progress()** (2 connections) — `src/app.rs`
- **window_icon.rs** (2 connections) — `src/ui/window_icon.rs`
- **EMBEDDED_ICON_BYTES** (1 connections) — `src/ui/window_icon.rs`
- **Tx** (1 connections)

## Relationships

- [AppState](AppState.md) (5 shared connections)
- [WorkerEvent](WorkerEvent.md) (2 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/ui/window_icon.rs`

## Audit Trail

- EXTRACTED: 17 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*