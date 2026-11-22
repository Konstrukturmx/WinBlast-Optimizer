# Graph Report - WinBlast0  (2026-11-22)

## Corpus Check
- 143 files · ~304,071 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 62 file(s) not represented in the graph (top: .slint 44, .toml 12, (none) 3)
- WARNING: 62 of 234 files seen (26%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4273 nodes · 8898 edges · 196 communities (138 shown, 58 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 330 edges (avg confidence: 0.79)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `db0e3a47`
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
- features_apps.rs
- Integration Suite: Tweak Sandbox E2E
- Scandetailkind Appxinstalled Appxinstalledps
- startup.rs
- theme.rs
- app.rs
- astar.rs
- WinBlast Optimizer
- in_progress_hud.rs
- full_telemetry_audit_with
- WinBlastError
- WinBlast Optimizer
- super
- clean_apps.rs
- sys_inventory.rs
- updater.rs
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
- topological_scheduler.rs
- parse_guid
- AppX Package & Modern App Scanner
- parking.rs
- Embedded Catalog Test
- ps_runner.rs
- FailAtMutationBackend
- power.rs
- features_apps.rs
- svc_mutator.rs
- Changelog
- registry_scanner.rs
- deserialize
- updater.rs
- PlatformSecurityInfo
- list_live_services
- backend.rs
- Project History & Release Version Ledger
- RegRawSnapshot
- driver_update.rs
- probe_physical_drive
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- Hardware Diagnostics & GPU Silicon Tuning
- AppxCache
- Error.Rs Winblasterror Appx
- FilterStrictSelection
- task_scanner.rs
- .maybe_arm_error
- Reversibility Instant Requireslogoff
- AutoTuneProfile
- dijkstra.rs
- TaskCache
- Project Documentation, Architecture & Licensing
- WorkerEvent
- app.rs
- task_scanner.rs
- PlatformSecurityInfo
- DriverUpdateItem
- .setup
- app.rs
- deserialize
- decode_oem_bytes
- HibernationStatus
- normalize_guid_str
- bridge.rs
- enums
- diff_sync.rs
- Runtime
- UiThemeStyle
- normalize_guid_str
- RegStringValue
- ThemePalette
- edge_cases_unorthodox.rs
- FeatureInstallState
- WindowsOptionalFeature
- restore_point.rs
- appx_scanner.rs
- adaptive_tuning.rs
- advanced_matrix.rs
- WinBlast Optimizer — Developer & Architecture Guide
- Project History & Release Version Ledger
- .sign
- PowerSchemeInfo
- Low-Level NT Core & Kernel Architecture
- privacy_shield.rs
- PlatformSecurityInfo
- PlatformSecurityInfo
- PdhParkingCollector
- RestorePoint
- appx_scanner.rs
- app.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- test_wave1_compound_actions_deep_edge_cases
- progress_bar.rs
- tools/game_overlay.rs
- tools/game_overlay.rs
- lazy_loader.rs
- TaskCache
- boot_config.rs
- power_governor.rs
- window_icon.rs
- reg_helpers.rs
- progress_bar.rs
- MonotonicCubicProgress
- adaptive_tuning.rs
- WindowsInfo
- AppxSafetyTier
- ui/mod.rs
- .sync_startup_tasks_busy
- .all_physical
- WinBlast Optimizer — Developer & Architecture Guide
- Item
- c_void
- DateTime
- BackendTweakValue
- enum_scm_services_by_type
- HealthSeverity
- startup.rs
- path
- WinBlastScrollable
- SlowScanBackend
- MatrixSortMode
- BackendTweakStatus
- test_status_i18n_all_languages_and_core_specs
- UpdateState
- UpdateStatus
- test_disks.ps1
- CachedScanBackend<'a>
- sysprep_engine.rs
- .new
- LiveServiceInfo
- PrereqInstallState
- .set_status
- PresetKind
- clean_exe_path
- views/network_diag.rs
- SignaturePolicy
- HealthSeverity
- latency_bench.rs
- SilentCommand
- RegStringValue
- OverallHealthGrade
- CleanSortMode
- BitSet256
- .reset_baseline
- .rollback_entry
- set_usb_root_hub_power_saving

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 371 edges
3. `ViewTextKey` - 220 edges
4. `SandboxEnv` - 126 edges
5. `SystemBackend` - 124 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `WorkerEvent` - 85 edges
9. `SlowScanBackend` - 85 edges
10. `RealBackend` - 84 edges

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
- 1-file cycle: `src/ui_sync/diff_sync.rs -> src/ui_sync/diff_sync.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (196 total, 58 thin omitted)

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
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (8): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, localize_dns_category(), localize_dns_status(), view()

### Community 7 - "features_apps.rs"
Cohesion: 0.09
Nodes (23): localize_ledger_action(), LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view() (+15 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.04
Nodes (3): PanickingBackend, SlowScanBackend, test_epoch_and_stale_scan_fencing_preserves_instant_mutation()

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.13
Nodes (37): escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry(), test_composite_string_merge_preserves_siblings(), test_hkcr_and_shallow_tree_delete_blocked() (+29 more)

### Community 11 - "startup.rs"
Cohesion: 0.07
Nodes (22): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate (+14 more)

### Community 12 - "theme.rs"
Cohesion: 0.07
Nodes (44): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, fmt1(), fmt2(), fmt3(), KFM_REDIRECT_MSG, localize_error_text(), localize_feature_install_state() (+36 more)

### Community 13 - "app.rs"
Cohesion: 0.07
Nodes (46): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+38 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "WinBlast Optimizer"
Cohesion: 0.08
Nodes (6): fallback_scheme_name(), is_on_ac_power(), resolve_indirect_string(), strip_indirect_prefix(), test_ac_power_status_and_enumerated_range_noop(), test_fallback_scheme_name_indirect_stripping()

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.09
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "WinBlast Optimizer"
Cohesion: 0.11
Nodes (21): assert_valid_permutation(), BitSet256, BitSetIter, find_min_weight_cycle_edge(), kahn_schedule(), position_of(), resolve_cycles(), SccResult (+13 more)

### Community 20 - "super"
Cohesion: 0.09
Nodes (3): ApplyCompleted, AutoTuneCompleted, LatencyBenchmarkCompleted

### Community 21 - "clean_apps.rs"
Cohesion: 0.14
Nodes (25): TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized, view(), render_compatible_card(), render_incompatible_card() (+17 more)

### Community 22 - "sys_inventory.rs"
Cohesion: 0.08
Nodes (10): panic_payload_to_string(), spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, CleanExecuted, ExplorerRestarted, OneDriveEdgeResult, RestorePointCreated (+2 more)

### Community 23 - "updater.rs"
Cohesion: 0.21
Nodes (13): apply_and_restart(), check_latest_release(), clean_exe_path(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO (+5 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (46): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+38 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.17
Nodes (26): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+18 more)

### Community 27 - "RealBackend"
Cohesion: 0.18
Nodes (21): BeforeAfterImpact, rgb(), ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score() (+13 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.15
Nodes (23): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+15 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.07
Nodes (7): RealBackend, resolve_persistent_app_dir(), is_hags_enabled(), is_mpo_disabled(), set_gpu_msi_mode(), set_hags(), set_mpo_disabled()

### Community 33 - "power.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.11
Nodes (3): SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 35 - "startup.rs"
Cohesion: 0.09
Nodes (21): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, FeatureToggled, GpuMsiUpdated (+13 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.16
Nodes (28): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+20 more)

### Community 38 - "parse_guid"
Cohesion: 0.08
Nodes (31): full_telemetry_audit(), full_telemetry_audit_with(), get_blocked_hosts(), KNOWN_TELEMETRY_SERVICES, parse_blocked_hosts_from_content(), scan_telemetry_agents(), TELEMETRY_HOSTS, TelemetryAuditResult (+23 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.17
Nodes (22): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+14 more)

### Community 40 - "parking.rs"
Cohesion: 0.12
Nodes (21): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+13 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (23): display_reg_value_name(), EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY (+15 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.12
Nodes (8): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak()

### Community 44 - "power.rs"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 45 - "features_apps.rs"
Cohesion: 0.16
Nodes (14): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), CURATED_WINGET_APPS, DISM_LOCK, extract_winget_error_detail(), find_onedrive_setup_exe(), is_progress_noise_char(), PROTECTED_CORE_PREFIXES (+6 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 47 - "Changelog"
Cohesion: 0.23
Nodes (18): validate_registry_mutation_path(), normalize_reg_value_name(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node() (+10 more)

### Community 49 - "deserialize"
Cohesion: 0.13
Nodes (5): test_windows_info_detect_host(), WindowsInfo, BiosSystemInfo, DisplayDriverInfo, GameProfile

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 52 - "list_live_services"
Cohesion: 0.27
Nodes (7): enum_scm_services_by_type(), list_live_services(), query_scm_service_config(), query_scm_win32_services(), resolve_indirect_display_string(), ScHandleGuard, ScmServiceRecord

### Community 53 - "backend.rs"
Cohesion: 0.16
Nodes (4): MetricsSampler, optional_feature_matches(), SandboxMetricsSampler, Result

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 55 - "RegRawSnapshot"
Cohesion: 0.15
Nodes (8): rgba(), UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 56 - "driver_update.rs"
Cohesion: 0.09
Nodes (11): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader, LazyPayload, Bios, Disks (+3 more)

### Community 57 - "probe_physical_drive"
Cohesion: 0.09
Nodes (23): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+15 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.11
Nodes (19): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, ExplorerRestarted, FeatureSet, GpuMsi, PowerCfgSet (+11 more)

### Community 60 - "backend.rs"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 62 - "AppxCache"
Cohesion: 0.11
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.13
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "task_scanner.rs"
Cohesion: 0.19
Nodes (11): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+3 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.10
Nodes (13): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme (+5 more)

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 68 - "AutoTuneProfile"
Cohesion: 0.12
Nodes (17): is_unsupported_power_guid_error(), apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CoreStatus, CPMAXCORES, CPMAXCORES1, CPMINCORES (+9 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.23
Nodes (13): escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml(), map_hive(), sanitize_computer_name(), sanitize_local_user() (+5 more)

### Community 70 - "TaskCache"
Cohesion: 0.12
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.21
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "WorkerEvent"
Cohesion: 0.16
Nodes (9): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, create_auto_tune_plan(), test_iso_29119_auto_tune_plan_generation_all_profiles() (+1 more)

### Community 74 - "app.rs"
Cohesion: 0.17
Nodes (16): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_startup_entries(), read_startup_approved_enabled(), replace_ascii_ci() (+8 more)

### Community 75 - "task_scanner.rs"
Cohesion: 0.06
Nodes (28): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), set_scheduled_task_enabled(), test_hosts_blocking_ignores_comments_and_subdomains() (+20 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "DriverUpdateItem"
Cohesion: 0.16
Nodes (16): DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), probe_physical_drive(), query_ata_smart_ioctl() (+8 more)

### Community 78 - ".setup"
Cohesion: 0.16
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "app.rs"
Cohesion: 0.03
Nodes (58): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle, AppMode, Advanced (+50 more)

### Community 80 - "deserialize"
Cohesion: 0.23
Nodes (10): install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind, DailyMemoryPurge, TelemetryGuard, WeeklyTempCleaner, MaintenanceTaskStatus, query_all_maintenance_tasks() (+2 more)

### Community 81 - "decode_oem_bytes"
Cohesion: 0.26
Nodes (10): decode_oem_bytes(), curated_windows_features_meta(), dism_error_from_output(), list_optional_features(), parse_dism_english_state(), query_dism_capabilities_locked(), query_dism_features_locked(), remove_edge_preserve_webview2() (+2 more)

### Community 83 - "normalize_guid_str"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 84 - "bridge.rs"
Cohesion: 0.09
Nodes (12): format_bytes(), SlintBridge, init_test_env(), test_all_26_views_navigation_and_mounting(), test_global_keyboard_shortcuts_and_actions(), test_main_window_properties_and_callbacks(), test_modals_state_and_lifecycle(), test_motion_constants_monotonicity() (+4 more)

### Community 85 - "enums"
Cohesion: 0.58
Nodes (6): build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header(), view()

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.23
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 88 - "UiThemeStyle"
Cohesion: 0.20
Nodes (8): detect_platform_security_info(), PlatformSecurityInfo, query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), test_ram_and_platform_inventory_nonblocking(), total_physical_ram_kb()

### Community 89 - "normalize_guid_str"
Cohesion: 0.20
Nodes (14): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+6 more)

### Community 90 - "RegStringValue"
Cohesion: 0.36
Nodes (3): RegStringValue, get_string(), get_string_for_vendor()

### Community 91 - "ThemePalette"
Cohesion: 0.23
Nodes (10): active_logical_core_count(), compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases() (+2 more)

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "WindowsOptionalFeature"
Cohesion: 0.14
Nodes (5): BiosInfoLoaded, CleanTargetsScanned, DiskReportLoaded, DriverScanCompleted, ThermalSnapshot

### Community 95 - "restore_point.rs"
Cohesion: 0.19
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats(), CreateRestorePoint

### Community 96 - "appx_scanner.rs"
Cohesion: 0.21
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 97 - "adaptive_tuning.rs"
Cohesion: 0.31
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 99 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.29
Nodes (5): InstalledAppxPackage, is_microsoft_appx_publisher(), list_installed_appx_from_cache(), list_installed_appx_from_cache_with_provisioned(), query_provisioned_appx_families()

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - ".sign"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 102 - "PowerSchemeInfo"
Cohesion: 0.14
Nodes (7): EngineMode, Instant, Normal, decode_hex(), test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports()

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.17
Nodes (3): check_elevation(), is_elevated(), relaunch_as_admin()

### Community 104 - "privacy_shield.rs"
Cohesion: 0.17
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 105 - "PlatformSecurityInfo"
Cohesion: 0.16
Nodes (7): CoreParkingUpdated, LatencyUpdated, MetricsSnapshot, RestorePointsLoaded, StatusUpdate, TelemetryAuditCompleted, UpdateCheckCompleted

### Community 106 - "PlatformSecurityInfo"
Cohesion: 0.31
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "RestorePoint"
Cohesion: 0.20
Nodes (11): assess_system_health(), MemoryTrimResult, OverallHealthGrade, Good, NeedsOptimization, Optimal, Suboptimal, SystemHealthAssessment (+3 more)

### Community 109 - "appx_scanner.rs"
Cohesion: 0.25
Nodes (10): appx_package_matches_family(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches(), test_split_appx_name_and_publisher_with_underscores_and_prefix(), is_appx_arch() (+2 more)

### Community 110 - "app.rs"
Cohesion: 0.31
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.17
Nodes (8): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS, test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 113 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.36
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 116 - "tools/game_overlay.rs"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.27
Nodes (3): GameOverlaySettings, GameSuitabilityReport, SuitabilityCheck

### Community 119 - "boot_config.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 120 - "power_governor.rs"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "progress_bar.rs"
Cohesion: 0.14
Nodes (5): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely(), RepairTask

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "WindowsInfo"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 127 - "AppxSafetyTier"
Cohesion: 0.15
Nodes (11): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, classify_appx_package(), extract_appx_short_name(), is_crockford_pub_id() (+3 more)

### Community 128 - "ui/mod.rs"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 130 - ".all_physical"
Cohesion: 0.27
Nodes (3): encode_hex(), RegRawSnapshot, get_raw()

### Community 131 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.16
Nodes (5): GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry, BenchmarkRunner, BenchmarkScore

### Community 132 - "Item"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 133 - "c_void"
Cohesion: 0.53
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 134 - "DateTime"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 139 - "path"
Cohesion: 0.21
Nodes (5): FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus, test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()

### Community 140 - "WinBlastScrollable"
Cohesion: 0.18
Nodes (8): metric_pill(), view(), rune::El, SCROLL_MIN_VELOCITY, SCROLL_MOMENTUM_FRICTION, SCROLL_VELOCITY_DECAY, winblast_scrollbar_style(), WinBlastScrollable

### Community 141 - "SlowScanBackend"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 144 - "BackendTweakStatus"
Cohesion: 0.22
Nodes (9): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, TweakShadow (+1 more)

### Community 145 - "test_status_i18n_all_languages_and_core_specs"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 146 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 147 - "UpdateStatus"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 148 - "test_disks.ps1"
Cohesion: 0.29
Nodes (5): GpuVendor, Amd, Intel, Nvidia, PresetGuards

### Community 150 - "sysprep_engine.rs"
Cohesion: 0.40
Nodes (4): DiskSubsystemReport, PhysicalDriveHealth, probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic()

### Community 154 - "PrereqInstallState"
Cohesion: 0.40
Nodes (5): PrereqInstallState, Failed, Installed, Installing, NotInstalled

### Community 156 - "PresetKind"
Cohesion: 0.50
Nodes (4): PresetKind, Gaming, LaptopBattery, ProAudio

### Community 158 - "clean_exe_path"
Cohesion: 0.60
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 159 - "views/network_diag.rs"
Cohesion: 0.70
Nodes (4): detect_screen_workarea(), main(), run_rune(), run_slint()

### Community 160 - "SignaturePolicy"
Cohesion: 0.50
Nodes (3): SignaturePolicy, AllowUnsigned, VerifyIfPresent

### Community 161 - "HealthSeverity"
Cohesion: 0.50
Nodes (4): HealthSeverity, Critical, Info, Warning

### Community 163 - "SilentCommand"
Cohesion: 0.50
Nodes (4): PowerActionKind, Hibernation, Scheme, UsbPower

### Community 166 - "RegStringValue"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

### Community 171 - "BitSet256"
Cohesion: 0.07
Nodes (7): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, WORDS

## Knowledge Gaps
- **1332 isolated node(s):** `Easy`, `Advanced`, `HardwareView`, `PrivacyShield`, `BackupCenter` (+1327 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1941 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **58 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `action_button_guarded`, `spawn_guarded_worker`, `.sync_startup_tasks_busy`, `WinBlast Optimizer — Developer & Architecture Guide`, `FailAtMutationBackend`, `features_apps.rs`, `enum_scm_services_by_type`, `HealthSeverity`, `startup.rs`, `startup.rs`, `path`, `app.rs`, `WinBlastScrollable`, `in_progress_hud.rs`, `UpdateStatus`, `super`, `clean_apps.rs`, `sys_inventory.rs`, `sysprep_engine.rs`, `System Architecture Changelog`, `.run_scan_internal`, `.Reg State() Vendor()`, `.set_status`, `RealBackend`, `LiveServiceInfo`, `power.rs`, `latency_bench.rs`, `startup.rs`, `registry_scanner.rs`, `parse_guid`, `AppX Package & Modern App Scanner`, `parking.rs`, `FailAtMutationBackend`, `deserialize`, `RegRawSnapshot`, `probe_physical_drive`, `.reset_baseline`, `.rollback_entry`, `backend.rs`, `Reversibility Instant Requireslogoff`, `dijkstra.rs`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `WorkerEvent`, `app.rs`, `.setup`, `app.rs`, `deserialize`, `decode_oem_bytes`, `normalize_guid_str`, `bridge.rs`, `enums`, `UiThemeStyle`, `FeatureInstallState`, `WindowsOptionalFeature`, `appx_scanner.rs`, `adaptive_tuning.rs`, `advanced_matrix.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `.sign`, `PowerSchemeInfo`, `privacy_shield.rs`, `PlatformSecurityInfo`, `PlatformSecurityInfo`, `PdhParkingCollector`, `RestorePoint`, `TaskCache`, `boot_config.rs`, `power_governor.rs`, `progress_bar.rs`?**
  _High betweenness centrality (0.259) - this node is a cross-community bridge._
- **Are the 25 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 25 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Easy`, `Advanced`, `HardwareView` to the rest of the system?**
  _1332 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `latency_bench.rs`, `Reversibility Instant Requireslogoff`, `parse_guid`, `features_apps.rs`, `theme.rs`, `app.rs`, `ui_scrollbar_test.rs`, `enums`, `clean_apps.rs`, `boot_config.rs`, `.run_scan_internal`, `.Reg State() Vendor()`, `RealBackend`?**
  _High betweenness centrality (0.232) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.01 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `Disk Cleaning & Temporary File Purger`, `features_apps.rs`, `WinBlastScrollable`, `ui_scrollbar_test.rs`, `enums`, `clean_apps.rs`, `RealBackend`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._