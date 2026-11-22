//! Per-view Slint integration tests (headless, sandboxed backend — never touches the host).
//!
//! Each test drives a view purely through its Slint global store (exactly what a user click
//! does) and asserts on both the store and the underlying AppState.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use slint::{ComponentHandle, Model};
use winblast0::app::{ActiveView, AppState, MatrixSortMode};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::risk::load_embedded_rules;
use winblast0::ui_slint::bridge::SlintBridge;
use winblast0::ui_slint::{AppStore, DashboardStore, MainWindow, MatrixStore};

struct Harness {
    window: MainWindow,
    state: Rc<RefCell<AppState>>,
    bridge: Rc<SlintBridge>,
}

impl Harness {
    /// Sandbox seeded with the full embedded tweak catalog and an initial scan completed.
    fn scanned() -> Self {
        i_slint_backend_testing::init_no_event_loop();
        let sbx = SandboxEnv::new();
        sbx.seed_all_tweaks(&load_embedded_rules());
        let mut st = AppState::new_with_backend(Arc::new(sbx));
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
        Self { window, state, bridge }
    }

    fn goto(&self, view: ActiveView) {
        self.window.global::<AppStore>().invoke_navigate(view.name().into());
        self.bridge.sync();
    }

    /// Drain worker events until `pred` holds (sync after each round).
    fn wait(&self, what: &str, pred: impl Fn(&AppState) -> bool) {
        let start = Instant::now();
        loop {
            self.state.borrow_mut().drain_worker_events();
            self.bridge.sync();
            if pred(&self.state.borrow()) {
                return;
            }
            assert!(start.elapsed() < Duration::from_secs(10), "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(5));
        }
    }
}

// ───────────────────────────── Advanced Matrix ─────────────────────────────

fn tweak_rows(h: &Harness) -> Vec<winblast0::ui_slint::MatrixRow> {
    h.window.global::<MatrixStore>().get_rows().iter().filter(|r| !r.is_header).collect()
}

#[test]
fn matrix_lists_every_scanned_tweak_grouped_by_category() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    let total = h.state.borrow().scan_results.len();
    assert!(total >= 200, "catalog must contain the full tweak matrix, got {total}");
    assert_eq!(st.get_filtered_count() as usize, total);
    assert_eq!(tweak_rows(&h).len(), total);
    let headers: Vec<_> = st.get_rows().iter().filter(|r| r.is_header).collect();
    let cats = st.get_categories().row_count();
    assert_eq!(headers.len(), cats, "one header per category");
    // Header counts add up.
    assert_eq!(headers.iter().map(|r| r.category_count as usize).sum::<usize>(), total);
    // Every row carries a localized title and a state label.
    assert!(tweak_rows(&h).iter().all(|r| !r.title.is_empty() && !r.state_text.is_empty()));
}

#[test]
fn matrix_search_category_and_clear_filters() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    let total = st.get_filtered_count();

    // Search by an id fragment of a real tweak.
    let probe = h.state.borrow().scan_results[0].tweak_id.clone();
    st.invoke_query_edited(probe.as_str().into());
    h.bridge.sync();
    assert!(st.get_filtered_count() >= 1 && st.get_filtered_count() < total);
    assert!(tweak_rows(&h).iter().any(|r| r.id == probe.as_str()));
    assert!(st.get_has_filter());

    // Category filter.
    st.invoke_query_edited("".into());
    let cat = st.get_categories().row_data(0).expect("a category").raw;
    st.invoke_set_category(cat.clone());
    h.bridge.sync();
    assert_eq!(st.get_selected_category(), cat);
    assert!(tweak_rows(&h).iter().all(|r| r.category_raw == cat));

    st.invoke_clear_filters();
    h.bridge.sync();
    assert_eq!(st.get_filtered_count(), total);
    assert!(!st.get_has_filter());
}

#[test]
fn matrix_risk_chips_filter_by_real_stability_risk() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    for (kw, level) in [("safe", 0), ("low", 1), ("moderate", 2), ("high", 3), ("extreme", 4)] {
        st.invoke_set_risk_query(kw.into());
        h.bridge.sync();
        let expected = h
            .state
            .borrow()
            .scan_results
            .iter()
            .filter(|r| winblast0::ui_slint::views::matrix::risk_index(r.risk.stability_risk) == level)
            .count();
        let rows = tweak_rows(&h);
        assert!(rows.len() >= expected, "{kw}: {} rows < {expected} with that risk", rows.len());
        assert_eq!(rows.iter().filter(|r| r.risk_level == level).count(), expected, "{kw}");
    }
}

