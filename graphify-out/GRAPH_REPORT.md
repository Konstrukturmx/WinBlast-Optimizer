# Graph Report - WinBlast0  (2026-11-20)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3857 nodes · 8006 edges · 171 communities (116 shown, 55 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 315 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a288ff8b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- action_button_guarded
- spawn_guarded_worker
- AppState
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- sys_inventory.rs
- features_apps.rs
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
- startup.rs
- theme.rs
- SlowScanBackend
- .take_injected_error
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
- Hardware Diagnostics & GPU Silicon Tuning
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
- FailAtMutationBackend
- SlowScanBackend
- deserialize
- updater.rs
- PanickingBackend
- SlowScanBackend
- Project History & Release Version Ledger
- RegRawSnapshot
- driver_update.rs
- DriverError
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- Hardware Diagnostics & GPU Silicon Tuning
- Any
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
- WorkerEvent
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
- FeatureInstallState
- WindowsOptionalFeature
- LiveScheduledTaskInfo
- RegTreeSnapshot
- Hardware Diagnostics & GPU Silicon Tuning
- advanced_matrix.rs
- AppxCache
- Project History & Release Version Ledger
- WinBlastScrollable
- PowerSchemeInfo
- Low-Level NT Core & Kernel Architecture
- privacy_shield.rs
- PlatformSecurityInfo
- PlatformSecurityInfo
- PdhParkingCollector
- RestorePoint
- probe_physical_drive
- SystemMetrics
- ui_scrollbar_test.rs
- test_wave1_compound_actions_deep_edge_cases
- progress_bar.rs
- tools/game_overlay.rs
- tools/game_overlay.rs
- lazy_loader.rs
- TaskCache
- boot_config.rs
- power_governor.rs
- GameProfile
- reg_helpers.rs
- progress_bar.rs
- storage.rs
- adaptive_tuning.rs
- .poll
- WinBlast Optimizer — Developer & Architecture Guide
- InstalledAppxPackage
- sysprep_engine.rs
- PowerSchemeInfo
- UpdateStatus
- main.rs
- StartupSortMode
- DiskSubsystemReport
- OverallHealthGrade
- AppxSafetyTier
- clean_exe_path
- views/network_diag.rs
- DriverSortMode

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 347 edges
3. `ViewTextKey` - 220 edges
4. `SystemBackend` - 122 edges
5. `SandboxEnv` - 119 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `SlowScanBackend` - 85 edges
9. `RealBackend` - 83 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SystemBackend`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (171 total, 55 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.05
Nodes (18): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, BaselineReset, BiosInfoLoaded, CleanExecuted, CleanTargetsScanned (+10 more)

### Community 3 - "AppState"
Cohesion: 0.04
Nodes (9): AppState, PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, LatencyBenchmarkCompleted, PerfBenchmarkCompleted (+1 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.10
Nodes (22): get_standard_game_benchmarks(), LayoutTier, Compact, Medium, Wide, view(), view(), view() (+14 more)

### Community 6 - "sys_inventory.rs"
Cohesion: 0.07
Nodes (21): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate (+13 more)

### Community 7 - "features_apps.rs"
Cohesion: 0.13
Nodes (24): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+16 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.06
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.09
Nodes (3): query_smart_health_wmi(), test_query_smart_health_wmi_fallback(), SystemCodeIntegrityInfo

### Community 11 - "startup.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 12 - "theme.rs"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.04
Nodes (57): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle, AppMode, Advanced (+49 more)

### Community 14 - ".take_injected_error"
Cohesion: 0.09
Nodes (23): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), build_populated_sandbox_state(), click_key(), click_text(), drain_state_workers(), init_cpu_parking_state(), init_dashboard_state() (+15 more)

### Community 18 - "WinBlastError"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 19 - "LazyLoader"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 20 - "super"
Cohesion: 0.06
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 21 - "clean_apps.rs"
Cohesion: 0.16
Nodes (9): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), LazyLoader, LazyPayload, Bios, Disks, Drivers, Thermals (+1 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.17
Nodes (26): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+18 more)

### Community 23 - "FailAtMutationBackend"
Cohesion: 0.08
Nodes (22): display_reg_value_name(), EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY (+14 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.09
Nodes (33): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+25 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.16
Nodes (22): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+14 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.11
Nodes (22): assert_valid_permutation(), BitSet256, BitSetIter, find_min_weight_cycle_edge(), kahn_schedule(), MAX_NODES, position_of(), resolve_cycles() (+14 more)

### Community 27 - "RealBackend"
Cohesion: 0.15
Nodes (31): LedgerEntry, is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), ABSENT_SENTINEL, apply_action_slice_with() (+23 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.09
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.11
Nodes (3): optional_feature_matches(), SetHags, SetMpo

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.05
Nodes (36): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, PresetKey, Gaming, LaptopBattery, ProAudio (+28 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.06
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 33 - "power.rs"
Cohesion: 0.13
Nodes (20): assess_system_health(), HealthSeverity, Critical, Info, Warning, install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind (+12 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.20
Nodes (14): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), normalize_task_path(), parse_csv_line(), query_single_task_detailed(), query_task_via_com() (+6 more)

### Community 35 - "startup.rs"
Cohesion: 0.12
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.15
Nodes (25): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+17 more)

### Community 37 - "elevation.rs"
Cohesion: 0.08
Nodes (5): fallback_scheme_name(), resolve_indirect_string(), set_usb_root_hub_power_saving(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 38 - "parse_guid"
Cohesion: 0.19
Nodes (26): CachedScanBackend, OsRequirement, TweakCatalog, TweakDefinition, revert_tweaks_by_id(), revert_tweaks_by_id_with(), evaluate_tweak(), evaluate_tweak_status() (+18 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.11
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 40 - "parking.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.13
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.11
Nodes (22): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, ElevationRequired (+14 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.29
Nodes (13): render_compatible_card(), render_incompatible_card(), view(), view(), action_button(), action_button_icon_only(), action_button_loading(), action_button_with_tooltip() (+5 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.19
Nodes (4): StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak()

### Community 46 - "svc_mutator.rs"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 49 - "deserialize"
Cohesion: 0.09
Nodes (8): BiosSystemInfo, DisplayDriverInfo, GameProfile, GameFpsComparison, LiveGameTelemetry, BenchmarkRunner, BenchmarkScore, RepairTask

### Community 50 - "updater.rs"
Cohesion: 0.23
Nodes (18): validate_registry_mutation_path(), normalize_reg_value_name(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node() (+10 more)

### Community 53 - "SlowScanBackend"
Cohesion: 0.15
Nodes (20): set_scheduled_task_enabled(), CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+12 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 56 - "driver_update.rs"
Cohesion: 0.11
Nodes (19): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, ExplorerRestarted, FeatureSet, GpuMsi, PowerCfgSet (+11 more)

### Community 57 - "DriverError"
Cohesion: 0.08
Nodes (27): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+19 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.10
Nodes (6): MetricsSampler, SandboxMetricsSampler, SandboxTempDir, SystemMonitor, test_sandbox_core_parking_labels_metrics_and_power_sync(), SystemMetrics

### Community 59 - "task_scanner.rs"
Cohesion: 0.16
Nodes (18): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), MAX_RETRIES, parse_pnputil_devices(), parse_pnputil_drivers(), PnpDeviceInfo (+10 more)

### Community 60 - "backend.rs"
Cohesion: 0.12
Nodes (5): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.16
Nodes (18): Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd() (+10 more)

### Community 62 - "Any"
Cohesion: 0.29
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.20
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 64 - "CoreParkingState"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 65 - "task_scanner.rs"
Cohesion: 0.17
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 66 - "InstalledAppxPackage"
Cohesion: 0.13
Nodes (15): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+7 more)

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.12
Nodes (6): EngineMode, Instant, Normal, test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports()

### Community 68 - "WorkerEvent"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 69 - "path"
Cohesion: 0.24
Nodes (9): active_logical_core_count(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases(), sample_live_cpu_and_parking() (+1 more)

### Community 70 - "TaskCache"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 71 - "backend.rs"
Cohesion: 0.21
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.11
Nodes (15): RiskLevel, Extreme, High, Low, Moderate, Safe, AutoTunePlan, AutoTuneProfile (+7 more)

### Community 73 - "WorkerEvent"
Cohesion: 0.07
Nodes (31): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled (+23 more)

### Community 74 - "app.rs"
Cohesion: 0.15
Nodes (18): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+10 more)

### Community 75 - ".set_status_spec"
Cohesion: 0.14
Nodes (10): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), RestorePoint, test_ensure_preflight_restore_point_sandbox() (+2 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.13
Nodes (5): SilentCommand, DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 77 - "DriverUpdateItem"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 78 - ".setup"
Cohesion: 0.21
Nodes (5): FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus, test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()

### Community 80 - "deserialize"
Cohesion: 0.56
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 81 - "Language"
Cohesion: 0.25
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 83 - "normalize_guid_str"
Cohesion: 0.27
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 84 - "DriverUpdateState"
Cohesion: 0.20
Nodes (4): encode_hex(), RegRawSnapshot, get_raw(), reg_type_to_u32()

### Community 86 - "SandboxEnv"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 87 - "Runtime"
Cohesion: 0.49
Nodes (7): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), tier_from_rect(), view()

### Community 88 - "UiThemeStyle"
Cohesion: 0.39
Nodes (5): enum_scm_services_by_type(), query_scm_service_config(), query_scm_win32_services(), ScHandleGuard, ScmServiceRecord

### Community 90 - "latency_bench.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 91 - "ThemePalette"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.12
Nodes (22): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+14 more)

### Community 95 - "LiveScheduledTaskInfo"
Cohesion: 0.23
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 96 - "RegTreeSnapshot"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 98 - "advanced_matrix.rs"
Cohesion: 0.16
Nodes (13): RiskValuation, TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), state_counts() (+5 more)

### Community 99 - "AppxCache"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 101 - "WinBlastScrollable"
Cohesion: 0.18
Nodes (8): before_after_comparison_card(), view(), rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 102 - "PowerSchemeInfo"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 105 - "PlatformSecurityInfo"
Cohesion: 0.16
Nodes (10): detect_platform_security_info(), NtQuerySystemInformation(), PlatformSecurityInfo, query_live_code_integrity_options(), query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name() (+2 more)

### Community 107 - "PdhParkingCollector"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 108 - "RestorePoint"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 109 - "probe_physical_drive"
Cohesion: 0.19
Nodes (14): DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), probe_physical_drive(), query_ata_smart_ioctl() (+6 more)

### Community 111 - "SystemMetrics"
Cohesion: 0.25
Nodes (4): compute_nt_cpu_deltas(), parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.47
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 116 - "tools/game_overlay.rs"
Cohesion: 0.27
Nodes (3): GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck

### Community 118 - "TaskCache"
Cohesion: 0.18
Nodes (3): TaskCache, list_live_tasks_from_cache(), LiveScheduledTaskInfo

### Community 119 - "boot_config.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 120 - "power_governor.rs"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 122 - "reg_helpers.rs"
Cohesion: 0.24
Nodes (11): Result, is_known_numeric_reg_sz(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry(), test_composite_string_merge_preserves_siblings(), test_hkcr_and_shallow_tree_delete_blocked(), test_known_numeric_reg_sz() (+3 more)

### Community 123 - "progress_bar.rs"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 124 - "storage.rs"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 125 - "adaptive_tuning.rs"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 149 - "InstalledAppxPackage"
Cohesion: 0.31
Nodes (4): InstalledAppxPackage, is_microsoft_appx_publisher(), list_installed_appx_from_cache(), list_installed_appx_from_cache_with_provisioned()

### Community 150 - "sysprep_engine.rs"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 152 - "UpdateStatus"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 154 - "StartupSortMode"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 155 - "DiskSubsystemReport"
Cohesion: 0.40
Nodes (4): DiskSubsystemReport, PhysicalDriveHealth, probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic()

### Community 156 - "OverallHealthGrade"
Cohesion: 0.33
Nodes (5): OverallHealthGrade, Good, NeedsOptimization, Optimal, Suboptimal

### Community 157 - "AppxSafetyTier"
Cohesion: 0.33
Nodes (5): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat

### Community 158 - "clean_exe_path"
Cohesion: 0.40
Nodes (3): relaunch_as_admin(), clean_exe_path(), test_clean_windows_exe_path()

### Community 159 - "views/network_diag.rs"
Cohesion: 0.70
Nodes (3): localize_dns_category(), localize_dns_status(), view()

### Community 160 - "DriverSortMode"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

## Knowledge Gaps
- **1215 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` (+1210 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1798 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **55 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `AppState` to `spawn_guarded_worker`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `sys_inventory.rs`, `startup.rs`, `theme.rs`, `SlowScanBackend`, `.take_injected_error`, `full_telemetry_audit_with`, `LazyLoader`, `super`, `InstalledAppxPackage`, `Multi-Language Tweak Localization Tables`, `PowerSchemeInfo`, `System Architecture Changelog`, `.Reg State() Vendor()`, `StartupSortMode`, `UpdateStatus`, `RealBackend`, `DiskSubsystemReport`, `clean_exe_path`, `Hardware Diagnostics & GPU Silicon Tuning`, `DriverSortMode`, `power.rs`, `views/network_diag.rs`, `startup.rs`, `parse_guid`, `AppX Package & Modern App Scanner`, `ps_runner.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `deserialize`, `SlowScanBackend`, `Project History & Release Version Ledger`, `DriverError`, `State Ledger & Restore Point Engine`, `Hardware Diagnostics & GPU Silicon Tuning`, `Error.Rs Winblasterror Appx`, `CoreParkingState`, `InstalledAppxPackage`, `Reversibility Instant Requireslogoff`, `WorkerEvent`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `WorkerEvent`, `.set_status_spec`, `PlatformSecurityInfo`, `DriverUpdateItem`, `.setup`, `Hardware Diagnostics & GPU Silicon Tuning`, `Language`, `HibernationStatus`, `normalize_guid_str`, `Runtime`, `WindowsOptionalFeature`, `LiveScheduledTaskInfo`, `advanced_matrix.rs`, `WinBlastScrollable`, `Low-Level NT Core & Kernel Architecture`, `privacy_shield.rs`, `PlatformSecurityInfo`, `PlatformSecurityInfo`, `RestorePoint`, `ui_scrollbar_test.rs`, `tools/game_overlay.rs`, `tools/game_overlay.rs`, `TaskCache`, `.poll`?**
  _High betweenness centrality (0.279) - this node is a cross-community bridge._
- **Are the 19 inferred relationships involving `AppState` (e.g. with `BiosSystemInfo` and `DiskSubsystemReport`) actually correct?**
  _`AppState` has 19 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges` to the rest of the system?**
  _1215 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `CoreParkingState`, `advanced_matrix.rs`, `AppState`, `Disk Cleaning & Temporary File Purger`, `WinBlastScrollable`, `Hardware Diagnostics & GPU Silicon Tuning`, `SlowScanBackend`, `.take_injected_error`, `SandboxEnv`, `Runtime`, `Multi-Language Tweak Localization Tables`, `.Reg State() Vendor()`, `Win32 Registry Scanner & Mutator`?**
  _High betweenness centrality (0.274) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `Disk Cleaning & Temporary File Purger`, `PlatformSecurityInfo`, `Hardware Diagnostics & GPU Silicon Tuning`, `tools/game_overlay.rs`, `Runtime`, `.Reg State() Vendor()`, `views/network_diag.rs`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._