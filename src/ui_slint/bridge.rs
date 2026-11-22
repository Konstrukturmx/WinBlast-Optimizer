//! SlintBridge: owns the window, the shared AppState handle, the shell binder and every view
//! binder, and drives the 100 ms UI tick (drain worker events -> sync stores -> persist prefs).

use std::cell::RefCell;
use std::rc::Rc;

use slint::ComponentHandle;

use crate::app::AppState;
use crate::core::settings::UserSettings;

use super::ctx::Ctx;
use super::shell::Shell;
use super::views::Views;
use super::{i18n, MainWindow};

/// Apply persisted preferences to a fresh AppState (before the first frame).
pub fn apply_settings(s: &mut AppState, cfg: &UserSettings) {
    s.set_language(cfg.language);
    s.set_theme_mode(cfg.theme_mode);
    s.set_color_scheme(cfg.color_scheme);
    s.set_theme_style(cfg.theme_style);
    s.set_telemetry_interval_ms(cfg.telemetry_interval_ms);
    s.sidebar_open = cfg.sidebar_open;
}

pub struct SlintBridge {
    window: MainWindow,
    ctx: Ctx,
    shell: Shell,
    views: Views,
    saved: RefCell<UserSettings>,
    persist: bool,
}

impl SlintBridge {
    /// Production constructor: preference changes are saved to disk.
    pub fn new(window: MainWindow, state: Rc<RefCell<AppState>>, settings: UserSettings) -> Rc<Self> {
        Self::build(window, state, settings, true)
    }

    /// Test/headless constructor: never writes the settings file.
    pub fn new_ephemeral(window: MainWindow, state: Rc<RefCell<AppState>>) -> Rc<Self> {
        Self::build(window, state, UserSettings::default(), false)
    }

    fn build(window: MainWindow, state: Rc<RefCell<AppState>>, settings: UserSettings, persist: bool) -> Rc<Self> {
        let ctx = Ctx::new(&window, state);
        let shell = Shell::new(&window, &settings);
        let views = Views::new(&window);
        let bridge = Rc::new(Self { window, ctx, shell, views, saved: RefCell::new(settings), persist });
        i18n::install(&bridge.window, &bridge.ctx);
        bridge.shell.bind(&bridge.window, &bridge.ctx);
        bridge.views.bind(&bridge.window, &bridge.ctx);
        bridge.sync();
        bridge
    }

    pub fn window(&self) -> &MainWindow {
        &self.window
    }

    pub fn ctx(&self) -> &Ctx {
        &self.ctx
    }

    /// Push the current AppState into every Slint store (active view gets full sync).
    pub fn sync(&self) {
        let Ok(s) = self.ctx.state.try_borrow() else {
            return; // a callback is mid-mutation; the next tick will sync
        };
        self.shell.sync(&self.window, &self.ctx, &s);
        self.views.sync(&self.window, &self.ctx, &s);
        self.ctx.dirty.set(false);
        let snapshot = self.shell.settings_snapshot(&s);
        drop(s);
        self.persist_if_changed(snapshot);
    }

    /// One UI tick: drain background worker events, then sync if anything changed.
    pub fn tick(&self) {
        let changed = match self.ctx.state.try_borrow_mut() {
            Ok(mut s) => s.poll_headless(),
            Err(_) => false,
        };
        if changed || self.ctx.dirty.get() {
            self.sync();
        }
    }

    fn persist_if_changed(&self, snapshot: UserSettings) {
        if self.persist && *self.saved.borrow() != snapshot {
            if let Err(e) = snapshot.save() {
                tracing::warn!("could not save UI settings: {e}");
            }
            *self.saved.borrow_mut() = snapshot;
        }
    }

    /// Start the repeating UI tick on the Slint event loop. Keep the returned Timer alive.
    pub fn start_ticking(self: &Rc<Self>) -> slint::Timer {
        let timer = slint::Timer::default();
        let weak = Rc::downgrade(self);
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
            if let Some(b) = weak.upgrade() {
                b.tick();
            }
        });
        timer
    }

    pub fn run(self: &Rc<Self>) -> Result<(), slint::PlatformError> {
        let _timer = self.start_ticking();
        self.window.run()
    }
}
