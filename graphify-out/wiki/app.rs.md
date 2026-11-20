# app.rs

> 19 nodes · cohesion 0.14

## Key Concepts

- **path** (12 connections)
- **pathbuf** (10 connections)
- **diagnostics.rs** (10 connections) — `src/tools/diagnostics.rs`
- **FaultDiagnosticsReport** (10 connections) — `src/tools/diagnostics.rs`
- **profile_io.rs** (8 connections) — `src/backup/profile_io.rs`
- **export_profile()** (4 connections) — `src/backup/profile_io.rs`
- **import_profile()** (3 connections) — `src/backup/profile_io.rs`
- **test_profile_export_import()** (3 connections) — `src/backup/profile_io.rs`
- **parse_multi_sz_lossless()** (3 connections) — `src/tools/diagnostics.rs`
- **TweakProfile** (2 connections) — `src/backup/profile_io.rs`
- **.new()** (2 connections) — `src/backup/profile_io.rs`
- **.scan()** (2 connections) — `src/tools/diagnostics.rs`
- **MinidumpEntry** (2 connections) — `src/tools/diagnostics.rs`
- **PendingRebootStatus** (2 connections) — `src/tools/diagnostics.rs`
- **test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()** (2 connections) — `tests/edge_cases_unorthodox.rs`
- **.clear_cbs_reboot_flags()** (1 connections) — `src/tools/diagnostics.rs`
- **.clear_minidumps()** (1 connections) — `src/tools/diagnostics.rs`
- **.clear_pending_renames()** (1 connections) — `src/tools/diagnostics.rs`
- **.restart_system()** (1 connections) — `src/tools/diagnostics.rs`

## Relationships

- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [Appx Package() Prefixes](Appx_Package_Prefixes.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [features_apps.rs](features_apps.rs.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [WorkerEvent](WorkerEvent.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (2 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)

## Source Files

- `src/backup/profile_io.rs`
- `src/tools/diagnostics.rs`
- `tests/edge_cases_unorthodox.rs`

## Audit Trail

- EXTRACTED: 53 (96%)
- INFERRED: 2 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*