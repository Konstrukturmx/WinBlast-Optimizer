# Graph Report - WinBlast0  (2026-11-21)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 4036 nodes · 8392 edges · 183 communities (128 shown, 55 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 333 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `38dc893c`
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
- automation.rs
- Scandetailkind Appxinstalled Appxinstalledps
- startup.rs
- theme.rs
- app.rs
- astar.rs
- in_progress_hud.rs
- full_telemetry_audit_with
- WinBlastError
- WinBlast Optimizer
- super
- clean_apps.rs
- sys_inventory.rs
- BitSet256
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
- Dynamicdwordformula Logicalcorecount Totalraminkb
- svc_mutator.rs
- Changelog
- registry_scanner.rs
- deserialize
- updater.rs
- PlatformSecurityInfo
- PanickingBackend
- PanickingBackend
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
- CoreParkingState
- task_scanner.rs
- Reversibility Instant Requireslogoff
- AutoTuneProfile
- dijkstra.rs
- TaskCache
- Project Documentation, Architecture & Licensing
- WorkerEvent
- app.rs
- .set_status_spec
- PlatformSecurityInfo
- DriverUpdateItem
- .setup
- hashset
- deserialize
- progress_bar.rs
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
- probe_physical_drive
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
- GameProfile
- reg_helpers.rs
- progress_bar.rs
- storage.rs
- adaptive_tuning.rs
- WindowsInfo
- diff_modal.rs
- .sync_startup_tasks_busy
- .all_physical
- WinBlast Optimizer — Developer & Architecture Guide
- Item
- c_void
- DateTime
- Utc
- enum_scm_services_by_type
- HealthSeverity
- startup.rs
- tools/gaming_stats.rs
- ActiveDropdown
- SlowScanBackend
- path
- MatrixSortMode
- HardwareSubTab
- UpdateStatus
- SilentCommand
- ActiveDropdown
- StartupSortMode
- RegStringValue
- DiskSubsystemReport
- OverallHealthGrade
- views/network_diag.rs
- CleanSortMode
- PrivacySortMode
- PresetKey
- BitSetIter
- .execute_auto_tune_profile
- NodeCost
- .all_physical

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 537 edges
2. `AppState` - 364 edges
3. `ViewTextKey` - 220 edges
4. `SystemBackend` - 124 edges
5. `SandboxEnv` - 123 edges
6. `StatusMessageSpec` - 105 edges
7. `ThemePalette` - 88 edges
8. `SlowScanBackend` - 85 edges
9. `WorkerEvent` - 85 edges
10. `RealBackend` - 84 edges

