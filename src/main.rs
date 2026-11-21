//! WinBlast0 — Entry point, elevation verification, and application launch.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "ui-slint")]
fn run_slint() {
    use slint::ComponentHandle;
    tracing::info!("Initializing WinBlast0 Slint 1.19 Presentation Layer");
    let window = winblast0::ui_slint::SlintMainWindow::new()
        .expect("Failed to create Slint MainWindow");

    let win_handle = window.as_weak();
    window.on_close_window(move || {
        if let Some(w) = win_handle.upgrade() {
            w.hide().unwrap();
        }
        std::process::exit(0);
    });

    let win_handle2 = window.as_weak();
    window.on_minimize_window(move || {
        if let Some(w) = win_handle2.upgrade() {
            w.window().set_minimized(true);
        }
    });

    let win_handle3 = window.as_weak();
    window.on_toggle_maximize(move || {
        if let Some(w) = win_handle3.upgrade() {
            let is_max = w.window().is_maximized();
            w.window().set_maximized(!is_max);
        }
    });

    window.run().expect("Failed to run Slint event loop");
}

#[cfg(feature = "ui-rune")]
fn run_rune() {
    use rune::AppSpec;
    use winblast0::{app, ui};

    let title = concat!(
        "WinBlast0 v",
        env!("CARGO_PKG_VERSION"),
        " — Windows System Optimizer"
    );
    let initial_size = detect_screen_workarea();
    tracing::info!("Launching WinBlast0 Rune UI with detected work area: {}x{}", initial_size.0, initial_size.1);

    let mut spec = AppSpec::new(
        title,
        initial_size,
        app::AppState::new,
        ui::view,
    );
    spec.min_size = Some((420.0, 400.0));
    spec.setup = Some(app::AppState::setup);
    spec.poll = Some(app::AppState::poll);
    spec.scenes = &[
        ("gpu", |s: &mut app::AppState| {
            s.navigate(app::ActiveView::GpuTuning);
        }),
        ("cpu", |s: &mut app::AppState| {
            s.navigate(app::ActiveView::CoreParking);
        }),
        ("matrix", |s: &mut app::AppState| {
            s.navigate(app::ActiveView::AdvancedMatrix);
        }),
        ("dashboard", |s: &mut app::AppState| {
            s.navigate(app::ActiveView::EasyDashboard);
        }),
    ];

    let mut argv: Vec<String> = std::env::args().skip(1).collect();
    if !argv.iter().any(|a| a == "--scroll-bars") {
        argv.push("--scroll-bars".to_string());
        argv.push("always".to_string());
    }
    rune::App::new(spec)
        .maximized(true)
        .args(argv)
        .run();
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

    let args: Vec<String> = std::env::args().collect();
    let prefer_rune = args.iter().any(|a| a == "--ui=rune" || a == "--rune");

    #[cfg(feature = "ui-rune")]
    if prefer_rune {
        run_rune();
        return;
    }

    #[cfg(feature = "ui-slint")]
    run_slint();

    #[cfg(all(feature = "ui-rune", not(feature = "ui-slint")))]
    run_rune();
}

/// Automatically detect the primary display resolution and return the full desktop work area
/// (full width and height minus taskbar, avoiding exclusive gaming fullscreen).
#[cfg(windows)]
fn detect_screen_workarea() -> (f32, f32) {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETWORKAREA, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
        GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
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
