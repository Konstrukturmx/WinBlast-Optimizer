# Graph Report - WinBlast0  (2026-11-25)

## Corpus Check
- 142 files · ~314,597 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 99 file(s) not represented in the graph (top: .slint 69, .toml 13, .err 6)
- WARNING: 99 of 309 files seen (32%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 4846 nodes · 9473 edges · 276 communities (141 shown, 135 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 408 edges (avg confidence: 0.75)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `4ccc5f67`
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
- load_embedded_rules
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
- WindowsInfo
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
- Language
- tools/startup.rs
- ps_runner.rs
- PlatformSecurityInfo
- .setup
- app.rs
- tools/automation.rs
- core/theme.rs
- HibernationStatus
- shell.rs
- componenthandle
- diff_sync.rs
- Runtime
- disks.rs
- open_subkey_read_64
- test_wave1_compound_actions_deep_edge_cases
- AutoTuneProfile
- edge_cases_unorthodox.rs
- FeatureInstallState
- tbw.rs
- restore_point.rs
- appx_scanner.rs
- DriverUpdateItem
- .navigate
- InstalledAppxPackage
- Project History & Release Version Ledger
- ui_slint/i18n.rs
- ui_parity.rs
- Low-Level NT Core & Kernel Architecture
- CleanTargetKind
- window_icon.rs
- HibernationStatus
- PdhParkingCollector
- parse_guid
- sync_vec
- parking.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- storage.rs
- progress_bar.rs
- tools/game_overlay.rs
- views/startup.rs
- lazy_loader.rs
- TaskCache
- gaming.rs
- Ctx
- reg_helpers.rs
- restart_explorer_safely
- MonotonicCubicProgress
- adaptive_tuning.rs
- BitSet256
- font_coverage.rs
- SystemMetrics
- .all_physical
- Changelog
- StartupEntry
- ui_layout_collision.rs
- DateTime
- BackendTweakStatus
- dijkstra.rs
- task_scanner.rs
- hardware.rs
- path
- SlowScanBackend
- ColorScheme
- hashmap
- diff.rs
- views/latency.rs
- UpdateStatus
- test_disks.ps1
- check_view
- Spring
- LiveServiceInfo
- .sync_startup_tasks_busy
- DriveMediaKind
- CleanTargetScan
- CleanTargetResult
- LiveScheduledTaskInfo
- BiosSystemInfo
- .sample
- views/network_diag.rs
- SignaturePolicy
- dashboard.rs
- drivers.rs
- DiskSubsystemReport
- backend.rs
- planner.rs
- BitSet256
- LiveGameTelemetry
- GameProfile
- super
- .sample
- views/gpu.rs
- WinBlast Optimizer — Developer & Architecture Guide
- PowerSchemeInfo
- WindowsOptionalFeature
- UpdateState
- StackFmt<N>
- .new
- path
- NetAction
- ui_slint/theme.rs
- BitSetIter
- NodeCost

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 658 edges
2. `AppState` - 345 edges
3. `ViewTextKey` - 215 edges
4. `SandboxEnv` - 152 edges
5. `SystemBackend` - 149 edges
6. `WorkerEvent` - 109 edges
7. `StatusMessageSpec` - 108 edges
8. `RealBackend` - 88 edges
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

## Communities (276 total, 135 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (633): TextKey, AboutBuiltWith, AboutNew1, AboutNew2, AboutNew3, AboutNew4, AboutNew5, AboutNew6 (+625 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (207): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+199 more)

### Community 2 - "spawn_guarded_worker"
Cohesion: 0.02
Nodes (101): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+93 more)

### Community 3 - "AppState"
Cohesion: 0.07
Nodes (22): aot_prewarm_and_zero_alloc_borrowed_reads(), atomic_epoch_invalidation_fences_all_entries_in_o1(), CacheMetrics, CacheMetricsSnapshot, CachePadded, CacheSet, CacheSet<K, V>, CacheSlot (+14 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.04
Nodes (8): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, AutoTuneCompleted, DiskReportLoaded, MemoryTrimmed

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.11
Nodes (44): LedgerEntry, is_unsupported_power_guid_error(), Result, decode_hex(), decode_hex_rejects_non_ascii_without_panicking(), encode_hex(), escape_pipe(), format_pipe_reg_target() (+36 more)

### Community 11 - "astar.rs"
Cohesion: 0.14
Nodes (8): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 12 - "theme.rs"
Cohesion: 0.05
Nodes (36): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, PresetKey, Gaming, LaptopBattery, ProAudio (+28 more)

### Community 13 - "app.rs"
Cohesion: 0.09
Nodes (35): localize_ledger_action(), CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci() (+27 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "power.rs"
Cohesion: 0.08
Nodes (6): fallback_scheme_name(), query_hibernation_and_standby(), resolve_indirect_string(), set_hibernation_enabled(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.08
Nodes (23): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+15 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.22
Nodes (14): assert_valid_permutation(), position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence(), test_max_capacity_256_nodes() (+6 more)

### Community 20 - "super"
Cohesion: 0.07
Nodes (10): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted, PerfBenchmarkCompleted (+2 more)

### Community 21 - "matrix.rs"
Cohesion: 0.08
Nodes (29): MatrixQuery, MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status (+21 more)

### Community 22 - "load_embedded_rules"
Cohesion: 0.13
Nodes (19): diff(), guard(), hive(), key_values(), Maker, probe(), rollback_all(), temp_ledger() (+11 more)

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
Cohesion: 0.08
Nodes (13): style_previews_follow_mode_and_scheme(), SurfaceSpec, ThemeMode, Dark, Light, ThemePalette, UiThemeStyle, .ALL (+5 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "reg_mutator.rs"
Cohesion: 0.12
Nodes (20): check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind(), get_binary(), get_dword(), get_dword_for_vendor() (+12 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.08
Nodes (24): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+16 more)

### Community 33 - "cleaner.rs"
Cohesion: 0.12
Nodes (12): delete_dir_contents(), dir_meta_if_real(), ERROR_LOCK_VIOLATION, ERROR_SHARING_VIOLATION, ERROR_USER_MAPPED_FILE, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT, is_reparse_point() (+4 more)

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.08
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 35 - "startup.rs"
Cohesion: 0.06
Nodes (33): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, DriverItemUpdated (+25 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.22
Nodes (22): RiskValuation, TweakDefinition, evaluate_tweak(), evaluate_tweak_status(), make_scan_result(), mk_test_tweak(), run_full_scan(), run_full_scan_with() (+14 more)

### Community 37 - "spawn_guarded_worker"
Cohesion: 0.06
Nodes (15): spawn_guarded_worker(), Waker, WorkerDropGuard, AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, DiskTrimCompleted, ExplorerRestarted (+7 more)

### Community 39 - "game_prereqs.rs"
Cohesion: 0.09
Nodes (29): SilentCommand, catalog_undetected(), detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64() (+21 more)

### Community 40 - "parking.rs"
Cohesion: 0.10
Nodes (21): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+13 more)

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
Cohesion: 0.10
Nodes (32): create_restore_point(), remove_appx_package(), set_scheduled_task_enabled(), decode_oem_bytes(), request_system_reboot(), AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package() (+24 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 47 - "Changelog"
Cohesion: 0.17
Nodes (19): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node() (+11 more)

### Community 50 - "updater.rs"
Cohesion: 0.21
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), test_canonical_encode_decode_roundtrip_unsigned(), test_digest_mismatch_tampering_detection() (+4 more)

### Community 51 - "PlatformSecurityInfo"
Cohesion: 0.19
Nodes (3): SandboxState, test_hkcr_and_shallow_tree_delete_blocked(), validate_registry_mutation_path()

### Community 52 - "ui_render_preview.rs"
Cohesion: 0.08
Nodes (7): test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_micro_and_ultrawide_viewports(), H, Headless, render_preview_frames(), settle_and_save(), W

### Community 53 - "CoreParkingState"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 55 - "ui_views.rs"
Cohesion: 0.08
Nodes (49): DASH, SensorsView, temp_tone(), about_shows_real_version_and_style_studio_switches_style(), automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection() (+41 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.10
Nodes (11): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader, LazyPayload, Bios, Disks (+3 more)

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
Cohesion: 0.07
Nodes (28): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+20 more)

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.14
Nodes (21): backup_root(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), estimate_download_size(), evaluate_version_from_store(), MAX_RETRIES, parse_pnputil_devices() (+13 more)

### Community 62 - "AppxCache"
Cohesion: 0.12
Nodes (5): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.14
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.14
Nodes (17): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+9 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.12
Nodes (7): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, test_compound_mutation_atomic_rollback_on_partial_failure()

### Community 67 - "ColorScheme"
Cohesion: 0.15
Nodes (9): accent_stays_distinct_from_danger_and_success(), ColorScheme, .ALL, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SchemeSpec (+1 more)

### Community 68 - "palette.rs"
Cohesion: 0.19
Nodes (14): is_known_key(), ACTIONS, clean_label(), empty_query_lists_actions_first_without_tweak_row(), every_view_is_listed_once_and_resolves(), item(), MAX_RESULTS, query_matches_are_ranked_and_end_with_tweak_search() (+6 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 70 - "TaskCache"
Cohesion: 0.11
Nodes (15): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+7 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.19
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "Language"
Cohesion: 0.14
Nodes (24): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+16 more)

### Community 74 - "tools/startup.rs"
Cohesion: 0.15
Nodes (19): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), expand_system_root_vars_handles_multibyte_paths(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service() (+11 more)

### Community 75 - "ps_runner.rs"
Cohesion: 0.26
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.24
Nodes (7): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64

### Community 78 - ".setup"
Cohesion: 0.14
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (56): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle, BeforeAfterImpact, CleanSortMode (+48 more)

### Community 80 - "tools/automation.rs"
Cohesion: 0.08
Nodes (26): available_physical_ram_kb(), assess_system_health(), HealthSeverity, Critical, Info, Warning, install_maintenance_task(), is_maintenance_task_installed() (+18 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.10
Nodes (38): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BADGE_ALPHA, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR (+30 more)

### Community 83 - "shell.rs"
Cohesion: 0.11
Nodes (12): EngineMode, Instant, Normal, _assert_model_types(), elapsed_formatting(), format_elapsed(), LANGUAGE_EN_NAMES, MAX_TOASTS (+4 more)

### Community 84 - "componenthandle"
Cohesion: 0.11
Nodes (6): AboutView, REPO_NAME, REPO_URL, DiagnosticsView, ISSUES_URL, open_external()

### Community 86 - "diff_sync.rs"
Cohesion: 0.14
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.21
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 88 - "disks.rs"
Cohesion: 0.05
Nodes (36): ata_fixture_goodram_cx400_g2_unknown_unit_is_not_available(), ata_receive_drive_data(), DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, hex_fixture(), NOT_REPORTED (+28 more)

### Community 89 - "open_subkey_read_64"
Cohesion: 0.15
Nodes (14): get_raw(), key_exists(), open_subkey_read_64(), reg_type_to_u32(), snapshot_key_recursive(), snapshot_tree(), check_service_start_type(), evaluate_service() (+6 more)

### Community 91 - "AutoTuneProfile"
Cohesion: 0.15
Nodes (10): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, create_auto_tune_plan(), test_iso_29119_auto_tune_plan_generation_all_profiles() (+2 more)

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 93 - "FeatureInstallState"
Cohesion: 0.43
Nodes (5): classify_efficiency_records(), query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 94 - "tbw.rs"
Cohesion: 0.12
Nodes (23): controller_variants_without_phison_firmware_are_not_guessed(), crucial_bx100_bx200_are_not_matched(), F1, F6, is_phison_firmware(), is_virtual_disk_model(), known_families_resolve(), lookup_sata_write_counter() (+15 more)

### Community 95 - "restore_point.rs"
Cohesion: 0.22
Nodes (8): ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), RestorePoint, test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 96 - "appx_scanner.rs"
Cohesion: 0.17
Nodes (5): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip(), read_tcp_overrides()

### Community 97 - "DriverUpdateItem"
Cohesion: 0.22
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 98 - ".navigate"
Cohesion: 0.09
Nodes (8): BiosInfoLoaded, DriverScanCompleted, FaultReportLoaded, GameTelemetryProbed, MetricsSnapshot, PrereqsScanned, ThermalSnapshot, UpdateCheckCompleted

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.18
Nodes (7): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, list_installed_appx_from_cache()

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - "ui_slint/i18n.rs"
Cohesion: 0.29
Nodes (6): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS, apply_language(), install(), lookup()

### Community 102 - "ui_parity.rs"
Cohesion: 0.17
Nodes (17): AppMode, Advanced, Easy, changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes(), easy_advanced_and_engine_mode_switches(), every_slint_translation_key_exists_in_the_catalogs(), every_theme_combination_reaches_slint() (+9 more)

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.18
Nodes (6): check_elevation(), is_elevated(), relaunch_as_admin(), apply_and_restart(), clean_exe_path(), test_clean_windows_exe_path()

### Community 104 - "CleanTargetKind"
Cohesion: 0.14
Nodes (12): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+4 more)

### Community 108 - "parse_guid"
Cohesion: 0.24
Nodes (18): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+10 more)

### Community 109 - "sync_vec"
Cohesion: 0.18
Nodes (4): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), AutomationView

### Community 110 - "parking.rs"
Cohesion: 0.17
Nodes (13): apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1, ParkingSnapshot (+5 more)

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "storage.rs"
Cohesion: 0.16
Nodes (18): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd() (+10 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 115 - "tools/game_overlay.rs"
Cohesion: 0.49
Nodes (8): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), open_subkey_flags_64()

### Community 116 - "views/startup.rs"
Cohesion: 0.16
Nodes (14): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst, build_rows(), is_run_once() (+6 more)

### Community 117 - "lazy_loader.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.13
Nodes (9): GameOverlaySettings, GameSuitabilityReport, OverlaySetting, DvrDisabled, GameBarDisabled, GameModeEnabled, read_u32(), SuitabilityCheck (+1 more)

### Community 120 - "gaming.rs"
Cohesion: 0.12
Nodes (11): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, dns_category(), dns_status() (+3 more)

### Community 121 - "Ctx"
Cohesion: 0.19
Nodes (3): Ctx, BackupView, Views

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "restart_explorer_safely"
Cohesion: 0.20
Nodes (5): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely(), RepairTask

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 127 - "font_coverage.rs"
Cohesion: 0.25
Nodes (10): files(), font(), inter_covers_all_non_cjk_ui_letters(), missing(), mono_covers_ascii_and_turkish(), root(), TURKISH, ui_letters() (+2 more)

### Community 129 - "SystemMetrics"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 131 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 132 - "StartupEntry"
Cohesion: 0.14
Nodes (13): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), StartupEntry, StartupSource, RegistryHkcuRun (+5 more)

### Community 133 - "ui_layout_collision.rs"
Cohesion: 0.21
Nodes (9): Harness, harness(), HEIGHT, is_text(), no_text_collisions_or_overflow_at_any_width(), overlap_y(), settle(), top_bar_fits_in_every_language_at_every_width() (+1 more)

### Community 134 - "DateTime"
Cohesion: 0.29
Nodes (3): parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 135 - "BackendTweakStatus"
Cohesion: 0.11
Nodes (18): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, BackendTweakValue (+10 more)

### Community 136 - "dijkstra.rs"
Cohesion: 0.27
Nodes (8): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks(), MAX_NODES

### Community 137 - "task_scanner.rs"
Cohesion: 0.13
Nodes (15): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), normalize_task_path(), parse_csv_line(), query_single_task_detailed(), query_task_via_com() (+7 more)

