# Graph Report - WinBlast0  (2026-11-19)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 3735 nodes · 7722 edges · 162 communities (108 shown, 54 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 313 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `94374692`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- action_button_guarded
- Privacy, Telemetry & AI Hardening Catalog
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
- spawn_guarded_worker
- LazyLoader
- deserialize
- Hardware Diagnostics & GPU Silicon Tuning
- Multi-Language Tweak Localization Tables
- PanickingBackend
- System Architecture Changelog
- .Reg State() Vendor()
- Appx Package() Prefixes
- RealBackend
- GPU Silicon & Hardware Tuning Policies
- Core UI Internationalization & Text Catalog
- Win32 Registry Scanner & Mutator
- driver_update.rs
- power.rs
- spawn_guarded_worker
- startup.rs
- registry_scanner.rs
- elevation.rs
- Task Scheduler Inspection Subsystem
- AppX Package & Modern App Scanner
- Changelog
- Embedded Catalog Test
- ps_runner.rs
- FailAtMutationBackend
- Hardware Diagnostics & GPU Silicon Tuning
- Dynamicdwordformula Logicalcorecount Totalraminkb
- svc_mutator.rs
- Integration Suite: Ui Responsive E2E
- ui_scrollbar_test.rs
- generational_cache.rs
- test_wave1_ledger_rollback_and_concurrency_regressions
- heuristic_scanner.rs
- .sync_startup_tasks_busy
- Project History & Release Version Ledger
- RegRawSnapshot
- ViewTextKey
- .set_status_spec
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- Hardware Diagnostics & GPU Silicon Tuning
- lazy_loader.rs
- Error.Rs Winblasterror Appx
- decode_oem_bytes
- task_scanner.rs
- Startup Tasks & Auto-Run Explorer
- Reversibility Instant Requireslogoff
- GpuVendor
- path
- TaskCache
- backend.rs
- Project Documentation, Architecture & Licensing
- restore_point.rs
- app.rs
- query_nt_processor_performance_into
- PlatformSecurityInfo
- Installed Appx .List
- .setup
- Hardware Diagnostics & GPU Silicon Tuning
- deserialize
- WinBlast Optimizer — Developer & Architecture Guide
- HibernationStatus
- normalize_guid_str
- StartupEntry
- enums
- SandboxEnv
- Runtime
- UiThemeStyle
- restart_explorer_safely
- latency_bench.rs
- ThemePalette
- edge_cases_unorthodox.rs
- ColorScheme
- LiveScheduledTaskInfo
- RegTreeSnapshot
- Hardware Diagnostics & GPU Silicon Tuning
- real_windows_env.rs
- Core UI Internationalization & Text Catalog
- Project History & Release Version Ledger
- LiveServiceInfo
- instant
- Low-Level NT Core & Kernel Architecture
- .navigate
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
- LiveScheduledTaskInfo
- test_wave1_ledger_rollback_and_concurrency_regressions
- view_i18n.rs
- PresetKey
- BiosSystemInfo
- CleanExecutionReport
- MatrixSortMode
- UpdateStatus
- DriverUpdateState
- GamingSubTab
- ui/mod.rs
- .sync_startup_tasks_busy
- goal_planner.rs
- HardwareSubTab
- action_button_guarded
- DriverSortMode
- PowerActionKind
- PrereqsSortMode
- .poll
- StagedTransition
- LiveServiceInfo
- diff_modal.rs
- StartupSource

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 347 edges
3. `ViewTextKey` - 220 edges
4. `SystemBackend` - 118 edges
5. `SandboxEnv` - 115 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 87 edges
8. `SlowScanBackend` - 85 edges
9. `RealBackend` - 84 edges
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

## Communities (162 total, 54 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.04
Nodes (6): AppState, ApplyCompleted, RepairTask, localize_dns_category(), localize_dns_status(), view()

### Community 3 - "AppState"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.10
Nodes (22): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+14 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.06
Nodes (31): CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp, WerAndCrashDumps (+23 more)

