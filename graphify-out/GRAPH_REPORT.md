# Graph Report - WinBlast0  (2026-11-17)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3399 nodes · 7085 edges · 155 communities (109 shown, 46 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 303 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `49c056e9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- action_button_guarded
- Privacy, Telemetry & AI Hardening Catalog
- AppState
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- Windows OS Settings & UX Customization
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
- Test Composite With()
- theme.rs
- SlowScanBackend
- automation.rs
- View() Color Scrollspec
- Hardware Diagnostics & GPU Silicon Tuning
- full_telemetry_audit_with
- Process Token Information
- Core UI Internationalization & Text Catalog
- Hardware Diagnostics & GPU Silicon Tuning
- Multi-Language Tweak Localization Tables
- Integration Suite: Tweak Sandbox E2E
- System Architecture Changelog
- .Reg State() Vendor()
- Appx Package() Prefixes
- RealBackend
- GPU Silicon & Hardware Tuning Policies
- Hardware Diagnostics & GPU Silicon Tuning
- Core UI Internationalization & Text Catalog
- Win32 Registry Scanner & Mutator
- driver_update.rs
- Hardware Diagnostics & GPU Silicon Tuning
- spawn_guarded_worker
- startup.rs
- Integration Suite: Tweak Sandbox E2E
- Disabled() Delete .Set
- Task Scheduler Inspection Subsystem
- AppX Package & Modern App Scanner
- .Create Restore Point()
- Embedded Catalog Test
- WinBlast0 Architectural Specifications
- FailAtMutationBackend
- Hardware Diagnostics & GPU Silicon Tuning
- Dynamicdwordformula Logicalcorecount Totalraminkb
- svc_mutator.rs
- Integration Suite: Ui Responsive E2E
- ui_scrollbar_test.rs
- engine.rs
- test_wave1_ledger_rollback_and_concurrency_regressions
- State Ledger & Restore Point Engine
- Optional Dism Feature()
- localize_tweak
- Project History & Release Version Ledger
- RegRawSnapshot
- ViewTextKey
- Hardware Diagnostics & GPU Silicon Tuning
- State Ledger & Restore Point Engine
- task_scanner.rs
- restore_point.rs
- Hardware Diagnostics & GPU Silicon Tuning
- Restore Points() .List
- Error.Rs Winblasterror Appx
- path
- Project Documentation, Architecture & Licensing
- Startup Tasks & Auto-Run Explorer
- Reversibility Instant Requireslogoff
- WinBlast0 Architectural Specifications
- Hardware Diagnostics & GPU Silicon Tuning
- LayoutTier
- backend.rs
- Project Documentation, Architecture & Licensing
- .Create Metrics Sampler()
- app.rs
- tools/game_prereqs.rs
- PlatformSecurityInfo
- Installed Appx .List
- .setup
- deserialize
- test_wave1_compound_actions_deep_edge_cases
- .sync_clean_apps_busy
- normalize_guid_str
- WinBlast0 Architectural Specifications
- enums
- action_button
- UiThemeStyle
- SilentCommand
- latency_bench.rs
- ThemePalette
- ThermalHealth
- updater.rs
- ColorScheme
- diff_modal.rs
- DriverCategory
- Hardware Diagnostics & GPU Silicon Tuning
- scan_class_registry
- WindowsInfo
- settle_frames
- DriverError
- DiskSubsystemReport
- PdhParkingCollector
- LiveServiceInfo
- PowerSchemeInfo
- progress_bar.rs
- tools/game_overlay.rs
- LiveScheduledTaskInfo
- goal_planner.rs
- DriverProgress
- PresetKey
- Spring
- RegTreeSnapshot
- MatrixSortMode
- UpdateStatus
- DriverUpdateState
- GamingSubTab
- PrivacySortMode
- .sync_startup_tasks_busy
- UpdateState
- StartupSource
- enum_scm_services_by_type
- RestorePoint
- WindowsOptionalFeature
- .new
- ActiveDropdown
- StartupSortMode
- RegStringValue
- HardwareSubTab
- CleanSortMode
- DriverSortMode
- PowerActionKind
- StartupStatusFilter
- delete_value_for_vendor
- StagedTransition
- RepairTask

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 334 edges
3. `ViewTextKey` - 137 edges
4. `SystemBackend` - 117 edges
5. `SandboxEnv` - 115 edges
6. `StatusMessageSpec` - 105 edges
7. `SlowScanBackend` - 85 edges
8. `RealBackend` - 84 edges
9. `ThemePalette` - 83 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (155 total, 46 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.17
Nodes (26): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+18 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 3 - "AppState"
Cohesion: 0.03
Nodes (11): AppState, PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted (+3 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 6 - "Windows OS Settings & UX Customization"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.09
Nodes (19): AStarNode, AStarPlanner, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler, ExecutionOrder (+11 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.07
Nodes (22): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+14 more)

### Community 11 - "Test Composite With()"
Cohesion: 0.10
Nodes (19): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, FeatureToggled, GpuMsiUpdated, HagsUpdated (+11 more)

### Community 12 - "theme.rs"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 14 - "automation.rs"
Cohesion: 0.07
Nodes (34): assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult (+26 more)

### Community 15 - "View() Color Scrollspec"
Cohesion: 0.09
Nodes (10): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+2 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.57
Nodes (6): full_telemetry_audit(), full_telemetry_audit_with(), get_blocked_hosts(), parse_blocked_hosts_from_content(), scan_telemetry_agents(), TelemetryAuditResult

### Community 19 - "Process Token Information"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 20 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.15
Nodes (27): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+19 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.09
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 23 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.09
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.14
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.14
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.06
Nodes (33): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+25 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.10
Nodes (33): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+25 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.07
Nodes (5): fallback_scheme_name(), resolve_indirect_string(), set_usb_root_hub_power_saving(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 32 - "driver_update.rs"
Cohesion: 0.14
Nodes (20): backup_root(), create_sandbox_test_item(), enumerate_pnputil_devices(), MAX_RETRIES, parse_pnputil_devices(), PnpDeviceInfo, scan_online_drivers(), scan_online_drivers_with_progress() (+12 more)

### Community 33 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.08
Nodes (10): detect_platform_security_info(), NtQuerySystemInformation(), query_live_code_integrity_options(), query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), SystemCodeIntegrityInfo (+2 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.10
Nodes (6): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, ExplorerRestarted, RollbackCompleted, TelemetryHostsBlocked

### Community 35 - "startup.rs"
Cohesion: 0.17
Nodes (12): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services(), replace_ascii_ci() (+4 more)

### Community 36 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.07
Nodes (22): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+14 more)

### Community 37 - "Disabled() Delete .Set"
Cohesion: 0.12
Nodes (22): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, ElevationRequired (+14 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.18
Nodes (5): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), rollback_entry()

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 40 - ".Create Restore Point()"
Cohesion: 0.17
Nodes (8): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), set_startup_entry_enabled(), set_startup_entry_enabled_by_id(), StartupEntry

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.17
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 42 - "WinBlast0 Architectural Specifications"
Cohesion: 0.13
Nodes (21): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+13 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.16
Nodes (26): CachedScanBackend, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action(), evaluate_tweak(), evaluate_tweak_status() (+18 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.29
Nodes (10): Result, delete_tree(), format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service() (+2 more)

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.13
Nodes (15): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary(), set_dword(), set_dword_for_vendor() (+7 more)

### Community 48 - "ui_scrollbar_test.rs"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 49 - "engine.rs"
Cohesion: 0.12
Nodes (41): is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry() (+33 more)

### Community 51 - "State Ledger & Restore Point Engine"
Cohesion: 0.11
Nodes (5): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 52 - "Optional Dism Feature()"
Cohesion: 0.13
Nodes (12): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), set_scheduled_task_enabled(), test_hosts_blocking_ignores_comments_and_subdomains() (+4 more)

### Community 53 - "localize_tweak"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 56 - "ViewTextKey"
Cohesion: 0.02
Nodes (124): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+116 more)

