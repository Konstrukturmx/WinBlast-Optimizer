# Integration Suite: Tweak Sandbox E2E

> 37 nodes · cohesion 0.09

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (19 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (7 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (6 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **DevInfoGuard** (4 connections) — `src/hardware/gpu.rs`
- **.as_str()** (4 connections) — `src/hardware/gpu.rs`
- **parse_target_vendor()** (4 connections) — `src/hardware/gpu.rs`
- **present_display_class_subkeys()** (4 connections) — `src/hardware/gpu.rs`
- **.gpu_vendors()** (3 connections) — `src/core/backend.rs`
- **.detect_gpus()** (3 connections) — `src/core/backend.rs`
- **primary_vendor()** (3 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **test_gpu_vendor_compatibility()** (2 connections) — `src/core/risk.rs`
- **.is_gpu_vendor_compatible()** (2 connections) — `src/core/risk.rs`
- **detect_dxgi_vram_table()** (2 connections) — `src/hardware/gpu.rs`
- **has_ascii_token()** (2 connections) — `src/hardware/gpu.rs`
- **parse_ven_dev()** (2 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path()** (2 connections) — `src/hardware/gpu.rs`
- **test_detect_vendor_no_false_positive_ati()** (2 connections) — `src/hardware/gpu.rs`
- *... and 12 more nodes in this community*

## Relationships

- [backend.rs](backend.rs.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [enums](enums.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [engine.rs](engine.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 87 (97%)
- INFERRED: 3 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*