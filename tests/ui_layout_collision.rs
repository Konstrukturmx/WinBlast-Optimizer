//! Responsive layout / collision sweep (headless, sandboxed backend — never touches the host).
//!
//! Renders every view at phone-narrow, tablet, laptop and desktop widths and checks, using the
//! element geometry from Slint's testing backend, that
//!   * no visible text runs off the right edge of the window, and
//!   * no two visible texts overlap each other inside the same region (shell or content viewport).
//!
//! It also checks that every ActiveView is reachable: routed in main.slint, listed in the
//! sidebar and in the Ctrl+K command palette.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::{ComponentHandle, LogicalSize};
use winblast0::app::{ActiveView, AppState};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::risk::load_embedded_rules;
use winblast0::ui_slint::bridge::SlintBridge;
use winblast0::ui_slint::{AppStore, MainWindow};

const WIDTHS: [f32; 5] = [560.0, 800.0, 1024.0, 1366.0, 1920.0];
const HEIGHT: f32 = 820.0;

struct Harness {
    window: MainWindow,
    state: Rc<RefCell<AppState>>,
    bridge: Rc<SlintBridge>,
}

fn harness() -> Harness {
    i_slint_backend_testing::init_no_event_loop();
    let sbx = Arc::new(SandboxEnv::new());
    sbx.seed_all_tweaks(&load_embedded_rules());
    let mut st = AppState::new_with_backend(sbx);
    st.start_scan();
    let start = Instant::now();
    loop {
        st.drain_worker_events();
        if !st.is_scanning && !st.scan_results.is_empty() {
            break;
        }
        assert!(start.elapsed() < Duration::from_secs(10), "sandbox scan timed out");
        std::thread::sleep(Duration::from_millis(5));
    }
    st.progress_message = None;
    let window = MainWindow::new().expect("MainWindow");
    let state = Rc::new(RefCell::new(st));
    let bridge = SlintBridge::new_ephemeral(window.clone_strong(), state.clone());
    Harness { window, state, bridge }
}

#[derive(Clone, Debug)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    label: String,
}

impl Rect {
    fn of(e: &ElementHandle) -> Self {
        let p = e.absolute_position();
        let s = e.size();
        Rect { x: p.x, y: p.y, w: s.width, h: s.height, label: e.accessible_label().map(|l| l.to_string()).unwrap_or_default() }
    }
    fn right(&self) -> f32 {
        self.x + self.w
    }
    fn bottom(&self) -> f32 {
        self.y + self.h
    }
    fn inside(&self, top: f32, bottom: f32, width: f32) -> bool {
        self.y >= top - 0.5 && self.bottom() <= bottom + 0.5 && self.x >= -0.5 && self.right() <= width + 0.5
    }
    /// Overlap area that is clearly visible (both overlap extents above 3 px).
    fn overlap(&self, o: &Rect) -> Option<(f32, f32)> {
        let ox = self.right().min(o.right()) - self.x.max(o.x);
        let oy = self.bottom().min(o.bottom()) - self.y.max(o.y);
        (ox > 3.0 && oy > 3.0).then_some((ox, oy))
    }
}

fn region(window: &MainWindow, type_name: &str) -> Option<Rect> {
    ElementHandle::find_by_element_type_name(window, type_name).next().map(|e| Rect::of(&e))
}

fn is_text(e: &ElementHandle) -> bool {
    matches!(e.type_name().as_deref(), Some("Text") | Some("SimpleText"))
        && e.accessible_role() == Some(AccessibleRole::Text)
        && e.computed_opacity() > 0.05
}

fn key(r: &Rect) -> (i64, i64, i64, i64, String) {
    (r.x.round() as i64, r.y.round() as i64, r.w.round() as i64, r.h.round() as i64, r.label.clone())
}

