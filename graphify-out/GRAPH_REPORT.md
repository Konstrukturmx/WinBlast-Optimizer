# Graph Report - WinBlast0  (2026-11-20)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 4007 nodes · 8334 edges · 165 communities (118 shown, 47 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 330 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9b9806d4`
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
- .logical_core_count
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
- app.rs
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
- reg_helpers.rs
- progress_bar.rs
- storage.rs
- adaptive_tuning.rs
- .poll
- diff_modal.rs
- ui/mod.rs
- .sync_startup_tasks_busy
- .all_physical
- WinBlast Optimizer — Developer & Architecture Guide
- Item
- c_void
- DateTime
- Utc
- enum_scm_services_by_type
- RestorePoint
- WindowsOptionalFeature
- .new
- ActiveDropdown
- StartupSortMode
- RegStringValue
- main.rs
- HardwareSubTab
- .run_scan_internal

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 348 edges
3. `ViewTextKey` - 220 edges
4. `SystemBackend` - 124 edges
5. `SandboxEnv` - 122 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `SlowScanBackend` - 85 edges
9. `RealBackend` - 84 edges
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

## Communities (165 total, 47 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (512): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+504 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 3 - "AppState"
Cohesion: 0.06
Nodes (65): encode_hex(), is_known_numeric_reg_sz(), test_hkcr_and_shallow_tree_delete_blocked(), test_known_numeric_reg_sz(), validate_registry_mutation_path(), display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation() (+57 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.05
Nodes (51): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+43 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (11): AppMode, Advanced, Easy, AppState, DriversSubTab, Drivers, Prerequisites, StagedTransition (+3 more)

### Community 6 - "sys_inventory.rs"
Cohesion: 0.05
Nodes (29): LatencySnapshot, active_logical_core_count(), compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtCpuSampler, NtQuerySystemInformation() (+21 more)

### Community 7 - "features_apps.rs"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 11 - "startup.rs"
Cohesion: 0.05
Nodes (3): PanickingBackend, SlowScanBackend, test_epoch_and_stale_scan_fencing_preserves_instant_mutation()

### Community 12 - "theme.rs"
Cohesion: 0.09
Nodes (22): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+14 more)

### Community 13 - "SlowScanBackend"
Cohesion: 0.05
Nodes (42): PowerActionKind, Hibernation, Scheme, UsbPower, UpdateStatus, Available, Checking, Downloading (+34 more)

