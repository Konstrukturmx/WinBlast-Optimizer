# test_wave1_ledger_rollback_and_concurrency_regressions

> 22 nodes · cohesion 0.12

## Key Concepts

- **updater.rs** (19 connections) — `src/core/updater.rs`
- **pathbuf** (10 connections)
- **UpdateState** (9 connections) — `src/core/updater.rs`
- **ReleaseInfo** (5 connections) — `src/core/updater.rs`
- **check_latest_release()** (4 connections) — `src/core/updater.rs`
- **is_version_newer()** (4 connections) — `src/core/updater.rs`
- **fetch_url_json()** (2 connections) — `src/core/updater.rs`
- **GhAsset** (2 connections) — `src/core/updater.rs`
- **GhRelease** (2 connections) — `src/core/updater.rs`
- **test_release_info_structure()** (2 connections) — `src/core/updater.rs`
- **test_version_comparison_logic()** (2 connections) — `src/core/updater.rs`
- **apply_and_restart()** (1 connections) — `src/core/updater.rs`
- **download_release_asset()** (1 connections) — `src/core/updater.rs`
- **FALLBACK_REPO** (1 connections) — `src/core/updater.rs`
- **GITHUB_REPO** (1 connections) — `src/core/updater.rs`
- **Checking** (1 connections) — `src/core/updater.rs`
- **Downloading** (1 connections) — `src/core/updater.rs`
- **Failed** (1 connections) — `src/core/updater.rs`
- **Idle** (1 connections) — `src/core/updater.rs`
- **ReadyToRestart** (1 connections) — `src/core/updater.rs`
- **UpdateAvailable** (1 connections) — `src/core/updater.rs`
- **UpToDate** (1 connections) — `src/core/updater.rs`

## Relationships

- [deserialize](deserialize.md) (3 shared connections)
- [path](path.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [heuristic_scanner.rs](heuristic_scanner.rs.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (1 shared connections)
- [SilentCommand](SilentCommand.md) (1 shared connections)

## Source Files

- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 40 (91%)
- INFERRED: 4 (9%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*