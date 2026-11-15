# Graph Report - WinBlast0  (2026-11-15)

## Corpus Check
- cluster-only mode — file stats not available

## Summary
- 2605 nodes · 5611 edges · 107 communities (66 shown, 41 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 231 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e99d53ea`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Core UI Internationalization & Text Catalog
- Core UI Internationalization & Text Catalog
- Privacy, Telemetry & AI Hardening Catalog
- Async() .Toggle .Refresh
- .Toggle .Set .Apply
- Disk Cleaning & Temporary File Purger
- Windows OS Settings & UX Customization
- Integration Suite: Tweak Sandbox E2E
- .Reg .Set Power
- Scandetailkind Appxinstalled Appxinstalledps
- Test Composite With()
- View() Card() Switch
- .Set Enabled() .Remove
- Accent Hover Secondary
- View() Color Scrollspec
- Hardware Diagnostics & GPU Silicon Tuning
- Integration Suite: Tweak Sandbox E2E
- Process Token Information
- Core UI Internationalization & Text Catalog
- Hardware Diagnostics & GPU Silicon Tuning
- Multi-Language Tweak Localization Tables
- Integration Suite: Tweak Sandbox E2E
- System Architecture Changelog
- .Reg State() Vendor()
- Appx Package() Prefixes
- Scan Test Single
- GPU Silicon & Hardware Tuning Policies
- Hardware Diagnostics & GPU Silicon Tuning
- Core UI Internationalization & Text Catalog
- Win32 Registry Scanner & Mutator
- Hardware Diagnostics & GPU Silicon Tuning
- Hardware Diagnostics & GPU Silicon Tuning
- Rollback Panic (`Src/Backup/State
- Integration Suite: Tweak Sandbox E2E
- Integration Suite: Tweak Sandbox E2E
- Disabled() Delete .Set
- Task Scheduler Inspection Subsystem
- AppX Package & Modern App Scanner
- .Create Restore Point()
- Embedded Catalog Test
- WinBlast0 Architectural Specifications
- Integration Suite: Real Windows Env
- Hardware Diagnostics & GPU Silicon Tuning
- Dynamicdwordformula Logicalcorecount Totalraminkb
- Telemetry Full Blocked
- Integration Suite: Ui Responsive E2E
- Project Documentation, Architecture & Licensing
- Task Scheduler Inspection Subsystem
- Windows Unattend Deployment Generator
- State Ledger & Restore Point Engine
- Optional Dism Feature()
- Win32 Registry Scanner & Mutator
- Project History & Release Version Ledger
- Project History & Release Version Ledger
- Project History & Release Version Ledger
- Hardware Diagnostics & GPU Silicon Tuning
- State Ledger & Restore Point Engine
- Hosts Telemetry Block
- Integration Suite: Tweak Sandbox E2E
- Hardware Diagnostics & GPU Silicon Tuning
- Restore Points() .List
- Error.Rs Winblasterror Appx
- WinBlast0 Architectural Specifications
- Project Documentation, Architecture & Licensing
- Startup Tasks & Auto-Run Explorer
- Reversibility Instant Requireslogoff
- WinBlast0 Architectural Specifications
- Hardware Diagnostics & GPU Silicon Tuning
- Themed Interactive Scrollbar Widget
- Multi-Language Tweak Localization Tables
- Project Documentation, Architecture & Licensing
- .Create Metrics Sampler()
- .Detect Core Parking()
- Hardware Diagnostics & GPU Silicon Tuning
- Hardware Diagnostics & GPU Silicon Tuning
- Installed Appx .List
- Startup Entries() .List
- Elevated() Token Closehandle
- Startup Tasks & Auto-Run Explorer
- Startup Tasks & Auto-Run Explorer
- Hibernation .Query Status()
- WinBlast0 Architectural Specifications
- Integration Suite: Tweak Sandbox E2E
- Project Documentation, Architecture & Licensing
- Diff With View
- Integration Suite: Ui Responsive E2E
- Hardware Diagnostics & GPU Silicon Tuning
- .Default() .New() Sandboxtempdir

## God Nodes (most connected - your core abstractions)
1. `TextKey` - 408 edges
2. `AppState` - 227 edges
3. `SandboxEnv` - 114 edges
4. `SystemBackend` - 110 edges
5. `StatusMessageSpec` - 105 edges
6. `SlowScanBackend` - 85 edges
7. `RealBackend` - 84 edges
8. `PanickingBackend` - 81 edges
9. `FailAtMutationBackend` - 79 edges
10. `WorkerEvent` - 68 edges

## Surprising Connections (you probably didn't know these)
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `SandboxEnv`  [EXTRACTED]
  DEVELOPMENT.md → src/core/backend.rs
- `Changelog` --references--> `AppState`  [EXTRACTED]
  CHANGELOG.md → src/app.rs
- `WinBlast Optimizer — Developer & Architecture Guide` --references--> `AppState`  [EXTRACTED]
  DEVELOPMENT.md → src/app.rs
- `assets/rules/registry_lag.toml` --references--> `TweakDefinition`  [EXTRACTED]
  Project WinBlast0 Project Structure.md → src/core/risk.rs
- `Changelog` --references--> `SystemBackend`  [EXTRACTED]
  CHANGELOG.md → src/core/backend.rs

## Import Cycles
- 2-file cycle: `src/core/backend.rs -> src/hardware/parking.rs -> src/core/backend.rs`

## Hyperedges (group relationships)
- **WinBlast0 Reactive Systems Architecture** — development_system_architecture, project_winblast0_project_structure_core_architecture_code_blueprint, readme_cpu_core_parking_hybrid_architecture_vis, readme_key_architecture_capabilities [INFERRED 0.85]

## Communities (107 total, 41 thin omitted)

### Community 0 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.01
Nodes (387): TextKey, ActiveChanges, ActiveOptimizations, ActiveRailAc, ActiveRailDc, ActiveRam, ActiveUnparked, AdapterName (+379 more)

### Community 1 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.06
Nodes (83): BeforeAfterImpact, CleanAppsSubTab, AppxFeatures, DiskCleaner, WinGetInstaller, StartupTasksSubTab, ScheduledTasks, Services (+75 more)

### Community 2 - "Privacy, Telemetry & AI Hardening Catalog"
Cohesion: 0.02
Nodes (93): StatusMessageSpec, ActionMessage, AppxFeatureScanFailed, AppxFeatureScanned, AppxFeatureScanning, AppxUninstalled, AppxUninstallFailed, BaselineResetCompleted (+85 more)

### Community 3 - "Async() .Toggle .Refresh"
Cohesion: 0.05
Nodes (10): AppState, PresetKind, Gaming, LaptopBattery, ProAudio, ApplyCompleted, BaselineReset, LatencyBenchmarkCompleted (+2 more)

### Community 4 - ".Toggle .Set .Apply"
Cohesion: 0.06
Nodes (32): CleanExecutionReport, CleanTargetKind, .ALL, DeliveryOptimizationCache, Prefetch, RecycleBin, ThumbnailCache, UserTemp (+24 more)

### Community 6 - "Windows OS Settings & UX Customization"
Cohesion: 0.04
Nodes (49): ScanDetailKind, AppxInstalled, AppxInstalledPs, AppxNoPackage, AppxRemovedOptimized, AppxRemovedPs, BinaryMatch, BinaryNotConfigured (+41 more)

### Community 8 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.08
Nodes (20): AStarNode, AStarPlanner, OptimizationGoal, OptimizationPlan, test_astar_finds_optimal_gaming_path(), test_astar_respects_reboot_constraint(), TweakCandidate, DijkstraScheduler (+12 more)

### Community 9 - ".Reg .Set Power"
Cohesion: 0.04
Nodes (45): Added, Changed, Changelog, Fixed, Fixed & Improved, Highlights, v0.1.0] - 2026-10-28, v0.2.0] - 2026-11-03 (+37 more)