### Community 138 - "hardware.rs"
Cohesion: 0.25
Nodes (4): cpu_title(), HardwareView, kv(), power_plan_name()

### Community 139 - "path"
Cohesion: 0.13
Nodes (10): FaultAction, ClearCbsRebootFlags, ClearMinidumps, ClearPendingRenames, RestartSystem, FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless() (+2 more)

### Community 141 - "SlowScanBackend"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 143 - "ColorScheme"
Cohesion: 0.13
Nodes (8): MAX_ZOOM, MIN_ZOOM, out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings, apply_settings(), SlintBridge

### Community 144 - "hashmap"
Cohesion: 0.28
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

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

### Community 149 - "check_view"
Cohesion: 0.45
Nodes (5): check_view(), key(), Rect, region(), visible_texts()

### Community 150 - "Spring"
Cohesion: 0.17
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 152 - ".sync_startup_tasks_busy"
Cohesion: 0.17
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 153 - "DriveMediaKind"
Cohesion: 0.11
Nodes (14): detect_platform_security_info(), DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard (+6 more)

### Community 155 - "CleanTargetResult"
Cohesion: 0.28
Nodes (7): ComStaGuard, empty_recycle_bin(), execute_clean_targets(), init_com_sta(), query_recycle_bin(), resolve_target_paths(), scan_all_clean_targets()

