# Graph Report - WinBlast0  (2026-11-22)

## Corpus Check
- 129 files · ~288,379 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 85 file(s) not represented in the graph (top: .slint 67, .toml 12, (none) 3)
- WARNING: 85 of 246 files seen (35%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4539 nodes · 8898 edges · 255 communities (137 shown, 118 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 365 edges (avg confidence: 0.73)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `904acf84`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- action_button_guarded
- spawn_guarded_worker
- AppState
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- FailAtMutationBackend
- Integration Suite: Tweak Sandbox E2E
- Scandetailkind Appxinstalled Appxinstalledps
- astar.rs
- theme.rs
- app.rs
- astar.rs
- WinBlast Optimizer
- in_progress_hud.rs
- full_telemetry_audit_with
- WinBlastError
- topological_scheduler.rs
- super
- matrix.rs
- sys_inventory.rs
- updater.rs
- System Architecture Changelog
- clean_apps.rs
- Appx Package() Prefixes
- UiThemeStyle
- GPU Silicon & Hardware Tuning Policies
- Hardware Diagnostics & GPU Silicon Tuning
- reg_mutator.rs
- Win32 Registry Scanner & Mutator
- RealBackend
- power.rs
- spawn_guarded_worker
- startup.rs
- registry_scanner.rs
- sys_inventory.rs
- AppX Package & Modern App Scanner
- parking.rs
- Embedded Catalog Test
- ps_runner.rs
- FailAtMutationBackend
- features_apps.rs
- svc_mutator.rs
- Changelog
- registry_scanner.rs
- WindowsInfo
- updater.rs
- PlatformSecurityInfo
- enum_scm_services_by_type
- backend.rs
- ui_views.rs
- driver_update.rs
- probe_physical_drive
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- Hardware Diagnostics & GPU Silicon Tuning
- AppxCache
- WinBlast Optimizer
- FilterStrictSelection
- FeatureInstallState
- .maybe_arm_error
- ColorScheme
- AutoTuneProfile
- dijkstra.rs
- TaskCache
- selection_automation_e2e.rs
- Project Documentation, Architecture & Licensing
- WorkerEvent
- app.rs
- ps_runner.rs
- PlatformSecurityInfo
- disks.rs
- .setup
- app.rs
- deserialize
- core/theme.rs
- HibernationStatus
- shell.rs
- Ctx
- diff_sync.rs
- Runtime
- PlatformSecurityInfo
- normalize_guid_str
- test_wave1_compound_actions_deep_edge_cases
- ThemePalette
- edge_cases_unorthodox.rs
- FeatureInstallState
- UserSettings
- restore_point.rs
- appx_scanner.rs
- adaptive_tuning.rs
- advanced_matrix.rs
- InstalledAppxPackage
- Project History & Release Version Ledger
- Language
- ui_parity.rs
- Low-Level NT Core & Kernel Architecture
- privacy_shield.rs
- .navigate
- PlatformSecurityInfo
- PdhParkingCollector
- tools/automation.rs
- sync_vec
- app.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- test_wave1_compound_actions_deep_edge_cases
- progress_bar.rs
- tools/game_overlay.rs
- views/startup.rs
- lazy_loader.rs
- TaskCache
- gaming.rs
- views/diagnostics.rs
- reg_helpers.rs
- silentcommand
- MonotonicCubicProgress
- adaptive_tuning.rs
- WindowsInfo
- BitSet256
- .sync_startup_tasks_busy
- .all_physical
- WinBlast Optimizer — Developer & Architecture Guide
- Item
- c_void
- DateTime
- BackendTweakValue
- enum_scm_services_by_type
- TaskCache
- hardware.rs
- path
- SlowScanBackend
- MatrixSortMode
- BackendTweakStatus
- diff.rs
- views/latency.rs
- UpdateStatus
- test_disks.ps1
- WindowsOptionalFeature
- Spring
- .new
- dijkstra.rs
- lazy_loader.rs
- PrereqInstallState
- .set_status
- hashmap
- task_scanner.rs
- .sample
- views/network_diag.rs
- SignaturePolicy
- dashboard.rs
- drivers.rs
- ui_slint/i18n.rs
- StartupEntry
- GamePrerequisiteItem
- planner.rs
- BitSet256
- GameProfile
- enums
- GamingSubTab
- ActiveDropdown
- OverlaySetting
- OverallHealthGrade
- BitSetIter
- .execute_auto_tune_profile
- NodeCost

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 567 edges
2. `AppState` - 339 edges
3. `ViewTextKey` - 207 edges
4. `SandboxEnv` - 151 edges
5. `SystemBackend` - 148 edges
6. `WorkerEvent` - 109 edges
7. `StatusMessageSpec` - 105 edges
8. `RealBackend` - 85 edges
9. `SlowScanBackend` - 85 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SystemBackend`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs

## Import Cycles
- 1-file cycle: `src/ui_sync/diff_sync.rs -> src/ui_sync/diff_sync.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (255 total, 118 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (544): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+536 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.02
Nodes (99): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+91 more)

### Community 3 - "AppState"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (6): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, MemoryTrimmed

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.12
Nodes (39): decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry(), test_composite_string_merge_preserves_siblings() (+31 more)

### Community 11 - "astar.rs"
Cohesion: 0.14
Nodes (8): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 12 - "theme.rs"
Cohesion: 0.06
Nodes (34): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+26 more)

### Community 13 - "app.rs"
Cohesion: 0.09
Nodes (34): localize_ledger_action(), CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci() (+26 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "WinBlast Optimizer"
Cohesion: 0.08
Nodes (4): fallback_scheme_name(), resolve_indirect_string(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.08
Nodes (22): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+14 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.22
Nodes (14): assert_valid_permutation(), position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence(), test_max_capacity_256_nodes() (+6 more)

### Community 20 - "super"
Cohesion: 0.07
Nodes (9): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted, RestorePointCreated (+1 more)

### Community 21 - "matrix.rs"
Cohesion: 0.06
Nodes (32): MatrixQuery, MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status (+24 more)

### Community 22 - "sys_inventory.rs"
Cohesion: 0.06
Nodes (14): spawn_guarded_worker(), Waker, WorkerDropGuard, AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, DiskTrimCompleted, ExplorerRestarted (+6 more)

### Community 23 - "updater.rs"
Cohesion: 0.14
Nodes (18): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+10 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (32): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+24 more)

### Community 25 - "clean_apps.rs"
Cohesion: 0.17
Nodes (9): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, CleanAppsView, folded(), format_mb(), target_id() (+1 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.05
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 27 - "UiThemeStyle"
Cohesion: 0.13
Nodes (9): ThemePalette, UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic (+1 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.14
Nodes (27): encode_hex(), check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword() (+19 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 33 - "power.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.09
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 35 - "startup.rs"
Cohesion: 0.05
Nodes (32): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, DiskReportLoaded (+24 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.21
Nodes (23): CachedScanBackend, RiskValuation, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan() (+15 more)

### Community 37 - "sys_inventory.rs"
Cohesion: 0.07
Nodes (20): detect_platform_security_info(), DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard (+12 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.28
Nodes (15): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+7 more)

### Community 40 - "parking.rs"
Cohesion: 0.10
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (21): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+13 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.12
Nodes (8): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud(), rollback_entry()

### Community 45 - "features_apps.rs"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.10
Nodes (24): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+16 more)

### Community 47 - "Changelog"
Cohesion: 0.20
Nodes (17): delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary(), set_dword() (+9 more)

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 52 - "enum_scm_services_by_type"
Cohesion: 0.70
Nodes (3): enum_scm_services_by_type(), query_scm_win32_services(), ScmServiceRecord

### Community 53 - "backend.rs"
Cohesion: 0.09
Nodes (5): MetricsSampler, SandboxMetricsSampler, SystemMonitor, CpuInfo, LiveServiceInfo

### Community 55 - "ui_views.rs"
Cohesion: 0.14
Nodes (40): automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection(), clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox(), clean_apps_sub_tabs_drive_state(), clean_apps_winget_catalog_filter_and_selection(), core_parking_actions_round_trip_in_sandbox() (+32 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.15
Nodes (10): scan_online_drivers(), background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), LazyLoader, LazyPayload, Bios, Disks, Drivers (+2 more)

### Community 57 - "probe_physical_drive"
Cohesion: 0.10
Nodes (23): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+15 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 60 - "backend.rs"
Cohesion: 0.08
Nodes (27): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+19 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 62 - "AppxCache"
Cohesion: 0.10
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.14
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.11
Nodes (23): list_restore_points(), FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind (+15 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.14
Nodes (6): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot

### Community 67 - "ColorScheme"
Cohesion: 0.11
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "AutoTuneProfile"
Cohesion: 0.16
Nodes (15): is_unsupported_power_guid_error(), apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1 (+7 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 70 - "TaskCache"
Cohesion: 0.12
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 71 - "selection_automation_e2e.rs"
Cohesion: 0.12
Nodes (3): StagedTransition, StageApply, StageRevert

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "WorkerEvent"
Cohesion: 0.15
Nodes (10): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, create_auto_tune_plan(), test_iso_29119_auto_tune_plan_generation_all_profiles() (+2 more)

### Community 74 - "app.rs"
Cohesion: 0.17
Nodes (19): set_scheduled_task_enabled(), CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_live_services(), list_startup_entries() (+11 more)

### Community 75 - "ps_runner.rs"
Cohesion: 0.14
Nodes (12): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+4 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "disks.rs"
Cohesion: 0.09
Nodes (20): DiskPerformanceSnapshot, DiskSubsystemReport, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), PhysicalDriveHealth (+12 more)

### Community 78 - ".setup"
Cohesion: 0.15
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (50): BeforeAfterImpact, CleanSortMode, NameAsc, SizeAsc, SizeDesc, DriverSortMode, Category, NameAsc (+42 more)

### Community 80 - "deserialize"
Cohesion: 0.15
Nodes (10): install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind, DailyMemoryPurge, TelemetryGuard, WeeklyTempCleaner, MaintenanceTaskStatus, query_all_maintenance_tasks() (+2 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.13
Nodes (27): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+19 more)

### Community 83 - "shell.rs"
Cohesion: 0.11
Nodes (12): AppMode, Advanced, Easy, _assert_model_types(), elapsed_formatting(), every_view_name_resolves(), format_elapsed(), LANGUAGE_EN_NAMES (+4 more)

### Community 84 - "Ctx"
Cohesion: 0.11
Nodes (4): Ctx, AutomationView, BackupView, Views

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.23
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 89 - "normalize_guid_str"
Cohesion: 0.29
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 91 - "ThemePalette"
Cohesion: 0.31
Nodes (7): compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), SystemProcessorPerformanceInformation, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "UserSettings"
Cohesion: 0.13
Nodes (7): MAX_ZOOM, MIN_ZOOM, out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings, SlintBridge

### Community 95 - "restore_point.rs"
Cohesion: 0.29
Nodes (7): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 96 - "appx_scanner.rs"
Cohesion: 0.11
Nodes (9): DNS_CATALOG, DnsProbeResult, NetAction, ApplyDns, FlushDns, ResetDnsDhcp, NetworkDiagnostics, query_local_ip() (+1 more)

### Community 97 - "adaptive_tuning.rs"
Cohesion: 0.18
Nodes (7): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 98 - "advanced_matrix.rs"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - "Language"
Cohesion: 0.14
Nodes (24): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+16 more)

### Community 102 - "ui_parity.rs"
Cohesion: 0.09
Nodes (20): EngineMode, Instant, Normal, test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports(), changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes() (+12 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.15
Nodes (6): check_elevation(), is_elevated(), relaunch_as_admin(), apply_and_restart(), clean_exe_path(), test_clean_windows_exe_path()

### Community 104 - "privacy_shield.rs"
Cohesion: 0.17
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 105 - ".navigate"
Cohesion: 0.07
Nodes (13): BiosInfoLoaded, DriverScanCompleted, GameTelemetryProbed, LatencyUpdated, MetricsSnapshot, PrereqsScanned, RestorePointsLoaded, StatusUpdate (+5 more)

### Community 106 - "PlatformSecurityInfo"
Cohesion: 0.14
Nodes (5): RollbackRefreshData, HibernationStatus, PowerSchemeInfo, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "tools/automation.rs"
Cohesion: 0.17
Nodes (10): assess_system_health(), HealthSeverity, Critical, Info, Warning, MemoryTrimResult, SystemHealthAssessment, SystemHealthIssue (+2 more)

### Community 109 - "sync_vec"
Cohesion: 0.16
Nodes (6): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), DASH, SensorsView, temp_tone()

### Community 110 - "app.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.16
Nodes (18): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd() (+10 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.42
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 116 - "views/startup.rs"
Cohesion: 0.16
Nodes (14): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst, build_rows(), is_run_once() (+6 more)

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.21
Nodes (5): GameOverlaySettings, GameSuitabilityReport, read_u32(), SuitabilityCheck, write_u32()

### Community 120 - "gaming.rs"
Cohesion: 0.17
Nodes (4): dns_category(), dns_status(), GamingView, suitability()

### Community 121 - "views/diagnostics.rs"
Cohesion: 0.18
Nodes (3): DiagnosticsView, ISSUES_URL, open_external()

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "WindowsInfo"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 127 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 131 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.16
Nodes (4): GameFpsComparison, get_standard_game_benchmarks(), BenchmarkRunner, BenchmarkScore

### Community 132 - "Item"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 133 - "c_void"
Cohesion: 0.53
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 137 - "TaskCache"
Cohesion: 0.13
Nodes (6): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes(), list_live_tasks_from_cache(), LiveScheduledTaskInfo

### Community 138 - "hardware.rs"
Cohesion: 0.23
Nodes (4): cpu_title(), HardwareView, kv(), power_plan_name()

### Community 139 - "path"
Cohesion: 0.12
Nodes (10): FaultAction, ClearCbsRebootFlags, ClearMinidumps, ClearPendingRenames, RestartSystem, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless() (+2 more)

### Community 141 - "SlowScanBackend"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 144 - "BackendTweakStatus"
Cohesion: 0.22
Nodes (9): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, TweakShadow (+1 more)

### Community 145 - "diff.rs"
Cohesion: 0.28
Nodes (13): diff_pair_card(), diff_single_card(), diff_view(), DiffRow, directed_pair(), header_with_dir(), rows_for(), Tone (+5 more)

### Community 146 - "views/latency.rs"
Cohesion: 0.29
Nodes (5): better(), comparison(), jitter_text(), LatencyView, ping_text()

### Community 147 - "UpdateStatus"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 148 - "test_disks.ps1"
Cohesion: 0.29
Nodes (5): GpuVendor, Amd, Intel, Nvidia, PresetGuards

### Community 150 - "Spring"
Cohesion: 0.16
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 152 - "dijkstra.rs"
Cohesion: 0.27
Nodes (8): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks(), MAX_NODES

### Community 154 - "PrereqInstallState"
Cohesion: 0.40
Nodes (5): PrereqInstallState, Failed, Installed, Installing, NotInstalled

### Community 156 - "hashmap"
Cohesion: 0.28
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

### Community 157 - "task_scanner.rs"
Cohesion: 0.28
Nodes (10): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+2 more)

### Community 158 - ".sample"
Cohesion: 0.30
Nodes (7): active_logical_core_count(), NtCpuSampler, query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 159 - "views/network_diag.rs"
Cohesion: 0.38
Nodes (5): WorkerDropGuard<P>, detect_screen_workarea(), main(), run_slint(), apply_settings()

### Community 160 - "SignaturePolicy"
Cohesion: 0.50
Nodes (3): SignaturePolicy, AllowUnsigned, VerifyIfPresent

### Community 161 - "dashboard.rs"
Cohesion: 0.29
Nodes (4): category_buckets(), DashboardView, health_score(), health_tone()

### Community 163 - "drivers.rs"
Cohesion: 0.27
Nodes (3): category_from_id(), DriversView, FILTER_CATEGORIES

### Community 165 - "ui_slint/i18n.rs"
Cohesion: 0.26
Nodes (6): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS, apply_language(), install(), lookup()

### Community 168 - "GamePrerequisiteItem"
Cohesion: 0.21
Nodes (8): catalog_undetected(), GamePrerequisiteItem, install_prereq_cascade(), scan_game_prereqs(), test_iso_29119_cascade_error_recovery_empty_source(), test_iso_29119_cascade_fallback_url_validity(), test_iso_29119_scan_returns_all_10_prereqs(), test_iso_29119_serde_roundtrip()

### Community 171 - "BitSet256"
Cohesion: 0.07
Nodes (7): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, WORDS

### Community 184 - "GamingSubTab"
Cohesion: 0.29
Nodes (7): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, SUB_TABS

### Community 185 - "ActiveDropdown"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 186 - "OverlaySetting"
Cohesion: 0.33
Nodes (4): OverlaySetting, DvrDisabled, GameBarDisabled, GameModeEnabled

### Community 187 - "OverallHealthGrade"
Cohesion: 0.33
Nodes (5): OverallHealthGrade, Good, NeedsOptimization, Optimal, Suboptimal

## Knowledge Gaps
- **1380 isolated node(s):** `Easy`, `Advanced`, `HardwareView`, `PrivacyShield`, `BackupCenter` (+1375 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2077 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **118 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `AppState`, `FailAtMutationBackend`, `astar.rs`, `in_progress_hud.rs`, `super`, `matrix.rs`, `sys_inventory.rs`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `power.rs`, `startup.rs`, `registry_scanner.rs`, `sys_inventory.rs`, `parking.rs`, `FailAtMutationBackend`, `WindowsInfo`, `backend.rs`, `ui_views.rs`, `probe_physical_drive`, `backend.rs`, `WinBlast Optimizer`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `selection_automation_e2e.rs`, `Project Documentation, Architecture & Licensing`, `WorkerEvent`, `disks.rs`, `.setup`, `app.rs`, `deserialize`, `shell.rs`, `Ctx`, `PlatformSecurityInfo`, `UserSettings`, `appx_scanner.rs`, `adaptive_tuning.rs`, `advanced_matrix.rs`, `InstalledAppxPackage`, `Language`, `ui_parity.rs`, `privacy_shield.rs`, `.navigate`, `PlatformSecurityInfo`, `PdhParkingCollector`, `tools/automation.rs`, `sync_vec`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `views/diagnostics.rs`, `silentcommand`, `.sync_startup_tasks_busy`, `WinBlast Optimizer — Developer & Architecture Guide`, `enum_scm_services_by_type`, `TaskCache`, `hardware.rs`, `path`, `diff.rs`, `views/latency.rs`, `UpdateStatus`, `WindowsOptionalFeature`, `dijkstra.rs`, `.set_status`, `hashmap`, `views/network_diag.rs`, `dashboard.rs`, `drivers.rs`, `StartupEntry`, `GamePrerequisiteItem`, `planner.rs`, `GameProfile`, `enums`, `GamingSubTab`, `ActiveDropdown`, `OverlaySetting`, `.execute_auto_tune_profile`?**
  _High betweenness centrality (0.276) - this node is a cross-community bridge._
- **Are the 24 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationPlan`) actually correct?**
  _`AppState` has 24 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Easy`, `Advanced`, `HardwareView` to the rest of the system?**
  _1380 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.003676470588235294 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `ui_slint/i18n.rs`, `planner.rs`, `ColorScheme`, `app.rs`, `shell.rs`, `ui_parity.rs`, `sync_vec`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`?**
  _High betweenness centrality (0.222) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `SandboxEnv` connect `WinBlastError` to `WinBlast Optimizer — Developer & Architecture Guide`, `FailAtMutationBackend`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `path`, `in_progress_hud.rs`, `WindowsOptionalFeature`, `.new`, `Win32 Registry Scanner & Mutator`, `power.rs`, `spawn_guarded_worker`, `startup.rs`, `registry_scanner.rs`, `StartupEntry`, `GamePrerequisiteItem`, `PlatformSecurityInfo`, `backend.rs`, `GameProfile`, `enums`, `ui_views.rs`, `task_scanner.rs`, `WinBlast Optimizer`, `.maybe_arm_error`, `selection_automation_e2e.rs`, `Project Documentation, Architecture & Licensing`, `disks.rs`, `.setup`, `deserialize`, `HibernationStatus`, `PlatformSecurityInfo`, `edge_cases_unorthodox.rs`, `restore_point.rs`, `appx_scanner.rs`, `adaptive_tuning.rs`, `advanced_matrix.rs`, `Project History & Release Version Ledger`, `ui_parity.rs`, `PlatformSecurityInfo`, `PdhParkingCollector`, `tools/automation.rs`, `test_wave1_compound_actions_deep_edge_cases`, `WindowsInfo`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._