# Graph Report - WinBlast0  (2026-11-25)

## Corpus Check
- 164 files · ~344,302 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 110 file(s) not represented in the graph (top: .slint 73, .toml 13, .ttf 7)
- WARNING: 110 of 317 files seen (35%) have no supported extractor and are missing from this graph entirely. It may look complete while representing only part of the codebase.

## Summary
- 5631 nodes · 11264 edges · 293 communities (157 shown, 136 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 485 edges (avg confidence: 0.74)
- Token cost: 13,424 input · 14,275 output

## Graph Freshness
- Built from commit: `74166a38`
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
- registry_scanner.rs
- Win32 Registry Scanner & Mutator
- RealBackend
- cleaner.rs
- NetAction
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
- WGPUBackend
- registry_scanner.rs
- Language
- updater.rs
- .take_injected_error
- ui_render_preview.rs
- GLItemRenderer<'a, R>
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
- i-slint-renderer-femtovg/lib.rs
- net_latency.rs
- tr_view
- tools/startup.rs
- process.rs
- PlatformSecurityInfo
- OpenGLBackend
- .setup
- app.rs
- backend.rs
- core/theme.rs
- HibernationStatus
- shell.rs
- componenthandle
- diff_sync.rs
- Runtime
- disks.rs
- wgpu.rs
- test_wave1_compound_actions_deep_edge_cases
- AutoTuneProfile
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
- Texture
- window_icon.rs
- HibernationStatus
- PdhParkingCollector
- UpdateStatus
- Changelog
- font_coverage.rs
- GameOverlaySettings
- ui_scrollbar_test.rs
- storage.rs
- raster.rs
- tools/game_overlay.rs
- views/startup.rs
- lazy_loader.rs
- TaskCache
- gaming.rs
- sync_vec
- reg_helpers.rs
- images.rs
- MonotonicCubicProgress
- adaptive_tuning.rs
- BitSet256
- elevation.rs
- SystemMetrics
- BiosSystemInfo
- itemrenderer.rs
- StartupEntry
- ui_layout_collision.rs
- RenderMode
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
- image_source.rs
- Spring
- LiveServiceInfo
- winblast_glyphs.rs
- DriveMediaKind
- windows_repair.rs
- BackendTweakStatus
- src/engine.rs
- WinBlast Optimizer — Developer & Architecture Guide
- .sample
- views/network_diag.rs
- .brush_to_paint
- dashboard.rs
- drivers.rs
- DiskSubsystemReport
- network_diag.rs
- WindowsInfo
- Ctx
- BitSet256
- serialize
- text_render.rs
- .sample
- selection_automation_e2e.rs
- test_status_i18n_all_languages_and_core_specs
- draw
- .navigate
- instant
- TextureImporter
- ShelfPacker
- duration
- hashmap
- UpdateState
- LazyLoader
- clean_exe_path
- DriverUpdateState
- ActiveDropdown
- restart_explorer_safely
- .new
- contrast.rs
- TextEngine
- BitSetIter
- NodeCost
- HardwareSubTab
- ui_slint/theme.rs
- .all_physical

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 658 edges
2. `AppState` - 346 edges
3. `ViewTextKey` - 249 edges
4. `SandboxEnv` - 159 edges
5. `SystemBackend` - 158 edges
6. `WorkerEvent` - 114 edges
7. `StatusMessageSpec` - 108 edges
8. `RealBackend` - 95 edges
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
- 2-file cycle: `vendor/i-slint-renderer-femtovg/itemrenderer.rs -> vendor/i-slint-renderer-femtovg/lib.rs -> vendor/i-slint-renderer-femtovg/itemrenderer.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/ui_slint/palette.rs -> src/ui_slint/shell.rs -> src/ui_slint/palette.rs`
- 2-file cycle: `vendor/i-slint-renderer-femtovg/images.rs -> vendor/i-slint-renderer-femtovg/itemrenderer.rs -> vendor/i-slint-renderer-femtovg/images.rs`
- 2-file cycle: `vendor/i-slint-renderer-femtovg/lib.rs -> vendor/i-slint-renderer-femtovg/wgpu.rs -> vendor/i-slint-renderer-femtovg/lib.rs`
- 3-file cycle: `vendor/i-slint-renderer-femtovg/images.rs -> vendor/i-slint-renderer-femtovg/itemrenderer.rs -> vendor/i-slint-renderer-femtovg/lib.rs -> vendor/i-slint-renderer-femtovg/images.rs`

## Communities (293 total, 136 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.00
Nodes (633): TextKey, AboutBuiltWith, AboutNew1, AboutNew2, AboutNew3, AboutNew4, AboutNew5, AboutNew6 (+625 more)

### Community 1 - "action_button_guarded"
Cohesion: 0.01
Nodes (241): ViewTextKey, AboutActiveBadge, AboutBullet1, AboutBullet2, AboutBullet3, AboutBullet4, AboutBullet5, AboutBullet6 (+233 more)

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
Nodes (8): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, AutoTuneCompleted, DiskReportLoaded, MemoryTrimmed

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.05
Nodes (3): PanickingBackend, SlowScanBackend, test_epoch_and_stale_scan_fencing_preserves_instant_mutation()

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.06
Nodes (62): StateLedger, .SCHEMA_VERSION, is_unsupported_power_guid_error(), Result, decode_hex(), decode_hex_rejects_non_ascii_without_panicking(), encode_hex(), escape_pipe() (+54 more)

### Community 11 - "astar.rs"
Cohesion: 0.15
Nodes (8): AStarNode, AStarPlanner, BitSet256, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 12 - "theme.rs"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 13 - "app.rs"
Cohesion: 0.09
Nodes (34): localize_ledger_action(), CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci() (+26 more)

### Community 14 - "astar.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 15 - "power.rs"
Cohesion: 0.06
Nodes (25): capture_parking_snapshot(), create_and_activate_performance_plan(), fallback_scheme_name(), format_guid(), get_active_power_scheme(), is_on_ac_power(), list_power_schemes(), LocalGuidPtr (+17 more)

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.07
Nodes (34): load_embedded_rules(), a1_write_ahead_rows_are_committed(), a2_foreign_prior_types_round_trip_byte_exact(), a2_rollback_removes_created_keys(), a4_reset_to_default_leaves_absent_values_absent(), a4_undo_restores_user_value_not_default(), a5_reapply_records_nothing(), a7_ledger_migration_and_foreign_schema() (+26 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.05
Nodes (43): UiCommand, ApplyStagedTweaks, AssessSystemHealth, BatchToggleSelectedStartup, BatchToggleSelectedTasks, CreateSystemRestorePoint, DeselectAllCleanTargets, DeselectFilteredTweaks (+35 more)

### Community 18 - "SandboxEnv"
Cohesion: 0.07
Nodes (3): CachedScanBackend<'a>, SandboxEnv, WindowsOptionalFeature

### Community 19 - "topological_scheduler.rs"
Cohesion: 0.22
Nodes (14): assert_valid_permutation(), position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence(), test_max_capacity_256_nodes() (+6 more)

### Community 20 - "super"
Cohesion: 0.07
Nodes (10): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted, PerfBenchmarkCompleted (+2 more)

### Community 21 - "matrix.rs"
Cohesion: 0.08
Nodes (21): MatrixQuery, MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status (+13 more)

### Community 22 - "load_embedded_rules"
Cohesion: 0.22
Nodes (18): diff(), guard(), hive(), key_values(), Maker, probe(), rollback_all(), temp_ledger() (+10 more)

### Community 23 - "updater.rs"
Cohesion: 0.14
Nodes (21): check_latest_release(), CURL_HTTPS_ONLY, download_release_asset(), DOWNLOAD_URL_PREFIX, fetch_url_json(), GhAsset, GhRelease, GITHUB_REPO (+13 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (30): ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter, BackupCenter, CleanAndApps, CoreParking, DiskHealth (+22 more)

### Community 25 - "clean_apps.rs"
Cohesion: 0.17
Nodes (9): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, CleanAppsView, folded(), format_mb(), target_id() (+1 more)

### Community 27 - "UiThemeStyle"
Cohesion: 0.10
Nodes (11): style_previews_follow_mode_and_scheme(), SurfaceSpec, ThemePalette, UiThemeStyle, .ALL, Classic, Compact, Glassy (+3 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.06
Nodes (31): PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField, ExecutionFailed (+23 more)

### Community 30 - "registry_scanner.rs"
Cohesion: 0.07
Nodes (59): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), normalize_reg_value_name(), delete_key_if_empty(), delete_tree(), delete_value(), delete_value_for_vendor(), delete_value_literal() (+51 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.07
Nodes (27): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+19 more)

### Community 33 - "cleaner.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 34 - "NetAction"
Cohesion: 0.33
Nodes (4): NetAction, ApplyDns, FlushDns, ResetDnsDhcp

### Community 35 - "startup.rs"
Cohesion: 0.04
Nodes (50): panic_payload_to_string(), WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted, CoreParkingUpdated (+42 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.13
Nodes (33): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+25 more)

### Community 37 - "spawn_guarded_worker"
Cohesion: 0.09
Nodes (7): spawn_guarded_worker(), AppxBatchRemoved, CleanExecuted, OneDriveEdgeResult, ScheduledTaskRunResult, WinGetInstallResult, record_then_change()

### Community 39 - "game_prereqs.rs"
Cohesion: 0.11
Nodes (28): catalog_undetected(), detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86() (+20 more)

### Community 40 - "parking.rs"
Cohesion: 0.10
Nodes (21): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+13 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.08
Nodes (22): display_reg_value_name(), EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY (+14 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.08
Nodes (15): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), parse_pdh_processor_instance(), PdhParkingCollector, PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info() (+7 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.13
Nodes (5): is_reversible_action(), LedgerEntry, LedgerEntryView<'a>, SystemBaseline, test_ledger_crud()

### Community 45 - "features_apps.rs"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.08
Nodes (29): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+21 more)

### Community 47 - "WGPUBackend"
Cohesion: 0.05
Nodes (20): BeginRendering, Acquired, Skipped, GraphicsBackend, .NAME, WindowSurface, composite_alpha_mode(), femtovg::renderer::WGPURenderOutput (+12 more)

### Community 49 - "Language"
Cohesion: 0.14
Nodes (24): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+16 more)

### Community 50 - "updater.rs"
Cohesion: 0.20
Nodes (12): build_signing_transcript(), compute_sha256(), OverrideKey, PresetMetadata, PresetPackage, sample_metadata(), Required, test_canonical_encode_decode_roundtrip_unsigned() (+4 more)

### Community 51 - ".take_injected_error"
Cohesion: 0.11
Nodes (4): optional_feature_matches(), SandboxState, test_hkcr_and_shallow_tree_delete_blocked(), validate_registry_mutation_path()

### Community 52 - "ui_render_preview.rs"
Cohesion: 0.07
Nodes (8): test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_micro_and_ultrawide_viewports(), H, Headless, render_all_views(), render_preview_frames(), settle_and_save(), W

### Community 53 - "GLItemRenderer<'a, R>"
Cohesion: 0.08
Nodes (4): GLItemRenderer<'a, R>, is_drawn_by_wrapping_opacity(), ItemGraphicsCacheEntry<R>, passes_to_wrapped_backdrop_blur()

### Community 55 - "ui_views.rs"
Cohesion: 0.13
Nodes (46): about_shows_real_version_and_style_studio_switches_style(), automation_profiles_autotune_trim_health_and_maintenance(), backup_ledger_undo_reverts_an_applied_tweak(), backup_restore_point_profile_and_unattend_roundtrip_in_sandbox(), clean_apps_appx_filter_protected_core_and_selection(), clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox(), clean_apps_sub_tabs_drive_state(), clean_apps_winget_catalog_filter_and_selection() (+38 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.21
Nodes (6): background_probe_delivers_payload_without_blocking(), LazyPayload, Bios, Disks, Drivers, Thermals

### Community 57 - "probe_physical_drive"
Cohesion: 0.13
Nodes (15): CoreEvent, AutoTuneCompleted, BiosInfoLoaded, CleanerCleanCompleted, CleanerScanCompleted, DiskReportLoaded, DriverScanCompleted, HealthScoreUpdated (+7 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.16
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.08
Nodes (22): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+14 more)

### Community 60 - "backend.rs"
Cohesion: 0.08
Nodes (23): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+15 more)

### Community 61 - "driver_update.rs"
Cohesion: 0.15
Nodes (19): backup_root(), cim_and_store_json_parse(), compare_versions(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), json_items(), json_str(), MAX_RETRIES (+11 more)

### Community 62 - "AppxCache"
Cohesion: 0.09
Nodes (5): FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit()

### Community 63 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 64 - "FilterStrictSelection"
Cohesion: 0.14
Nodes (7): ConfirmationRequirement, Confirmed, NotRequired, Required, FilterableEntry, FilterStrictSelection, SelectionMetrics

### Community 65 - "FeatureInstallState"
Cohesion: 0.12
Nodes (22): create_restore_point(), FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind (+14 more)

### Community 66 - ".maybe_arm_error"
Cohesion: 0.09
Nodes (12): OsRequirement, Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme (+4 more)

### Community 67 - "ColorScheme"
Cohesion: 0.09
Nodes (14): MAX_ZOOM, MIN_ZOOM, accent_stays_distinct_from_danger_and_success(), ColorScheme, .ALL, CrimsonSunset, CyberIndigo, EmeraldMint (+6 more)

### Community 68 - "palette.rs"
Cohesion: 0.13
Nodes (19): apply_language(), install(), is_known_key(), lookup(), ACTIONS, clean_label(), empty_query_lists_actions_first_without_tweak_row(), every_view_is_listed_once_and_resolves() (+11 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.23
Nodes (13): escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml(), map_hive(), sanitize_computer_name(), sanitize_local_user() (+5 more)

### Community 70 - "TaskCache"
Cohesion: 0.13
Nodes (14): DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display, Input (+6 more)

### Community 71 - "i-slint-renderer-femtovg/lib.rs"
Cohesion: 0.07
Nodes (5): FemtoVGOpenGLRenderer, FemtoVGRenderer<B>, FemtoVGRendererExt, PhysicalSize, select_render_target()

### Community 72 - "net_latency.rs"
Cohesion: 0.20
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 73 - "tr_view"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 74 - "tools/startup.rs"
Cohesion: 0.16
Nodes (18): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), expand_system_root_vars_handles_multibyte_paths(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service() (+10 more)

### Community 75 - "process.rs"
Cohesion: 0.06
Nodes (31): default_timeout(), hung_command_is_stopped_after_its_timeout(), kill_process_tree(), known_folder(), known_program_data_dir(), open_url_unelevated(), output_is_captured(), paths_and_unknown_names_are_unchanged() (+23 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.22
Nodes (8): BinaryReader<'a>, ParamValue, Bool, Bytes, I64, Str, U64, test_corrupted_magic_rejection()

### Community 77 - "OpenGLBackend"
Cohesion: 0.06
Nodes (12): FemtoVGOpenGLRendererExt, FemtoVGRenderer<opengl::OpenGLBackend>, FemtoVGRenderer<OpenGLBackend>, GLWindowSurface, Screen, Snapshot, OpenGLBackend, .NAME (+4 more)

### Community 78 - ".setup"
Cohesion: 0.14
Nodes (13): cached_disk_report(), ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth (+5 more)

### Community 79 - "app.rs"
Cohesion: 0.04
Nodes (57): AppMode, Advanced, Easy, BeforeAfterImpact, CleanSortMode, NameAsc, SizeAsc, SizeDesc (+49 more)

### Community 80 - "backend.rs"
Cohesion: 0.07
Nodes (17): SandboxMetricsSampler, test_sandbox_core_parking_labels_metrics_and_power_sync(), apply_parking_snapshot(), compute_is_fully_unparked(), CoreParkingState, CoreStatus, CPMAXCORES, CPMAXCORES1 (+9 more)

### Community 81 - "core/theme.rs"
Cohesion: 0.10
Nodes (38): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BADGE_ALPHA, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR (+30 more)

### Community 83 - "shell.rs"
Cohesion: 0.13
Nodes (9): _assert_model_types(), elapsed_formatting(), format_elapsed(), LANGUAGE_EN_NAMES, MAX_TOASTS, Shell, TextSettings, toast_tone() (+1 more)

### Community 84 - "componenthandle"
Cohesion: 0.11
Nodes (6): AboutView, REPO_NAME, REPO_URL, DiagnosticsView, ISSUES_URL, open_external()

### Community 86 - "diff_sync.rs"
Cohesion: 0.15
Nodes (7): MAX_TELEMETRY_CHANNELS, PreInternedStrings, TelemetryChannelSpec, TelemetryDiffSync, TelemetryShadow, TweakDiffSync, TweakStaticDescriptor

### Community 87 - "Runtime"
Cohesion: 0.26
Nodes (13): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+5 more)

### Community 88 - "disks.rs"
Cohesion: 0.04
Nodes (44): ata_fixture_goodram_cx400_g2_unknown_unit_is_not_available(), ata_receive_drive_data(), DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, hex_fixture(), NOT_REPORTED (+36 more)

### Community 89 - "wgpu.rs"
Cohesion: 0.08
Nodes (3): FemtoVGWGPURenderer, FemtoVGWGPURendererExt, wgpu_take_snapshot_pixels()

### Community 91 - "AutoTuneProfile"
Cohesion: 0.12
Nodes (10): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, create_auto_tune_plan(), test_iso_29119_auto_tune_plan_generation_all_profiles() (+2 more)

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
Cohesion: 0.20
Nodes (8): create_sandbox_test_item(), download_and_install_driver(), DriverUpdateItem, scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata(), test_driver_sandbox_lifecycle(), test_driver_sandbox_lifecycle_execution()

### Community 98 - "TaskCache"
Cohesion: 0.14
Nodes (3): TaskCache, list_live_tasks_from_cache(), LiveScheduledTaskInfo

### Community 99 - "InstalledAppxPackage"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 102 - "ui_parity.rs"
Cohesion: 0.22
Nodes (14): changelog_toggle_and_rollback_id_validation(), diff_modal_shows_inspected_tweak_and_closes(), easy_advanced_and_engine_mode_switches(), every_slint_translation_key_exists_in_the_catalogs(), every_theme_combination_reaches_slint(), harness(), hud_follows_background_work_and_can_be_dismissed(), language_switch_relocalizes_live_without_restart() (+6 more)

### Community 104 - "Texture"
Cohesion: 0.10
Nodes (15): Texture, TextureCache, blur_and_colorize(), CanvasRc, FemtovgBoxShadowCache, GLItemRenderer, ItemGraphicsCache, ItemGraphicsCacheEntry (+7 more)

### Community 106 - "HibernationStatus"
Cohesion: 0.24
Nodes (4): RollbackRefreshData, HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 108 - "UpdateStatus"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 109 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 110 - "font_coverage.rs"
Cohesion: 0.27
Nodes (10): files(), font(), inter_covers_all_non_cjk_ui_letters(), missing(), mono_covers_ascii_and_turkish(), root(), TURKISH, ui_letters() (+2 more)

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 113 - "storage.rs"
Cohesion: 0.16
Nodes (18): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd() (+10 more)

### Community 114 - "raster.rs"
Cohesion: 0.13
Nodes (15): open_contours_are_closed_and_bounds_cover_all_points(), SegmentPen, coverage_equals_exact_area_for_fractional_square(), flattened_circle_area_matches_pi_r_squared(), Flattener, hole_with_opposite_winding_is_empty_and_overlap_is_clamped(), pixel_aligned_square_is_fully_covered_inside_and_empty_outside(), Point (+7 more)

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
Cohesion: 0.11
Nodes (13): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor, GameSuitabilityReport, SuitabilityCheck (+5 more)

### Community 121 - "sync_vec"
Cohesion: 0.14
Nodes (6): new_model(), sync_vec(), sync_vec_grows_shrinks_and_updates(), DASH, SensorsView, temp_tone()

### Community 122 - "reg_helpers.rs"
Cohesion: 0.22
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 123 - "images.rs"
Cohesion: 0.11
Nodes (5): base_image_flags(), image_buffer_to_image_source(), Texture<R>, TextureCache<R>, TextureCacheKey

### Community 124 - "MonotonicCubicProgress"
Cohesion: 0.25
Nodes (4): BatchDescriptor, HudFrame, MAX_BATCHES, MonotonicCubicProgress

### Community 126 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 129 - "SystemMetrics"
Cohesion: 0.16
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 131 - "itemrenderer.rs"
Cohesion: 0.08
Nodes (6): append_rect_with_radius_to_path(), rect_to_path(), rect_with_radius_to_path(), PhysicalBorderRadius, PhysicalPoint, PhysicalRect

### Community 132 - "StartupEntry"
Cohesion: 0.14
Nodes (15): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), set_startup_entry_enabled(), set_startup_entry_enabled_by_id(), StartupEntry (+7 more)

### Community 133 - "ui_layout_collision.rs"
Cohesion: 0.20
Nodes (14): check_view(), Harness, harness(), HEIGHT, is_text(), key(), no_text_collisions_or_overflow_at_any_width(), overlap_y() (+6 more)

### Community 134 - "RenderMode"
Cohesion: 0.12
Nodes (16): config(), CONTRAST, DEFAULT_CONTRAST, GENERATION, generation(), MODE, RenderMode, .ALL (+8 more)

### Community 135 - "BackendTweakValue"
Cohesion: 0.17
Nodes (10): StackFmt, BackendTweakValue, Bool, Int, Megabytes, MillisTenths, None, PercentTenths (+2 more)

### Community 136 - "dijkstra.rs"
Cohesion: 0.27
Nodes (8): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks(), MAX_NODES

### Community 137 - "task_scanner.rs"
Cohesion: 0.18
Nodes (16): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), normalize_task_path(), parse_csv_line(), parse_task_state_lines(), query_single_task_detailed() (+8 more)

### Community 138 - "hardware.rs"
Cohesion: 0.25
Nodes (3): HardwareView, kv(), power_plan_name()

### Community 139 - "super"
Cohesion: 0.07
Nodes (19): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, LedgerEntryView, FaultAction, ClearCbsRebootFlags, ClearMinidumps (+11 more)

### Community 143 - "ColorScheme"
Cohesion: 0.15
Nodes (7): out_of_range_values_are_clamped(), partial_json_keeps_other_defaults(), roundtrip_via_env_override(), UserSettings, apply_settings(), SlintBridge, write_bmp()

### Community 144 - "hashmap"
Cohesion: 0.32
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

### Community 145 - "diff.rs"
Cohesion: 0.28
Nodes (13): diff_pair_card(), diff_single_card(), diff_view(), DiffRow, directed_pair(), header_with_dir(), rows_for(), Tone (+5 more)

### Community 146 - "views/latency.rs"
Cohesion: 0.29
Nodes (5): better(), comparison(), jitter_text(), LatencyView, ping_text()

### Community 147 - "tools/automation.rs"
Cohesion: 0.07
Nodes (26): SilentCommand, assess_system_health(), HealthSeverity, Critical, Info, Warning, install_maintenance_task(), is_maintenance_task_installed() (+18 more)

### Community 148 - "test_disks.ps1"
Cohesion: 0.29
Nodes (5): GpuVendor, Amd, Intel, Nvidia, PresetGuards

### Community 149 - "image_source.rs"
Cohesion: 0.12
Nodes (6): image_tracks_source(), init_offscreen_platform(), OffscreenPlatform, OffscreenWindow, poll_once(), solid_image()

### Community 150 - "Spring"
Cohesion: 0.16
Nodes (10): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC, Spring, .BOUNCY, .NONE (+2 more)

### Community 152 - "winblast_glyphs.rs"
Cohesion: 0.11
Nodes (5): GlyphKey, Atlas, ATLAS_SIZE, Slot, try_draw()

### Community 153 - "DriveMediaKind"
Cohesion: 0.29
Nodes (6): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown

### Community 154 - "windows_repair.rs"
Cohesion: 0.38
Nodes (7): decode_tool_output(), last_lines(), progress_lines_are_dropped(), RepairTask, run_tool(), summary(), utf16_output_is_decoded()

### Community 155 - "BackendTweakStatus"
Cohesion: 0.22
Nodes (9): BackendTweakStatus, Applied, Failed, Pending, Queued, Running, Skipped, TweakShadow (+1 more)

### Community 156 - "src/engine.rs"
Cohesion: 0.10
Nodes (10): coords_key(), font_key(), FONTS, GlyphPlacement, INTER, keys_are_stable_and_distinguish_inputs(), MAX_PX, MIN_PX (+2 more)

### Community 157 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.18
Nodes (11): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+3 more)

