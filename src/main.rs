//! WinBlast0 — Entry point, elevation verification, and application launch.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rune::AppSpec;
use winblast0::{app, core, ui};

fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    tracing::info!("WinBlast0 v{} starting up", env!("CARGO_PKG_VERSION"));

    // Verify Administrator elevation
    if !core::elevation::is_elevated() {
        tracing::warn!("Process is not elevated. Running in inspection mode.");
    } else {
        tracing::info!("Running with Administrator privileges");
    }

    let title = core::i18n::I18n::translate(
        core::i18n::Language::default(),
        core::i18n::TextKey::AppWindowTitle,
    );
    let mut spec = AppSpec::new(
        title,
        (1150.0, 780.0),
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

    // Themed scroll bars are a first-class part of the WinBlast UI, so they stay visible in every
    // overflowing viewport instead of following Windows' hide-until-scrolling system default.
    // An explicit `--scroll-bars MODE` on the command line still wins (e.g. for scripted captures).
    let mut argv: Vec<String> = std::env::args().skip(1).collect();
    if !argv.iter().any(|a| a == "--scroll-bars") {
        argv.push("--scroll-bars".to_string());
        argv.push("always".to_string());
    }
    rune::App::new(spec).args(argv).run();
}
