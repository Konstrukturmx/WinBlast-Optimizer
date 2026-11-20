# SandboxEnv

> 16 nodes · cohesion 0.21

## Key Concepts

- **DriverError** (13 connections) — `src/hardware/driver_update.rs`
- **download_and_install_driver_with_progress()** (10 connections) — `src/hardware/driver_update.rs`
- **install_sandbox_driver()** (8 connections) — `src/hardware/driver_update.rs`
- **backup_driver()** (7 connections) — `src/hardware/driver_update.rs`
- **DriverProgressCallback** (6 connections) — `src/hardware/driver_update.rs`
- **StagingFailed** (3 connections) — `src/hardware/driver_update.rs`
- **Failed** (3 connections) — `src/hardware/driver_update.rs`
- **Formatter** (2 connections)
- **.fmt()** (2 connections) — `src/backup/preset_sharing.rs`
- **BackupFailed** (2 connections) — `src/hardware/driver_update.rs`
- **DownloadFailed** (2 connections) — `src/hardware/driver_update.rs`
- **.fmt()** (2 connections) — `src/hardware/driver_update.rs`
- **InstallFailed** (2 connections) — `src/hardware/driver_update.rs`
- **Other** (1 connections) — `src/hardware/driver_update.rs`
- **ScanFailed** (1 connections) — `src/hardware/driver_update.rs`
- **VerificationFailed** (1 connections) — `src/hardware/driver_update.rs`

## Relationships

- [RegRawSnapshot](RegRawSnapshot.md) (7 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (6 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [adaptive_tuning.rs](adaptive_tuning.rs.md) (1 shared connections)

## Source Files

- `src/backup/preset_sharing.rs`
- `src/hardware/driver_update.rs`

## Audit Trail

- EXTRACTED: 40 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*