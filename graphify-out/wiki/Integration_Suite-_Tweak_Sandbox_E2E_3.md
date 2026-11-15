# Integration Suite: Tweak Sandbox E2E

> 32 nodes · cohesion 0.11

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (19 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (9 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **parse_target_vendor()** (6 connections) — `src/hardware/gpu.rs`
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
- **.detect_gpus()** (2 connections) — `src/core/backend.rs`
- **test_gpu_vendor_compatibility()** (2 connections) — `src/core/risk.rs`
- **.is_gpu_vendor_compatible()** (2 connections) — `src/core/risk.rs`
- **detect_dxgi_vram_table()** (2 connections) — `src/hardware/gpu.rs`
- **has_ascii_token()** (2 connections) — `src/hardware/gpu.rs`
- **parse_ven_dev()** (2 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path()** (2 connections) — `src/hardware/gpu.rs`
- **test_detect_vendor_no_false_positive_ati()** (2 connections) — `src/hardware/gpu.rs`
- *... and 7 more nodes in this community*

## Relationships

- [Scan Test Single](Scan_Test_Single.md) (6 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (4 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 92 (97%)
- INFERRED: 3 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*