//! Per-view binders. Each module owns one Slint global store and keeps it in sync with AppState.

use slint::ComponentHandle as _;

use crate::app::AppState;

use super::ctx::Ctx;
use super::MainWindow;

/// Registry of all view binders.
pub struct Views {}

impl Views {
    pub fn new(_window: &MainWindow) -> Self {
        Self {}
    }

    pub fn bind(&self, _window: &MainWindow, _ctx: &Ctx) {}

    pub fn sync(&self, _window: &MainWindow, _ctx: &Ctx, _s: &AppState) {}
}

#[allow(dead_code)]
fn _unused(w: &MainWindow) {
    let _ = w.as_weak();
}