### Community 10 - "Scandetailkind Appxinstalled Appxinstalledps"
Cohesion: 0.08
Nodes (21): load_embedded_rules(), drain_until(), HkcuScratchGuard, make_synth_tweak(), normalize_task_path(), test_app_state_export_import_profile_staging_sync(), test_app_state_hardware_and_privacy_ledger_recording(), test_app_state_instant_mode_error_unlocks_in_flight_tweaks() (+13 more)

### Community 11 - "Test Composite With()"
Cohesion: 0.07
Nodes (29): WorkerEvent, AppxAndFeaturesLoaded, AppxBatchRemoved, AppxScanFailed, CleanExecuted, CleanScanFailed, CleanTargetsScanned, CoreParkingActionCompleted (+21 more)

### Community 12 - "View() Card() Switch"
Cohesion: 0.09
Nodes (31): ACCENT, ACCENT_HOVER, ACCENT_PRESS, BG_CARD, BG_CARD_HOVER, BG_PRIMARY, BG_SIDEBAR, BORDER (+23 more)

### Community 14 - "Accent Hover Secondary"
Cohesion: 0.08
Nodes (8): spawn_guarded_worker(), WorkerDropGuard, ExplorerRestarted, OneDriveEdgeResult, ScheduledTaskRunResult, ServiceLiveUpdated, StartupEntryToggled, TelemetryHostsBlocked

