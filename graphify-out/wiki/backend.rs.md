# backend.rs

> 25 nodes · cohesion 0.10

## Key Concepts

- **backend.rs** (77 connections) — `src/core/backend.rs`
- **LiveServiceInfo** (14 connections) — `src/tools/startup.rs`
- **CpuInfo** (13 connections) — `src/hardware/cpu.rs`
- **MetricsSampler** (11 connections) — `src/core/backend.rs`
- **SandboxMetricsSampler** (5 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_cpu()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **RwLock** (3 connections)
- **.create_metrics_sampler()** (3 connections) — `src/core/backend.rs`
- **.sample()** (3 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_cpu()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_cpu()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **OnceLock** (2 connections)
- **.sample()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **.detect_cpu()** (2 connections) — `src/core/backend.rs`
- **.list_live_services()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **.detect_cpu()** (2 connections) — `src/core/backend.rs`
- **.list_live_services()** (2 connections) — `src/core/backend.rs`
- **SystemMonitor** (2 connections) — `src/core/backend.rs`
- **.sample()** (2 connections) — `src/core/backend.rs`
- **normalize_task_path** (1 connections)

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (7 shared connections)
- [SystemMetrics](SystemMetrics.md) (5 shared connections)
- [SandboxEnv](SandboxEnv.md) (5 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (5 shared connections)
- [Error.Rs Winblasterror Appx](Error.Rs_Winblasterror_Appx.md) (4 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (4 shared connections)
- [.take_injected_error](take_injected_error.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (4 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (3 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/cpu.rs`
- `src/tools/startup.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 140 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*