//! Per-view binders. Each module owns one Slint global store and keeps it in sync with AppState.

pub mod clean_apps;
pub mod dashboard;
pub mod matrix;

use crate::app::{ActiveView, AppState};

use super::ctx::Ctx;
use super::MainWindow;

/// Registry of all view binders.
pub struct Views {
    matrix: matrix::MatrixView,
    dashboard: dashboard::DashboardView,
    clean_apps: clean_apps::CleanAppsView,
}

impl Views {
    pub fn new(window: &MainWindow) -> Self {
        Self { matrix: matrix::MatrixView::new(window), dashboard: dashboard::DashboardView::new(window), clean_apps: clean_apps::CleanAppsView::new(window) }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        self.matrix.bind(window, ctx);
        self.dashboard.bind(window, ctx);
        self.clean_apps.bind(window, ctx);
    }

    /// Sync the active view's store (inactive views are refreshed when navigated to).
    pub fn sync(&self, window: &MainWindow, _ctx: &Ctx, s: &AppState) {
        match s.active_view {
            ActiveView::AdvancedMatrix => self.matrix.sync(window, s),
            ActiveView::EasyDashboard => self.dashboard.sync(window, s),
            ActiveView::CleanAndApps => self.clean_apps.sync(window, s),
            _ => {}
        }
    }
}
