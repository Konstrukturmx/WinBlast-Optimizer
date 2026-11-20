# UpdateStatus

> 12 nodes · cohesion 0.17

## Key Concepts

- **UpdateStatus** (10 connections) — `src/app.rs`
- **.download_and_install_update()** (5 connections) — `src/app.rs`
- **Failed** (4 connections) — `src/app.rs`
- **.apply_and_restart()** (3 connections) — `src/app.rs`
- **Available** (2 connections) — `src/app.rs`
- **UpdateDownloadCompleted** (2 connections) — `src/app.rs`
- **download_release_asset()** (2 connections) — `src/core/updater.rs`
- **Checking** (1 connections) — `src/app.rs`
- **Downloading** (1 connections) — `src/app.rs`
- **Idle** (1 connections) — `src/app.rs`
- **ReadyToRestart** (1 connections) — `src/app.rs`
- **UpToDate** (1 connections) — `src/app.rs`

## Relationships

- [WorkerEvent](WorkerEvent.md) (4 shared connections)
- [AppState](AppState.md) (3 shared connections)
- [updater.rs](updater.rs.md) (2 shared connections)
- [elevation.rs](elevation.rs.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 21 (95%)
- INFERRED: 1 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*