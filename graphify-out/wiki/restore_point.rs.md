# restore_point.rs

> 15 nodes · cohesion 0.18

## Key Concepts

- **decode_oem_bytes()** (18 connections) — `src/scanner/task_scanner.rs`
- **restore_point.rs** (14 connections) — `src/backup/restore_point.rs`
- **parse_restore_point_time()** (5 connections) — `src/backup/restore_point.rs`
- **ensure_preflight_restore_point_with()** (4 connections) — `src/backup/restore_point.rs`
- **is_recent_restore_point_present_with()** (3 connections) — `src/backup/restore_point.rs`
- **list_restore_points()** (3 connections) — `src/backup/restore_point.rs`
- **create_restore_point()** (2 connections) — `src/backup/restore_point.rs`
- **ensure_preflight_restore_point()** (2 connections) — `src/backup/restore_point.rs`
- **test_ensure_preflight_restore_point_sandbox()** (2 connections) — `src/backup/restore_point.rs`
- **test_parse_restore_point_time_formats()** (2 connections) — `src/backup/restore_point.rs`
- **request_system_reboot()** (2 connections) — `src/tools/explorer_ctrl.rs`
- **remove_edge_preserve_webview2()** (2 connections) — `src/tools/features_apps.rs`
- **run_scheduled_task()** (2 connections) — `src/tools/startup.rs`
- **DateTime** (1 connections)
- **Utc** (1 connections)

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (3 shared connections)
- [RestorePoint](RestorePoint.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [.setup](setup.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [path](path.md) (1 shared connections)
- [SandboxEnv](SandboxEnv.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/backup/restore_point.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/explorer_ctrl.rs`
- `src/tools/features_apps.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 41 (93%)
- INFERRED: 3 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*