### Community 158 - ".sample"
Cohesion: 0.33
Nodes (7): compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), SystemProcessorPerformanceInformation

### Community 159 - "views/network_diag.rs"
Cohesion: 0.47
Nodes (4): WorkerDropGuard<P>, detect_screen_workarea(), main(), run_slint()

### Community 160 - "SignaturePolicy"
Cohesion: 0.40
Nodes (4): SignaturePolicy, AllowUnsigned, Required, VerifyIfPresent

### Community 161 - "dashboard.rs"
Cohesion: 0.29
Nodes (6): category_buckets(), DashboardView, health_caption(), health_counts(), health_score(), health_tone()

### Community 165 - "DiskSubsystemReport"
Cohesion: 0.15
Nodes (9): DiskSubsystemReport, PhysicalDriveHealth, print_live_disk_report(), probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic(), HostWrites, NotAvailable, Reported (+1 more)

### Community 168 - "backend.rs"
Cohesion: 0.12
Nodes (4): CachedScanBackend, CachedScanBackend<'a>, SandboxMetricsSampler, CpuInfo

### Community 171 - "BitSet256"
Cohesion: 0.08
Nodes (8): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, StackFmt, WORDS

### Community 183 - "super"
Cohesion: 0.08
Nodes (15): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, test_windows_info_detect_host(), acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a> (+7 more)

