# Graph Report - WinBlast0  (2026-11-15)

## Corpus Check
- 108 files · ~194,502 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 15 file(s) not represented in the graph (top: .toml 10, (none) 3, .ico 2)

## Summary
- 2969 nodes · 6327 edges · 129 communities (88 shown, 41 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 255 edges (avg confidence: 0.81)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `685454ca`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- clean_apps.rs
- Privacy, Telemetry & AI Hardening Catalog
- Async() .Toggle .Refresh
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- Windows OS Settings & UX Customization
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
- Test Composite With()
- View() Card() Switch
- SlowScanBackend
- .sync_clean_apps_busy
- View() Color Scrollspec
- Hardware Diagnostics & GPU Silicon Tuning
- app.rs
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
- .set_status_spec
- Hardware Diagnostics & GPU Silicon Tuning
- spawn_guarded_worker
- list_live_services
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
- Project Documentation, Architecture & Licensing
- engine.rs
- test_wave1_ledger_rollback_and_concurrency_regressions
- State Ledger & Restore Point Engine
- Optional Dism Feature()
- Win32 Registry Scanner & Mutator
- Project History & Release Version Ledger
- RegTreeSnapshot
- ViewTextKey
- Hardware Diagnostics & GPU Silicon Tuning
- State Ledger & Restore Point Engine
- TaskCache
- Integration Suite: Tweak Sandbox E2E
- Hardware Diagnostics & GPU Silicon Tuning
- Restore Points() .List
- Error.Rs Winblasterror Appx
- diagnostics.rs
- Project Documentation, Architecture & Licensing
- Startup Tasks & Auto-Run Explorer
- Reversibility Instant Requireslogoff
- WinBlast0 Architectural Specifications
- Hardware Diagnostics & GPU Silicon Tuning
- LayoutTier
- backend.rs
- Project Documentation, Architecture & Licensing
- .Create Metrics Sampler()
- .Detect Core Parking()
- Hardware Diagnostics & GPU Silicon Tuning
- Installed Appx .List
- action_button_guarded
- Elevated() Token Closehandle
- test_wave1_compound_actions_deep_edge_cases
- driver_update.rs
- .setup
- WinBlast0 Architectural Specifications
- deserialize
- action_button
- ui_scrollbar_test.rs
- curated_windows_features_meta
- tools/network_diag.rs
- latency_bench.rs
- easy_dashboard.rs
- ThermalHealth
- settle_frames
- .sync_startup_tasks_busy
- diff_modal.rs
- Hardware Diagnostics & GPU Silicon Tuning
- .Default() .New() Sandboxtempdir
- WindowsInfo
- view
- StartupSource
- uninstall_onedrive_safe
- PdhParkingCollector
- LiveServiceInfo
- view_i18n.rs
- get_binary
- GameOverlaySettings
- .set_status
- goal_planner.rs
- PowerActionKind
- PresetKey

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 444 edges
2. `AppState` - 279 edges
3. `ViewTextKey` - 137 edges
4. `SandboxEnv` - 114 edges
5. `SystemBackend` - 110 edges
6. `StatusMessageSpec` - 105 edges
7. `SlowScanBackend` - 85 edges
8. `RealBackend` - 84 edges
9. `PanickingBackend` - 81 edges
10. `FailAtMutationBackend` - 79 edges

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

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (129 total, 41 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (422): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+414 more)

### Community 1 - "clean_apps.rs"
Cohesion: 0.24
Nodes (13): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, build_appx_features_body(), build_disk_cleaner_body(), build_winget_body(), clean_target_label() (+5 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 3 - "Async() .Toggle .Refresh"
Cohesion: 0.06
Nodes (3): AppState, ApplyCompleted, RepairTask

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 6 - "Windows OS Settings & UX Customization"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.08
Nodes (20): AStarNode, AStarPlanner, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler (+12 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.07
Nodes (26): load_embedded_rules(), Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, drain_until(), HkcuScratchGuard (+18 more)

### Community 11 - "Test Composite With()"
Cohesion: 0.10
Nodes (20): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled, GpuMsiUpdated (+12 more)

### Community 12 - "View() Card() Switch"
Cohesion: 0.06
Nodes (42): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+34 more)

### Community 14 - ".sync_clean_apps_busy"
Cohesion: 0.13
Nodes (5): AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, OneDriveEdgeResult, WinGetInstallResult

### Community 15 - "View() Color Scrollspec"
Cohesion: 0.09
Nodes (10): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+2 more)

### Community 17 - "app.rs"
Cohesion: 0.11
Nodes (23): AppMode, Advanced, Easy, EngineMode, Instant, Normal, StagedTransition, StageApply (+15 more)

### Community 19 - "Process Token Information"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 20 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.11
Nodes (24): encode_hex(), check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_dword(), get_dword_for_vendor() (+16 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.09
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.07
Nodes (30): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, BackupCenter, CleanAndApps (+22 more)

### Community 23 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.12
Nodes (21): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+13 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.14
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.12
Nodes (23): AI_BLOAT_PREFIXES, classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), is_appx_arch(), is_appx_version() (+15 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.14
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 27 - "RealBackend"
Cohesion: 0.09
Nodes (8): RealBackend, resolve_persistent_app_dir(), is_hags_enabled(), is_mpo_disabled(), set_gpu_msi_mode(), set_hags(), set_mpo_disabled(), set_usb_root_hub_power_saving()

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.08
Nodes (20): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG, EMBEDDED_CATALOG_TELEMETRY_APPS (+12 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.10
Nodes (30): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+22 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.09
Nodes (20): create_and_activate_performance_plan(), fallback_scheme_name(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str() (+12 more)

### Community 32 - ".set_status_spec"
Cohesion: 0.08
Nodes (8): PresetKind, Gaming, LaptopBattery, ProAudio, BaselineReset, LatencyBenchmarkCompleted, RestorePointCreated, RollbackCompleted

### Community 33 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.07
Nodes (11): DiskSubsystemReport, extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback(), query_ram_dimms(), RamDimmInfo (+3 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.12
Nodes (5): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, ExplorerRestarted, TelemetryHostsBlocked

### Community 35 - "list_live_services"
Cohesion: 0.27
Nodes (7): enum_scm_services_by_type(), list_live_services(), query_scm_service_config(), query_scm_win32_services(), resolve_indirect_display_string(), ScHandleGuard, ScmServiceRecord

### Community 36 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.09
Nodes (22): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+14 more)

### Community 37 - "Disabled() Delete .Set"
Cohesion: 0.12
Nodes (16): WinBlastError, AppX, Backup, Database, ElevationRequired, Hardware, OsIncompatible, PowerShell (+8 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.16
Nodes (7): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak(), revert_tweaks_by_id(), revert_tweaks_by_id_with()

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 40 - ".Create Restore Point()"
Cohesion: 0.13
Nodes (16): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_startup_entries(), read_startup_approved_enabled(), replace_ascii_ci() (+8 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.15
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 42 - "WinBlast0 Architectural Specifications"
Cohesion: 0.12
Nodes (18): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+10 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.16
Nodes (21): CachedScanBackend, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action(), evaluate_tweak() (+13 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.10
Nodes (21): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+13 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.36
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.15
Nodes (12): delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, set_binary(), set_dword(), set_dword_for_vendor(), set_string() (+4 more)

### Community 48 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.12
Nodes (18): create_restore_point(), set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), normalize_task_path() (+10 more)

### Community 49 - "engine.rs"
Cohesion: 0.12
Nodes (38): is_unsupported_power_guid_error(), Result, decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target() (+30 more)

### Community 51 - "State Ledger & Restore Point Engine"
Cohesion: 0.11
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 52 - "Optional Dism Feature()"
Cohesion: 0.16
Nodes (11): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+3 more)

### Community 53 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.24
Nodes (9): lookup(), lookup(), lookup(), lookup(), localize_tweak(), LocalizedTweakText, lookup(), lookup() (+1 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 55 - "RegTreeSnapshot"
Cohesion: 0.17
Nodes (7): RegRawSnapshot, RegTreeSnapshot, validate_registry_mutation_path(), restore_tree(), restore_tree_node(), set_raw(), u32_to_reg_type()

### Community 56 - "ViewTextKey"
Cohesion: 0.02
Nodes (124): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+116 more)

### Community 57 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 59 - "TaskCache"
Cohesion: 0.15
Nodes (3): TaskCache, list_live_tasks_from_cache(), LiveScheduledTaskInfo

### Community 60 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.19
Nodes (7): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, list_installed_appx_from_cache()

### Community 62 - "Restore Points() .List"
Cohesion: 0.26
Nodes (8): active_logical_core_count(), compute_nt_cpu_deltas(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemProcessorPerformanceInformation

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 64 - "diagnostics.rs"
Cohesion: 0.20
Nodes (7): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, FaultDiagnosticsReport, MinidumpEntry, PendingRebootStatus

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.24
Nodes (4): RollbackRefreshData, HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 68 - "WinBlast0 Architectural Specifications"
Cohesion: 0.31
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 69 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 70 - "LayoutTier"
Cohesion: 0.09
Nodes (24): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+16 more)

### Community 71 - "backend.rs"
Cohesion: 0.17
Nodes (5): list_restore_points(), RestorePoint, optional_feature_matches(), SandboxMetricsSampler, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.22
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 73 - ".Create Metrics Sampler()"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 76 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.19
Nodes (6): detect_platform_security_info(), NtQuerySystemInformation(), PlatformSecurityInfo, query_live_code_integrity_options(), test_ram_and_platform_inventory_nonblocking(), total_physical_ram_kb()

### Community 77 - "Installed Appx .List"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 78 - "action_button_guarded"
Cohesion: 0.35
Nodes (12): StartupTasksSubTab, ScheduledTasks, Services, StartupApps, build_services_list(), build_startup_apps_list(), build_tasks_list(), empty_card() (+4 more)

### Community 82 - "driver_update.rs"
Cohesion: 0.10
Nodes (26): compare_versions(), create_sandbox_test_item(), download_and_install_driver(), DriverCategory, Audio, Chipset, Display, Network (+18 more)

### Community 83 - ".setup"
Cohesion: 0.24
Nodes (5): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted

### Community 84 - "WinBlast0 Architectural Specifications"
Cohesion: 0.60
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 85 - "deserialize"
Cohesion: 0.11
Nodes (8): BiosSystemInfo, DisplayDriverInfo, GameSuitabilityReport, SuitabilityCheck, GameProfile, GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry

### Community 86 - "action_button"
Cohesion: 0.15
Nodes (24): RiskValuation, TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, ThemePalette, view() (+16 more)

### Community 87 - "ui_scrollbar_test.rs"
Cohesion: 0.35
Nodes (12): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+4 more)

### Community 89 - "tools/network_diag.rs"
Cohesion: 0.16
Nodes (5): DnsProbeResult, NetworkDiagnostics, query_local_ip(), BenchmarkRunner, BenchmarkScore

### Community 91 - "easy_dashboard.rs"
Cohesion: 0.36
Nodes (8): BeforeAfterImpact, before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card(), quick_actions_card(), view()

### Community 92 - "ThermalHealth"
Cohesion: 0.24
Nodes (7): ComponentThermal, SystemThermalSnapshot, ThermalHealth, Cool, Hot, Optimal, Warm

### Community 93 - "settle_frames"
Cohesion: 0.42
Nodes (10): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction(), test_ui_responsive_matrix_all_viewports_views_themes_locales_scales() (+2 more)

### Community 94 - ".sync_startup_tasks_busy"
Cohesion: 0.29
Nodes (3): ScheduledTaskRunResult, ServiceLiveUpdated, StartupEntryToggled

### Community 95 - "diff_modal.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 108 - "view"
Cohesion: 0.64
Nodes (5): build_sidebar(), mode_tab(), nav_button(), nav_section_header(), view()

### Community 109 - "StartupSource"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 110 - "uninstall_onedrive_safe"
Cohesion: 0.40
Nodes (3): check_onedrive_kfm_active(), find_onedrive_setup_exe(), uninstall_onedrive_safe()

### Community 111 - "PdhParkingCollector"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 113 - "view_i18n.rs"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 114 - "get_binary"
Cohesion: 0.40
Nodes (3): get_binary(), ScratchCleanup, test_strict_reg_type_dispatch_and_negative_numeric_sz()

### Community 117 - "goal_planner.rs"
Cohesion: 0.39
Nodes (3): metric_pill(), view(), winblast_scrollbar_style()

### Community 118 - "PowerActionKind"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 119 - "PresetKey"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

## Knowledge Gaps
- **930 isolated node(s):** `AboutTitle`, `AboutSubtitle`, `AboutRepoLabel`, `AboutOpenGithub`, `AboutLicense` (+925 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1356 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **41 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `clean_apps.rs`, `Async() .Toggle .Refresh`, `LayoutTier`, `.Create Metrics Sampler()`, `view`, `View() Card() Switch`, `action_button_guarded`, `View() Color Scrollspec`, `app.rs`, `Process Token Information`, `action_button`, `latency_bench.rs`, `easy_dashboard.rs`, `diff_modal.rs`?**
  _High betweenness centrality (0.297) - this node is a cross-community bridge._
- **Are the 12 inferred relationships involving `AppState` (e.g. with `OptimizationPlan` and `ExecutionOrder`) actually correct?**
  _`AppState` has 12 INFERRED edges - model-reasoned connections that need verification._
- **What connects `AboutTitle`, `AboutSubtitle`, `AboutRepoLabel` to the rest of the system?**
  _930 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.004739336492890996 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `Async() .Toggle .Refresh` to `clean_apps.rs`, `Privacy, Telemetry & AI Hardening Catalog`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `Integration Suite: Tweak Sandbox E2E`, `.Reg .Set Power`, `Scandetailkind Appxinstalled Appxinstalledps`, `Test Composite With()`, `View() Card() Switch`, `.sync_clean_apps_busy`, `View() Color Scrollspec`, `app.rs`, `Multi-Language Tweak Localization Tables`, `Integration Suite: Tweak Sandbox E2E`, `Appx Package() Prefixes`, `.set_status_spec`, `Hardware Diagnostics & GPU Silicon Tuning`, `spawn_guarded_worker`, `Task Scheduler Inspection Subsystem`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `WinBlast0 Architectural Specifications`, `Hardware Diagnostics & GPU Silicon Tuning`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `Project History & Release Version Ledger`, `Hardware Diagnostics & GPU Silicon Tuning`, `State Ledger & Restore Point Engine`, `TaskCache`, `Integration Suite: Tweak Sandbox E2E`, `Hardware Diagnostics & GPU Silicon Tuning`, `Error.Rs Winblasterror Appx`, `diagnostics.rs`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `LayoutTier`, `backend.rs`, `.Detect Core Parking()`, `Hardware Diagnostics & GPU Silicon Tuning`, `action_button_guarded`, `driver_update.rs`, `.setup`, `deserialize`, `action_button`, `ui_scrollbar_test.rs`, `tools/network_diag.rs`, `latency_bench.rs`, `easy_dashboard.rs`, `ThermalHealth`, `settle_frames`, `.sync_startup_tasks_busy`, `diff_modal.rs`, `WindowsInfo`, `view`, `LiveServiceInfo`, `view_i18n.rs`, `GameOverlaySettings`, `.set_status`, `goal_planner.rs`?**
  _High betweenness centrality (0.276) - this node is a cross-community bridge._
- **Should `Privacy, Telemetry & AI Hardening Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.021505376344086023 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `ViewTextKey` to `view_i18n.rs`, `LayoutTier`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._