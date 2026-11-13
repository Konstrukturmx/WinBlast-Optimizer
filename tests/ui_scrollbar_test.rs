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
        ActiveView::GpuTuning,
        ActiveView::HardwareView,
        ActiveView::PrivacyShield,
        ActiveView::BackupCenter,
        ActiveView::CoreParking,
        ActiveView::LatencyTester,
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