### Community 15 - "View() Color Scrollspec"
Cohesion: 0.10
Nodes (20): ThemeMode, Dark, Light, assert_clean_appx_compact_vertical_geometry(), assert_frame_clean(), build_populated_sandbox_state(), click_key(), click_text() (+12 more)

### Community 17 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.07
Nodes (26): AppMode, Advanced, Easy, EngineMode, Instant, Normal, panic_payload_to_string(), PowerActionKind (+18 more)

### Community 19 - "Process Token Information"
Cohesion: 0.06
Nodes (32): ADMIN_RUN_MSG, ERROR_PREFIX_TABLE, KFM_REDIRECT_MSG, POLICY_LOCKED_LABEL, STATIC_APPX_SCANNING, STATIC_BASELINE_RESET_OK, STATIC_BASELINE_RESETTING, STATIC_CHANGELOG_CLEARED (+24 more)

### Community 20 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.14
Nodes (29): encode_hex(), normalize_reg_value_name(), delete_value_for_vendor(), check_dword(), DISPLAY_CLASS_LOWER, evaluate_dword(), evaluate_dword_value(), evaluate_dword_value_with_kind() (+21 more)

### Community 21 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.08
Nodes (11): CPU_SAMPLE_MIN_INTERVAL, GetPerformanceInfo(), parse_pdh_processor_instance(), PdhParkingCollector, PerformanceInformation, query_system_process_and_thread_counts(), query_system_thread_count(), STATUS_INFO_LENGTH_MISMATCH (+3 more)

### Community 22 - "Multi-Language Tweak Localization Tables"
Cohesion: 0.11
Nodes (29): Advanced Optimization Matrix UI, CPU Core Parking Topology Visualizer UI, Easy Mode Telemetry Dashboard UI, GPU Tuning & Vendor Gating UI, ActiveView, AdvancedMatrix, BackupCenter, CleanAndApps (+21 more)

### Community 23 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.11
Nodes (20): test_gpu_vendor_compatibility(), detect_dxgi_vram_table(), detect_gpus(), detect_vendor(), DetectedGpu, DevInfoGuard, DISPLAY_CLASS_PREFIX, GpuVendor (+12 more)

### Community 24 - "System Architecture Changelog"
Cohesion: 0.11
Nodes (9): appx_package_matches_family(), AppxCache, check_package_removed(), check_package_removed_ps(), split_appx_name_and_publisher(), test_appx_cache_loaded_empty_no_fallback(), test_appx_publisher_suffix_matching(), test_evaluate_package_with_kind_all_cache_branches() (+1 more)

### Community 25 - ".Reg State() Vendor()"
Cohesion: 0.12
Nodes (23): AI_BLOAT_PREFIXES, classify_appx_package(), CURATED_WINGET_APPS, DISM_LOCK, extract_appx_short_name(), extract_winget_error_detail(), is_appx_arch(), is_appx_version() (+15 more)

### Community 26 - "Appx Package() Prefixes"
Cohesion: 0.12
Nodes (26): Language, .ALL, Chinese, English, French, German, Japanese, Russian (+18 more)