### Community 57 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 59 - "task_scanner.rs"
Cohesion: 0.11
Nodes (14): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+6 more)

### Community 60 - "restore_point.rs"
Cohesion: 0.15
Nodes (9): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats(), SystemMonitor (+1 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 62 - "Restore Points() .List"
Cohesion: 0.26
Nodes (8): active_logical_core_count(), compute_nt_cpu_deltas(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemProcessorPerformanceInformation

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 64 - "path"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.20
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 68 - "WinBlast0 Architectural Specifications"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 69 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 70 - "LayoutTier"
Cohesion: 0.09
Nodes (29): build_dropdown_overlay(), build_sidebar(), LayoutTier, Compact, Medium, Wide, mode_tab(), nav_button() (+21 more)

### Community 71 - "backend.rs"
Cohesion: 0.17
Nodes (4): MetricsSampler, optional_feature_matches(), SandboxMetricsSampler, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.29
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 73 - ".Create Metrics Sampler()"
Cohesion: 0.17
Nodes (8): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS, test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 74 - "app.rs"
Cohesion: 0.11
Nodes (18): AppMode, Advanced, Easy, DriversSubTab, Drivers, Prerequisites, EngineMode, Instant (+10 more)

### Community 75 - "tools/game_prereqs.rs"
Cohesion: 0.17
Nodes (23): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+15 more)

### Community 77 - "Installed Appx .List"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 78 - ".setup"
Cohesion: 0.18
Nodes (7): CoreParkingUpdated, LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 80 - "deserialize"
Cohesion: 0.12
Nodes (6): SilentCommand, GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry, BenchmarkRunner, BenchmarkScore

### Community 82 - ".sync_clean_apps_busy"
Cohesion: 0.15
Nodes (5): AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, OneDriveEdgeResult, WinGetInstallResult

### Community 83 - "normalize_guid_str"
Cohesion: 0.27
Nodes (14): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid(), read_power_setting_index() (+6 more)

### Community 84 - "WinBlast0 Architectural Specifications"
Cohesion: 0.47
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 85 - "enums"
Cohesion: 0.12
Nodes (6): BiosSystemInfo, DisplayDriverInfo, FaultDiagnosticsReport, MinidumpEntry, PendingRebootStatus, GameProfile

### Community 86 - "action_button"
Cohesion: 0.13
Nodes (25): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), render_compatible_card(), render_incompatible_card() (+17 more)

