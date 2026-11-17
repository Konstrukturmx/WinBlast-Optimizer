# Project Documentation, Architecture & Licensing

> 23 nodes · cohesion 0.14

## Key Concepts

- **task_scanner.rs** (20 connections) — `src/scanner/task_scanner.rs`
- **decode_oem_bytes()** (18 connections) — `src/scanner/task_scanner.rs`
- **query_single_task_detailed()** (8 connections) — `src/scanner/task_scanner.rs`
- **set_scheduled_task_enabled()** (5 connections) — `src/executor/ps_runner.rs`
- **query_task_via_schtasks()** (5 connections) — `src/scanner/task_scanner.rs`
- **.load()** (5 connections) — `src/scanner/task_scanner.rs`
- **is_disabled_status()** (4 connections) — `src/scanner/task_scanner.rs`
- **create_restore_point()** (3 connections) — `src/backup/restore_point.rs`
- **ComGuard** (3 connections) — `src/scanner/task_scanner.rs`
- **get_task_enabled()** (3 connections) — `src/scanner/task_scanner.rs`
- **parse_csv_line()** (3 connections) — `src/scanner/task_scanner.rs`
- **query_task_via_com()** (3 connections) — `src/scanner/task_scanner.rs`
- **remove_edge_preserve_webview2()** (3 connections) — `src/tools/features_apps.rs`
- **run_scheduled_task()** (3 connections) — `src/tools/startup.rs`
- **.create_restore_point()** (2 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (2 connections) — `src/core/backend.rs`
- **.remove_edge_preserve_webview2()** (2 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (2 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (2 connections) — `src/core/backend.rs`
- **canonical_task_path()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_multilingual_disabled_statuses()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_query_single_task_distinguishes_absent_vs_access_denied()** (2 connections) — `src/scanner/task_scanner.rs`
- **.drop()** (1 connections) — `src/scanner/task_scanner.rs`

## Relationships

- [TaskCache](TaskCache.md) (6 shared connections)
- [RealBackend](RealBackend.md) (5 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (3 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (3 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (2 shared connections)
- [.Create Restore Point()](Create_Restore_Point.md) (1 shared connections)
- [Optional Dism Feature()](Optional_Dism_Feature.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)

## Source Files

- `src/backup/restore_point.rs`
- `src/core/backend.rs`
- `src/executor/ps_runner.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/features_apps.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 65 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*