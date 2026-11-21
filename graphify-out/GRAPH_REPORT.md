# Graph Report - WinBlast0  (2026-11-22)

## Corpus Check
- 153 files · ~298,002 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 4126 nodes · 8683 edges · 184 communities (137 shown, 47 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 324 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6
- Community 7
- Community 8
- Community 10
- Community 11
- Community 12
- Community 13
- Community 14
- Community 15
- Community 16
- Community 17
- Community 18
- Community 19
- Community 20
- Community 21
- Community 22
- Community 23
- Community 24
- Community 25
- Community 27
- Community 28
- Community 29
- Community 30
- Community 31
- Community 32
- Community 33
- Community 34
- Community 35
- Community 36
- Community 37
- Community 38
- Community 39
- Community 40
- Community 41
- Community 42
- Community 43
- Community 44
- Community 45
- Community 46
- Community 47
- Community 48
- Community 49
- Community 50
- Community 51
- Community 52
- Community 53
- Community 54
- Community 55
- Community 56
- Community 57
- Community 58
- Community 59
- Community 60
- Community 61
- Community 62
- Community 63
- Community 64
- Community 65
- Community 66
- Community 67
- Community 68
- Community 69
- Community 70
- Community 71
- Community 72
- Community 73
- Community 74
- Community 75
- Community 76
- Community 77
- Community 78
- Community 79
- Community 80
- Community 81
- Community 82
- Community 83
- Community 84
- Community 85
- Community 86
- Community 87
- Community 88
- Community 89
- Community 90
- Community 91
- Community 92
- Community 93
- Community 94
- Community 95
- Community 96
- Community 97
- Community 98
- Community 99
- Community 100
- Community 101
- Community 102
- Community 103
- Community 104
- Community 105
- Community 106
- Community 107
- Community 108
- Community 109
- Community 110
- Community 111
- Community 112
- Community 113
- Community 114
- Community 115
- Community 116
- Community 117
- Community 118
- Community 119
- Community 120
- Community 121
- Community 122
- Community 123
- Community 124
- Community 125
- Community 126
- Community 127
- Community 128
- Community 129
- Community 130
- Community 131
- Community 132
- Community 133
- Community 134
- Community 135
- Community 136
- Community 137
- Community 138
- Community 139
- Community 140
- Community 141
- Community 143
- Community 144
- Community 145
- Community 146
- Community 147
- Community 148
- Community 149
- Community 150
- Community 151
- Community 152
- Community 153
- Community 154
- Community 155
- Community 156
- Community 157
- Community 158
- Community 159
- Community 160
- Community 161
- Community 162
- Community 163
- Community 164
- Community 165
- Community 166
- Community 168
- Community 170

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 366 edges
3. `ViewTextKey` - 220 edges
4. `SandboxEnv` - 126 edges
5. `SystemBackend` - 124 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `WorkerEvent` - 85 edges
9. `SlowScanBackend` - 85 edges
10. `RealBackend` - 84 edges

## Surprising Connections (you probably didn't know these)
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs

## Import Cycles
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (184 total, 47 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "Community 1"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "Community 2"
Cohesion: 0.02
Nodes (99): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+91 more)

### Community 3 - "Community 3"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - "Community 4"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Community 5"
Cohesion: 0.04
Nodes (8): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, localize_dns_category(), localize_dns_status(), view()

### Community 6 - "Community 6"
Cohesion: 0.04
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats(), SystemBackend

### Community 7 - "Community 7"
Cohesion: 0.10
Nodes (22): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+14 more)

### Community 10 - "Community 10"
Cohesion: 0.10
Nodes (43): StateLedger, is_unsupported_power_guid_error(), Result, decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs() (+35 more)

### Community 11 - "Community 11"
Cohesion: 0.07
Nodes (21): AStarNode, AStarPlanner, BitSet256, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler (+13 more)

### Community 12 - "Community 12"
Cohesion: 0.06
Nodes (47): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, fmt1(), fmt2(), fmt3(), idx(), KFM_REDIRECT_MSG, localize_error_text() (+39 more)

