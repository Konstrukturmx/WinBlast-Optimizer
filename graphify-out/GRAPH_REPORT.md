# Graph Report - WinBlast0  (2026-11-25)

## Corpus Check
- 144 files · ~311,573 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 104 file(s) not represented in the graph (top: .slint 73, .toml 13, .err 6)
- WARNING: 104 of 276 files seen (38%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4993 nodes · 9827 edges · 284 communities (143 shown, 141 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 448 edges (avg confidence: 0.76)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `d0ab421b`
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
- automation.rs
- Scandetailkind Appxinstalled Appxinstalledps
- astar.rs
- theme.rs
- app.rs
- astar.rs
- power.rs
- in_progress_hud.rs
- full_telemetry_audit_with
- SandboxEnv
- topological_scheduler.rs
- super
- matrix.rs
- load_embedded_rules
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
- spawn_guarded_worker
- game_prereqs.rs
- parking.rs
- Embedded Catalog Test
- ps_runner.rs
- FailAtMutationBackend
- features_apps.rs
- svc_mutator.rs
- Changelog
- registry_scanner.rs
- Language
- updater.rs
- PlatformSecurityInfo
- ui_render_preview.rs
- CoreParkingState
- ui_views.rs
- driver_update.rs
- probe_physical_drive
- State Ledger & Restore Point Engine
- task_scanner.rs
- backend.rs
- driver_update.rs
- AppxCache
- WinBlast Optimizer
- FilterStrictSelection
- FeatureInstallState
- .maybe_arm_error
- ColorScheme
- palette.rs
- dijkstra.rs
- TaskCache
- ps_runner.rs
- net_latency.rs
- ui_slint/i18n.rs
- tools/startup.rs
- process.rs
- PlatformSecurityInfo
- sys_inventory.rs
- .setup
- app.rs
- parking.rs
- core/theme.rs
- HibernationStatus
- shell.rs
- componenthandle
- diff_sync.rs
- Runtime
- disks.rs
- service_scanner.rs
- RegStringValue
- views/automation.rs
- edge_cases_unorthodox.rs
- FeatureInstallState
- tbw.rs
- RestorePoint
- decode_oem_bytes
- DriverUpdateItem
- TaskCache
- InstalledAppxPackage
- Project History & Release Version Ledger
- backup.rs
- ui_parity.rs
- lazy_loader.rs
- edge_cases_unorthodox.rs
- window_icon.rs
- HibernationStatus
- PdhParkingCollector
- UpdateStatus
- Changelog
- font_coverage.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- storage.rs
- GamePrerequisiteItem
- tools/game_overlay.rs
- views/startup.rs
- lazy_loader.rs
- TaskCache
- gaming.rs
- sync_vec
- reg_helpers.rs
- PdhParkingCollector
- MonotonicCubicProgress
- adaptive_tuning.rs
- BitSet256
- elevation.rs
- SystemMetrics
- BiosSystemInfo
- WindowsOptionalFeature
- StartupEntry
- ui_layout_collision.rs
- path
- BackendTweakValue
- dijkstra.rs
- task_scanner.rs
- hardware.rs
- super
- RegRawSnapshot
- ColorScheme
- hashmap
- diff.rs
- views/latency.rs
- tools/automation.rs
- test_disks.ps1
- list_installed_appx_from_cache_with_provisioned
- Spring
- LiveServiceInfo
- adaptive_tuning.rs
- DriveMediaKind
- windows_repair.rs
- BackendTweakStatus
- LiveScheduledTaskInfo
- WinBlast Optimizer — Developer & Architecture Guide
- .sample
- views/network_diag.rs
- PresetKey
- dashboard.rs
- drivers.rs
- DiskSubsystemReport
- backend.rs
- planner.rs
- BitSet256
- serialize
- GameProfile
- sysprep_engine.rs
- .sample
- CleanTargetKind
- test_status_i18n_all_languages_and_core_specs
- enum_scm_services_by_type
- UpdateState
- PowerSchemeInfo
- delete_dir_contents
- StartupEntry
- ActiveDropdown
- BitSetIter
- NodeCost
- CleanTargetScan
- CleanAppsSubTab

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 658 edges
2. `AppState` - 346 edges
3. `ViewTextKey` - 235 edges
4. `SandboxEnv` - 158 edges
5. `SystemBackend` - 158 edges
6. `WorkerEvent` - 114 edges
7. `StatusMessageSpec` - 108 edges
8. `RealBackend` - 95 edges
9. `SlowScanBackend` - 85 edges
10. `PanickingBackend` - 81 edges

## Surprising Connections (you probably didn't know these)
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SystemBackend`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs

## Import Cycles
- 1-file cycle: `src/ui_sync/diff_sync.rs -> src/ui_sync/diff_sync.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/ui_slint/palette.rs -> src/ui_slint/shell.rs -> src/ui_slint/palette.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Communities (284 total, 141 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (633): TextKey, AboutBuiltWith, AboutNew1, AboutNew2, AboutNew3, AboutNew4, AboutNew5, AboutNew6 (+625 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (227): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+219 more)

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
Nodes (3): AppState, DiskReportLoaded, MemoryTrimmed

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.04
Nodes (3): PanickingBackend, SlowScanBackend, test_epoch_and_stale_scan_fencing_preserves_instant_mutation()

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.08
Nodes (56): is_unsupported_power_guid_error(), decode_hex(), decode_hex_rejects_non_ascii_without_panicking(), encode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs() (+48 more)

### Community 11 - "astar.rs"
Cohesion: 0.14
Nodes (8): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 12 - "theme.rs"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 13 - "app.rs"
Cohesion: 0.10
Nodes (32): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+24 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "power.rs"
Cohesion: 0.09
Nodes (22): create_and_activate_performance_plan(), fallback_scheme_name(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str() (+14 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.05
Nodes (41): simulate_preset_dry_run(), test_simulate_preset_dry_run_with_catalog(), load_embedded_rules(), apply_tweak_with(), test_apply_tweak_fallback_execution(), test_foreign_gpu_and_non_elevated_feature_and_partial_power_scan(), test_scan_single_tweak_in_sandbox(), test_unorthodox_concurrent_worker_event_stress() (+33 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.22
Nodes (14): assert_valid_permutation(), position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence(), test_max_capacity_256_nodes() (+6 more)

### Community 20 - "super"
Cohesion: 0.07
Nodes (7): ApplyCompleted, AutoTuneCompleted, BaselineReset, LatencyBenchmarkCompleted, PerfBenchmarkCompleted, RestorePointCreated, RollbackCompleted

### Community 21 - "matrix.rs"
Cohesion: 0.06
Nodes (31): MatrixQuery, MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status (+23 more)

### Community 22 - "load_embedded_rules"
Cohesion: 0.21
Nodes (20): scan_single_tweak_with(), diff(), guard(), hive(), key_values(), Maker, probe(), rollback_all() (+12 more)

### Community 23 - "updater.rs"
Cohesion: 0.12
Nodes (25): relaunch_as_admin(), apply_and_restart(), check_latest_release(), clean_exe_path(), CURL_HTTPS_ONLY, download_release_asset(), DOWNLOAD_URL_PREFIX, fetch_url_json() (+17 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (30): ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter, BackupCenter, CleanAndApps, CoreParking, DiskHealth (+22 more)

### Community 25 - "clean_apps.rs"
Cohesion: 0.23
Nodes (5): CleanAppsView, folded(), format_mb(), target_id(), target_label()

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.05
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 27 - "UiThemeStyle"
Cohesion: 0.08
Nodes (14): style_previews_follow_mode_and_scheme(), SurfaceSpec, ThemeMode, Dark, Light, ThemePalette, UiThemeStyle, .ALL (+6 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.14
Nodes (26): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+18 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (25): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+17 more)

### Community 33 - "cleaner.rs"
Cohesion: 0.10
Nodes (12): ComStaGuard, empty_recycle_bin(), ERROR_LOCK_VIOLATION, ERROR_SHARING_VIOLATION, ERROR_USER_MAPPED_FILE, execute_clean_targets(), FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT (+4 more)

### Community 35 - "startup.rs"
Cohesion: 0.04
Nodes (48): WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, BiosInfoLoaded, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted, CoreParkingUpdated (+40 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.13
Nodes (31): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+23 more)

### Community 37 - "spawn_guarded_worker"
Cohesion: 0.06
Nodes (14): spawn_guarded_worker(), Waker, WorkerDropGuard, AppxBatchRemoved, CleanExecuted, DiskTrimCompleted, ExplorerRestarted, FaultActionDone (+6 more)

### Community 39 - "game_prereqs.rs"
Cohesion: 0.19
Nodes (21): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+13 more)

### Community 40 - "parking.rs"
Cohesion: 0.10
Nodes (21): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+13 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (22): display_reg_value_name(), EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY (+14 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.09
Nodes (10): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH, SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_CLASS (+2 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.09
Nodes (9): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, .SCHEMA_VERSION, SystemBaseline, test_ledger_crud() (+1 more)

### Community 45 - "features_apps.rs"
Cohesion: 0.16
Nodes (16): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), CURATED_WINGET_APPS, DISM_LOCK, extract_winget_error_detail(), find_onedrive_setup_exe(), is_progress_noise_char(), PROTECTED_CORE_PREFIXES (+8 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.10
Nodes (24): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+16 more)

### Community 47 - "Changelog"
Cohesion: 0.21
Nodes (22): normalize_reg_value_name(), delete_key_if_empty(), delete_tree(), delete_value(), delete_value_for_vendor(), delete_value_literal(), REG_DELETE_RIGHT, restore_tree() (+14 more)

### Community 49 - "Language"
Cohesion: 0.13
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 52 - "ui_render_preview.rs"
Cohesion: 0.14
Nodes (6): H, Headless, render_all_views(), render_preview_frames(), settle_and_save(), W

### Community 53 - "CoreParkingState"
Cohesion: 0.19
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 55 - "ui_views.rs"
Cohesion: 0.12
Nodes (46): about_shows_real_version_and_style_studio_switches_style(), automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection(), clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox(), clean_apps_sub_tabs_drive_state(), clean_apps_winget_catalog_filter_and_selection() (+38 more)

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
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 60 - "backend.rs"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 61 - "driver_update.rs"
Cohesion: 0.12
Nodes (23): backup_root(), cim_and_store_json_parse(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), json_items(), json_str(), MAX_RETRIES (+15 more)

### Community 62 - "AppxCache"
Cohesion: 0.09
Nodes (7): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.14
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.14
Nodes (18): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+10 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.10
Nodes (7): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, test_compound_mutation_atomic_rollback_on_partial_failure()

### Community 67 - "ColorScheme"
Cohesion: 0.20
Nodes (7): ColorScheme, .ALL, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, COLOR_SCHEMES

### Community 68 - "palette.rs"
Cohesion: 0.18
Nodes (14): is_known_key(), ACTIONS, clean_label(), empty_query_lists_actions_first_without_tweak_row(), every_view_is_listed_once_and_resolves(), item(), MAX_RESULTS, query_matches_are_ranked_and_end_with_tweak_search() (+6 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.21
Nodes (13): escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml(), map_hive(), sanitize_computer_name(), sanitize_local_user() (+5 more)

### Community 70 - "TaskCache"
Cohesion: 0.13
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 71 - "ps_runner.rs"
Cohesion: 0.12
Nodes (17): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, hosts_file_path(), hosts_not_yet_blocked(), hosts_not_yet_blocked_lists_only_new_hosts() (+9 more)

### Community 72 - "net_latency.rs"
Cohesion: 0.06
Nodes (27): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+19 more)

### Community 73 - "ui_slint/i18n.rs"
Cohesion: 0.17
Nodes (8): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS, Ctx, apply_language(), install(), lookup(), Views

### Community 74 - "tools/startup.rs"
Cohesion: 0.19
Nodes (16): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), expand_system_root_vars_handles_multibyte_paths(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+8 more)

### Community 75 - "process.rs"
Cohesion: 0.15
Nodes (13): default_timeout(), hung_command_is_stopped_after_its_timeout(), kill_process_tree(), known_folder(), known_program_data_dir(), open_url_unelevated(), output_is_captured(), paths_and_unknown_names_are_unchanged() (+5 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "sys_inventory.rs"
Cohesion: 0.08
Nodes (10): detect_platform_security_info(), NtQuerySystemInformation(), query_live_code_integrity_options(), query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), SystemCodeIntegrityInfo (+2 more)

### Community 78 - ".setup"
Cohesion: 0.14
Nodes (13): cached_disk_report(), ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth (+5 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (54): AppMode, Advanced, Easy, BeforeAfterImpact, CleanSortMode, NameAsc, SizeAsc, SizeDesc (+46 more)

### Community 80 - "parking.rs"
Cohesion: 0.17
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.09
Nodes (40): ACCENT, ACCENT_HOVER, ACCENT_PRESS, accent_stays_distinct_from_danger_and_success(), BADGE_ALPHA, BG_CARD, BG_CARD_HOVER, BG_PRIMARY (+32 more)

### Community 83 - "shell.rs"
Cohesion: 0.12
Nodes (12): _assert_model_types(), elapsed_formatting(), every_view_name_resolves(), format_elapsed(), LANGUAGE_EN_NAMES, MAX_TOASTS, Shell, toast_tone() (+4 more)

### Community 84 - "componenthandle"
Cohesion: 0.12
Nodes (6): AboutView, REPO_NAME, REPO_URL, DiagnosticsView, ISSUES_URL, open_external()

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.26
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 88 - "disks.rs"
Cohesion: 0.09
Nodes (29): ata_fixture_goodram_cx400_g2_unknown_unit_is_not_available(), ata_receive_drive_data(), DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, hex_fixture(), NOT_REPORTED (+21 more)

### Community 89 - "service_scanner.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.23
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "tbw.rs"
Cohesion: 0.12
Nodes (23): controller_variants_without_phison_firmware_are_not_guessed(), crucial_bx100_bx200_are_not_matched(), F1, F6, is_phison_firmware(), is_virtual_disk_model(), known_families_resolve(), lookup_sata_write_counter() (+15 more)

### Community 96 - "decode_oem_bytes"
Cohesion: 0.29
Nodes (7): ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 97 - "DriverUpdateItem"
Cohesion: 0.27
Nodes (6): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_driver_sandbox_lifecycle()

### Community 98 - "TaskCache"
Cohesion: 0.15
Nodes (5): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes(), list_live_tasks_from_cache()

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 102 - "ui_parity.rs"
Cohesion: 0.17
Nodes (14): changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes(), easy_advanced_and_engine_mode_switches(), every_slint_translation_key_exists_in_the_catalogs(), every_theme_combination_reaches_slint(), harness(), hud_follows_background_work_and_can_be_dismissed(), language_switch_relocalizes_live_without_restart() (+6 more)

### Community 104 - "edge_cases_unorthodox.rs"
Cohesion: 0.12
Nodes (4): EngineMode, Instant, Normal, test_unorthodox_micro_and_ultrawide_viewports()

### Community 106 - "HibernationStatus"
Cohesion: 0.27
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "UpdateStatus"
Cohesion: 0.18
Nodes (9): UpdateStatus, Available, Checking, Downloading, Failed, Idle, ReadyToRestart, UpToDate (+1 more)

### Community 109 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 110 - "font_coverage.rs"
Cohesion: 0.31
Nodes (10): files(), font(), inter_covers_all_non_cjk_ui_letters(), missing(), mono_covers_ascii_and_turkish(), root(), TURKISH, ui_letters() (+2 more)

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.23
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "storage.rs"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 114 - "GamePrerequisiteItem"
Cohesion: 0.18
Nodes (7): catalog_undetected(), GamePrerequisiteItem, PrereqInstallState, Failed, Installed, Installing, NotInstalled

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.40
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 116 - "views/startup.rs"
Cohesion: 0.12
Nodes (18): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst, StartupStatusFilter, ActiveOnly (+10 more)

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.13
Nodes (9): GameOverlaySettings, GameSuitabilityReport, OverlaySetting, DvrDisabled, GameBarDisabled, GameModeEnabled, read_u32(), SuitabilityCheck (+1 more)

### Community 120 - "gaming.rs"
Cohesion: 0.12
Nodes (11): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, dns_category(), dns_status() (+3 more)

### Community 121 - "sync_vec"
Cohesion: 0.15
Nodes (6): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), DASH, SensorsView, temp_tone()

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "PdhParkingCollector"
Cohesion: 0.20
Nodes (5): compute_nt_cpu_deltas(), parse_pdh_processor_instance(), PdhParkingCollector, query_processor_group_bases(), test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 129 - "SystemMetrics"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 132 - "StartupEntry"
Cohesion: 0.21
Nodes (10): read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers (+2 more)

### Community 133 - "ui_layout_collision.rs"
Cohesion: 0.19
Nodes (14): check_view(), Harness, harness(), HEIGHT, is_text(), key(), no_text_collisions_or_overflow_at_any_width(), overlap_y() (+6 more)

### Community 134 - "path"
Cohesion: 0.25
Nodes (7): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, BLANK, scan(), ui_strings_only_use_glyphs_that_render()

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 136 - "dijkstra.rs"
Cohesion: 0.27
Nodes (8): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks(), MAX_NODES

### Community 137 - "task_scanner.rs"
Cohesion: 0.17
Nodes (17): create_restore_point(), set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line() (+9 more)

### Community 138 - "hardware.rs"
Cohesion: 0.26
Nodes (4): cpu_title(), HardwareView, kv(), power_plan_name()

### Community 139 - "super"
Cohesion: 0.12
Nodes (11): FaultAction, ClearCbsRebootFlags, ClearMinidumps, ClearPendingRenames, RestartSystem, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless() (+3 more)

### Community 143 - "ColorScheme"
Cohesion: 0.12
Nodes (9): MAX_ZOOM, MIN_ZOOM, out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings, apply_settings(), SlintBridge (+1 more)

### Community 144 - "hashmap"
Cohesion: 0.28
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

### Community 145 - "diff.rs"
Cohesion: 0.28
Nodes (13): diff_pair_card(), diff_single_card(), diff_view(), DiffRow, directed_pair(), header_with_dir(), rows_for(), Tone (+5 more)

### Community 146 - "views/latency.rs"
Cohesion: 0.29
Nodes (5): better(), comparison(), jitter_text(), LatencyView, ping_text()

### Community 147 - "tools/automation.rs"
Cohesion: 0.05
Nodes (34): assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, create_auto_tune_plan() (+26 more)

### Community 148 - "test_disks.ps1"
Cohesion: 0.29
Nodes (5): GpuVendor, Amd, Intel, Nvidia, PresetGuards

### Community 149 - "list_installed_appx_from_cache_with_provisioned"
Cohesion: 0.25
Nodes (11): classify_appx_package(), extract_appx_short_name(), is_appx_arch(), is_appx_version(), is_crockford_pub_id(), is_microsoft_appx_publisher(), list_installed_appx_from_cache(), list_installed_appx_from_cache_with_provisioned() (+3 more)

### Community 150 - "Spring"
Cohesion: 0.16
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 152 - "adaptive_tuning.rs"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 153 - "DriveMediaKind"
Cohesion: 0.12
Nodes (11): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, PlatformSecurityInfo (+3 more)

### Community 154 - "windows_repair.rs"
Cohesion: 0.33
Nodes (8): SilentCommand, decode_tool_output(), last_lines(), progress_lines_are_dropped(), RepairTask, run_tool(), summary(), utf16_output_is_decoded()

### Community 155 - "BackendTweakStatus"
Cohesion: 0.22
Nodes (9): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, TweakShadow (+1 more)

### Community 157 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 158 - ".sample"
Cohesion: 0.29
Nodes (8): active_logical_core_count(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), sample_live_cpu_and_parking(), SystemProcessorPerformanceInformation

### Community 159 - "views/network_diag.rs"
Cohesion: 0.33
Nodes (6): WorkerDropGuard<P>, acquire_single_instance(), detect_screen_workarea(), main(), run_slint(), show_already_running()

### Community 160 - "PresetKey"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

### Community 161 - "dashboard.rs"
Cohesion: 0.29
Nodes (6): category_buckets(), DashboardView, health_caption(), health_counts(), health_score(), health_tone()

### Community 163 - "drivers.rs"
Cohesion: 0.15
Nodes (10): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate, category_from_id() (+2 more)

### Community 165 - "DiskSubsystemReport"
Cohesion: 0.15
Nodes (9): DiskSubsystemReport, PhysicalDriveHealth, print_live_disk_report(), probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic(), HostWrites, NotAvailable, Reported (+1 more)

### Community 168 - "backend.rs"
Cohesion: 0.10
Nodes (6): optional_feature_matches(), SandboxMetricsSampler, SandboxTempDir, test_sandbox_core_parking_labels_metrics_and_power_sync(), test_windows_info_detect_host(), WindowsInfo

### Community 171 - "BitSet256"
Cohesion: 0.08
Nodes (7): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, WORDS

### Community 183 - "sysprep_engine.rs"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 184 - ".sample"
Cohesion: 0.36
Nodes (5): query_system_perf_info(), query_system_process_and_thread_counts(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 185 - "CleanTargetKind"
Cohesion: 0.14
Nodes (12): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+4 more)

### Community 186 - "test_status_i18n_all_languages_and_core_specs"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 188 - "enum_scm_services_by_type"
Cohesion: 0.39
Nodes (5): enum_scm_services_by_type(), query_scm_service_config(), query_scm_win32_services(), ScHandleGuard, ScmServiceRecord

### Community 263 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 265 - "delete_dir_contents"
Cohesion: 0.48
Nodes (7): delete_dir_contents(), dir_meta_if_real(), is_reparse_point(), measure_dir(), record_dir_io_error(), should_skip_entry(), test_delete_dir_preserves_dot_net_and_reparse_safe()

### Community 267 - "ActiveDropdown"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 275 - "CleanAppsSubTab"
Cohesion: 0.50
Nodes (4): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller

## Knowledge Gaps
- **1499 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `AboutBuiltWith`, `AboutNew1`, `AboutNew2` (+1494 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2278 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **141 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `planner.rs`, `ColorScheme`, `ui_slint/i18n.rs`, `app.rs`, `shell.rs`, `views/automation.rs`, `ui_parity.rs`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`, `sync_vec`?**
  _High betweenness centrality (0.264) - this node is a cross-community bridge._
- **Are the 24 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationGoal`) actually correct?**
  _`AppState` has 24 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `AboutBuiltWith` to the rest of the system?**
  _1499 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00315955766192733 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `AppState`, `FailAtMutationBackend`, `astar.rs`, `in_progress_hud.rs`, `super`, `matrix.rs`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `startup.rs`, `registry_scanner.rs`, `spawn_guarded_worker`, `parking.rs`, `FailAtMutationBackend`, `Language`, `CoreParkingState`, `ui_views.rs`, `probe_physical_drive`, `backend.rs`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `net_latency.rs`, `ui_slint/i18n.rs`, `sys_inventory.rs`, `.setup`, `app.rs`, `shell.rs`, `componenthandle`, `views/automation.rs`, `FeatureInstallState`, `RestorePoint`, `DriverUpdateItem`, `InstalledAppxPackage`, `backup.rs`, `ui_parity.rs`, `edge_cases_unorthodox.rs`, `HibernationStatus`, `PdhParkingCollector`, `UpdateStatus`, `Changelog`, `GamePrerequisiteItem`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `sync_vec`, `SystemMetrics`, `BiosSystemInfo`, `WindowsOptionalFeature`, `ui_layout_collision.rs`, `dijkstra.rs`, `hardware.rs`, `super`, `ColorScheme`, `hashmap`, `diff.rs`, `views/latency.rs`, `tools/automation.rs`, `LiveServiceInfo`, `DriveMediaKind`, `windows_repair.rs`, `LiveScheduledTaskInfo`, `WinBlast Optimizer — Developer & Architecture Guide`, `dashboard.rs`, `drivers.rs`, `DiskSubsystemReport`, `backend.rs`, `planner.rs`, `serialize`, `GameProfile`, `CleanTargetKind`, `PowerSchemeInfo`, `StartupEntry`, `ActiveDropdown`, `CleanTargetScan`, `CleanAppsSubTab`?**
  _High betweenness centrality (0.199) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.00881057268722467 - nodes in this community are weakly interconnected._
- **Why does `SandboxEnv` connect `SandboxEnv` to `BiosSystemInfo`, `WindowsOptionalFeature`, `ui_layout_collision.rs`, `FailAtMutationBackend`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `super`, `in_progress_hud.rs`, `tools/automation.rs`, `adaptive_tuning.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `Win32 Registry Scanner & Mutator`, `spawn_guarded_worker`, `registry_scanner.rs`, `DiskSubsystemReport`, `backend.rs`, `serialize`, `PlatformSecurityInfo`, `GameProfile`, `sysprep_engine.rs`, `ui_views.rs`, `task_scanner.rs`, `.maybe_arm_error`, `net_latency.rs`, `.setup`, `HibernationStatus`, `RegStringValue`, `edge_cases_unorthodox.rs`, `decode_oem_bytes`, `DriverUpdateItem`, `Project History & Release Version Ledger`, `ui_parity.rs`, `edge_cases_unorthodox.rs`, `GamePrerequisiteItem`, `TaskCache`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._