### Community 88 - "UiThemeStyle"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 89 - "SilentCommand"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 91 - "ThemePalette"
Cohesion: 0.14
Nodes (21): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+13 more)

### Community 92 - "ThermalHealth"
Cohesion: 0.24
Nodes (7): ComponentThermal, SystemThermalSnapshot, ThermalHealth, Cool, Hot, Optimal, Warm

### Community 93 - "updater.rs"
Cohesion: 0.21
Nodes (10): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+2 more)

### Community 94 - "ColorScheme"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 95 - "diff_modal.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 96 - "DriverCategory"
Cohesion: 0.14
Nodes (12): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+4 more)

### Community 98 - "scan_class_registry"
Cohesion: 0.24
Nodes (9): compare_versions(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), parse_pnputil_drivers(), PnpDriverPackage, scan_class_registry(), test_compare_versions_logic() (+1 more)

### Community 108 - "settle_frames"
Cohesion: 0.45
Nodes (10): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction(), test_ui_responsive_matrix_all_viewports_views_themes_locales_scales() (+2 more)

### Community 109 - "DriverError"
Cohesion: 0.20
Nodes (15): backup_driver(), download_and_install_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other (+7 more)

### Community 110 - "DiskSubsystemReport"
Cohesion: 0.24
Nodes (6): DiskSubsystemReport, extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback()

### Community 111 - "PdhParkingCollector"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 113 - "PowerSchemeInfo"
Cohesion: 0.27
Nodes (4): RollbackRefreshData, list_power_schemes(), PowerSchemeInfo, test_power_scheme_enumeration_nonblocking()

### Community 114 - "progress_bar.rs"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.27
Nodes (3): GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck

### Community 117 - "goal_planner.rs"
Cohesion: 0.48
Nodes (3): OptimizationGoal, metric_pill(), view()

### Community 118 - "DriverProgress"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 119 - "PresetKey"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

### Community 120 - "Spring"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 122 - "MatrixSortMode"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 123 - "UpdateStatus"
Cohesion: 0.18
Nodes (9): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate (+1 more)

### Community 124 - "DriverUpdateState"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 125 - "GamingSubTab"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 126 - "PrivacySortMode"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 129 - ".sync_startup_tasks_busy"
Cohesion: 0.29
Nodes (3): ScheduledTaskRunResult, ServiceLiveUpdated, StartupEntryToggled

### Community 130 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 131 - "StartupSource"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 136 - "enum_scm_services_by_type"
Cohesion: 0.48
Nodes (5): enum_scm_services_by_type(), query_scm_service_config(), query_scm_win32_services(), ScHandleGuard, ScmServiceRecord

### Community 140 - "ActiveDropdown"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 141 - "StartupSortMode"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 144 - "HardwareSubTab"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

