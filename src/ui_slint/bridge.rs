//! State bridge between WinBlast0 core engine and Slint 1.19 UI thread.

use crate::app::AppState;
use super::SlintMainWindow;

pub struct SlintBridge {
    window: SlintMainWindow,
}

impl SlintBridge {
    pub fn new(window: SlintMainWindow) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &SlintMainWindow {
        &self.window
    }

    /// Synchronizes core AppState snapshot into Slint properties
    pub fn sync_from_state(&self, state: &AppState) {
        self.window.set_app_version(env!("CARGO_PKG_VERSION").into());
        self.window.set_is_elevated(crate::core::elevation::is_elevated());
        self.window.set_active_view(state.active_view.name().into());
    }
}
