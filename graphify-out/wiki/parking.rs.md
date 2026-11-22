# parking.rs

> 30 nodes · cohesion 0.12

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (10 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (9 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_target_paths()** (7 connections) — `src/scanner/registry_scanner.rs`
- **parse_target_vendor()** (6 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **DevInfoGuard** (4 connections) — `src/hardware/gpu.rs`
- **.as_str()** (4 connections) — `src/hardware/gpu.rs`
- **present_display_class_subkeys()** (4 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (3 connections) — `src/core/backend.rs`
- **primary_vendor()** (3 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **test_gpu_vendor_compatibility()** (2 connections) — `src/core/risk.rs`
- **detect_dxgi_vram_table()** (2 connections) — `src/hardware/gpu.rs`
- **has_ascii_token()** (2 connections) — `src/hardware/gpu.rs`
- **parse_ven_dev()** (2 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path()** (2 connections) — `src/hardware/gpu.rs`
- **test_detect_vendor_no_false_positive_ati()** (2 connections) — `src/hardware/gpu.rs`
- **HDEVINFO** (1 connections)
- **.drop()** (1 connections) — `src/hardware/gpu.rs`
- *... and 5 more nodes in this community*

## Relationships

- [driver_update.rs](driver_update.rs.md) (6 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (3 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [AppxCache](AppxCache.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `src/scanner/registry_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 84 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*