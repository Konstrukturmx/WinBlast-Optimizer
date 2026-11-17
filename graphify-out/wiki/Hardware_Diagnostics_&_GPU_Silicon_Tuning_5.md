# Hardware Diagnostics & GPU Silicon Tuning

> 24 nodes · cohesion 0.19

## Key Concepts

- **TweakDefinition** (46 connections) — `src/core/risk.rs`
- **scanner/mod.rs** (30 connections) — `src/scanner/mod.rs`
- **ScanResult** (23 connections) — `src/scanner/mod.rs`
- **scan_single_tweak_with()** (16 connections) — `src/scanner/mod.rs`
- **run_full_scan_with()** (12 connections) — `src/scanner/mod.rs`
- **CachedScanBackend** (10 connections) — `src/core/backend.rs`
- **evaluate_single_action()** (8 connections) — `src/scanner/mod.rs`
- **evaluate_tweak()** (8 connections) — `src/scanner/mod.rs`
- **ActionEval** (6 connections) — `src/scanner/mod.rs`
- **make_scan_result()** (6 connections) — `src/scanner/mod.rs`
- **resolve_dynamic_dword()** (6 connections) — `src/scanner/mod.rs`
- **mk_test_tweak()** (4 connections) — `src/scanner/mod.rs`
- **run_full_scan()** (4 connections) — `src/scanner/mod.rs`
- **scan_single_tweak()** (4 connections) — `src/scanner/mod.rs`
- **test_foreign_gpu_and_non_elevated_feature_and_partial_power_scan()** (4 connections) — `src/scanner/mod.rs`
- **test_full_scan_execution_in_sandbox()** (3 connections) — `src/scanner/mod.rs`
- **test_run_full_scan_populates_detail_kind_and_sub_category()** (3 connections) — `src/scanner/mod.rs`
- **test_scan_single_tweak_detail_kind_every_action_and_na_branch()** (3 connections) — `src/scanner/mod.rs`
- **test_scan_single_tweak_in_sandbox()** (3 connections) — `src/scanner/mod.rs`
- **OnceLock** (2 connections)
- **.scan_detail_text()** (2 connections) — `src/app.rs`
- **FeatureAuthRequired** (2 connections) — `src/scanner/mod.rs`
- **Evaluated** (1 connections) — `src/scanner/mod.rs`
- **SkippedPower** (1 connections) — `src/scanner/mod.rs`

## Relationships

- [engine.rs](engine.rs.md) (12 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (11 shared connections)
- [action_button](action_button.md) (9 shared connections)
- [AppState](AppState.md) (8 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (7 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (5 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (5 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (5 shared connections)
- [app.rs](app.rs.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [Windows OS Settings & UX Customization](Windows_OS_Settings_&_UX_Customization.md) (4 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 140 (91%)
- INFERRED: 14 (9%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*