### Community 145 - "CleanSortMode"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 146 - "DriverSortMode"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

### Community 147 - "PowerActionKind"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 148 - "StartupStatusFilter"
Cohesion: 0.50
Nodes (4): StartupStatusFilter, ActiveOnly, All, InactiveOnly

### Community 150 - "StagedTransition"
Cohesion: 0.67
Nodes (3): StagedTransition, StageApply, StageRevert

## Knowledge Gaps
- **1113 isolated node(s):** `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc`, `ActiveRailDc`, `ActiveRam` (+1108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1610 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **46 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `action_button_guarded`, `AppState`, `LayoutTier`, `.Create Metrics Sampler()`, `app.rs`, `View() Color Scrollspec`, `Process Token Information`, `action_button`, `latency_bench.rs`, `ThemePalette`, `ColorScheme`, `diff_modal.rs`?**
  _High betweenness centrality (0.304) - this node is a cross-community bridge._
- **Are the 26 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 26 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` to the rest of the system?**
  _1113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `AppState` to `.sync_startup_tasks_busy`, `action_button_guarded`, `Privacy, Telemetry & AI Hardening Catalog`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `Integration Suite: Tweak Sandbox E2E`, `.Reg .Set Power`, `RestorePoint`, `Test Composite With()`, `ActiveDropdown`, `StartupSortMode`, `automation.rs`, `WindowsOptionalFeature`, `HardwareSubTab`, `CleanSortMode`, `DriverSortMode`, `full_telemetry_audit_with`, `StartupStatusFilter`, `View() Color Scrollspec`, `Multi-Language Tweak Localization Tables`, `StagedTransition`, `Integration Suite: Tweak Sandbox E2E`, `RepairTask`, `Appx Package() Prefixes`, `Hardware Diagnostics & GPU Silicon Tuning`, `spawn_guarded_worker`, `Task Scheduler Inspection Subsystem`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `Hardware Diagnostics & GPU Silicon Tuning`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `ui_scrollbar_test.rs`, `engine.rs`, `Scandetailkind Appxinstalled Appxinstalledps`, `Project History & Release Version Ledger`, `ViewTextKey`, `Hardware Diagnostics & GPU Silicon Tuning`, `State Ledger & Restore Point Engine`, `restore_point.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `Error.Rs Winblasterror Appx`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `LayoutTier`, `app.rs`, `tools/game_prereqs.rs`, `PlatformSecurityInfo`, `.setup`, `deserialize`, `.sync_clean_apps_busy`, `enums`, `action_button`, `UiThemeStyle`, `SilentCommand`, `latency_bench.rs`, `ThemePalette`, `ThermalHealth`, `ColorScheme`, `diff_modal.rs`, `DriverCategory`, `WindowsInfo`, `settle_frames`, `DriverError`, `DiskSubsystemReport`, `LiveServiceInfo`, `PowerSchemeInfo`, `tools/game_overlay.rs`, `LiveScheduledTaskInfo`, `goal_planner.rs`, `MatrixSortMode`, `UpdateStatus`, `GamingSubTab`, `PrivacySortMode`?**
  _High betweenness centrality (0.292) - this node is a cross-community bridge._
- **Should `Privacy, Telemetry & AI Hardening Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.021505376344086023 - nodes in this community are weakly interconnected._
- **Why does `SystemBackend` connect `Disk Cleaning & Temporary File Purger` to `AppState`, `.Toggle .Set .Apply`, `.Reg .Set Power`, `WindowsOptionalFeature`, `RestorePoint`, `Scandetailkind Appxinstalled Appxinstalledps`, `SlowScanBackend`, `RegStringValue`, `Hardware Diagnostics & GPU Silicon Tuning`, `full_telemetry_audit_with`, `Integration Suite: Tweak Sandbox E2E`, `System Architecture Changelog`, `RealBackend`, `Integration Suite: Tweak Sandbox E2E`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `WinBlast0 Architectural Specifications`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `engine.rs`, `Optional Dism Feature()`, `RegRawSnapshot`, `Hardware Diagnostics & GPU Silicon Tuning`, `State Ledger & Restore Point Engine`, `task_scanner.rs`, `restore_point.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `backend.rs`, `app.rs`, `PlatformSecurityInfo`, `WindowsInfo`, `LiveServiceInfo`, `PowerSchemeInfo`, `LiveScheduledTaskInfo`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._