# backend.rs

> 19 nodes · cohesion 0.11

## Key Concepts

- **real_windows_env.rs** (33 connections) — `tests/real_windows_env.rs`
- **fs** (4 connections)
- **uuid** (4 connections)
- **TempTestEnvGuard** (3 connections) — `tests/real_windows_env.rs`
- **regvalue** (2 connections)
- **HkcuTestEnvGuard** (2 connections) — `tests/real_windows_env.rs`
- **.new()** (2 connections) — `tests/real_windows_env.rs`
- **test_ntfs_junction_and_readonly_cleanup_sandbox()** (2 connections) — `tests/real_windows_env.rs`
- **test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()** (2 connections) — `tests/real_windows_env.rs`
- **cleantargetkind** (1 connections)
- **coreparkingstate** (1 connections)
- **cpuinfo** (1 connections)
- **is_known_numeric_reg_sz** (1 connections)
- **run_full_scan_with** (1 connections)
- **FILE_ATTRIBUTE_REPARSE_POINT** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **test_real_windows_disk_and_thermal_telemetry()** (1 connections) — `tests/real_windows_env.rs`
- **windowsinfo** (1 connections)

## Relationships

- [theme.rs](theme.rs.md) (3 shared connections)
- [features_apps.rs](features_apps.rs.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (2 shared connections)
- [DriverError](DriverError.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [tools/game_overlay.rs](tools-game_overlay.rs.md) (1 shared connections)

## Source Files

- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 44 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*