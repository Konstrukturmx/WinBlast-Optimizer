# State Ledger & Restore Point Engine

> 24 nodes · cohesion 0.16

## Key Concepts

- **LazyLoader** (19 connections) — `src/hardware/lazy_loader.rs`
- **LazyPayload** (12 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe()** (10 connections) — `src/hardware/lazy_loader.rs`
- **background_probe_delivers_payload_without_blocking()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.is_any_loading()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.new()** (5 connections) — `src/hardware/lazy_loader.rs`
- **background_panic_is_contained_and_clears_in_flight_flag()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_drivers()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.try_recv()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.drain_ready()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.is_drivers_loading()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_bios()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_disks()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_thermals()** (3 connections) — `src/hardware/lazy_loader.rs`
- **Drivers** (3 connections) — `src/hardware/lazy_loader.rs`
- **.default()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_bios_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_disks_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- **.is_thermals_loading()** (2 connections) — `src/hardware/lazy_loader.rs`
- **Bios** (2 connections) — `src/hardware/lazy_loader.rs`
- **Disks** (2 connections) — `src/hardware/lazy_loader.rs`
- **Thermals** (2 connections) — `src/hardware/lazy_loader.rs`
- **try_recv_is_non_blocking_when_empty()** (2 connections) — `src/hardware/lazy_loader.rs`
- **F** (1 connections)

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (8 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (2 shared connections)
- [CoreParkingState](CoreParkingState.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (1 shared connections)

## Source Files

- `src/hardware/lazy_loader.rs`

## Audit Trail

- EXTRACTED: 59 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*