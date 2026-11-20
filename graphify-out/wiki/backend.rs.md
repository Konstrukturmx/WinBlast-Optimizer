# backend.rs

> 17 nodes · cohesion 0.12

## Key Concepts

- **real_windows_env.rs** (33 connections) — `tests/real_windows_env.rs`
- **uuid** (4 connections)
- **TempTestEnvGuard** (3 connections) — `tests/real_windows_env.rs`
- **test_real_backend_readonly_live_host_audit()** (3 connections) — `tests/real_windows_env.rs`
- **gpuvendor** (2 connections)
- **regvalue** (2 connections)
- **HkcuTestEnvGuard** (2 connections) — `tests/real_windows_env.rs`
- **.new()** (2 connections) — `tests/real_windows_env.rs`
- **test_ntfs_junction_and_readonly_cleanup_sandbox()** (2 connections) — `tests/real_windows_env.rs`
- **tweakstate** (2 connections)
- **is_known_numeric_reg_sz** (1 connections)
- **realbackend** (1 connections)
- **run_full_scan_with** (1 connections)
- **FILE_ATTRIBUTE_REPARSE_POINT** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **test_real_windows_disk_and_thermal_telemetry()** (1 connections) — `tests/real_windows_env.rs`

## Relationships

- [super](super.md) (4 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (2 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [.take_injected_error](take_injected_error.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [DriverUpdateState](DriverUpdateState.md) (1 shared connections)
- [enums](enums.md) (1 shared connections)
- [DriverError](DriverError.md) (1 shared connections)
- [updater.rs](updater.rs.md) (1 shared connections)

## Source Files

- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 43 (96%)
- INFERRED: 2 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*