### Community 13 - "Community 13"
Cohesion: 0.07
Nodes (45): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+37 more)

### Community 14 - "Community 14"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "Community 15"
Cohesion: 0.09
Nodes (20): create_and_activate_performance_plan(), fallback_scheme_name(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str() (+12 more)

### Community 16 - "Community 16"
Cohesion: 0.08
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 17 - "Community 17"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "Community 19"
Cohesion: 0.11
Nodes (21): assert_valid_permutation(), BitSet256, BitSetIter, find_min_weight_cycle_edge(), kahn_schedule(), position_of(), resolve_cycles(), SccResult (+13 more)

### Community 20 - "Community 20"
Cohesion: 0.08
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, LatencyBenchmarkCompleted, RollbackCompleted

### Community 21 - "Community 21"
Cohesion: 0.14
Nodes (25): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), render_compatible_card(), render_incompatible_card() (+17 more)

### Community 22 - "Community 22"
Cohesion: 0.07
Nodes (12): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, BaselineReset, CleanExecuted, CleanTargetsScanned, ExplorerRestarted (+4 more)

### Community 23 - "Community 23"
Cohesion: 0.07
Nodes (32): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate (+24 more)

### Community 24 - "Community 24"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 25 - "Community 25"
Cohesion: 0.15
Nodes (27): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+19 more)

### Community 27 - "Community 27"
Cohesion: 0.15
Nodes (21): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+13 more)

### Community 28 - "Community 28"
Cohesion: 0.11
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Community 29"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "Community 30"
Cohesion: 0.11
Nodes (25): encode_hex(), check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_dword(), get_dword_for_vendor() (+17 more)

### Community 31 - "Community 31"
Cohesion: 0.07
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 32 - "Community 32"
Cohesion: 0.07
Nodes (11): RealBackend, resolve_persistent_app_dir(), is_hags_enabled(), is_mpo_disabled(), set_gpu_msi_mode(), set_hags(), set_mpo_disabled(), set_usb_root_hub_power_saving() (+3 more)

### Community 33 - "Community 33"
Cohesion: 0.10
Nodes (17): CleanTargetResult, ComStaGuard, delete_dir_contents(), dir_meta_if_real(), empty_recycle_bin(), ERROR_LOCK_VIOLATION, ERROR_SHARING_VIOLATION, ERROR_USER_MAPPED_FILE (+9 more)

### Community 34 - "Community 34"
Cohesion: 0.12
Nodes (3): optional_feature_matches(), SetHags, SetMpo

### Community 35 - "Community 35"
Cohesion: 0.08
Nodes (21): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled, GpuMsiUpdated (+13 more)

### Community 36 - "Community 36"
Cohesion: 0.17
Nodes (28): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+20 more)

### Community 38 - "Community 38"
Cohesion: 0.09
Nodes (12): build_populated_sandbox_state(), init_cpu_parking_state(), init_dashboard_state(), init_fault_diagnostics_state(), init_gpu_state(), init_matrix_state(), init_minimalistic_theme_state(), init_support_about_state() (+4 more)

### Community 39 - "Community 39"
Cohesion: 0.12
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 40 - "Community 40"
Cohesion: 0.13
Nodes (20): detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor, Amd (+12 more)

### Community 41 - "Community 41"
Cohesion: 0.08
Nodes (22): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 42 - "Community 42"
Cohesion: 0.09
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 43 - "Community 43"
Cohesion: 0.12
Nodes (6): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, SystemBaseline, test_ledger_crud()

### Community 44 - "Community 44"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 45 - "Community 45"
Cohesion: 0.13
Nodes (23): AI_BLOAT_PREFIXES, classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), is_appx_arch(), is_appx_version() (+15 more)

### Community 46 - "Community 46"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 47 - "Community 47"
Cohesion: 0.13
Nodes (15): delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary(), set_dword_for_vendor() (+7 more)

