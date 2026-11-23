# WinBlast0 — v1.6.1 (Rune) → v1.7.0 (Slint) Parity Matrix

Originally generated 2026-11-22 from the closures in tag `v1.6.1` `src/ui/**` that change `AppState`.
**Re-audited 2026-11-23** against all of `src/ui_slint/**` on branch `1.7.0`.

**Totals:** 146 action entries · 143 WIRED · 3 REPLACED by a safer path · 0 missing.

Status legend: **WIRED** = a Slint callback reaches this AppState action · **REPLACED** = reached through a
different, backend-routed or UI-local path (explained inline) · read-only lookups are left out.
Behaviour of every view is exercised by `tests/ui_views.rs` (sandboxed backend, never the host).

## mod.rs

| Action | Status |
|---|---|
| `navigate()` | WIRED |
| `mode =` | WIRED |
| `toggle_engine_mode()` | WIRED |
| `nav_prev_view()` | WIRED |
| `nav_next_view()` | WIRED |
| `start_scan()` | WIRED |
| `toggle_sidebar()` | WIRED |
| `active_dropdown =` | REPLACED — dropdown open/close is UI-local state (`AppStore.dropdown`) |
| `toggle_theme_mode()` | WIRED |
| `relaunch_elevated()` | WIRED |
| `toggle_changelog()` | WIRED |
| `rollback_entry()` | WIRED |
| `open_bug_report()` | WIRED |
| `sidebar_open =` | WIRED |
| `show_diff_preview =` | WIRED |
| `inspected_tweak =` | WIRED |
| `set_language()` | WIRED |
| `set_theme_style()` | WIRED |
| `set_theme_mode()` | WIRED |
| `set_color_scheme()` | WIRED |

## views/advanced_matrix.rs

| Action | Status |
|---|---|
| `select_filtered_tweaks()` | WIRED |
| `deselect_filtered_tweaks()` | WIRED |
| `start_scan()` | WIRED |
| `apply_selected()` | WIRED |
| `revert_selected()` | WIRED |
| `show_diff_preview =` | WIRED |
| `open_diff_preview()` | WIRED |
| `restart_explorer_safely()` | WIRED |
| `apply_preset()` | WIRED |
| `selected_category =` | WIRED |
| `clear_advanced_filters()` | WIRED |
| `matrix_sort_mode =` | WIRED |
| `search_edit.text =` | WIRED |
| `handle_switch_click()` | WIRED |
| `status_message =` | WIRED |
| `toggle_category_selection()` | WIRED |
| `inspect_tweak()` | WIRED |

## views/automation_center.rs

| Action | Status |
|---|---|
| `execute_auto_tune_profile()` | WIRED |
| `execute_memory_trim()` | WIRED |
| `selected_auto_tune_profile =` | WIRED |

## views/backup_center.rs

| Action | Status |
|---|---|
| `create_restore_point()` | WIRED |
| `refresh_restore_points()` | WIRED |
| `export_profile()` | WIRED |
| `import_profile()` | WIRED |
| `rollback_entry()` | WIRED |
| `apply_unattend_preset()` | WIRED |
| `unattend_opts.architecture =` | WIRED |
| `unattend_opts.bypass_tpm_ram_requirements =` | WIRED |
| `unattend_opts.bypass_msa_oobe =` | WIRED |
| `unattend_opts.skip_oobe_privacy_screens =` | WIRED |
| `unattend_opts.include_selected_tweaks =` | WIRED |
| `unattend_opts.auto_timezone_utc =` | WIRED |
| `export_autounattend_xml()` | WIRED |

## views/clean_apps.rs

| Action | Status |
|---|---|
| `clean_sub_tab =` | WIRED |
| `select_all_clean_targets()` | WIRED |
| `deselect_all_clean_targets()` | WIRED |
| `scan_clean_targets_async()` | WIRED |
| `execute_clean_selected()` | WIRED |
| `toggle_clean_target()` | WIRED |
| `scan_appx_and_features_async()` | WIRED |
| `remove_selected_appx_async()` | WIRED |
| `select_filtered_appx()` | WIRED |
| `select_safe_and_ai_appx()` | WIRED |
| `deselect_all_appx()` | WIRED |
| `uninstall_onedrive_async()` | WIRED |
| `remove_edge_async()` | WIRED |
| `toggle_appx_selection()` | WIRED |
| `toggle_optional_feature_async()` | WIRED |
| `deselect_all_winget()` | WIRED |
| `install_selected_winget_async()` | WIRED |
| `toggle_winget_selection()` | WIRED |

## views/cpu_parking.rs

| Action | Status |
|---|---|
| `unpark_all_cores()` | WIRED |
| `restore_stock_parking()` | WIRED |
| `refresh_core_parking()` | WIRED |

## views/disk_health.rs

| Action | Status |
|---|---|
| `scan_disk_health_async()` | WIRED |

## views/driver_bios.rs

