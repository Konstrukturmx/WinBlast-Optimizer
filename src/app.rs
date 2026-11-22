//! Top-level application state machine for WinBlast0 under the Rune framework.
//!
//! Manages navigation between Easy and Advanced modes, orchestrates background
//! workers via injected [`SystemBackend`], tracks live mutations in SQLite WAL
//! [`StateLedger`], and maintains reactive state for responsive Rune views.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use crate::backup::profile_io::{export_profile, import_profile};
use crate::backup::restore_point::RestorePoint;
use crate::backup::state_ledger::{LedgerEntry, StateLedger, SystemBaseline};
use crate::core::backend::{RealBackend, SystemBackend};
use crate::core::i18n::{I18n, Language, TextKey};
use crate::core::os_detect::WindowsInfo;
use crate::core::risk::{
    load_rules_with_fallback, FeatureInstallState, OptionalFeatureKind, RiskLevel, TweakAction,
    TweakDefinition,
};
use crate::core::status_i18n::{self, PresetKey, StatusMessageSpec};
use crate::hardware::cpu::CpuInfo;
use crate::hardware::gpu::{DetectedGpu, GpuVendor};
use crate::hardware::latency::LatencySnapshot;
use crate::hardware::monitor::SystemMetrics;
use crate::hardware::net_latency::LatencyBenchmarkSnapshot;
use crate::hardware::parking::CoreParkingState;
use crate::hardware::power::{HibernationStatus, PowerSchemeInfo};
use crate::hardware::sys_inventory::{PlatformSecurityInfo, RamDimmInfo, StorageVolumeInfo};
use crate::scanner::telemetry_scanner::{
    full_telemetry_audit_with, TelemetryAuditResult, KNOWN_TELEMETRY_SERVICES, TELEMETRY_HOSTS,
};
use crate::scanner::{
    scan_all_tweaks_parallel_with, scan_single_tweak_with, ScanResult, TweakState,
};
use crate::tools::autounattend::{generate_autounattend_xml, UnattendOptions};
use crate::tools::cleaner::{CleanExecutionReport, CleanTargetKind, CleanTargetScan};
use crate::tools::features_apps::{InstalledAppxPackage, WindowsOptionalFeature};
use crate::tools::startup::{LiveScheduledTaskInfo, LiveServiceInfo, StartupEntry};
use crate::core::theme::{ColorScheme, ThemeMode, ThemePalette};

/// Thread-safe UI wake-up hook handed to background workers (the Slint bridge wires this to
/// `slint::invoke_from_event_loop`; headless/test callers pass a no-op).
pub type Waker = Arc<dyn Fn() + Send + Sync>;

/// Toolkit-neutral single-line text buffer (replaces the Rune `TextEdit` widget state).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextEdit {
    pub text: String,
}

/// Pre-folded Advanced-Matrix query (see `AppState::tweak_matches_query`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MatrixQuery {
    pub folded: String,
    pub risk: Option<RiskLevel>,
}

impl MatrixQuery {
    pub fn new(raw: &str) -> Self {
        let raw = raw.trim();
        let risk = match raw.to_ascii_lowercase().as_str() {
            "safe" => Some(RiskLevel::Safe),
            "low" => Some(RiskLevel::Low),
            "moderate" | "caution" => Some(RiskLevel::Moderate),
            "high" => Some(RiskLevel::High),
            "extreme" => Some(RiskLevel::Extreme),
            _ => None,
        };
        let folded = if raw.is_empty() { String::new() } else { I18n::fold_search_str(raw) };
        Self { folded, risk }
    }
}

/// High-level application mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Easy,
    Advanced,
}

/// Active view tab in the navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    EasyDashboard,
    AdvancedMatrix,
    GpuTuning,
    HardwareView,
    PrivacyShield,
    BackupCenter,
    LatencyTester,
    CoreParking,
    CleanAndApps,
    StartupAndTasks,
    GoalPlanner,
    // v1.1.0 New Feature Views
    DriverBios,
    PerformanceTests,
    FaultDiagnostics,
    ThermalMonitor,
    DiskHealth,
    GamingStats,
    FpsBenchmark,
    NetworkDiag,
    GameProfiles,
    GameOverlay,
    GameSuitability,
    WindowsRepair,
    SupportAbout,
    GamePrerequisites,
    AutomationCenter,
}

impl ActiveView {
    pub const ALL_VIEWS: [ActiveView; 26] = [
        ActiveView::EasyDashboard,
        ActiveView::HardwareView,
        ActiveView::DriverBios,
        ActiveView::PerformanceTests,
        ActiveView::FaultDiagnostics,
        ActiveView::LatencyTester,
        ActiveView::CoreParking,
        ActiveView::ThermalMonitor,
        ActiveView::DiskHealth,
        ActiveView::GamingStats,
        ActiveView::FpsBenchmark,
        ActiveView::NetworkDiag,
        ActiveView::AdvancedMatrix,
        ActiveView::GoalPlanner,
        ActiveView::GameProfiles,
        ActiveView::GameOverlay,
        ActiveView::GameSuitability,
        ActiveView::GamePrerequisites,
        ActiveView::AutomationCenter,
        ActiveView::CleanAndApps,
        ActiveView::StartupAndTasks,
        ActiveView::WindowsRepair,
        ActiveView::BackupCenter,
        ActiveView::GpuTuning,
        ActiveView::PrivacyShield,
        ActiveView::SupportAbout,
    ];

    pub fn next(self) -> Self {
        let idx = Self::ALL_VIEWS.iter().position(|&v| v == self).unwrap_or(0);
        Self::ALL_VIEWS[(idx + 1) % Self::ALL_VIEWS.len()]
    }

    pub fn prev(self) -> Self {
        let idx = Self::ALL_VIEWS.iter().position(|&v| v == self).unwrap_or(0);
        if idx == 0 {
            Self::ALL_VIEWS[Self::ALL_VIEWS.len() - 1]
        } else {
            Self::ALL_VIEWS[idx - 1]
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ActiveView::EasyDashboard => "dashboard",
            ActiveView::HardwareView => "hardware",
            ActiveView::DriverBios => "driver_bios",
            ActiveView::PerformanceTests => "performance_tests",
            ActiveView::FaultDiagnostics => "fault_diagnostics",
            ActiveView::LatencyTester => "latency",
            ActiveView::CoreParking => "cpu_parking",
            ActiveView::ThermalMonitor => "thermal",
            ActiveView::DiskHealth => "disk_health",
            ActiveView::GamingStats => "gaming_stats",
            ActiveView::FpsBenchmark => "fps_benchmark",
            ActiveView::NetworkDiag => "network_diag",
            ActiveView::AdvancedMatrix => "matrix",
            ActiveView::GoalPlanner => "goal_planner",
            ActiveView::GameProfiles => "game_profiles",
            ActiveView::GameOverlay => "game_overlay",
            ActiveView::GameSuitability => "game_suitability",
            ActiveView::GamePrerequisites => "game_prereqs",
            ActiveView::AutomationCenter => "automation",
            ActiveView::CleanAndApps => "clean_apps",
            ActiveView::StartupAndTasks => "startup_tasks",
            ActiveView::WindowsRepair => "windows_repair",
            ActiveView::BackupCenter => "backup",
            ActiveView::GpuTuning => "gpu",
            ActiveView::PrivacyShield => "privacy",
            ActiveView::SupportAbout => "support_about",
        }
    }
}

/// Sub-tabs inside the Clean & Apps view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanAppsSubTab {
    DiskCleaner,
    AppxFeatures,
    WinGetInstaller,
}

/// Sub-tabs inside the Startup, Tasks & Services view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupTasksSubTab {
    StartupApps,
    ScheduledTasks,
    Services,
}

/// Sub-tabs inside the unified Gaming & Performance Hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GamingSubTab {
    #[default]
    LiveMonitor,
    FpsBenchmark,
    GameProfiles,
    GameOverlay,
    GameSuitability,
}

/// Sub-tabs inside the unified Hardware & Sensors Hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HardwareSubTab {
    #[default]
    Specs,
    CpuParking,
    Thermals,
    Disks,
}

/// Sub-tabs inside the unified Drivers & Runtimes Hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DriversSubTab {
    #[default]
    Drivers,
    Prerequisites,
}

/// Sub-tabs inside the unified Repair & Diagnostics Hub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepairSubTab {
    #[default]
    WindowsRepair,
    FaultDiagnostics,
}

/// Sorting mode for Advanced Matrix view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MatrixSortMode {
    #[default]
    Default,
    NameAsc,
    NameDesc,
    Category,
    Risk,
    Status,
}

/// Sorting mode for Clean & Apps view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CleanSortMode {
    #[default]
    SizeDesc,
    SizeAsc,
    NameAsc,
}

/// Active status filter for Startup & Tasks view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartupStatusFilter {
    #[default]
    All,
    ActiveOnly,
    InactiveOnly,
}

/// Sorting mode for Startup & Tasks view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StartupSortMode {
    #[default]
    NameAsc,
    NameDesc,
    StatusActiveFirst,
    StatusInactiveFirst,
    Source,
}

/// Active floating dropdown menu in the top bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveDropdown {
    #[default]
    None,
    Language,
    ThemeStyle,
    ThemeMode,
    ColorScheme,
}

/// Sorting mode for Drivers view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DriverSortMode {
    #[default]
    UpdatesFirst,
    NameAsc,
    Category,
}

/// Sorting mode for Game Prerequisites view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrereqsSortMode {
    #[default]
    MissingFirst,
    NameAsc,
    Category,
}

/// Sorting mode for Privacy Shield view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrivacySortMode {
    #[default]
    UnprotectedFirst,
    NameAsc,
    Category,
}

/// GitHub self-updater status.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    Available(crate::core::updater::ReleaseInfo),
    UpToDate,
    Downloading { progress_pct: f32 },
    ReadyToRestart { new_binary_path: std::path::PathBuf },
    Failed(String),
}

/// Execution mode determining whether tweak switch clicks apply immediately or stage for batch commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineMode {
    /// Instant execution: toggling a switch immediately applies or reverts the tweak.
    Instant,
    /// Normal execution: toggling a switch stages the change until the user clicks "Apply Selected".
    Normal,
}

/// Staged state transition for a tweak in Normal mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedTransition {
    StageApply,
    StageRevert,
}

/// One-click optimization presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetKind {
    Gaming,
    ProAudio,
    LaptopBattery,
}

/// Calculated before and after optimization impact metrics.
#[derive(Debug, Clone)]
pub struct BeforeAfterImpact {
    pub baseline_ram_mb: u64,
    pub current_ram_mb: u64,
    pub ram_saved_mb: i64,
    pub baseline_processes: usize,
    pub current_processes: usize,
    pub processes_saved: i64,
    pub baseline_timer_ns: u64,
    pub current_timer_ns: u64,
    pub baseline_optimized_count: usize,
    pub current_optimized_count: usize,
    pub tweaks_gained: usize,
}

/// Post-rollback platform snapshot gathered off the UI thread.
#[derive(Debug, Clone)]
pub struct RollbackRefreshData {
    pub gpus: Vec<DetectedGpu>,
    pub hags_enabled: bool,
    pub mpo_disabled: bool,
    pub core_parking: CoreParkingState,
    pub power_schemes: Vec<PowerSchemeInfo>,
    pub hibernation_status: HibernationStatus,
    pub needs_tweak_rescan: bool,
    pub needs_telemetry_audit: bool,
    pub needs_startup_refresh: bool,
    pub needs_features_refresh: bool,
}

/// Power action category for scoped concurrency guards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerActionKind {
    Scheme,
    Hibernation,
    UsbPower,
}

/// Typed background worker messages communicated back to the UI thread.
#[derive(Debug)]
pub enum WorkerEvent {
    ScanCompleted {
        epoch: u64,
        results: Vec<ScanResult>,
    },
    ScanFailed {
        epoch: u64,
        error: String,
    },
    SingleTweakUpdated {
        epoch: u64,
        tweak_id: String,
        result: ScanResult,
    },
    SingleTweakFailed {
        epoch: u64,
        tweak_id: String,
        error: String,
    },
    ApplyCompleted(Result<usize, String>),
    RestorePointsLoaded(Vec<RestorePoint>),
    RestorePointCreated(Result<(), String>),
    RollbackCompleted(Result<RollbackRefreshData, String>),
    GpuMsiUpdated {
        pnp_id: String,
        result: Result<(), String>,
        gpus: Option<Vec<DetectedGpu>>,
    },
    LatencyUpdated(LatencySnapshot),
    TelemetryAuditCompleted(TelemetryAuditResult),
    MetricsSnapshot(SystemMetrics),
    LatencyBenchmarkCompleted(LatencyBenchmarkSnapshot),
    LatencyBenchmarkFailed(String),
    CoreParkingUpdated(CoreParkingState),
    CoreParkingActionCompleted {
        state: Option<CoreParkingState>,
        msg: Result<String, String>,
    },
    HagsUpdated {
        enable: bool,
        result: Result<(), String>,
    },
    MpoUpdated {
        disable: bool,
        result: Result<(), String>,
    },
    TelemetryHostsBlocked(Result<usize, String>),
    TelemetryServicesNeutralized(Result<usize, String>),
    CleanTargetsScanned(Vec<CleanTargetScan>),
    CleanScanFailed(String),
    CleanExecuted(Result<CleanExecutionReport, String>),
    AppxAndFeaturesLoaded {
        appx: Vec<InstalledAppxPackage>,
        features: Vec<WindowsOptionalFeature>,
        kfm_warn: Option<String>,
    },
    AppxScanFailed(String),
    AppxBatchRemoved(Result<usize, String>),
    FeatureToggled {
        name: String,
        result: Result<FeatureInstallState, String>,
    },
    OneDriveEdgeResult(Result<String, String>),
    WinGetInstallResult(Result<String, String>),
    StartupAndTasksLoaded {
        startup: Vec<StartupEntry>,
        tasks: Vec<LiveScheduledTaskInfo>,
        services: Vec<LiveServiceInfo>,
    },
    StartupScanFailed(String),
    StartupEntryToggled(Result<String, String>),
    ScheduledTaskRunResult(Result<String, String>),
    ServiceLiveUpdated(Result<String, String>),
    PlatformInventoryLoaded {
        dimms: Vec<RamDimmInfo>,
        volumes: Vec<StorageVolumeInfo>,
        sec: PlatformSecurityInfo,
        schemes: Vec<PowerSchemeInfo>,
        hib: HibernationStatus,
    },
    PowerActionCompleted {
        kind: PowerActionKind,
        schemes: Vec<PowerSchemeInfo>,
        hib: HibernationStatus,
        msg: Result<String, String>,
    },
    ExplorerRestarted(Result<String, String>),
    BaselineReset(Result<(Option<SystemBaseline>, Option<SystemBaseline>), String>),
    StatusUpdate(String),
    PerfBenchmarkCompleted(crate::tools::performance_tests::BenchmarkScore),
    UpdateCheckCompleted(Result<Option<crate::core::updater::ReleaseInfo>, String>),
    UpdateDownloadCompleted(Result<std::path::PathBuf, String>),
    ThermalSnapshot(crate::hardware::thermal::SystemThermalSnapshot),
    BiosInfoLoaded(crate::hardware::bios::BiosSystemInfo),
    DiskReportLoaded(crate::hardware::disks::DiskSubsystemReport),
    DriverScanCompleted(Vec<crate::hardware::driver_update::DriverUpdateItem>),
    AutoTuneCompleted(Result<crate::tools::automation::AutoTuneResult, String>),
    StartupBatchToggled(Result<(usize, bool), String>),
    TasksBatchToggled(Result<(usize, bool), String>),
}

struct WorkerDropGuard<P: FnOnce(String) -> WorkerEvent> {
    tx: crossbeam_channel::Sender<WorkerEvent>,
    waker: Option<Waker>,
    completed: bool,
    panic_msg: Option<String>,
    on_panic_event: Option<P>,
}

impl<P: FnOnce(String) -> WorkerEvent> Drop for WorkerDropGuard<P> {
    fn drop(&mut self) {
        if !self.completed {
            if let Some(mk_event) = self.on_panic_event.take() {
                let msg = self
                    .panic_msg
                    .take()
                    .unwrap_or_else(|| "Worker thread panicked unexpectedly".to_string());
                let _ = self.tx.send(mk_event(msg));
            }
        }
        if let Some(ref w) = self.waker {
            w();
        }
    }
}

fn panic_payload_to_string(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown worker panic".to_string()
    }
}

/// Spawn a panic-safe background worker thread with RAII `Drop` notification so
/// in-flight flags can NEVER become permanently stuck on unexpected panics.
fn spawn_guarded_worker<F, P>(
    tx: crossbeam_channel::Sender<WorkerEvent>,
    waker: Option<Waker>,
    on_panic_event: P,
    body: F,
) where
    F: FnOnce(&crossbeam_channel::Sender<WorkerEvent>) + Send + 'static,
    P: FnOnce(String) -> WorkerEvent + Send + 'static,
{
    std::thread::spawn(move || {
        let mut guard = WorkerDropGuard {
            tx,
            waker,
            completed: false,
            panic_msg: None,
            on_panic_event: Some(on_panic_event),
        };
        let tx_ref = guard.tx.clone();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            body(&tx_ref);
        })) {
            Ok(()) => {
                guard.completed = true;
            }
            Err(payload) => {
                guard.panic_msg = Some(panic_payload_to_string(payload));
            }
        }
    });
}

/// Top-level application state.
pub struct AppState {
    pub backend: Arc<dyn SystemBackend>,
    pub is_elevated: bool,
    pub mode: AppMode,
    pub engine_mode: EngineMode,
    pub active_view: ActiveView,
    pub clean_sub_tab: CleanAppsSubTab,
    pub startup_sub_tab: StartupTasksSubTab,
    pub sidebar_open: bool,
    pub language: Language,
    pub theme_mode: ThemeMode,
    pub color_scheme: ColorScheme,
    pub theme_palette: ThemePalette,
    pub core_parking: CoreParkingState,
    pub latest_latency_bench: Option<LatencyBenchmarkSnapshot>,
    pub latency_before: Option<LatencyBenchmarkSnapshot>,
    pub latency_after: Option<LatencyBenchmarkSnapshot>,
    pub is_benchmarking_latency: bool,
    pub tweaks: Arc<Vec<TweakDefinition>>,
    pub scan_results: Vec<ScanResult>,
    pub scan_index: HashMap<String, usize>,
    pub selected_tweaks: HashSet<String>,
    pub staged_transitions: HashMap<String, StagedTransition>,
    pub in_flight_tweaks: HashSet<String>,
    pub pending_actions: Vec<TweakAction>,
    /// Pending mutations paired with direction flag (`true` = revert to default, `false` = apply optimization).
    pub pending_items: Vec<(TweakAction, bool)>,
    pub inspected_tweak: Option<(String, Vec<TweakAction>)>,
    pub search_query: String,
    pub search_edit: TextEdit,
    last_synced_search: String,
    pub selected_category: Option<String>,
    pub show_diff_preview: bool,
    pub os_info: Option<WindowsInfo>,
    pub cpu_info: Option<CpuInfo>,
    pub gpus: Vec<DetectedGpu>,
    pub latency: Option<LatencySnapshot>,
    pub hags_enabled: bool,
    pub mpo_disabled: bool,
    pub telemetry_audit: Option<TelemetryAuditResult>,
    pub restore_points: Vec<RestorePoint>,
    pub ledger_history: Vec<LedgerEntry>,
    pub ledger_path: PathBuf,
    pub profile_path: PathBuf,
    pub active_goal: crate::graph::astar::OptimizationGoal,
    pub planned_path: Option<crate::graph::astar::OptimizationPlan>,
    pub raven_resolutions: Vec<crate::graph::raven::ConflictResolution>,
    pub scheduled_order: Option<crate::graph::dijkstra::ExecutionOrder>,
    pub is_scanning: bool,
    scan_worker_inflight: bool,
    mutated_during_scan: HashSet<String>,
    pub is_applying: bool,
    pub is_rolling_back: bool,
    pub hags_inflight: bool,
    pub mpo_inflight: bool,
    pub gpu_msi_inflight: HashSet<String>,
    pub core_parking_inflight: bool,
    pub power_scheme_inflight: bool,
    pub hibernation_inflight: bool,
    pub usb_power_inflight: bool,
    pub telemetry_hosts_inflight: bool,
    pub telemetry_services_inflight: bool,
    pub restore_point_inflight: bool,
    pub baseline_reset_inflight: bool,
    pub pending_post_baseline: bool,
    pub status_spec: StatusMessageSpec,
    pub status_message: String,
    pub current_epoch: u64,
    pub show_changelog: bool,
    pub hidden_tweaks: HashSet<String>,
    pub progress: f32,
    pub progress_message: Option<String>,

