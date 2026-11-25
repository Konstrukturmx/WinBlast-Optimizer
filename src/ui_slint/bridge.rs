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
        let _tour = std::env::var_os("WINBLAST_SCREENSHOT_DIR").map(|dir| self.start_screenshot_tour(dir.into()));
        self.window.run()
    }

    /// Documentation helper: with `WINBLAST_SCREENSHOT_DIR` set, open each view named in
    /// `WINBLAST_SCREENSHOT_VIEWS` (comma-separated view ids, default: every view), wait
    /// `WINBLAST_SCREENSHOT_WAIT` seconds (default 6) for its read-only data, save the window as
    /// `NN-<view>.bmp` and quit. It only navigates; it never presses a button.
    fn start_screenshot_tour(self: &Rc<Self>, dir: std::path::PathBuf) -> slint::Timer {
        use crate::app::ActiveView;
        let views: Vec<ActiveView> = match std::env::var("WINBLAST_SCREENSHOT_VIEWS") {
            Ok(list) => list.split(',').filter_map(|n| crate::ui_slint::shell::view_from_name(n.trim())).collect(),
            Err(_) => ActiveView::ALL_VIEWS.to_vec(),
        };
        let wait: u32 = std::env::var("WINBLAST_SCREENSHOT_WAIT").ok().and_then(|v| v.parse().ok()).unwrap_or(6);
        let _ = std::fs::create_dir_all(&dir);
        let weak = Rc::downgrade(self);
        let step = std::cell::Cell::new(0u32);
        let timer = slint::Timer::default();
        // The first view also waits for the startup scan.
        let first_wait = wait.max(20);
        timer.start(slint::TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
            let Some(b) = weak.upgrade() else { return };
            let n = step.get();
            step.set(n + 1);
            let mut elapsed = n;
            for (i, view) in views.iter().enumerate() {
                let w = if i == 0 { first_wait } else { wait };
                if elapsed == 0 {
                    b.ctx.act(|s| s.navigate(*view));
                    return;
                }
                if elapsed == w {
                    let path = dir.join(format!("{:02}-{}.bmp", i + 1, view.name()));
                    match b.window.window().take_snapshot() {
                        Ok(buf) => {
                            if let Err(e) = write_bmp(&path, buf.width(), buf.height(), buf.as_bytes()) {
                                tracing::warn!("screenshot {}: {e}", path.display());
                            }
                        }
                        Err(e) => tracing::warn!("screenshot {}: {e}", path.display()),
                    }
                    return;
                }
                if elapsed < w {
                    return;
                }
                elapsed -= w + 1;
            }
            let _ = slint::quit_event_loop();
        });
        timer
    }
}

/// 32-bit top-down BMP from RGBA8 pixels (no image crate needed for the docs screenshots).
fn write_bmp(path: &std::path::Path, width: u32, height: u32, rgba: &[u8]) -> std::io::Result<()> {
    let data_len = width * height * 4;
    let mut out = Vec::with_capacity(54 + data_len as usize);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(54 + data_len).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(width as i32).to_le_bytes());
    out.extend_from_slice(&(-(height as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&32u16.to_le_bytes());
    out.extend_from_slice(&[0u8; 24]);
    for px in rgba.as_chunks::<4>().0 {
        out.extend_from_slice(&[px[2], px[1], px[0], 255]);
    }
    std::fs::write(path, out)
}
