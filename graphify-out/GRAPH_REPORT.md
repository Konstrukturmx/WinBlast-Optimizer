# Graph Report - WinBlast0  (2026-11-24)

## Corpus Check
- 134 files · ~300,781 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 90 file(s) not represented in the graph (top: .slint 69, .toml 13, (none) 3)
- WARNING: 90 of 290 files seen (31%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4758 nodes · 9265 edges · 272 communities (139 shown, 133 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 379 edges (avg confidence: 0.74)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `c7b12feb`
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
- power.rs
- in_progress_hud.rs
- full_telemetry_audit_with
- SandboxEnv
- topological_scheduler.rs
- super
- matrix.rs
- sys_inventory.rs
- updater.rs
- System Architecture Changelog
- clean_apps.rs
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
- .navigate
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
- Project Documentation, Architecture & Licensing
- ui_layout_collision.rs
- tools/startup.rs
- ps_runner.rs
- PlatformSecurityInfo
- probe_physical_drive
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
- get_binary
- test_wave1_compound_actions_deep_edge_cases
- ThemePalette
- edge_cases_unorthodox.rs
- FeatureInstallState
- SlintBridge
- restore_point.rs
- appx_scanner.rs
- adaptive_tuning.rs
- parking.rs
- InstalledAppxPackage
- Project History & Release Version Ledger
- TaskCache
- ui_parity.rs
- Low-Level NT Core & Kernel Architecture
- CleanTargetKind
- window_icon.rs
- HibernationStatus
- PdhParkingCollector
- BitSet256
- sync_vec
- app.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- storage.rs
- progress_bar.rs
- tools/game_overlay.rs
- views/startup.rs
- lazy_loader.rs
- TaskCache
- gaming.rs
- Changelog
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
- PlatformSecurityInfo
- task_scanner.rs
- hardware.rs
- path
- SlowScanBackend
- UserSettings
- BackendTweakStatus
- diff.rs
- views/latency.rs
- UpdateStatus
- test_disks.ps1
- dijkstra.rs
- Spring
- hashmap
- .sync_startup_tasks_busy
- DriveMediaKind
- adaptive_tuning.rs
- CleanTargetResult
- LiveScheduledTaskInfo
- WinBlast Optimizer — Developer & Architecture Guide
- .sample
- views/network_diag.rs
- SignaturePolicy
- dashboard.rs
- drivers.rs
- DiskSubsystemReport
- CachedScanBackend<'a>
- planner.rs
- BitSet256
- LiveGameTelemetry
- GameProfile
- deserialize
- StackFmt<N>
- RestorePoint
- .new
- enum_scm_services_by_type
- WindowsOptionalFeature
- CleanAppsSubTab
- PresetKey
- BitSetIter
- NtQuerySystemInformation
- clean_exe_path
- NodeCost

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 658 edges
2. `AppState` - 344 edges
3. `ViewTextKey` - 213 edges
4. `SandboxEnv` - 152 edges
5. `SystemBackend` - 148 edges
6. `WorkerEvent` - 109 edges
7. `StatusMessageSpec` - 108 edges
8. `RealBackend` - 85 edges
9. `SlowScanBackend` - 85 edges
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
- 1-file cycle: `src/ui_sync/diff_sync.rs -> src/ui_sync/diff_sync.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/ui_slint/palette.rs -> src/ui_slint/shell.rs -> src/ui_slint/palette.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (272 total, 133 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (633): TextKey, AboutBuiltWith, AboutNew1, AboutNew2, AboutNew3, AboutNew4, AboutNew5, AboutNew6 (+625 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (205): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+197 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.02
Nodes (99): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+91 more)

### Community 3 - "AppState"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (3): AppState, AutoTuneCompleted, MemoryTrimmed

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.11
Nodes (42): is_unsupported_power_guid_error(), decode_hex(), decode_hex_rejects_non_ascii_without_panicking(), encode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs() (+34 more)

### Community 11 - "astar.rs"
Cohesion: 0.14
Nodes (8): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 12 - "theme.rs"
Cohesion: 0.06
Nodes (46): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, fmt1(), fmt2(), fmt3(), KFM_REDIRECT_MSG, localize_error_text(), localize_feature_install_state() (+38 more)

### Community 13 - "app.rs"
Cohesion: 0.06
Nodes (49): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+41 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "power.rs"
Cohesion: 0.09
Nodes (22): create_and_activate_performance_plan(), fallback_scheme_name(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str() (+14 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.07
Nodes (25): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_all_embedded_tweaks_full_sandbox_lifecycle(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording() (+17 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.22
Nodes (14): assert_valid_permutation(), position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence(), test_max_capacity_256_nodes() (+6 more)

### Community 20 - "super"
Cohesion: 0.08
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, relaunch_as_admin(), test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()

### Community 21 - "matrix.rs"
Cohesion: 0.08
Nodes (21): MatrixQuery, MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status (+13 more)

### Community 22 - "sys_inventory.rs"
Cohesion: 0.05
Nodes (18): spawn_guarded_worker(), Waker, WorkerDropGuard, AppxBatchRemoved, BaselineReset, CleanExecuted, DiskTrimCompleted, ExplorerRestarted (+10 more)

### Community 23 - "updater.rs"
Cohesion: 0.15
Nodes (18): check_latest_release(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO, is_version_newer(), ReleaseInfo (+10 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (32): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+24 more)

### Community 25 - "clean_apps.rs"
Cohesion: 0.23
Nodes (5): CleanAppsView, folded(), format_mb(), target_id(), target_label()

### Community 27 - "UiThemeStyle"
Cohesion: 0.11
Nodes (9): style_previews_follow_mode_and_scheme(), SurfaceSpec, UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic (+1 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.13
Nodes (24): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_dword(), get_dword_for_vendor(), get_raw() (+16 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 33 - "cleaner.rs"
Cohesion: 0.11
Nodes (12): delete_dir_contents(), dir_meta_if_real(), ERROR_LOCK_VIOLATION, ERROR_SHARING_VIOLATION, ERROR_USER_MAPPED_FILE, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT, is_reparse_point() (+4 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.08
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 35 - "startup.rs"
Cohesion: 0.06
Nodes (33): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted, CoreParkingUpdated (+25 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.13
Nodes (34): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+26 more)

### Community 37 - ".navigate"
Cohesion: 0.08
Nodes (11): BiosInfoLoaded, DriverScanCompleted, GameTelemetryProbed, LatencyUpdated, MetricsSnapshot, PrereqsScanned, RestorePointsLoaded, StatusUpdate (+3 more)

### Community 39 - "game_prereqs.rs"
Cohesion: 0.11
Nodes (28): catalog_undetected(), detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86() (+20 more)

### Community 40 - "parking.rs"
Cohesion: 0.10
Nodes (22): RollbackRefreshData, test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX (+14 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (20): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+12 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.12
Nodes (8): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak()

### Community 45 - "features_apps.rs"
Cohesion: 0.12
Nodes (26): remove_appx_package(), AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail() (+18 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.10
Nodes (24): Result, test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database (+16 more)

### Community 47 - "Changelog"
Cohesion: 0.22
Nodes (17): validate_registry_mutation_path(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary() (+9 more)

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 52 - "ui_render_preview.rs"
Cohesion: 0.07
Nodes (10): EngineMode, Instant, Normal, test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_micro_and_ultrawide_viewports(), H, Headless, render_preview_frames() (+2 more)

### Community 53 - "backend.rs"
Cohesion: 0.11
Nodes (4): SandboxMetricsSampler, PowerSchemeInfo, CleanTargetScan, LiveServiceInfo

### Community 55 - "ui_views.rs"
Cohesion: 0.09
Nodes (48): DiagnosticsView, ISSUES_URL, about_shows_real_version_and_style_studio_switches_style(), automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection(), clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox() (+40 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.09
Nodes (11): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader, LazyPayload, Bios, Disks (+3 more)

### Community 57 - "probe_physical_drive"
Cohesion: 0.08
Nodes (24): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+16 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 60 - "backend.rs"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 62 - "AppxCache"
Cohesion: 0.09
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.13
Nodes (8): MAX_TWEAKS, ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.15
Nodes (17): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+9 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.10
Nodes (13): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme (+5 more)

### Community 67 - "ColorScheme"
Cohesion: 0.13
Nodes (9): accent_stays_distinct_from_danger_and_success(), ColorScheme, .ALL, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SchemeSpec (+1 more)

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

### Community 73 - "ui_layout_collision.rs"
Cohesion: 0.18
Nodes (14): check_view(), Harness, harness(), HEIGHT, is_text(), key(), no_text_collisions_or_overflow_at_any_width(), overlap_y() (+6 more)

### Community 74 - "tools/startup.rs"
Cohesion: 0.18
Nodes (16): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), expand_system_root_vars_handles_multibyte_paths(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+8 more)

### Community 75 - "ps_runner.rs"
Cohesion: 0.26
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "probe_physical_drive"
Cohesion: 0.16
Nodes (16): DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), probe_physical_drive(), query_ata_smart_ioctl() (+8 more)

### Community 78 - ".setup"
Cohesion: 0.15
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (51): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle, BeforeAfterImpact, CleanSortMode (+43 more)

### Community 80 - "tools/automation.rs"
Cohesion: 0.05
Nodes (36): available_physical_ram_kb(), assess_system_health(), AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability (+28 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.10
Nodes (38): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BADGE_ALPHA, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR (+30 more)

### Community 83 - "shell.rs"
Cohesion: 0.11
Nodes (11): _assert_model_types(), elapsed_formatting(), every_view_name_resolves(), format_elapsed(), LANGUAGE_EN_NAMES, MAX_TOASTS, Shell, THEME_STYLES (+3 more)

### Community 84 - "Ctx"
Cohesion: 0.13
Nodes (9): Ctx, apply_language(), install(), lookup(), AboutView, REPO_NAME, REPO_URL, open_external() (+1 more)

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (8): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakShadow, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 89 - "get_binary"
Cohesion: 0.40
Nodes (3): get_binary(), ScratchCleanup, test_strict_reg_type_dispatch_and_negative_numeric_sz()

### Community 91 - "ThemePalette"
Cohesion: 0.38
Nodes (5): compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtQuerySystemInformation(), query_nt_processor_performance_into(), SystemProcessorPerformanceInformation

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.23
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 95 - "restore_point.rs"
Cohesion: 0.25
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 96 - "appx_scanner.rs"
Cohesion: 0.12
Nodes (9): DNS_CATALOG, DnsProbeResult, NetAction, ApplyDns, FlushDns, ResetDnsDhcp, NetworkDiagnostics, query_local_ip() (+1 more)

### Community 97 - "adaptive_tuning.rs"
Cohesion: 0.22
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 98 - "parking.rs"
Cohesion: 0.10
Nodes (17): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CoreParkingState, CoreStatus, CPMAXCORES, CPMAXCORES1, CPMINCORES (+9 more)

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.16
Nodes (9): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, is_microsoft_appx_publisher(), list_installed_appx_from_cache() (+1 more)

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - "TaskCache"
Cohesion: 0.18
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 102 - "ui_parity.rs"
Cohesion: 0.17
Nodes (17): AppMode, Advanced, Easy, changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes(), easy_advanced_and_engine_mode_switches(), every_slint_translation_key_exists_in_the_catalogs(), every_theme_combination_reaches_slint() (+9 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.18
Nodes (3): check_elevation(), is_elevated(), apply_and_restart()

### Community 104 - "CleanTargetKind"
Cohesion: 0.14
Nodes (11): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+3 more)

### Community 106 - "HibernationStatus"
Cohesion: 0.20
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 109 - "sync_vec"
Cohesion: 0.11
Nodes (7): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), BackupView, DASH, SensorsView, temp_tone()

### Community 110 - "app.rs"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "storage.rs"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.56
Nodes (7): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name()

### Community 116 - "views/startup.rs"
Cohesion: 0.10
Nodes (22): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst, StartupStatusFilter, ActiveOnly (+14 more)

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.13
Nodes (9): GameOverlaySettings, GameSuitabilityReport, OverlaySetting, DvrDisabled, GameBarDisabled, GameModeEnabled, read_u32(), SuitabilityCheck (+1 more)

### Community 120 - "gaming.rs"
Cohesion: 0.12
Nodes (11): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, dns_category(), dns_status() (+3 more)

### Community 121 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "restart_explorer_safely"
Cohesion: 0.25
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "WindowsInfo"
Cohesion: 0.31
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 127 - "ThemeMode"
Cohesion: 0.20
Nodes (6): ThemeMode, Dark, Light, ThemePalette, apply(), to_slint()

### Community 129 - "SystemMetrics"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 132 - "StartupEntry"
Cohesion: 0.14
Nodes (13): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), StartupEntry, StartupSource, RegistryHkcuRun (+5 more)

### Community 133 - "c_void"
Cohesion: 0.25
Nodes (7): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, BLANK, scan(), ui_strings_only_use_glyphs_that_render()

### Community 134 - "DateTime"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 136 - "PlatformSecurityInfo"
Cohesion: 0.18
Nodes (8): detect_platform_security_info(), PlatformSecurityInfo, query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name(), test_ram_and_platform_inventory_nonblocking(), total_physical_ram_kb()

### Community 137 - "task_scanner.rs"
Cohesion: 0.21
Nodes (14): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed() (+6 more)

### Community 138 - "hardware.rs"
Cohesion: 0.15
Nodes (5): AutomationView, cpu_title(), HardwareView, kv(), power_plan_name()

### Community 139 - "path"
Cohesion: 0.13
Nodes (10): FaultAction, ClearCbsRebootFlags, ClearMinidumps, ClearPendingRenames, RestartSystem, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless() (+2 more)

### Community 141 - "SlowScanBackend"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 143 - "UserSettings"
Cohesion: 0.21
Nodes (6): MAX_ZOOM, MIN_ZOOM, out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings

### Community 144 - "BackendTweakStatus"
Cohesion: 0.25
Nodes (8): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, TweakStatus

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

### Community 149 - "dijkstra.rs"
Cohesion: 0.27
Nodes (8): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks(), MAX_NODES

### Community 150 - "Spring"
Cohesion: 0.16
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 151 - "hashmap"
Cohesion: 0.28
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

### Community 152 - ".sync_startup_tasks_busy"
Cohesion: 0.17
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 153 - "DriveMediaKind"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 154 - "adaptive_tuning.rs"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 155 - "CleanTargetResult"
Cohesion: 0.24
Nodes (8): CleanTargetResult, ComStaGuard, empty_recycle_bin(), execute_clean_targets(), init_com_sta(), query_recycle_bin(), resolve_target_paths(), scan_all_clean_targets()

### Community 157 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 158 - ".sample"
Cohesion: 0.27
Nodes (8): active_logical_core_count(), NtCpuSampler, query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 159 - "views/network_diag.rs"
Cohesion: 0.38
Nodes (5): WorkerDropGuard<P>, detect_screen_workarea(), main(), run_slint(), apply_settings()

### Community 160 - "SignaturePolicy"
Cohesion: 0.50
Nodes (3): SignaturePolicy, AllowUnsigned, VerifyIfPresent

### Community 161 - "dashboard.rs"
Cohesion: 0.26
Nodes (6): category_buckets(), DashboardView, health_caption(), health_counts(), health_score(), health_tone()

### Community 163 - "drivers.rs"
Cohesion: 0.27
Nodes (3): category_from_id(), DriversView, FILTER_CATEGORIES

### Community 165 - "DiskSubsystemReport"
Cohesion: 0.29
Nodes (4): DiskSubsystemReport, PhysicalDriveHealth, probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic()

### Community 171 - "BitSet256"
Cohesion: 0.08
Nodes (6): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), WORDS

### Community 183 - "deserialize"
Cohesion: 0.11
Nodes (6): DisplayDriverInfo, GameFpsComparison, get_standard_game_benchmarks(), BenchmarkRunner, BenchmarkScore, RepairTask

### Community 187 - "enum_scm_services_by_type"
Cohesion: 0.70
Nodes (3): enum_scm_services_by_type(), query_scm_win32_services(), ScmServiceRecord

### Community 259 - "CleanAppsSubTab"
Cohesion: 0.50
Nodes (4): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller

### Community 260 - "PresetKey"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

## Knowledge Gaps
- **1473 isolated node(s):** `Easy`, `Advanced`, `HardwareView`, `PrivacyShield`, `BackupCenter` (+1468 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2201 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **133 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `AppState`, `FailAtMutationBackend`, `astar.rs`, `app.rs`, `in_progress_hud.rs`, `super`, `matrix.rs`, `sys_inventory.rs`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `startup.rs`, `registry_scanner.rs`, `.navigate`, `game_prereqs.rs`, `parking.rs`, `FailAtMutationBackend`, `WindowsInfo`, `ui_render_preview.rs`, `backend.rs`, `ui_views.rs`, `probe_physical_drive`, `backend.rs`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `ui_layout_collision.rs`, `.setup`, `app.rs`, `tools/automation.rs`, `shell.rs`, `Ctx`, `FeatureInstallState`, `SlintBridge`, `appx_scanner.rs`, `adaptive_tuning.rs`, `parking.rs`, `InstalledAppxPackage`, `ui_parity.rs`, `CleanTargetKind`, `HibernationStatus`, `PdhParkingCollector`, `sync_vec`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `Changelog`, `ThemeMode`, `SystemMetrics`, `StartupEntry`, `PlatformSecurityInfo`, `hardware.rs`, `path`, `UserSettings`, `diff.rs`, `views/latency.rs`, `UpdateStatus`, `dijkstra.rs`, `hashmap`, `.sync_startup_tasks_busy`, `DriveMediaKind`, `LiveScheduledTaskInfo`, `WinBlast Optimizer — Developer & Architecture Guide`, `views/network_diag.rs`, `dashboard.rs`, `drivers.rs`, `DiskSubsystemReport`, `planner.rs`, `LiveGameTelemetry`, `GameProfile`, `deserialize`, `RestorePoint`, `WindowsOptionalFeature`, `CleanAppsSubTab`, `clean_exe_path`?**
  _High betweenness centrality (0.268) - this node is a cross-community bridge._
- **Are the 24 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationGoal`) actually correct?**
  _`AppState` has 24 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Easy`, `Advanced`, `HardwareView` to the rest of the system?**
  _1473 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00315955766192733 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `planner.rs`, `ColorScheme`, `app.rs`, `tools/automation.rs`, `shell.rs`, `Ctx`, `ui_parity.rs`, `sync_vec`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`?**
  _High betweenness centrality (0.264) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.00975609756097561 - nodes in this community are weakly interconnected._
- **Why does `SystemBackend` connect `FailAtMutationBackend` to `SystemMetrics`, `.all_physical`, `StartupEntry`, `Disk Cleaning & Temporary File Purger`, `PlatformSecurityInfo`, `Integration Suite: Tweak Sandbox E2E`, `Scandetailkind Appxinstalled Appxinstalledps`, `path`, `in_progress_hud.rs`, `SandboxEnv`, `adaptive_tuning.rs`, `LiveScheduledTaskInfo`, `WinBlast Optimizer — Developer & Architecture Guide`, `Win32 Registry Scanner & Mutator`, `RealBackend`, `spawn_guarded_worker`, `registry_scanner.rs`, `DiskSubsystemReport`, `game_prereqs.rs`, `CachedScanBackend<'a>`, `parking.rs`, `LiveGameTelemetry`, `WindowsInfo`, `backend.rs`, `GameProfile`, `deserialize`, `RestorePoint`, `State Ledger & Restore Point Engine`, `task_scanner.rs`, `WindowsOptionalFeature`, `probe_physical_drive`, `backend.rs`, `FeatureInstallState`, `.maybe_arm_error`, `Project Documentation, Architecture & Licensing`, `ps_runner.rs`, `.setup`, `app.rs`, `tools/automation.rs`, `HibernationStatus`, `Runtime`, `test_wave1_compound_actions_deep_edge_cases`, `edge_cases_unorthodox.rs`, `FeatureInstallState`, `restore_point.rs`, `appx_scanner.rs`, `adaptive_tuning.rs`, `parking.rs`, `InstalledAppxPackage`, `Project History & Release Version Ledger`, `TaskCache`, `CleanTargetKind`, `HibernationStatus`, `PdhParkingCollector`, `TaskCache`, `Changelog`, `WindowsInfo`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._