/// Visible texts, each clipped to the viewport of the innermost scroll area (Flickable) that
/// contains it, so content scrolled out of view never counts as a collision.
fn visible_texts(window: &MainWindow) -> Vec<Rect> {
    use std::collections::HashMap;
    let mut clip: HashMap<(i64, i64, i64, i64, String), Rect> = HashMap::new();
    for f in ElementHandle::find_by_element_type_name(window, "Flickable") {
        let vp = Rect::of(&f);
        let area = vp.w * vp.h;
        f.visit_descendants(|e| {
            if is_text(&e) {
                let r = Rect::of(&e);
                let k = key(&r);
                let smaller = clip.get(&k).is_none_or(|c| c.w * c.h > area);
                if smaller {
                    clip.insert(k, vp.clone());
                }
            }
            std::ops::ControlFlow::<()>::Continue(())
        });
    }
    // Toasts float above the content on purpose; they are not layout collisions.
    let mut floating = std::collections::HashSet::new();
    for t in ElementHandle::find_by_element_type_name(window, "AppToasts") {
        t.visit_descendants(|e| {
            if is_text(&e) {
                floating.insert(key(&Rect::of(&e)));
            }
            std::ops::ControlFlow::<()>::Continue(())
        });
    }
    ElementHandle::find_by_element_type_name(window, "Text")
        .chain(ElementHandle::find_by_element_type_name(window, "SimpleText"))
        .filter(is_text)
        .map(|e| Rect::of(&e))
        .filter(|r| !floating.contains(&key(r)))
        .filter_map(|r| match clip.get(&key(&r)) {
            None => Some(r),
            Some(vp) => {
                let (x0, y0) = (r.x.max(vp.x), r.y.max(vp.y));
                let (x1, y1) = (r.right().min(vp.right()), r.bottom().min(vp.bottom()));
                (x1 - x0 > 0.5 && y1 - y0 > 0.5).then_some(Rect { x: x0, y: y0, w: x1 - x0, h: y1 - y0, label: r.label })
            }
        })
        .filter(|r| !r.label.trim().is_empty() && r.w > 0.5 && r.h > 0.5)
        .collect()
}

