# .poll

> 6 nodes · cohesion 0.40

## Key Concepts

- **.drain_worker_events_bounded()** (5 connections) — `src/app.rs`
- **.poll()** (5 connections) — `src/app.rs`
- **.sync_search_state()** (4 connections) — `src/app.rs`
- **.drain_worker_events()** (2 connections) — `src/app.rs`
- **.is_in_progress()** (2 connections) — `src/app.rs`
- **Tx** (1 connections)

## Relationships

- [AppState](AppState.md) (5 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [.navigate](navigate.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 13 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*