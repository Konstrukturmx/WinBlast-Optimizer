# GpuVendor

> 16 nodes · cohesion 0.16

## Key Concepts

- **GpuVendor** (19 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **RollbackRefreshData** (6 connections) — `src/app.rs`
- **.detect_gpus()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (4 connections) — `src/core/backend.rs`
- **.gpu_vendors()** (3 connections) — `src/core/backend.rs`
- **primary_vendor()** (3 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **.is_gpu_vendor_compatible()** (2 connections) — `src/core/risk.rs`
- **Amd** (1 connections) — `src/hardware/gpu.rs`
- **Intel** (1 connections) — `src/hardware/gpu.rs`
- **Nvidia** (1 connections) — `src/hardware/gpu.rs`
- **Unknown** (1 connections) — `src/hardware/gpu.rs`

## Relationships

- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (7 shared connections)
- [AppState](AppState.md) (4 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (1 shared connections)
- [HibernationStatus](HibernationStatus.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 51 (94%)
- INFERRED: 3 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*