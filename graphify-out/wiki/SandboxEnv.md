# SandboxEnv

> 46 nodes · cohesion 0.08

## Key Concepts

- **SandboxEnv** (115 connections) — `src/core/backend.rs`
- **.state_read()** (39 connections) — `src/core/backend.rs`
- **normalize_task_path()** (8 connections) — `src/scanner/task_scanner.rs`
- **CachedScanBackend<'a>** (6 connections) — `src/core/backend.rs`
- **.detect_core_parking()** (5 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (5 connections) — `src/core/backend.rs`
- **.load_task_cache()** (5 connections) — `src/core/backend.rs`
- **.detect_gpus()** (4 connections) — `src/core/backend.rs`
- **.list_optional_features()** (4 connections) — `src/core/backend.rs`
- **.list_restore_points()** (4 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (4 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (4 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults_for_tweak()** (4 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (4 connections) — `src/core/backend.rs`
- **.appx()** (3 connections) — `src/core/backend.rs`
- **.gpu_vendors()** (3 connections) — `src/core/backend.rs`
- **.optional_features()** (3 connections) — `src/core/backend.rs`
- **.tasks()** (3 connections) — `src/core/backend.rs`
- **.detect_cpu()** (3 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (3 connections) — `src/core/backend.rs`
- **.list_installed_appx_detailed()** (3 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (3 connections) — `src/core/backend.rs`
- **.list_power_schemes()** (3 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (3 connections) — `src/core/backend.rs`
- *... and 21 more nodes in this community*

## Relationships

- [task_scanner.rs](task_scanner.rs.md) (38 shared connections)
- [.take_injected_error](take_injected_error.md) (31 shared connections)
- [backend.rs](backend.rs.md) (9 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [theme.rs](theme.rs.md) (4 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (3 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (3 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [GpuVendor](GpuVendor.md) (2 shared connections)
- [Command Osstr Process.Rs](Command_Osstr_Process.Rs.md) (2 shared connections)
- [TaskCache](TaskCache.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/task_scanner.rs`

## Audit Trail

- EXTRACTED: 204 (98%)
- INFERRED: 4 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*