## Surprising Connections (you probably didn't know these)
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SystemBackend`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`
- 2-file cycle: `src/backup/restore_point.rs -> src/core/backend.rs -> src/backup/restore_point.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (183 total, 55 thin omitted)

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
Cohesion: 0.03
Nodes (8): AppState, StartupStatusFilter, ActiveOnly, All, InactiveOnly, BiosInfoLoaded, DiskReportLoaded, DriverScanCompleted

### Community 5 - "Disk Cleaning & Temporary File Purger"
Cohesion: 0.07
Nodes (42): BLOCKING_FILE_CALLS, BLOCKING_REGISTRY_CALLS, BOUNDS_CHECK_MARKERS, BugRule, IntegerTruncationCast, OutOfBoundsIndexing, UnbufferedBlockingLoop, UncheckedUnwrapOrExpect (+34 more)

### Community 7 - "features_apps.rs"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 9 - "automation.rs"
Cohesion: 0.20
Nodes (12): install_maintenance_task(), is_maintenance_task_installed(), MaintenanceTaskKind, DailyMemoryPurge, TelemetryGuard, WeeklyTempCleaner, MaintenanceTaskStatus, MemoryTrimResult (+4 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.09
Nodes (23): assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), build_populated_sandbox_state(), click_key(), click_text(), drain_state_workers(), init_cpu_parking_state(), init_dashboard_state() (+15 more)

### Community 12 - "theme.rs"
Cohesion: 0.07
Nodes (24): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+16 more)

### Community 13 - "app.rs"
Cohesion: 0.07
Nodes (32): DriverSortMode, Category, NameAsc, UpdatesFirst, DriversSubTab, Drivers, Prerequisites, HardwareSubTab (+24 more)

### Community 14 - "astar.rs"
Cohesion: 0.15
Nodes (7): AStarNode, AStarPlanner, BitSet256, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate

### Community 16 - "in_progress_hud.rs"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 17 - "full_telemetry_audit_with"
Cohesion: 0.07
Nodes (3): CachedScanBackend<'a>, SandboxEnv, normalize_task_path()

### Community 18 - "WinBlastError"
Cohesion: 0.09
Nodes (24): LayoutTier, Compact, Medium, Wide, tier_from_rect(), view(), view(), view() (+16 more)

### Community 19 - "WinBlast Optimizer"
Cohesion: 0.10
Nodes (20): 1. 🦀 Pure-Rust Rune Engine & Zero-Lag GUI, 2. 🛡️ Dual-Layer Transactional Rollback & Safety, 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite, 4. 🧠 Intelligent Hardware & CPU Core Parking Engine, Advanced Optimization Matrix (168 Curated Tweaks), Build Commands, Building & Verification, Comprehensive Quality Assurance & Test Matrix (+12 more)

### Community 20 - "super"
Cohesion: 0.06
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 21 - "clean_apps.rs"
Cohesion: 0.12
Nodes (39): is_unsupported_power_guid_error(), decode_hex(), escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), parse_pipe_reg_target(), read_composite_entry() (+31 more)

### Community 22 - "sys_inventory.rs"
Cohesion: 0.09
Nodes (3): query_smart_health_wmi(), test_query_smart_health_wmi_fallback(), SystemCodeIntegrityInfo

### Community 23 - "BitSet256"
Cohesion: 0.24
Nodes (6): BitSet256, find_min_weight_cycle_edge(), kahn_schedule(), resolve_cycles(), SccResult, tarjan_scc()

### Community 24 - "System Architecture Changelog"
Cohesion: 0.06
Nodes (46): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, fmt1(), fmt2(), fmt3(), idx(), KFM_REDIRECT_MSG, localize_error_text() (+38 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.06
Nodes (33): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, .ALL_VIEWS, AutomationCenter (+25 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.11
Nodes (22): relaunch_as_admin(), apply_and_restart(), check_latest_release(), clean_exe_path(), FALLBACK_REPO, fetch_url_json(), GhAsset, GhRelease (+14 more)

### Community 27 - "RealBackend"
Cohesion: 0.07
Nodes (45): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+37 more)

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.07
Nodes (12): spawn_guarded_worker(), WorkerDropGuard, AppxBatchRemoved, CleanExecuted, CleanTargetsScanned, ExplorerRestarted, LatencyBenchmarkCompleted, OneDriveEdgeResult (+4 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.12
Nodes (8): is_reversible_action(), LedgerEntry, LedgerEntryView, LedgerEntryView<'a>, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak()

### Community 30 - "reg_mutator.rs"
Cohesion: 0.09
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 31 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.18
Nodes (28): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+20 more)

### Community 32 - "driver_update.rs"
Cohesion: 0.12
Nodes (24): lookup(), tweak_description(), tweak_title(), lookup(), tweak_description(), tweak_title(), lookup(), tweak_description() (+16 more)

### Community 33 - "power.rs"
Cohesion: 0.11
Nodes (4): optional_feature_matches(), SetHags, SetMpo, test_sandbox_core_parking_labels_metrics_and_power_sync()

### Community 34 - "spawn_guarded_worker"
Cohesion: 0.08
Nodes (30): BinaryReader<'a>, PresetError, CollectionTooLarge, ConflictingTweaks, DependencyUnsatisfied, DigestMismatch, DuplicateTweakId, EmptyField (+22 more)

### Community 35 - "startup.rs"
Cohesion: 0.06
Nodes (27): BinaryReader, DIGEST_LEN, FORMAT_REVISION_V1, HEADER_LEN, MAX_AUTHOR_LEN, MAX_DESCRIPTION_LEN, MAX_OVERRIDE_COUNT, MAX_PACKAGE_BYTES (+19 more)

### Community 36 - "registry_scanner.rs"
Cohesion: 0.07
Nodes (7): PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, PerfBenchmarkCompleted

### Community 37 - "topological_scheduler.rs"
Cohesion: 0.25
Nodes (15): assert_valid_permutation(), MAX_NODES, position_of(), schedule_optimizations_topological(), test_blocking_reboot_precedes_pure_tail_reboot(), test_cyclic_dependency_lowest_weight_break(), test_diamond_dependency_graph(), test_linear_dependency_sequence() (+7 more)

### Community 38 - "parse_guid"
Cohesion: 0.12
Nodes (27): AI_BLOAT_PREFIXES, check_onedrive_kfm_active(), classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), find_onedrive_setup_exe() (+19 more)

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.09
Nodes (12): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), NtQueryTimerResolution(), PerformanceInformation, query_nt_timer_resolution(), query_system_perf_info(), query_system_process_and_thread_counts(), query_system_thread_count() (+4 more)

### Community 40 - "parking.rs"
Cohesion: 0.11
Nodes (23): test_error_formatting(), test_error_helpers(), WinBlastError, AppX, Backup, Com, Database, DatabaseConversion (+15 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.07
Nodes (29): Failed, WorkerEvent, AppxAndFeaturesLoaded, AppxScanFailed, CleanScanFailed, CoreParkingActionCompleted, CoreParkingUpdated, FeatureToggled (+21 more)

### Community 42 - "ps_runner.rs"
Cohesion: 0.08
Nodes (22): EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 43 - "FailAtMutationBackend"
Cohesion: 0.13
Nodes (27): detect_directx_legacy(), detect_dotnet_desktop_8(), detect_dotnet_desktop_9(), detect_dotnet_framework_481(), detect_openal(), detect_vcredist_2015_2022_x64(), detect_vcredist_2015_2022_x86(), detect_vcredist_all_in_one() (+19 more)

### Community 44 - "power.rs"
Cohesion: 0.06
Nodes (37): Result, apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1 (+29 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.11
Nodes (24): build_indeterminate_track(), build_orbital_spinner(), build_typography_block(), CARD_WIDTH, DOT_COUNT, DOT_PHASE_STEP, ease_in_out_cubic(), format_elapsed() (+16 more)

### Community 46 - "svc_mutator.rs"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 47 - "Changelog"
Cohesion: 0.12
Nodes (16): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+8 more)

### Community 48 - "registry_scanner.rs"
Cohesion: 0.06
Nodes (52): encode_hex(), is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), delete_tree(), delete_value(), delete_value_for_vendor(), REG_DELETE_RIGHT, restore_tree() (+44 more)

### Community 49 - "deserialize"
Cohesion: 0.14
Nodes (5): MetricsSampler, SandboxMetricsSampler, SystemMonitor, SystemMetrics, CoreStatus

### Community 50 - "updater.rs"
Cohesion: 0.16
Nodes (20): BeforeAfterImpact, ThemePalette, admin_warning_banner(), before_after_card(), hardware_overview_card(), health_color(), health_score(), live_telemetry_card() (+12 more)

### Community 51 - "PlatformSecurityInfo"
Cohesion: 0.16
Nodes (10): detect_platform_security_info(), NtQuerySystemInformation(), PlatformSecurityInfo, query_live_code_integrity_options(), query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name() (+2 more)

### Community 52 - "PanickingBackend"
Cohesion: 0.17
Nodes (17): compute_sha256(), OverrideKey, ParamValue, Bool, Bytes, I64, Str, U64 (+9 more)

### Community 55 - "RegRawSnapshot"
Cohesion: 0.18
Nodes (16): backup_root(), enumerate_pnputil_devices(), enumerate_pnputil_driver_store(), MAX_RETRIES, parse_pnputil_devices(), parse_pnputil_drivers(), PnpDeviceInfo, PnpDriverPackage (+8 more)

### Community 56 - "driver_update.rs"
Cohesion: 0.16
Nodes (23): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+15 more)

### Community 57 - "probe_physical_drive"
Cohesion: 0.19
Nodes (14): DiskPerformanceSnapshot, DriveSmartData, extract_null_terminated_str(), HandleGuard, parse_ata_smart_attributes(), parse_nvme_smart_log(), probe_physical_drive(), query_ata_smart_ioctl() (+6 more)

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.09
Nodes (11): background_panic_is_contained_and_clears_in_flight_flag(), background_probe_delivers_payload_without_blocking(), extract_panic_message(), InFlightGuard, LazyLoader, LazyPayload, Bios, Disks (+3 more)

### Community 59 - "task_scanner.rs"
Cohesion: 0.12
Nodes (28): StagedTransition, StageApply, StageRevert, TweakState, AlreadyOptimized, NotApplicable, NotOptimized, PartiallyOptimized (+20 more)

### Community 60 - "backend.rs"
Cohesion: 0.16
Nodes (7): UiThemeStyle, .ALL, Classic, Compact, Glassy, Materialistic, Minimalistic

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.09
Nodes (12): Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot, RiskLevel, Extreme, High (+4 more)

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.09
Nodes (20): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+12 more)

### Community 64 - "CoreParkingState"
Cohesion: 0.17
Nodes (4): test_windows_info_detect_host(), GameSuitabilityReport, SuitabilityCheck, GameProfile

### Community 65 - "task_scanner.rs"
Cohesion: 0.25
Nodes (10): diagnose_storage_medium(), probe_volume_seek_penalty(), StorageMedium, MechanicalHdd, Nvme, SataSsd, Unknown, UsbRemovable (+2 more)

### Community 67 - "Reversibility Instant Requireslogoff"
Cohesion: 0.15
Nodes (5): BiosSystemInfo, DisplayDriverInfo, BenchmarkRunner, BenchmarkScore, RepairTask

### Community 68 - "AutoTuneProfile"
Cohesion: 0.15
Nodes (10): AutoTunePlan, AutoTuneProfile, GamingLowLatency, LaptopBatterySaving, ProAudioDpc, WorkstationStability, AutoTuneResult, create_auto_tune_plan() (+2 more)

### Community 69 - "dijkstra.rs"
Cohesion: 0.31
Nodes (7): DijkstraScheduler, ExecutionOrder, ExecutionStep, test_dijkstra_cycle_resolution(), test_dijkstra_dependency_order(), test_dijkstra_empty_schedule(), test_dijkstra_reboot_cost_prioritizes_instant_tweaks()

### Community 70 - "TaskCache"
Cohesion: 0.21
Nodes (16): display_reg_value_name(), normalize_reg_value_name(), test_risk_valuation(), escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml() (+8 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.19
Nodes (4): DNS_CATALOG, DnsProbeResult, NetworkDiagnostics, query_local_ip()

### Community 73 - "WorkerEvent"
Cohesion: 0.31
Nodes (7): enum_scm_services_by_type(), list_live_services(), query_scm_service_config(), query_scm_win32_services(), resolve_indirect_display_string(), ScHandleGuard, ScmServiceRecord

### Community 74 - "app.rs"
Cohesion: 0.24
Nodes (5): FaultDiagnosticsReport, MinidumpEntry, parse_multi_sz_lossless(), PendingRebootStatus, test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()

### Community 75 - ".set_status_spec"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 76 - "PlatformSecurityInfo"
Cohesion: 0.15
Nodes (27): CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services, StartupApps (+19 more)

### Community 77 - "DriverUpdateItem"
Cohesion: 0.20
Nodes (10): DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget, probe_network_target(), run_full_latency_benchmark(), TargetPingResult (+2 more)

### Community 78 - ".setup"
Cohesion: 0.12
Nodes (12): ColorScheme, .ALL, ArcticAurora, CrimsonSunset, CyberIndigo, EmeraldMint, MidnightStealth, SolarAmber (+4 more)

### Community 79 - "hashset"
Cohesion: 0.32
Nodes (5): ConflictResolution, RavenEngine, test_raven_pareto_dominance(), test_raven_weighted_conflict_resolution(), TweakCriteria

### Community 80 - "deserialize"
Cohesion: 0.17
Nodes (12): ComponentThermal, NVIDIA_SMI_STATUS, probe_acpi_wmi_and_perf(), probe_gpu_thermals(), SystemThermalSnapshot, test_thermal_snapshot_read_no_panic(), ThermalHealth, Cool (+4 more)

### Community 81 - "progress_bar.rs"
Cohesion: 0.39
Nodes (7): rgb(), rgba(), progress_bar(), progress_bar_indeterminate(), progress_bar_labeled(), progress_bar_segmented(), progress_bar_styled()

### Community 82 - "HibernationStatus"
Cohesion: 0.24
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 83 - "normalize_guid_str"
Cohesion: 0.12
Nodes (16): compare_versions(), DriverCategory, Audio, Biometric, Bluetooth, Camera, Chipset, Display (+8 more)

### Community 84 - "DriverUpdateState"
Cohesion: 0.28
Nodes (10): canonical_task_path(), ComGuard, get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed(), query_task_via_com(), query_task_via_schtasks() (+2 more)

### Community 85 - "enums"
Cohesion: 0.12
Nodes (6): EngineMode, Instant, Normal, test_unorthodox_concurrent_worker_event_stress(), test_unorthodox_extreme_registry_corruption(), test_unorthodox_micro_and_ultrawide_viewports()

### Community 86 - "SandboxEnv"
Cohesion: 0.29
Nodes (12): backup_driver(), download_and_install_driver_with_progress(), DriverError, BackupFailed, DownloadFailed, InstallFailed, Other, ScanFailed (+4 more)

### Community 87 - "Runtime"
Cohesion: 0.34
Nodes (13): content_thumb(), CONTENT_X, count_key(), create_test_state(), frames(), runtime_for(), sidebar_thumb(), test_advanced_matrix_scrollbar_structure_and_drag() (+5 more)

### Community 88 - "UiThemeStyle"
Cohesion: 0.11
Nodes (23): FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind, Capability (+15 more)

### Community 89 - "restart_explorer_safely"
Cohesion: 0.25
Nodes (7): StartupSource, RegistryHkcuRun, RegistryHklmRun, ScheduledTaskLogon, StartupFolderAllUsers, StartupFolderCurrentUser, UwpStartupTask

### Community 90 - "latency_bench.rs"
Cohesion: 0.21
Nodes (10): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching(), unblock_telemetry_hosts_in_content() (+2 more)

### Community 91 - "ThemePalette"
Cohesion: 0.25
Nodes (8): create_restore_point(), ensure_preflight_restore_point(), ensure_preflight_restore_point_with(), is_recent_restore_point_present_with(), list_restore_points(), parse_restore_point_time(), test_ensure_preflight_restore_point_sandbox(), test_parse_restore_point_time_formats()

### Community 92 - "edge_cases_unorthodox.rs"
Cohesion: 0.17
Nodes (3): RollbackRefreshData, CoreParkingState, PowerSchemeInfo

### Community 93 - "FeatureInstallState"
Cohesion: 0.19
Nodes (13): apply_legacy_f8_boot_menu(), BitLockerProtectionState, Indeterminate, ProtectedOn, ProtectionOff, BootMenuPolicyResult, AppliedLegacy, RestoredStandard (+5 more)

### Community 94 - "WindowsOptionalFeature"
Cohesion: 0.24
Nodes (9): active_logical_core_count(), compute_nt_cpu_deltas_into(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_nt_processor_performance_into(), query_processor_group_bases(), sample_live_cpu_and_parking() (+1 more)

### Community 95 - "LiveScheduledTaskInfo"
Cohesion: 0.23
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 96 - "appx_scanner.rs"
Cohesion: 0.33
Nodes (7): appx_package_matches_family(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches(), test_split_appx_name_and_publisher_with_underscores_and_prefix()

### Community 97 - "adaptive_tuning.rs"
Cohesion: 0.38
Nodes (8): AdaptiveTuningConfig, apply_adaptive_tuning(), evaluate_adaptive_tuning(), MemoryStats, query_physical_ram(), test_apply_adaptive_tuning_sandbox(), test_evaluate_adaptive_tuning_high_spec_nvme(), test_evaluate_adaptive_tuning_low_spec_hdd()

### Community 98 - "advanced_matrix.rs"
Cohesion: 0.36
Nodes (9): AppMode, Advanced, Easy, build_dropdown_overlay(), build_sidebar(), mode_tab(), nav_button(), nav_section_header() (+1 more)

### Community 99 - "WinBlast Optimizer — Developer & Architecture Guide"
Cohesion: 0.22
Nodes (9): Adding a New Declarative Tweak, Checklist for Adding Tweaks:, Coding & Safety Guidelines, WinBlast Optimizer — Developer & Architecture Guide, Key Subsystems:, Running Tests:, System Architecture, Testing with `SystemBackend` & `SandboxEnv` (+1 more)

### Community 100 - "Project History & Release Version Ledger"
Cohesion: 0.27
Nodes (11): apply_chassis_governor(), ChassisProfile, evaluate_chassis_profile(), FormFactor, Desktop, Laptop, Unknown, probe_system_form_factor() (+3 more)

### Community 101 - ".sign"
Cohesion: 0.22
Nodes (5): build_signing_transcript(), SignaturePolicy, AllowUnsigned, Required, VerifyIfPresent

### Community 102 - "PowerSchemeInfo"
Cohesion: 0.20
Nodes (6): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage

### Community 103 - "Low-Level NT Core & Kernel Architecture"
Cohesion: 0.18
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 106 - "PlatformSecurityInfo"
Cohesion: 0.25
Nodes (5): I18n, test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), test_left_menu_translations_all_8_languages(), TRANSLATIONS

### Community 108 - "RestorePoint"
Cohesion: 0.36
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 109 - "probe_physical_drive"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 110 - "app.rs"
Cohesion: 0.20
Nodes (10): DriverProgress, BackingUp, Comparing, Complete, Downloading, Failed, Idle, Installing (+2 more)

### Community 112 - "ui_scrollbar_test.rs"
Cohesion: 0.47
Nodes (7): diff_pair_card(), diff_single_card(), directed_pair(), header_with_dir(), view_items_with_palette(), view_state(), view_with_palette()

### Community 113 - "test_wave1_compound_actions_deep_edge_cases"
Cohesion: 0.42
Nodes (9): format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service(), validate_service_name(), is_protected_core_service() (+1 more)

### Community 114 - "progress_bar.rs"
Cohesion: 0.25
Nodes (7): PresetCategory, Custom, Gaming, Latency, PowerEfficiency, Privacy, Workstation

### Community 116 - "tools/game_overlay.rs"
Cohesion: 0.24
Nodes (6): acquire_registry_mount_privileges(), HiveMountGuard, HiveMountGuard<'a>, propagate_hkcu_dword(), SysprepPropagationReport, test_sysprep_propagation_sandbox()

### Community 117 - "lazy_loader.rs"
Cohesion: 0.27
Nodes (10): create_sandbox_test_item(), download_and_install_driver(), DriverProgressCallback, DriverUpdateItem, scan_class_registry(), scan_online_drivers(), scan_online_drivers_with_progress(), test_create_sandbox_item_metadata() (+2 more)

### Community 118 - "TaskCache"
Cohesion: 0.25
Nodes (4): compute_nt_cpu_deltas(), parse_pdh_processor_instance(), PdhParkingCollector, test_nt_cpu_delta_handles_idledisable_and_multigroup()

### Community 119 - "boot_config.rs"
Cohesion: 0.29
Nodes (5): EASING_CLASSIC, EASING_COMPACT, EASING_GLASSY, EASING_MATERIALISTIC, EASING_MINIMALISTIC

### Community 120 - "power_governor.rs"
Cohesion: 0.17
Nodes (5): ScheduledTaskRunResult, ServiceLiveUpdated, StartupBatchToggled, StartupEntryToggled, TasksBatchToggled

### Community 121 - "GameProfile"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 124 - "storage.rs"
Cohesion: 0.48
Nodes (3): OptimizationGoal, metric_pill(), view()

### Community 125 - "adaptive_tuning.rs"
Cohesion: 0.29
Nodes (7): DriverUpdateState, Downloading, Installed, Installing, ReadyToInstall, UpdateAvailable, UpToDate

### Community 129 - ".sync_startup_tasks_busy"
Cohesion: 0.33
Nodes (6): GamingSubTab, FpsBenchmark, GameOverlay, GameProfiles, GameSuitability, LiveMonitor

### Community 130 - ".all_physical"
Cohesion: 0.40
Nodes (4): GpuVendor, Amd, Intel, Nvidia

### Community 134 - "DateTime"
Cohesion: 0.47
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 135 - "Utc"
Cohesion: 0.50
Nodes (3): test_view_translations_all_8_languages_complete(), tr_view(), VIEW_TRANSLATIONS

### Community 137 - "HealthSeverity"
Cohesion: 0.25
Nodes (8): assess_system_health(), HealthSeverity, Critical, Info, Warning, SystemHealthAssessment, SystemHealthIssue, test_iso_29119_health_assessment_scoring_and_grades()

### Community 138 - "startup.rs"
Cohesion: 0.16
Nodes (15): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), list_startup_entries(), read_startup_approved_enabled(), replace_ascii_ci() (+7 more)

### Community 139 - "tools/gaming_stats.rs"
Cohesion: 0.32
Nodes (3): GameFpsComparison, get_standard_game_benchmarks(), LiveGameTelemetry

### Community 142 - "path"
Cohesion: 0.43
Nodes (4): export_profile(), import_profile(), test_profile_export_import(), TweakProfile

### Community 143 - "MatrixSortMode"
Cohesion: 0.29
Nodes (7): MatrixSortMode, Category, Default, NameAsc, NameDesc, Risk, Status

### Community 162 - "UpdateStatus"
Cohesion: 0.29
Nodes (7): UpdateStatus, Available, Checking, Downloading, Idle, ReadyToRestart, UpToDate

### Community 164 - "ActiveDropdown"
Cohesion: 0.33
Nodes (6): ActiveDropdown, ColorScheme, Language, None, ThemeMode, ThemeStyle

### Community 165 - "StartupSortMode"
Cohesion: 0.33
Nodes (6): StartupSortMode, NameAsc, NameDesc, Source, StatusActiveFirst, StatusInactiveFirst

### Community 167 - "DiskSubsystemReport"
Cohesion: 0.40
Nodes (4): DiskSubsystemReport, PhysicalDriveHealth, probe_volume_fallback(), test_disk_subsystem_report_probe_no_panic()

### Community 168 - "OverallHealthGrade"
Cohesion: 0.33
Nodes (5): OverallHealthGrade, Good, NeedsOptimization, Optimal, Suboptimal

### Community 169 - "views/network_diag.rs"
Cohesion: 0.70
Nodes (3): localize_dns_category(), localize_dns_status(), view()

### Community 170 - "CleanSortMode"
Cohesion: 0.50
Nodes (4): CleanSortMode, NameAsc, SizeAsc, SizeDesc

### Community 171 - "PrivacySortMode"
Cohesion: 0.50
Nodes (4): PrivacySortMode, Category, NameAsc, UnprotectedFirst

### Community 172 - "PresetKey"
Cohesion: 0.50
Nodes (4): PresetKey, Gaming, LaptopBattery, ProAudio

## Knowledge Gaps
- **1256 isolated node(s):** `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` (+1251 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1880 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **55 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppState` connect `.Toggle .Set .Apply` to `.sync_startup_tasks_busy`, `spawn_guarded_worker`, `WinBlast Optimizer — Developer & Architecture Guide`, `Item`, `features_apps.rs`, `Utc`, `automation.rs`, `HealthSeverity`, `startup.rs`, `enum_scm_services_by_type`, `app.rs`, `astar.rs`, `MatrixSortMode`, `tools/gaming_stats.rs`, `startup.rs`, `WinBlastError`, `theme.rs`, `.Reg State() Vendor()`, `RealBackend`, `GPU Silicon & Hardware Tuning Policies`, `Hardware Diagnostics & GPU Silicon Tuning`, `reg_mutator.rs`, `Win32 Registry Scanner & Mutator`, `UpdateStatus`, `ActiveDropdown`, `registry_scanner.rs`, `StartupSortMode`, `DiskSubsystemReport`, `Embedded Catalog Test`, `CleanSortMode`, `PrivacySortMode`, `FailAtMutationBackend`, `views/network_diag.rs`, `svc_mutator.rs`, `Changelog`, `.execute_auto_tune_profile`, `deserialize`, `updater.rs`, `PlatformSecurityInfo`, `Scandetailkind Appxinstalled Appxinstalledps`, `task_scanner.rs`, `backend.rs`, `CoreParkingState`, `Reversibility Instant Requireslogoff`, `AutoTuneProfile`, `dijkstra.rs`, `TaskCache`, `Project Documentation, Architecture & Licensing`, `app.rs`, `.set_status_spec`, `PlatformSecurityInfo`, `DriverUpdateItem`, `.setup`, `hashset`, `deserialize`, `HibernationStatus`, `normalize_guid_str`, `enums`, `Runtime`, `edge_cases_unorthodox.rs`, `LiveScheduledTaskInfo`, `advanced_matrix.rs`, `WinBlast Optimizer — Developer & Architecture Guide`, `PowerSchemeInfo`, `PlatformSecurityInfo`, `GameOverlaySettings`, `ui_scrollbar_test.rs`, `lazy_loader.rs`, `power_governor.rs`, `reg_helpers.rs`, `storage.rs`, `WindowsInfo`?**
  _High betweenness centrality (0.282) - this node is a cross-community bridge._