    // v0.7.0 Clean & Apps State
    pub clean_targets: Vec<CleanTargetScan>,
    pub last_clean_report: Option<CleanExecutionReport>,
    pub installed_appx: Vec<InstalledAppxPackage>,
    pub selected_appx: HashSet<String>,
    pub appx_filter_edit: TextEdit,
    pub winget_filter_edit: TextEdit,
    pub optional_features: Vec<WindowsOptionalFeature>,
    pub selected_winget: HashSet<String>,
    pub onedrive_kfm_warning: Option<String>,
    pub clean_apps_busy: bool,
    clean_apps_inflight: usize,
    clean_scan_inflight: bool,
    appx_scan_inflight: bool,
    clean_apps_mutating: bool,

    // v0.7.0 Startup, Tasks & Services State
    pub startup_entries: Vec<StartupEntry>,
    pub selected_startup_entries: HashSet<String>,
    pub scheduled_tasks: Vec<LiveScheduledTaskInfo>,
    pub selected_scheduled_tasks: HashSet<String>,
    pub live_services: Vec<LiveServiceInfo>,
    pub hide_microsoft_tasks: bool,
    pub startup_filter_edit: TextEdit,
    pub startup_tasks_busy: bool,
    startup_tasks_inflight: usize,
    startup_scan_inflight: bool,
    startup_mutating: bool,

    // v0.7.0 Platform Hardware, Power & Autounattend State
    pub ram_dimms: Vec<RamDimmInfo>,
    pub storage_volumes: Vec<StorageVolumeInfo>,
    pub platform_security: Option<PlatformSecurityInfo>,
    pub power_schemes: Vec<PowerSchemeInfo>,
    pub hibernation_status: HibernationStatus,
    pub unattend_opts: UnattendOptions,
    pub unattend_last_export: Option<String>,

    // Real-time Task Manager style monitoring (v1.4.0 adaptive interval)
    pub realtime_metrics: Option<SystemMetrics>,
    pub cpu_sparkline: Vec<f32>,
    pub ram_sparkline: Vec<f32>,
    pub initial_baseline: Option<SystemBaseline>,
    pub latest_baseline: Option<SystemBaseline>,
    pub telemetry_interval_ms: std::sync::Arc<std::sync::atomic::AtomicU64>,
    pub window_icon_attached: bool,

    // Latency & GPU power optimization (Milestone 2 dirty-rect throttling)
    pub last_interaction_secs: f64,
    pub last_render_secs: f64,
    pub last_cpu_metric: f32,
    pub last_ram_metric: f32,

    // Background worker communication channel
    pub rx_events: crossbeam_channel::Receiver<WorkerEvent>,
    pub tx_events: crossbeam_channel::Sender<WorkerEvent>,
    pub waker: Option<Waker>,

    // v1.1.0 Advanced Themes & Diagnostics Subsystems
    pub theme_style: crate::core::theme::UiThemeStyle,
    pub bios_info: Option<crate::hardware::bios::BiosSystemInfo>,
    pub is_bios_scanning: bool,
    pub fault_report: Option<crate::tools::diagnostics::FaultDiagnosticsReport>,
    pub thermal_snapshot: Option<crate::hardware::thermal::SystemThermalSnapshot>,
    pub is_thermal_scanning: bool,
    pub disk_report: Option<crate::hardware::disks::DiskSubsystemReport>,
    pub is_disk_scanning: bool,
    pub perf_score: Option<crate::tools::performance_tests::BenchmarkScore>,
    pub is_benchmarking_perf: bool,
    pub game_telemetry: crate::tools::gaming_stats::LiveGameTelemetry,
    pub network_diag: Option<crate::tools::network_diag::NetworkDiagnostics>,
    pub game_profiles: Vec<crate::tools::game_profiles::GameProfile>,
    pub game_suitability: crate::tools::game_overlay::GameSuitabilityReport,
    pub game_overlay: crate::tools::game_overlay::GameOverlaySettings,
    pub repair_tasks: Vec<crate::tools::windows_repair::RepairTask>,
    pub repair_log: Vec<String>,
    pub driver_updates: Vec<crate::hardware::driver_update::DriverUpdateItem>,
    pub is_driver_scanning: bool,
    pub driver_install_in_flight: Option<String>,

    // v1.4.0 Diagnostics, Game Prerequisites & Automation Subsystems
    pub driver_search_edit: TextEdit,
    pub driver_category_filter: Option<crate::hardware::driver_update::DriverCategory>,
    pub game_prereqs: Vec<crate::tools::game_prereqs::GamePrerequisiteItem>,
    pub is_prereqs_scanning: bool,
    pub prereq_install_in_flight: Option<String>,
    pub prereq_filter_edit: TextEdit,
    pub auto_tune_plan: Option<crate::tools::automation::AutoTunePlan>,
    pub auto_tune_result: Option<crate::tools::automation::AutoTuneResult>,
    pub selected_auto_tune_profile: crate::tools::automation::AutoTuneProfile,
    pub system_health: Option<crate::tools::automation::SystemHealthAssessment>,
    pub system_health_assessment: Option<crate::tools::automation::SystemHealthAssessment>,
    pub maintenance_tasks: Vec<crate::tools::automation::MaintenanceTaskStatus>,
    pub last_memory_trim_result: Option<crate::tools::automation::MemoryTrimResult>,
    pub is_running_automation: bool,

    // v1.5.0 Self-Updater, Unified Hubs, and Sorting
    pub update_status: UpdateStatus,
    pub matrix_sort_mode: MatrixSortMode,
    pub clean_sort_mode: CleanSortMode,
    pub startup_sort_mode: StartupSortMode,
    pub startup_status_filter: StartupStatusFilter,
    pub active_dropdown: ActiveDropdown,
    pub driver_sort_mode: DriverSortMode,
    pub prereqs_sort_mode: PrereqsSortMode,
    pub privacy_sort_mode: PrivacySortMode,
    pub gaming_sub_tab: GamingSubTab,
    pub hardware_sub_tab: HardwareSubTab,
    pub drivers_sub_tab: DriversSubTab,
    pub repair_sub_tab: RepairSubTab,
    pub current_time_secs: f64,
    pub dismiss_progress_hud: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Initialize state against the live Windows OS.
    pub fn new() -> Self {
        Self::new_with_backend(Arc::new(RealBackend))
    }

