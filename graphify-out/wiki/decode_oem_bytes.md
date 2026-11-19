# decode_oem_bytes

> 11 nodes · cohesion 0.25

## Key Concepts

- **decode_oem_bytes()** (18 connections) — `src/scanner/task_scanner.rs`
- **dism_error_from_output()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_capabilities_locked()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_features_locked()** (6 connections) — `src/tools/features_apps.rs`
- **list_optional_features()** (5 connections) — `src/tools/features_apps.rs`
- **set_scheduled_task_enabled()** (4 connections) — `src/executor/ps_runner.rs`
- **parse_dism_english_state()** (4 connections) — `src/tools/features_apps.rs`
- **request_system_reboot()** (2 connections) — `src/tools/explorer_ctrl.rs`
- **remove_edge_preserve_webview2()** (2 connections) — `src/tools/features_apps.rs`
- **run_scheduled_task()** (2 connections) — `src/tools/startup.rs`
- **Output** (1 connections)

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (9 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (6 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (4 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [restore_point.rs](restore_point.rs.md) (2 shared connections)
- [.setup](setup.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (1 shared connections)
- [Command Osstr Process.Rs](Command_Osstr_Process.Rs.md) (1 shared connections)

## Source Files

- `src/executor/ps_runner.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/explorer_ctrl.rs`
- `src/tools/features_apps.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 42 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*