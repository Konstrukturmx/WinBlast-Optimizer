# Project Documentation, Architecture & Licensing

> 21 nodes · cohesion 0.16

## Key Concepts

- **task_scanner.rs** (20 connections) — `src/scanner/task_scanner.rs`
- **decode_oem_bytes()** (18 connections) — `src/scanner/task_scanner.rs`
- **query_single_task_detailed()** (8 connections) — `src/scanner/task_scanner.rs`
- **set_scheduled_task_enabled()** (5 connections) — `src/executor/ps_runner.rs`
- **query_task_via_schtasks()** (5 connections) — `src/scanner/task_scanner.rs`
- **.load()** (5 connections) — `src/scanner/task_scanner.rs`
- **is_disabled_status()** (4 connections) — `src/scanner/task_scanner.rs`
- **ComGuard** (3 connections) — `src/scanner/task_scanner.rs`
- **get_task_enabled()** (3 connections) — `src/scanner/task_scanner.rs`
- **parse_csv_line()** (3 connections) — `src/scanner/task_scanner.rs`
- **query_task_via_com()** (3 connections) — `src/scanner/task_scanner.rs`
- **remove_edge_preserve_webview2()** (3 connections) — `src/tools/features_apps.rs`
- **run_scheduled_task()** (3 connections) — `src/tools/startup.rs`
- **.get_task_enabled()** (2 connections) — `src/core/backend.rs`
- **.remove_edge_preserve_webview2()** (2 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (2 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (2 connections) — `src/core/backend.rs`
- **canonical_task_path()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_multilingual_disabled_statuses()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_query_single_task_distinguishes_absent_vs_access_denied()** (2 connections) — `src/scanner/task_scanner.rs`
- **.drop()** (1 connections) — `src/scanner/task_scanner.rs`

## Relationships

- [Hosts Telemetry Block](Hosts_Telemetry_Block.md) (6 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (4 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (3 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (2 shared connections)
- [.Create Restore Point()](Create_Restore_Point.md) (1 shared connections)
- [Optional Dism Feature()](Optional_Dism_Feature.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Bloatware & AppX Deprovisioning Rules](Bloatware_&_AppX_Deprovisioning_Rules.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/ps_runner.rs`
- `src/scanner/task_scanner.rs`
- `src/tools/features_apps.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 62 (95%)
- INFERRED: 3 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*