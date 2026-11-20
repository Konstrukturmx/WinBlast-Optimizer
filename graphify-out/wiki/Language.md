# Language

> 11 nodes · cohesion 0.25

## Key Concepts

- **DriverUpdateItem** (16 connections) — `src/hardware/driver_update.rs`
- **download_and_install_driver()** (7 connections) — `src/hardware/driver_update.rs`
- **scan_online_drivers_with_progress()** (7 connections) — `src/hardware/driver_update.rs`
- **create_sandbox_test_item()** (6 connections) — `src/hardware/driver_update.rs`
- **scan_online_drivers()** (5 connections) — `src/hardware/driver_update.rs`
- **test_driver_sandbox_lifecycle()** (4 connections) — `src/hardware/driver_update.rs`
- **.test_sandbox_driver_install()** (3 connections) — `src/app.rs`
- **.install_driver_update()** (2 connections) — `src/app.rs`
- **test_create_sandbox_item_metadata()** (2 connections) — `src/hardware/driver_update.rs`
- **test_driver_sandbox_lifecycle_execution()** (2 connections) — `src/hardware/driver_update.rs`
- **.is_update_available()** (1 connections) — `src/hardware/driver_update.rs`

## Relationships

- [task_scanner.rs](task_scanner.rs.md) (8 shared connections)
- [TaskCache](TaskCache.md) (6 shared connections)
- [AppState](AppState.md) (3 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (3 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (2 shared connections)
- [PdhParkingCollector](PdhParkingCollector.md) (1 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [GameProfile](GameProfile.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/hardware/driver_update.rs`

## Audit Trail

- EXTRACTED: 39 (95%)
- INFERRED: 2 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*