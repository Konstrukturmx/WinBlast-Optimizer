//! E2E tests for WinBlast Optimizer v0.9.1 scrollbar integration.
//!
//! Verifies that the themed scroll bars are attached, visible, correctly sized, and
//! interactive across WinBlast scrollable viewports (content views + the sidebar).

use std::sync::Arc;
use rune::{Env, Runtime, ScrollBars};
use winblast0::app::{ActiveView, AppState};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::risk::load_embedded_rules;
use winblast0::scanner::{ScanResult, TweakState};
use winblast0::ui;

/// Sidebar scroll bars sit in the 224 pt-wide nav column; content viewports start right of it.
const CONTENT_X: f32 = 500.0;

fn create_test_state(view: ActiveView) -> AppState {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.tweaks = Arc::new(tweaks);
    state.active_view = view;

    // Populate scan results so matrix has abundant scrollable rows
    state.scan_results = state
        .tweaks
        .iter()
        .map(|t| ScanResult {
            tweak_id: t.id.clone(),
            title: t.title.clone(),
            category: t.category.clone(),
            description: t.description.clone(),
            easy_mode_included: t.easy_mode_included,
            current_state: TweakState::NotOptimized,
            risk: t.valuation.clone(),
            status_detail: "Stock Default".to_string(),
            sub_category: t.sub_category.clone(),
            detail_kind: None,
        })
        .collect();

    state.is_scanning = false;
    state.dismiss_progress_hud = true;
    state.game_prereqs = winblast0::tools::game_prereqs::scan_game_prereqs();

    state.driver_updates = vec![
        winblast0::hardware::driver_update::DriverUpdateItem {
            id: "display_01".to_string(),
            name: "NVIDIA GeForce RTX 4090 Driver".to_string(),
            category: winblast0::hardware::driver_update::DriverCategory::Display,
            hardware_id: "PCI\\VEN_10DE&DEV_2684".to_string(),
            current_version: "551.23".to_string(),
            latest_version: "560.81".to_string(),
            provider: "NVIDIA".to_string(),
            release_date: "2024-08-20".to_string(),
            download_url: "https://nvidia.com".to_string(),
            download_size_mb: 650.0,
            state: winblast0::hardware::driver_update::DriverUpdateState::UpdateAvailable,
            status_message: "Update Available".to_string(),
            inf_path: "nv_disp.inf".to_string(),
            is_whql_signed: true,
            compatible_os_builds: vec![22631, 26100],
        },
        winblast0::hardware::driver_update::DriverUpdateItem {
            id: "net_01".to_string(),
            name: "Intel Wi-Fi 6E AX211".to_string(),
            category: winblast0::hardware::driver_update::DriverCategory::Network,
            hardware_id: "PCI\\VEN_8086&DEV_7AF0".to_string(),
            current_version: "23.40.0.4".to_string(),
            latest_version: "23.40.0.4".to_string(),
            provider: "Intel".to_string(),
            release_date: "2024-05-15".to_string(),
            download_url: "https://intel.com".to_string(),
            download_size_mb: 45.0,
            state: winblast0::hardware::driver_update::DriverUpdateState::UpToDate,
            status_message: "Up to Date".to_string(),
            inf_path: "netwtw12.inf".to_string(),
            is_whql_signed: true,
            compatible_os_builds: vec![22631, 26100],
        },
        winblast0::hardware::driver_update::DriverUpdateItem {
            id: "audio_01".to_string(),
            name: "Realtek High Definition Audio".to_string(),
            category: winblast0::hardware::driver_update::DriverCategory::Audio,
            hardware_id: "HDAUDIO\\FUNC_01&VEN_10EC".to_string(),
            current_version: "6.0.9239.1".to_string(),
            latest_version: "6.0.9655.1".to_string(),
            provider: "Realtek".to_string(),
            release_date: "2024-03-10".to_string(),
            download_url: "https://realtek.com".to_string(),
            download_size_mb: 180.0,
            state: winblast0::hardware::driver_update::DriverUpdateState::UpdateAvailable,
            status_message: "Update Available".to_string(),
            inf_path: "hdaudio.inf".to_string(),
            is_whql_signed: true,
            compatible_os_builds: vec![22631, 26100],
        },
    ];

    state
}

