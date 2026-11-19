# Multi-Language Tweak Localization Tables

> 22 nodes · cohesion 0.15

## Key Concepts

- **gpu.rs** (27 connections) — `src/hardware/gpu.rs`
- **detect_gpus()** (7 connections) — `src/hardware/gpu.rs`
- **detect_vendor()** (7 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path_for_vendor()** (6 connections) — `src/hardware/gpu.rs`
- **DevInfoGuard** (4 connections) — `src/hardware/gpu.rs`
- **.as_str()** (4 connections) — `src/hardware/gpu.rs`
- **parse_target_vendor()** (4 connections) — `src/hardware/gpu.rs`
- **present_display_class_subkeys()** (4 connections) — `src/hardware/gpu.rs`
- **test_gpu_vendor_compatibility()** (2 connections) — `src/core/risk.rs`
- **detect_dxgi_vram_table()** (2 connections) — `src/hardware/gpu.rs`
- **has_ascii_token()** (2 connections) — `src/hardware/gpu.rs`
- **parse_ven_dev()** (2 connections) — `src/hardware/gpu.rs`
- **resolve_gpu_class_path()** (2 connections) — `src/hardware/gpu.rs`
- **test_detect_vendor_no_false_positive_ati()** (2 connections) — `src/hardware/gpu.rs`
- **HDEVINFO** (1 connections)
- **.drop()** (1 connections) — `src/hardware/gpu.rs`
- **DISPLAY_CLASS_PREFIX** (1 connections) — `src/hardware/gpu.rs`
- **is_hags_enabled()** (1 connections) — `src/hardware/gpu.rs`
- **is_mpo_disabled()** (1 connections) — `src/hardware/gpu.rs`
- **set_gpu_msi_mode()** (1 connections) — `src/hardware/gpu.rs`
- **set_hags()** (1 connections) — `src/hardware/gpu.rs`
- **set_mpo_disabled()** (1 connections) — `src/hardware/gpu.rs`

## Relationships

- [GpuVendor](GpuVendor.md) (7 shared connections)
- [deserialize](deserialize.md) (4 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (1 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (1 shared connections)

## Source Files

- `src/core/risk.rs`
- `src/hardware/gpu.rs`

## Audit Trail

- EXTRACTED: 47 (96%)
- INFERRED: 2 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*