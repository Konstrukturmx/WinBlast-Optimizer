# Graph Report - WinBlast0  (2026-11-20)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3750 nodes · 7750 edges · 172 communities (107 shown, 65 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 319 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8571b966`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- action_button_guarded
- AppState
- AppState
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- sys_inventory.rs
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
- startup.rs
- theme.rs
- SlowScanBackend
- .take_injected_error
- WinBlast Optimizer
- in_progress_hud.rs
- full_telemetry_audit_with
- WinBlastError
- LazyLoader
- super
- clean_apps.rs
- Multi-Language Tweak Localization Tables
- FailAtMutationBackend
- System Architecture Changelog
- .Reg State() Vendor()
- Appx Package() Prefixes
- RealBackend
- GPU Silicon & Hardware Tuning Policies
- reg_mutator.rs
- Win32 Registry Scanner & Mutator
- driver_update.rs
- power.rs
- spawn_guarded_worker
- startup.rs
- registry_scanner.rs
- elevation.rs
- parse_guid
- AppX Package & Modern App Scanner
- parking.rs
- Embedded Catalog Test
- ps_runner.rs
- FailAtMutationBackend
- Hardware Diagnostics & GPU Silicon Tuning
- Dynamicdwordformula Logicalcorecount Totalraminkb
- svc_mutator.rs
- ThemePalette
- ui_scrollbar_test.rs
- generational_cache.rs
- updater.rs
- heuristic_scanner.rs
- DriverCategory
- SlowScanBackend
- Project History & Release Version Ledger
- RegRawSnapshot
- driver_update.rs
- DriverError
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- Hardware Diagnostics & GPU Silicon Tuning
- Error.Rs Winblasterror Appx
- CoreParkingState
- task_scanner.rs
- InstalledAppxPackage
- Reversibility Instant Requireslogoff
- WorkerEvent
- path
- TaskCache
- backend.rs
- Project Documentation, Architecture & Licensing
- restore_point.rs
- app.rs
- .set_status_spec
- PlatformSecurityInfo
- DriverUpdateItem
- .setup
- Hardware Diagnostics & GPU Silicon Tuning
- deserialize
- Language
- HibernationStatus
- normalize_guid_str
- DriverUpdateState
- enums
- SandboxEnv
- Runtime
- UiThemeStyle
- restart_explorer_safely
- latency_bench.rs
- ThemePalette
- edge_cases_unorthodox.rs
- action_button
- LiveScheduledTaskInfo
- RegTreeSnapshot
- Hardware Diagnostics & GPU Silicon Tuning
- real_windows_env.rs
- action_button_guarded
- Project History & Release Version Ledger
- Changelog
- PowerSchemeInfo
- Low-Level NT Core & Kernel Architecture
- privacy_shield.rs
- Command Osstr Process.Rs
- PlatformSecurityInfo
- PdhParkingCollector
- RestorePoint
- DriverError
- app.rs
- SystemMetrics
- test_wave1_compound_actions_deep_edge_cases
- progress_bar.rs
- tools/game_overlay.rs
- AutoTuneProfile
- test_wave1_ledger_rollback_and_concurrency_regressions
- view_i18n.rs
- PresetKey
- GameOverlaySettings
- GameProfile
- .navigate
- WinBlastScrollable
- UpdateStatus
- GamingSubTab
- .setup
- diff_modal.rs
- ui/mod.rs
- .sync_startup_tasks_busy
- .all_physical
- WinBlast Optimizer — Developer & Architecture Guide
- UpdateState
- .run_scan_internal
- WindowsInfo
- .block_telemetry_hosts
- .new
- StartupEntry
- spawn_guarded_worker
- goal_planner.rs
- views/network_diag.rs
- window_icon.rs

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 347 edges
3. `ViewTextKey` - 220 edges
4. `SystemBackend` - 117 edges
5. `SandboxEnv` - 115 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `SlowScanBackend` - 85 edges
9. `RealBackend` - 83 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SystemBackend`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (172 total, 65 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 3 - "AppState"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.10
Nodes (22): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+14 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 6 - "sys_inventory.rs"
Cohesion: 0.06
Nodes (25): DiskSubsystemReport, extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback(), query_disk_capacity_ioctl(), query_nvme_smart_ioctl() (+17 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.10
Nodes (25): assess_system_health(), HealthSeverity, Critical, Info, Warning, install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind (+17 more)

### Community 11 - "startup.rs"
Cohesion: 0.16
Nodes (19): set_scheduled_task_enabled(), CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service() (+11 more)

### Community 12 - "theme.rs"
Cohesion: 0.07
Nodes (24): load_embedded_rules(), test_unorthodox_concurrent_worker_event_stress(), test_real_backend_readonly_live_host_audit(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync() (+16 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.07
Nodes (21): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate (+13 more)

### Community 14 - ".take_injected_error"
Cohesion: 0.11
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 15 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.13
Nodes (37): is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry() (+29 more)

### Community 18 - "WinBlastError"
Cohesion: 0.11
Nodes (23): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+15 more)

### Community 19 - "LazyLoader"
Cohesion: 0.08
Nodes (14): panic_payload_to_string(), BiosSystemInfo, DisplayDriverInfo, background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader (+6 more)

### Community 20 - "super"
Cohesion: 0.10
Nodes (8): GameSuitabilityReport, SuitabilityCheck, GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry, BenchmarkRunner, BenchmarkScore, RepairTask

### Community 21 - "clean_apps.rs"
Cohesion: 0.18
Nodes (15): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, view(), build_appx_features_body(), build_disk_cleaner_body(), build_winget_body() (+7 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.10
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 27 - "RealBackend"
Cohesion: 0.09
Nodes (35): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+27 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.07
Nodes (12): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+4 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.18
Nodes (20): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), validate_registry_mutation_path(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree() (+12 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.08
Nodes (22): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.12
Nodes (22): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+14 more)

### Community 33 - "power.rs"
Cohesion: 0.08
Nodes (5): fallback_scheme_name(), resolve_indirect_string(), set_usb_root_hub_power_saving(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 35 - "startup.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.14
Nodes (27): encode_hex(), check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword() (+19 more)

### Community 37 - "elevation.rs"
Cohesion: 0.22
Nodes (3): check_elevation(), is_elevated(), apply_and_restart()

### Community 38 - "parse_guid"
Cohesion: 0.29
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 40 - "parking.rs"
Cohesion: 0.17
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.20
Nodes (27): CachedScanBackend, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action(), evaluate_tweak() (+19 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.21
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.16
Nodes (7): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak(), revert_tweaks_by_id(), revert_tweaks_by_id_with()

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.12
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 47 - "ThemePalette"
Cohesion: 0.24
Nodes (15): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+7 more)

### Community 48 - "ui_scrollbar_test.rs"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 49 - "generational_cache.rs"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 50 - "updater.rs"
Cohesion: 0.22
Nodes (12): check_latest_release(), clean_exe_path(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer() (+4 more)

### Community 51 - "heuristic_scanner.rs"
Cohesion: 0.06
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 52 - "DriverCategory"
Cohesion: 0.14
Nodes (15): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+7 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 55 - "RegRawSnapshot"
Cohesion: 0.11
Nodes (23): list_restore_points(), FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind (+15 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.16
Nodes (18): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), MAX_RETRIES, parse_pnputil_devices(), parse_pnputil_drivers(), PnpDeviceInfo (+10 more)

### Community 57 - "DriverError"
Cohesion: 0.23
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 60 - "backend.rs"
Cohesion: 0.10
Nodes (5): MetricsSampler, SandboxMetricsSampler, SystemMonitor, CpuInfo, LiveServiceInfo

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.18
Nodes (13): active_logical_core_count(), compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases() (+5 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 64 - "CoreParkingState"
Cohesion: 0.21
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 65 - "task_scanner.rs"
Cohesion: 0.20
Nodes (13): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+5 more)

### Community 66 - "InstalledAppxPackage"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "WorkerEvent"
Cohesion: 0.07
Nodes (25): WorkerEvent, AppxAndFeaturesLoaded, AppxBatchRemoved, AppxScanFailed, CleanExecuted, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted (+17 more)

### Community 69 - "path"
Cohesion: 0.14
Nodes (9): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus (+1 more)

### Community 71 - "backend.rs"
Cohesion: 0.28
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 73 - "restore_point.rs"
Cohesion: 0.29
Nodes (7): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 74 - "app.rs"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 75 - ".set_status_spec"
Cohesion: 0.09
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, BaselineReset, PerfBenchmarkCompleted, relaunch_as_admin()

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 77 - "DriverUpdateItem"
Cohesion: 0.31
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 78 - ".setup"
Cohesion: 0.49
Nodes (8): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), open_subkey_flags_64()

### Community 79 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.41
Nodes (11): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_dropdown_anchoring_and_header_height_stability(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() (+3 more)

### Community 81 - "Language"
Cohesion: 0.13
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 82 - "HibernationStatus"
Cohesion: 0.24
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 83 - "normalize_guid_str"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 84 - "DriverUpdateState"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 85 - "enums"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 86 - "SandboxEnv"
Cohesion: 0.07
Nodes (3): CachedScanBackend<'a>, SandboxEnv, normalize_task_path()

### Community 87 - "Runtime"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 88 - "UiThemeStyle"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 89 - "restart_explorer_safely"
Cohesion: 0.29
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 91 - "ThemePalette"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 94 - "action_button"
Cohesion: 0.29
Nodes (13): render_compatible_card(), render_incompatible_card(), view(), view(), action_button(), action_button_icon_only(), action_button_loading(), action_button_with_tooltip() (+5 more)

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 98 - "real_windows_env.rs"
Cohesion: 0.08
Nodes (5): SilentCommand, FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox()

### Community 99 - "action_button_guarded"
Cohesion: 0.26
Nodes (14): StartupTasksSubTab, ScheduledTasks, Services, StartupApps, before_after_comparison_card(), view(), build_services_list(), build_startup_apps_list() (+6 more)

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 101 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 104 - "privacy_shield.rs"
Cohesion: 0.09
Nodes (26): StagedTransition, StageApply, StageRevert, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot (+18 more)

### Community 107 - "PdhParkingCollector"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 109 - "DriverError"
Cohesion: 0.29
Nodes (6): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown

### Community 110 - "app.rs"
Cohesion: 0.04
Nodes (55): AppMode, Advanced, Easy, CleanSortMode, NameAsc, SizeAsc, SizeDesc, DriverSortMode (+47 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 116 - "AutoTuneProfile"
Cohesion: 0.16
Nodes (9): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult, create_auto_tune_plan() (+1 more)

### Community 118 - "view_i18n.rs"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 122 - ".navigate"
Cohesion: 0.14
Nodes (4): BiosInfoLoaded, DiskReportLoaded, DriverScanCompleted, ThermalSnapshot

### Community 123 - "WinBlastScrollable"
Cohesion: 0.23
Nodes (6): rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 124 - "UpdateStatus"
Cohesion: 0.17
Nodes (10): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate (+2 more)

### Community 125 - "GamingSubTab"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

### Community 126 - ".setup"
Cohesion: 0.21
Nodes (6): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 127 - "diff_modal.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 128 - "ui/mod.rs"
Cohesion: 0.58
Nodes (6): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), view()

### Community 131 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 145 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 157 - "StartupEntry"
Cohesion: 0.14
Nodes (13): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), StartupEntry, StartupSource, RegistryHkcuRun (+5 more)

### Community 159 - "spawn_guarded_worker"
Cohesion: 0.07
Nodes (10): spawn_guarded_worker(), WorkerDropGuard, ExplorerRestarted, LatencyBenchmarkCompleted, RestorePointCreated, RollbackCompleted, ScheduledTaskRunResult, ServiceLiveUpdated (+2 more)

### Community 162 - "views/network_diag.rs"
Cohesion: 0.70
Nodes (3): localize_dns_category(), localize_dns_status(), view()

## Knowledge Gaps
- **1199 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` (+1194 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1786 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **65 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `ui/mod.rs`, `Reversibility Instant Requireslogoff`, `.Toggle .Set .Apply`, `action_button_guarded`, `privacy_shield.rs`, `app.rs`, `ThemePalette`, `.run_scan_internal`, `enums`, `clean_apps.rs`, `Appx Package() Prefixes`, `GPU Silicon & Hardware Tuning Policies`, `action_button`, `diff_modal.rs`?**
  _High betweenness centrality (0.275) - this node is a cross-community bridge._
- **Are the 26 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 26 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges` to the rest of the system?**
  _1199 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `AppState` to `ui/mod.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `AppState`, `Disk Cleaning & Temporary File Purger`, `sys_inventory.rs`, `.Toggle .Set .Apply`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `theme.rs`, `SlowScanBackend`, `.run_scan_internal`, `LazyLoader`, `super`, `clean_apps.rs`, `WindowsInfo`, `Multi-Language Tweak Localization Tables`, `System Architecture Changelog`, `.poll`, `GPU Silicon & Hardware Tuning Policies`, `StartupEntry`, `spawn_guarded_worker`, `goal_planner.rs`, `views/network_diag.rs`, `AppX Package & Modern App Scanner`, `Embedded Catalog Test`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `ThemePalette`, `ui_scrollbar_test.rs`, `generational_cache.rs`, `updater.rs`, `DriverCategory`, `Project History & Release Version Ledger`, `State Ledger & Restore Point Engine`, `backend.rs`, `Error.Rs Winblasterror Appx`, `CoreParkingState`, `InstalledAppxPackage`, `Reversibility Instant Requireslogoff`, `WorkerEvent`, `path`, `backend.rs`, `.set_status_spec`, `PlatformSecurityInfo`, `DriverUpdateItem`, `Hardware Diagnostics & GPU Silicon Tuning`, `deserialize`, `Language`, `HibernationStatus`, `action_button`, `LiveScheduledTaskInfo`, `action_button_guarded`, `Changelog`, `PowerSchemeInfo`, `Low-Level NT Core & Kernel Architecture`, `privacy_shield.rs`, `Command Osstr Process.Rs`, `PlatformSecurityInfo`, `RestorePoint`, `app.rs`, `SystemMetrics`, `progress_bar.rs`, `tools/game_overlay.rs`, `AutoTuneProfile`, `view_i18n.rs`, `GameOverlaySettings`, `GameProfile`, `.navigate`, `UpdateStatus`, `.setup`, `diff_modal.rs`?**
  _High betweenness centrality (0.254) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `ui/mod.rs`, `goal_planner.rs`, `views/network_diag.rs`, `.Toggle .Set .Apply`, `ThemePalette`, `view_i18n.rs`, `action_button`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._