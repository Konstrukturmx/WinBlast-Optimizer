# Embedded Catalog Test

> 28 nodes · cohesion 0.20

## Key Concepts

- **TweakDefinition** (51 connections) — `src/core/risk.rs`
- **scanner/mod.rs** (38 connections) — `src/scanner/mod.rs`
- **ScanResult** (28 connections) — `src/scanner/mod.rs`
- **scan_single_tweak_with()** (17 connections) — `src/scanner/mod.rs`
- **scan_all_tweaks_parallel_with()** (11 connections) — `src/scanner/mod.rs`
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
- **test_scan_single_tweak_in_sandbox()** (3 connections) — `src/scanner/mod.rs`
- *... and 3 more nodes in this community*

## Relationships

- [theme.rs](theme.rs.md) (16 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (12 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (8 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (7 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (7 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (6 shared connections)
- [app.rs](app.rs.md) (5 shared connections)
- [AppState](AppState.md) (5 shared connections)
- [action_button](action_button.md) (5 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (4 shared connections)
- [SandboxEnv](SandboxEnv.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 167 (92%)
- INFERRED: 15 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*