    /// Initialize state using an explicit backend (`RealBackend` or `SandboxEnv`).
    pub fn new_with_backend(backend: Arc<dyn SystemBackend>) -> Self {
        let (tx_events, rx_events) = crossbeam_channel::unbounded();

        let rules_dir = std::path::Path::new("assets/rules");
        let tweaks = load_rules_with_fallback(Some(rules_dir));
        tracing::info!(
            "Loaded {} tweak definitions into state machine",
            tweaks.len()
        );

        let is_elevated = backend.is_elevated();
        let os_info = backend.detect_os().ok();
        let cpu_info = Some(backend.detect_cpu());
        let gpus = backend.detect_gpus();
        let hags_enabled = backend.is_hags_enabled();
        let mpo_disabled = backend.is_mpo_disabled();

        let ledger_path = backend.default_ledger_path();
        let profile_path = backend.default_profile_path();

        let ledger = StateLedger::new(&ledger_path).ok();
        let ledger_history = ledger
            .as_ref()
            .and_then(|l| l.get_history().ok())
            .unwrap_or_default();

        // Capture lightweight initial metrics without blocking window creation
        let mut initial_sampler = backend.create_metrics_sampler(35);
        let initial_metrics = initial_sampler.sample();

        let initial_baseline = ledger
            .as_ref()
            .and_then(|l| l.get_initial_baseline().ok())
            .flatten();
        let latest_baseline = ledger
            .as_ref()
            .and_then(|l| l.get_latest_baseline().ok())
            .flatten();

        let cpu_sparkline = vec![initial_metrics.cpu_usage];
        let ram_sparkline = vec![initial_metrics.ram_percentage];

        let language = Language::default();
        let theme_mode = ThemeMode::default();
        let color_scheme = ColorScheme::default();
        let theme_style = crate::core::theme::UiThemeStyle::default();
        let theme_palette = ThemePalette::new(theme_mode, color_scheme, theme_style);
        let core_parking = backend.detect_core_parking();

        Self {
            backend,
            is_elevated,
            mode: AppMode::Easy,
            engine_mode: EngineMode::Normal,
            active_view: ActiveView::EasyDashboard,
            clean_sub_tab: CleanAppsSubTab::DiskCleaner,
            startup_sub_tab: StartupTasksSubTab::StartupApps,
            sidebar_open: false,
            language,
            theme_mode,
            color_scheme,
            theme_palette,
            core_parking,
            latest_latency_bench: None,
            latency_before: None,
            latency_after: None,
            is_benchmarking_latency: false,
            tweaks: Arc::new(tweaks),
            scan_results: Vec::new(),
            scan_index: HashMap::new(),
            selected_tweaks: HashSet::new(),
            staged_transitions: HashMap::new(),
            in_flight_tweaks: HashSet::new(),
            pending_actions: Vec::new(),
            pending_items: Vec::new(),
            inspected_tweak: None,
            search_query: String::new(),
            search_edit: TextEdit::default(),
            last_synced_search: String::new(),
            selected_category: None,
            show_diff_preview: false,
            os_info,
            cpu_info,
            gpus,
            latency: None,
            hags_enabled,
            mpo_disabled,
            telemetry_audit: None,
            restore_points: Vec::new(),
            ledger_history,
            ledger_path,
            profile_path,
            active_goal: crate::graph::astar::OptimizationGoal::ultra_gaming(),
            planned_path: None,
            raven_resolutions: Vec::new(),
            scheduled_order: None,
            is_scanning: true,
            scan_worker_inflight: false,
            mutated_during_scan: HashSet::new(),
            is_applying: false,
            is_rolling_back: false,
            hags_inflight: false,
            mpo_inflight: false,
            gpu_msi_inflight: HashSet::new(),
            core_parking_inflight: false,
            power_scheme_inflight: false,
            hibernation_inflight: false,
            usb_power_inflight: false,
            telemetry_hosts_inflight: false,
            telemetry_services_inflight: false,
            restore_point_inflight: false,
            baseline_reset_inflight: false,
            pending_post_baseline: false,
            status_spec: StatusMessageSpec::ScanningStart,
            status_message: status_i18n::localize_status(
                language,
                &StatusMessageSpec::ScanningStart,
            ),
            current_epoch: 0,
            show_changelog: false,
            hidden_tweaks: HashSet::new(),
            progress: 0.1,
            progress_message: Some(status_i18n::localize_status(
                language,
                &StatusMessageSpec::ScanningStart,
            )),
            clean_targets: Vec::new(),
            last_clean_report: None,
            installed_appx: Vec::new(),
            selected_appx: HashSet::new(),
            appx_filter_edit: TextEdit::default(),
            winget_filter_edit: TextEdit::default(),
            optional_features: Vec::new(),
            selected_winget: HashSet::new(),
            onedrive_kfm_warning: None,
            clean_apps_busy: false,
            clean_apps_inflight: 0,
            clean_scan_inflight: false,
            appx_scan_inflight: false,
            clean_apps_mutating: false,
            startup_entries: Vec::new(),
            selected_startup_entries: HashSet::new(),
            scheduled_tasks: Vec::new(),
            selected_scheduled_tasks: HashSet::new(),
            live_services: Vec::new(),
            hide_microsoft_tasks: true,
            startup_filter_edit: TextEdit::default(),
            startup_tasks_busy: false,
            startup_tasks_inflight: 0,
            startup_scan_inflight: false,
            startup_mutating: false,
            ram_dimms: Vec::new(),
            storage_volumes: Vec::new(),
            platform_security: None,
            power_schemes: Vec::new(),
            hibernation_status: HibernationStatus::default(),
            unattend_opts: UnattendOptions::default(),
            unattend_last_export: None,
            realtime_metrics: Some(initial_metrics),
            cpu_sparkline,
            ram_sparkline,
            initial_baseline,
            latest_baseline,
            telemetry_interval_ms: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(2000)),
            window_icon_attached: false,
            last_interaction_secs: 0.0,
            last_render_secs: 0.0,
            last_cpu_metric: 0.0,
            last_ram_metric: 0.0,
            rx_events,
            tx_events,
            waker: None,

            // v1.1.0 Subsystems
            theme_style,
            bios_info: None,
            is_bios_scanning: false,
            fault_report: Some(crate::tools::diagnostics::FaultDiagnosticsReport::scan()),
            thermal_snapshot: None,
            is_thermal_scanning: false,
            disk_report: None,
            is_disk_scanning: false,
            perf_score: Some(crate::tools::performance_tests::BenchmarkScore::default()),
            is_benchmarking_perf: false,
            game_telemetry: crate::tools::gaming_stats::LiveGameTelemetry::default(),
            network_diag: None,
            game_profiles: crate::tools::game_profiles::GameProfile::all(),
            game_suitability: crate::tools::game_overlay::GameSuitabilityReport::default(),
            game_overlay: crate::tools::game_overlay::GameOverlaySettings::default(),
            repair_tasks: crate::tools::windows_repair::RepairTask::all(),
            repair_log: Vec::new(),
            driver_updates: Vec::new(),
            is_driver_scanning: false,
            driver_install_in_flight: None,
            driver_search_edit: TextEdit::default(),
            driver_category_filter: None,
            game_prereqs: Vec::new(),
            is_prereqs_scanning: false,
            prereq_install_in_flight: None,
            prereq_filter_edit: TextEdit::default(),
            auto_tune_plan: None,
            auto_tune_result: None,
            selected_auto_tune_profile: crate::tools::automation::AutoTuneProfile::GamingLowLatency,
            system_health: None,
            system_health_assessment: None,
            maintenance_tasks: crate::tools::automation::query_all_maintenance_tasks(),
            last_memory_trim_result: None,
            is_running_automation: false,

            // v1.5.0 Self-Updater, Unified Hubs, and Sorting
            update_status: UpdateStatus::Idle,
            matrix_sort_mode: MatrixSortMode::Default,
            clean_sort_mode: CleanSortMode::SizeDesc,
            startup_sort_mode: StartupSortMode::NameAsc,
            startup_status_filter: StartupStatusFilter::All,
            active_dropdown: ActiveDropdown::None,
            driver_sort_mode: DriverSortMode::UpdatesFirst,
            prereqs_sort_mode: PrereqsSortMode::MissingFirst,
            privacy_sort_mode: PrivacySortMode::UnprotectedFirst,
            gaming_sub_tab: GamingSubTab::LiveMonitor,
            hardware_sub_tab: HardwareSubTab::Specs,
            drivers_sub_tab: DriversSubTab::Drivers,
            repair_sub_tab: RepairSubTab::WindowsRepair,
            current_time_secs: 0.0,
            dismiss_progress_hud: false,
        }
    }

    /// Rune framework lifecycle setup hook. Captures the window waker and starts async warmup tasks.
    pub fn setup(&mut self, waker: Waker) {
        self.waker = Some(waker.clone());
        self.start_scan();
        self.refresh_platform_inventory();
        self.refresh_restore_points();
        self.refresh_latency();
        self.run_telemetry_audit();
        self.check_for_updates();

        // Adaptive Task Manager style monitoring worker with live thermal telemetry
        let tx_monitor = self.tx_events.clone();
        let waker_monitor = waker;
        let backend = Arc::clone(&self.backend);
        let interval_holder = Arc::clone(&self.telemetry_interval_ms);
        std::thread::spawn(move || {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut monitor = backend.create_metrics_sampler(35);
                let mut thermal_counter = 0u32;
                loop {
                    let metrics = monitor.sample();
                    if tx_monitor
                        .send(WorkerEvent::MetricsSnapshot(metrics))
                        .is_err()
                    {
                        break;
                    }
                    thermal_counter += 1;
                    if thermal_counter >= 2 {
                        thermal_counter = 0;
                        let snapshot = crate::hardware::thermal::SystemThermalSnapshot::read();
                        let _ = tx_monitor.send(WorkerEvent::ThermalSnapshot(snapshot));
                    }
                    waker_monitor();
                    let sleep_ms = interval_holder
                        .load(std::sync::atomic::Ordering::Relaxed)
                        .max(250);
                    std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
                }
            }));
        });

        // Ensure native Windows title bar and taskbar inherit official WinBlast icon
        #[cfg(windows)]
        std::thread::spawn(|| {
            for _ in 0..15 {
                std::thread::sleep(std::time::Duration::from_millis(50));
                crate::core::window_icon::attach_window_icon();
            }
        });
    }

    /// Slint & headless lifecycle setup: spawns background monitoring, telemetry, inventory and updates.
    pub fn setup_slint(&mut self) {
        let waker: Waker = std::sync::Arc::new(|| {});
        self.setup(waker);
    }

    /// Slint framework lifecycle poll hook. Drains bounded worker events without requiring Rune types.
    pub fn poll_headless(&mut self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        self.current_time_secs = now;
        self.sync_search_state();
        let processed = self.drain_worker_events_bounded(64);
        processed > 0 || self.is_in_progress() || !self.rx_events.is_empty()
    }

    /// Synchronize `search_query` and `search_edit.text` so clearing either never resurrects stale text.
    pub fn sync_search_state(&mut self) {
        if self.search_edit.text != self.last_synced_search {
            self.search_query = self.search_edit.text.clone();
            self.last_synced_search = self.search_edit.text.clone();
        } else if self.search_query != self.last_synced_search {
            self.search_edit.text = self.search_query.clone();
            self.last_synced_search = self.search_query.clone();
        }
    }

    /// Programmatically update the search filter keeping `search_query` and `search_edit.text` in sync.
    pub fn set_search_query(&mut self, query: impl Into<String>) {
        let q = query.into();
        self.search_query = q.clone();
        self.search_edit.text = q.clone();
        self.last_synced_search = q;
    }

    /// Record user interaction (mouse move, click, keystroke, view navigation).
    #[inline]
    pub fn record_user_interaction(&mut self) {
        self.last_interaction_secs = self.current_time_secs;
    }


    /// Drain up to `limit` queued worker events.
    pub fn drain_worker_events_bounded(&mut self, limit: usize) -> usize {
        self.sync_search_state();
        let mut count = 0;
        while count < limit {
            let Ok(event) = self.rx_events.try_recv() else {
                break;
            };
            self.apply_worker_event(event);
            count += 1;
        }
        count
    }

    /// Synchronously drain all queued worker events (useful for deterministic headless testing).
    pub fn drain_worker_events(&mut self) -> usize {
        self.drain_worker_events_bounded(usize::MAX)
    }

    /// True if any Clean & Apps scan or action worker is still running.
    pub fn is_clean_apps_busy(&self) -> bool {
        self.clean_apps_inflight > 0
    }

    fn sync_clean_apps_busy(&mut self) {
        self.clean_apps_busy = self.clean_apps_inflight > 0;
    }

    /// True if any Startup & Tasks scan or action worker is still running.
    pub fn is_startup_tasks_busy(&self) -> bool {
        self.startup_tasks_inflight > 0
    }

    /// True if any operation is actively executing in background and progress HUD is not dismissed.
    pub fn is_in_progress(&self) -> bool {
        if self.dismiss_progress_hud {
            return false;
        }
        self.is_scanning
            || self.is_applying
            || self.is_rolling_back
            || self.clean_apps_busy
            || self.startup_tasks_busy
            || self.driver_install_in_flight.is_some()
            || self.prereq_install_in_flight.is_some()
            || self.is_bios_scanning
            || self.is_driver_scanning
            || self.is_thermal_scanning
            || self.is_disk_scanning
            || self.is_benchmarking_latency
    }

    /// Dynamic status description for the centered in-progress HUD overlay.
    pub fn in_progress_status(&self) -> String {
        if self.is_scanning {
            return "Scanning system optimization matrix...".to_string();
        }
        if self.is_applying {
            if let Some(ref msg) = self.progress_message {
                return msg.clone();
            }
            return "Applying selected system optimizations...".to_string();
        }
        if self.is_rolling_back {
            return "Rolling back optimizations to baseline...".to_string();
        }
        if self.is_bios_scanning {
            return "Probing SMBIOS motherboard tables & UEFI firmware...".to_string();
        }
        if self.is_driver_scanning {
            return "Scanning online and local device driver store...".to_string();
        }
        if self.is_thermal_scanning {
            return "Querying CPU/GPU junction temperatures & fan speeds...".to_string();
        }
        if self.is_disk_scanning {
            return "Evaluating physical storage SMART health & TRIM telemetry...".to_string();
        }
        if self.clean_apps_busy {
            return "Executing system cleanup and app management...".to_string();
        }
        if self.startup_tasks_busy {
            return "Updating startup services and scheduled tasks...".to_string();
        }
        if let Some(ref d) = self.driver_install_in_flight {
            return format!("Staging driver installation for {}...", d);
        }
        if let Some(ref p) = self.prereq_install_in_flight {
            return format!("Installing gaming prerequisite: {}...", p);
        }
        if self.is_benchmarking_latency {
            return "Benchmarking microsecond DPC and execution latency...".to_string();
        }
        "Executing background operation...".to_string()
    }

    fn sync_startup_tasks_busy(&mut self) {
        self.startup_tasks_busy = self.startup_tasks_inflight > 0;
    }

    /// Rebuild `scan_index` HashMap for O(1) per-card lookup during rendering.
    pub fn rebuild_scan_index(&mut self) {
        self.scan_index.clear();
        self.scan_index.reserve(self.scan_results.len());
        for (i, r) in self.scan_results.iter().enumerate() {
            self.scan_index.insert(r.tweak_id.clone(), i);
        }
    }

    /// O(1) lookup of a scanned tweak result by ID.
    #[inline]
    pub fn get_tweak_scan(&self, tweak_id: &str) -> Option<&ScanResult> {
        self.scan_index
            .get(tweak_id)
            .and_then(|&i| self.scan_results.get(i))
    }

    /// Process a typed worker event from a background thread.
    pub fn apply_worker_event(&mut self, event: WorkerEvent) {
        match event {
            WorkerEvent::ScanCompleted { epoch, results } => {
                if epoch == self.current_epoch {
                    self.is_scanning = false;
                    self.scan_worker_inflight = false;
                    if self.mutated_during_scan.is_empty() {
                        self.scan_results = results;
                    } else {
                        let mut merged = Vec::with_capacity(results.len());
                        for r in results {
                            if self.mutated_during_scan.contains(&r.tweak_id) {
                                if let Some(existing) = self.get_tweak_scan(&r.tweak_id) {
                                    merged.push(existing.clone());
                                    continue;
                                }
                            }
                            merged.push(r);
                        }
                        self.scan_results = merged;
                        self.mutated_during_scan.clear();
                    }
                    self.rebuild_scan_index();
                    self.progress = self.optimization_progress();
                    self.progress_message = None;
                    self.set_status_spec(StatusMessageSpec::ScanCompleted(self.scan_results.len()));

                    if self.initial_baseline.is_none() {
                        if let (Ok(l), Some(ref m)) =
                            (StateLedger::new(&self.ledger_path), &self.realtime_metrics)
                        {
                            let opt_count = self
                                .scan_results
                                .iter()
                                .filter(|r| r.current_state == TweakState::AlreadyOptimized)
                                .count();
                            if l.save_baseline(
                                m.total_ram_mb,
                                m.used_ram_mb,
                                m.process_count,
                                m.thread_count,
                                m.timer_resolution_ns,
                                opt_count,
                                "Initial Pre-Optimization",
                            )
                            .is_ok()
                            {
                                self.initial_baseline = l.get_initial_baseline().ok().flatten();
                            }
                        }
                    }

                    if self.pending_post_baseline {
                        self.pending_post_baseline = false;
                        self.record_current_baseline("Post-Optimization");
                    }
                }
            }
            WorkerEvent::ScanFailed { epoch, error } => {
                if epoch == self.current_epoch {
                    self.is_scanning = false;
                    self.scan_worker_inflight = false;
                    self.mutated_during_scan.clear();
                    self.pending_post_baseline = false;
                    self.progress_message = None;
                    self.set_status_spec(StatusMessageSpec::ScanFailed(error));
                }
            }
            WorkerEvent::SingleTweakUpdated {
                epoch,
                tweak_id,
                result,
            } => {
                self.in_flight_tweaks.remove(&tweak_id);
                let removed_sel = self.selected_tweaks.remove(&tweak_id);
                let removed_staged = self.staged_transitions.remove(&tweak_id).is_some();
                if removed_sel || removed_staged {
                    self.refresh_pending_actions();
                }
                if epoch == self.current_epoch {
                    if self.scan_worker_inflight {
                        self.mutated_during_scan.insert(tweak_id.clone());
                    }
                    if let Some(&pos) = self.scan_index.get(&tweak_id) {
                        if pos < self.scan_results.len() {
                            self.scan_results[pos] = result;
                        }
                    } else {
                        let pos = self.scan_results.len();
                        self.scan_index.insert(tweak_id.clone(), pos);
                        self.scan_results.push(result);
                    }
                    self.progress = self.optimization_progress();
                    self.refresh_ledger();
                    self.set_status_spec(StatusMessageSpec::InstantCompleted(tweak_id.clone()));
                    if !self.core_parking_inflight {
                        self.refresh_core_parking();
                    }
                }
            }
            WorkerEvent::SingleTweakFailed {
                epoch,
                tweak_id,
                error,
            } => {
                // Guaranteed removal prevents permanent switch deadlock on IO/registry errors or panics
                self.in_flight_tweaks.remove(&tweak_id);
                if epoch == self.current_epoch || self.status_message.is_empty() {
                    self.set_status_spec(StatusMessageSpec::InstantFailed {
                        tweak_id,
                        err: error,
                    });
                }
            }
            WorkerEvent::ApplyCompleted(res) => {
                self.is_applying = false;
                self.progress = self.optimization_progress();
                self.progress_message = None;
                match res {
                    Ok(count) => {
                        self.set_status_spec(StatusMessageSpec::BatchCompleted(count));
                        self.selected_tweaks.clear();
                        self.staged_transitions.clear();
                        self.refresh_pending_actions();
                        self.refresh_ledger();
                        self.pending_post_baseline = true;
                        if !self.core_parking_inflight {
                            self.refresh_core_parking();
                        }
                        self.force_scan();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::BatchFailed(e));
                    }
                }
            }
            WorkerEvent::MetricsSnapshot(metrics) => {
                // O(1) in-place rolling sparkline update without memory reallocation
                if self.cpu_sparkline.len() >= 35 {
                    self.cpu_sparkline.rotate_left(1);
                    if let Some(last) = self.cpu_sparkline.last_mut() {
                        *last = metrics.cpu_usage;
                    }
                } else {
                    self.cpu_sparkline.push(metrics.cpu_usage);
                }

                if self.ram_sparkline.len() >= 35 {
                    self.ram_sparkline.rotate_left(1);
                    if let Some(last) = self.ram_sparkline.last_mut() {
                        *last = metrics.ram_percentage;
                    }
                } else {
                    self.ram_sparkline.push(metrics.ram_percentage);
                }

                if !self.core_parking.cores.is_empty() {
                    let has_parked = !metrics.core_parked.is_empty();
                    for (i, core) in self.core_parking.cores.iter_mut().enumerate() {
                        if let Some(&usage) = metrics.core_usages.get(i) {
                            core.usage_pct = usage;
                        }
                        if self.core_parking.is_fully_unparked {
                            core.is_parked = false;
                        } else if has_parked {
                            if let Some(&parked) = metrics.core_parked.get(i) {
                                core.is_parked = parked;
                            }
                        }
                    }
                    self.core_parking.parked_count = self
                        .core_parking
                        .cores
                        .iter()
                        .filter(|c| c.is_parked)
                        .count();
                    self.core_parking.unparked_count = self
                        .core_parking
                        .cores
                        .len()
                        .saturating_sub(self.core_parking.parked_count);
                }

                self.realtime_metrics = Some(metrics);
            }
            WorkerEvent::RestorePointsLoaded(points) => {
                self.restore_points = points;
            }
            WorkerEvent::RestorePointCreated(res) => {
                self.restore_point_inflight = false;
                match res {
                    Ok(()) => {
                        self.set_status_spec(StatusMessageSpec::RestorePointCreated);
                        self.refresh_restore_points();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::RestorePointFailed(e));
                    }
                }
            }
            WorkerEvent::RollbackCompleted(res) => {
                self.is_rolling_back = false;
                match res {
                    Ok(refresh) => {
                        self.gpus = refresh.gpus;
                        self.hags_enabled = refresh.hags_enabled;
                        self.mpo_disabled = refresh.mpo_disabled;
                        self.core_parking = refresh.core_parking;
                        self.power_schemes = refresh.power_schemes;
                        self.hibernation_status = refresh.hibernation_status;
                        self.set_status_spec(StatusMessageSpec::RollbackCompleted);
                        self.refresh_ledger();
                        if !self.core_parking_inflight {
                            self.refresh_core_parking();
                        }
                        if refresh.needs_tweak_rescan {
                            self.force_scan();
                        }
                        if refresh.needs_telemetry_audit {
                            self.run_telemetry_audit();
                        }
                        if refresh.needs_startup_refresh {
                            self.scan_startup_and_tasks_async();
                        }
                        if refresh.needs_features_refresh {
                            self.scan_appx_and_features_async();
                        }
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::RollbackFailed(e));
                    }
                }
            }
            WorkerEvent::GpuMsiUpdated {
                pnp_id,
                result,
                gpus,
            } => {
                self.gpu_msi_inflight.remove(&pnp_id);
                match result {
                    Ok(()) => {
                        if let Some(gpus) = gpus {
                            self.gpus = gpus;
                        }
                        self.set_status_spec(StatusMessageSpec::GpuMsiUpdated);
                        self.refresh_ledger();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::GpuMsiFailed(e));
                    }
                }
            }
            WorkerEvent::LatencyUpdated(snap) => {
                self.latency = Some(snap);
            }
            WorkerEvent::TelemetryAuditCompleted(audit) => {
                self.telemetry_audit = Some(audit);
            }
            WorkerEvent::LatencyBenchmarkCompleted(bench) => {
                self.is_benchmarking_latency = false;
                if self.latency_before.is_none() {
                    self.latency_before = Some(bench.clone());
                } else {
                    self.latency_after = Some(bench.clone());
                }
                self.latest_latency_bench = Some(bench);
                self.set_status_spec(StatusMessageSpec::LatencyBenchmarkCompleted);
            }
            WorkerEvent::LatencyBenchmarkFailed(err) => {
                self.is_benchmarking_latency = false;
                self.set_status_spec(StatusMessageSpec::LatencyBenchmarkFailed(err));
            }
            WorkerEvent::CoreParkingUpdated(state) => {
                self.core_parking_inflight = false;
                self.core_parking = state;
                self.refresh_ledger();
            }
            WorkerEvent::CoreParkingActionCompleted { state, msg } => {
                self.core_parking_inflight = false;
                if let Some(st) = state {
                    self.core_parking = st;
                }
                match msg {
                    Ok(ok) => {
                        self.refresh_ledger();
                        let restoring = ok.contains("restor")
                            || ok.contains("Restor")
                            || ok.contains("Stock")
                            || ok.contains("stock");
                        if restoring {
                            self.set_status_spec(StatusMessageSpec::CoreParkingRestored);
                        } else {
                            self.set_status_spec(StatusMessageSpec::CoreParkingUnparked);
                        }
                        self.force_scan();
                    }
                    Err(err) => {
                        let restoring = err.contains("restor") || err.contains("Restor");
                        self.set_status_spec(StatusMessageSpec::CoreParkingFailed {
                            restoring,
                            err,
                        });
                    }
                }
            }
            WorkerEvent::HagsUpdated { enable, result } => {
                self.hags_inflight = false;
                match result {
                    Ok(()) => {
                        self.hags_enabled = enable;
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::HagsUpdated(enable));
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::HagsFailed(e));
                    }
                }
            }
            WorkerEvent::MpoUpdated { disable, result } => {
                self.mpo_inflight = false;
                match result {
                    Ok(()) => {
                        self.mpo_disabled = disable;
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::MpoUpdated(disable));
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::MpoFailed(e));
                    }
                }
            }
            WorkerEvent::TelemetryHostsBlocked(res) => {
                self.telemetry_hosts_inflight = false;
                match res {
                    Ok(count) => {
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::HostsBlocked(count));
                        self.run_telemetry_audit();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::HostsFailed(e));
                    }
                }
            }
            WorkerEvent::TelemetryServicesNeutralized(res) => {
                self.telemetry_services_inflight = false;
                match res {
                    Ok(disabled_count) => {
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::TelemetryServicesNeutralized(
                            disabled_count,
                        ));
                        self.run_telemetry_audit();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::TelemetryServicesFailed(e));
                    }
                }
            }
            WorkerEvent::CleanTargetsScanned(mut targets) => {
                self.clean_scan_inflight = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                if !self.clean_targets.is_empty() {
                    for t in &mut targets {
                        if let Some(prev) = self.clean_targets.iter().find(|p| p.kind == t.kind) {
                            t.selected = prev.selected;
                        }
                    }
                }
                self.clean_targets = targets;
                let mb = self
                    .clean_targets
                    .iter()
                    .map(|t| t.reclaimable_bytes)
                    .sum::<u64>() as f64
                    / (1024.0 * 1024.0);
                self.set_status_spec(StatusMessageSpec::DiskScanned {
                    count: self.clean_targets.len(),
                    mb,
                });
            }
            WorkerEvent::CleanScanFailed(err) => {
                self.clean_scan_inflight = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                self.set_status_spec(StatusMessageSpec::DiskScanFailed(err));
            }
            WorkerEvent::CleanExecuted(res) => {
                self.clean_apps_mutating = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                match res {
                    Ok(report) => {
                        let mb = report.total_freed_bytes as f64 / (1024.0 * 1024.0);
                        self.set_status_spec(StatusMessageSpec::DiskCleaned {
                            mb,
                            files: report.total_deleted_files as usize,
                            skipped: report.total_skipped_locked as usize,
                        });
                        self.last_clean_report = Some(report);
                        self.refresh_ledger();
                        self.scan_clean_targets_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::DiskCleanFailed(e));
                    }
                }
            }
            WorkerEvent::AppxAndFeaturesLoaded {
                appx,
                features,
                kfm_warn,
            } => {
                self.appx_scan_inflight = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                self.installed_appx = appx;
                self.optional_features = features;
                self.onedrive_kfm_warning = kfm_warn;
                self.selected_appx
                    .retain(|id| self.installed_appx.iter().any(|p| &p.family_name == id));
                self.set_status_spec(StatusMessageSpec::AppxFeatureScanned {
                    appx: self.installed_appx.len(),
                    features: self.optional_features.len(),
                });
            }
            WorkerEvent::AppxScanFailed(err) => {
                self.appx_scan_inflight = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                self.set_status_spec(StatusMessageSpec::AppxFeatureScanFailed(err));
            }
            WorkerEvent::AppxBatchRemoved(res) => {
                self.clean_apps_mutating = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                match res {
                    Ok(n) => {
                        self.selected_appx.clear();
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::AppxUninstalled(n));
                        self.scan_appx_and_features_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::AppxUninstallFailed(e));
                    }
                }
            }
            WorkerEvent::FeatureToggled { name, result } => {
                self.clean_apps_mutating = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                match result {
                    Ok(new_state) => {
                        let req_prefix = name.split_once("~~~~").map(|(p, _)| p).unwrap_or(&name);
                        if let Some(f) = self.optional_features.iter_mut().find(|f| {
                            f.name.eq_ignore_ascii_case(&name)
                                || f.name
                                    .split_once("~~~~")
                                    .map(|(p, _)| p)
                                    .unwrap_or(&f.name)
                                    .eq_ignore_ascii_case(req_prefix)
                        }) {
                            f.state = new_state;
                        }
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::FeatureUpdated {
                            name,
                            enabled: new_state == FeatureInstallState::Enabled,
                        });
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::FeatureUpdateFailed {
                            name,
                            err: e,
                        });
                    }
                }
            }
            WorkerEvent::OneDriveEdgeResult(res) => {
                self.clean_apps_mutating = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                match res {
                    Ok(msg) => {
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::ActionMessage(msg));
                        self.scan_appx_and_features_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::ActionMessage(e));
                    }
                }
            }
            WorkerEvent::WinGetInstallResult(res) => {
                self.clean_apps_mutating = false;
                self.clean_apps_inflight = self.clean_apps_inflight.saturating_sub(1);
                self.sync_clean_apps_busy();
                match res {
                    Ok(msg) => {
                        self.selected_winget.clear();
                        self.set_status_spec(StatusMessageSpec::ActionMessage(msg));
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::WinGetFailed(e));
                    }
                }
            }
            WorkerEvent::StartupAndTasksLoaded {
                startup,
                tasks,
                services,
            } => {
                self.startup_scan_inflight = false;
                self.startup_tasks_inflight = self.startup_tasks_inflight.saturating_sub(1);
                self.sync_startup_tasks_busy();
                self.startup_entries = startup;
                self.scheduled_tasks = tasks;
                self.live_services = services;
                self.set_status_spec(StatusMessageSpec::StartupScanned {
                    startup: self.startup_entries.len(),
                    tasks: self.scheduled_tasks.len(),
                    services: self.live_services.len(),
                });
            }
            WorkerEvent::StartupScanFailed(err) => {
                self.startup_scan_inflight = false;
                self.startup_tasks_inflight = self.startup_tasks_inflight.saturating_sub(1);
                self.sync_startup_tasks_busy();
                self.set_status_spec(StatusMessageSpec::StartupScanFailed(err));
            }
            WorkerEvent::StartupEntryToggled(res)
            | WorkerEvent::ScheduledTaskRunResult(res)
            | WorkerEvent::ServiceLiveUpdated(res) => {
                self.startup_mutating = false;
                self.startup_tasks_inflight = self.startup_tasks_inflight.saturating_sub(1);
                self.sync_startup_tasks_busy();
                match res {
                    Ok(msg) => {
                        self.refresh_ledger();
                        self.set_status_spec(StatusMessageSpec::ActionMessage(msg));
                        self.scan_startup_and_tasks_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::ActionMessage(e));
                    }
                }
            }
            WorkerEvent::PlatformInventoryLoaded {
                dimms,
                volumes,
                sec,
                schemes,
                hib,
            } => {
                self.ram_dimms = dimms;
                self.storage_volumes = volumes;
                self.platform_security = Some(sec);
                self.power_schemes = schemes;
                self.hibernation_status = hib;
            }
            WorkerEvent::PowerActionCompleted {
                kind,
                schemes,
                hib,
                msg,
            } => {
                match kind {
                    PowerActionKind::Scheme => self.power_scheme_inflight = false,
                    PowerActionKind::Hibernation => self.hibernation_inflight = false,
                    PowerActionKind::UsbPower => self.usb_power_inflight = false,
                }
                self.power_schemes = schemes;
                self.hibernation_status = hib;
                self.refresh_ledger();
                if msg.is_ok() {
                    if !self.core_parking_inflight {
                        self.refresh_core_parking();
                    }
                    self.force_scan();
                }
                match msg {
                    Ok(ok) => self.set_status_spec(StatusMessageSpec::ActionMessage(ok)),
                    Err(err) => self.set_status_spec(StatusMessageSpec::PowerPlanError(err)),
                }
            }
            WorkerEvent::ExplorerRestarted(res) => match res {
                Ok(_) => self.set_status_spec(StatusMessageSpec::ExplorerRestartedOk),
                Err(err) => self.set_status_spec(StatusMessageSpec::ExplorerRestartError(err)),
            },
            WorkerEvent::BaselineReset(res) => {
                self.baseline_reset_inflight = false;
                match res {
                    Ok((initial, latest)) => {
                        self.initial_baseline = initial;
                        self.latest_baseline = latest;
                        self.set_status_spec(StatusMessageSpec::BaselineResetCompleted);
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::BaselineResetFailed(e));
                    }
                }
            }
            WorkerEvent::StatusUpdate(msg) => {
                self.set_status_spec(StatusMessageSpec::Raw(msg));
            }
            WorkerEvent::PerfBenchmarkCompleted(score) => {
                self.perf_score = Some(score);
                self.is_benchmarking_perf = false;
                self.set_status("Performans testi başarıyla tamamlandı.");
            }
            WorkerEvent::UpdateCheckCompleted(res) => {
                match res {
                    Ok(Some(info)) => {
                        let tag = info.tag_name.clone();
                        self.update_status = UpdateStatus::Available(info);
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "New release {tag} found! Downloading update..."
                        )));
                        // Auto-download and replace as requested
                        self.download_and_install_update();
                    }
                    Ok(None) => {
                        self.update_status = UpdateStatus::UpToDate;
                        self.set_status("WinBlast0 is up to date.");
                    }
                    Err(e) => {
                        self.update_status = UpdateStatus::Failed(e.clone());
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Update check failed: {e}"
                        )));
                    }
                }
            }
            WorkerEvent::UpdateDownloadCompleted(res) => match res {
                Ok(path) => {
                    self.update_status = UpdateStatus::ReadyToRestart {
                        new_binary_path: path.clone(),
                    };
                    self.set_status("Update downloaded. Applying and restarting WinBlast0...");
                    if let Err(e) = crate::core::updater::apply_and_restart(&path) {
                        self.update_status =
                            UpdateStatus::Failed(format!("Failed to auto-restart: {e}"));
                    }
                }
                Err(e) => {
                    self.update_status = UpdateStatus::Failed(e.clone());
                    self.set_status_spec(StatusMessageSpec::Raw(format!(
                        "Update download failed: {e}"
                    )));
                }
            },
            WorkerEvent::ThermalSnapshot(snapshot) => {
                self.thermal_snapshot = Some(snapshot);
                self.is_thermal_scanning = false;
            }
            WorkerEvent::BiosInfoLoaded(info) => {
                self.bios_info = Some(info);
                self.is_bios_scanning = false;
            }
            WorkerEvent::DiskReportLoaded(report) => {
                self.disk_report = Some(report);
                self.is_disk_scanning = false;
            }
            WorkerEvent::DriverScanCompleted(updates) => {
                self.driver_updates = updates;
                self.is_driver_scanning = false;
            }
            WorkerEvent::AutoTuneCompleted(res) => {
                self.is_running_automation = false;
                match res {
                    Ok(result) => {
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Auto-Tune completed: {} applied, {} failed ({} ms)",
                            result.tweaks_applied, result.tweaks_failed, result.duration_ms
                        )));
                        self.auto_tune_result = Some(result);
                        self.refresh_system_health();
                        self.start_scan();
                        self.assess_system_health();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Auto-Tune failed: {}",
                            e
                        )));
                    }
                }
            }
            WorkerEvent::StartupBatchToggled(res) => {
                self.startup_mutating = false;
                // v1.7.0 fix: the batch path incremented startup_tasks_inflight but never released
                // it, leaving the Startup view (and the HUD) busy forever after any batch toggle.
                self.startup_tasks_inflight = self.startup_tasks_inflight.saturating_sub(1);
                self.sync_startup_tasks_busy();
                match res {
                    Ok((count, enable)) => {
                        self.selected_startup_entries.clear();
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Batch updated {} startup items (enabled: {})",
                            count, enable
                        )));
                        self.scan_startup_and_tasks_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Batch startup update failed: {}",
                            e
                        )));
                    }
                }
            }
            WorkerEvent::TasksBatchToggled(res) => {
                self.startup_mutating = false;
                self.startup_tasks_inflight = self.startup_tasks_inflight.saturating_sub(1);
                self.sync_startup_tasks_busy();
                match res {
                    Ok((count, enable)) => {
                        self.selected_scheduled_tasks.clear();
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Batch updated {} scheduled tasks (enabled: {})",
                            count, enable
                        )));
                        self.scan_startup_and_tasks_async();
                    }
                    Err(e) => {
                        self.set_status_spec(StatusMessageSpec::Raw(format!(
                            "Batch task update failed: {}",
                            e
                        )));
                    }
                }
            }
        }
    }

    fn record_current_baseline(&mut self, label: &str) {
        if let Ok(l) = StateLedger::new(&self.ledger_path) {
            if let Some(ref m) = self.realtime_metrics {
                let opt_count = self
                    .scan_results
                    .iter()
                    .filter(|r| r.current_state == TweakState::AlreadyOptimized)
                    .count();
                let _ = l.save_baseline(
                    m.total_ram_mb,
                    m.used_ram_mb,
                    m.process_count,
                    m.thread_count,
                    m.timer_resolution_ns,
                    opt_count,
                    label,
                );
                self.latest_baseline = l.get_latest_baseline().ok().flatten();
            }
        }
    }

    /// Calculate before and after system impact metrics.
    pub fn calculate_before_after_impact(&self) -> Option<BeforeAfterImpact> {
        let baseline = self.initial_baseline.as_ref()?;
        let current_ram = self
            .realtime_metrics
            .as_ref()
            .map(|m| m.used_ram_mb)
            .unwrap_or(baseline.used_ram_mb);
        let current_processes = self
            .realtime_metrics
            .as_ref()
            .map(|m| m.process_count)
            .unwrap_or(baseline.process_count);
        let current_timer = self
            .realtime_metrics
            .as_ref()
            .map(|m| m.timer_resolution_ns)
            .unwrap_or(baseline.timer_resolution_ns);
        let current_opt = self
            .scan_results
            .iter()
            .filter(|r| r.current_state == TweakState::AlreadyOptimized)
            .count();

        let ram_saved = (baseline.used_ram_mb as i64) - (current_ram as i64);
        let processes_saved = (baseline.process_count as i64) - (current_processes as i64);
        let tweaks_gained = current_opt.saturating_sub(baseline.optimized_count);

        Some(BeforeAfterImpact {
            baseline_ram_mb: baseline.used_ram_mb,
            current_ram_mb: current_ram,
            ram_saved_mb: ram_saved,
            baseline_processes: baseline.process_count,
            current_processes,
            processes_saved,
            baseline_timer_ns: baseline.timer_resolution_ns,
            current_timer_ns: current_timer,
            baseline_optimized_count: baseline.optimized_count,
            current_optimized_count: current_opt,
            tweaks_gained,
        })
    }

    /// Active theme palette for all UI view rendering.
    pub fn palette(&self) -> &ThemePalette {
        &self.theme_palette
    }

    /// Retrieve localized text string without allocation.
    pub fn tr(&self, key: TextKey) -> &'static str {
        I18n::get(self.language, key)
    }

    /// Retrieve localized view-specific text string without allocation.
    pub fn tr_v(&self, key: crate::core::view_i18n::ViewTextKey) -> &'static str {
        crate::core::view_i18n::tr_view(self.language, key)
    }

    /// Set typed status message specification and re-render localized string.
    pub fn set_status_spec(&mut self, spec: StatusMessageSpec) {
        self.status_message = status_i18n::localize_status(self.language, &spec);
        self.status_spec = spec;
    }

    /// Set status message from a raw string or format macro.
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.set_status_spec(StatusMessageSpec::Raw(msg.into()));
    }

    /// Retrieve dynamically localized status message in currently active language.
    pub fn localized_status_message(&self) -> String {
        status_i18n::localize_status(self.language, &self.status_spec)
    }

    /// Retrieve localized tweak title.
    pub fn tweak_title<'a>(&self, id: &str, fallback: &'a str) -> &'a str {
        crate::core::tweak_i18n::localize_tweak(self.language, id)
            .map(|lt| lt.title)
            .unwrap_or(fallback)
    }

    /// Retrieve localized tweak description.
    pub fn tweak_description<'a>(&self, id: &str, fallback: &'a str) -> &'a str {
        crate::core::tweak_i18n::localize_tweak(self.language, id)
            .map(|lt| lt.description)
            .unwrap_or(fallback)
    }

    /// Retrieve localized category label.
    pub fn tweak_category<'a>(&self, raw_cat: &'a str) -> &'a str {
        crate::core::tweak_i18n::localize_category(self.language, raw_cat)
    }

    /// Retrieve localized sub-category label.
    pub fn tweak_sub_category<'a>(&self, raw_sub: &'a str) -> &'a str {
        crate::core::tweak_i18n::localize_sub_category(self.language, raw_sub)
    }

    /// Retrieve localized broken features list.
    pub fn tweak_broken_features(&self, id: &str, fallback: &[String]) -> Vec<String> {
        if let Some(lt) = crate::core::tweak_i18n::localize_tweak(self.language, id) {
            lt.broken_features.iter().map(|s| s.to_string()).collect()
        } else {
            fallback.to_vec()
        }
    }

    /// Retrieve localized scan detail string.
    pub fn scan_detail_text(&self, sr: &ScanResult) -> String {
        status_i18n::localize_scan_detail(self.language, sr.detail_kind.as_ref(), &sr.status_detail)
    }

    /// Set UI interface language.
    pub fn set_language(&mut self, lang: Language) {
        self.language = lang;
        self.set_status_spec(StatusMessageSpec::LanguageChanged(lang));
    }

    /// Toggle between Dark Mode and Light Mode.
    pub fn toggle_theme_mode(&mut self) {
        let new_mode = match self.theme_mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.set_theme_mode(new_mode);
    }

    /// Set theme mode (Dark or Light) and immediately rebuild theme palette.
    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.theme_palette =
            ThemePalette::new(self.theme_mode, self.color_scheme, self.theme_style);
    }

    /// Set brand accent color scheme.
    pub fn set_color_scheme(&mut self, scheme: ColorScheme) {
        self.color_scheme = scheme;
        self.theme_palette =
            ThemePalette::new(self.theme_mode, self.color_scheme, self.theme_style);
    }

    /// Set complete UI theme layout & geometry style (Minimalistic, Classic, Glassy, Compact, Materialistic).
    pub fn set_theme_style(&mut self, style: crate::core::theme::UiThemeStyle) {
        self.theme_style = style;
        self.theme_palette = ThemePalette::new(self.theme_mode, self.color_scheme, style);
    }

    /// Toggle compact navigation drawer open/closed.
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_open = !self.sidebar_open;
    }

    /// Read active telemetry update interval in milliseconds.
    pub fn telemetry_interval_ms(&self) -> u64 {
        self.telemetry_interval_ms
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Update telemetry interval in milliseconds (clamped to 250..10000 ms).
    pub fn set_telemetry_interval_ms(&mut self, ms: u64) {
        self.telemetry_interval_ms
            .store(ms.clamp(250, 10000), std::sync::atomic::Ordering::Relaxed);
    }

    /// Read active telemetry update interval in fractional seconds.
    pub fn telemetry_interval_secs(&self) -> f32 {
        self.telemetry_interval_ms() as f32 / 1000.0
    }

    /// Cycle telemetry update interval between 2.0s (default/increased) -> 4.0s (eco) -> 0.5s (fast) -> 1.0s (standard).
    pub fn cycle_telemetry_interval(&mut self) {
        let current = self.telemetry_interval_ms();
        let next = match current {
            0..=750 => 1000,
            751..=1500 => 2000,
            1501..=3000 => 4000,
            _ => 500,
        };
        self.set_telemetry_interval_ms(next);
    }

    /// Navigate to a view tab and close compact drawer, syncing mode & lazily warming view-specific scans.
    pub fn navigate(&mut self, view: ActiveView) {
        self.record_user_interaction();
        self.sync_search_state();
        self.active_view = view;
        self.sidebar_open = false;
        match view {
            ActiveView::EasyDashboard => self.mode = AppMode::Easy,
            ActiveView::AdvancedMatrix => self.mode = AppMode::Advanced,
            ActiveView::CleanAndApps => {
                if self.clean_targets.is_empty() && !self.clean_scan_inflight {
                    self.scan_clean_targets_async();
                }
                if self.installed_appx.is_empty() && !self.appx_scan_inflight {
                    self.scan_appx_and_features_async();
                }
            }
            ActiveView::StartupAndTasks => {
                if self.startup_entries.is_empty()
                    && self.scheduled_tasks.is_empty()
                    && !self.startup_scan_inflight
                {
                    self.scan_startup_and_tasks_async();
                }
            }
            ActiveView::HardwareView if self.power_schemes.is_empty() => {
                self.refresh_platform_inventory();
            }
            ActiveView::DriverBios => {
                if self.bios_info.is_none() && !self.is_bios_scanning {
                    self.scan_bios_info_async();
                }
                if self.driver_updates.is_empty() && !self.is_driver_scanning {
                    self.scan_online_drivers();
                }
            }
            ActiveView::FaultDiagnostics if self.fault_report.is_none() => {
                self.fault_report = Some(crate::tools::diagnostics::FaultDiagnosticsReport::scan());
            }
            ActiveView::ThermalMonitor => {
                if self.thermal_snapshot.is_none() && !self.is_thermal_scanning {
                    self.scan_thermal_snapshot_async();
                }
            }
            ActiveView::DiskHealth if self.disk_report.is_none() => {
                if !self.is_disk_scanning {
                    self.scan_disk_health_async();
                }
            }
            ActiveView::NetworkDiag if self.network_diag.is_none() => {
                self.network_diag = Some(crate::tools::network_diag::NetworkDiagnostics::default());
            }
            ActiveView::GamePrerequisites => {
                if self.game_prereqs.is_empty() && !self.is_prereqs_scanning {
                    self.scan_game_prereqs();
                }
            }
            ActiveView::AutomationCenter if self.system_health_assessment.is_none() => {
                self.refresh_system_health();
            }
            _ => {}
        }
    }

    /// Navigate to next view in keyboard sequence.
    pub fn nav_next_view(&mut self) {
        let nxt = self.active_view.next();
        self.navigate(nxt);
    }

    /// Navigate to previous view in keyboard sequence.
    pub fn nav_prev_view(&mut self) {
        let prv = self.active_view.prev();
        self.navigate(prv);
    }

    /// Perform online driver scan across Display, Network, Audio, Storage, and Virtual Sandbox devices asynchronously.
    pub fn scan_online_drivers(&mut self) {
        if self.is_driver_scanning {
            return;
        }
        self.is_driver_scanning = true;
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        spawn_guarded_worker(
            tx,
            waker,
            |_err| WorkerEvent::DriverScanCompleted(Vec::new()),
            |tx| {
                let updates = crate::hardware::driver_update::scan_online_drivers();
                let _ = tx.send(WorkerEvent::DriverScanCompleted(updates));
            },
        );
    }

    /// Probe motherboard and BIOS telemetry asynchronously.
    pub fn scan_bios_info_async(&mut self) {
        if self.is_bios_scanning {
            return;
        }
        self.is_bios_scanning = true;
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        spawn_guarded_worker(
            tx,
            waker,
            |_err| WorkerEvent::BiosInfoLoaded(crate::hardware::bios::BiosSystemInfo::default()),
            |tx| {
                let info = crate::hardware::bios::BiosSystemInfo::probe();
                let _ = tx.send(WorkerEvent::BiosInfoLoaded(info));
            },
        );
    }

    /// Probe disk subsystem and SMART telemetry asynchronously.
    pub fn scan_disk_health_async(&mut self) {
        if self.is_disk_scanning {
            return;
        }
        self.is_disk_scanning = true;
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        spawn_guarded_worker(
            tx,
            waker,
            |_err| {
                WorkerEvent::DiskReportLoaded(crate::hardware::disks::DiskSubsystemReport::default())
            },
            |tx| {
                let report = crate::hardware::disks::DiskSubsystemReport::probe();
                let _ = tx.send(WorkerEvent::DiskReportLoaded(report));
            },
        );
    }

    /// Probe system thermals and fan speed telemetry asynchronously.
    pub fn scan_thermal_snapshot_async(&mut self) {
        if self.is_thermal_scanning {
            return;
        }
        self.is_thermal_scanning = true;
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        spawn_guarded_worker(
            tx,
            waker,
            |_err| {
                WorkerEvent::ThermalSnapshot(
                    crate::hardware::thermal::SystemThermalSnapshot::default(),
                )
            },
            |tx| {
                let snap = crate::hardware::thermal::SystemThermalSnapshot::read();
                let _ = tx.send(WorkerEvent::ThermalSnapshot(snap));
            },
        );
    }

    /// Download and stage driver update.
    pub fn install_driver_update(&mut self, driver_id: String) {
        self.driver_install_in_flight = Some(driver_id.clone());
        let staging_dir = std::env::temp_dir().join("winblast_driver_staging");
        if let Some(item) = self.driver_updates.iter_mut().find(|d| d.id == driver_id) {
            let _ = crate::hardware::driver_update::download_and_install_driver(item, &staging_dir);
        }
        self.driver_install_in_flight = None;
        if let Some(ref w) = self.waker {
            w();
        }
    }

    /// Run isolated test driver install in virtual sandbox.
    pub fn test_sandbox_driver_install(&mut self) {
        self.driver_install_in_flight = Some("virtual_test_sandbox_driver".to_string());
        let staging_dir = std::env::temp_dir().join("winblast_driver_sandbox");
        if let Some(item) = self
            .driver_updates
            .iter_mut()
            .find(|d| d.id == "virtual_test_sandbox_driver")
        {
            let _ = crate::hardware::driver_update::download_and_install_driver(item, &staging_dir);
        } else {
            let mut item = crate::hardware::driver_update::create_sandbox_test_item();
            let _ = crate::hardware::driver_update::download_and_install_driver(
                &mut item,
                &staging_dir,
            );
            self.driver_updates.push(item);
        }
        self.driver_install_in_flight = None;
        if let Some(ref w) = self.waker {
            w();
        }
    }

    /// Backup an installed driver by ID to %TEMP%\winblast_driver_backup\.
    pub fn backup_driver(&mut self, driver_id: String) {
        if let Some(item) = self.driver_updates.iter_mut().find(|d| d.id == driver_id) {
            match crate::hardware::driver_update::backup_driver(item, None) {
                Ok(path) => {
                    item.status_message = format!("Backup created at {}", path.display());
                }
                Err(e) => {
                    item.status_message = format!("Backup failed: {}", e);
                }
            }
        }
        if let Some(ref w) = self.waker {
            w();
        }
    }

    /// Launch performance benchmark in an isolated worker thread so the UI never freezes.
    pub fn start_perf_benchmark_async(&mut self) {
        if self.is_benchmarking_perf {
            return;
        }
        self.is_benchmarking_perf = true;
        self.set_status("Performans benchmark'ı çalıştırılıyor...");
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();

        std::thread::spawn(move || {
            let score = crate::tools::performance_tests::BenchmarkRunner::run_quick();
            let _ = tx.send(WorkerEvent::PerfBenchmarkCompleted(score));
            if let Some(ref w) = waker {
                w();
            }
        });
    }

    /// Relaunch WinBlast0 with elevated administrator privileges via Windows UAC.
    pub fn relaunch_elevated(&self) {
        if let Ok(exe) = std::env::current_exe() {
            let clean_exe = crate::core::updater::clean_exe_path(&exe.to_string_lossy());
            let file_w: Vec<u16> = clean_exe.encode_utf16().chain(std::iter::once(0)).collect();
            let op_w: Vec<u16> = "runas".encode_utf16().chain(std::iter::once(0)).collect();
            let hinst = unsafe {
                windows::Win32::UI::Shell::ShellExecuteW(
                    None,
                    windows::core::PCWSTR(op_w.as_ptr()),
                    windows::core::PCWSTR(file_w.as_ptr()),
                    None,
                    None,
                    windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
                )
            };
            if (hinst.0 as isize) > 32 {
                std::process::exit(0);
            } else {
                tracing::warn!(
                    "UAC elevation relaunch was canceled or failed (code: {})",
                    hinst.0 as isize
                );
            }
        }
    }

    /// Check whether current process has Administrator elevation.
    pub fn is_admin_elevated(&self) -> bool {
        self.is_elevated
    }

    /// Relaunch WinBlast0 with administrator privileges.
    pub fn relaunch_as_admin(&mut self) {
        if let Err(e) = crate::core::elevation::relaunch_as_admin() {
            self.set_status_spec(StatusMessageSpec::Raw(format!("Relaunch failed: {e}")));
        }
    }

    /// Trigger asynchronous GitHub release query.
    pub fn check_for_updates(&mut self) {
        self.update_status = UpdateStatus::Checking;
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        std::thread::spawn(move || {
            let res = crate::core::updater::check_latest_release(crate::core::updater::GITHUB_REPO);
            let _ = tx.send(WorkerEvent::UpdateCheckCompleted(res));
            if let Some(ref w) = waker {
                w();
            }
        });
    }

    /// Download latest release binary from GitHub.
    pub fn download_and_install_update(&mut self) {
        let release = match &self.update_status {
            UpdateStatus::Available(info) => info.clone(),
            _ => return,
        };
        let url = match &release.download_url {
            Some(u) => u.clone(),
            None => {
                self.update_status = UpdateStatus::Failed(
                    "No release asset binary available for download.".to_string(),
                );
                return;
            }
        };
        let asset_name = release
            .asset_name
            .clone()
            .unwrap_or_else(|| "WinBlast0.exe".to_string());

        self.update_status = UpdateStatus::Downloading { progress_pct: 10.0 };
        let tx = self.tx_events.clone();
        let waker = self.waker.clone();
        std::thread::spawn(move || {
            let res = crate::core::updater::download_release_asset(&url, &asset_name);
            let _ = tx.send(WorkerEvent::UpdateDownloadCompleted(res));
            if let Some(ref w) = waker {
                w();
            }
        });
    }

    /// Replace binary and restart into new version.
    pub fn apply_and_restart(&mut self) {
        let path = match &self.update_status {
            UpdateStatus::ReadyToRestart { new_binary_path } => new_binary_path.clone(),
            _ => return,
        };
        if let Err(e) = crate::core::updater::apply_and_restart(&path) {
            self.update_status = UpdateStatus::Failed(format!("Restart failed: {e}"));
        }
    }

    /// Open GitHub new issue URL in default browser for bug reports.
    pub fn open_bug_report(&self) {
        let _ = crate::core::process::SilentCommand::new("rundll32")
            .args([
                "url.dll,FileProtocolHandler",
                "https://github.com/Konstrukturmx/WinBlast-Optimizer/issues/new",
            ])
            .spawn();
    }

    /// Toggle bottom changelog and transaction ledger drawer.
    pub fn toggle_changelog(&mut self) {
        self.show_changelog = !self.show_changelog;
    }

    /// Clear transaction history ledger.
    pub fn clear_changelog(&mut self) {
        if let Ok(ledger) = StateLedger::new(&self.ledger_path) {
            let _ = ledger.clear_history();
        }
        self.ledger_history.clear();
        self.set_status_spec(StatusMessageSpec::ChangelogCleared);
    }

    /// Calculate system-wide optimization progress percentage (0.0 ..= 1.0) over applicable tweaks only.
    pub fn optimization_progress(&self) -> f32 {
        let applicable_count = self
            .scan_results
            .iter()
            .filter(|r| r.current_state != TweakState::NotApplicable)
            .count();
        if applicable_count == 0 {
            return 0.0;
        }
        let opt = self
            .scan_results
            .iter()
            .filter(|r| r.current_state == TweakState::AlreadyOptimized)
            .count() as f32;
        (opt / applicable_count as f32).clamp(0.0, 1.0)
    }

    /// Refresh CPU core parking statuses asynchronously.
    pub fn refresh_core_parking(&mut self) {
        if self.core_parking_inflight || self.is_rolling_back {
            return;
        }
        self.core_parking_inflight = true;
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StatusUpdate(format!("Core parking refresh panic: {}", msg)),
            move |tx| {
                let state = backend.detect_core_parking_sampled();
                let _ = tx.send(WorkerEvent::CoreParkingUpdated(state));
            },
        );
    }

    /// Unpark all CPU logical cores via stealth powercfg command + record in changelog ledger.
    pub fn unpark_all_cores(&mut self) {
        if self.core_parking_inflight || self.is_rolling_back {
            return;
        }
        self.core_parking_inflight = true;
        self.set_status_spec(StatusMessageSpec::CoreParkingUnparking);
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::CoreParkingActionCompleted {
                state: None,
                msg: Err(format!("Failed to unpark cores (worker panic): {}", msg)),
            },
            move |tx| {
                let prev_snap = {
                    let (min_ac, min_dc) = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMINCORES,
                        )
                        .unwrap_or((100, 100));
                    let (max_ac, max_dc) = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMAXCORES,
                        )
                        .unwrap_or((100, 100));
                    let min1 = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMINCORES1,
                        )
                        .ok();
                    let max1 = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMAXCORES1,
                        )
                        .ok();
                    crate::hardware::parking::ParkingSnapshot {
                        min_ac,
                        min_dc,
                        max_ac,
                        max_dc,
                        min1,
                        max1,
                    }
                };
                let (state, msg) = match backend.unpark_all_cores() {
                    Ok(()) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "cpu_unpark_all",
                                "CoreParking:AllLogicalCores",
                                &prev_snap.serialize(),
                                "unparked",
                            );
                        }
                        (
                            Some(backend.detect_core_parking_sampled()),
                            Ok("All CPU cores successfully unparked.".to_string()),
                        )
                    }
                    Err(e) => (None, Err(format!("Failed to unpark cores: {}", e))),
                };
                let _ = tx.send(WorkerEvent::CoreParkingActionCompleted { state, msg });
            },
        );
    }

    /// Restore stock Windows core parking thresholds + record in changelog ledger.
    pub fn restore_stock_parking(&mut self) {
        if self.core_parking_inflight || self.is_rolling_back {
            return;
        }
        self.core_parking_inflight = true;
        self.set_status_spec(StatusMessageSpec::CoreParkingRestoring);
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::CoreParkingActionCompleted {
                state: None,
                msg: Err(format!(
                    "Failed to restore core parking (worker panic): {}",
                    msg
                )),
            },
            move |tx| {
                let prev_snap = {
                    let (min_ac, min_dc) = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMINCORES,
                        )
                        .unwrap_or((100, 100));
                    let (max_ac, max_dc) = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMAXCORES,
                        )
                        .unwrap_or((100, 100));
                    let min1 = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMINCORES1,
                        )
                        .ok();
                    let max1 = backend
                        .read_power_setting_index(
                            None,
                            crate::hardware::parking::SUB_PROCESSOR,
                            crate::hardware::parking::CPMAXCORES1,
                        )
                        .ok();
                    crate::hardware::parking::ParkingSnapshot {
                        min_ac,
                        min_dc,
                        max_ac,
                        max_dc,
                        min1,
                        max1,
                    }
                };
                let (state, msg) = match backend.restore_stock_parking() {
                    Ok(()) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "cpu_parking_stock",
                                "CoreParking:AllLogicalCores",
                                &prev_snap.serialize(),
                                "parked",
                            );
                        }
                        (
                            Some(backend.detect_core_parking_sampled()),
                            Ok("Stock core parking thresholds restored.".to_string()),
                        )
                    }
                    Err(e) => (None, Err(format!("Failed to restore core parking: {}", e))),
                };
                let _ = tx.send(WorkerEvent::CoreParkingActionCompleted { state, msg });
            },
        );
    }

    /// Start a full system and network latency benchmark asynchronously.
    pub fn start_latency_benchmark(&mut self) {
        if self.is_benchmarking_latency {
            return;
        }
        self.is_benchmarking_latency = true;
        self.set_status_spec(StatusMessageSpec::LatencyBenchmarkRunning);
        let backend = Arc::clone(&self.backend);
        let stage = if self.latency_before.is_some() {
            "AFTER"
        } else {
            "BEFORE"
        };
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            WorkerEvent::LatencyBenchmarkFailed,
            move |tx| {
                let bench = backend.run_latency_benchmark(stage);
                let _ = tx.send(WorkerEvent::LatencyBenchmarkCompleted(bench));
            },
        );
    }

    /// Set current latency test snapshot as "Before" baseline.
    pub fn set_latency_baseline_before(&mut self) {
        if let Some(bench) = &self.latest_latency_bench {
            let mut before = bench.clone();
            before.stage = "BEFORE".to_string();
            self.latency_before = Some(before);
            self.latency_after = None;
            self.set_status_spec(StatusMessageSpec::LatencyBaselineSaved);
        } else {
            self.set_status_spec(StatusMessageSpec::LatencyRunTestFirst);
        }
    }

    /// Trigger a full non-blocking background system scan (ignores if one is already running).
    pub fn start_scan(&mut self) {
        if self.scan_worker_inflight {
            return;
        }
        self.run_scan_internal();
    }

    /// Trigger a full multi-threaded parallel system scan in the background (Milestone 1).
    pub fn trigger_full_scan(&mut self) {
        self.start_scan();
    }

    /// Force a fresh system scan, bumping the epoch so any older in-flight scan is discarded.
    pub fn force_scan(&mut self) {
        self.run_scan_internal();
    }

    fn run_scan_internal(&mut self) {
        self.is_scanning = true;
        self.scan_worker_inflight = true;
        self.mutated_during_scan.clear();
        self.progress = 0.25;
        self.progress_message = Some(self.tr(TextKey::ProgressBarScanning).to_string());
        self.current_epoch += 1;
        let epoch = self.current_epoch;
        self.set_status_spec(StatusMessageSpec::ScanningStart);

        let backend = Arc::clone(&self.backend);
        let tweaks = Arc::clone(&self.tweaks);
        let build = self
            .os_info
            .as_ref()
            .map(|o| o.build_number)
            .unwrap_or(19041);
        let edition = self
            .os_info
            .as_ref()
            .map(|o| o.edition.clone())
            .unwrap_or_else(|| "Pro".to_string());

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::ScanFailed { epoch, error: msg },
            move |tx| {
                let results = scan_all_tweaks_parallel_with(&*backend, &tweaks, build, &edition);
                let _ = tx.send(WorkerEvent::ScanCompleted { epoch, results });
            },
        );
    }

    /// Toggle between Normal (staged batch apply) and Instant (immediate apply/revert) mode.
    pub fn toggle_engine_mode(&mut self) {
        self.engine_mode = match self.engine_mode {
            EngineMode::Normal => {
                self.staged_transitions.clear();
                self.selected_tweaks.clear();
                self.pending_actions.clear();
                self.pending_items.clear();
                EngineMode::Instant
            }
            EngineMode::Instant => {
                for id in &self.selected_tweaks {
                    self.staged_transitions
                        .entry(id.clone())
                        .or_insert(StagedTransition::StageApply);
                }
                self.refresh_pending_actions();
                EngineMode::Normal
            }
        };
        self.set_status_spec(match self.engine_mode {
            EngineMode::Instant => StatusMessageSpec::ModeSwitchedInstant,
            EngineMode::Normal => StatusMessageSpec::ModeSwitchedNormal,
        });
    }

    /// Determine whether a tweak's switch should be rendered as Active/ON (O(1) via scan_index).
    pub fn is_switch_active(&self, tweak_id: &str) -> bool {
        if self.engine_mode == EngineMode::Instant {
            return self
                .get_tweak_scan(tweak_id)
                .map(|r| r.current_state == TweakState::AlreadyOptimized)
                .unwrap_or(false);
        }
        if let Some(transition) = self.staged_transitions.get(tweak_id) {
            match transition {
                StagedTransition::StageApply => true,
                StagedTransition::StageRevert => false,
            }
        } else {
            self.get_tweak_scan(tweak_id)
                .map(|r| r.current_state == TweakState::AlreadyOptimized)
                .unwrap_or(false)
        }
    }

    /// Handle user interaction with a tweak's toggle switch.
    pub fn handle_switch_click(&mut self, tweak_id: String) {
        if self.is_applying || self.is_rolling_back || self.in_flight_tweaks.contains(&tweak_id) {
            return;
        }

        let is_currently_active = self.is_switch_active(&tweak_id);
        let target_active = !is_currently_active;

        match self.engine_mode {
            EngineMode::Normal => {
                let live_optimized = self
                    .get_tweak_scan(&tweak_id)
                    .map(|r| r.current_state == TweakState::AlreadyOptimized)
                    .unwrap_or(false);

                if target_active == live_optimized {
                    self.staged_transitions.remove(&tweak_id);
                    self.selected_tweaks.remove(&tweak_id);
                } else if target_active {
                    self.staged_transitions
                        .insert(tweak_id.clone(), StagedTransition::StageApply);
                    self.selected_tweaks.insert(tweak_id);
                } else {
                    self.staged_transitions
                        .insert(tweak_id.clone(), StagedTransition::StageRevert);
                    self.selected_tweaks.insert(tweak_id);
                }
                self.refresh_pending_actions();
                self.set_status_spec(StatusMessageSpec::StagedCount(
                    self.staged_transitions.len(),
                ));
            }
            EngineMode::Instant => {
                self.in_flight_tweaks.insert(tweak_id.clone());
                let tweak_def = match self.tweaks.iter().find(|t| t.id == tweak_id).cloned() {
                    Some(t) => t,
                    None => {
                        self.in_flight_tweaks.remove(&tweak_id);
                        return;
                    }
                };

                let backend = Arc::clone(&self.backend);
                let ledger_path = self.ledger_path.clone();
                let epoch = self.current_epoch;
                let panic_id = tweak_def.id.clone();
                let build = self
                    .os_info
                    .as_ref()
                    .map(|o| o.build_number)
                    .unwrap_or(19041);
                let edition = self
                    .os_info
                    .as_ref()
                    .map(|o| o.edition.clone())
                    .unwrap_or_else(|| "Pro".to_string());

                self.set_status_spec(StatusMessageSpec::InstantUpdating {
                    tweak_id: tweak_def.id.clone(),
                    tweak_title_fallback: tweak_def.title.clone(),
                    apply: target_active,
                });

                spawn_guarded_worker(
                    self.tx_events.clone(),
                    self.waker.clone(),
                    move |msg| WorkerEvent::SingleTweakFailed {
                        epoch,
                        tweak_id: panic_id,
                        error: format!("Worker panic: {}", msg),
                    },
                    move |tx| {
                        let ledger = match StateLedger::new(&ledger_path) {
                            Ok(l) => l,
                            Err(e) => {
                                let _ = tx.send(WorkerEvent::SingleTweakFailed {
                                    epoch,
                                    tweak_id: tweak_def.id.clone(),
                                    error: format!("Ledger error: {}", e),
                                });
                                return;
                            }
                        };

                        let res = if target_active {
                            crate::executor::engine::apply_tweak_with(
                                &tweak_def, &ledger, &*backend,
                            )
                        } else {
                            crate::executor::engine::revert_tweak_with(
                                &tweak_def, &ledger, &*backend,
                            )
                        };

                        match res {
                            Ok(_) => {
                                let updated_result =
                                    scan_single_tweak_with(&*backend, &tweak_def, build, &edition);
                                let _ = tx.send(WorkerEvent::SingleTweakUpdated {
                                    epoch,
                                    tweak_id: tweak_def.id.clone(),
                                    result: updated_result,
                                });
                            }
                            Err(e) => {
                                let _ = tx.send(WorkerEvent::SingleTweakFailed {
                                    epoch,
                                    tweak_id: tweak_def.id.clone(),
                                    error: e.to_string(),
                                });
                            }
                        }
                    },
                );
            }
        }
    }

    /// Reset performance baseline to the current system metrics asynchronously on a guarded worker.
    pub fn reset_baseline(&mut self) {
        if self.baseline_reset_inflight {
            return;
        }
        self.baseline_reset_inflight = true;
        self.set_status_spec(StatusMessageSpec::BaselineResetting);
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let opt_count = self
            .scan_results
            .iter()
            .filter(|r| r.current_state == TweakState::AlreadyOptimized)
            .count();

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::BaselineReset(Err(format!("Baseline reset panic: {}", msg))),
            move |tx| {
                let res =
                    (|| -> Result<(Option<SystemBaseline>, Option<SystemBaseline>), String> {
                        let ledger = StateLedger::new(&ledger_path)
                            .map_err(|e| format!("Failed to open ledger: {}", e))?;
                        let snap = backend.capture_latency();
                        ledger
                            .reset_baseline(
                                snap.total_ram_mb,
                                snap.used_ram_mb,
                                snap.process_count,
                                snap.thread_count,
                                snap.timer_resolution_ns,
                                opt_count,
                                "User Reset Baseline",
                            )
                            .map_err(|e| format!("Failed to reset baseline: {}", e))?;
                        let initial = ledger.get_initial_baseline().ok().flatten();
                        let latest = ledger.get_latest_baseline().ok().flatten();
                        Ok((initial, latest))
                    })();
                let _ = tx.send(WorkerEvent::BaselineReset(res));
            },
        );
    }

    /// Apply all staged tweaks with SQLite transactional rollback recording.
    pub fn apply_selected(&mut self) {
        if self.is_applying || self.is_rolling_back || !self.in_flight_tweaks.is_empty() {
            return;
        }
        let mut to_apply = Vec::new();
        let mut to_revert = Vec::new();

        for (id, transition) in &self.staged_transitions {
            match transition {
                StagedTransition::StageApply => to_apply.push(id.clone()),
                StagedTransition::StageRevert => to_revert.push(id.clone()),
            }
        }

        for id in &self.selected_tweaks {
            if !self.staged_transitions.contains_key(id) {
                to_apply.push(id.clone());
            }
        }

        if to_apply.is_empty() && to_revert.is_empty() {
            self.set_status_spec(StatusMessageSpec::NothingStaged);
            return;
        }

        self.is_applying = true;
        self.progress = 0.50;
        self.progress_message = Some(self.tr(TextKey::ProgressBarApplying).to_string());
        self.set_status_spec(StatusMessageSpec::BatchApplying {
            apply_count: to_apply.len(),
            revert_count: to_revert.len(),
        });

        let backend = Arc::clone(&self.backend);
        let tweaks = Arc::clone(&self.tweaks);
        let ledger_path = self.ledger_path.clone();

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::ApplyCompleted(Err(format!("Apply worker panic: {}", msg))),
            move |tx| {
                let res = (|| -> Result<usize, String> {
                    let ledger = StateLedger::new(&ledger_path)
                        .map_err(|e| format!("Failed to open ledger: {}", e))?;
                    let mut count = 0usize;
                    if !to_revert.is_empty() {
                        count += crate::executor::engine::revert_tweaks_by_id_with(
                            &to_revert, &tweaks, &ledger, &*backend,
                        )
                        .map_err(|e| format!("Revert error: {}", e))?;
                    }
                    if !to_apply.is_empty() {
                        count += crate::executor::engine::apply_tweaks_by_id_with(
                            &to_apply, &tweaks, &ledger, &*backend,
                        )
                        .map_err(|e| format!("Apply error: {}", e))?;
                    }
                    Ok(count)
                })();
                let _ = tx.send(WorkerEvent::ApplyCompleted(res));
            },
        );
    }

    /// Solve the current optimization goal using A* heuristic graph search,
    /// validate Pareto dominance with Raven, and sequence dependencies via Dijkstra.
    pub fn solve_current_goal(&mut self) {
        let planner = crate::graph::astar::AStarPlanner::new();
        let candidates: Vec<crate::graph::astar::TweakCandidate> = self
            .tweaks
            .iter()
            .filter(|t| {
                if let Some(sr) = self.get_tweak_scan(&t.id) {
                    sr.current_state != TweakState::AlreadyOptimized
                        && sr.current_state != TweakState::NotApplicable
                } else {
                    true
                }
            })
            .map(|t| {
                let risk_weight = match t.valuation.stability_risk {
                    crate::core::risk::RiskLevel::Safe => 0,
                    crate::core::risk::RiskLevel::Low => 1,
                    crate::core::risk::RiskLevel::Moderate => 2,
                    crate::core::risk::RiskLevel::High => 4,
                    crate::core::risk::RiskLevel::Extreme => 8,
                };
                let requires_reboot =
                    t.valuation.reversibility == crate::core::risk::Reversibility::RequiresReboot;
                crate::graph::astar::TweakCandidate {
                    id: t.id.clone(),
                    category: t.category.clone(),
                    perf_gain: t.valuation.performance_gain as u32,
                    privacy_gain: t.valuation.privacy_gain as u32,
                    risk_weight,
                    requires_reboot,
                }
            })
            .collect();

        let plan = planner.plan(&self.active_goal, &candidates);

        let raven = crate::graph::raven::RavenEngine::default();
        let raven_candidates: Vec<crate::graph::raven::TweakCriteria> = self
            .tweaks
            .iter()
            .filter(|t| plan.selected_tweak_ids.contains(&t.id))
            .map(|t| {
                let stab_score = match t.valuation.stability_risk {
                    crate::core::risk::RiskLevel::Safe => 0.0,
                    crate::core::risk::RiskLevel::Low => 2.5,
                    crate::core::risk::RiskLevel::Moderate => 5.0,
                    crate::core::risk::RiskLevel::High => 8.5,
                    crate::core::risk::RiskLevel::Extreme => 10.0,
                };
                let rev_score = match t.valuation.reversibility {
                    crate::core::risk::Reversibility::Instant => 10.0,
                    crate::core::risk::Reversibility::RequiresLogoff => 7.0,
                    crate::core::risk::Reversibility::RequiresReboot => 4.0,
                    crate::core::risk::Reversibility::RequiresPackageReinstall => 2.0,
                };
                crate::graph::raven::TweakCriteria {
                    id: t.id.clone(),
                    title: t.title.clone(),
                    performance_gain: t.valuation.performance_gain as f32,
                    privacy_gain: t.valuation.privacy_gain as f32,
                    stability_risk: stab_score,
                    reversibility_score: rev_score,
                    conflicts_with: t.conflicts_with.clone(),
                }
            })
            .collect();

        let (sanitized_ids, resolutions) = raven.resolve_conflicts(&raven_candidates);
        self.raven_resolutions = resolutions;

        let scheduler = crate::graph::dijkstra::DijkstraScheduler::new();
        let tweak_tuples = self.tweaks.iter().map(|t| {
            let requires_reboot =
                t.valuation.reversibility == crate::core::risk::Reversibility::RequiresReboot;
            let touches_services = t
                .actions
                .iter()
                .any(|a| matches!(a, TweakAction::ServiceState { .. }));
            (
                t.id.as_str(),
                t.prerequisites.as_slice(),
                requires_reboot,
                touches_services,
            )
        });

        let execution_order = scheduler.schedule_execution(tweak_tuples, &sanitized_ids);
        self.scheduled_order = Some(execution_order);
        self.planned_path = Some(plan);
    }

    /// Stage all tweaks recommended by the current A* plan into pending mutations.
    pub fn stage_planned_goal(&mut self) {
        if let Some(plan) = &self.planned_path {
            let count = plan.selected_tweak_ids.len();
            for id in &plan.selected_tweak_ids {
                self.selected_tweaks.insert(id.clone());
                self.staged_transitions
                    .insert(id.clone(), StagedTransition::StageApply);
            }
            self.refresh_pending_actions();
            self.set_status(format!(
                "{} A* plan tweak'i başarıyla aşamalandırıldı.",
                count
            ));
        }
    }

    /// Create a Windows System Restore point asynchronously.
    pub fn create_restore_point(&mut self) {
        if self.restore_point_inflight {
            return;
        }
        self.restore_point_inflight = true;
        self.set_status_spec(StatusMessageSpec::RestorePointCreating);
        let backend = Arc::clone(&self.backend);

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::RestorePointCreated(Err(format!("Restore point panic: {}", msg))),
            move |tx| {
                let res = backend
                    .create_restore_point("WinBlast0 Pre-Optimization Restore Point")
                    .map_err(|e| format!("Restore point error: {}", e));
                let _ = tx.send(WorkerEvent::RestorePointCreated(res));
            },
        );
    }

    /// Reload the list of system restore points.
    pub fn refresh_restore_points(&mut self) {
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StatusUpdate(format!("Restore point scan panic: {}", msg)),
            move |tx| {
                if let Ok(points) = backend.list_restore_points() {
                    let _ = tx.send(WorkerEvent::RestorePointsLoaded(points));
                }
            },
        );
    }

    /// Roll back an individual ledger mutation by its database ID.
    pub fn rollback_entry(&mut self, entry_id: i64) {
        if self.is_rolling_back || self.is_applying || !self.in_flight_tweaks.is_empty() {
            return;
        }
        self.is_rolling_back = true;
        self.set_status_spec(StatusMessageSpec::RollingBack(entry_id));
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::RollbackCompleted(Err(format!("Rollback worker panic: {}", msg))),
            move |tx| {
                let res = (|| -> Result<RollbackRefreshData, String> {
                    let ledger = StateLedger::new(&ledger_path)
                        .map_err(|e| format!("Failed to open ledger: {}", e))?;
                    let history = ledger
                        .get_history()
                        .map_err(|e| format!("Failed to read history: {}", e))?;
                    let entry = history
                        .into_iter()
                        .find(|e| e.id == entry_id)
                        .ok_or_else(|| format!("Entry #{} not found", entry_id))?;
                    crate::executor::engine::rollback_entry_with(&entry, &ledger, &*backend)
                        .map_err(|e| format!("Rollback error: {}", e))?;

                    let raw_action = entry
                        .action_type
                        .strip_prefix("Revert:")
                        .unwrap_or(&entry.action_type);
                    let kind = raw_action.split(':').next().unwrap_or_default();
                    let needs_startup_refresh =
                        matches!(kind, "StartupApproved" | "ServiceState" | "ScheduledTask");
                    let needs_features_refresh = kind == "WindowsFeature";
                    let needs_telemetry_audit = matches!(kind, "HostsBlock" | "ServiceState");
                    let needs_tweak_rescan = !matches!(kind, "GpuMsi" | "StartupApproved");

                    Ok(RollbackRefreshData {
                        gpus: backend.detect_gpus(),
                        hags_enabled: backend.is_hags_enabled(),
                        mpo_disabled: backend.is_mpo_disabled(),
                        core_parking: backend.detect_core_parking_sampled(),
                        power_schemes: backend.list_power_schemes(),
                        hibernation_status: backend.query_hibernation_status(),
                        needs_tweak_rescan,
                        needs_telemetry_audit,
                        needs_startup_refresh,
                        needs_features_refresh,
                    })
                })();
                let _ = tx.send(WorkerEvent::RollbackCompleted(res));
            },
        );
    }

    /// Toggle PCIe MSI mode for a GPU adapter and record in ledger.
    pub fn toggle_gpu_msi(&mut self, pnp_id: String, enable: bool, priority: u32) {
        if self.is_rolling_back || self.gpu_msi_inflight.contains(&pnp_id) {
            return;
        }
        self.gpu_msi_inflight.insert(pnp_id.clone());
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let new_prio = priority.clamp(1, 3);
        let panic_pnp = pnp_id.clone();

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::GpuMsiUpdated {
                pnp_id: panic_pnp,
                result: Err(format!("MSI worker panic: {}", msg)),
                gpus: None,
            },
            move |tx| {
                let (prev_msi, prev_prio) = backend
                    .detect_gpus()
                    .into_iter()
                    .find(|g| g.pnp_instance_id == pnp_id)
                    .map(|g| (g.msi_enabled, g.interrupt_priority))
                    .unwrap_or((!enable, 2));
                let res = backend
                    .set_gpu_msi_mode(&pnp_id, enable, new_prio)
                    .map_err(|e| format!("MSI configuration error: {}", e));
                let gpus = if res.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "gpu_msi_mode",
                            &format!("GpuMsi:{}", pnp_id),
                            &format!("{}|{}", prev_msi, prev_prio),
                            &format!("{}|{}", enable, new_prio),
                        );
                    }
                    Some(backend.detect_gpus())
                } else {
                    None
                };
                let _ = tx.send(WorkerEvent::GpuMsiUpdated {
                    pnp_id,
                    result: res,
                    gpus,
                });
            },
        );
    }

    /// Toggle Hardware Accelerated GPU Scheduling asynchronously and log in changelog ledger.
    pub fn toggle_hags(&mut self, enable: bool) {
        if self.hags_inflight || self.is_rolling_back {
            return;
        }
        self.hags_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::HagsUpdated {
                enable,
                result: Err(format!("HAGS worker panic: {}", msg)),
            },
            move |tx| {
                let prev = backend.is_hags_enabled();
                let res = backend.set_hags(enable).map_err(|e| e.to_string());
                if res.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "gpu_hags",
                            "GpuHags:HwSchMode",
                            if prev { "true" } else { "false" },
                            if enable { "true" } else { "false" },
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::HagsUpdated {
                    enable,
                    result: res,
                });
            },
        );
    }

    /// Toggle Multi-Plane Overlay (MPO) asynchronously and log in changelog ledger.
    pub fn toggle_mpo(&mut self, disable: bool) {
        if self.mpo_inflight || self.is_rolling_back {
            return;
        }
        self.mpo_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::MpoUpdated {
                disable,
                result: Err(format!("MPO worker panic: {}", msg)),
            },
            move |tx| {
                let prev = backend.is_mpo_disabled();
                let res = backend.set_mpo_disabled(disable).map_err(|e| e.to_string());
                if res.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "gpu_mpo",
                            "GpuMpo:OverlayTestMode",
                            if prev { "true" } else { "false" },
                            if disable { "true" } else { "false" },
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::MpoUpdated {
                    disable,
                    result: res,
                });
            },
        );
    }

    /// Return list of detected unique GPU hardware vendors.
    pub fn detected_gpu_vendors(&self) -> Vec<GpuVendor> {
        let mut vendors: Vec<GpuVendor> = Vec::new();
        for g in &self.gpus {
            if g.vendor != GpuVendor::Unknown && !vendors.contains(&g.vendor) {
                vendors.push(g.vendor);
            }
        }
        vendors
    }

    /// Toggle visibility of a tweak in views.
    pub fn toggle_hide_tweak(&mut self, id: String) {
        if self.hidden_tweaks.contains(&id) {
            self.hidden_tweaks.remove(&id);
        } else {
            self.hidden_tweaks.insert(id);
        }
    }

    /// Open transactional inspection modal for a specific single tweak ID without clobbering `pending_actions`.
    pub fn inspect_tweak(&mut self, id: &str) {
        if let Some(t) = self.tweaks.iter().find(|t| t.id == id) {
            self.inspected_tweak = Some((t.title.clone(), t.actions.clone()));
            self.show_diff_preview = true;
        }
    }

    /// Open the staged Diff Preview modal after refreshing pending actions & revert directions.
    pub fn open_diff_preview(&mut self) {
        self.inspected_tweak = None;
        self.refresh_pending_actions();
        self.show_diff_preview = true;
    }

    /// Capture low-latency snapshot asynchronously.
    pub fn refresh_latency(&mut self) {
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StatusUpdate(format!("Latency refresh panic: {}", msg)),
            move |tx| {
                let snap = backend.capture_latency();
                let _ = tx.send(WorkerEvent::LatencyUpdated(snap));
            },
        );
    }

    /// Execute a full telemetry audit.
    pub fn run_telemetry_audit(&mut self) {
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StatusUpdate(format!("Telemetry audit panic: {}", msg)),
            move |tx| {
                let audit = full_telemetry_audit_with(&*backend);
                let _ = tx.send(WorkerEvent::TelemetryAuditCompleted(audit));
            },
        );
    }

    /// Block known telemetry hosts in hosts file asynchronously and log in ledger.
    pub fn block_telemetry_hosts(&mut self) {
        if self.telemetry_hosts_inflight || self.is_rolling_back {
            return;
        }
        self.telemetry_hosts_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::TelemetryHostsBlocked(Err(format!("Hosts block panic: {}", msg))),
            move |tx| {
                let res = backend
                    .block_telemetry_hosts(TELEMETRY_HOSTS)
                    .map_err(|e| e.to_string());
                if let Ok(count) = res {
                    if count > 0 {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "privacy_hosts_block",
                                "HostsBlock:TelemetryDomains",
                                "unblocked",
                                &format!("+{} blocked", count),
                            );
                        }
                    }
                }
                let _ = tx.send(WorkerEvent::TelemetryHostsBlocked(res));
            },
        );
    }

    /// Disable known third-party background telemetry services asynchronously with ledger recording.
    pub fn neutralize_telemetry_services(&mut self) {
        if self.telemetry_services_inflight || self.is_rolling_back {
            return;
        }
        self.telemetry_services_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| {
                WorkerEvent::TelemetryServicesNeutralized(Err(format!(
                    "Service neutralize panic: {}",
                    msg
                )))
            },
            move |tx| {
                let mut disabled_count = 0usize;
                let ledger = StateLedger::new(&ledger_path).ok();
                for (svc, _) in KNOWN_TELEMETRY_SERVICES {
                    if let Some(prev_start) = backend.get_service_start_type(svc) {
                        if prev_start != 4 && backend.set_service_start_type(svc, 4).is_ok() {
                            if let Some(ref l) = ledger {
                                let _ = l.record_change(
                                    "privacy_neutralize_services",
                                    &format!("ServiceState:{}", svc),
                                    &prev_start.to_string(),
                                    "4",
                                );
                            }
                            disabled_count += 1;
                        }
                    }
                }
                let _ = tx.send(WorkerEvent::TelemetryServicesNeutralized(Ok(
                    disabled_count,
                )));
            },
        );
    }

    fn is_tweak_compatible_and_applicable(&self, t: &TweakDefinition) -> bool {
        let build = self
            .os_info
            .as_ref()
            .map(|o| o.build_number)
            .unwrap_or(19041);
        let edition = self
            .os_info
            .as_ref()
            .map(|o| o.edition.as_str())
            .unwrap_or("Pro");
        let vendors = self.detected_gpu_vendors();
        t.is_compatible(build, edition)
            && t.is_gpu_vendor_compatible(&vendors)
            && self
                .scan_index
                .get(&t.id)
                .and_then(|&i| self.scan_results.get(i))
                .is_none_or(|r| r.current_state != TweakState::NotApplicable)
    }

    /// Export currently selected forward tweaks (excluding `StageRevert`) to profile file.
    pub fn export_profile(&mut self) {
        let mut selected_ids: Vec<String> = self
            .selected_tweaks
            .iter()
            .filter(|id| self.staged_transitions.get(*id) != Some(&StagedTransition::StageRevert))
            .cloned()
            .collect();
        for (id, tr) in &self.staged_transitions {
            if *tr == StagedTransition::StageApply && !selected_ids.contains(id) {
                selected_ids.push(id.clone());
            }
        }
        selected_ids.sort();
        let path = self.profile_path.clone();
        match export_profile(&path, &selected_ids) {
            Ok(()) => {
                self.set_status_spec(StatusMessageSpec::ProfileSaved(
                    path.to_string_lossy().to_string(),
                ));
            }
            Err(e) => {
                self.set_status_spec(StatusMessageSpec::ProfileSaveFailed(e.to_string()));
            }
        }
    }

    /// Import a profile file, synchronize `staged_transitions` (Normal mode only), and preview actions.
    pub fn import_profile(&mut self) {
        let path = self.profile_path.clone();
        match import_profile(&path) {
            Ok(ids) => {
                let compatible_ids: HashSet<String> = self
                    .tweaks
                    .iter()
                    .filter(|t| self.is_tweak_compatible_and_applicable(t))
                    .map(|t| t.id.clone())
                    .collect();

                self.selected_tweaks.clear();
                self.staged_transitions.clear();
                let mut count = 0usize;
                let mut skipped = 0usize;
                for id in ids {
                    if compatible_ids.contains(&id) {
                        self.selected_tweaks.insert(id.clone());
                        if self.engine_mode == EngineMode::Normal {
                            self.staged_transitions
                                .insert(id, StagedTransition::StageApply);
                        }
                        count += 1;
                    } else {
                        skipped += 1;
                    }
                }
                self.refresh_pending_actions();
                self.set_status_spec(StatusMessageSpec::ProfileImported {
                    imported: count,
                    skipped,
                });
            }
            Err(e) => {
                self.set_status_spec(StatusMessageSpec::ProfileImportFailed(e.to_string()));
            }
        }
    }

    /// Apply an optimization preset aligned with AutoTuneProfile algorithms.
    pub fn apply_preset(&mut self, preset: PresetKind) {
        self.selected_tweaks.clear();
        self.staged_transitions.clear();
        let is_normal = self.engine_mode == EngineMode::Normal;
        let profile = match preset {
            PresetKind::Gaming => crate::tools::automation::AutoTuneProfile::GamingLowLatency,
            PresetKind::ProAudio => crate::tools::automation::AutoTuneProfile::ProAudioDpc,
            PresetKind::LaptopBattery => {
                crate::tools::automation::AutoTuneProfile::LaptopBatterySaving
            }
        };
        let plan = crate::tools::automation::create_auto_tune_plan(&self.tweaks, profile);
        for id in plan.recommended_tweak_ids {
            if let Some(t) = self.tweaks.iter().find(|t| t.id == id) {
                if (self.mode != AppMode::Easy || t.easy_mode_included)
                    && self.is_tweak_compatible_and_applicable(t)
                {
                    self.selected_tweaks.insert(id.clone());
                    if is_normal {
                        let live_optimized = self
                            .get_tweak_scan(&id)
                            .map(|r| r.current_state == TweakState::AlreadyOptimized)
                            .unwrap_or(false);
                        if !live_optimized {
                            self.staged_transitions
                                .insert(id, StagedTransition::StageApply);
                        }
                    }
                }
            }
        }

        self.set_status_spec(StatusMessageSpec::PresetActivated {
            kind: match preset {
                PresetKind::Gaming => PresetKey::Gaming,
                PresetKind::ProAudio => PresetKey::ProAudio,
                PresetKind::LaptopBattery => PresetKey::LaptopBattery,
            },
            count: self.selected_tweaks.len(),
        });
        self.refresh_pending_actions();

        if self.engine_mode == EngineMode::Instant && !self.selected_tweaks.is_empty() {
            self.execute_auto_tune_profile(profile);
        }
    }

    /// Privacy Shield "Select All": every eligible privacy/telemetry tweak, using the same
    /// category predicate as the Privacy list and the same safety rules as the matrix
    /// (risk below High, compatible with this OS/hardware, Easy-mode inclusion). Staging only
    /// happens in Normal engine mode for tweaks that are not already optimized.
    /// (v1.6.1 used a different substring test, ignored compatibility and staged in Instant mode.)
    pub fn select_privacy_tweaks(&mut self) {
        let is_normal = self.engine_mode == EngineMode::Normal;
        let ids: Vec<String> = self
            .tweaks
            .iter()
            .filter(|t| {
                crate::core::tweak_i18n::is_privacy_or_telemetry_category(&t.category)
                    && t.valuation.stability_risk < RiskLevel::High
                    && (self.mode != AppMode::Easy || t.easy_mode_included)
                    && self.is_tweak_compatible_and_applicable(t)
                    && self.get_tweak_scan(&t.id).is_some_and(|r| r.current_state != TweakState::NotApplicable)
            })
            .map(|t| t.id.clone())
            .collect();
        for id in &ids {
            self.selected_tweaks.insert(id.clone());
            if is_normal {
                let optimized = self.get_tweak_scan(id).is_some_and(|r| r.current_state == TweakState::AlreadyOptimized);
                if !optimized {
                    self.staged_transitions.insert(id.clone(), StagedTransition::StageApply);
                }
            }
        }
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::SelectedCount(self.selected_tweaks.len()));
    }

    /// Shared Advanced-Matrix filter used by the list, Select-All and Deselect-All so they can
    /// never disagree: accent/diacritic-folded match against raw and localized title, category,
    /// description and id, plus risk keywords ("safe", "low", "moderate", "high", "extreme")
    /// that match the tweak's actual stability risk.
    pub fn tweak_matches_query(
        &self,
        q: &MatrixQuery,
        id: &str,
        title: &str,
        category: &str,
        description: &str,
        stability: RiskLevel,
    ) -> bool {
        if q.folded.is_empty() {
            return true;
        }
        if q.risk == Some(stability) {
            return true;
        }
        [
            title,
            id,
            category,
            description,
            self.tweak_title(id, title),
            self.tweak_category(category),
            self.tweak_description(id, description),
        ]
        .iter()
        .any(|f| I18n::fold_search_str(f).contains(q.folded.as_str()))
    }

    /// Select filtered subset of tweaks matching query and category without affecting others (Invariant I1).
    pub fn select_filtered_tweaks(&mut self, query: &str, category: Option<&str>) {
        let is_normal = self.engine_mode == EngineMode::Normal;
        let filter = MatrixQuery::new(query);
        let eligible_ids: Vec<String> = self
            .tweaks
            .iter()
            .filter(|t| {
                if let Some(cat) = category {
                    if !t.category.as_str().eq_ignore_ascii_case(cat) {
                        return false;
                    }
                }
                if !self.tweak_matches_query(&filter, &t.id, &t.title, &t.category, &t.description, t.valuation.stability_risk) {
                    return false;
                }
                t.valuation.stability_risk < RiskLevel::High
                    && (self.mode != AppMode::Easy || t.easy_mode_included)
                    && self.is_tweak_compatible_and_applicable(t)
            })
            .map(|t| t.id.clone())
            .collect();

        for id in &eligible_ids {
            self.selected_tweaks.insert(id.clone());
            if is_normal {
                let live_optimized = self
                    .get_tweak_scan(id)
                    .map(|r| r.current_state == TweakState::AlreadyOptimized)
                    .unwrap_or(false);
                if !live_optimized {
                    self.staged_transitions
                        .insert(id.clone(), StagedTransition::StageApply);
                }
            }
        }
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::SelectedCount(self.selected_tweaks.len()));
    }

    /// Deselect filtered subset of tweaks matching query and category (Invariant I1).
    pub fn deselect_filtered_tweaks(&mut self, query: &str, category: Option<&str>) {
        let filter = MatrixQuery::new(query);
        let to_remove: Vec<String> = self
            .tweaks
            .iter()
            .filter(|t| {
                if let Some(cat) = category {
                    if !t.category.as_str().eq_ignore_ascii_case(cat) {
                        return false;
                    }
                }
                self.tweak_matches_query(&filter, &t.id, &t.title, &t.category, &t.description, t.valuation.stability_risk)
            })
            .map(|t| t.id.clone())
            .collect();

        for id in &to_remove {
            self.selected_tweaks.remove(id);
            self.staged_transitions.remove(id);
        }
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::SelectedCount(self.selected_tweaks.len()));
    }

    /// Idempotently toggle selection of an entire category while keeping staged_transitions in sync.
    pub fn toggle_category_selection(&mut self, tweak_ids: &[String], select: bool) {
        let is_normal = self.engine_mode == EngineMode::Normal;
        for id in tweak_ids {
            if select {
                self.selected_tweaks.insert(id.clone());
                if is_normal {
                    let live_optimized = self
                        .get_tweak_scan(id)
                        .map(|r| r.current_state == TweakState::AlreadyOptimized)
                        .unwrap_or(false);
                    if !live_optimized {
                        self.staged_transitions
                            .insert(id.clone(), StagedTransition::StageApply);
                    }
                }
            } else {
                self.selected_tweaks.remove(id);
                self.staged_transitions.remove(id);
            }
        }
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::SelectedCount(self.selected_tweaks.len()));
    }

    /// Revert all currently selected tweaks back to Windows default values.
    pub fn revert_selected(&mut self) {
        if self.is_applying || self.is_rolling_back || !self.in_flight_tweaks.is_empty() {
            return;
        }
        match self.engine_mode {
            EngineMode::Normal => {
                let selected: Vec<String> = self.selected_tweaks.iter().cloned().collect();
                if selected.is_empty() {
                    self.set_status_spec(StatusMessageSpec::NothingStaged);
                    return;
                }
                for id in &selected {
                    let live_optimized = self
                        .get_tweak_scan(id)
                        .map(|r| r.current_state == TweakState::AlreadyOptimized)
                        .unwrap_or(false);
                    if live_optimized {
                        self.staged_transitions
                            .insert(id.clone(), StagedTransition::StageRevert);
                    } else {
                        self.staged_transitions.remove(id);
                    }
                }
                self.refresh_pending_actions();
                self.set_status_spec(StatusMessageSpec::StagedCount(
                    self.staged_transitions.len(),
                ));
            }
            EngineMode::Instant => {
                let to_revert: Vec<String> = self
                    .selected_tweaks
                    .iter()
                    .filter(|id| {
                        self.get_tweak_scan(id)
                            .map(|r| r.current_state == TweakState::AlreadyOptimized)
                            .unwrap_or(false)
                    })
                    .cloned()
                    .collect();
                if to_revert.is_empty() {
                    self.set_status_spec(StatusMessageSpec::ActionMessage(
                        "No selected tweaks require reversion.".into(),
                    ));
                    return;
                }
                for id in &to_revert {
                    self.staged_transitions
                        .insert(id.clone(), StagedTransition::StageRevert);
                }
                self.apply_selected();
            }
        }
    }

    /// Toggle a single tweak on or off using the unified engine switch handler.
    #[allow(dead_code)]
    pub fn toggle_tweak(&mut self, id: String, _select: bool) {
        self.handle_switch_click(id);
    }

    /// Select all applicable low/moderate-risk tweaks matching the current mode (`High`/`Extreme` require explicit opt-in).
    pub fn select_all(&mut self) {
        self.selected_tweaks.clear();
        self.staged_transitions.clear();
        let is_normal = self.engine_mode == EngineMode::Normal;
        let eligible_ids: Vec<String> = self
            .tweaks
            .iter()
            .filter(|t| {
                t.valuation.stability_risk < RiskLevel::High
                    && (self.mode != AppMode::Easy || t.easy_mode_included)
                    && self.is_tweak_compatible_and_applicable(t)
            })
            .map(|t| t.id.clone())
            .collect();
        for id in eligible_ids {
            self.selected_tweaks.insert(id.clone());
            if is_normal {
                self.staged_transitions
                    .insert(id, StagedTransition::StageApply);
            }
        }
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::SelectedCount(self.selected_tweaks.len()));
    }

    /// Deselect all tweaks.
    pub fn deselect_all(&mut self) {
        self.selected_tweaks.clear();
        self.staged_transitions.clear();
        self.refresh_pending_actions();
        self.set_status_spec(StatusMessageSpec::DeselectedAll);
    }

    /// Recompute pending mutations & direction flags for diff modal inspection.
    pub fn refresh_pending_actions(&mut self) {
        let mut actions = Vec::new();
        let mut items = Vec::new();
        for t in self.tweaks.iter() {
            let transition = self.staged_transitions.get(&t.id).copied();
            if transition.is_some() || self.selected_tweaks.contains(&t.id) {
                let is_revert = transition == Some(StagedTransition::StageRevert);
                for a in &t.actions {
                    actions.push(a.clone());
                    items.push((a.clone(), is_revert));
                }
            }
        }
        self.pending_actions = actions;
        self.pending_items = items;
    }

    /// Refresh ledger records from SQLite.
    pub fn refresh_ledger(&mut self) {
        if let Ok(ledger) = StateLedger::new(&self.ledger_path) {
            if let Ok(history) = ledger.get_history() {
                self.ledger_history = history;
            }
        }
    }

    // =========================================================================
    // v0.7.0 Platform Hardware, Power, Explorer, Cleanup, Apps & Startup Actions
    // =========================================================================

    /// Refresh SMBIOS RAM DIMMs, storage volumes, UEFI/SecureBoot/VBS/HVCI, power schemes & hibernation asynchronously.
    pub fn refresh_platform_inventory(&mut self) {
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StatusUpdate(format!("Platform inventory panic: {}", msg)),
            move |tx| {
                let sec = backend.detect_platform_security();
                let dimms = sec.ram_dimms.clone();
                let volumes = sec.volumes.clone();
                let schemes = backend.list_power_schemes();
                let hib = backend.query_hibernation_status();
                let _ = tx.send(WorkerEvent::PlatformInventoryLoaded {
                    dimms,
                    volumes,
                    sec,
                    schemes,
                    hib,
                });
            },
        );
    }

    /// Switch active Windows power plan asynchronously via native `powrprof.dll`.
    pub fn set_active_power_plan(&mut self, guid: String) {
        if self.power_scheme_inflight || self.is_rolling_back {
            return;
        }
        self.power_scheme_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let backend_err = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::PowerActionCompleted {
                kind: PowerActionKind::Scheme,
                schemes: backend_err.list_power_schemes(),
                hib: backend_err.query_hibernation_status(),
                msg: Err(format!("Power plan panic: {}", msg)),
            },
            move |tx| {
                let prev = backend.get_active_power_scheme().unwrap_or_default();
                let msg = match backend.set_active_power_scheme(&guid) {
                    Ok(()) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "power_active_scheme",
                                "PowerCfg:ActiveScheme",
                                &prev,
                                &guid,
                            );
                        }
                        Ok(format!("Activated power scheme {}.", guid))
                    }
                    Err(e) => Err(format!("Failed to set power plan: {}", e)),
                };
                let schemes = backend.list_power_schemes();
                let hib = backend.query_hibernation_status();
                let _ = tx.send(WorkerEvent::PowerActionCompleted {
                    kind: PowerActionKind::Scheme,
                    schemes,
                    hib,
                    msg,
                });
            },
        );
    }

    /// Create and activate the tuned `WinBlast0 Ultimate Performance` power scheme asynchronously.
    pub fn activate_winblast_ultimate_power_plan(&mut self) {
        if self.power_scheme_inflight || self.is_rolling_back {
            return;
        }
        self.power_scheme_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let backend_err = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::PowerActionCompleted {
                kind: PowerActionKind::Scheme,
                schemes: backend_err.list_power_schemes(),
                hib: backend_err.query_hibernation_status(),
                msg: Err(format!("Ultimate power plan panic: {}", msg)),
            },
            move |tx| {
                let prev = backend.get_active_power_scheme().unwrap_or_default();
                let msg = match backend.create_and_activate_performance_plan() {
                    Ok(guid) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "power_ultimate_plan",
                                "PowerCfg:UltimateScheme",
                                &prev,
                                &guid,
                            );
                        }
                        Ok(format!("WinBlast0 Ultimate Performance active ({}).", guid))
                    }
                    Err(e) => Err(format!("Power plan error: {}", e)),
                };
                let schemes = backend.list_power_schemes();
                let hib = backend.query_hibernation_status();
                let _ = tx.send(WorkerEvent::PowerActionCompleted {
                    kind: PowerActionKind::Scheme,
                    schemes,
                    hib,
                    msg,
                });
            },
        );
    }

    /// Enable or disable `hiberfil.sys` via `CallNtPowerInformation(SystemReserveHiberFile)`.
    pub fn toggle_hibernation(&mut self, enable: bool) {
        if self.hibernation_inflight || self.is_rolling_back {
            return;
        }
        self.hibernation_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let backend_err = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::PowerActionCompleted {
                kind: PowerActionKind::Hibernation,
                schemes: backend_err.list_power_schemes(),
                hib: backend_err.query_hibernation_status(),
                msg: Err(format!("Hibernation panic: {}", msg)),
            },
            move |tx| {
                let prev = backend.query_hibernation_status().hibernation_enabled;
                let (hib, msg) = match backend.set_hibernation_enabled(enable) {
                    Ok(new_hib) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "power_hibernation",
                                "PowerCfg:Hibernation",
                                if prev { "enabled" } else { "disabled" },
                                if enable { "enabled" } else { "disabled" },
                            );
                        }
                        (
                            new_hib,
                            Ok(format!(
                                "Hibernation {}.",
                                if enable {
                                    "enabled"
                                } else {
                                    "disabled (hiberfil.sys freed)"
                                }
                            )),
                        )
                    }
                    Err(e) => (
                        backend.query_hibernation_status(),
                        Err(format!("Hibernation error: {}", e)),
                    ),
                };
                let schemes = backend.list_power_schemes();
                let _ = tx.send(WorkerEvent::PowerActionCompleted {
                    kind: PowerActionKind::Hibernation,
                    schemes,
                    hib,
                    msg,
                });
            },
        );
    }

    /// Disable USB Root Hub selective suspend & D3 idle across enumerated USB hubs.
    pub fn toggle_usb_power_saving(&mut self, disable_power_saving: bool) {
        if self.usb_power_inflight || self.is_rolling_back {
            return;
        }
        self.usb_power_inflight = true;
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let backend_err = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::PowerActionCompleted {
                kind: PowerActionKind::UsbPower,
                schemes: backend_err.list_power_schemes(),
                hib: backend_err.query_hibernation_status(),
                msg: Err(format!("USB power panic: {}", msg)),
            },
            move |tx| {
                const USB_SUB: &str = "2a737441-1930-4402-8d77-b2bebba308a3";
                const USB_SET: &str = "48e6b7a6-50f5-4782-a5d4-53bb8f07e226";
                let prev_enabled = backend
                    .read_power_setting_index(None, USB_SUB, USB_SET)
                    .map(|(ac, _)| ac != 0)
                    .unwrap_or(disable_power_saving);
                let msg =
                    match backend.set_usb_power_saving_enabled(!disable_power_saving) {
                        Ok(n) => {
                            if let Ok(l) = StateLedger::new(&ledger_path) {
                                let _ = l.record_change(
                                    "power_usb_hub_idle",
                                    "PowerCfg:UsbPowerSaving",
                                    if prev_enabled { "enabled" } else { "disabled" },
                                    if disable_power_saving {
                                        "disabled"
                                    } else {
                                        "enabled"
                                    },
                                );
                            }
                            Ok(format!(
                            "Updated {} USB Root Hub controller power states (power saving {}).",
                            n,
                            if disable_power_saving { "disabled" } else { "enabled" }
                        ))
                        }
                        Err(e) => Err(format!("USB power update failed: {}", e)),
                    };
                let schemes = backend.list_power_schemes();
                let hib = backend.query_hibernation_status();
                let _ = tx.send(WorkerEvent::PowerActionCompleted {
                    kind: PowerActionKind::UsbPower,
                    schemes,
                    hib,
                    msg,
                });
            },
        );
    }

    /// Restart `explorer.exe` cleanly under the non-elevated desktop shell token.
    pub fn restart_explorer_safely(&mut self) {
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::ExplorerRestarted(Err(format!("Explorer restart panic: {}", msg))),
            move |tx| {
                let res = backend.restart_explorer_safely().map_err(|e| e.to_string());
                let _ = tx.send(WorkerEvent::ExplorerRestarted(res));
            },
        );
    }

    /// Scan 8 safe disk cleanup targets asynchronously without deleting anything.
    pub fn scan_clean_targets_async(&mut self) {
        if self.clean_scan_inflight {
            return;
        }
        self.clean_scan_inflight = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            WorkerEvent::CleanScanFailed,
            move |tx| {
                let targets = backend.scan_clean_targets();
                let _ = tx.send(WorkerEvent::CleanTargetsScanned(targets));
            },
        );
    }

    /// Toggle whether a cleanup target kind is selected for cleaning.
    pub fn toggle_clean_target(&mut self, kind: CleanTargetKind) {
        if let Some(t) = self.clean_targets.iter_mut().find(|t| t.kind == kind) {
            t.selected = !t.selected;
        }
    }

    /// Select all cleanup target kinds.
    pub fn select_all_clean_targets(&mut self) {
        for t in self.clean_targets.iter_mut() {
            t.selected = true;
        }
    }

    /// Deselect all cleanup target kinds.
    pub fn deselect_all_clean_targets(&mut self) {
        for t in self.clean_targets.iter_mut() {
            t.selected = false;
        }
    }

    /// Execute deletion of currently checked cleanup targets asynchronously.
    pub fn execute_clean_selected(&mut self) {
        if self.clean_apps_mutating {
            return;
        }
        let kinds: Vec<CleanTargetKind> = self
            .clean_targets
            .iter()
            .filter(|t| t.selected)
            .map(|t| t.kind)
            .collect();
        if kinds.is_empty() {
            self.set_status_spec(StatusMessageSpec::NoCleanupSelected);
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::CleanExecuted(Err(format!("Cleanup worker panic: {}", msg))),
            move |tx| {
                let res = backend
                    .execute_clean_targets(&kinds)
                    .map_err(|e| e.to_string());
                if let Ok(ref rep) = res {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "disk_cleanup",
                            "CleanTargets:Execute",
                            &format!("{} targets", kinds.len()),
                            &format!(
                                "{:.1} MB ({} files)",
                                rep.total_freed_bytes as f64 / (1024.0 * 1024.0),
                                rep.total_deleted_files
                            ),
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::CleanExecuted(res));
            },
        );
    }

    /// Refresh installed AppX packages (with Core OS classification), DISM optional features, & OneDrive KFM guard.
    pub fn scan_appx_and_features_async(&mut self) {
        if self.appx_scan_inflight {
            return;
        }
        self.appx_scan_inflight = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            WorkerEvent::AppxScanFailed,
            move |tx| {
                let appx = backend.list_installed_appx_detailed();
                let features = backend.list_optional_features().unwrap_or_default();
                let kfm_warn = backend.check_onedrive_kfm_active();
                let _ = tx.send(WorkerEvent::AppxAndFeaturesLoaded {
                    appx,
                    features,
                    kfm_warn,
                });
            },
        );
    }

    /// Toggle check selection for a single AppX package (refuses ProtectedCore packages).
    pub fn toggle_appx_selection(&mut self, family: String) {
        let is_core = self
            .installed_appx
            .iter()
            .find(|p| p.family_name == family)
            .is_some_and(|p| {
                p.safety_tier == crate::tools::features_apps::AppxSafetyTier::ProtectedCore
            });
        if is_core {
            return;
        }
        if !self.selected_appx.remove(&family) {
            self.selected_appx.insert(family);
        }
    }

    /// Check all installed SafeBloat + AiBloat AppX packages (strictly skipping Caution & ProtectedCore).
    pub fn select_safe_and_ai_appx(&mut self) {
        use crate::tools::features_apps::AppxSafetyTier;
        self.selected_appx.clear();
        for p in &self.installed_appx {
            if matches!(
                p.safety_tier,
                AppxSafetyTier::SafeBloat | AppxSafetyTier::AiBloat
            ) {
                self.selected_appx.insert(p.family_name.clone());
            }
        }
    }

    /// Select AppX packages matching search query. If query is empty, selects all un-protected packages.
    pub fn select_filtered_appx(&mut self, query: &str) {
        use crate::tools::features_apps::AppxSafetyTier;
        let q = query.trim().to_lowercase();
        self.selected_appx.clear();
        for p in &self.installed_appx {
            if p.safety_tier == AppxSafetyTier::ProtectedCore {
                continue;
            }
            if q.is_empty()
                || p.short_name.to_lowercase().contains(&q)
                || p.family_name.to_lowercase().contains(&q)
            {
                self.selected_appx.insert(p.family_name.clone());
            }
        }
    }

    /// Uninstall selected AppX packages in one batched PowerShell call asynchronously.
    pub fn remove_selected_appx_async(&mut self) {
        if self.clean_apps_mutating {
            return;
        }
        let fams: Vec<String> = self.selected_appx.iter().cloned().collect();
        if fams.is_empty() {
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::AppxBatchRemoved(Err(format!("AppX worker panic: {}", msg))),
            move |tx| {
                let res = backend
                    .remove_appx_batch(&fams, true)
                    .map_err(|e| e.to_string());
                if let Ok(n) = res {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "appx_batch_remove",
                            "AppXBatch:RemoveAllUsers",
                            &format!("{} selected", fams.len()),
                            &format!("{} removed", n),
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::AppxBatchRemoved(res));
            },
        );
    }

    /// Enable or disable a Windows Optional Feature / Capability cleanly via serialized DISM worker.
    pub fn toggle_optional_feature_async(
        &mut self,
        name: String,
        kind: OptionalFeatureKind,
        enable: bool,
    ) {
        if self.clean_apps_mutating {
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let req_prefix = name.split_once("~~~~").map(|(p, _)| p).unwrap_or(&name);
        let prev_enabled_opt = self
            .optional_features
            .iter()
            .find(|f| {
                f.name.eq_ignore_ascii_case(&name)
                    || f.name
                        .split_once("~~~~")
                        .map(|(p, _)| p)
                        .unwrap_or(&f.name)
                        .eq_ignore_ascii_case(req_prefix)
            })
            .map(|f| {
                matches!(
                    f.state,
                    FeatureInstallState::Enabled | FeatureInstallState::EnablePending
                )
            });
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let panic_name = name.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            move |msg| WorkerEvent::FeatureToggled {
                name: panic_name,
                result: Err(format!("Feature worker panic: {}", msg)),
            },
            move |tx| {
                let pfx = name.split_once("~~~~").map(|(p, _)| p).unwrap_or(&name);
                let prev_enabled = backend
                    .list_optional_features()
                    .ok()
                    .and_then(|feats| {
                        feats.into_iter().find(|f| {
                            f.name.eq_ignore_ascii_case(&name)
                                || f.name
                                    .split_once("~~~~")
                                    .map(|(p, _)| p)
                                    .unwrap_or(&f.name)
                                    .eq_ignore_ascii_case(pfx)
                        })
                    })
                    .map(|f| {
                        matches!(
                            f.state,
                            FeatureInstallState::Enabled | FeatureInstallState::EnablePending
                        )
                    })
                    .or(prev_enabled_opt)
                    .unwrap_or(!enable);
                let result = backend
                    .set_optional_feature(&name, kind, enable)
                    .map_err(|e| e.to_string());
                if result.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            &format!("feature_{}", name),
                            &format!(
                                "WindowsFeature:{:?}|{}",
                                kind,
                                crate::core::reg_helpers::escape_pipe(&name)
                            ),
                            &prev_enabled.to_string(),
                            &enable.to_string(),
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::FeatureToggled { name, result });
            },
        );
    }

    /// Safely uninstall OneDrive (refusing if Known Folder Move Desktop/Documents backup is active).
    pub fn uninstall_onedrive_async(&mut self) {
        if self.clean_apps_mutating {
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::OneDriveEdgeResult(Err(format!("OneDrive worker panic: {}", msg))),
            move |tx| {
                let res = backend.uninstall_onedrive_safe().map_err(|e| e.to_string());
                if res.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "app_onedrive_remove",
                            "App:OneDrive",
                            "Installed",
                            "Removed (KFM Checked)",
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::OneDriveEdgeResult(res));
            },
        );
    }

    /// Uninstall Microsoft Edge non-destructively while keeping WebView2 runtime intact.
    pub fn remove_edge_async(&mut self) {
        if self.clean_apps_mutating {
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::OneDriveEdgeResult(Err(format!("Edge worker panic: {}", msg))),
            move |tx| {
                let res = backend
                    .remove_edge_preserve_webview2()
                    .map_err(|e| e.to_string());
                if res.is_ok() {
                    if let Ok(l) = StateLedger::new(&ledger_path) {
                        let _ = l.record_change(
                            "app_edge_remove",
                            "App:EdgeBrowser",
                            "Installed",
                            "Removed (WebView2 Preserved)",
                        );
                    }
                }
                let _ = tx.send(WorkerEvent::OneDriveEdgeResult(res));
            },
        );
    }

    /// Toggle curated WinGet package checkbox.
    pub fn toggle_winget_selection(&mut self, pkg_id: String) {
        if !self.selected_winget.remove(&pkg_id) {
            self.selected_winget.insert(pkg_id);
        }
    }

    /// Install checked WinGet tools silently in background worker.
    pub fn install_selected_winget_async(&mut self) {
        if self.clean_apps_mutating {
            return;
        }
        let ids: Vec<String> = self.selected_winget.iter().cloned().collect();
        if ids.is_empty() {
            return;
        }
        self.clean_apps_mutating = true;
        self.clean_apps_inflight += 1;
        self.sync_clean_apps_busy();
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::WinGetInstallResult(Err(format!("WinGet worker panic: {}", msg))),
            move |tx| {
                let mut ok_count = 0usize;
                let mut errs = Vec::new();
                for id in &ids {
                    match backend.winget_install_package(id) {
                        Ok(_) => ok_count += 1,
                        Err(e) => errs.push(format!("{}: {}", id, e)),
                    }
                }
                let res = if errs.is_empty() {
                    Ok(format!(
                        "Successfully installed {} WinGet packages.",
                        ok_count
                    ))
                } else {
                    Err(format!(
                        "Installed {}/{} packages; errors: {}",
                        ok_count,
                        ids.len(),
                        errs.join("; ")
                    ))
                };
                let _ = tx.send(WorkerEvent::WinGetInstallResult(res));
            },
        );
    }

    /// Scan all 4 Startup sources, live Scheduled Tasks, and Win32 Services asynchronously.
    pub fn scan_startup_and_tasks_async(&mut self) {
        if self.startup_scan_inflight {
            return;
        }
        self.startup_scan_inflight = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            WorkerEvent::StartupScanFailed,
            move |tx| {
                let startup = backend.list_startup_entries().unwrap_or_default();
                let tasks = backend.list_live_tasks();
                let services = backend.list_live_services().unwrap_or_default();
                let _ = tx.send(WorkerEvent::StartupAndTasksLoaded {
                    startup,
                    tasks,
                    services,
                });
            },
        );
    }

    /// Toggle an individual startup entry via 12-byte `StartupApproved` / UWP State / Task API.
    pub fn toggle_startup_entry_async(&mut self, id: String, enable: bool) {
        if self.startup_mutating {
            return;
        }
        let Some(entry) = self.startup_entries.iter().find(|e| e.id == id).cloned() else {
            return;
        };
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::StartupEntryToggled(Err(format!("Startup worker panic: {}", msg))),
            move |tx| {
                let live_entry = backend
                    .list_startup_entries()
                    .ok()
                    .and_then(|list| list.into_iter().find(|e| e.id == entry.id))
                    .unwrap_or(entry);
                let prev_enabled = live_entry.enabled;
                let res = match backend.set_startup_entry_enabled(&live_entry, enable) {
                    Ok(()) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                &format!("startup_{}", live_entry.name),
                                &format!(
                                    "StartupApproved:{}",
                                    crate::core::reg_helpers::escape_pipe(&live_entry.id)
                                ),
                                &prev_enabled.to_string(),
                                &enable.to_string(),
                            );
                        }
                        Ok(format!(
                            "Startup item '{}' {}.",
                            live_entry.name,
                            if enable { "enabled" } else { "disabled" }
                        ))
                    }
                    Err(e) => Err(format!("Failed to update '{}': {}", live_entry.name, e)),
                };
                let _ = tx.send(WorkerEvent::StartupEntryToggled(res));
            },
        );
    }

    /// Toggle any scheduled task Enabled / Disabled asynchronously.
    pub fn toggle_live_task_async(&mut self, task_path: String, enable: bool) {
        if self.startup_mutating {
            return;
        }
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::ScheduledTaskRunResult(Err(format!("Task worker panic: {}", msg))),
            move |tx| {
                let prev_enabled = backend.get_task_enabled(&task_path).unwrap_or(!enable);
                let res = match backend.set_scheduled_task_enabled(&task_path, enable) {
                    Ok(()) => {
                        if let Ok(l) = StateLedger::new(&ledger_path) {
                            let _ = l.record_change(
                                "scheduled_task_toggle",
                                &format!("ScheduledTask:{}", task_path),
                                &prev_enabled.to_string(),
                                &enable.to_string(),
                            );
                        }
                        Ok(format!(
                            "Task '{}' {}.",
                            task_path,
                            if enable { "enabled" } else { "disabled" }
                        ))
                    }
                    Err(e) => Err(format!("Task error '{}': {}", task_path, e)),
                };
                let _ = tx.send(WorkerEvent::ScheduledTaskRunResult(res));
            },
        );
    }

    /// Run a scheduled task immediately via backend.
    pub fn run_live_task_async(&mut self, task_path: String) {
        if self.startup_mutating {
            return;
        }
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();
        let backend = Arc::clone(&self.backend);
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::ScheduledTaskRunResult(Err(format!("Task run panic: {}", msg))),
            move |tx| {
                let res = backend
                    .run_scheduled_task(&task_path)
                    .map(|()| format!("Triggered scheduled task '{}'.", task_path))
                    .map_err(|e| format!("Failed to run task '{}': {}", task_path, e));
                let _ = tx.send(WorkerEvent::ScheduledTaskRunResult(res));
            },
        );
    }

    /// Toggle selection of a startup entry for batch operations.
    pub fn toggle_startup_entry_selection(&mut self, id: String) {
        if !self.selected_startup_entries.remove(&id) {
            self.selected_startup_entries.insert(id);
        }
    }

    /// Select all startup entries matching query, hide_microsoft, and status filter (Invariant I1).
    pub fn select_all_filtered_startup(
        &mut self,
        query: &str,
        hide_ms: bool,
        filter: StartupStatusFilter,
    ) {
        let q = query.trim().to_lowercase();
        self.selected_startup_entries.clear();
        for e in &self.startup_entries {
            if hide_ms && e.is_microsoft {
                continue;
            }
            let status_ok = match filter {
                StartupStatusFilter::All => true,
                StartupStatusFilter::ActiveOnly => e.enabled,
                StartupStatusFilter::InactiveOnly => !e.enabled,
            };
            if status_ok
                && (q.is_empty()
                    || e.name.to_lowercase().contains(&q)
                    || e.command.to_lowercase().contains(&q))
            {
                self.selected_startup_entries.insert(e.id.clone());
            }
        }
    }

    /// Deselect all startup entries.
    pub fn deselect_all_startup(&mut self) {
        self.selected_startup_entries.clear();
    }

    /// Batch enable or disable selected startup entries asynchronously via worker.
    pub fn batch_toggle_selected_startup_async(&mut self, enable: bool) {
        if self.startup_mutating {
            return;
        }
        let ids: Vec<String> = self.selected_startup_entries.iter().cloned().collect();
        if ids.is_empty() {
            return;
        }
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();

        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        let entries = self.startup_entries.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| {
                WorkerEvent::StartupBatchToggled(Err(format!(
                    "Batch startup worker panic: {}",
                    msg
                )))
            },
            move |tx| {
                let mut updated = 0;
                let ledger = StateLedger::new(&ledger_path).ok();
                for id in &ids {
                    if let Some(entry) = entries.iter().find(|e| &e.id == id) {
                        let prev_enabled = entry.enabled;
                        if backend.set_startup_entry_enabled(entry, enable).is_ok() {
                            updated += 1;
                            if let Some(ref l) = ledger {
                                let _ = l.record_change(
                                    &format!("startup_{}", entry.name),
                                    &format!(
                                        "StartupApproved:{}",
                                        crate::core::reg_helpers::escape_pipe(&entry.id)
                                    ),
                                    &prev_enabled.to_string(),
                                    &enable.to_string(),
                                );
                            }
                        }
                    }
                }
                let _ = tx.send(WorkerEvent::StartupBatchToggled(Ok((updated, enable))));
            },
        );
    }

    /// Toggle selection of a scheduled task for batch operations.
    pub fn toggle_scheduled_task_selection(&mut self, path: String) {
        if !self.selected_scheduled_tasks.remove(&path) {
            self.selected_scheduled_tasks.insert(path);
        }
    }

    /// Select all scheduled tasks matching query, hide_microsoft, and status filter (Invariant I1).
    pub fn select_all_filtered_tasks(
        &mut self,
        query: &str,
        hide_ms: bool,
        filter: StartupStatusFilter,
    ) {
        let q = query.trim().to_lowercase();
        self.selected_scheduled_tasks.clear();
        for t in &self.scheduled_tasks {
            if hide_ms && t.is_microsoft {
                continue;
            }
            let status_ok = match filter {
                StartupStatusFilter::All => true,
                StartupStatusFilter::ActiveOnly => t.enabled,
                StartupStatusFilter::InactiveOnly => !t.enabled,
            };
            if status_ok
                && (q.is_empty()
                    || t.task_name.to_lowercase().contains(&q)
                    || t.task_path.to_lowercase().contains(&q))
            {
                self.selected_scheduled_tasks.insert(t.task_path.clone());
            }
        }
    }

    /// Deselect all scheduled tasks.
    pub fn deselect_all_tasks(&mut self) {
        self.selected_scheduled_tasks.clear();
    }

    /// Batch enable or disable selected scheduled tasks asynchronously via worker.
    pub fn batch_toggle_selected_tasks_async(&mut self, enable: bool) {
        if self.startup_mutating {
            return;
        }
        let paths: Vec<String> = self.selected_scheduled_tasks.iter().cloned().collect();
        if paths.is_empty() {
            return;
        }
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();

        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::TasksBatchToggled(Err(format!("Batch tasks worker panic: {}", msg))),
            move |tx| {
                let mut updated = 0;
                let ledger = StateLedger::new(&ledger_path).ok();
                for task_path in &paths {
                    let prev_enabled = backend.get_task_enabled(task_path).unwrap_or(!enable);
                    if backend
                        .set_scheduled_task_enabled(task_path, enable)
                        .is_ok()
                    {
                        updated += 1;
                        if let Some(ref l) = ledger {
                            let _ = l.record_change(
                                "scheduled_task_batch_toggle",
                                &format!("ScheduledTask:{}", task_path),
                                &prev_enabled.to_string(),
                                &enable.to_string(),
                            );
                        }
                    }
                }
                let _ = tx.send(WorkerEvent::TasksBatchToggled(Ok((updated, enable))));
            },
        );
    }

    /// Update a live Win32 service startup mode (`2=Auto, 3=Manual, 4=Disabled`) with Core OS service protection.
    pub fn set_live_service_start_async(&mut self, service_name: String, start_type: u32) {
        if start_type == 4 && crate::tools::startup::is_protected_core_service(&service_name) {
            self.set_status_spec(StatusMessageSpec::ServiceCriticalRefused(service_name));
            return;
        }
        if self.startup_mutating {
            return;
        }
        self.startup_mutating = true;
        self.startup_tasks_inflight += 1;
        self.sync_startup_tasks_busy();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();
        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::ServiceLiveUpdated(Err(format!("Service worker panic: {}", msg))),
            move |tx| {
                let res = match backend.get_service_start_type(&service_name) {
                    None => Err(format!(
                        "Service '{}' not found or inaccessible.",
                        service_name
                    )),
                    Some(prev) => match backend.set_service_start_type(&service_name, start_type) {
                        Ok(()) => {
                            if let Ok(l) = StateLedger::new(&ledger_path) {
                                let _ = l.record_change(
                                    &format!("service_{}", service_name),
                                    &format!("ServiceState:{}", service_name),
                                    &prev.to_string(),
                                    &start_type.to_string(),
                                );
                            }
                            Ok(format!(
                                "Service '{}' start mode set to {}.",
                                service_name, start_type
                            ))
                        }
                        Err(e) => Err(format!("Service '{}' error: {}", service_name, e)),
                    },
                };
                let _ = tx.send(WorkerEvent::ServiceLiveUpdated(res));
            },
        );
    }

    /// Export setup-safe catalog tweaks and OOBE/TPM bypasses into `autounattend.xml` alongside profile storage.
    pub fn export_autounattend_xml(&mut self) {
        let selected_refs: Vec<&TweakDefinition> = if self.selected_tweaks.is_empty() {
            self.tweaks
                .iter()
                .filter(|t| t.unattend_safe && t.valuation.stability_risk < RiskLevel::High)
                .collect()
        } else {
            self.tweaks
                .iter()
                .filter(|t| self.selected_tweaks.contains(&t.id) && t.unattend_safe)
                .collect()
        };

        let xml = generate_autounattend_xml(&self.unattend_opts, &selected_refs);
        let dest = self.profile_path.with_file_name("autounattend.xml");
        if let Some(parent) = dest.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match std::fs::write(&dest, &xml) {
            Ok(()) => {
                let disp = dest.display().to_string();
                self.unattend_last_export = Some(disp.clone());
                self.set_status_spec(StatusMessageSpec::UnattendExported {
                    bytes: xml.len(),
                    path: disp,
                });
            }
            Err(e) => {
                self.set_status_spec(StatusMessageSpec::UnattendExportFailed(e.to_string()));
            }
        }
    }

    /// Clear staged AppX bloat removal selection.
    pub fn deselect_all_appx(&mut self) {
        self.selected_appx.clear();
    }

    /// Clear staged WinGet package selection.
    pub fn deselect_all_winget(&mut self) {
        self.selected_winget.clear();
    }

    /// Reset captured latency benchmark snapshots cleanly.
    pub fn reset_latency_benchmark(&mut self) {
        self.latency_before = None;
        self.latency_after = None;
        self.latest_latency_bench = None;
    }

    /// Clear search input and category filter in Advanced Matrix.
    pub fn clear_advanced_filters(&mut self) {
        self.search_edit = TextEdit::default();
        self.search_query.clear();
        self.last_synced_search.clear();
        self.selected_category = None;
    }

    /// Apply a quick configuration preset for `autounattend.xml` export.
    pub fn apply_unattend_preset(&mut self, preset: &str) {
        match preset {
            "minimal" => {
                self.unattend_opts.bypass_tpm_ram_requirements = true;
                self.unattend_opts.bypass_msa_oobe = true;
                self.unattend_opts.skip_oobe_privacy_screens = false;
                self.unattend_opts.include_selected_tweaks = false;
                self.unattend_opts.auto_timezone_utc = false;
            }
            "privacy" => {
                self.unattend_opts.bypass_tpm_ram_requirements = true;
                self.unattend_opts.bypass_msa_oobe = true;
                self.unattend_opts.skip_oobe_privacy_screens = true;
                self.unattend_opts.include_selected_tweaks = true;
                self.unattend_opts.auto_timezone_utc = false;
            }
            "gaming" => {
                self.unattend_opts.bypass_tpm_ram_requirements = true;
                self.unattend_opts.bypass_msa_oobe = true;
                self.unattend_opts.skip_oobe_privacy_screens = true;
                self.unattend_opts.include_selected_tweaks = true;
                self.unattend_opts.auto_timezone_utc = true;
            }
            _ => {}
        }
    }

    // ── v1.4 Game Prerequisites & Automation Methods ──────────────────────────

    pub fn scan_game_prereqs(&mut self) {
        self.is_prereqs_scanning = true;
        self.game_prereqs = crate::tools::game_prereqs::scan_game_prereqs();
        self.is_prereqs_scanning = false;
        if let Some(ref w) = self.waker {
            w();
        }
    }

    pub fn install_game_prereq(&mut self, prereq_id: String) {
        if self.prereq_install_in_flight.is_some() {
            return;
        }
        self.prereq_install_in_flight = Some(prereq_id.clone());
        if let Some(item) = self.game_prereqs.iter_mut().find(|p| p.id == prereq_id) {
            let _ = crate::tools::game_prereqs::install_prereq_cascade(item);
        }
        self.prereq_install_in_flight = None;
        if let Some(ref w) = self.waker {
            w();
        }
    }

    pub fn install_all_missing_prereqs(&mut self) {
        if self.prereq_install_in_flight.is_some() {
            return;
        }
        let missing_ids: Vec<String> = self
            .game_prereqs
            .iter()
            .filter(|p| !p.installed)
            .map(|p| p.id.clone())
            .collect();

        for id in missing_ids {
            self.install_game_prereq(id);
        }
    }

    pub fn refresh_system_health(&mut self) {
        let assessment = crate::tools::automation::assess_system_health();
        self.system_health = Some(assessment.clone());
        self.system_health_assessment = Some(assessment);
        if let Some(ref w) = self.waker {
            w();
        }
    }

    pub fn refresh_maintenance_tasks(&mut self) {
        self.maintenance_tasks = crate::tools::automation::query_all_maintenance_tasks();
        if let Some(ref w) = self.waker {
            w();
        }
    }

    pub fn toggle_maintenance_task(&mut self, kind: crate::tools::automation::MaintenanceTaskKind) {
        let is_installed = crate::tools::automation::is_maintenance_task_installed(kind);
        if is_installed {
            let _ = crate::tools::automation::remove_maintenance_task(kind);
        } else {
            let _ = crate::tools::automation::install_maintenance_task(kind);
        }
        self.refresh_maintenance_tasks();
    }

    pub fn generate_auto_tune_plan(&mut self, profile: crate::tools::automation::AutoTuneProfile) {
        self.selected_auto_tune_profile = profile;
        let plan = crate::tools::automation::create_auto_tune_plan(&self.tweaks, profile);
        self.auto_tune_plan = Some(plan);
        if let Some(ref w) = self.waker {
            w();
        }
    }

    pub fn execute_auto_tune(&mut self) {
        let profile = self.selected_auto_tune_profile;
        self.execute_auto_tune_profile(profile);
    }

    pub fn assess_system_health(&mut self) {
        self.refresh_system_health();
    }

    pub fn execute_auto_tune_profile(
        &mut self,
        profile: crate::tools::automation::AutoTuneProfile,
    ) {
        if self.is_running_automation {
            return;
        }
        self.is_running_automation = true;
        let plan = crate::tools::automation::create_auto_tune_plan(&self.tweaks, profile);
        let tweaks = self.tweaks.clone();
        let backend = Arc::clone(&self.backend);
        let ledger_path = self.ledger_path.clone();

        spawn_guarded_worker(
            self.tx_events.clone(),
            self.waker.clone(),
            |msg| WorkerEvent::AutoTuneCompleted(Err(format!("Auto-Tune worker panic: {}", msg))),
            move |tx| {
                let start = std::time::Instant::now();
                let mut applied = 0;
                let mut failed = 0;
                let ledger = crate::backup::state_ledger::StateLedger::new(&ledger_path).ok();
                for id in &plan.recommended_tweak_ids {
                    if let Some(tweak) = tweaks.iter().find(|t| &t.id == id) {
                        if let Some(ref l) = ledger {
                            match crate::executor::engine::apply_tweak_with(tweak, l, &*backend) {
                                Ok(_) => applied += 1,
                                Err(_) => failed += 1,
                            }
                        } else {
                            failed += 1;
                        }
                    }
                }
                let duration_ms = start.elapsed().as_millis() as u64;
                let res = crate::tools::automation::AutoTuneResult {
                    profile,
                    tweaks_applied: applied,
                    tweaks_failed: failed,
                    duration_ms,
                    message: format!(
                        "Successfully applied {} optimizations in {} ms ({} failed/skipped).",
                        applied, duration_ms, failed
                    ),
                };
                let _ = tx.send(WorkerEvent::AutoTuneCompleted(Ok(res)));
            },
        );
    }

    pub fn trim_system_memory_and_cache(&mut self) {
        self.execute_memory_trim();
    }

    pub fn execute_memory_trim(&mut self) {
        self.is_running_automation = true;
        let res = crate::tools::automation::trim_system_memory_and_cache();
        self.last_memory_trim_result = Some(res);
        self.is_running_automation = false;
        if let Some(ref w) = self.waker {
            w();
        }
    }
}
