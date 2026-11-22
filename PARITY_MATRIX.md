# WinBlast0 — v1.6.1 (Rune) → v1.7.0 (Slint) Parity Matrix

Generated 2026-11-22 by scanning the closures in tag `v1.6.1` `src/ui/**` that change `AppState`, and comparing them against `src/ui_slint/bridge.rs` at commit 5afc324.
**Totals:** 146 action entries across views, 19 wired, 127 missing.

Status legend: **WIRED** = a Slint callback reaches this AppState action · **MISSING** = no Slint path · read-only lookups are left out.

## mod.rs

| Action | Status |
|---|---|
| `navigate()` | WIRED |
| `mode =` | **MISSING** |
| `toggle_engine_mode()` | **MISSING** |
| `nav_prev_view()` | **MISSING** |
| `nav_next_view()` | **MISSING** |
| `start_scan()` | WIRED |
| `toggle_sidebar()` | **MISSING** |
| `active_dropdown =` | **MISSING** |
| `toggle_theme_mode()` | **MISSING** |
| `relaunch_elevated()` | **MISSING** |
| `toggle_changelog()` | **MISSING** |
| `rollback_entry()` | **MISSING** |
| `open_bug_report()` | **MISSING** |
| `sidebar_open =` | **MISSING** |
| `show_diff_preview =` | **MISSING** |
| `inspected_tweak =` | **MISSING** |
| `set_language()` | **MISSING** |
| `set_theme_style()` | **MISSING** |
| `set_theme_mode()` | **MISSING** |
| `set_color_scheme()` | **MISSING** |

## views/advanced_matrix.rs

| Action | Status |
|---|---|
| `select_filtered_tweaks()` | **MISSING** |
| `deselect_filtered_tweaks()` | **MISSING** |
| `start_scan()` | WIRED |
| `apply_selected()` | **MISSING** |
| `revert_selected()` | **MISSING** |
| `show_diff_preview =` | **MISSING** |
| `open_diff_preview()` | **MISSING** |
| `restart_explorer_safely()` | **MISSING** |
| `apply_preset()` | **MISSING** |
| `selected_category =` | **MISSING** |
| `clear_advanced_filters()` | **MISSING** |
| `matrix_sort_mode =` | **MISSING** |
| `search_edit.text =` | **MISSING** |
| `handle_switch_click()` | **MISSING** |
| `status_message =` | WIRED |
| `toggle_category_selection()` | **MISSING** |
| `inspect_tweak()` | **MISSING** |

## views/automation_center.rs

| Action | Status |
|---|---|
| `execute_auto_tune_profile()` | **MISSING** |
| `execute_memory_trim()` | **MISSING** |
| `selected_auto_tune_profile =` | **MISSING** |

## views/backup_center.rs

| Action | Status |
|---|---|
| `create_restore_point()` | WIRED |
| `refresh_restore_points()` | **MISSING** |
| `export_profile()` | **MISSING** |
| `import_profile()` | **MISSING** |
| `rollback_entry()` | **MISSING** |
| `apply_unattend_preset()` | **MISSING** |
| `unattend_opts.architecture =` | **MISSING** |
| `unattend_opts.bypass_tpm_ram_requirements =` | **MISSING** |
| `unattend_opts.bypass_msa_oobe =` | **MISSING** |
| `unattend_opts.skip_oobe_privacy_screens =` | **MISSING** |
| `unattend_opts.include_selected_tweaks =` | **MISSING** |
| `unattend_opts.auto_timezone_utc =` | **MISSING** |
| `export_autounattend_xml()` | **MISSING** |

## views/clean_apps.rs

| Action | Status |
|---|---|
| `clean_sub_tab =` | **MISSING** |
| `select_all_clean_targets()` | **MISSING** |
| `deselect_all_clean_targets()` | **MISSING** |
| `scan_clean_targets_async()` | **MISSING** |
| `execute_clean_selected()` | WIRED |
| `toggle_clean_target()` | **MISSING** |
| `scan_appx_and_features_async()` | **MISSING** |
| `remove_selected_appx_async()` | WIRED |
| `select_filtered_appx()` | **MISSING** |
| `select_safe_and_ai_appx()` | **MISSING** |
| `deselect_all_appx()` | **MISSING** |
| `uninstall_onedrive_async()` | **MISSING** |
| `remove_edge_async()` | **MISSING** |
| `toggle_appx_selection()` | WIRED |
| `toggle_optional_feature_async()` | **MISSING** |
| `deselect_all_winget()` | **MISSING** |
| `install_selected_winget_async()` | **MISSING** |
| `toggle_winget_selection()` | **MISSING** |

## views/cpu_parking.rs

