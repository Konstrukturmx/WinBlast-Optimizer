# Disk Cleaning & Temporary File Purger

> 57 nodes · cohesion 0.06

## Key Concepts

- **cleaner.rs** (46 connections) — `src/tools/cleaner.rs`
- **CleanTargetKind** (33 connections) — `src/tools/cleaner.rs`
- **CleanTargetScan** (15 connections) — `src/tools/cleaner.rs`
- **delete_dir_contents()** (9 connections) — `src/tools/cleaner.rs`
- **measure_dir()** (8 connections) — `src/tools/cleaner.rs`
- **CleanTargetResult** (6 connections) — `src/tools/cleaner.rs`
- **execute_clean_targets()** (6 connections) — `src/tools/cleaner.rs`
- **scan_all_clean_targets()** (6 connections) — `src/tools/cleaner.rs`
- **empty_recycle_bin()** (5 connections) — `src/tools/cleaner.rs`
- **prelude** (4 connections)
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
- **test_delete_dir_preserves_dot_net_and_reparse_safe()** (3 connections) — `src/tools/cleaner.rs`
- **.scan_clean_targets()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.scan_clean_targets()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- *... and 32 more nodes in this community*

## Relationships

- [CleanExecutionReport](CleanExecutionReport.md) (8 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (3 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (2 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [test_wave1_ledger_rollback_and_concurrency_regressions](test_wave1_ledger_rollback_and_concurrency_regressions.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (2 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/tools/cleaner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 135 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*