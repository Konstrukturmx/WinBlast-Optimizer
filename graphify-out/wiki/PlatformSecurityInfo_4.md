# PlatformSecurityInfo

> 10 nodes · cohesion 0.31

## Key Concepts

- **HibernationStatus** (22 connections) — `src/hardware/power.rs`
- **query_hibernation_and_standby()** (4 connections) — `src/hardware/power.rs`
- **set_hibernation_enabled()** (4 connections) — `src/hardware/power.rs`
- **.query_hibernation_status()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (3 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (3 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (2 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (2 connections) — `src/core/backend.rs`

## Relationships

- [registry_scanner.rs](registry_scanner.rs.md) (3 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [driver_update.rs](driver_update.rs.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 36 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*