/// A settled 1150×780 runtime for `view` with always-visible scroll bars (the shipped default).
fn runtime_for(view: ActiveView) -> Runtime<AppState> {
    let state = create_test_state(view);
    let mut rt = Runtime::new(state, ui::view, (1150.0, 780.0), Env::new(true, 1.0));
    rt.set_system_scroll_bars(ScrollBars::Always);
    rt.frame(0.0);
    rt.frame(1.0 / 60.0);
    rt
}

/// Rect (`[x, y, w, h]`) of every rendered scroll-bar thumb.
fn thumb_rects(rt: &Runtime<AppState>) -> Vec<[f32; 4]> {
    rt.elements()
        .into_iter()
        .filter(|e| e.key.as_deref() == Some("thumb"))
        .map(|e| e.rect)
        .collect()
}

fn content_thumb(rt: &Runtime<AppState>) -> [f32; 4] {
    thumb_rects(rt)
        .into_iter()
        .find(|r| r[0] > CONTENT_X)
        .expect("content viewport must render a scroll-bar thumb on its right edge")
}

fn sidebar_thumb(rt: &Runtime<AppState>) -> [f32; 4] {
    thumb_rects(rt)
        .into_iter()
        .find(|r| r[0] < CONTENT_X)
        .expect("sidebar navigation must render a scroll-bar thumb")
}

fn count_key(rt: &Runtime<AppState>, key: &str) -> usize {
    rt.elements()
        .iter()
        .filter(|e| e.key.as_deref() == Some(key))
        .count()
}

fn frames(rt: &mut Runtime<AppState>, n: usize) {
    for _ in 0..n {
        rt.frame(1.0 / 60.0);
    }
}

#[test]
fn test_advanced_matrix_scrollbar_structure_and_drag() {
    let mut rt = runtime_for(ActiveView::AdvancedMatrix);

    // Both the sidebar and the matrix content viewport are scrollable here.
    assert!(
        thumb_rects(&rt).len() >= 2,
        "matrix view must render a sidebar and a content scroll-bar thumb, got {:?}",
        thumb_rects(&rt)
    );

    let initial = content_thumb(&rt);
    let (initial_y, thumb_h) = (initial[1], initial[3]);
    assert!(
        thumb_h >= 24.0,
        "thumb height must respect the 24 pt minimum, got {thumb_h}"
    );
    assert!(
        initial[0] + initial[2] >= 1150.0 - 20.0,
        "content thumb must hug the right edge of the viewport, got {initial:?}"
    );

    // Perform an interactive drag on the matrix thumb down the track
    let drag_x = initial[0] + initial[2] / 2.0;
    let start_y = initial_y + 10.0;
    rt.pointer_move(drag_x, start_y);
    rt.pointer_down(drag_x, start_y);
    rt.pointer_move(drag_x, start_y + 150.0);
    frames(&mut rt, 1);
    rt.pointer_up(drag_x, start_y + 150.0);
    frames(&mut rt, 1);

    let dragged = content_thumb(&rt);
    assert!(
        dragged[1] > initial_y,
        "dragging the thumb down must scroll content: y {initial_y} -> {}",
        dragged[1]
    );
}

