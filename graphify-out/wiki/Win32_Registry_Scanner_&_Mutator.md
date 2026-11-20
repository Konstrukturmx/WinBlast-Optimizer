# Win32 Registry Scanner & Mutator

> 34 nodes · cohesion 0.15

## Key Concepts

- **TweakDefinition** (53 connections) — `src/core/risk.rs`
- **scanner/mod.rs** (38 connections) — `src/scanner/mod.rs`
- **ScanResult** (28 connections) — `src/scanner/mod.rs`
- **scan_single_tweak_with()** (18 connections) — `src/scanner/mod.rs`
- **TweakState** (18 connections) — `src/scanner/mod.rs`
- **scan_all_tweaks_parallel_with()** (12 connections) — `src/scanner/mod.rs`
- **CachedScanBackend** (10 connections) — `src/core/backend.rs`
- **evaluate_single_action()** (10 connections) — `src/scanner/mod.rs`
- **evaluate_tweak()** (10 connections) — `src/scanner/mod.rs`
- **run_full_scan_with()** (9 connections) — `src/scanner/mod.rs`
- **scan_tweaks_chunked_parallel_with()** (7 connections) — `src/scanner/mod.rs`
- **ActionEval** (6 connections) — `src/scanner/mod.rs`
- **evaluate_tweak_status()** (6 connections) — `src/scanner/mod.rs`
- **make_scan_result()** (6 connections) — `src/scanner/mod.rs`
- **test_parallel_scan_parity_with_sequential()** (5 connections) — `src/scanner/mod.rs`
- **mk_test_tweak()** (4 connections) — `src/scanner/mod.rs`
- **run_full_scan()** (4 connections) — `src/scanner/mod.rs`
- **scan_all_tweaks_parallel()** (4 connections) — `src/scanner/mod.rs`
- **scan_single_tweak()** (4 connections) — `src/scanner/mod.rs`
- **scan_tweaks_chunked_parallel()** (4 connections) — `src/scanner/mod.rs`
- **test_foreign_gpu_and_non_elevated_feature_and_partial_power_scan()** (4 connections) — `src/scanner/mod.rs`
- **test_evaluate_tweak_status_single()** (3 connections) — `src/scanner/mod.rs`
- **test_full_scan_execution_in_sandbox()** (3 connections) — `src/scanner/mod.rs`
- **test_run_full_scan_populates_detail_kind_and_sub_category()** (3 connections) — `src/scanner/mod.rs`
- **test_scan_single_tweak_detail_kind_every_action_and_na_branch()** (3 connections) — `src/scanner/mod.rs`
- *... and 9 more nodes in this community*

## Relationships

- [theme.rs](theme.rs.md) (18 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (14 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (10 shared connections)
- [startup.rs](startup.rs.md) (8 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (7 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (6 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (6 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (6 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (6 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (4 shared connections)
- [deserialize](deserialize.md) (3 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 187 (93%)
- INFERRED: 15 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*