### Community 184 - ".sample"
Cohesion: 0.31
Nodes (6): active_logical_core_count(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 185 - "views/gpu.rs"
Cohesion: 0.31
Nodes (3): gpu_tweaks(), GpuView, PREFERRED_GPU_ORDER

### Community 186 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 259 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 262 - "path"
Cohesion: 0.50
Nodes (3): BLANK, scan(), ui_strings_only_use_glyphs_that_render()

### Community 263 - "NetAction"
Cohesion: 0.40
Nodes (4): NetAction, ApplyDns, FlushDns, ResetDnsDhcp

## Knowledge Gaps
- **1484 isolated node(s):** `Easy`, `Advanced`, `HardwareView`, `PrivacyShield`, `BackupCenter` (+1479 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2228 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **135 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `AppState`, `FailAtMutationBackend`, `Scandetailkind Appxinstalled Appxinstalledps`, `astar.rs`, `in_progress_hud.rs`, `super`, `matrix.rs`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `startup.rs`, `registry_scanner.rs`, `spawn_guarded_worker`, `game_prereqs.rs`, `parking.rs`, `FailAtMutationBackend`, `WindowsInfo`, `CoreParkingState`, `ui_views.rs`, `probe_physical_drive`, `backend.rs`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `Language`, `.setup`, `app.rs`, `tools/automation.rs`, `shell.rs`, `componenthandle`, `disks.rs`, `AutoTuneProfile`, `restore_point.rs`, `appx_scanner.rs`, `DriverUpdateItem`, `.navigate`, `InstalledAppxPackage`, `ui_parity.rs`, `CleanTargetKind`, `HibernationStatus`, `PdhParkingCollector`, `sync_vec`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `Ctx`, `restart_explorer_safely`, `SystemMetrics`, `Changelog`, `StartupEntry`, `ui_layout_collision.rs`, `dijkstra.rs`, `hardware.rs`, `path`, `ColorScheme`, `hashmap`, `diff.rs`, `views/latency.rs`, `UpdateStatus`, `LiveServiceInfo`, `.sync_startup_tasks_busy`, `DriveMediaKind`, `CleanTargetScan`, `LiveScheduledTaskInfo`, `BiosSystemInfo`, `dashboard.rs`, `drivers.rs`, `DiskSubsystemReport`, `backend.rs`, `planner.rs`, `LiveGameTelemetry`, `GameProfile`, `super`, `views/gpu.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `PowerSchemeInfo`, `WindowsOptionalFeature`?**
  _High betweenness centrality (0.268) - this node is a cross-community bridge._
- **Are the 24 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationGoal`) actually correct?**
  _`AppState` has 24 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Easy`, `Advanced`, `HardwareView` to the rest of the system?**
  _1484 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00315955766192733 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `planner.rs`, `ui_views.rs`, `views/gpu.rs`, `ColorScheme`, `app.rs`, `shell.rs`, `AutoTuneProfile`, `ui_slint/i18n.rs`, `ui_parity.rs`, `sync_vec`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`?**
  _High betweenness centrality (0.209) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.00966183574879227 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `dashboard.rs`, `ui_slint/i18n.rs`, `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `componenthandle`, `ui_views.rs`, `gaming.rs`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._