### Community 158 - ".sample"
Cohesion: 0.23
Nodes (10): active_logical_core_count(), compute_nt_cpu_deltas(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases() (+2 more)

### Community 159 - "views/network_diag.rs"
Cohesion: 0.33
Nodes (6): WorkerDropGuard<P>, acquire_single_instance(), detect_screen_workarea(), main(), run_slint(), show_already_running()

### Community 160 - ".brush_to_paint"
Cohesion: 0.14
Nodes (6): GlyphBrush, Fill, Stroke, to_femtovg_color(), to_femtovg_stops(), PhysicalLength

### Community 161 - "dashboard.rs"
Cohesion: 0.25
Nodes (7): category_buckets(), cpu_title(), DashboardView, health_caption(), health_counts(), health_score(), health_tone()

### Community 163 - "drivers.rs"
Cohesion: 0.27
Nodes (3): category_from_id(), DriversView, FILTER_CATEGORIES

### Community 165 - "DiskSubsystemReport"
Cohesion: 0.15
Nodes (9): DiskSubsystemReport, PhysicalDriveHealth, print_live_disk_report(), probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic(), HostWrites, NotAvailable, Reported (+1 more)

### Community 166 - "network_diag.rs"
Cohesion: 0.16
Nodes (7): DEFAULT_ROUTE_ADAPTER, DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip(), read_tcp_overrides(), run_dns_script()

### Community 170 - "Ctx"
Cohesion: 0.14
Nodes (4): Ctx, Views, goal_id(), PlannerView

### Community 171 - "BitSet256"
Cohesion: 0.08
Nodes (7): BitSet256, .EMPTY, BitSetIter, CellBitSet256, contains_ascii_icase(), MAX_TWEAKS, WORDS

### Community 183 - "text_render.rs"
Cohesion: 0.18
Nodes (15): apply(), CJK_SC_FAMILY, DEFAULT_TEXT_CONTRAST, DEFAULT_TEXT_RENDERING, DEFAULT_UI_FONT, effective_family(), env_override(), font_index() (+7 more)

### Community 184 - ".sample"
Cohesion: 0.47
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 186 - "test_status_i18n_all_languages_and_core_specs"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 188 - "draw"
Cohesion: 0.28
Nodes (14): chinese_font_and_grafted_symbols_rasterize(), contrast_darkens_thin_strokes(), crisp_mode_puts_vertical_stems_on_full_pixels(), darkest_row(), draw(), every_bundled_font_draws_turkish_text_in_every_mode(), flat_bottomed_letters_sit_on_the_baseline_row(), gid() (+6 more)

### Community 255 - ".navigate"
Cohesion: 0.12
Nodes (5): BiosInfoLoaded, DriverScanCompleted, GameTelemetryProbed, PrereqsScanned, ThermalSnapshot

### Community 256 - "instant"
Cohesion: 0.24
Nodes (6): BenchmarkRunner, BenchmarkScore, disk_read_is_measured(), measure_disk_read_mb_s(), measure_ram_latency_ns(), ram_latency_is_measured_not_fixed()

### Community 257 - "TextureImporter"
Cohesion: 0.31
Nodes (3): femtovg::renderer::OpenGl, femtovg::renderer::WGPURenderer, TextureImporter

### Community 260 - "ShelfPacker"
Cohesion: 0.31
Nodes (5): allocations_never_overlap_and_stay_inside(), PADDING, Shelf, ShelfPacker, too_big_fails_and_reset_frees_space()

### Community 261 - "duration"
Cohesion: 0.22
Nodes (8): TelemetryTickData, ToastNotification, ToastSeverity, Error, Info, Success, Warning, AutoTuneResult

### Community 263 - "UpdateState"
Cohesion: 0.25
Nodes (8): UpdateState, Checking, Downloading, Failed, Idle, ReadyToRestart, UpdateAvailable, UpToDate

### Community 264 - "LazyLoader"
Cohesion: 0.33
Nodes (3): background_panic_is_contained_and_clears_in_flight_flag(), LazyLoader, try_recv_is_non_blocking_when_empty()

### Community 265 - "clean_exe_path"
Cohesion: 0.29
Nodes (5): Failed, relaunch_as_admin(), apply_and_restart(), clean_exe_path(), test_clean_windows_exe_path()

### Community 266 - "DriverUpdateState"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 267 - "ActiveDropdown"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 268 - "restart_explorer_safely"
Cohesion: 0.29
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 270 - "contrast.rs"
Cohesion: 0.60
Nodes (4): endpoints_are_fixed_and_curve_is_monotonic(), lut(), MAX_GAMMA, zero_contrast_is_identity_and_more_contrast_is_darker()

### Community 274 - "HardwareSubTab"
Cohesion: 0.40
Nodes (5): HardwareSubTab, CpuParking, Disks, Specs, Thermals

## Knowledge Gaps
- **1525 isolated node(s):** `Sharp`, `Crisp`, `Smooth`, `Classic`, `Easy` (+1520 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 2437 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **136 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `Disk Cleaning & Temporary File Purger` to `spawn_guarded_worker`, `FailAtMutationBackend`, `Scandetailkind Appxinstalled Appxinstalledps`, `astar.rs`, `power.rs`, `in_progress_hud.rs`, `SandboxEnv`, `super`, `matrix.rs`, `System Architecture Changelog`, `clean_apps.rs`, `UiThemeStyle`, `cleaner.rs`, `startup.rs`, `registry_scanner.rs`, `spawn_guarded_worker`, `game_prereqs.rs`, `parking.rs`, `FailAtMutationBackend`, `Language`, `ui_views.rs`, `backend.rs`, `ColorScheme`, `dijkstra.rs`, `TaskCache`, `net_latency.rs`, `tr_view`, `.setup`, `app.rs`, `backend.rs`, `shell.rs`, `componenthandle`, `disks.rs`, `AutoTuneProfile`, `FeatureInstallState`, `RestorePoint`, `DriverUpdateItem`, `TaskCache`, `InstalledAppxPackage`, `backup.rs`, `ui_parity.rs`, `HibernationStatus`, `PdhParkingCollector`, `UpdateStatus`, `Changelog`, `views/startup.rs`, `TaskCache`, `gaming.rs`, `sync_vec`, `SystemMetrics`, `BiosSystemInfo`, `StartupEntry`, `ui_layout_collision.rs`, `dijkstra.rs`, `hardware.rs`, `super`, `ColorScheme`, `hashmap`, `diff.rs`, `views/latency.rs`, `tools/automation.rs`, `LiveServiceInfo`, `windows_repair.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `dashboard.rs`, `drivers.rs`, `DiskSubsystemReport`, `network_diag.rs`, `WindowsInfo`, `Ctx`, `serialize`, `.navigate`, `instant`, `duration`, `clean_exe_path`, `ActiveDropdown`, `HardwareSubTab`?**
  _High betweenness centrality (0.213) - this node is a cross-community bridge._
- **Are the 23 inferred relationships involving `AppState` (e.g. with `UiThemeStyle` and `OptimizationPlan`) actually correct?**
  _`AppState` has 23 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Sharp`, `Crisp`, `Smooth` to the rest of the system?**
  _1525 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00315955766192733 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Disk Cleaning & Temporary File Purger`, `hardware.rs`, `theme.rs`, `diff.rs`, `views/latency.rs`, `matrix.rs`, `clean_apps.rs`, `dashboard.rs`, `drivers.rs`, `Ctx`, `ColorScheme`, `palette.rs`, `app.rs`, `shell.rs`, `AutoTuneProfile`, `ui_parity.rs`, `ui_scrollbar_test.rs`, `views/startup.rs`, `gaming.rs`, `sync_vec`?**
  _High betweenness centrality (0.173) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.008298755186721992 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `dashboard.rs`, `palette.rs`, `tr_view`, `hardware.rs`, `componenthandle`, `gaming.rs`, `sync_vec`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._