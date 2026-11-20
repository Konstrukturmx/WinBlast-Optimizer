# parse_guid

> 28 nodes · cohesion 0.19

## Key Concepts

- **TweakDefinition** (51 connections) — `src/core/risk.rs`
- **scanner/mod.rs** (38 connections) — `src/scanner/mod.rs`
- **ScanResult** (28 connections) — `src/scanner/mod.rs`
- **scan_single_tweak_with()** (16 connections) — `src/scanner/mod.rs`
- **scan_all_tweaks_parallel_with()** (12 connections) — `src/scanner/mod.rs`
- **CachedScanBackend** (10 connections) — `src/core/backend.rs`
- **evaluate_tweak()** (10 connections) — `src/scanner/mod.rs`
- **run_full_scan_with()** (9 connections) — `src/scanner/mod.rs`
- **scan_tweaks_chunked_parallel_with()** (7 connections) — `src/scanner/mod.rs`
- **revert_tweaks_by_id_with()** (6 connections) — `src/executor/engine.rs`
- **evaluate_tweak_status()** (6 connections) — `src/scanner/mod.rs`
- **make_scan_result()** (6 connections) — `src/scanner/mod.rs`
- **test_parallel_scan_parity_with_sequential()** (5 connections) — `src/scanner/mod.rs`
- **OsRequirement** (4 connections) — `src/core/risk.rs`
- **revert_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
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
- *... and 3 more nodes in this community*

## Relationships

- [RealBackend](RealBackend.md) (14 shared connections)
- [super](super.md) (13 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (10 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (9 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (8 shared connections)
- [DriverError](DriverError.md) (8 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (8 shared connections)
- [AppState](AppState.md) (7 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (4 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (3 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (3 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/executor/engine.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 169 (92%)
- INFERRED: 14 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*