### Community 6 - "sys_inventory.rs"
Cohesion: 0.06
Nodes (25): DiskSubsystemReport, extract_null_terminated_str(), HandleGuard, PhysicalDriveHealth, probe_physical_drive(), probe_volume_fallback(), query_disk_capacity_ioctl(), query_nvme_smart_ioctl() (+17 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.06
Nodes (40): RiskLevel, Extreme, High, Low, Moderate, Safe, assess_system_health(), AutoTunePlan (+32 more)

### Community 11 - "startup.rs"
Cohesion: 0.20
Nodes (18): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services(), list_startup_entries() (+10 more)

### Community 12 - "theme.rs"
Cohesion: 0.07
Nodes (22): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+14 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.07
Nodes (20): AStarNode, AStarPlanner, BitSet256, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler (+12 more)

### Community 14 - ".take_injected_error"
Cohesion: 0.12
Nodes (3): optional_feature_matches(), SetHags, SetMpo

### Community 15 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.10
Nodes (23): build_circular_radar(), build_mini_progress_bar(), CARD_PADDING, CARD_RADIUS, CARD_WIDTH, compute_dot_alpha(), DEFAULT_STATUS_MESSAGE, dot_alpha_stays_within_expected_bounds() (+15 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.12
Nodes (39): is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry() (+31 more)

### Community 18 - "spawn_guarded_worker"
Cohesion: 0.07
Nodes (12): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, ExplorerRestarted, LatencyBenchmarkCompleted (+4 more)

### Community 19 - "LazyLoader"
Cohesion: 0.16
Nodes (9): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), LazyLoader, LazyPayload, Bios, Disks, Drivers, Thermals (+1 more)

### Community 20 - "deserialize"
Cohesion: 0.07
Nodes (12): FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus, GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck, GameProfile (+4 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.17
Nodes (26): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+18 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.15
Nodes (13): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DevInfoGuard, DISPLAY_CLASS_PREFIX, has_ascii_token(), parse_target_vendor() (+5 more)

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
Cohesion: 0.08
Nodes (13): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+5 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.09
Nodes (21): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled, GpuMsiUpdated (+13 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (23): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+15 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.12
Nodes (22): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+14 more)

### Community 33 - "power.rs"
Cohesion: 0.05
Nodes (37): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CoreParkingState, CoreStatus, CPMAXCORES, CPMAXCORES1, CPMINCORES (+29 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.08
Nodes (33): AI_BLOAT_PREFIXES, AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, check_onedrive_kfm_active(), classify_appx_package() (+25 more)

### Community 35 - "startup.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.05
Nodes (72): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+64 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.14
Nodes (25): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+17 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.08
Nodes (27): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+19 more)

### Community 40 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.23
Nodes (23): CachedScanBackend, RiskValuation, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan() (+15 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.21
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.18
Nodes (5): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), rollback_entry()

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.13
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.25
Nodes (15): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+7 more)

### Community 48 - "ui_scrollbar_test.rs"
Cohesion: 0.15
Nodes (19): display_reg_value_name(), Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, escape_win32_quoted_arg(), escape_xml() (+11 more)

### Community 49 - "generational_cache.rs"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 50 - "test_wave1_ledger_rollback_and_concurrency_regressions"
Cohesion: 0.12
Nodes (18): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+10 more)

### Community 51 - "heuristic_scanner.rs"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 52 - ".sync_startup_tasks_busy"
Cohesion: 0.29
Nodes (3): ScheduledTaskRunResult, ServiceLiveUpdated, StartupEntryToggled

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 55 - "RegRawSnapshot"
Cohesion: 0.15
Nodes (14): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+6 more)

### Community 56 - "ViewTextKey"
Cohesion: 0.05
Nodes (63): backup_driver(), backup_root(), compare_versions(), create_sandbox_test_item(), download_and_install_driver(), download_and_install_driver_with_progress(), DriverCategory, Audio (+55 more)

### Community 57 - ".set_status_spec"
Cohesion: 0.08
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, BaselineReset, PerfBenchmarkCompleted, RestorePointCreated

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 60 - "backend.rs"
Cohesion: 0.09
Nodes (6): MetricsSampler, SandboxMetricsSampler, SandboxTempDir, SystemMonitor, test_sandbox_core_parking_labels_metrics_and_power_sync(), CpuInfo

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.30
Nodes (7): active_logical_core_count(), NtCpuSampler, query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 64 - "decode_oem_bytes"
Cohesion: 0.25
Nodes (10): set_scheduled_task_enabled(), decode_oem_bytes(), request_system_reboot(), dism_error_from_output(), list_optional_features(), parse_dism_english_state(), query_dism_capabilities_locked(), query_dism_features_locked() (+2 more)

### Community 65 - "task_scanner.rs"
Cohesion: 0.20
Nodes (13): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+5 more)

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.18
Nodes (8): before_after_comparison_card(), view(), rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "GpuVendor"
Cohesion: 0.22
Nodes (7): DetectedGpu, GpuVendor, Amd, Intel, Nvidia, Unknown, primary_vendor()

### Community 69 - "path"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 71 - "backend.rs"
Cohesion: 0.28
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 73 - "restore_point.rs"
Cohesion: 0.25
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 74 - "app.rs"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 75 - "query_nt_processor_performance_into"
Cohesion: 0.31
Nodes (7): compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), SystemProcessorPerformanceInformation, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 77 - "Installed Appx .List"
Cohesion: 0.21
Nodes (6): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 78 - ".setup"
Cohesion: 0.56
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 79 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.41
Nodes (11): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_dropdown_anchoring_and_header_height_stability(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() (+3 more)

### Community 81 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 82 - "HibernationStatus"
Cohesion: 0.14
Nodes (5): RollbackRefreshData, HibernationStatus, PowerSchemeInfo, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 83 - "normalize_guid_str"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 85 - "enums"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 86 - "SandboxEnv"
Cohesion: 0.08
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

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.12
Nodes (6): EngineMode, Instant, Normal, test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports()

### Community 94 - "ColorScheme"
Cohesion: 0.22
Nodes (8): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate

### Community 95 - "LiveScheduledTaskInfo"
Cohesion: 0.32
Nodes (3): list_live_tasks_from_cache(), LiveScheduledTaskInfo, test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 98 - "real_windows_env.rs"
Cohesion: 0.09
Nodes (6): SilentCommand, FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 104 - ".navigate"
Cohesion: 0.14
Nodes (4): BiosInfoLoaded, DiskReportLoaded, DriverScanCompleted, ThermalSnapshot

### Community 109 - "DriverError"
Cohesion: 0.29
Nodes (6): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown

### Community 110 - "app.rs"
Cohesion: 0.16
Nodes (17): AppMode, Advanced, Easy, DriversSubTab, Drivers, Prerequisites, RepairSubTab, FaultDiagnostics (+9 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 116 - "LiveScheduledTaskInfo"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 118 - "view_i18n.rs"
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

### Community 128 - "ui/mod.rs"
Cohesion: 0.58
Nodes (6): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), view()

### Community 130 - "goal_planner.rs"
Cohesion: 0.48
Nodes (3): OptimizationGoal, metric_pill(), view()

### Community 146 - "HardwareSubTab"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

### Community 147 - "action_button_guarded"
Cohesion: 0.13
Nodes (25): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), render_compatible_card(), render_incompatible_card() (+17 more)

