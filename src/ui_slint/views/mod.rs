//! Per-view binders. Each module owns one Slint global store and keeps it in sync with AppState.

pub mod about;
pub mod automation;
pub mod backup;
pub mod clean_apps;
pub mod dashboard;
pub mod diagnostics;
pub mod drivers;
pub mod gaming;
pub mod gpu;
pub mod hardware;
pub mod latency;
pub mod matrix;
pub mod planner;
pub mod privacy;
pub mod sensors;
pub mod startup;

use crate::app::{ActiveView, AppState};

use super::ctx::Ctx;
use super::MainWindow;

/// Registry of all view binders.
pub struct Views {
    matrix: matrix::MatrixView,
    dashboard: dashboard::DashboardView,
    clean_apps: clean_apps::CleanAppsView,
    startup: startup::StartupView,
    backup: backup::BackupView,
    privacy: privacy::PrivacyView,
    gpu: gpu::GpuView,
    hardware: hardware::HardwareView,
    sensors: sensors::SensorsView,
    drivers: drivers::DriversView,
    latency: latency::LatencyView,
    planner: planner::PlannerView,
    automation: automation::AutomationView,
    diagnostics: diagnostics::DiagnosticsView,
    gaming: gaming::GamingView,
    about: about::AboutView,
}

impl Views {
    pub fn new(window: &MainWindow) -> Self {
        Self { matrix: matrix::MatrixView::new(window), dashboard: dashboard::DashboardView::new(window), clean_apps: clean_apps::CleanAppsView::new(window), startup: startup::StartupView::new(window), backup: backup::BackupView::new(window), privacy: privacy::PrivacyView::new(window), gpu: gpu::GpuView::new(window), hardware: hardware::HardwareView::new(window), sensors: sensors::SensorsView::new(window), drivers: drivers::DriversView::new(window), latency: latency::LatencyView::new(window), planner: planner::PlannerView::new(window), automation: automation::AutomationView::new(window), diagnostics: diagnostics::DiagnosticsView::new(window), gaming: gaming::GamingView::new(window), about: about::AboutView::new(window) }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        self.matrix.bind(window, ctx);
        self.dashboard.bind(window, ctx);
        self.clean_apps.bind(window, ctx);
        self.startup.bind(window, ctx);
        self.backup.bind(window, ctx);
        self.privacy.bind(window, ctx);
        self.gpu.bind(window, ctx);
        self.hardware.bind(window, ctx);
        self.sensors.bind(window, ctx);
        self.drivers.bind(window, ctx);
        self.latency.bind(window, ctx);
        self.planner.bind(window, ctx);
        self.automation.bind(window, ctx);
        self.diagnostics.bind(window, ctx);
        self.gaming.bind(window, ctx);
        self.about.bind(window, ctx);
    }

    /// Sync the active view's store (inactive views are refreshed when navigated to).
    pub fn sync(&self, window: &MainWindow, _ctx: &Ctx, s: &AppState) {
        if s.active_view != ActiveView::FaultDiagnostics {
            self.diagnostics.disarm();
        }
        match s.active_view {
            ActiveView::AdvancedMatrix => self.matrix.sync(window, s),
            ActiveView::EasyDashboard => self.dashboard.sync(window, s),
            ActiveView::CleanAndApps => self.clean_apps.sync(window, s),
            ActiveView::StartupAndTasks => self.startup.sync(window, s),
            ActiveView::BackupCenter => self.backup.sync(window, s),
            ActiveView::PrivacyShield => self.privacy.sync(window, s),
            ActiveView::GpuTuning => self.gpu.sync(window, s),
            ActiveView::HardwareView => self.hardware.sync(window, s),
            ActiveView::CoreParking => self.sensors.sync_parking(window, s),
            ActiveView::ThermalMonitor => self.sensors.sync_thermal(window, s),
            ActiveView::DiskHealth => self.sensors.sync_disks(window, s),
            ActiveView::DriverBios => self.drivers.sync(window, s),
            ActiveView::LatencyTester => self.latency.sync(window, s),
            ActiveView::GoalPlanner => self.planner.sync(window, s),
            ActiveView::AutomationCenter => self.automation.sync(window, s),
            ActiveView::FaultDiagnostics => self.diagnostics.sync_fault(window, s),
            ActiveView::WindowsRepair => self.diagnostics.sync_repair(window, s),
            ActiveView::PerformanceTests => self.diagnostics.sync_perf(window, s),
            ActiveView::GamingStats | ActiveView::FpsBenchmark | ActiveView::GameProfiles | ActiveView::GameOverlay | ActiveView::GameSuitability => self.gaming.sync_gaming(window, s),
            ActiveView::GamePrerequisites => self.gaming.sync_prereqs(window, s),
            ActiveView::NetworkDiag => self.gaming.sync_network(window, s),
            _ => {}
        }
    }
}
