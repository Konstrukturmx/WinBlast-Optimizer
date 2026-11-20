# task_scanner.rs

> 21 nodes · cohesion 0.16

## Key Concepts

- **driver_update.rs** (50 connections) — `src/hardware/driver_update.rs`
- **parse_pnputil_devices()** (5 connections) — `src/hardware/driver_update.rs`
- **PnpDriverPackage** (5 connections) — `src/hardware/driver_update.rs`
- **parse_pnputil_drivers()** (4 connections) — `src/hardware/driver_update.rs`
- **PnpDeviceInfo** (4 connections) — `src/hardware/driver_update.rs`
- **vendor_download_url()** (4 connections) — `src/hardware/driver_update.rs`
- **backup_root()** (3 connections) — `src/hardware/driver_update.rs`
- **compare_versions()** (3 connections) — `src/hardware/driver_update.rs`
- **enumerate_pnputil_devices()** (3 connections) — `src/hardware/driver_update.rs`
- **enumerate_pnputil_driver_store()** (3 connections) — `src/hardware/driver_update.rs`
- **test_backup_root_exists()** (2 connections) — `src/hardware/driver_update.rs`
- **test_compare_versions_logic()** (2 connections) — `src/hardware/driver_update.rs`
- **test_parse_pnputil_devices_empty()** (2 connections) — `src/hardware/driver_update.rs`
- **test_parse_pnputil_devices_sample()** (2 connections) — `src/hardware/driver_update.rs`
- **test_parse_pnputil_drivers_sample()** (2 connections) — `src/hardware/driver_update.rs`
- **test_vendor_download_url_known()** (2 connections) — `src/hardware/driver_update.rs`
- **test_vendor_download_url_unknown()** (2 connections) — `src/hardware/driver_update.rs`
- **MAX_RETRIES** (1 connections) — `src/hardware/driver_update.rs`
- **test_virtual_device_filter()** (1 connections) — `src/hardware/driver_update.rs`
- **verify_driver_signature()** (1 connections) — `src/hardware/driver_update.rs`
- **VIRTUAL_DEVICE_FILTERS** (1 connections) — `src/hardware/driver_update.rs`

## Relationships

- [InstalledAppxPackage](InstalledAppxPackage.md) (10 shared connections)
- [Language](Language.md) (8 shared connections)
- [TaskCache](TaskCache.md) (6 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [GameProfile](GameProfile.md) (2 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [.setup](setup.md) (1 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (1 shared connections)

## Source Files

- `src/hardware/driver_update.rs`

## Audit Trail

- EXTRACTED: 66 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*