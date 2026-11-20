# Disk Cleaning & Temporary File Purger

> 60 nodes · cohesion 0.06

## Key Concepts

- **cleaner.rs** (46 connections) — `src/tools/cleaner.rs`
- **CleanTargetKind** (32 connections) — `src/tools/cleaner.rs`
- **CleanTargetScan** (15 connections) — `src/tools/cleaner.rs`
- **CleanExecutionReport** (14 connections) — `src/tools/cleaner.rs`
- **delete_dir_contents()** (9 connections) — `src/tools/cleaner.rs`
- **measure_dir()** (8 connections) — `src/tools/cleaner.rs`
- **CleanTargetResult** (6 connections) — `src/tools/cleaner.rs`
- **execute_clean_targets()** (6 connections) — `src/tools/cleaner.rs`
- **scan_all_clean_targets()** (6 connections) — `src/tools/cleaner.rs`
- **.execute_clean_targets()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **empty_recycle_bin()** (5 connections) — `src/tools/cleaner.rs`
- **.execute_clean_targets()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.execute_clean_targets()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **init_com_sta()** (4 connections) — `src/tools/cleaner.rs`
- **query_recycle_bin()** (4 connections) — `src/tools/cleaner.rs`
- **resolve_target_paths()** (4 connections) — `src/tools/cleaner.rs`
- **should_skip_entry()** (4 connections) — `src/tools/cleaner.rs`
- **.scan_clean_targets()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.execute_clean_targets()** (3 connections) — `src/core/backend.rs`
- **.scan_clean_targets()** (3 connections) — `src/core/backend.rs`
- **.execute_clean_targets()** (3 connections) — `src/core/backend.rs`
- **ComStaGuard** (3 connections) — `src/tools/cleaner.rs`
- **dir_meta_if_real()** (3 connections) — `src/tools/cleaner.rs`
- **is_reparse_point()** (3 connections) — `src/tools/cleaner.rs`
- **record_dir_io_error()** (3 connections) — `src/tools/cleaner.rs`
- *... and 35 more nodes in this community*

## Relationships

- [backend.rs](backend.rs.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (4 shared connections)
- [AppState](AppState.md) (3 shared connections)
- [super](super.md) (3 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [theme.rs](theme.rs.md) (3 shared connections)
- [BlastResult](BlastResult.md) (3 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [path](path.md) (2 shared connections)
- [WinBlastError](WinBlastError.md) (2 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/tools/cleaner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 148 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*