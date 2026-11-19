# RegRawSnapshot

> 24 nodes · cohesion 0.13

## Key Concepts

- **FeatureInstallState** (26 connections) — `src/core/risk.rs`
- **OptionalFeatureKind** (21 connections) — `src/core/risk.rs`
- **WindowsOptionalFeature** (18 connections) — `src/tools/features_apps.rs`
- **dism_error_from_output()** (7 connections) — `src/tools/features_apps.rs`
- **query_dism_capabilities_locked()** (7 connections) — `src/tools/features_apps.rs`
- **curated_windows_features_meta()** (6 connections) — `src/tools/features_apps.rs`
- **query_dism_features_locked()** (6 connections) — `src/tools/features_apps.rs`
- **set_optional_feature()** (6 connections) — `src/tools/features_apps.rs`
- **list_optional_features()** (5 connections) — `src/tools/features_apps.rs`
- **.list_optional_features()** (4 connections) — `src/core/backend.rs`
- **parse_dism_english_state()** (4 connections) — `src/tools/features_apps.rs`
- **resolve_capability_identity_locked()** (4 connections) — `src/tools/features_apps.rs`
- **.optional_features()** (3 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (3 connections) — `src/core/backend.rs`
- **.set_optional_feature()** (3 connections) — `src/core/backend.rs`
- **.list_optional_features()** (2 connections) — `src/core/backend.rs`
- **Disabled** (1 connections) — `src/core/risk.rs`
- **DisablePending** (1 connections) — `src/core/risk.rs`
- **Enabled** (1 connections) — `src/core/risk.rs`
- **EnablePending** (1 connections) — `src/core/risk.rs`
- **Removed** (1 connections) — `src/core/risk.rs`
- **Capability** (1 connections) — `src/core/risk.rs`
- **OptionalFeature** (1 connections) — `src/core/risk.rs`
- **Output** (1 connections)

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (12 shared connections)
- [AppState](AppState.md) (4 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [theme.rs](theme.rs.md) (3 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [Command Osstr Process.Rs](Command_Osstr_Process.Rs.md) (3 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [.take_injected_error](take_injected_error.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/tools/features_apps.rs`

## Audit Trail

- EXTRACTED: 95 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*