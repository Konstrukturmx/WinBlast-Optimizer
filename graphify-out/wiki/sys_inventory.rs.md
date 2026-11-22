# sys_inventory.rs

> 33 nodes · cohesion 0.08

## Key Concepts

- **spawn_guarded_worker()** (51 connections) — `src/app.rs`
- **.sync_clean_apps_busy()** (10 connections) — `src/app.rs`
- **.execute_clean_selected()** (5 connections) — `src/app.rs`
- **.scan_appx_and_features_async()** (5 connections) — `src/app.rs`
- **WorkerDropGuard** (5 connections) — `src/app.rs`
- **.create_restore_point()** (4 connections) — `src/app.rs`
- **.install_selected_winget_async()** (4 connections) — `src/app.rs`
- **.remove_edge_async()** (4 connections) — `src/app.rs`
- **.remove_selected_appx_async()** (4 connections) — `src/app.rs`
- **.toggle_optional_feature_async()** (4 connections) — `src/app.rs`
- **.uninstall_onedrive_async()** (4 connections) — `src/app.rs`
- **Waker** (4 connections)
- **Sender** (3 connections)
- **.block_telemetry_hosts()** (3 connections) — `src/app.rs`
- **.restart_explorer_safely()** (3 connections) — `src/app.rs`
- **panic_payload_to_string()** (3 connections) — `src/app.rs`
- **OneDriveEdgeResult** (3 connections) — `src/app.rs`
- **P** (2 connections)
- **.activate_winblast_ultimate_power_plan()** (2 connections) — `src/app.rs`
- **.set_active_power_plan()** (2 connections) — `src/app.rs`
- **.toggle_gpu_msi()** (2 connections) — `src/app.rs`
- **.toggle_hags()** (2 connections) — `src/app.rs`
- **.toggle_hibernation()** (2 connections) — `src/app.rs`
- **.toggle_mpo()** (2 connections) — `src/app.rs`
- **.toggle_usb_power_saving()** (2 connections) — `src/app.rs`
- *... and 8 more nodes in this community*

## Relationships

- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (20 shared connections)
- [startup.rs](startup.rs.md) (12 shared connections)
- [super](super.md) (8 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (7 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (7 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (6 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [.reset_baseline](reset_baseline.md) (1 shared connections)
- [.rollback_entry](rollback_entry.md) (1 shared connections)
- [.run_scan_internal](run_scan_internal.md) (1 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 108 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*