### Community 27 - "Scan Test Single"
Cohesion: 0.07
Nodes (8): RealBackend, resolve_persistent_app_dir(), delete_value(), is_hags_enabled(), is_mpo_disabled(), set_gpu_msi_mode(), set_hags(), set_mpo_disabled()

### Community 28 - "GPU Silicon & Hardware Tuning Policies"
Cohesion: 0.08
Nodes (22): display_reg_value_name(), EMBEDDED_CATALOG_AI_PRIVACY_EXT, EMBEDDED_CATALOG_APPX, EMBEDDED_CATALOG_HARDWARE, EMBEDDED_CATALOG_OS_SETTINGS, EMBEDDED_CATALOG_PERF_POWER_EXT, EMBEDDED_CATALOG_PRIVACY, EMBEDDED_CATALOG_REGISTRY_LAG (+14 more)

### Community 29 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.11
Nodes (28): CAT_BROWSERS, CAT_DEVELOPMENT, CAT_DIAGNOSTICS, CAT_MEDIA, CAT_UTILITIES, CATEGORY_TABLE, contains_ascii_ci(), is_privacy_or_telemetry_category() (+20 more)

### Community 30 - "Core UI Internationalization & Text Catalog"
Cohesion: 0.08
Nodes (4): fallback_scheme_name(), resolve_indirect_string(), strip_indirect_prefix(), test_fallback_scheme_name_indirect_stripping()

### Community 34 - "Rollback Panic (`Src/Backup/State"
Cohesion: 0.17
Nodes (16): create_and_activate_performance_plan(), format_guid(), get_active_power_scheme(), list_power_schemes(), LocalGuidPtr, map_win32_err(), normalize_guid_str(), parse_guid() (+8 more)

### Community 35 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.14
Nodes (17): CRITICAL_CORE_SERVICES, encode_startup_approved_bytes(), enum_scm_services_by_type(), expand_system_root_vars(), is_microsoft_run_command(), is_microsoft_service_binary(), is_protected_core_service(), list_live_services() (+9 more)

### Community 36 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.08
Nodes (22): SandboxOp, AppxRemove, BlockHosts, CleanExecuted, CreateRestorePoint, ExplorerRestarted, FeatureSet, GpuMsi (+14 more)

### Community 37 - "Disabled() Delete .Set"
Cohesion: 0.08
Nodes (17): Result, WinBlastError, AppX, Backup, Database, ElevationRequired, Hardware, OsIncompatible (+9 more)

### Community 38 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.16
Nodes (7): LedgerEntry, StateLedger, SystemBaseline, test_ledger_crud(), revert_tweak(), revert_tweaks_by_id(), revert_tweaks_by_id_with()

### Community 39 - "AppX Package & Modern App Scanner"
Cohesion: 0.13
Nodes (13): NtQueryTimerResolution(), query_nt_timer_resolution(), test_nt_timer_resolution(), DpcJitterResult, LatencyBenchmarkSnapshot, measure_dpc_jitter(), PING_TARGETS, PingTarget (+5 more)

### Community 40 - ".Create Restore Point()"
Cohesion: 0.12
Nodes (15): list_startup_entries(), read_startup_approved_enabled(), scan_run_key(), scan_startup_folder(), scan_uwp_startup_tasks(), set_startup_entry_enabled(), set_startup_entry_enabled_by_id(), StartupEntry (+7 more)

### Community 41 - "Embedded Catalog Test"
Cohesion: 0.13
Nodes (15): is_unsupported_power_guid_error(), apply_parking_snapshot(), capture_parking_snapshot(), compute_is_fully_unparked(), CPMAXCORES, CPMAXCORES1, CPMINCORES, CPMINCORES1 (+7 more)

### Community 42 - "WinBlast0 Architectural Specifications"
Cohesion: 0.14
Nodes (19): optional_feature_matches(), FeatureInstallState, Disabled, DisablePending, Enabled, EnablePending, Removed, OptionalFeatureKind (+11 more)

### Community 44 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.23
Nodes (21): CachedScanBackend, RiskValuation, TweakDefinition, ActionEval, Evaluated, FeatureAuthRequired, SkippedPower, evaluate_single_action() (+13 more)

