# Integration Suite: Tweak Sandbox E2E

> 14 nodes · cohesion 0.18

## Key Concepts

- **SystemMetrics** (14 connections) — `src/hardware/monitor.rs`
- **MetricsSampler** (11 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.sample()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **SystemMonitor** (2 connections) — `src/core/backend.rs`
- **.sample()** (2 connections) — `src/core/backend.rs`
- **DateTime** (2 connections)
- **Utc** (2 connections)
- **.ram_display()** (1 connections) — `src/hardware/monitor.rs`
- **.timer_res_ms_display()** (1 connections) — `src/hardware/monitor.rs`

## Relationships

- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/monitor.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 35 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*