### Community 15 - "WinBlast Optimizer"
Cohesion: 0.07
Nodes (21): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate (+13 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.06
Nodes (24): load_embedded_rules(), test_unorthodox_concurrent_worker_event_stress(), test_real_backend_readonly_live_host_audit(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync() (+16 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (6): check_elevation(), is_elevated(), broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 18 - "WinBlastError"
Cohesion: 0.07
Nodes (34): assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult (+26 more)

### Community 20 - "super"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 21 - "clean_apps.rs"
Cohesion: 0.11
Nodes (44): is_unsupported_power_guid_error(), Result, decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target() (+36 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 23 - "FailAtMutationBackend"
Cohesion: 0.09
Nodes (23): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), build_populated_sandbox_state(), click_key(), click_text(), drain_state_workers(), init_cpu_parking_state(), init_dashboard_state() (+15 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.08
Nodes (3): CachedScanBackend<'a>, SandboxEnv, normalize_task_path()

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.07
Nodes (12): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, CleanExecuted, ExplorerRestarted, OneDriveEdgeResult, ScheduledTaskRunResult (+4 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.08
Nodes (10): detect_platform_security_info(), NtQuerySystemInformation(), query_live_code_integrity_options(), query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), SystemCodeIntegrityInfo (+2 more)

### Community 27 - "RealBackend"
Cohesion: 0.11
Nodes (22): assert_valid_permutation(), BitSet256, BitSetIter, find_min_weight_cycle_edge(), kahn_schedule(), MAX_NODES, position_of(), resolve_cycles() (+14 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.10
Nodes (11): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader, LazyPayload, Bios, Disks (+3 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.05
Nodes (36): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, PresetKey, Gaming, LaptopBattery, ProAudio (+28 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.10
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.09
Nodes (33): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+25 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.07
Nodes (9): PresetKind, Gaming, LaptopBattery, ProAudio, BaselineReset, LatencyBenchmarkCompleted, PerfBenchmarkCompleted, RestorePointCreated (+1 more)

### Community 33 - "power.rs"
Cohesion: 0.09
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 35 - "startup.rs"
Cohesion: 0.11
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.13
Nodes (7): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud()

### Community 37 - "elevation.rs"
Cohesion: 0.17
Nodes (26): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+18 more)

### Community 38 - "parse_guid"
Cohesion: 0.07
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.08
Nodes (6): fallback_scheme_name(), query_hibernation_and_standby(), resolve_indirect_string(), set_hibernation_enabled(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 40 - "parking.rs"
Cohesion: 0.08
Nodes (23): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+15 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (22): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.13
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.13
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.15
Nodes (21): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_live_services(), list_startup_entries() (+13 more)

### Community 47 - "FailAtMutationBackend"
Cohesion: 0.15
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 48 - "SlowScanBackend"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 49 - "deserialize"
Cohesion: 0.15
Nodes (14): RiskValuation, TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), state_counts() (+6 more)

### Community 50 - "updater.rs"
Cohesion: 0.12
Nodes (20): DiskPerformanceSnapshot, DiskSubsystemReport, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), PhysicalDriveHealth (+12 more)

### Community 51 - ".logical_core_count"
Cohesion: 0.25
Nodes (22): CachedScanBackend, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan(), run_full_scan_with() (+14 more)

### Community 52 - "PanickingBackend"
Cohesion: 0.09
Nodes (12): Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme, High (+4 more)

### Community 53 - "SlowScanBackend"
Cohesion: 0.24
Nodes (15): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+7 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.12
Nodes (21): relaunch_as_admin(), check_latest_release(), clean_exe_path(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO (+13 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.09
Nodes (23): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, TweakAction, AppXPackage, CompositeStringKey, PowerCfgSetting (+15 more)

### Community 57 - "DriverError"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.20
Nodes (13): build_signing_transcript(), compute_sha256(), OverrideKey, PresetGuards, PresetMetadata, PresetPackage, sample_metadata(), test_canonical_encode_decode_roundtrip_unsigned() (+5 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 60 - "backend.rs"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.13
Nodes (6): GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry, BenchmarkRunner, BenchmarkScore, RepairTask

### Community 64 - "CoreParkingState"
Cohesion: 0.27
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 65 - "task_scanner.rs"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 66 - "InstalledAppxPackage"
Cohesion: 0.16
Nodes (10): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+2 more)

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.29
Nodes (12): render_compatible_card(), render_incompatible_card(), view(), view(), action_button(), action_button_icon_only(), action_button_loading(), action_button_with_tooltip() (+4 more)

### Community 68 - "WorkerEvent"
Cohesion: 0.12
Nodes (4): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox()

### Community 69 - "path"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 70 - "TaskCache"
Cohesion: 0.20
Nodes (15): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed() (+7 more)

### Community 71 - "backend.rs"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.18
Nodes (8): before_after_comparison_card(), view(), rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 73 - "WorkerEvent"
Cohesion: 0.18
Nodes (12): apply_parking_snapshot(), capture_parking_snapshot(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot, restore_stock_parking() (+4 more)

### Community 74 - "app.rs"
Cohesion: 0.15
Nodes (6): compute_is_fully_unparked(), CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power(), test_ac_power_status_and_enumerated_range_noop()

### Community 75 - ".set_status_spec"
Cohesion: 0.12
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.12
Nodes (5): EngineMode, Instant, Normal, test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports()

### Community 77 - "DriverUpdateItem"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 78 - ".setup"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 79 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.15
Nodes (15): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall (+7 more)

### Community 80 - "deserialize"
Cohesion: 0.19
Nodes (5): test_windows_info_detect_host(), BiosSystemInfo, DisplayDriverInfo, GameSuitabilityReport, SuitabilityCheck

### Community 81 - "Language"
Cohesion: 0.21
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 82 - "HibernationStatus"
Cohesion: 0.17
Nodes (4): MetricsSampler, SandboxMetricsSampler, SystemMonitor, SystemMetrics

### Community 83 - "normalize_guid_str"
Cohesion: 0.18
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 84 - "DriverUpdateState"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 85 - "enums"
Cohesion: 0.18
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 86 - "SandboxEnv"
Cohesion: 0.14
Nodes (4): BiosInfoLoaded, DiskReportLoaded, DriverScanCompleted, ThermalSnapshot

### Community 87 - "Runtime"
Cohesion: 0.20
Nodes (9): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), RestorePoint, test_ensure_preflight_restore_point_sandbox() (+1 more)

### Community 88 - "UiThemeStyle"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 89 - "restart_explorer_safely"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 90 - "latency_bench.rs"
Cohesion: 0.21
Nodes (5): FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus, test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()

### Community 93 - "FeatureInstallState"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 94 - "WindowsOptionalFeature"
Cohesion: 0.42
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 96 - "RegTreeSnapshot"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 97 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 98 - "advanced_matrix.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 99 - "AppxCache"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 101 - "WinBlastScrollable"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 102 - "PowerSchemeInfo"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 104 - "privacy_shield.rs"
Cohesion: 0.58
Nodes (6): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), view()

### Community 105 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 106 - "PlatformSecurityInfo"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 107 - "PdhParkingCollector"
Cohesion: 0.39
Nodes (8): full_telemetry_audit(), full_telemetry_audit_with(), get_blocked_hosts(), KNOWN_TELEMETRY_SERVICES, parse_blocked_hosts_from_content(), scan_telemetry_agents(), TELEMETRY_HOSTS, TelemetryAuditResult

### Community 108 - "RestorePoint"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 109 - "probe_physical_drive"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 111 - "SystemMetrics"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 113 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 114 - "progress_bar.rs"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 123 - "progress_bar.rs"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 124 - "storage.rs"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 125 - "adaptive_tuning.rs"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 126 - ".poll"
Cohesion: 0.33
Nodes (4): GpuVendor, Amd, Intel, Nvidia

### Community 130 - ".all_physical"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 131 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

### Community 132 - "Item"
Cohesion: 0.40
Nodes (4): SignaturePolicy, AllowUnsigned, Required, VerifyIfPresent

### Community 136 - "enum_scm_services_by_type"
Cohesion: 0.70
Nodes (3): localize_dns_category(), localize_dns_status(), view()

### Community 137 - "RestorePoint"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 138 - "WindowsOptionalFeature"
Cohesion: 0.50
Nodes (4): DriverSortMode, Category, NameAsc, UpdatesFirst

### Community 139 - ".new"
Cohesion: 0.50
Nodes (4): PrereqsSortMode, Category, MissingFirst, NameAsc

### Community 140 - "ActiveDropdown"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 141 - "StartupSortMode"
Cohesion: 0.50
Nodes (4): StartupStatusFilter, ActiveOnly, All, InactiveOnly

### Community 143 - "main.rs"
Cohesion: 0.67
Nodes (3): RepairSubTab, FaultDiagnostics, WindowsRepair

## Knowledge Gaps
- **1256 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` (+1251 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1864 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **47 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `AppState`, `.Toggle .Set .Apply`, `sys_inventory.rs`, `.Reg .Set Power`, `theme.rs`, `SlowScanBackend`, `.take_injected_error`, `WinBlast Optimizer`, `in_progress_hud.rs`, `WinBlastError`, `Multi-Language Tweak Localization Tables`, `FailAtMutationBackend`, `.Reg State() Vendor()`, `Appx Package() Prefixes`, `Win32 Registry Scanner & Mutator`, `driver_update.rs`, `power.rs`, `spawn_guarded_worker`, `startup.rs`, `registry_scanner.rs`, `elevation.rs`, `ps_runner.rs`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `deserialize`, `updater.rs`, `.logical_core_count`, `SlowScanBackend`, `Project History & Release Version Ledger`, `driver_update.rs`, `task_scanner.rs`, `backend.rs`, `Error.Rs Winblasterror Appx`, `InstalledAppxPackage`, `Reversibility Instant Requireslogoff`, `path`, `backend.rs`, `Project Documentation, Architecture & Licensing`, `app.rs`, `.set_status_spec`, `PlatformSecurityInfo`, `.setup`, `Hardware Diagnostics & GPU Silicon Tuning`, `deserialize`, `HibernationStatus`, `enums`, `SandboxEnv`, `Runtime`, `restart_explorer_safely`, `latency_bench.rs`, `ThemePalette`, `edge_cases_unorthodox.rs`, `Low-Level NT Core & Kernel Architecture`, `privacy_shield.rs`, `PdhParkingCollector`, `app.rs`, `progress_bar.rs`, `tools/game_overlay.rs`, `tools/game_overlay.rs`, `lazy_loader.rs`, `TaskCache`, `progress_bar.rs`, `storage.rs`, `adaptive_tuning.rs`, `diff_modal.rs`, `.all_physical`, `WinBlast Optimizer — Developer & Architecture Guide`, `c_void`, `DateTime`, `Utc`, `enum_scm_services_by_type`, `RestorePoint`, `WindowsOptionalFeature`, `.new`, `ActiveDropdown`, `StartupSortMode`, `main.rs`?**
  _High betweenness centrality (0.278) - this node is a cross-community bridge._
- **Are the 26 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 26 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges` to the rest of the system?**
  _1256 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `RegTreeSnapshot`, `Reversibility Instant Requireslogoff`, `Disk Cleaning & Temporary File Purger`, `path`, `elevation.rs`, `privacy_shield.rs`, `Project Documentation, Architecture & Licensing`, `Low-Level NT Core & Kernel Architecture`, `theme.rs`, `deserialize`, `.logical_core_count`, `SlowScanBackend`, `FailAtMutationBackend`, `Hardware Diagnostics & GPU Silicon Tuning`?**
  _High betweenness centrality (0.269) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `.all_physical`, `Reversibility Instant Requireslogoff`, `Utc`, `privacy_shield.rs`, `enum_scm_services_by_type`, `theme.rs`, `SlowScanBackend`?**
  _High betweenness centrality (0.144) - this node is a cross-community bridge._