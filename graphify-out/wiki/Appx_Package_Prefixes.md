# Appx Package() Prefixes

> 36 nodes · cohesion 0.07

## Key Concepts

- **updater.rs** (21 connections) — `src/core/updater.rs`
- **UpdateStatus** (10 connections) — `src/app.rs`
- **UpdateState** (9 connections) — `src/core/updater.rs`
- **.download_and_install_update()** (5 connections) — `src/app.rs`
- **check_latest_release()** (5 connections) — `src/core/updater.rs`
- **clean_exe_path()** (5 connections) — `src/core/updater.rs`
- **ReleaseInfo** (5 connections) — `src/core/updater.rs`
- **Failed** (4 connections) — `src/app.rs`
- **apply_and_restart()** (4 connections) — `src/core/updater.rs`
- **is_version_newer()** (4 connections) — `src/core/updater.rs`
- **.apply_and_restart()** (3 connections) — `src/app.rs`
- **.relaunch_elevated()** (2 connections) — `src/app.rs`
- **Available** (2 connections) — `src/app.rs`
- **UpdateDownloadCompleted** (2 connections) — `src/app.rs`
- **relaunch_as_admin()** (2 connections) — `src/core/elevation.rs`
- **download_release_asset()** (2 connections) — `src/core/updater.rs`
- **fetch_url_json()** (2 connections) — `src/core/updater.rs`
- **GhAsset** (2 connections) — `src/core/updater.rs`
- **GhRelease** (2 connections) — `src/core/updater.rs`
- **test_clean_windows_exe_path()** (2 connections) — `src/core/updater.rs`
- **test_release_info_structure()** (2 connections) — `src/core/updater.rs`
- **test_version_comparison_logic()** (2 connections) — `src/core/updater.rs`
- **Checking** (1 connections) — `src/app.rs`
- **Downloading** (1 connections) — `src/app.rs`
- **Idle** (1 connections) — `src/app.rs`
- *... and 11 more nodes in this community*

## Relationships

- [Embedded Catalog Test](Embedded_Catalog_Test.md) (6 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (4 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [PdhParkingCollector](PdhParkingCollector.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [WinBlastScrollable](WinBlastScrollable.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/elevation.rs`
- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 60 (92%)
- INFERRED: 5 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*