### Community 149 - "DriverSortMode"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

### Community 150 - "PowerActionKind"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 151 - "PrereqsSortMode"
Cohesion: 0.50
Nodes (4): PrereqsSortMode, Category, MissingFirst, NameAsc

### Community 153 - "StagedTransition"
Cohesion: 0.67
Nodes (3): StagedTransition, StageApply, StageRevert

### Community 154 - "LiveServiceInfo"
Cohesion: 0.39
Nodes (5): enum_scm_services_by_type(), query_scm_service_config(), query_scm_win32_services(), ScHandleGuard, ScmServiceRecord

### Community 155 - "diff_modal.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 157 - "StartupSource"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

## Knowledge Gaps
- **1199 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` (+1194 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1777 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **54 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Privacy, Telemetry & AI Hardening Catalog` to `ui/mod.rs`, `goal_planner.rs`, `AppState`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `sys_inventory.rs`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `theme.rs`, `SlowScanBackend`, `spawn_guarded_worker`, `HardwareSubTab`, `deserialize`, `Hardware Diagnostics & GPU Silicon Tuning`, `DriverSortMode`, `PrereqsSortMode`, `System Architecture Changelog`, `.poll`, `StagedTransition`, `action_button_guarded`, `diff_modal.rs`, `GPU Silicon & Hardware Tuning Policies`, `Core UI Internationalization & Text Catalog`, `power.rs`, `spawn_guarded_worker`, `Task Scheduler Inspection Subsystem`, `AppX Package & Modern App Scanner`, `Changelog`, `Embedded Catalog Test`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`, `Integration Suite: Ui Responsive E2E`, `ui_scrollbar_test.rs`, `.sync_startup_tasks_busy`, `Project History & Release Version Ledger`, `ViewTextKey`, `.set_status_spec`, `State Ledger & Restore Point Engine`, `backend.rs`, `Error.Rs Winblasterror Appx`, `Startup Tasks & Auto-Run Explorer`, `Reversibility Instant Requireslogoff`, `GpuVendor`, `backend.rs`, `PlatformSecurityInfo`, `Installed Appx .List`, `Hardware Diagnostics & GPU Silicon Tuning`, `deserialize`, `WinBlast Optimizer — Developer & Architecture Guide`, `HibernationStatus`, `StartupEntry`, `edge_cases_unorthodox.rs`, `ColorScheme`, `LiveScheduledTaskInfo`, `Core UI Internationalization & Text Catalog`, `LiveServiceInfo`, `instant`, `Low-Level NT Core & Kernel Architecture`, `.navigate`, `Command Osstr Process.Rs`, `PlatformSecurityInfo`, `RestorePoint`, `app.rs`, `SystemMetrics`, `progress_bar.rs`, `tools/game_overlay.rs`, `LiveScheduledTaskInfo`, `view_i18n.rs`, `BiosSystemInfo`, `CleanExecutionReport`, `MatrixSortMode`, `UpdateStatus`, `DriverUpdateState`?**
  _High betweenness centrality (0.296) - this node is a cross-community bridge._
- **Are the 25 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 25 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges` to the rest of the system?**
  _1199 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `ui/mod.rs`, `Privacy, Telemetry & AI Hardening Catalog`, `Reversibility Instant Requireslogoff`, `.Toggle .Set .Apply`, `Startup Tasks & Auto-Run Explorer`, `app.rs`, `Integration Suite: Ui Responsive E2E`, `action_button_guarded`, `enums`, `Hardware Diagnostics & GPU Silicon Tuning`, `Appx Package() Prefixes`, `diff_modal.rs`, `GPU Silicon & Hardware Tuning Policies`?**
  _High betweenness centrality (0.276) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `ui/mod.rs`, `goal_planner.rs`, `Privacy, Telemetry & AI Hardening Catalog`, `.Toggle .Set .Apply`, `Integration Suite: Ui Responsive E2E`, `action_button_guarded`, `view_i18n.rs`?**
  _High betweenness centrality (0.121) - this node is a cross-community bridge._