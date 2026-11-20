# LazyLoader

> 42 nodes · cohesion 0.08

## Key Concepts

- **lazy_loader.rs** (26 connections) — `src/hardware/lazy_loader.rs`
- **LazyLoader** (19 connections) — `src/hardware/lazy_loader.rs`
- **LazyPayload** (12 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe()** (10 connections) — `src/hardware/lazy_loader.rs`
- **arc** (8 connections)
- **BiosSystemInfo** (8 connections) — `src/hardware/bios.rs`
- **background_probe_delivers_payload_without_blocking()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.is_any_loading()** (5 connections) — `src/hardware/lazy_loader.rs`
- **.new()** (5 connections) — `src/hardware/lazy_loader.rs`
- **background_panic_is_contained_and_clears_in_flight_flag()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_drivers()** (4 connections) — `src/hardware/lazy_loader.rs`
- **.try_recv()** (4 connections) — `src/hardware/lazy_loader.rs`
- **Any** (3 connections)
- **Receiver** (3 connections)
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **extract_panic_message()** (3 connections) — `src/hardware/lazy_loader.rs`
- **InFlightGuard** (3 connections) — `src/hardware/lazy_loader.rs`
- **.drain_ready()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.is_drivers_loading()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_bios()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_disks()** (3 connections) — `src/hardware/lazy_loader.rs`
- **.spawn_probe_thermals()** (3 connections) — `src/hardware/lazy_loader.rs`
- **Drivers** (3 connections) — `src/hardware/lazy_loader.rs`
- **thread** (3 connections)
- **DisplayDriverInfo** (2 connections) — `src/hardware/bios.rs`
- *... and 17 more nodes in this community*

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (5 shared connections)
- [super](super.md) (4 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (3 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (3 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [generational_cache.rs](generational_cache.rs.md) (2 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/hardware/bios.rs`
- `src/hardware/lazy_loader.rs`

## Audit Trail

- EXTRACTED: 101 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*