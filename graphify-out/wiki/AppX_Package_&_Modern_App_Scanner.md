# AppX Package & Modern App Scanner

> 28 nodes · cohesion 0.08

## Key Concepts

- **TweakAction** (33 connections) — `src/core/risk.rs`
- **evaluate_single_action()** (10 connections) — `src/scanner/mod.rs`
- **PowerTargetScope** (8 connections) — `src/core/risk.rs`
- **DynamicDwordFormula** (7 connections) — `src/core/risk.rs`
- **resolve_dynamic_dword()** (7 connections) — `src/scanner/mod.rs`
- **ActionEval** (6 connections) — `src/scanner/mod.rs`
- **.seed_tweak_defaults()** (3 connections) — `src/core/backend.rs`
- **FeatureAuthRequired** (2 connections) — `src/scanner/mod.rs`
- **LogicalCoreCount** (1 connections) — `src/core/risk.rs`
- **TotalRamInKb** (1 connections) — `src/core/risk.rs`
- **TotalRamInKbPlusMargin** (1 connections) — `src/core/risk.rs`
- **AcAndDc** (1 connections) — `src/core/risk.rs`
- **AcOnly** (1 connections) — `src/core/risk.rs`
- **DcOnly** (1 connections) — `src/core/risk.rs`
- **AppXPackage** (1 connections) — `src/core/risk.rs`
- **CompositeStringKey** (1 connections) — `src/core/risk.rs`
- **PowerCfgSetting** (1 connections) — `src/core/risk.rs`
- **RegistryBinaryBit** (1 connections) — `src/core/risk.rs`
- **RegistryDelete** (1 connections) — `src/core/risk.rs`
- **RegistryDeleteTree** (1 connections) — `src/core/risk.rs`
- **RegistryDword** (1 connections) — `src/core/risk.rs`
- **RegistryDynamicDword** (1 connections) — `src/core/risk.rs`
- **RegistryString** (1 connections) — `src/core/risk.rs`
- **ScheduledTask** (1 connections) — `src/core/risk.rs`
- **ServiceState** (1 connections) — `src/core/risk.rs`
- *... and 3 more nodes in this community*

## Relationships

- [Embedded Catalog Test](Embedded_Catalog_Test.md) (8 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (7 shared connections)
- [theme.rs](theme.rs.md) (4 shared connections)
- [ThemePalette](ThemePalette.md) (4 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 63 (94%)
- INFERRED: 4 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*