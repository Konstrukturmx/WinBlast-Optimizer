# Graph Report - WinBlast0  (2026-11-23)

## Corpus Check
- 132 files · ~296,426 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 89 file(s) not represented in the graph (top: .slint 69, .toml 12, (none) 3)
- WARNING: 89 of 256 files seen (35%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4698 nodes · 9133 edges · 256 communities (127 shown, 129 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 376 edges (avg confidence: 0.74)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `02c19978`
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
- dijkstra.rs
- theme.rs
- app.rs
- astar.rs
- power.rs
- in_progress_hud.rs
- full_telemetry_audit_with
- .state_read
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
- cleaner.rs
- spawn_guarded_worker
- startup.rs
- registry_scanner.rs
- Language
- game_prereqs.rs
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
- ui_render_preview.rs
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
- palette.rs
- dijkstra.rs
- TaskCache
- selection_automation_e2e.rs
- Project Documentation, Architecture & Licensing
- RiskLevel
- tools/startup.rs
- ps_runner.rs
- PlatformSecurityInfo
- disks.rs
- .setup
- app.rs
- tools/automation.rs
- core/theme.rs
- HibernationStatus
- shell.rs
- Ctx
- diff_sync.rs
- Runtime
- sys_inventory.rs
- evaluate_dword_value_with_kind
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
- TaskCache
- ui_parity.rs
- Low-Level NT Core & Kernel Architecture
- views/automation.rs
- window_icon.rs
- HibernationStatus
- PdhParkingCollector
- backup.rs
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
- restart_explorer_safely
- MonotonicCubicProgress
- adaptive_tuning.rs
- WindowsInfo
- ThemeMode
- SystemMetrics
- .all_physical
- SilentCommand
- StartupEntry
- c_void
- DateTime
- BackendTweakValue
- StartupStatusFilter
- task_scanner.rs
- hardware.rs
- path
- SlowScanBackend
- Harness
- BackendTweakStatus
- diff.rs
- views/latency.rs
- UpdateStatus
- test_disks.ps1
- NetAction
- Spring
- StartupTasksSubTab
- test_status_i18n_all_languages_and_core_specs
- lazy_loader.rs
- UpdateState
- resolve_persistent_app_dir
- LiveScheduledTaskInfo
- .sample
- views/network_diag.rs
- SignaturePolicy
- dashboard.rs
- drivers.rs
- planner.rs
- BitSet256
- LiveGameTelemetry
- GameProfile
- enums

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 660 edges
2. `AppState` - 339 edges
3. `ViewTextKey` - 207 edges
4. `SandboxEnv` - 151 edges
5. `SystemBackend` - 148 edges
6. `WorkerEvent` - 109 edges
7. `StatusMessageSpec` - 108 edges
8. `RealBackend` - 85 edges
9. `SlowScanBackend` - 85 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs

## Import Cycles
- 1-file cycle: `src/ui_sync/diff_sync.rs -> src/ui_sync/diff_sync.rs`
- 2-file cycle: `src/ui_slint/palette.rs -> src/ui_slint/shell.rs -> src/ui_slint/palette.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (256 total, 129 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (635): TextKey, AboutBuiltWith, AboutNew1, AboutNew2, AboutNew3, AboutNew4, AboutNew5, AboutNew6 (+627 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (199): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+191 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 3 - "AppState"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (3): AppState, MemoryTrimmed, PerfBenchmarkCompleted

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.11
Nodes (43): LedgerEntry, is_unsupported_power_guid_error(), Result, decode_hex(), encode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair() (+35 more)

### Community 11 - "dijkstra.rs"
Cohesion: 0.07
Nodes (22): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate (+14 more)

### Community 12 - "theme.rs"
Cohesion: 0.05
Nodes (36): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, PresetKey, Gaming, LaptopBattery, ProAudio (+28 more)

### Community 13 - "app.rs"
Cohesion: 0.09
Nodes (34): localize_ledger_action(), CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci() (+26 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "power.rs"
Cohesion: 0.05
Nodes (39): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CoreStatus, CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1 (+31 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.07
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.11
Nodes (21): assert_valid_permutation(), BitSet256, BitSetIter, find_min_weight_cycle_edge(), kahn_schedule(), position_of(), resolve_cycles(), SccResult (+13 more)

### Community 20 - "super"
Cohesion: 0.06
Nodes (7): ApplyCompleted, AutoTuneCompleted, BaselineReset, LatencyBenchmarkCompleted, RestorePointCreated, RollbackCompleted, test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()

### Community 21 - "matrix.rs"
Cohesion: 0.09
Nodes (22): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status, PresetKind (+14 more)

### Community 22 - "sys_inventory.rs"
Cohesion: 0.05
Nodes (19): panic_payload_to_string(), spawn_guarded_worker(), Waker, WorkerDropGuard, AppxBatchRemoved, CleanExecuted, DiskTrimCompleted, ExplorerRestarted (+11 more)

### Community 23 - "updater.rs"
Cohesion: 0.26
Nodes (10): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+2 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (32): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+24 more)

### Community 25 - "clean_apps.rs"
Cohesion: 0.17
Nodes (9): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, CleanAppsView, folded(), format_mb(), target_id() (+1 more)

### Community 27 - "UiThemeStyle"
Cohesion: 0.13
Nodes (11): rgb(), rgba(), UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic (+3 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.19
Nodes (18): DISPLAY_CLASS_LOWER, get_dword_for_vendor(), get_raw(), get_string(), get_string_for_vendor(), has_active_ip(), key_exists(), NET_INTERFACES_LOWER (+10 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 33 - "cleaner.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 35 - "startup.rs"
Cohesion: 0.04
Nodes (43): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, BiosInfoLoaded, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted (+35 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.13
Nodes (33): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+25 more)

### Community 37 - "Language"
Cohesion: 0.12
Nodes (27): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+19 more)

### Community 39 - "game_prereqs.rs"
Cohesion: 0.11
Nodes (28): catalog_undetected(), detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86() (+20 more)

### Community 40 - "parking.rs"
Cohesion: 0.11
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (21): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+13 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.12
Nodes (7): is_reversible_action(), LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak()

### Community 45 - "features_apps.rs"
Cohesion: 0.13
Nodes (26): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+18 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 47 - "Changelog"
Cohesion: 0.17
Nodes (21): is_known_numeric_reg_sz(), test_hkcr_and_shallow_tree_delete_blocked(), test_known_numeric_reg_sz(), validate_registry_mutation_path(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT (+13 more)

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 52 - "ui_render_preview.rs"
Cohesion: 0.14
Nodes (5): H, Headless, render_preview_frames(), settle_and_save(), W

### Community 53 - "backend.rs"
Cohesion: 0.09
Nodes (6): optional_feature_matches(), SandboxMetricsSampler, SandboxTempDir, test_sandbox_core_parking_labels_metrics_and_power_sync(), CpuInfo, LiveServiceInfo

### Community 55 - "ui_views.rs"
Cohesion: 0.15
Nodes (39): about_shows_real_version_and_style_studio_switches_style(), automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection(), clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox(), clean_apps_sub_tabs_drive_state(), clean_apps_winget_catalog_filter_and_selection() (+31 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.16
Nodes (9): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), LazyLoader, LazyPayload, Bios, Disks, Drivers, Thermals (+1 more)

### Community 57 - "probe_physical_drive"
Cohesion: 0.09
Nodes (23): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+15 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.09
Nodes (22): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+14 more)

### Community 60 - "backend.rs"
Cohesion: 0.08
Nodes (23): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+15 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 62 - "AppxCache"
Cohesion: 0.10
Nodes (5): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.14
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.12
Nodes (21): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+13 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.13
Nodes (7): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, test_compound_mutation_atomic_rollback_on_partial_failure()

### Community 67 - "ColorScheme"
Cohesion: 0.12
Nodes (11): MAX_ZOOM, MIN_ZOOM, ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint (+3 more)

### Community 68 - "palette.rs"
Cohesion: 0.22
Nodes (12): is_known_key(), ACTIONS, clean_label(), empty_query_lists_actions_first_without_tweak_row(), every_view_is_listed_once_and_resolves(), item(), MAX_RESULTS, query_matches_are_ranked_and_end_with_tweak_search() (+4 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 70 - "TaskCache"
Cohesion: 0.12
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "RiskLevel"
Cohesion: 0.16
Nodes (8): MatrixQuery, RiskLevel, Extreme, High, Low, Moderate, Safe, matches()

### Community 74 - "tools/startup.rs"
Cohesion: 0.16
Nodes (19): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+11 more)

### Community 75 - "ps_runner.rs"
Cohesion: 0.14
Nodes (12): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+4 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "disks.rs"
Cohesion: 0.08
Nodes (21): DiskPerformanceSnapshot, DiskSubsystemReport, DriveSmartData, extract_null_terminated_str(), HandleGuard, NOT_REPORTED, parse_ata_smart_attributes(), parse_nvme_smart_log() (+13 more)

### Community 78 - ".setup"
Cohesion: 0.14
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (53): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle, BeforeAfterImpact, CleanSortMode (+45 more)

### Community 80 - "tools/automation.rs"
Cohesion: 0.05
Nodes (36): available_physical_ram_kb(), assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability (+28 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.15
Nodes (24): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+16 more)

### Community 83 - "shell.rs"
Cohesion: 0.14
Nodes (10): _assert_model_types(), elapsed_formatting(), every_view_name_resolves(), format_elapsed(), LANGUAGE_EN_NAMES, MAX_TOASTS, Shell, toast_tone() (+2 more)

### Community 84 - "Ctx"
Cohesion: 0.10
Nodes (10): Ctx, apply_language(), install(), lookup(), AboutView, REPO_NAME, REPO_URL, open_external() (+2 more)

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.25
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 88 - "sys_inventory.rs"
Cohesion: 0.06
Nodes (21): detect_platform_security_info(), DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard (+13 more)

### Community 89 - "evaluate_dword_value_with_kind"
Cohesion: 0.18
Nodes (9): check_dword(), evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), ScratchCleanup, test_evaluate_dword_value_with_kind_all_branches() (+1 more)

### Community 91 - "ThemePalette"
Cohesion: 0.33
Nodes (7): compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), SystemProcessorPerformanceInformation

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "UserSettings"
Cohesion: 0.12
Nodes (6): out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings, apply_settings(), SlintBridge

### Community 95 - "restore_point.rs"
Cohesion: 0.17
Nodes (9): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), RestorePoint, test_ensure_preflight_restore_point_sandbox() (+1 more)

### Community 96 - "appx_scanner.rs"
Cohesion: 0.15
Nodes (5): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip(), read_tcp_overrides()

### Community 97 - "adaptive_tuning.rs"
Cohesion: 0.22
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 98 - "advanced_matrix.rs"
Cohesion: 0.15
Nodes (3): RollbackRefreshData, CoreParkingState, PowerSchemeInfo

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.18
Nodes (7): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, list_installed_appx_from_cache()

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 102 - "ui_parity.rs"
Cohesion: 0.17
Nodes (17): AppMode, Advanced, Easy, changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes(), easy_advanced_and_engine_mode_switches(), every_slint_translation_key_exists_in_the_catalogs(), every_theme_combination_reaches_slint() (+9 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.16
Nodes (6): check_elevation(), is_elevated(), relaunch_as_admin(), apply_and_restart(), clean_exe_path(), test_clean_windows_exe_path()

### Community 109 - "sync_vec"
Cohesion: 0.15
Nodes (6): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), DASH, SensorsView, temp_tone()

### Community 110 - "app.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.13
Nodes (19): test_windows_info_detect_host(), AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme() (+11 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.56
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 116 - "views/startup.rs"
Cohesion: 0.16
Nodes (14): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst, build_rows(), is_run_once() (+6 more)

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.14
Nodes (7): GameOverlaySettings, OverlaySetting, DvrDisabled, GameBarDisabled, GameModeEnabled, read_u32(), write_u32()

### Community 120 - "gaming.rs"
Cohesion: 0.09
Nodes (15): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, GameSuitabilityReport, SuitabilityCheck (+7 more)

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "restart_explorer_safely"
Cohesion: 0.20
Nodes (5): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely(), RepairTask

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "WindowsInfo"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 127 - "ThemeMode"
Cohesion: 0.22
Nodes (6): ThemeMode, Dark, Light, ThemePalette, apply(), to_slint()

### Community 129 - "SystemMetrics"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 132 - "StartupEntry"
Cohesion: 0.14
Nodes (13): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), StartupEntry, StartupSource, RegistryHkcuRun (+5 more)

### Community 133 - "c_void"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 134 - "DateTime"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 136 - "StartupStatusFilter"
Cohesion: 0.33
Nodes (4): StartupStatusFilter, ActiveOnly, All, InactiveOnly

### Community 137 - "task_scanner.rs"
Cohesion: 0.17
Nodes (15): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), normalize_task_path(), parse_csv_line(), query_single_task_detailed() (+7 more)

### Community 138 - "hardware.rs"
Cohesion: 0.23
Nodes (4): cpu_title(), HardwareView, kv(), power_plan_name()

### Community 139 - "path"
Cohesion: 0.09
Nodes (12): FaultAction, ClearCbsRebootFlags, ClearMinidumps, ClearPendingRenames, RestartSystem, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless() (+4 more)

### Community 141 - "SlowScanBackend"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 143 - "Harness"
Cohesion: 0.33
Nodes (5): Harness, services_respect_protected_core_and_reject_bad_start_types(), startup_batch_disable_selected_entries_in_sandbox(), startup_lists_filters_and_sorts(), startup_ready()

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

### Community 149 - "NetAction"
Cohesion: 0.40
Nodes (4): NetAction, ApplyDns, FlushDns, ResetDnsDhcp

### Community 150 - "Spring"
Cohesion: 0.16
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 151 - "StartupTasksSubTab"
Cohesion: 0.50
Nodes (4): StartupTasksSubTab, ScheduledTasks, Services, StartupApps

### Community 152 - "test_status_i18n_all_languages_and_core_specs"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 154 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 158 - ".sample"
Cohesion: 0.31
Nodes (6): active_logical_core_count(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 159 - "views/network_diag.rs"
Cohesion: 0.47
Nodes (4): WorkerDropGuard<P>, detect_screen_workarea(), main(), run_slint()

### Community 160 - "SignaturePolicy"
Cohesion: 0.50
Nodes (3): SignaturePolicy, AllowUnsigned, VerifyIfPresent

### Community 161 - "dashboard.rs"
Cohesion: 0.29
Nodes (4): category_buckets(), DashboardView, health_score(), health_tone()

### Community 163 - "drivers.rs"
Cohesion: 0.27
Nodes (3): category_from_id(), DriversView, FILTER_CATEGORIES

### Community 171 - "BitSet256"
Cohesion: 0.07
Nodes (7): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, WORDS

## Knowledge Gaps
- **1471 isolated node(s):** `English`, `Turkish`, `German`, `French`, `Spanish` (+1466 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2189 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **129 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `super`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `planner.rs`, `ColorScheme`, `app.rs`, `tools/automation.rs`, `shell.rs`, `Ctx`, `ui_parity.rs`, `views/automation.rs`, `sync_vec`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`?**
  _High betweenness centrality (0.244) - this node is a cross-community bridge._
- **Are the 24 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationGoal`) actually correct?**
  _`AppState` has 24 INFERRED edges - model-reasoned connections that need verification._
- **What connects `English`, `Turkish`, `German` to the rest of the system?**
  _1471 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.0031496062992125984 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `SystemMetrics`, `spawn_guarded_worker`, `AppState`, `StartupEntry`, `FailAtMutationBackend`, `StartupStatusFilter`, `Scandetailkind Appxinstalled Appxinstalledps`, `dijkstra.rs`, `path`, `hardware.rs`, `Harness`, `in_progress_hud.rs`, `diff.rs`, `views/latency.rs`, `UpdateStatus`, `super`, `matrix.rs`, `sys_inventory.rs`, `StartupTasksSubTab`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `LiveScheduledTaskInfo`, `cleaner.rs`, `dashboard.rs`, `startup.rs`, `registry_scanner.rs`, `Language`, `drivers.rs`, `game_prereqs.rs`, `parking.rs`, `planner.rs`, `FailAtMutationBackend`, `LiveGameTelemetry`, `WindowsInfo`, `backend.rs`, `GameProfile`, `enums`, `ui_views.rs`, `probe_physical_drive`, `backend.rs`, `WinBlast Optimizer`, `FeatureInstallState`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `RiskLevel`, `disks.rs`, `.setup`, `app.rs`, `tools/automation.rs`, `shell.rs`, `Ctx`, `sys_inventory.rs`, `UserSettings`, `restore_point.rs`, `appx_scanner.rs`, `adaptive_tuning.rs`, `advanced_matrix.rs`, `InstalledAppxPackage`, `ui_parity.rs`, `views/automation.rs`, `HibernationStatus`, `PdhParkingCollector`, `backup.rs`, `sync_vec`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `views/diagnostics.rs`, `restart_explorer_safely`, `ThemeMode`?**
  _High betweenness centrality (0.235) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `dashboard.rs`, `Language`, `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `sync_vec`, `Ctx`, `gaming.rs`, `views/diagnostics.rs`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._