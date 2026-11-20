# System Architecture Changelog

> 36 nodes · cohesion 0.09

## Key Concepts

- **tweak_i18n/mod.rs** (42 connections) — `src/core/tweak_i18n/mod.rs`
- **lookup_text_table()** (6 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_category()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_optional_feature()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_sub_category()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_winget()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **lookup_pair_table()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **non_english_idx()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **test_winget_optional_features_and_telemetry_services()** (5 connections) — `src/core/tweak_i18n/mod.rs`
- **is_privacy_or_telemetry_category()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_ledger_token()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_power_scheme_name()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **localize_telemetry_service()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **test_all_10_gaming_latency_ultra_tweaks_localized_all_7_languages()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **test_all_32_deep_opt_tweaks_localized_all_7_languages()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **test_categories_and_sub_categories()** (4 connections) — `src/core/tweak_i18n/mod.rs`
- **test_power_schemes_and_ledger_tokens()** (3 connections) — `src/core/tweak_i18n/mod.rs`
- **tweak_description()** (3 connections) — `src/core/tweak_i18n/mod.rs`
- **tweak_title()** (3 connections) — `src/core/tweak_i18n/mod.rs`
- **.tweak_category()** (2 connections) — `src/app.rs`
- **.tweak_sub_category()** (2 connections) — `src/app.rs`
- **contains_ascii_ci()** (2 connections) — `src/core/tweak_i18n/mod.rs`
- **NON_ENGLISH_LANGS** (2 connections) — `src/core/tweak_i18n/mod.rs`
- **CAT_BROWSERS** (1 connections) — `src/core/tweak_i18n/mod.rs`
- **CAT_DEVELOPMENT** (1 connections) — `src/core/tweak_i18n/mod.rs`
- *... and 11 more nodes in this community*

## Relationships

- [ps_runner.rs](ps_runner.rs.md) (12 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (5 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (2 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [features_apps.rs](features_apps.rs.md) (1 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/tweak_i18n/mod.rs`

## Audit Trail

- EXTRACTED: 83 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*