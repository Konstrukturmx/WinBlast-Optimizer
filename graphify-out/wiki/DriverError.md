# DriverError

> 28 nodes · cohesion 0.08

## Key Concepts

- **TweakAction** (34 connections) — `src/core/risk.rs`
- **evaluate_single_action()** (10 connections) — `src/scanner/mod.rs`
- **PowerTargetScope** (7 connections) — `src/core/risk.rs`
- **resolve_dynamic_dword()** (7 connections) — `src/scanner/mod.rs`
- **DynamicDwordFormula** (6 connections) — `src/core/risk.rs`
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

- [parse_guid](parse_guid.md) (8 shared connections)
- [RealBackend](RealBackend.md) (6 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (4 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (3 shared connections)
- [super](super.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/scanner/mod.rs`

## Audit Trail

- EXTRACTED: 63 (95%)
- INFERRED: 3 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*