### Community 49 - "Community 49"
Cohesion: 0.11
Nodes (9): test_windows_info_detect_host(), WindowsInfo, BiosSystemInfo, DisplayDriverInfo, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus (+1 more)

### Community 50 - "Community 50"
Cohesion: 0.14
Nodes (17): build_signing_transcript(), compute_sha256(), OverrideKey, U64, PresetGuards, PresetMetadata, PresetPackage, sample_metadata() (+9 more)

### Community 52 - "Community 52"
Cohesion: 0.15
Nodes (17): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+9 more)

### Community 53 - "Community 53"
Cohesion: 0.11
Nodes (4): MetricsSampler, SandboxMetricsSampler, SandboxTempDir, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 54 - "Community 54"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 55 - "Community 55"
Cohesion: 0.14
Nodes (8): rgba(), UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 56 - "Community 56"
Cohesion: 0.16
Nodes (9): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), LazyLoader, LazyPayload, Bios, Disks, Drivers, Thermals (+1 more)

### Community 57 - "Community 57"
Cohesion: 0.10
Nodes (22): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+14 more)

### Community 58 - "Community 58"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 59 - "Community 59"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 60 - "Community 60"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 61 - "Community 61"
Cohesion: 0.15
Nodes (18): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), MAX_RETRIES, parse_pnputil_devices(), parse_pnputil_drivers(), PnpDeviceInfo (+10 more)

### Community 62 - "Community 62"
Cohesion: 0.10
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 63 - "Community 63"
Cohesion: 0.10
Nodes (21): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+13 more)

### Community 64 - "Community 64"
Cohesion: 0.14
Nodes (13): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+5 more)

### Community 65 - "Community 65"
Cohesion: 0.15
Nodes (18): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+10 more)

### Community 66 - "Community 66"
Cohesion: 0.10
Nodes (12): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme (+4 more)

### Community 67 - "Community 67"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "Community 68"
Cohesion: 0.15
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 69 - "Community 69"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 70 - "Community 70"
Cohesion: 0.11
Nodes (16): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+8 more)

### Community 71 - "Community 71"
Cohesion: 0.11
Nodes (6): EngineMode, Instant, Normal, StagedTransition, StageApply, StageRevert

### Community 72 - "Community 72"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "Community 73"
Cohesion: 0.13
Nodes (11): AutoTuneCompleted, AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult (+3 more)

### Community 74 - "Community 74"
Cohesion: 0.18
Nodes (8): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), set_startup_entry_enabled(), set_startup_entry_enabled_by_id(), StartupEntry

### Community 75 - "Community 75"
Cohesion: 0.16
Nodes (11): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+3 more)

### Community 76 - "Community 76"
Cohesion: 0.23
Nodes (7): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, test_corrupted_magic_rejection()

### Community 77 - "Community 77"
Cohesion: 0.16
Nodes (16): DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), probe_physical_drive(), query_ata_smart_ioctl() (+8 more)

### Community 78 - "Community 78"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "Community 79"
Cohesion: 0.18
Nodes (14): DriversSubTab, Drivers, Prerequisites, RepairSubTab, FaultDiagnostics, WindowsRepair, full_telemetry_audit(), full_telemetry_audit_with() (+6 more)

### Community 80 - "Community 80"
Cohesion: 0.22
Nodes (10): install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind, DailyMemoryPurge, TelemetryGuard, WeeklyTempCleaner, MaintenanceTaskStatus, query_all_maintenance_tasks() (+2 more)

### Community 81 - "Community 81"
Cohesion: 0.21
Nodes (12): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed() (+4 more)

### Community 83 - "Community 83"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 84 - "Community 84"
Cohesion: 0.20
Nodes (10): init_test_env(), test_all_26_views_navigation_and_mounting(), test_global_keyboard_shortcuts_and_actions(), test_main_window_properties_and_callbacks(), test_modals_state_and_lifecycle(), test_motion_constants_monotonicity(), test_scale_token_bounds_and_typography_ramp(), test_statusbar_telemetry_properties() (+2 more)