| Action | Status |
|---|---|
| `unpark_all_cores()` | **MISSING** |
| `restore_stock_parking()` | WIRED |
| `refresh_core_parking()` | **MISSING** |

## views/disk_health.rs

| Action | Status |
|---|---|
| `scan_disk_health_async()` | WIRED |

## views/driver_bios.rs

| Action | Status |
|---|---|
| `scan_online_drivers()` | **MISSING** |
| `test_sandbox_driver_install()` | **MISSING** |
| `scan_bios_info_async()` | **MISSING** |
| `driver_category_filter =` | **MISSING** |
| `backup_driver()` | **MISSING** |
| `install_driver_update()` | **MISSING** |

## views/easy_dashboard.rs

| Action | Status |
|---|---|
| `reset_baseline()` | **MISSING** |
| `apply_preset()` | **MISSING** |
| `navigate()` | WIRED |
| `relaunch_as_admin()` | **MISSING** |
| `check_for_updates()` | **MISSING** |
| `download_and_install_update()` | **MISSING** |
| `apply_and_restart()` | **MISSING** |
| `start_scan()` | WIRED |

## views/fault_diagnostics.rs

| Action | Status |
|---|---|
| `fault_report =` | **MISSING** |
| `navigate()` | WIRED |

## views/game_overlay.rs

| Action | Status |
|---|---|
| `game_overlay.game_bar_disabled =` | **MISSING** |

## views/game_prereqs.rs

| Action | Status |
|---|---|
| `scan_game_prereqs()` | **MISSING** |
| `install_all_missing_prereqs()` | **MISSING** |
| `install_game_prereq()` | **MISSING** |

## views/game_profiles.rs

| Action | Status |
|---|---|
| `status_message =` | WIRED |

## views/gaming_stats.rs

| Action | Status |
|---|---|
| `gaming_sub_tab =` | **MISSING** |
| `game_telemetry =` | **MISSING** |

## views/gpu_tuning.rs

| Action | Status |
|---|---|
| `toggle_hide_tweak()` | **MISSING** |
| `inspect_tweak()` | **MISSING** |
| `toggle_hags()` | WIRED |
| `toggle_mpo()` | **MISSING** |
| `toggle_gpu_msi()` | **MISSING** |

## views/hardware_view.rs

| Action | Status |
|---|---|
| `refresh_platform_inventory()` | **MISSING** |
| `refresh_latency()` | **MISSING** |
| `navigate()` | WIRED |
| `set_active_power_plan()` | **MISSING** |
| `activate_winblast_ultimate_power_plan()` | **MISSING** |
| `toggle_hibernation()` | **MISSING** |
| `toggle_usb_power_saving()` | **MISSING** |

## views/latency_bench.rs

| Action | Status |
|---|---|
| `start_latency_benchmark()` | **MISSING** |
| `set_latency_baseline_before()` | **MISSING** |
| `reset_latency_benchmark()` | **MISSING** |

## views/network_diag.rs

| Action | Status |
|---|---|
| `network_diag =` | **MISSING** |
| `set_status()` | **MISSING** |

## views/performance_tests.rs

| Action | Status |
|---|---|
| `start_perf_benchmark_async()` | **MISSING** |

## views/privacy_shield.rs

| Action | Status |
|---|---|
| `run_telemetry_audit()` | **MISSING** |
| `block_telemetry_hosts()` | **MISSING** |
| `neutralize_telemetry_services()` | **MISSING** |
| `deselect_all()` | **MISSING** |
| `apply_selected()` | **MISSING** |

## views/startup_tasks.rs

| Action | Status |
|---|---|
| `startup_sub_tab =` | **MISSING** |
| `toggle_startup_entry_async()` | **MISSING** |
| `toggle_startup_entry_selection()` | **MISSING** |
| `toggle_scheduled_task_selection()` | **MISSING** |
| `run_live_task_async()` | **MISSING** |
| `toggle_live_task_async()` | **MISSING** |
| `set_live_service_start_async()` | **MISSING** |
| `select_all_filtered_startup()` | **MISSING** |
| `deselect_all_startup()` | **MISSING** |
| `batch_toggle_selected_startup_async()` | **MISSING** |
| `select_all_filtered_tasks()` | **MISSING** |
| `deselect_all_tasks()` | **MISSING** |
| `batch_toggle_selected_tasks_async()` | **MISSING** |
| `hide_microsoft_tasks =` | **MISSING** |
| `scan_startup_and_tasks_async()` | WIRED |
| `startup_status_filter =` | **MISSING** |
| `startup_sort_mode =` | **MISSING** |

## views/support_about.rs