/// Let background work started by opening a view finish so the in-progress HUD is gone.
fn settle(h: &Harness) {
    let start = Instant::now();
    loop {
        h.state.borrow_mut().drain_worker_events();
        if h.state.borrow().busy_mask() == 0 || start.elapsed() > Duration::from_secs(5) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    if h.state.borrow().busy_mask() != 0 {
        h.state.borrow_mut().dismiss_progress_hud();
    }
    h.bridge.sync();
}

fn check_view(h: &Harness, view: ActiveView, width: f32) -> Vec<String> {
    h.window.window().set_size(LogicalSize::new(width, HEIGHT));
    h.window.global::<AppStore>().invoke_navigate(view.name().into());
    h.bridge.sync();
    settle(h);
    // Let entrance animations finish so positions are final.
    for _ in 0..30 {
        i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
        slint::platform::update_timers_and_animations();
    }
    settle(h);

    let mut problems = Vec::new();
    let tag = format!("{}@{}px", view.name(), width);
    let top = region(&h.window, "AppTopBar").map(|r| r.bottom()).unwrap_or(0.0);
    let bottom = [region(&h.window, "AppProgressStrip"), region(&h.window, "AppFooter")]
        .into_iter()
        .flatten()
        .map(|r| r.y)
        .fold(HEIGHT, f32::min);
    let texts = visible_texts(&h.window);

    for t in &texts {
        // Only texts that are on screen vertically (scrolled-away content is clipped).
        if t.bottom() <= 0.0 || t.y >= HEIGHT {
            continue;
        }
        if t.right() > width + 1.0 && t.x < width {
            problems.push(format!("{tag}: text '{}' runs off the right edge ({:.0} > {width})", t.label, t.right()));
        }
    }

    // Pairwise overlaps inside the shell bars or fully inside the content viewport.
    let in_content: Vec<&Rect> = texts.iter().filter(|t| t.inside(top, bottom, width)).collect();
    let in_shell: Vec<&Rect> = texts.iter().filter(|t| t.bottom() <= top + 0.5 && t.y >= -0.5).collect();
    for group in [in_content, in_shell] {
        for (i, a) in group.iter().enumerate() {
            for b in &group[i + 1..] {
                if let Some((ox, oy)) = a.overlap(b) {
                    problems.push(format!(
                        "{tag}: '{}' overlaps '{}' by {ox:.0}x{oy:.0}px at ({:.0},{:.0})",
                        a.label, b.label, a.x.max(b.x), a.y.max(b.y)
                    ));
                }
            }
        }
    }
    problems
}

#[test]
fn no_text_collisions_or_overflow_at_any_width() {
    let h = harness();
    let mut problems = Vec::new();
    for width in WIDTHS {
        for view in ActiveView::ALL_VIEWS {
            problems.extend(check_view(&h, view, width));
        }
    }
    // The longest translations (German, Russian) at a laptop-narrow width.
    let app = h.window.global::<AppStore>();
    for lang in [2, 5] {
        app.invoke_set_language(lang);
        h.bridge.sync();
        for view in ActiveView::ALL_VIEWS {
            problems.extend(check_view(&h, view, 800.0).into_iter().map(|p| format!("lang {lang}: {p}")));
        }
    }
    app.invoke_set_language(0);
    // Keep the state alive until the end (workers may still post events).
    h.state.borrow_mut().drain_worker_events();
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{} layout collisions:\n{}", problems.len(), problems.join("\n"));
}

#[test]
fn every_view_is_routed_listed_and_searchable() {
    let main = include_str!("../ui/main.slint");
    let sidebar = include_str!("../ui/shell/sidebar.slint");
    let nav = include_str!("../ui/stores/nav.slint");
    let palette = include_str!("../src/ui_slint/palette.rs");
    let mut missing = Vec::new();
    for view in ActiveView::ALL_VIEWS {
        let n = view.name();
        // Merged into Gaming (E1): navigating there opens the Gaming page.
        if view == ActiveView::FpsBenchmark {
            continue;
        }
        if !main.contains(&format!("active-view == \"{n}\"")) {
            missing.push(format!("{n}: not routed in ui/main.slint"));
        }
        // A view is reachable from the sidebar directly or as a tab of a merged page (E).
        if !sidebar.contains(&format!("view: \"{n}\"")) && !nav.contains(&format!("view: \"{n}\"")) {
            missing.push(format!("{n}: neither in the sidebar nor a page tab"));
        }
        if !palette.contains(&format!("(\"{n}\",")) {
            missing.push(format!("{n}: missing from the command palette"));
        }
    }
    assert!(missing.is_empty(), "unreachable pages:\n{}", missing.join("\n"));
}

#[test]
fn top_bar_fits_in_every_language_at_every_width() {
    let h = harness();
    let app = h.window.global::<AppStore>();
    let mut problems = Vec::new();
    for lang in 0..8 {
        app.invoke_set_language(lang);
        h.bridge.sync();
        for width in [560.0, 700.0, 800.0, 900.0, 1024.0, 1100.0, 1280.0, 1366.0, 1600.0, 1920.0] {
            problems.extend(
                check_view(&h, ActiveView::EasyDashboard, width)
                    .into_iter()
                    .filter(|p| p.contains("off the right edge") || overlap_y(p).is_some_and(|y| y < 44.0))
                    .map(|p| format!("lang {lang}: {p}")),
            );
        }
    }
    app.invoke_set_language(0);
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{} top bar collisions:\n{}", problems.len(), problems.join("\n"));
}

/// Y coordinate of an "... at (x,y)" overlap report.
fn overlap_y(p: &str) -> Option<f32> {
    let tail = &p[p.rfind(" at (")? + 5..];
    tail.trim_end_matches(')').split(',').nth(1)?.trim().parse().ok()
}