### Community 45 - "Dynamicdwordformula Logicalcorecount Totalraminkb"
Cohesion: 0.09
Nodes (22): DynamicDwordFormula, LogicalCoreCount, TotalRamInKb, TotalRamInKbPlusMargin, PowerTargetScope, AcAndDc, AcOnly, DcOnly (+14 more)

### Community 46 - "Telemetry Full Blocked"
Cohesion: 0.16
Nodes (9): test_windows_info_detect_host(), WindowsInfo, format_sc_output(), output_has_sc_code(), sc_start_mode(), set_service_start_type(), start_service(), stop_service() (+1 more)

### Community 47 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.19
Nodes (16): validate_registry_mutation_path(), delete_tree(), REG_DELETE_RIGHT, restore_tree(), restore_tree_node(), set_binary(), set_dword(), set_dword_for_vendor() (+8 more)

### Community 48 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.16
Nodes (14): set_scheduled_task_enabled(), canonical_task_path(), ComGuard, decode_oem_bytes(), get_task_enabled(), is_disabled_status(), parse_csv_line(), query_single_task_detailed() (+6 more)

### Community 49 - "Task Scheduler Inspection Subsystem"
Cohesion: 0.18
Nodes (20): parse_pipe_reg_target(), unescape_pipe(), ABSENT_SENTINEL, apply_tweak(), apply_tweak_with(), apply_tweaks_by_id(), apply_tweaks_by_id_with(), ensure_tweak_compatible() (+12 more)

### Community 51 - "State Ledger & Restore Point Engine"
Cohesion: 0.12
Nodes (8): is_known_numeric_reg_sz(), test_known_numeric_reg_sz(), FILE_ATTRIBUTE_REPARSE_POINT, HkcuTestEnvGuard, TempTestEnvGuard, test_ntfs_junction_and_readonly_cleanup_sandbox(), test_real_backend_readonly_live_host_audit(), test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()

### Community 52 - "Optional Dism Feature()"
Cohesion: 0.15
Nodes (11): block_telemetry_hosts(), block_telemetry_hosts_in_content(), find_exact_marker_line(), HOSTS_BLOCK_FOOTER, HOSTS_BLOCK_HEADER, remove_appx_package(), test_hosts_blocking_ignores_comments_and_subdomains(), test_hosts_bom_ipv6_tabs_and_exact_footer_matching() (+3 more)

### Community 53 - "Win32 Registry Scanner & Mutator"
Cohesion: 0.22
Nodes (10): lookup(), lookup(), lookup(), lookup(), localize_tweak(), LocalizedTweakText, test_all_168_tweaks_localized_all_7_languages(), lookup() (+2 more)

### Community 54 - "Project History & Release Version Ledger"
Cohesion: 0.23
Nodes (13): escape_win32_quoted_arg(), escape_xml(), format_reg_add_cmd(), format_reg_delete_cmd(), generate_autounattend_xml(), map_hive(), sanitize_computer_name(), sanitize_local_user() (+5 more)

### Community 56 - "Project History & Release Version Ledger"
Cohesion: 0.28
Nodes (15): escape_pipe(), format_pipe_reg_target(), merge_composite_pair(), parse_composite_pairs(), read_composite_entry(), test_composite_string_merge_preserves_siblings(), test_hkcr_and_shallow_tree_delete_blocked(), test_pipe_escape_handles_default_and_backslashes() (+7 more)

### Community 57 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.21
Nodes (6): classify_efficiency_records(), CpuInfo, query_win32_core_topology(), test_classify_efficiency_records_hybrid_and_non_ht_and_uniform(), test_cpu_detect_host_safe(), Win32CoreTopology

### Community 58 - "State Ledger & Restore Point Engine"
Cohesion: 0.18
Nodes (4): CoreParkingState, CoreStatus, test_core_parking_detect(), is_on_ac_power()

### Community 59 - "Hosts Telemetry Block"
Cohesion: 0.18
Nodes (4): task_leaf_name(), TaskCache, test_evaluate_task_with_kind_all_cache_and_error_branches(), test_task_cache_loaded_does_not_cross_match_prefixes()

