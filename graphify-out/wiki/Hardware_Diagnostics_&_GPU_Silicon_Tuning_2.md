# Hardware Diagnostics & GPU Silicon Tuning

> 26 nodes · cohesion 0.14

## Key Concepts

- **driver_update.rs** (50 connections) — `src/hardware/driver_update.rs`
- **scan_class_registry()** (10 connections) — `src/hardware/driver_update.rs`
- **evaluate_version_from_store()** (9 connections) — `src/hardware/driver_update.rs`
- **parse_pnputil_devices()** (5 connections) — `src/hardware/driver_update.rs`
- **PnpDriverPackage** (5 connections) — `src/hardware/driver_update.rs`
- **parse_pnputil_drivers()** (4 connections) — `src/hardware/driver_update.rs`
- **PnpDeviceInfo** (4 connections) — `src/hardware/driver_update.rs`
- **vendor_download_url()** (4 connections) — `src/hardware/driver_update.rs`
- **backup_root()** (3 connections) — `src/hardware/driver_update.rs`
- **compare_versions()** (3 connections) — `src/hardware/driver_update.rs`
- **.class_guid()** (3 connections) — `src/hardware/driver_update.rs`
- **enumerate_pnputil_devices()** (3 connections) — `src/hardware/driver_update.rs`
- **enumerate_pnputil_driver_store()** (3 connections) — `src/hardware/driver_update.rs`
- **estimate_download_size()** (3 connections) — `src/hardware/driver_update.rs`
- **RegKey** (2 connections)
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
- *... and 1 more nodes in this community*

## Relationships

- [adaptive_tuning.rs](adaptive_tuning.rs.md) (10 shared connections)
- [Runtime](Runtime.md) (7 shared connections)
- [TaskCache](TaskCache.md) (7 shared connections)
- [AppxCache](AppxCache.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)
- [path](path.md) (1 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)
- [lazy_loader.rs](lazy_loader.rs.md) (1 shared connections)

## Source Files

- `src/hardware/driver_update.rs`

## Audit Trail

- EXTRACTED: 79 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*