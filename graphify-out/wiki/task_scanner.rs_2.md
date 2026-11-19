# task_scanner.rs

> 19 nodes · cohesion 0.18

## Key Concepts

- **task_scanner.rs** (20 connections) — `src/scanner/task_scanner.rs`
- **hashmap** (8 connections)
- **query_single_task_detailed()** (8 connections) — `src/scanner/task_scanner.rs`
- **.evaluate_task_with_kind()** (7 connections) — `src/scanner/task_scanner.rs`
- **query_task_via_schtasks()** (5 connections) — `src/scanner/task_scanner.rs`
- **.load()** (5 connections) — `src/scanner/task_scanner.rs`
- **is_disabled_status()** (4 connections) — `src/scanner/task_scanner.rs`
- **.from_map()** (4 connections) — `src/scanner/task_scanner.rs`
- **ComGuard** (3 connections) — `src/scanner/task_scanner.rs`
- **parse_csv_line()** (3 connections) — `src/scanner/task_scanner.rs`
- **query_task_via_com()** (3 connections) — `src/scanner/task_scanner.rs`
- **test_evaluate_task_with_kind_all_cache_and_error_branches()** (3 connections) — `src/scanner/task_scanner.rs`
- **canonical_task_path()** (2 connections) — `src/scanner/task_scanner.rs`
- **get_task_enabled()** (2 connections) — `src/scanner/task_scanner.rs`
- **task_leaf_name()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_multilingual_disabled_statuses()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_query_single_task_distinguishes_absent_vs_access_denied()** (2 connections) — `src/scanner/task_scanner.rs`
- **test_task_cache_loaded_does_not_cross_match_prefixes()** (2 connections) — `src/scanner/task_scanner.rs`
- **.drop()** (1 connections) — `src/scanner/task_scanner.rs`

## Relationships

- [TaskCache](TaskCache.md) (5 shared connections)
- [SandboxEnv](SandboxEnv.md) (4 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (2 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [path](path.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)

## Source Files

- `src/scanner/task_scanner.rs`

## Audit Trail

- EXTRACTED: 53 (96%)
- INFERRED: 2 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*