- **Are the 25 inferred relationships involving `AppState` (e.g. with `OptimizationGoal` and `OptimizationPlan`) actually correct?**
  _`AppState` has 25 INFERRED edges - model-reasoned connections that need verification._
- **What connects `CacheSet<K, V>`, `CacheSlot<K, V>`, `ActiveChanges` to the rest of the system?**
  _1256 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00390625 - nodes in this community are weakly interconnected._
- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `advanced_matrix.rs`, `.Toggle .Set .Apply`, `PlatformSecurityInfo`, `Scandetailkind Appxinstalled Appxinstalledps`, `PlatformSecurityInfo`, `app.rs`, `.setup`, `ui_scrollbar_test.rs`, `WinBlastError`, `updater.rs`, `System Architecture Changelog`, `task_scanner.rs`?**
  _High betweenness centrality (0.265) - this node is a cross-community bridge._
- **Should `action_button_guarded` be split into smaller, more focused modules?**
  _Cohesion score 0.010050251256281407 - nodes in this community are weakly interconnected._
- **Why does `ViewTextKey` connect `action_button_guarded` to `advanced_matrix.rs`, `Utc`, `views/network_diag.rs`, `PlatformSecurityInfo`, `WinBlastError`, `updater.rs`, `task_scanner.rs`, `storage.rs`?**
  _High betweenness centrality (0.133) - this node is a cross-community bridge._