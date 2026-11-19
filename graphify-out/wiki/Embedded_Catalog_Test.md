# Embedded Catalog Test

> 25 nodes · cohesion 0.23

## Key Concepts

- **TweakDefinition** (51 connections) — `src/core/risk.rs`
- **scanner/mod.rs** (38 connections) — `src/scanner/mod.rs`
- **ScanResult** (28 connections) — `src/scanner/mod.rs`
- **scan_single_tweak_with()** (17 connections) — `src/scanner/mod.rs`
- **RiskValuation** (13 connections) — `src/core/risk.rs`
- **scan_all_tweaks_parallel_with()** (12 connections) — `src/scanner/mod.rs`
- **CachedScanBackend** (10 connections) — `src/core/backend.rs`
- **evaluate_tweak()** (10 connections) — `src/scanner/mod.rs`
- **run_full_scan_with()** (9 connections) — `src/scanner/mod.rs`
- **scan_tweaks_chunked_parallel_with()** (7 connections) — `src/scanner/mod.rs`
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
- **.scan_detail_text()** (2 connections) — `src/app.rs`

## Relationships

- [action_button_guarded](action_button_guarded.md) (15 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (14 shared connections)
- [theme.rs](theme.rs.md) (14 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (8 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (8 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (7 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (6 shared connections)
- [app.rs](app.rs.md) (5 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (4 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (3 shared connections)
- [SandboxEnv](SandboxEnv.md) (3 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 169 (92%)
- INFERRED: 14 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*