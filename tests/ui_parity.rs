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
        assert_eq!(app.get_app_version(), env!("CARGO_PKG_VERSION"));

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
        let state = winblast0::app::AppState::new_with_backend(std::sync::Arc::new(
            winblast0::core::backend::SandboxEnv::new(),
        ));

        bridge.sync_from_state(&state);

        assert_eq!(app.get_app_version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(app.get_active_view(), state.active_view.name());
    }

    #[test]
    fn test_all_26_views_navigation_and_mounting() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        let view_ids = [
            "dashboard",
            "matrix",
            "clean_apps",
            "startup_tasks",
            "gpu",
            "cpu_parking",
            "hardware",
            "thermal",
            "disk_health",
            "driver_bios",
            "latency",
            "fps_benchmark",
            "gaming_stats",
            "network_diag",
            "performance_tests",
            "game_profiles",
            "game_overlay",
            "game_suitability",
            "game_prereqs",
            "automation",
            "goal_planner",
            "windows_repair",
            "fault_diagnostics",
            "backup",
            "privacy",
            "support_about",
        ];

        for view in view_ids {
            app.set_active_view(view.into());
            assert_eq!(
                app.get_active_view(),
                view,
                "View navigation failed for {view}"
            );
        }
    }

    #[test]
    fn test_modals_state_and_lifecycle() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        // Diff modal
        assert!(!app.get_diff_modal_open());
        app.set_diff_modal_open(true);
        assert!(app.get_diff_modal_open());
        app.set_diff_modal_open(false);
        assert!(!app.get_diff_modal_open());

        // In-progress HUD
        assert!(!app.get_hud_active());
        app.set_hud_active(true);
        app.set_hud_task_title("Deep Optimization".into());
        app.set_hud_current_step("Applying BCD tweak...".into());
        app.set_hud_progress(45.5);

        assert!(app.get_hud_active());
        assert_eq!(app.get_hud_task_title(), "Deep Optimization");
        assert_eq!(app.get_hud_current_step(), "Applying BCD tweak...");
        assert!((app.get_hud_progress() - 45.5).abs() < f32::EPSILON);

        app.set_hud_active(false);
        assert!(!app.get_hud_active());
    }

    #[test]
    fn test_statusbar_telemetry_properties() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        app.set_cpu_usage(22.8);
        assert!((app.get_cpu_usage() - 22.8).abs() < f32::EPSILON);

        app.set_ram_usage(68.2);
        assert!((app.get_ram_usage() - 68.2).abs() < f32::EPSILON);

        app.set_ram_text("10.9 / 16.0 GB".into());
        assert_eq!(app.get_ram_text(), "10.9 / 16.0 GB");

        app.set_reboot_pending(true);
        assert!(app.get_reboot_pending());

        app.set_worker_active(true);
        app.set_worker_label("Scanning disk health...".into());
        assert!(app.get_worker_active());
        assert_eq!(app.get_worker_label(), "Scanning disk health...");

        use std::cell::Cell;
        use std::rc::Rc;
        let reboot_clicked = Rc::new(Cell::new(false));
        let rc_clone = reboot_clicked.clone();
        app.on_reboot_system(move || {
            rc_clone.set(true);
        });
        app.invoke_reboot_system();
        assert!(reboot_clicked.get(), "reboot_system callback must fire");
    }

    #[test]
    fn test_global_keyboard_shortcuts_and_actions() {
        init_test_env();
        let app = MainWindow::new().expect("Failed to instantiate Slint MainWindow");

        use std::cell::Cell;
        use std::rc::Rc;

        let search_fired = Rc::new(Cell::new(false));
        let sf_clone = search_fired.clone();
        app.on_search_triggered(move || {
            sf_clone.set(true);
        });

        let boost_fired = Rc::new(Cell::new(false));
        let bf_clone = boost_fired.clone();
        app.on_quick_boost_triggered(move || {
            bf_clone.set(true);
        });

        app.invoke_search_triggered();
        assert!(search_fired.get(), "search_triggered callback must fire");

        app.invoke_quick_boost_triggered();
        assert!(
            boost_fired.get(),
            "quick_boost_triggered callback must fire"
        );
    }
}
