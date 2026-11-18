# Graph Report - WinBlast0  (2026-11-18)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3413 nodes · 7159 edges · 145 communities (100 shown, 45 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 298 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d28126cc`
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
- Bloatware & AppX Deprovisioning Rules
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
- Hardware Diagnostics & GPU Silicon Tuning
- deserialize
- test_wave1_compound_actions_deep_edge_cases
- .sync_clean_apps_busy
- normalize_guid_str
- WinBlast0 Architectural Specifications
- enums
- action_button
- Runtime
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
- Core UI Internationalization & Text Catalog
- Project History & Release Version Ledger
- Features() .List Optional
- .List Live Services()
- Low-Level NT Core & Kernel Architecture
- Manifest Asinvoker Build.Rs
- Command Osstr Process.Rs
- Integration Suite: Tweak Sandbox E2E
- WindowsInfo
- settle_frames
- DriverError
- DiskSubsystemReport
- PdhParkingCollector
- PowerSchemeInfo
- progress_bar.rs
- tools/game_overlay.rs
- LiveScheduledTaskInfo
- goal_planner.rs
- DriverProgress
- Spring
- RegTreeSnapshot
- MatrixSortMode
- UpdateStatus
- DriverUpdateState
- GamingSubTab
- PrivacySortMode
- make_ico.ps1
- sign_binary.ps1
- .sync_startup_tasks_busy
- UpdateState

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 334 edges
3. `ViewTextKey` - 137 edges
4. `SystemBackend` - 117 edges
5. `SandboxEnv` - 114 edges
6. `StatusMessageSpec` - 105 edges
7. `SlowScanBackend` - 85 edges
8. `RealBackend` - 84 edges
9. `ThemePalette` - 84 edges
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
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (145 total, 45 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.02
Nodes (124): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+116 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.03
Nodes (12): AppState, PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted (+4 more)

### Community 3 - "AppState"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.10
Nodes (32): SystemThermalSnapshot, build_dropdown_overlay(), build_sidebar(), LayoutTier, Compact, Medium, Wide, mode_tab() (+24 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 6 - "Windows OS Settings & UX Customization"
Cohesion: 0.06
Nodes (24): extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback(), query_disk_capacity_ioctl(), query_nvme_smart_ioctl(), query_smart_health_wmi() (+16 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.07
Nodes (34): assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult (+26 more)

### Community 11 - "Test Composite With()"
Cohesion: 0.08
Nodes (31): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+23 more)

### Community 12 - "theme.rs"
Cohesion: 0.07
Nodes (22): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+14 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.08
Nodes (20): AStarNode, AStarPlanner, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler (+12 more)

### Community 15 - "View() Color Scrollspec"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 16 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.12
Nodes (24): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), render_compatible_card(), render_incompatible_card() (+16 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.13
Nodes (37): is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry() (+29 more)

### Community 18 - "Bloatware & AppX Deprovisioning Rules"
Cohesion: 0.07
Nodes (11): spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, CleanExecuted, ExplorerRestarted, OneDriveEdgeResult, ScheduledTaskRunResult, ServiceLiveUpdated (+3 more)

### Community 19 - "Process Token Information"
Cohesion: 0.05
Nodes (35): AppMode, Advanced, Easy, DriverSortMode, Category, NameAsc, UpdatesFirst, DriversSubTab (+27 more)

### Community 20 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.08
Nodes (10): BiosSystemInfo, DisplayDriverInfo, FaultDiagnosticsReport, MinidumpEntry, PendingRebootStatus, GameSuitabilityReport, SuitabilityCheck, GameProfile (+2 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (27): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+19 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.09
Nodes (19): detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor, Amd (+11 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 27 - "RealBackend"
Cohesion: 0.10
Nodes (33): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+25 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.08
Nodes (12): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+4 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.09
Nodes (22): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled (+14 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (24): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+16 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 33 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.07
Nodes (5): fallback_scheme_name(), resolve_indirect_string(), set_usb_root_hub_power_saving(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 35 - "startup.rs"
Cohesion: 0.09
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 36 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.15
Nodes (24): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+16 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.14
Nodes (25): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+17 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.08
Nodes (27): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+19 more)

### Community 40 - ".Create Restore Point()"
Cohesion: 0.17
Nodes (15): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), is_on_ac_power(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+7 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.22
Nodes (23): CachedScanBackend, RiskValuation, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan() (+15 more)

### Community 42 - "WinBlast0 Architectural Specifications"
Cohesion: 0.12
Nodes (22): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, ElevationRequired (+14 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.16
Nodes (7): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak(), revert_tweaks_by_id(), revert_tweaks_by_id_with()

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.17
Nodes (23): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+15 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.08
Nodes (21): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, ExplorerRestarted, FeatureSet, GpuMsi, PowerCfgSet (+13 more)

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.24
Nodes (15): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+7 more)

### Community 48 - "ui_scrollbar_test.rs"
Cohesion: 0.15
Nodes (19): display_reg_value_name(), Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, escape_win32_quoted_arg(), escape_xml() (+11 more)

### Community 49 - "engine.rs"
Cohesion: 0.10
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 50 - "test_wave1_ledger_rollback_and_concurrency_regressions"
Cohesion: 0.12
Nodes (18): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+10 more)

### Community 51 - "State Ledger & Restore Point Engine"
Cohesion: 0.23
Nodes (18): validate_registry_mutation_path(), normalize_reg_value_name(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node() (+10 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 55 - "RegRawSnapshot"
Cohesion: 0.14
Nodes (18): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+10 more)

### Community 56 - "ViewTextKey"
Cohesion: 0.16
Nodes (18): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), MAX_RETRIES, parse_pnputil_devices(), parse_pnputil_drivers(), PnpDeviceInfo (+10 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.12
Nodes (10): SilentCommand, ComponentThermal, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool, Hot (+2 more)

### Community 60 - "restore_point.rs"
Cohesion: 0.14
Nodes (4): MetricsSampler, optional_feature_matches(), SandboxMetricsSampler, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.18
Nodes (11): active_logical_core_count(), compute_nt_cpu_deltas(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor (+3 more)

### Community 62 - "Restore Points() .List"
Cohesion: 0.15
Nodes (11): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+3 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 64 - "path"
Cohesion: 0.16
Nodes (12): apply_parking_snapshot(), capture_parking_snapshot(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot, restore_stock_parking() (+4 more)

### Community 65 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.18
Nodes (14): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed() (+6 more)

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.18
Nodes (8): metric_pill(), view(), rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "WinBlast0 Architectural Specifications"
Cohesion: 0.14
Nodes (15): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+7 more)

### Community 69 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.16
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 70 - "LayoutTier"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 71 - "backend.rs"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 73 - ".Create Metrics Sampler()"
Cohesion: 0.17
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats(), CreateRestorePoint

### Community 75 - "tools/game_prereqs.rs"
Cohesion: 0.18
Nodes (4): compute_is_fully_unparked(), CoreParkingState, CoreStatus, test_core_parking_detect()

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 77 - "Installed Appx .List"
Cohesion: 0.19
Nodes (7): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, ThermalSnapshot, UpdateCheckCompleted

### Community 78 - ".setup"
Cohesion: 0.36
Nodes (9): Result, format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name() (+1 more)

### Community 79 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.41
Nodes (11): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_dropdown_anchoring_and_header_height_stability(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() (+3 more)

### Community 81 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 82 - ".sync_clean_apps_busy"
Cohesion: 0.24
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 83 - "normalize_guid_str"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 84 - "WinBlast0 Architectural Specifications"
Cohesion: 0.25
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 85 - "enums"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 87 - "Runtime"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 88 - "UiThemeStyle"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 89 - "SilentCommand"
Cohesion: 0.27
Nodes (4): RollbackRefreshData, list_power_schemes(), PowerSchemeInfo, test_power_scheme_enumeration_nonblocking()

### Community 90 - "latency_bench.rs"
Cohesion: 0.27
Nodes (3): encode_hex(), RegRawSnapshot, get_raw()

### Community 91 - "ThemePalette"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 92 - "ThermalHealth"
Cohesion: 0.36
Nodes (7): appx_package_matches_family(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches(), test_split_appx_name_and_publisher_with_underscores_and_prefix()

### Community 93 - "updater.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 94 - "ColorScheme"
Cohesion: 0.22
Nodes (8): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate

### Community 95 - "diff_modal.rs"
Cohesion: 0.33
Nodes (3): list_live_tasks_from_cache(), LiveScheduledTaskInfo, test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 101 - "Features() .List Optional"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 102 - ".List Live Services()"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 108 - "settle_frames"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 109 - "DriverError"
Cohesion: 0.29
Nodes (6): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown

### Community 110 - "DiskSubsystemReport"
Cohesion: 0.57
Nodes (6): full_telemetry_audit(), full_telemetry_audit_with(), get_blocked_hosts(), parse_blocked_hosts_from_content(), scan_telemetry_agents(), TelemetryAuditResult

### Community 111 - "PdhParkingCollector"
Cohesion: 0.29
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 114 - "progress_bar.rs"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 116 - "LiveScheduledTaskInfo"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 118 - "DriverProgress"
Cohesion: 0.33
Nodes (6): RiskLevel, Extreme, High, Low, Moderate, Safe

### Community 120 - "Spring"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 122 - "MatrixSortMode"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 123 - "UpdateStatus"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 124 - "DriverUpdateState"
Cohesion: 0.50
Nodes (4): StartupStatusFilter, ActiveOnly, All, InactiveOnly

### Community 125 - "GamingSubTab"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

## Knowledge Gaps
- **1113 isolated node(s):** `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc`, `ActiveRailDc`, `ActiveRam` (+1108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1606 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **45 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Privacy, Telemetry & AI Hardening Catalog`, `Reversibility Instant Requireslogoff`, `.Toggle .Set .Apply`, `Integration Suite: Ui Responsive E2E`, `Hardware Diagnostics & GPU Silicon Tuning`, `Process Token Information`, `enums`, `Hardware Diagnostics & GPU Silicon Tuning`, `Appx Package() Prefixes`, `GPU Silicon & Hardware Tuning Policies`, `updater.rs`?**
  _High betweenness centrality (0.303) - this node is a cross-community bridge._
- **Are the 22 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 22 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` to the rest of the system?**
  _1113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `Privacy, Telemetry & AI Hardening Catalog` to `sign_binary.ps1`, `AppState`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `Windows OS Settings & UX Customization`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `Test Composite With()`, `theme.rs`, `SlowScanBackend`, `View() Color Scrollspec`, `Hardware Diagnostics & GPU Silicon Tuning`, `Bloatware & AppX Deprovisioning Rules`, `Process Token Information`, `Core UI Internationalization & Text Catalog`, `Hardware Diagnostics & GPU Silicon Tuning`, `Multi-Language Tweak Localization Tables`, `System Architecture Changelog`, `GPU Silicon & Hardware Tuning Policies`, `Core UI Internationalization & Text Catalog`, `Task Scheduler Inspection Subsystem`, `AppX Package & Modern App Scanner`, `Embedded Catalog Test`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `Integration Suite: Ui Responsive E2E`, `ui_scrollbar_test.rs`, `Project History & Release Version Ledger`, `Error.Rs Winblasterror Appx`, `Startup Tasks & Auto-Run Explorer`, `Reversibility Instant Requireslogoff`, `WinBlast0 Architectural Specifications`, `LayoutTier`, `backend.rs`, `Project Documentation, Architecture & Licensing`, `tools/game_prereqs.rs`, `PlatformSecurityInfo`, `Installed Appx .List`, `Hardware Diagnostics & GPU Silicon Tuning`, `deserialize`, `test_wave1_compound_actions_deep_edge_cases`, `.sync_clean_apps_busy`, `WinBlast0 Architectural Specifications`, `SilentCommand`, `updater.rs`, `ColorScheme`, `diff_modal.rs`, `scan_class_registry`, `Core UI Internationalization & Text Catalog`, `Low-Level NT Core & Kernel Architecture`, `Manifest Asinvoker Build.Rs`, `Command Osstr Process.Rs`, `Integration Suite: Tweak Sandbox E2E`, `DiskSubsystemReport`, `PowerSchemeInfo`, `progress_bar.rs`, `tools/game_overlay.rs`, `LiveScheduledTaskInfo`, `goal_planner.rs`, `Spring`, `RegTreeSnapshot`, `MatrixSortMode`, `UpdateStatus`, `DriverUpdateState`, `make_ico.ps1`?**
  _High betweenness centrality (0.265) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.016129032258064516 - nodes in this community are weakly interconnected._
- **Why does `SystemBackend` connect `Integration Suite: Tweak Sandbox E2E` to `Privacy, Telemetry & AI Hardening Catalog`, `Disk Cleaning & Temporary File Purger`, `Test Composite With()`, `theme.rs`, `automation.rs`, `View() Color Scrollspec`, `full_telemetry_audit_with`, `Process Token Information`, `Multi-Language Tweak Localization Tables`, `Integration Suite: Tweak Sandbox E2E`, `.Reg State() Vendor()`, `AppX Package & Modern App Scanner`, `Embedded Catalog Test`, `FailAtMutationBackend`, `Optional Dism Feature()`, `RegRawSnapshot`, `Hardware Diagnostics & GPU Silicon Tuning`, `restore_point.rs`, `Restore Points() .List`, `Error.Rs Winblasterror Appx`, `path`, `Hardware Diagnostics & GPU Silicon Tuning`, `Project Documentation, Architecture & Licensing`, `.Create Metrics Sampler()`, `app.rs`, `tools/game_prereqs.rs`, `deserialize`, `test_wave1_compound_actions_deep_edge_cases`, `.sync_clean_apps_busy`, `SilentCommand`, `latency_bench.rs`, `diff_modal.rs`, `DriverCategory`, `Core UI Internationalization & Text Catalog`, `Manifest Asinvoker Build.Rs`, `Command Osstr Process.Rs`, `Integration Suite: Tweak Sandbox E2E`, `DiskSubsystemReport`, `goal_planner.rs`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._