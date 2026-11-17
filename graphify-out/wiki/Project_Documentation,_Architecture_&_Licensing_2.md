# Project Documentation, Architecture & Licensing

> 13 nodes · cohesion 0.21

## Key Concepts

- **LatencySnapshot** (18 connections) — `src/hardware/latency.rs`
- **latency.rs** (4 connections) — `src/hardware/latency.rs`
- **System** (4 connections)
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

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/latency.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 32 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*