### Community 60 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.18
Nodes (3): MetricsSampler, SystemMonitor, SystemMetrics

### Community 61 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.19
Nodes (7): AppxSafetyTier, AiBloat, Caution, ProtectedCore, SafeBloat, InstalledAppxPackage, list_installed_appx_from_cache()

### Community 62 - "Restore Points() .List"
Cohesion: 0.26
Nodes (8): active_logical_core_count(), compute_nt_cpu_deltas(), NtCpuSampler, NtQuerySystemInformation(), query_nt_processor_performance(), query_processor_group_bases(), sample_live_cpu_and_parking(), SystemProcessorPerformanceInformation

### Community 63 - "Error.Rs Winblasterror Appx"
Cohesion: 0.17
Nodes (10): DriveMediaKind, Hdd, Nvme, Removable, Ssd, Unknown, HandleGuard, probe_drive_media_kind() (+2 more)

### Community 64 - "WinBlast0 Architectural Specifications"
Cohesion: 0.21
Nodes (5): export_profile(), import_profile(), test_profile_export_import(), TweakProfile, create_restore_point()

### Community 66 - "Startup Tasks & Auto-Run Explorer"
Cohesion: 0.24
Nodes (3): HibernationStatus, query_hibernation_and_standby(), set_hibernation_enabled()

### Community 68 - "WinBlast0 Architectural Specifications"
Cohesion: 0.31
Nodes (8): check_service_start_type(), evaluate_service(), evaluate_service_value(), evaluate_service_value_with_kind(), get_service_start_type(), start_type_name(), test_evaluate_absent_optional_service(), test_evaluate_service_value_with_kind_all_branches()

### Community 69 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.20
Nodes (10): 1\. Domain Model & Risk Valuation Engine**, 2\. Declarative Rule Format**, 3\. Hardware GPU MSI Mode & Priority Optimizer Logic**, 4\. Auto-Elevation Manifest Setup**, Architectural Enhancements**, Core Architecture & Code Blueprint**, assets/rules/registry_lag.toml, Project Directory Structure** (+2 more)

### Community 72 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.22
Nodes (4): broadcast_shell_settings_changed(), EnvBlockGuard, HandleCloser, restart_explorer_safely()

### Community 73 - ".Create Metrics Sampler()"
Cohesion: 0.25
Nodes (3): test_all_languages_and_all_keys_exhaustive(), test_i18n_all_languages_populated(), TRANSLATIONS

### Community 76 - "Hardware Diagnostics & GPU Silicon Tuning"
Cohesion: 0.29
Nodes (5): detect_platform_security_info(), NtQuerySystemInformation(), query_live_code_integrity_options(), test_ram_and_platform_inventory_nonblocking(), total_physical_ram_kb()

### Community 77 - "Installed Appx .List"
Cohesion: 0.25
Nodes (8): BatchCompleted, HagsUpdated, LanguageChanged, MpoUpdated, ScanCompleted, ScanFailed, test_status_i18n_all_languages_and_core_specs(), test_turkish_language_changed_status_message()

### Community 83 - "Hibernation .Query Status()"
Cohesion: 0.33
Nodes (6): RiskLevel, Extreme, High, Low, Moderate, Safe

### Community 84 - "WinBlast0 Architectural Specifications"
Cohesion: 0.47
Nodes (3): SystemMonitor, test_monitor_cold_main_thread_never_sleeps_or_spikes(), test_monitor_sample()

### Community 86 - "Integration Suite: Tweak Sandbox E2E"
Cohesion: 0.40
Nodes (3): check_onedrive_kfm_active(), find_onedrive_setup_exe(), uninstall_onedrive_safe()

### Community 87 - "Project Documentation, Architecture & Licensing"
Cohesion: 0.40
Nodes (5): Reversibility, Instant, RequiresLogoff, RequiresPackageReinstall, RequiresReboot

### Community 89 - "Integration Suite: Ui Responsive E2E"
Cohesion: 0.50
Nodes (4): query_ram_dimms(), RamDimmInfo, read_smbios_string(), smbios_mem_type_name()

