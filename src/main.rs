//! WinBlast0 — Entry point, elevation verification, and application launch.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn run_slint() {
    use std::cell::RefCell;
    use std::rc::Rc;

    use slint::ComponentHandle;
    use winblast0::core::settings::UserSettings;
    use winblast0::ui_slint::bridge::{apply_settings, SlintBridge};

    tracing::info!("Initializing WinBlast0 Slint 1.19 presentation layer");
    let window = match winblast0::ui_slint::SlintMainWindow::new() {
        Ok(w) => w,
        Err(e) => {
            tracing::error!("Failed to create the main window: {e}");
            std::process::exit(1);
        }
    };

    // 3D depth effects only when a hardware GPU drives the renderer (and no software override).
    let software_forced = std::env::var("SLINT_BACKEND")
        .map(|b| b.to_ascii_lowercase().contains("software") || b.to_ascii_lowercase().ends_with("-sw"))
        .unwrap_or(false);
    let gpu = !software_forced && winblast0::hardware::gpu::hardware_render_adapter_present();
    tracing::info!("GPU depth effects: {}", if gpu { "on" } else { "off" });
    window.global::<winblast0::ui_slint::Fx>().set_gpu(gpu);

    let settings = UserSettings::load();
    let mut state = winblast0::app::AppState::new();
    apply_settings(&mut state, &settings);
    state.setup_slint();

    let (w, h) = detect_screen_workarea();
    window.window().set_size(slint::LogicalSize::new(w.min(1600.0), h.min(1000.0)));
    window.window().set_maximized(true);

    let state = Rc::new(RefCell::new(state));
    let bridge = SlintBridge::new(window.clone_strong(), state, settings);
    #[cfg(windows)]
    {
        let attach = slint::Timer::default();
        attach.start(slint::TimerMode::SingleShot, std::time::Duration::from_millis(300), || {
            winblast0::core::window_icon::attach_window_icon();
        });
        if let Err(e) = bridge.run() {
            tracing::error!("Slint event loop failed: {e}");
        }
        drop(attach);
    }
    #[cfg(not(windows))]
    if let Err(e) = bridge.run() {
        tracing::error!("Slint event loop failed: {e}");
    }
}

fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    tracing::info!("WinBlast0 v{} starting up", env!("CARGO_PKG_VERSION"));

    // Verify Administrator elevation
    if !winblast0::core::elevation::is_elevated() {
        tracing::warn!("Process is not elevated. Running in inspection mode.");
    } else {
        tracing::info!("Running with Administrator privileges");
    }

    run_slint();
}

/// Automatically detect the primary display resolution and return the full desktop work area
/// (full width and height minus taskbar, avoiding exclusive gaming fullscreen).
#[cfg(windows)]
fn detect_screen_workarea() -> (f32, f32) {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SystemParametersInfoW, SM_CXSCREEN, SM_CYSCREEN, SPI_GETWORKAREA,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };

    let mut rect = RECT::default();
    let success = unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut rect as *mut _ as *mut _),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };

    if success.is_ok() {
        let width = (rect.right - rect.left) as f32;
        let height = (rect.bottom - rect.top) as f32;
        if width >= 640.0 && height >= 480.0 {
            return (width, height);
        }
    }

    let cx = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let cy = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    if cx > 0 && cy > 0 {
        return (cx as f32, (cy as f32 - 48.0).max(480.0));
    }

    (1150.0, 780.0)
}

#[cfg(not(windows))]
fn detect_screen_workarea() -> (f32, f32) {
    (1150.0, 780.0)
}