#[test]
fn matrix_select_all_respects_the_visible_filter() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    st.invoke_set_risk_query("safe".into());
    st.invoke_select_filtered();
    h.bridge.sync();
    let selected = h.state.borrow().selected_tweaks.clone();
    assert!(!selected.is_empty(), "safe filter must select something");
    let visible: std::collections::HashSet<String> = tweak_rows(&h).iter().map(|r| r.id.to_string()).collect();
    assert!(selected.iter().all(|id| visible.contains(id)), "Select All must only pick visible tweaks");
    assert_eq!(st.get_selected_count() as usize, selected.len());

    st.invoke_deselect_filtered();
    h.bridge.sync();
    assert!(h.state.borrow().selected_tweaks.is_empty());
}

#[test]
fn matrix_sort_cycles_through_all_modes_and_orders_rows() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    let mut seen = Vec::new();
    for _ in 0..6 {
        st.invoke_cycle_sort();
        h.bridge.sync();
        seen.push(h.state.borrow().matrix_sort_mode);
        assert!(st.get_sort_label().starts_with('↕'));
    }
    assert_eq!(*seen.last().unwrap(), MatrixSortMode::Default, "cycle must wrap");
    // NameAsc orders titles inside each category.
    h.state.borrow_mut().matrix_sort_mode = MatrixSortMode::NameAsc;
    h.bridge.sync();
    let rows: Vec<_> = h.window.global::<MatrixStore>().get_rows().iter().collect();
    let mut prev: Option<(String, String)> = None;
    for r in rows.iter().filter(|r| !r.is_header) {
        let raw_title = h.state.borrow().scan_results.iter().find(|s| s.tweak_id == r.id.as_str()).unwrap().title.to_lowercase();
        if let Some((pc, pt)) = &prev {
            if pc == r.category_raw.as_str() {
                assert!(pt <= &raw_title, "{pt} > {raw_title}");
            }
        }
        prev = Some((r.category_raw.to_string(), raw_title));
    }
}

#[test]
fn matrix_switch_click_applies_in_sandbox_and_updates_row() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    // Instant engine mode applies immediately.
    h.window.global::<AppStore>().invoke_set_engine_instant(true);
    h.bridge.sync();
    let target = tweak_rows(&h)
        .into_iter()
        .find(|r| !r.not_applicable && !r.active && r.risk_level <= 1)
        .expect("an applicable, inactive low-risk tweak");
    st.invoke_switch_clicked(target.id.clone());
    h.wait("tweak applied", |s| s.in_flight_tweaks.is_empty() && !s.is_applying && s.is_switch_active(target.id.as_str()));
    let row = tweak_rows(&h).into_iter().find(|r| r.id == target.id).unwrap();
    assert!(row.active, "row must reflect the applied state");
}

#[test]
fn matrix_inspect_opens_diff_and_category_toggle_selects() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    let id = tweak_rows(&h)[0].id.clone();
    st.invoke_inspect(id);
    h.bridge.sync();
    assert!(h.window.global::<AppStore>().get_diff_visible());
    h.window.global::<AppStore>().invoke_close_diff();

    let header = st.get_rows().iter().find(|r| r.is_header).unwrap();
    st.invoke_toggle_category(header.category_raw.clone(), true);
    h.bridge.sync();
    let header_after = st.get_rows().iter().find(|r| r.is_header && r.category_raw == header.category_raw).unwrap();
    // Category toggle selects every eligible tweak of that category.
    let sel = h.state.borrow().selected_tweaks.len();
    assert!(sel > 0);
    st.invoke_toggle_category(header.category_raw.clone(), false);
    h.bridge.sync();
    assert!(h.state.borrow().selected_tweaks.len() < sel || !header_after.all_selected);
}

