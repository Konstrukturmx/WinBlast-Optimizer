# .sync_clean_apps_busy

> 14 nodes · cohesion 0.15

## Key Concepts

- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.scan_clean_targets_async()** (6 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **AppxBatchRemoved** (2 connections) — `src/app.rs`
- **CleanExecuted** (2 connections) — `src/app.rs`
- **CleanTargetsScanned** (2 connections) — `src/app.rs`
- **WinGetInstallResult** (2 connections) — `src/app.rs`

## Relationships

- [AppState](AppState.md) (12 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (8 shared connections)
- [Test Composite With()](Test_Composite_With.md) (8 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 43 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*