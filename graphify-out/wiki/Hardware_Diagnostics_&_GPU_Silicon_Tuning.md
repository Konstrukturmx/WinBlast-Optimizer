# Hardware Diagnostics & GPU Silicon Tuning

> 44 nodes · cohesion 0.07

## Key Concepts

- **SandboxEnv** (115 connections) — `src/core/backend.rs`
- **.take_injected_error()** (36 connections) — `src/core/backend.rs`
- **normalize_task_path()** (8 connections) — `src/scanner/task_scanner.rs`
- **.sync_parking_in_state()** (7 connections) — `src/core/backend.rs`
- **.write_power_setting_index()** (6 connections) — `src/core/backend.rs`
- **.remove_appx_family_in_state()** (4 connections) — `src/core/backend.rs`
- **.restore_stock_parking()** (4 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults_for_tweak()** (4 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (4 connections) — `src/core/backend.rs`
- **.apply_parking_snapshot()** (4 connections) — `src/core/backend.rs`
- **.reg_delete_value()** (3 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (3 connections) — `src/core/backend.rs`
- **.reg_set_dword()** (3 connections) — `src/core/backend.rs`
- **.remove_appx_batch()** (3 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (3 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (3 connections) — `src/core/backend.rs`
- **.seed_all_tweaks()** (3 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults()** (3 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (3 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (3 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (2 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (2 connections) — `src/core/backend.rs`
- **.reg_get_binary()** (2 connections) — `src/core/backend.rs`
- **.remove_edge_preserve_webview2()** (2 connections) — `src/core/backend.rs`
- **.request_system_reboot()** (2 connections) — `src/core/backend.rs`
- *... and 19 more nodes in this community*

## Relationships

- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (40 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (9 shared connections)
- [backend.rs](backend.rs.md) (7 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (5 shared connections)
- [normalize_guid_str](normalize_guid_str.md) (5 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (5 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (5 shared connections)
- [engine.rs](engine.rs.md) (4 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (4 shared connections)
- [.new](new.md) (3 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (3 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/task_scanner.rs`

## Audit Trail

- EXTRACTED: 184 (97%)
- INFERRED: 6 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*