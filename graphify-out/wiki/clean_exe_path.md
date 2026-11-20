# clean_exe_path

> 5 nodes · cohesion 0.40

## Key Concepts

- **clean_exe_path()** (5 connections) — `src/core/updater.rs`
- **.relaunch_as_admin()** (3 connections) — `src/app.rs`
- **relaunch_as_admin()** (3 connections) — `src/core/elevation.rs`
- **.relaunch_elevated()** (2 connections) — `src/app.rs`
- **test_clean_windows_exe_path()** (2 connections) — `src/core/updater.rs`

## Relationships

- [AppState](AppState.md) (3 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [restart_explorer_safely](restart_explorer_safely.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/elevation.rs`
- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 10 (91%)
- INFERRED: 1 (9%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*