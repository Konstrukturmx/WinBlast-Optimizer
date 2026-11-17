# Hardware Diagnostics & GPU Silicon Tuning

> 15 nodes · cohesion 0.21

## Key Concepts

- **CpuInfo** (13 connections) — `src/hardware/cpu.rs`
- **cpu.rs** (9 connections) — `src/hardware/cpu.rs`
- **classify_efficiency_records()** (4 connections) — `src/hardware/cpu.rs`
- **query_win32_core_topology()** (4 connections) — `src/hardware/cpu.rs`
- **.detect_cpu()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect()** (3 connections) — `src/hardware/cpu.rs`
- **Win32CoreTopology** (3 connections) — `src/hardware/cpu.rs`
- **.detect_cpu()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_cpu()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_cpu()** (2 connections) — `src/core/backend.rs`
- **.detect_cpu()** (2 connections) — `src/core/backend.rs`
- **.detect_cpu()** (2 connections) — `src/core/backend.rs`
- **test_classify_efficiency_records_hybrid_and_non_ht_and_uniform()** (2 connections) — `src/hardware/cpu.rs`
- **test_cpu_detect_host_safe()** (2 connections) — `src/hardware/cpu.rs`
- **cpuid** (1 connections)

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/cpu.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 35 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*