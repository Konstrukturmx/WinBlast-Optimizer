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

    rune::run(spec);
}