| Action | Status |
|---|---|
| `scan_online_drivers()` | WIRED |
| `test_sandbox_driver_install()` | WIRED |
| `scan_bios_info_async()` | WIRED |
| `driver_category_filter =` | WIRED |
| `backup_driver()` | WIRED |
| `install_driver_update()` | WIRED |

## views/easy_dashboard.rs

| Action | Status |
|---|---|
| `reset_baseline()` | WIRED |
| `apply_preset()` | WIRED |
| `navigate()` | WIRED |
| `relaunch_as_admin()` | WIRED |
| `check_for_updates()` | WIRED |
| `download_and_install_update()` | WIRED |
| `apply_and_restart()` | WIRED |
| `start_scan()` | WIRED |

## views/fault_diagnostics.rs

| Action | Status |
|---|---|
| `fault_report =` | WIRED |
| `navigate()` | WIRED |

## views/game_overlay.rs

| Action | Status |
|---|---|
| `game_overlay.game_bar_disabled =` | REPLACED — `set_game_overlay_async()` (backend writes the real registry value; v1.6.1 only flipped a field) |

## views/game_prereqs.rs

| Action | Status |
|---|---|
| `scan_game_prereqs()` | WIRED |
| `install_all_missing_prereqs()` | WIRED |
| `install_game_prereq()` | WIRED |

## views/game_profiles.rs

| Action | Status |
|---|---|
| `status_message =` | WIRED |

## views/gaming_stats.rs

| Action | Status |
|---|---|
| `gaming_sub_tab =` | WIRED |
| `game_telemetry =` | WIRED |

## views/gpu_tuning.rs

| Action | Status |
|---|---|
| `toggle_hide_tweak()` | WIRED |
| `inspect_tweak()` | WIRED |
| `toggle_hags()` | WIRED |
| `toggle_mpo()` | WIRED |
| `toggle_gpu_msi()` | WIRED |

## views/hardware_view.rs

| Action | Status |
|---|---|
| `refresh_platform_inventory()` | WIRED |
| `refresh_latency()` | WIRED |
| `navigate()` | WIRED |
| `set_active_power_plan()` | WIRED |
| `activate_winblast_ultimate_power_plan()` | WIRED |
| `toggle_hibernation()` | WIRED |
| `toggle_usb_power_saving()` | WIRED |

## views/latency_bench.rs

| Action | Status |
|---|---|
| `start_latency_benchmark()` | WIRED |
| `set_latency_baseline_before()` | WIRED |
| `reset_latency_benchmark()` | WIRED |

## views/network_diag.rs

| Action | Status |
|---|---|
| `network_diag =` | WIRED |
| `set_status()` | REPLACED — `network_action_async()` reports the real netsh outcome (v1.6.1 always claimed success) |

## views/performance_tests.rs

| Action | Status |
|---|---|
| `start_perf_benchmark_async()` | WIRED |

## views/privacy_shield.rs

| Action | Status |
|---|---|
| `run_telemetry_audit()` | WIRED |
| `block_telemetry_hosts()` | WIRED |
| `neutralize_telemetry_services()` | WIRED |
| `deselect_all()` | WIRED |
| `apply_selected()` | WIRED |

## views/startup_tasks.rs

| Action | Status |
|---|---|
| `startup_sub_tab =` | WIRED |
| `toggle_startup_entry_async()` | WIRED |
| `toggle_startup_entry_selection()` | WIRED |
| `toggle_scheduled_task_selection()` | WIRED |
| `run_live_task_async()` | WIRED |
| `toggle_live_task_async()` | WIRED |
| `set_live_service_start_async()` | WIRED |
| `select_all_filtered_startup()` | WIRED |
| `deselect_all_startup()` | WIRED |
| `batch_toggle_selected_startup_async()` | WIRED |
| `select_all_filtered_tasks()` | WIRED |
| `deselect_all_tasks()` | WIRED |
| `batch_toggle_selected_tasks_async()` | WIRED |
| `hide_microsoft_tasks =` | WIRED |
| `scan_startup_and_tasks_async()` | WIRED |
| `startup_status_filter =` | WIRED |
| `startup_sort_mode =` | WIRED |

## views/support_about.rs

| Action | Status |
|---|---|
| `open_bug_report()` | WIRED |
| `set_theme_style()` | WIRED |

## views/thermal_monitor.rs

| Action | Status |
|---|---|
| `scan_thermal_snapshot_async()` | WIRED |

## views/windows_repair.rs

| Action | Status |
|---|---|
| `status_message =` | WIRED |

## widgets/button.rs

| Action | Status |
|---|---|
| `handle_switch_click()` | WIRED |

## widgets/diff_modal.rs

| Action | Status |
|---|---|
| `show_diff_preview =` | WIRED |
| `inspected_tweak =` | WIRED |

## widgets/in_progress_hud.rs

| Action | Status |
|---|---|
| `dismiss_progress_hud =` | WIRED |
