# Integration Suite: Tweak Sandbox E2E

> 42 nodes · cohesion 0.08

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (19 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (8 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (6 connections) — `src/hardware/gpu.rs`
- **parse_target_vendor()** (5 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **DevInfoGuard** (4 connections) — `src/hardware/gpu.rs`
- **.as_str()** (4 connections) — `src/hardware/gpu.rs`
- **present_display_class_subkeys()** (4 connections) — `src/hardware/gpu.rs`
- **.gpu_vendors()** (3 connections) — `src/core/backend.rs`
- **.detect_gpus()** (3 connections) — `src/core/backend.rs`
- **.detect_gpus()** (3 connections) — `src/core/backend.rs`
- **primary_vendor()** (3 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_gpus()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_hags_enabled()** (2 connections) — `src/core/backend.rs`
- **.is_mpo_disabled()** (2 connections) — `src/core/backend.rs`
- **.set_gpu_msi_mode()** (2 connections) — `src/core/backend.rs`
- **.set_hags()** (2 connections) — `src/core/backend.rs`
- **.set_mpo_disabled()** (2 connections) — `src/core/backend.rs`
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **test_gpu_vendor_compatibility()** (2 connections) — `src/core/risk.rs`
- **.is_gpu_vendor_compatible()** (2 connections) — `src/core/risk.rs`
- *... and 17 more nodes in this community*

## Relationships

- [RealBackend](RealBackend.md) (6 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [enums](enums.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 99 (97%)
- INFERRED: 3 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*