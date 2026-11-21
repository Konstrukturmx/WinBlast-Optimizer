//! tests/ui_parity.rs
//! Automated Slint 1.19 Integration & UI Parity Tests
//!
//! Uses `i-slint-backend-testing` to verify visual tokens, multi-layer scaling,
//! theme propagation, and component lifecycle for WinBlast0 Slint migration.

#[cfg(not(feature = "ui-slint"))]
#[test]
fn test_ui_parity_skipped_when_ui_slint_disabled() {
    // Slint UI parity tests run when `--features ui-slint` is enabled.
}

#[cfg(feature = "ui-slint")]
mod slint_parity {
    use slint::ComponentHandle;
    use winblast0::ui_slint::{MainWindow, Motion, Scale, Theme};

    fn init_test_env() {
        // init_no_event_loop can be called multiple times
        i_slint_backend_testing::init_no_event_loop();
    }

    #[test]
    fn test_scale_token_bounds_and_typography_ramp() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        let scale = app.global::<Scale>();

        // Default factor is 1.0
        assert!((scale.get_factor() - 1.0).abs() < f32::EPSILON);
        let default_font_md = scale.get_font_md();
        assert!(default_font_md > 0.0);

        // Zoom to 1.5x
        scale.set_factor(1.5);
        assert!((scale.get_factor() - 1.5).abs() < f32::EPSILON);
        let zoomed_font_md = scale.get_font_md();
        assert!(
            zoomed_font_md > default_font_md,
            "Font size must scale up at 1.5x zoom: default={default_font_md}, zoomed={zoomed_font_md}"
        );

        // Verify titlebar and statusbar scale tokens
        let titlebar_h = scale.get_titlebar_h();
        assert!(titlebar_h > 0.0);
        let statusbar_h = scale.get_statusbar_h();
        assert!(statusbar_h > 0.0);

        // Reset to 1.0 for test cleanliness
        scale.set_factor(1.0);
    }

    #[test]
    fn test_theme_token_elevation_and_blur_fallback() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        let theme = app.global::<Theme>();

        // Default is dark mode with blur enabled
        assert!(theme.get_dark());
        assert!(theme.get_supports_blur());
        assert_eq!(theme.get_blur_radius(), 16.0);

        // Test software fallback simulation: supports_blur = false
        theme.set_supports_blur(false);
        assert!(!theme.get_supports_blur());
        assert_eq!(
            theme.get_blur_radius(),
            0.0,
            "Blur radius must collapse to 0px when software fallback is active"
        );

        // Test theme mode toggling
        theme.set_dark(false);
        assert!(!theme.get_dark());

        // Restore defaults
        theme.set_supports_blur(true);
        theme.set_dark(true);
    }

    #[test]
    fn test_motion_constants_monotonicity() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        let motion = app.global::<Motion>();
        let instant = motion.get_instant();
        let snappy = motion.get_snappy();
        let normal = motion.get_normal();
        let smooth = motion.get_smooth();
        let slow = motion.get_slow();

        assert!(instant < snappy, "instant must be faster than snappy");
        assert!(snappy < normal, "snappy must be faster than normal");
        assert!(normal < smooth, "normal must be faster than smooth");
        assert!(smooth < slow, "smooth must be faster than slow");
    }

    #[test]
    fn test_main_window_properties_and_callbacks() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        // Initial default properties
        assert_eq!(app.get_active_view(), "dashboard");
        assert!(app.get_is_elevated());
        assert_eq!(app.get_app_version(), "1.6.1");

        // Mutate properties
        app.set_active_view("tweaks".into());
        assert_eq!(app.get_active_view(), "tweaks");

        app.set_is_elevated(false);
        assert!(!app.get_is_elevated());

        // Callback registration and dispatch
        use std::cell::Cell;
        use std::rc::Rc;

        let navigated = Rc::new(Cell::new(false));
        let navigated_clone = navigated.clone();
        app.on_navigate(move |_target| {
            navigated_clone.set(true);
        });

        let closed = Rc::new(Cell::new(false));
        let closed_clone = closed.clone();
        app.on_close_window(move || {
            closed_clone.set(true);
        });

        let minimized = Rc::new(Cell::new(false));
        let minimized_clone = minimized.clone();
        app.on_minimize_window(move || {
            minimized_clone.set(true);
        });

        let maximized = Rc::new(Cell::new(false));
        let maximized_clone = maximized.clone();
        app.on_toggle_maximize(move || {
            maximized_clone.set(true);
        });

        app.invoke_navigate("settings".into());
        assert!(navigated.get(), "navigate callback must fire");

        app.invoke_close_window();
        assert!(closed.get(), "close_window callback must fire");

        app.invoke_minimize_window();
        assert!(minimized.get(), "minimize_window callback must fire");

        app.invoke_toggle_maximize();
        assert!(maximized.get(), "toggle_maximize callback must fire");
    }

    #[test]
    fn test_ui_slint_bridge_synchronization() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");
        let bridge = winblast0::ui_slint::bridge::SlintBridge::new(app.clone_strong());

        // Create a test AppState
        let state = winblast0::app::AppState::new_with_backend(
            std::sync::Arc::new(winblast0::core::backend::SandboxEnv::new()),
        );

        bridge.sync_from_state(&state);

        assert_eq!(app.get_app_version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(app.get_active_view(), state.active_view.name());
    }
}
