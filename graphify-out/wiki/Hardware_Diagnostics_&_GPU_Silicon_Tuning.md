# Hardware Diagnostics & GPU Silicon Tuning

> 35 nodes · cohesion 0.09

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (17 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (7 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (6 connections) — `src/hardware/gpu.rs`
- **.detect_gpus()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **DevInfoGuard** (4 connections) — `src/hardware/gpu.rs`
- **.as_str()** (4 connections) — `src/hardware/gpu.rs`
- **parse_target_vendor()** (4 connections) — `src/hardware/gpu.rs`
- **present_display_class_subkeys()** (4 connections) — `src/hardware/gpu.rs`
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
- **HDEVINFO** (1 connections)
- **.drop()** (1 connections) — `src/hardware/gpu.rs`
- *... and 10 more nodes in this community*

## Relationships

- [FailAtMutationBackend](FailAtMutationBackend.md) (3 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (3 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [parse_guid](parse_guid.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (2 shared connections)
- [super](super.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 82 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*