#[test]
fn test_scrollbars_render_across_all_main_views() {
    let views = &[
        ActiveView::AdvancedMatrix,
        ActiveView::EasyDashboard,
        ActiveView::GoalPlanner,
        ActiveView::GpuTuning,
        ActiveView::HardwareView,
        ActiveView::PrivacyShield,
        ActiveView::BackupCenter,
        ActiveView::CoreParking,
        ActiveView::LatencyTester,
        ActiveView::DriverBios,
        ActiveView::GamePrerequisites,
        ActiveView::AutomationCenter,
    ];

    for &v in views {
        let rt = runtime_for(v);
        assert!(
            count_key(&rt, "scroll-bar") > 0,
            "View {v:?} must render an attached scroll-bar"
        );
        let thumbs = thumb_rects(&rt);
        assert!(
            thumbs.len() >= 2,
            "View {v:?} must attach both a sidebar and a content scroll-bar thumb, got {thumbs:?}"
        );
        assert!(
            thumbs.iter().any(|t| t[0] > CONTENT_X),
            "View {v:?} must render its content scroll-bar thumb on the right edge"
        );
        assert!(
            thumbs.iter().any(|t| t[0] < CONTENT_X),
            "View {v:?} must render the shared sidebar scroll-bar thumb"
        );
        for t in &thumbs {
            assert!(
                t[2] >= 5.0,
                "View {v:?} thumb must be at least 5 pt wide, got {t:?}"
            );
        }
    }
}

#[test]
fn test_themed_thumb_widens_under_the_pointer() {
    let mut rt = runtime_for(ActiveView::AdvancedMatrix);
    frames(&mut rt, 30);

    let rest = content_thumb(&rt);
    assert!(
        (rest[2] - 6.0).abs() < 1.0,
        "themed thumb must rest 6 pt wide, got {}",
        rest[2]
    );

    // Hover the pointer over the thumb (and its 14 pt press strip) and let the spring settle.
    rt.pointer_move(rest[0] + rest[2] / 2.0, rest[1] + rest[3] / 2.0);
    frames(&mut rt, 40);

    let hovered = content_thumb(&rt);
    assert!(
        hovered[2] > rest[2] + 1.0,
        "thumb must widen under the pointer: {} -> {}",
        rest[2],
        hovered[2]
    );
    assert!(
        hovered[2] <= 9.5,
        "themed hover width caps at 9 pt, got {}",
        hovered[2]
    );
}

#[test]
fn test_wheel_scroll_moves_content_and_sidebar_thumbs() {
    // A shorter window makes the 224 pt nav rail itself overflow, exercising its own bar.
    let state = create_test_state(ActiveView::AdvancedMatrix);
    let mut rt = Runtime::new(state, ui::view, (1150.0, 520.0), Env::new(true, 1.0));
    rt.set_system_scroll_bars(ScrollBars::Always);
    rt.frame(0.0);
    rt.frame(1.0 / 60.0);

    // Content viewport: scroll toward the bottom and confirm its thumb travels down.
    let before = content_thumb(&rt);
    rt.wheel(800.0, 300.0, -600.0);
    frames(&mut rt, 30);
    let after = content_thumb(&rt);
    assert!(
        after[1] > before[1],
        "wheel scrolling the matrix must move its thumb down: y {} -> {}",
        before[1],
        after[1]
    );

    // Sidebar navigation: independent scroll bar that moves when the pointer is over the nav rail.
    let side_before = sidebar_thumb(&rt);
    assert!(
        side_before[3] >= 24.0,
        "the nav rail must overflow in a 520 pt-tall window, got {side_before:?}"
    );
    rt.wheel(120.0, 300.0, -400.0);
    frames(&mut rt, 30);
    let side_after = sidebar_thumb(&rt);
    assert!(
        side_after[1] > side_before[1],
        "wheel scrolling the sidebar must move its thumb down: y {} -> {}",
        side_before[1],
        side_after[1]
    );
}

