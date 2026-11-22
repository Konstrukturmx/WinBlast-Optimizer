//! Per-view binders. Each module owns one Slint global store and keeps it in sync with AppState.

pub mod backup;
pub mod clean_apps;
pub mod dashboard;
pub mod gpu;
pub mod hardware;
pub mod matrix;
pub mod privacy;
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
}

impl Views {
    pub fn new(window: &MainWindow) -> Self {
        Self { matrix: matrix::MatrixView::new(window), dashboard: dashboard::DashboardView::new(window), clean_apps: clean_apps::CleanAppsView::new(window), startup: startup::StartupView::new(window), backup: backup::BackupView::new(window), privacy: privacy::PrivacyView::new(window), gpu: gpu::GpuView::new(window), hardware: hardware::HardwareView::new(window) }
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
    }

    /// Sync the active view's store (inactive views are refreshed when navigated to).
    pub fn sync(&self, window: &MainWindow, _ctx: &Ctx, s: &AppState) {
        match s.active_view {
            ActiveView::AdvancedMatrix => self.matrix.sync(window, s),
            ActiveView::EasyDashboard => self.dashboard.sync(window, s),
            ActiveView::CleanAndApps => self.clean_apps.sync(window, s),
            ActiveView::StartupAndTasks => self.startup.sync(window, s),
            ActiveView::BackupCenter => self.backup.sync(window, s),
            ActiveView::PrivacyShield => self.privacy.sync(window, s),
            ActiveView::GpuTuning => self.gpu.sync(window, s),
            ActiveView::HardwareView => self.hardware.sync(window, s),
            _ => {}
        }
    }
}
