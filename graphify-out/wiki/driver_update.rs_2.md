# driver_update.rs

> 38 nodes · cohesion 0.09

## Key Concepts

- **lazy_loader.rs** (26 connections) — `src/hardware/lazy_loader.rs`
- **LazyLoader** (19 connections) — `src/hardware/lazy_loader.rs`
- **LazyPayload** (12 connections) — `src/hardware/lazy_loader.rs`
- **arc** (10 connections)
- **.spawn_probe()** (10 connections) — `src/hardware/lazy_loader.rs`
- **background_probe_delivers_payload_without_blocking()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.is_any_loading()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.new()** (5 connections) — `src/hardware/lazy_loader.rs`
- **background_panic_is_contained_and_clears_in_flight_flag()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_drivers()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.try_recv()** (4 connections) — `src/hardware/lazy_loader.rs`
- **extract_panic_message()** (3 connections) — `src/hardware/lazy_loader.rs`
- **InFlightGuard** (3 connections) — `src/hardware/lazy_loader.rs`
- **.drain_ready()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.is_drivers_loading()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_bios()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_disks()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_thermals()** (3 connections) — `src/hardware/lazy_loader.rs`
- **Drivers** (3 connections) — `src/hardware/lazy_loader.rs`
- **thread** (3 connections)
- **assertunwindsafe** (2 connections)
- **.default()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_bios_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_disks_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_thermals_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- *... and 13 more nodes in this community*

## Relationships

- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [.setup](setup.md) (3 shared connections)
- [adaptive_tuning.rs](adaptive_tuning.rs.md) (3 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (2 shared connections)
- [.maybe_arm_error](maybe_arm_error.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [sysprep_engine.rs](sysprep_engine.rs.md) (2 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [selection_automation_e2e.rs](selection_automation_e2e.rs.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)

## Source Files

- `src/hardware/lazy_loader.rs`

## Audit Trail

- EXTRACTED: 95 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*