## Knowledge Gaps
- **738 isolated node(s):** `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc`, `ActiveRailDc`, `ActiveRam` (+733 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1147 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **41 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `TextKey` connect `Core UI Internationalization & Text Catalog` to `Core UI Internationalization & Text Catalog`, `Async() .Toggle .Refresh`, `.Create Metrics Sampler()`, `View() Card() Switch`, `View() Color Scrollspec`, `Integration Suite: Tweak Sandbox E2E`, `Process Token Information`?**
  _High betweenness centrality (0.300) - this node is a cross-community bridge._
- **What connects `ActiveChanges`, `ActiveOptimizations`, `ActiveRailAc` to the rest of the system?**
  _738 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.00516795865633075 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `Async() .Toggle .Refresh` to `Core UI Internationalization & Text Catalog`, `Privacy, Telemetry & AI Hardening Catalog`, `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `Integration Suite: Tweak Sandbox E2E`, `.Reg .Set Power`, `Scandetailkind Appxinstalled Appxinstalledps`, `Test Composite With()`, `View() Card() Switch`, `Accent Hover Secondary`, `View() Color Scrollspec`, `Integration Suite: Tweak Sandbox E2E`, `Multi-Language Tweak Localization Tables`, `Integration Suite: Tweak Sandbox E2E`, `Appx Package() Prefixes`, `Integration Suite: Tweak Sandbox E2E`, `Task Scheduler Inspection Subsystem`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `Hardware Diagnostics & GPU Silicon Tuning`, `Dynamicdwordformula Logicalcorecount Totalraminkb`, `Telemetry Full Blocked`, `Project History & Release Version Ledger`, `Hardware Diagnostics & GPU Silicon Tuning`, `State Ledger & Restore Point Engine`, `Integration Suite: Tweak Sandbox E2E`, `Hardware Diagnostics & GPU Silicon Tuning`, `Error.Rs Winblasterror Appx`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `Themed Interactive Scrollbar Widget`, `.Detect Core Parking()`, `Hardware Diagnostics & GPU Silicon Tuning`, `Startup Entries() .List`, `Startup Tasks & Auto-Run Explorer`, `Project History & Release Version Ledger`, `Integration Suite: Ui Responsive E2E`?**
  _High betweenness centrality (0.188) - this node is a cross-community bridge._
- **Should `Core UI Internationalization & Text Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.05528123658222413 - nodes in this community are weakly interconnected._
- **Why does `SandboxEnv` connect `Hardware Diagnostics & GPU Silicon Tuning` to `.Toggle .Set .Apply`, `Disk Cleaning & Temporary File Purger`, `.Reg .Set Power`, `Scandetailkind Appxinstalled Appxinstalledps`, `.Set Enabled() .Remove`, `View() Color Scrollspec`, `Multi-Language Tweak Localization Tables`, `Integration Suite: Tweak Sandbox E2E`, `System Architecture Changelog`, `.Reg State() Vendor()`, `Win32 Registry Scanner & Mutator`, `Rollback Panic (`Src/Backup/State`, `Integration Suite: Tweak Sandbox E2E`, `Integration Suite: Tweak Sandbox E2E`, `Disabled() Delete .Set`, `AppX Package & Modern App Scanner`, `.Create Restore Point()`, `WinBlast0 Architectural Specifications`, `Hardware Diagnostics & GPU Silicon Tuning`, `Telemetry Full Blocked`, `Task Scheduler Inspection Subsystem`, `Windows Unattend Deployment Generator`, `Optional Dism Feature()`, `Hardware Diagnostics & GPU Silicon Tuning`, `State Ledger & Restore Point Engine`, `Project Documentation, Architecture & Licensing`, `Startup Tasks & Auto-Run Explorer`, `Themed Interactive Scrollbar Widget`, `Multi-Language Tweak Localization Tables`, `.Detect Core Parking()`, `Hardware Diagnostics & GPU Silicon Tuning`, `Startup Entries() .List`, `Startup Tasks & Auto-Run Explorer`, `Diff With View`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._
- **Should `Privacy, Telemetry & AI Hardening Catalog` be split into smaller, more focused modules?**
  _Cohesion score 0.021505376344086023 - nodes in this community are weakly interconnected._