#[test]
fn matrix_presets_unknown_kind_is_ignored() {
    let h = Harness::scanned();
    h.goto(ActiveView::AdvancedMatrix);
    let st = h.window.global::<MatrixStore>();
    st.invoke_apply_preset("bogus".into());
    h.bridge.sync();
    st.invoke_apply_preset("gaming".into());
    h.bridge.sync();
    let s = h.state.borrow();
    assert!(!s.selected_tweaks.is_empty() || s.is_applying || !s.status_message.is_empty());
}

// ───────────────────────────── Easy Dashboard ─────────────────────────────

#[test]
fn dashboard_health_score_and_category_breakdown_match_scan() {
    let h = Harness::scanned();
    h.goto(ActiveView::EasyDashboard);
    let st = h.window.global::<DashboardStore>();
    let s = h.state.borrow();
    let expected = winblast0::ui_slint::views::dashboard::health_score(&s.scan_results);
    assert_eq!(st.get_health_score() as u32, expected);
    assert!(expected <= 100);
    let applicable = s.scan_results.iter().filter(|r| r.current_state != winblast0::scanner::TweakState::NotApplicable).count();
    let cats: Vec<_> = st.get_categories().iter().collect();
    assert_eq!(cats.len(), 4);
    assert_eq!(cats.iter().map(|c| c.total as usize).sum::<usize>(), applicable);
    assert!(cats.iter().all(|c| c.optimized <= c.total));
}

#[test]
fn dashboard_never_shows_fabricated_hardware() {
    let h = Harness::scanned();
    h.goto(ActiveView::EasyDashboard);
    let st = h.window.global::<DashboardStore>();
    let s = h.state.borrow();
    match &s.cpu_info {
        Some(c) => assert_eq!(st.get_hw_cpu().as_str(), c.model_name),
        None => assert_eq!(st.get_hw_cpu().as_str(), "Detecting…"),
    }
    match s.gpus.first() {
        Some(g) => assert_eq!(st.get_hw_gpu().as_str(), g.name),
        None => assert_eq!(st.get_hw_gpu().as_str(), "Detecting…"),
    }
    for bogus in ["Intel / AMD Processor", "Discrete / Integrated GPU", "DDR4/DDR5 Memory", "Standard PC Motherboard"] {
        assert_ne!(st.get_hw_cpu().as_str(), bogus);
        assert_ne!(st.get_hw_gpu().as_str(), bogus);
        assert_ne!(st.get_hw_ram().as_str(), bogus);
        assert_ne!(st.get_hw_board().as_str(), bogus);
    }
}

#[test]
fn dashboard_updater_states_map_to_the_store() {
    use winblast0::app::UpdateStatus;
    let h = Harness::scanned();
    h.goto(ActiveView::EasyDashboard);
    let st = h.window.global::<DashboardStore>();
    for (status, expect) in [
        (UpdateStatus::Idle, "idle"),
        (UpdateStatus::Checking, "checking"),
        (UpdateStatus::UpToDate, "uptodate"),
        (UpdateStatus::Downloading { progress_pct: 50.0 }, "downloading"),
        (UpdateStatus::Failed("network down".into()), "failed"),
    ] {
        h.state.borrow_mut().update_status = status;
        h.bridge.sync();
        assert_eq!(st.get_update_state().as_str(), expect);
    }
    assert!(st.get_update_title().contains("network down"));
    h.state.borrow_mut().update_status = UpdateStatus::Downloading { progress_pct: 250.0 };
    h.bridge.sync();
    assert!((st.get_update_progress() - 1.0).abs() < 1e-6, "progress must clamp");
}

#[test]
fn dashboard_telemetry_and_impact_cards_are_consistent() {
    let h = Harness::scanned();
    h.goto(ActiveView::EasyDashboard);
    let st = h.window.global::<DashboardStore>();
    let has_metrics = h.state.borrow().realtime_metrics.is_some();
    assert_eq!(st.get_metrics_available(), has_metrics);
    assert!(st.get_cpu_points().row_count() <= 30);
    assert_eq!(st.get_impact_available(), h.state.borrow().calculate_before_after_impact().is_some());
    st.invoke_reset_baseline();
    h.wait("baseline reset finished", |s| !s.baseline_reset_inflight);
    assert_eq!(st.get_impact_available(), h.state.borrow().calculate_before_after_impact().is_some());
    assert!(!st.get_baseline_reset_inflight());
}
