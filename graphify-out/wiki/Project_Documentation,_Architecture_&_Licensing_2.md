# Project Documentation, Architecture & Licensing

> 12 nodes · cohesion 0.23

## Key Concepts

- **LatencySnapshot** (18 connections) — `src/hardware/latency.rs`
- **latency.rs** (4 connections) — `src/hardware/latency.rs`
- **.capture_latency()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.capture_latency()** (2 connections) — `src/core/backend.rs`
- **.capture_latency()** (2 connections) — `src/core/backend.rs`
- **.capture_latency()** (2 connections) — `src/core/backend.rs`
- **DateTime** (2 connections)
- **Utc** (2 connections)
- **.ram_display()** (1 connections) — `src/hardware/latency.rs`
- **.timer_resolution_display()** (1 connections) — `src/hardware/latency.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Windows Unattend Deployment Generator](Windows_Unattend_Deployment_Generator.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 29 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*