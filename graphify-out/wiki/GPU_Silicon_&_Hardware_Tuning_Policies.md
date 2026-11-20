# GPU Silicon & Hardware Tuning Policies

> 47 nodes · cohesion 0.08

## Key Concepts

- **SandboxEnv** (119 connections) — `src/core/backend.rs`
- **.state_read()** (39 connections) — `src/core/backend.rs`
- **CachedScanBackend<'a>** (6 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (5 connections) — `src/core/backend.rs`
- **.load_task_cache()** (5 connections) — `src/core/backend.rs`
- **.detect_gpus()** (4 connections) — `src/core/backend.rs`
- **.list_optional_features()** (4 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (4 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (4 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults_for_tweak()** (4 connections) — `src/core/backend.rs`
- **.appx()** (3 connections) — `src/core/backend.rs`
- **.gpu_vendors()** (3 connections) — `src/core/backend.rs`
- **.optional_features()** (3 connections) — `src/core/backend.rs`
- **.tasks()** (3 connections) — `src/core/backend.rs`
- **.capture_latency()** (3 connections) — `src/core/backend.rs`
- **.detect_cpu()** (3 connections) — `src/core/backend.rs`
- **.detect_os()** (3 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (3 connections) — `src/core/backend.rs`
- **.list_installed_appx_detailed()** (3 connections) — `src/core/backend.rs`
- **.list_live_services()** (3 connections) — `src/core/backend.rs`
- **.list_live_tasks()** (3 connections) — `src/core/backend.rs`
- **.list_power_schemes()** (3 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (3 connections) — `src/core/backend.rs`
- *... and 22 more nodes in this community*

## Relationships

- [RegRawSnapshot](RegRawSnapshot.md) (32 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (32 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (7 shared connections)
- [.set_status_spec](set_status_spec.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (3 shared connections)
- [TaskCache](TaskCache.md) (3 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [parse_guid](parse_guid.md) (3 shared connections)
- [AppxCache](AppxCache.md) (2 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 201 (99%)
- INFERRED: 2 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*