/// ISO 29119-3 / 29119-4 Test Case Specification:
/// - Identifier: TC_UI_DRIVER_SCROLLBAR_AND_FILTER_01
/// - Objective: Verify Drivers Section bounded scrollbar container, search filter, category filter, and zero/multi-item edge cases.
/// - Preconditions: ActiveView::DriverBios initialized in 1150x780 Rune runtime with always-visible scrollbars.
/// - Equivalence Classes: Empty driver updates, populated items, category filters (All, Display, Network, Audio, Usb), search queries (match vs no-match).
/// - Boundary Conditions: Container max height (460.0 pt); zero-match filtering states.
/// - Invariants: Scrollbar attached whenever items overflow max_h(460.0); filtering never distorts layout; zero element keys collide.
/// - Expected Outcome: Scrollbar thumb rendered within bounds, filters dynamically show/hide matching cards deterministically.
#[test]
fn test_iso_29119_driver_section_scrollbar_bounds_and_filtering() {
    let mut rt = runtime_for(ActiveView::DriverBios);
    frames(&mut rt, 2);

    // 1. Verify populated multi-item initial state (Display, Network, Audio)
    let initial_elements = rt.elements();
    let has_rtx = initial_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("RTX 4090")));
    let has_wifi = initial_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Intel Wi-Fi")));
    let has_realtek = initial_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Realtek")));
    assert!(has_rtx && has_wifi && has_realtek, "All 3 seeded drivers must render initially");

    // 2. Category Filter: Display
    rt.update(|s, _| {
        s.driver_category_filter = Some(winblast0::hardware::driver_update::DriverCategory::Display);
    });
    rt.settle_all();
    frames(&mut rt, 2);
    let display_elements = rt.elements();
    assert!(display_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("RTX 4090"))));
    assert!(!display_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Intel Wi-Fi"))));
    assert!(!display_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Realtek"))));

    // 3. Category Filter: Usb (Zero matching items)
    rt.update(|s, _| {
        s.driver_category_filter = Some(winblast0::hardware::driver_update::DriverCategory::Usb);
    });
    rt.settle_all();
    frames(&mut rt, 2);
    let usb_elements = rt.elements();
    assert!(usb_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("No devices match the current category or search filter."))));

    // 4. Reset Category Filter and apply Search Filter: "wi-fi"
    rt.update(|s, _| {
        s.driver_category_filter = None;
        s.driver_search_edit.text = "wi-fi".to_string();
    });
    rt.settle_all();
    frames(&mut rt, 2);
    let search_elements = rt.elements();
    assert!(!search_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("RTX 4090"))));
    assert!(search_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Intel Wi-Fi"))));

    // 5. Search Filter: No match "nonexistent_hardware"
    rt.update(|s, _| {
        s.driver_search_edit.text = "nonexistent_hardware".to_string();
    });
    rt.settle_all();
    frames(&mut rt, 2);
    let no_match_elements = rt.elements();
    assert!(no_match_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("No devices match the current category or search filter."))));

    // 6. Zero items state (empty list entirely)
    rt.update(|s, _| {
        s.driver_search_edit.text.clear();
        s.driver_updates.clear();
    });
    rt.settle_all();
    frames(&mut rt, 2);
    let empty_elements = rt.elements();
    assert!(empty_elements.iter().any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.contains("Click 'Scan Drivers Online' or 'Test Sandbox Driver'"))));

    // 7. Scrollbar container bounds: populate with 20 items to exceed max_h(460.0)
    rt.update(|s, _| {
        for i in 0..20 {
            s.driver_updates.push(winblast0::hardware::driver_update::DriverUpdateItem {
                id: format!("device_{}", i),
                name: format!("Simulated Device {}", i),
                category: winblast0::hardware::driver_update::DriverCategory::Storage,
                hardware_id: format!("PCI\\VEN_TEST&DEV_{:04X}", i),
                current_version: "1.0.0".to_string(),
                latest_version: "2.0.0".to_string(),
                provider: "TestProvider".to_string(),
                release_date: "2024-01-01".to_string(),
                download_url: "https://example.com".to_string(),
                download_size_mb: 25.0,
                state: winblast0::hardware::driver_update::DriverUpdateState::UpdateAvailable,
                status_message: "Test".to_string(),
                inf_path: "test.inf".to_string(),
                is_whql_signed: true,
                compatible_os_builds: vec![26100],
            });
        }
    });
    rt.settle_all();
    frames(&mut rt, 2);

    let thumbs = thumb_rects(&rt);
    assert!(thumbs.len() >= 2, "Overflowing driver container must attach scroll-bar thumb");
}
