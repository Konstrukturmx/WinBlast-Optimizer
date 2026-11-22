# WinBlastError

> 40 nodes · cohesion 0.09

## Key Concepts

- **SandboxEnv** (126 connections) — `src/core/backend.rs`
- **.state_read()** (39 connections) — `src/core/backend.rs`
- **.reg_get_dword_for_vendor()** (6 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (5 connections) — `src/core/backend.rs`
- **.detect_gpus()** (4 connections) — `src/core/backend.rs`
- **.list_optional_features()** (4 connections) — `src/core/backend.rs`
- **.list_restore_points()** (4 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (4 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults_for_tweak()** (4 connections) — `src/core/backend.rs`
- **.capture_latency()** (3 connections) — `src/core/backend.rs`
- **.detect_cpu()** (3 connections) — `src/core/backend.rs`
- **.detect_os()** (3 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (3 connections) — `src/core/backend.rs`
- **.list_live_services()** (3 connections) — `src/core/backend.rs`
- **.list_power_schemes()** (3 connections) — `src/core/backend.rs`
- **.list_startup_entries()** (3 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (3 connections) — `src/core/backend.rs`
- **.recorded_ops()** (3 connections) — `src/core/backend.rs`
- **.reg_get_binary()** (3 connections) — `src/core/backend.rs`
- **.reg_get_dword()** (3 connections) — `src/core/backend.rs`
- **.run_latency_benchmark()** (3 connections) — `src/core/backend.rs`
- **.scan_clean_targets()** (3 connections) — `src/core/backend.rs`
- **.seed_all_tweaks()** (3 connections) — `src/core/backend.rs`
- **.uninstall_onedrive_safe()** (3 connections) — `src/core/backend.rs`
- *... and 15 more nodes in this community*

## Relationships

- [PlatformSecurityInfo](PlatformSecurityInfo.md) (32 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (31 shared connections)
- [CachedScanBackend<'a>](CachedScanBackend_'a_.md) (8 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (4 shared connections)
- [backend.rs](backend.rs.md) (3 shared connections)
- [.new](new.md) (3 shared connections)
- [restore_point.rs](restore_point.rs.md) (3 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (3 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (3 shared connections)
- [HibernationStatus](HibernationStatus.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 200 (99%)
- INFERRED: 2 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*