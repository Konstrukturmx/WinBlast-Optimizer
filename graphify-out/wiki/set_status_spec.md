# .set_status_spec

> 18 nodes · cohesion 0.14

## Key Concepts

- **restore_point.rs** (14 connections) — `src/backup/restore_point.rs`
- **RestorePoint** (14 connections) — `src/backup/restore_point.rs`
- **parse_restore_point_time()** (5 connections) — `src/backup/restore_point.rs`
- **.create_restore_point()** (5 connections) — `src/core/backend.rs`
- **ensure_preflight_restore_point_with()** (4 connections) — `src/backup/restore_point.rs`
- **.list_restore_points()** (4 connections) — `src/core/backend.rs`
- **is_recent_restore_point_present_with()** (3 connections) — `src/backup/restore_point.rs`
- **list_restore_points()** (3 connections) — `src/backup/restore_point.rs`
- **.ensure_preflight_restore_point()** (3 connections) — `src/core/backend.rs`
- **create_restore_point()** (2 connections) — `src/backup/restore_point.rs`
- **ensure_preflight_restore_point()** (2 connections) — `src/backup/restore_point.rs`
- **test_ensure_preflight_restore_point_sandbox()** (2 connections) — `src/backup/restore_point.rs`
- **test_parse_restore_point_time_formats()** (2 connections) — `src/backup/restore_point.rs`
- **.list_restore_points()** (2 connections) — `src/core/backend.rs`
- **CreateRestorePoint** (2 connections) — `src/core/backend.rs`
- **.list_restore_points()** (2 connections) — `src/core/backend.rs`
- **DateTime** (1 connections)
- **Utc** (1 connections)

## Relationships

- [full_telemetry_audit_with](full_telemetry_audit_with.md) (5 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (4 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (2 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)

## Source Files

- `src/backup/restore_point.rs`
- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 48 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*