### Community 85 - "Community 85"
Cohesion: 0.25
Nodes (11): AppMode, Advanced, Easy, localize_ledger_action(), build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button() (+3 more)

### Community 86 - "Community 86"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 87 - "Community 87"
Cohesion: 0.21
Nodes (12): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+4 more)

### Community 88 - "Community 88"
Cohesion: 0.18
Nodes (8): detect_platform_security_info(), PlatformSecurityInfo, query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), test_ram_and_platform_inventory_nonblocking(), total_physical_ram_kb()

### Community 89 - "Community 89"
Cohesion: 0.18
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 91 - "Community 91"
Cohesion: 0.23
Nodes (10): active_logical_core_count(), compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases() (+2 more)

### Community 92 - "Community 92"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "Community 93"
Cohesion: 0.23
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "Community 94"
Cohesion: 0.14
Nodes (4): BiosInfoLoaded, DiskReportLoaded, DriverScanCompleted, ThermalSnapshot

### Community 96 - "Community 96"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 97 - "Community 97"
Cohesion: 0.23
Nodes (10): create_sandbox_test_item(), download_and_install_driver(), DriverProgressCallback, DriverUpdateItem, scan_class_registry(), scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata() (+2 more)

### Community 98 - "Community 98"
Cohesion: 0.21
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 99 - "Community 99"
Cohesion: 0.19
Nodes (7): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, list_installed_appx_from_cache()

### Community 100 - "Community 100"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - "Community 101"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 104 - "Community 104"
Cohesion: 0.18
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 105 - "Community 105"
Cohesion: 0.21
Nodes (6): LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 106 - "Community 106"
Cohesion: 0.24
Nodes (4): RollbackRefreshData, HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "Community 108"
Cohesion: 0.24
Nodes (10): assess_system_health(), HealthSeverity, Critical, Info, Warning, MemoryTrimResult, SystemHealthAssessment, SystemHealthIssue (+2 more)

### Community 109 - "Community 109"
Cohesion: 0.41
Nodes (11): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), click_key(), click_text(), drain_state_workers(), settle_frames(), test_ui_dropdown_anchoring_and_header_height_stability(), test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() (+3 more)

### Community 110 - "Community 110"
Cohesion: 0.31
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 112 - "Community 112"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "Community 113"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 114 - "Community 114"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "Community 115"
Cohesion: 0.47
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 116 - "Community 116"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 117 - "Community 117"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "Community 118"
Cohesion: 0.27
Nodes (3): GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck

### Community 119 - "Community 119"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 120 - "Community 120"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 122 - "Community 122"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "Community 123"
Cohesion: 0.22
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 126 - "Community 126"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 127 - "Community 127"
Cohesion: 0.58
Nodes (6): rgb(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 128 - "Community 128"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 131 - "Community 131"
Cohesion: 0.32
Nodes (3): GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry

### Community 132 - "Community 132"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 133 - "Community 133"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 134 - "Community 134"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 135 - "Community 135"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 139 - "Community 139"
Cohesion: 0.29
Nodes (4): set_dword(), get_binary(), ScratchCleanup, test_strict_reg_type_dispatch_and_negative_numeric_sz()

### Community 140 - "Community 140"
Cohesion: 0.48
Nodes (3): OptimizationGoal, metric_pill(), view()

### Community 141 - "Community 141"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 145 - "Community 145"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 146 - "Community 146"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 147 - "Community 147"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 148 - "Community 148"
Cohesion: 0.33
Nodes (4): GpuVendor, Amd, Intel, Nvidia

### Community 150 - "Community 150"
Cohesion: 0.40
Nodes (4): DiskSubsystemReport, PhysicalDriveHealth, probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic()

### Community 151 - "Community 151"
Cohesion: 0.33
Nodes (5): OverallHealthGrade, Good, NeedsOptimization, Optimal, Suboptimal

### Community 154 - "Community 154"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 155 - "Community 155"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

### Community 156 - "Community 156"
Cohesion: 0.40
Nodes (3): check_onedrive_kfm_active(), find_onedrive_setup_exe(), uninstall_onedrive_safe()

### Community 158 - "Community 158"
Cohesion: 0.60
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 159 - "Community 159"
Cohesion: 0.70
Nodes (4): detect_screen_workarea(), main(), run_rune(), run_slint()

### Community 161 - "Community 161"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 162 - "Community 162"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

### Community 163 - "Community 163"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 164 - "Community 164"
Cohesion: 0.50
Nodes (4): PrereqsSortMode, Category, MissingFirst, NameAsc

### Community 165 - "Community 165"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 166 - "Community 166"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

## Knowledge Gaps
- **1316 isolated node(s):** `Advanced Optimization Matrix UI`, `CPU Core Parking Topology Visualizer UI`, `Easy Mode Telemetry Dashboard UI`, `GPU Tuning & Vendor Gating UI`, `1\. Domain Model & Risk Valuation Engine**` (+1311 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1880 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **47 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Community 5` to `Community 2`, `Community 6`, `Community 7`, `Community 11`, `Community 12`, `Community 13`, `Community 16`, `Community 20`, `Community 21`, `Community 22`, `Community 23`, `Community 24`, `Community 25`, `Community 27`, `Community 28`, `Community 35`, `Community 36`, `Community 38`, `Community 39`, `Community 40`, `Community 43`, `Community 49`, `Community 52`, `Community 55`, `Community 60`, `Community 64`, `Community 67`, `Community 69`, `Community 70`, `Community 71`, `Community 72`, `Community 73`, `Community 74`, `Community 78`, `Community 79`, `Community 80`, `Community 83`, `Community 85`, `Community 86`, `Community 87`, `Community 88`, `Community 93`, `Community 94`, `Community 96`, `Community 97`, `Community 98`, `Community 99`, `Community 101`, `Community 103`, `Community 104`, `Community 105`, `Community 106`, `Community 107`, `Community 108`, `Community 109`, `Community 118`, `Community 119`, `Community 120`, `Community 121`, `Community 124`, `Community 129`, `Community 135`, `Community 136`, `Community 137`, `Community 138`, `Community 140`, `Community 144`, `Community 145`, `Community 146`, `Community 147`, `Community 150`, `Community 152`, `Community 153`, `Community 154`, `Community 155`, `Community 157`, `Community 160`, `Community 161`, `Community 162`, `Community 164`, `Community 165`?**
  _High betweenness centrality (0.267) - this node is a cross-community bridge._
- **Are the 21 inferred relationships involving `AppState` (e.g. with `OptimizationPlan` and `ExecutionOrder`) actually correct?**
  _`AppState` has 21 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Advanced Optimization Matrix UI`, `CPU Core Parking Topology Visualizer UI`, `Easy Mode Telemetry Dashboard UI` to the rest of the system?**
  _1316 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Community 0` to `Community 67`, `Community 5`, `Community 38`, `Community 7`, `Community 12`, `Community 79`, `Community 112`, `Community 85`, `Community 21`, `Community 119`, `Community 25`, `Community 27`?**
  _High betweenness centrality (0.257) - this node is a cross-community bridge._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `SystemBackend` connect `Community 6` to `Community 130`, `Community 5`, `Community 136`, `Community 137`, `Community 138`, `Community 10`, `Community 8`, `Community 16`, `Community 18`, `Community 149`, `Community 157`, `Community 31`, `Community 32`, `Community 36`, `Community 40`, `Community 52`, `Community 53`, `Community 58`, `Community 59`, `Community 60`, `Community 62`, `Community 64`, `Community 65`, `Community 66`, `Community 68`, `Community 72`, `Community 74`, `Community 75`, `Community 79`, `Community 82`, `Community 83`, `Community 88`, `Community 89`, `Community 90`, `Community 92`, `Community 93`, `Community 98`, `Community 99`, `Community 100`, `Community 106`, `Community 107`, `Community 116`, `Community 120`, `Community 124`, `Community 125`, `Community 126`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._