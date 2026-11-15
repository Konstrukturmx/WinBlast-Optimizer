# Hardware Diagnostics & GPU Silicon Tuning

> 35 nodes · cohesion 0.09

## Key Concepts

- **SandboxEnv** (114 connections) — `src/core/backend.rs`
- **.take_injected_error()** (36 connections) — `src/core/backend.rs`
- **normalize_task_path()** (8 connections) — `src/scanner/task_scanner.rs`
- **.reg_set_dword_for_vendor()** (7 connections) — `src/core/backend.rs`
- **.new()** (5 connections) — `src/core/backend.rs`
- **.reg_restore_tree()** (5 connections) — `src/core/backend.rs`
- **.seed_tweak_defaults_for_tweak()** (4 connections) — `src/core/backend.rs`
- **.reg_set_dword()** (3 connections) — `src/core/backend.rs`
- **.run_scheduled_task()** (3 connections) — `src/core/backend.rs`
- **.seed_all_tweaks()** (3 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (3 connections) — `src/core/backend.rs`
- **.set_usb_root_hub_power_saving()** (3 connections) — `src/core/backend.rs`
- **.create_and_activate_performance_plan()** (2 connections) — `src/core/backend.rs`
- **.default()** (2 connections) — `src/core/backend.rs`
- **.get_task_enabled()** (2 connections) — `src/core/backend.rs`
- **.remove_edge_preserve_webview2()** (2 connections) — `src/core/backend.rs`
- **.request_system_reboot()** (2 connections) — `src/core/backend.rs`
- **.restart_explorer_safely()** (2 connections) — `src/core/backend.rs`
- **.set_gpu_msi_mode()** (2 connections) — `src/core/backend.rs`
- **.set_startup_entry_enabled_by_id()** (2 connections) — `src/core/backend.rs`
- **.uninstall_onedrive_safe()** (2 connections) — `src/core/backend.rs`
- **.write_hosts_file()** (2 connections) — `src/core/backend.rs`
- **.set_usb_power_saving_enabled()** (2 connections) — `src/core/backend.rs`
- **.broadcast_shell_settings_changed()** (1 connections) — `src/core/backend.rs`
- **.check_onedrive_kfm_active()** (1 connections) — `src/core/backend.rs`
- *... and 10 more nodes in this community*

## Relationships

- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (38 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (11 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (11 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (7 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [Rollback Panic (`Src/Backup/State](Rollback_Panic_`Src-Backup-State.md) (5 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (4 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (4 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (3 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (3 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (3 shared connections)
- [.Create Restore Point()](Create_Restore_Point.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/task_scanner.rs`

## Audit Trail

- EXTRACTED: 170 (97%)
- INFERRED: 6 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*