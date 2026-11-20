# reg_mutator.rs

> 35 nodes · cohesion 0.09

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **DetectedGpu** (17 connections) — `src/hardware/gpu.rs`
- **GpuVendor** (15 connections) — `src/hardware/gpu.rs`
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

- [deserialize](deserialize.md) (3 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (2 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/risk.rs`
- `src/hardware/gpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 80 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*