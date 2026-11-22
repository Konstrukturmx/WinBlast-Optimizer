# task_scanner.rs

> 56 nodes · cohesion 0.06

## Key Concepts

- **task_scanner.rs** (20 connections) — `src/scanner/task_scanner.rs`
- **TaskCache** (19 connections) — `src/scanner/task_scanner.rs`
- **ps_runner.rs** (16 connections) — `src/executor/ps_runner.rs`
- **normalize_task_path()** (8 connections) — `src/scanner/task_scanner.rs`
- **query_single_task_detailed()** (8 connections) — `src/scanner/task_scanner.rs`
- **block_telemetry_hosts_in_content()** (7 connections) — `src/executor/ps_runner.rs`
- **.evaluate_task_with_kind()** (7 connections) — `src/scanner/task_scanner.rs`
- **StackFmt<N>** (6 connections) — `src/ui_sync/bitops.rs`
- **set_scheduled_task_enabled()** (5 connections) — `src/executor/ps_runner.rs`
- **query_task_via_schtasks()** (5 connections) — `src/scanner/task_scanner.rs`
- **.load()** (5 connections) — `src/scanner/task_scanner.rs`
- **.load_task_cache()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **block_telemetry_hosts()** (4 connections) — `src/executor/ps_runner.rs`
- **unblock_telemetry_hosts_in_content()** (4 connections) — `src/executor/ps_runner.rs`
- **unblock_telemetry_hosts_with()** (4 connections) — `src/executor/ps_runner.rs`
- **is_disabled_status()** (4 connections) — `src/scanner/task_scanner.rs`
- **.from_map()** (4 connections) — `src/scanner/task_scanner.rs`
- **find_exact_marker_line()** (3 connections) — `src/executor/ps_runner.rs`
- **remove_appx_package()** (3 connections) — `src/executor/ps_runner.rs`
- **test_hosts_bom_ipv6_tabs_and_exact_footer_matching()** (3 connections) — `src/executor/ps_runner.rs`
- **write_hosts_file_raw()** (3 connections) — `src/executor/ps_runner.rs`
- **ComGuard** (3 connections) — `src/scanner/task_scanner.rs`
- **get_task_enabled()** (3 connections) — `src/scanner/task_scanner.rs`
- **parse_csv_line()** (3 connections) — `src/scanner/task_scanner.rs`
- **query_task_via_com()** (3 connections) — `src/scanner/task_scanner.rs`
- *... and 31 more nodes in this community*

## Relationships

- [driver_update.rs](driver_update.rs.md) (6 shared connections)
- [decode_oem_bytes](decode_oem_bytes.md) (4 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (2 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [CachedScanBackend<'a>](CachedScanBackend_'a_.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [parse_guid](parse_guid.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/ps_runner.rs`
- `src/scanner/task_scanner.rs`
- `src/ui_sync/bitops.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 118 (94%)
- INFERRED: 8 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*