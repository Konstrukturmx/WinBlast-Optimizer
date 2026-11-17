# Graph Report - WinBlast0  (2026-11-17)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3402 nodes · 7159 edges · 165 communities (114 shown, 51 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 292 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a27055ac`
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
- BlastResult
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
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
- LiveServiceInfo
- PowerSchemeInfo
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
- make_ico.ps1
- sign_binary.ps1
- .sync_startup_tasks_busy
- UpdateState
- StartupSource
- delete_value_for_vendor

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
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (165 total, 51 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.02
Nodes (124): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+116 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.02
Nodes (99): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+91 more)

### Community 3 - "AppState"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (40): RiskLevel, Extreme, High, Low, Moderate, Safe, assess_system_health(), AutoTunePlan (+32 more)

### Community 6 - "Windows OS Settings & UX Customization"
Cohesion: 0.07
Nodes (16): DiskSubsystemReport, extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback(), detect_platform_security_info(), NtQuerySystemInformation() (+8 more)

### Community 7 - "BlastResult"
Cohesion: 0.07
Nodes (45): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+37 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.06
Nodes (48): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, fmt1(), fmt2(), fmt3(), idx(), KFM_REDIRECT_MSG, localize_error_text() (+40 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.05
Nodes (6): check_elevation(), is_elevated(), broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 12 - "theme.rs"
Cohesion: 0.10
Nodes (14): view(), view(), view(), view(), view(), view(), view(), view() (+6 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 14 - "automation.rs"
Cohesion: 0.06
Nodes (4): AppState, RepairTask, before_after_comparison_card(), view()

### Community 15 - "View() Color Scrollspec"
Cohesion: 0.09
Nodes (19): AStarNode, AStarPlanner, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler, ExecutionOrder (+11 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.09
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 18 - "Bloatware & AppX Deprovisioning Rules"
Cohesion: 0.09
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 19 - "Process Token Information"
Cohesion: 0.07
Nodes (12): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, BaselineReset, CleanExecuted, ExplorerRestarted, LatencyBenchmarkCompleted (+4 more)

### Community 20 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.14
Nodes (34): escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry(), test_composite_string_merge_preserves_siblings(), test_hkcr_and_shallow_tree_delete_blocked() (+26 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.11
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.12
Nodes (22): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_startup_entries(), read_startup_approved_enabled(), replace_ascii_ci() (+14 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.17
Nodes (23): CachedScanBackend, RiskValuation, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan() (+15 more)

### Community 27 - "RealBackend"
Cohesion: 0.11
Nodes (14): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+6 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.09
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, PerfBenchmarkCompleted, RollbackCompleted, relaunch_as_admin()

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.08
Nodes (22): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.09
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 33 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.08
Nodes (4): fallback_scheme_name(), resolve_indirect_string(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.09
Nodes (11): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+3 more)

### Community 35 - "startup.rs"
Cohesion: 0.10
Nodes (20): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled, GpuMsiUpdated (+12 more)

### Community 36 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.25
Nodes (22): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, ThemePalette, build_appx_features_body(), build_disk_cleaner_body(), build_winget_body() (+14 more)

### Community 37 - "Disabled() Delete .Set"
Cohesion: 0.14
Nodes (20): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+12 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.11
Nodes (23): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+15 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.08
Nodes (27): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+19 more)

### Community 40 - ".Create Restore Point()"
Cohesion: 0.15
Nodes (17): StartupTasksSubTab, ScheduledTasks, Services, StartupApps, LayoutTier, Compact, Medium, Wide (+9 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 42 - "WinBlast0 Architectural Specifications"
Cohesion: 0.16
Nodes (7): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak(), rollback_by_id(), rollback_entry()

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.11
Nodes (22): Failed, UpdateDownloadCompleted, apply_and_restart(), check_latest_release(), download_release_asset(), FALLBACK_REPO, fetch_url_json(), GhAsset (+14 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.17
Nodes (23): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+15 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.11
Nodes (5): MetricsSampler, optional_feature_matches(), SandboxMetricsSampler, SystemMonitor, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.19
Nodes (18): validate_registry_mutation_path(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary() (+10 more)

### Community 48 - "ui_scrollbar_test.rs"
Cohesion: 0.14
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 49 - "engine.rs"
Cohesion: 0.08
Nodes (21): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, ExplorerRestarted, FeatureSet, GpuMsi, PowerCfgSet (+13 more)

### Community 50 - "test_wave1_ledger_rollback_and_concurrency_regressions"
Cohesion: 0.19
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 51 - "State Ledger & Restore Point Engine"
Cohesion: 0.13
Nodes (15): is_unsupported_power_guid_error(), apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1 (+7 more)

### Community 52 - "Optional Dism Feature()"
Cohesion: 0.10
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 53 - "localize_tweak"
Cohesion: 0.12
Nodes (6): SilentCommand, GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry, BenchmarkRunner, BenchmarkScore

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.14
Nodes (9): PING_TARGETS, PingTarget, probe_network_target(), TargetPingResult, test_probe_network_zero_probes(), DNS_CATALOG, DnsProbeResult, NetworkDiagnostics (+1 more)

### Community 55 - "RegRawSnapshot"
Cohesion: 0.24
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 57 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.16
Nodes (12): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), state_counts(), view() (+4 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.13
Nodes (12): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), set_scheduled_task_enabled(), test_hosts_blocking_ignores_comments_and_subdomains() (+4 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.12
Nodes (11): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, PlatformSecurityInfo (+3 more)

### Community 62 - "Restore Points() .List"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.15
Nodes (7): encode_hex(), RegRawSnapshot, RegTreeSnapshot, get_raw(), reg_type_to_u32(), snapshot_key_recursive(), snapshot_tree()

### Community 64 - "path"
Cohesion: 0.26
Nodes (10): sub_tab_chip(), view(), render_compatible_card(), render_incompatible_card(), view(), view(), action_button(), kv_row() (+2 more)

### Community 65 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.15
Nodes (7): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, FaultDiagnosticsReport, MinidumpEntry, PendingRebootStatus

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.29
Nodes (13): BeforeAfterImpact, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card(), optimization_categories_summary() (+5 more)

### Community 68 - "WinBlast0 Architectural Specifications"
Cohesion: 0.17
Nodes (12): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+4 more)

### Community 69 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.12
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 70 - "LayoutTier"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 71 - "backend.rs"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.17
Nodes (3): BiosSystemInfo, DisplayDriverInfo, GameProfile

### Community 73 - ".Create Metrics Sampler()"
Cohesion: 0.17
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats(), CreateRestorePoint

### Community 74 - "app.rs"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 75 - "tools/game_prereqs.rs"
Cohesion: 0.26
Nodes (8): active_logical_core_count(), compute_nt_cpu_deltas(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemProcessorPerformanceInformation

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.32
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 77 - "Installed Appx .List"
Cohesion: 0.23
Nodes (6): rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 78 - ".setup"
Cohesion: 0.22
Nodes (4): RollbackRefreshData, HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 79 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.18
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 80 - "deserialize"
Cohesion: 0.22
Nodes (11): decode_oem_bytes(), request_system_reboot(), curated_windows_features_meta(), dism_error_from_output(), list_optional_features(), parse_dism_english_state(), query_dism_capabilities_locked(), query_dism_features_locked() (+3 more)

### Community 81 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.17
Nodes (11): AppMode, Advanced, Easy, DriversSubTab, Drivers, Prerequisites, StagedTransition, StageApply (+3 more)

### Community 82 - ".sync_clean_apps_busy"
Cohesion: 0.21
Nodes (6): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 84 - "WinBlast0 Architectural Specifications"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 85 - "enums"
Cohesion: 0.41
Nodes (11): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_dropdown_anchoring_and_header_height_stability(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() (+3 more)

### Community 86 - "action_button"
Cohesion: 0.27
Nodes (7): enum_scm_services_by_type(), list_live_services(), query_scm_service_config(), query_scm_win32_services(), resolve_indirect_display_string(), ScHandleGuard, ScmServiceRecord

### Community 87 - "Runtime"
Cohesion: 0.25
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 88 - "UiThemeStyle"
Cohesion: 0.25
Nodes (3): list_live_tasks_from_cache(), LiveScheduledTaskInfo, test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()

### Community 89 - "SilentCommand"
Cohesion: 0.25
Nodes (5): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), run_full_latency_benchmark(), test_dpc_jitter_measurement()

### Community 90 - "latency_bench.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 91 - "ThemePalette"
Cohesion: 0.22
Nodes (4): SystemMetrics, SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 92 - "ThermalHealth"
Cohesion: 0.49
Nodes (7): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), tier_from_rect(), view()

### Community 93 - "updater.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 94 - "ColorScheme"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 96 - "DriverCategory"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.24
Nodes (7): ComponentThermal, SystemThermalSnapshot, ThermalHealth, Cool, Hot, Optimal, Warm

### Community 98 - "scan_class_registry"
Cohesion: 0.27
Nodes (3): GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck

### Community 99 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 101 - "Features() .List Optional"
Cohesion: 0.29
Nodes (3): ScheduledTaskRunResult, ServiceLiveUpdated, StartupEntryToggled

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.36
Nodes (3): RegStringValue, get_string(), get_string_for_vendor()

### Community 104 - "Manifest Asinvoker Build.Rs"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 105 - "Command Osstr Process.Rs"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 106 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 111 - "PdhParkingCollector"
Cohesion: 0.48
Nodes (3): OptimizationGoal, metric_pill(), view()

### Community 112 - "LiveServiceInfo"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 113 - "PowerSchemeInfo"
Cohesion: 0.57
Nodes (6): full_telemetry_audit(), full_telemetry_audit_with(), get_blocked_hosts(), parse_blocked_hosts_from_content(), scan_telemetry_agents(), TelemetryAuditResult

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 116 - "LiveScheduledTaskInfo"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 117 - "goal_planner.rs"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 120 - "Spring"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 121 - "RegTreeSnapshot"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

### Community 122 - "MatrixSortMode"
Cohesion: 0.40
Nodes (5): Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot

### Community 123 - "UpdateStatus"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 124 - "DriverUpdateState"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

### Community 125 - "GamingSubTab"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 126 - "PrivacySortMode"
Cohesion: 0.50
Nodes (4): PrereqsSortMode, Category, MissingFirst, NameAsc

### Community 127 - "make_ico.ps1"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 128 - "sign_binary.ps1"
Cohesion: 0.50
Nodes (4): StartupStatusFilter, ActiveOnly, All, InactiveOnly

### Community 129 - ".sync_startup_tasks_busy"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

### Community 130 - "UpdateState"
Cohesion: 0.67
Nodes (3): EngineMode, Instant, Normal

### Community 131 - "StartupSource"
Cohesion: 0.67
Nodes (3): RepairSubTab, FaultDiagnostics, WindowsRepair

## Knowledge Gaps
- **1113 isolated node(s):** `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc`, `ActiveRailDc`, `ActiveRam` (+1108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1596 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **51 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `path`, `Project Documentation, Architecture & Licensing`, `spawn_guarded_worker`, `Reversibility Instant Requireslogoff`, `Integration Suite: Tweak Sandbox E2E`, `Core UI Internationalization & Text Catalog`, `Integration Suite: Tweak Sandbox E2E`, `.Create Restore Point()`, `theme.rs`, `automation.rs`, `test_wave1_compound_actions_deep_edge_cases`, `PresetKey`, `Hardware Diagnostics & GPU Silicon Tuning`, `latency_bench.rs`, `ThermalHealth`?**
  _High betweenness centrality (0.304) - this node is a cross-community bridge._
- **Are the 18 inferred relationships involving `AppState` (e.g. with `OptimizationPlan` and `ExecutionOrder`) actually correct?**
  _`AppState` has 18 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` to the rest of the system?**
  _1113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `automation.rs` to `Privacy, Telemetry & AI Hardening Catalog`, `AppState`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `Windows OS Settings & UX Customization`, `BlastResult`, `theme.rs`, `SlowScanBackend`, `View() Color Scrollspec`, `full_telemetry_audit_with`, `Bloatware & AppX Deprovisioning Rules`, `Process Token Information`, `Hardware Diagnostics & GPU Silicon Tuning`, `Multi-Language Tweak Localization Tables`, `System Architecture Changelog`, `.Reg State() Vendor()`, `Core UI Internationalization & Text Catalog`, `spawn_guarded_worker`, `startup.rs`, `Integration Suite: Tweak Sandbox E2E`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `WinBlast0 Architectural Specifications`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `test_wave1_ledger_rollback_and_concurrency_regressions`, `localize_tweak`, `Project History & Release Version Ledger`, `Hardware Diagnostics & GPU Silicon Tuning`, `task_scanner.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `Restore Points() .List`, `path`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `Reversibility Instant Requireslogoff`, `Hardware Diagnostics & GPU Silicon Tuning`, `LayoutTier`, `backend.rs`, `Project Documentation, Architecture & Licensing`, `app.rs`, `.setup`, `Hardware Diagnostics & GPU Silicon Tuning`, `test_wave1_compound_actions_deep_edge_cases`, `.sync_clean_apps_busy`, `normalize_guid_str`, `enums`, `Runtime`, `UiThemeStyle`, `SilentCommand`, `ThemePalette`, `ThermalHealth`, `diff_modal.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `scan_class_registry`, `Core UI Internationalization & Text Catalog`, `Features() .List Optional`, `.List Live Services()`, `Command Osstr Process.Rs`, `Integration Suite: Tweak Sandbox E2E`, `WindowsInfo`, `settle_frames`, `DriverError`, `PdhParkingCollector`, `PowerSchemeInfo`, `tools/game_overlay.rs`, `LiveScheduledTaskInfo`, `goal_planner.rs`, `DriverProgress`, `PresetKey`, `Spring`, `RegTreeSnapshot`, `UpdateStatus`, `DriverUpdateState`, `PrivacySortMode`, `make_ico.ps1`, `sign_binary.ps1`, `UpdateState`, `StartupSource`?**
  _High betweenness centrality (0.266) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.016129032258064516 - nodes in this community are weakly interconnected._
- **Why does `SystemBackend` connect `Disk Cleaning & Temporary File Purger` to `AppState`, `SlowScanBackend`, `automation.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `full_telemetry_audit_with`, `Bloatware & AppX Deprovisioning Rules`, `Core UI Internationalization & Text Catalog`, `Integration Suite: Tweak Sandbox E2E`, `System Architecture Changelog`, `.Reg State() Vendor()`, `Appx Package() Prefixes`, `RealBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `AppX Package & Modern App Scanner`, `svc_mutator.rs`, `ui_scrollbar_test.rs`, `State Ledger & Restore Point Engine`, `State Ledger & Restore Point Engine`, `restore_point.rs`, `Hardware Diagnostics & GPU Silicon Tuning`, `Error.Rs Winblasterror Appx`, `WinBlast0 Architectural Specifications`, `backend.rs`, `.Create Metrics Sampler()`, `app.rs`, `.setup`, `Hardware Diagnostics & GPU Silicon Tuning`, `test_wave1_compound_actions_deep_edge_cases`, `normalize_guid_str`, `UiThemeStyle`, `SilentCommand`, `.List Live Services()`, `Low-Level NT Core & Kernel Architecture`, `WindowsInfo`, `settle_frames`, `DriverError`, `PowerSchemeInfo`, `DriverProgress`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._