| Action | Status |
|---|---|
| `open_bug_report()` | **MISSING** |
| `set_theme_style()` | **MISSING** |

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
| `handle_switch_click()` | **MISSING** |

## widgets/diff_modal.rs

| Action | Status |
|---|---|
| `show_diff_preview =` | **MISSING** |
| `inspected_tweak =` | **MISSING** |

## widgets/in_progress_hud.rs

| Action | Status |
|---|---|
| `dismiss_progress_hud =` | **MISSING** |

## Port status (v1.7.0)

| View | Status | Tests |
|---|---|---|
| Shell (top bar, sidebar, progress, changelog, footer, dropdowns, HUD, diff) | DONE | tests/ui_parity.rs (14) |
| Driver & BIOS | DONE: online scan, 8 category chips, folded search, WHQL/state badges, async install/backup, sandbox test driver, board/firmware/Secure Boot, display drivers | tests/ui_views.rs drivers_* (1) |
| Core Parking / Thermal / Disk Health | DONE: unpark/restore/refresh + per-core cards; thermal gauges + components; SMART drives + async TRIM. Fixed fake sensor values, a false DirectStorage claim and a UI-thread TRIM | tests/ui_views.rs core_parking_/thermal_/disk_ (3) |
| Hardware | DONE: waveforms, CPU topology + per-core load, NT latency, power plans, Ultimate plan, hibernation, USB power, platform security, DIMMs, volumes | tests/ui_views.rs hardware_* (2) |
| GPU Tuning | DONE: adapters, HAGS (OS-gated), MPO, MSI + interrupt priority per adapter, vendor-gated tweak cards with hide/unhide/inspect; fake overclock sliders from the pre-1.7.0 Slint view removed | tests/ui_views.rs gpu_* (2) |
| Privacy Shield | DONE: stats, hosts/services audit, block hosts, neutralize services, privacy switch list; fixed an inconsistent Select All | tests/ui_views.rs privacy_* (2) |
| Backup Center | DONE: restore points, profile export/import, Autounattend.xml (presets, arch, 5 options), ledger undo | tests/ui_views.rs backup_* (2) |
| Startup, Tasks & Services | DONE: 3 sub-tabs, filters, 5 sorts, hide-Microsoft, batch enable/disable, run task, service start types (protected core locked). Fixed a stuck-busy bug after batch toggles | tests/ui_views.rs startup_*/services_* (3) |
| Clean & Apps | DONE: disk cleaner (8 targets), AppX bloat with locked protected core, optional features, OneDrive/Edge, WinGet; "Select Safe & AI" localized | tests/ui_views.rs clean_apps_* (4) |
| Easy Dashboard | DONE: all cards and actions; removed invented fallbacks ("Intel / AMD Processor", made-up 12.5 MB update size) | tests/ui_views.rs dashboard_* (4) |
| Advanced Matrix | DONE: all actions wired, plus fixes for risk filtering, consistent Select All matching and a real Export | tests/ui_views.rs matrix_* (8) |

## Cross-cutting features (checked by hand during implementation)

| Feature | v1.6.1 source | Slint status |
|---|---|---|
| Sub-tabs (Clean/Apps/Features/Winget, Startup/Tasks/Services, Gaming, Hardware, Drivers, Repair) | `app.rs` *SubTab enums | partly missing |
| Sort and filter modes (Matrix, Clean, Startup, Driver, Prereqs, Privacy) | `app.rs` *SortMode enums | missing |
| Engine mode Easy/Advanced and preset staging/apply | `toggle_engine_mode`, `apply_preset` | missing |
| Theme mode, 6 color schemes, 5 theme styles, live switch | `theme.rs`, `set_theme_*` | missing (Slint tokens are static) |
| 8-language i18n (TextKey, ViewTextKey, tweak_i18n, status_i18n) | `core/*i18n*` | missing (Slint strings are hard-coded English) |
| Settings persistence (language, theme, zoom, telemetry interval) | `AppState` and prefs | to verify |
| Diff preview modal, tweak inspector | `widgets/diff_modal.rs` | not wired |
| In-progress HUD with cancel/dismiss | `widgets/in_progress_hud.rs` | partly wired |
| Auto-updater (check, download, apply, restart) | `core/updater.rs` | missing |
| Relaunch elevated | `relaunch_elevated` | missing |
| Autounattend.xml exporter and presets | `tools/autounattend.rs` | missing |
| Profile export/import, Ed25519 .winpreset sharing | `backup/preset_sharing.rs` | missing |
| Hotkeys and view cycling (nav_next/prev, sidebar) | `ui/mod.rs` | to verify |
| Fabricated defaults in .slint properties (AGENTS rule 2) | n/a | ~65 to remove |
