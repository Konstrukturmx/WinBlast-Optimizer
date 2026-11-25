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
use winblast0::ui_slint::{AppStore, AutomationStore, BackupStore, FaultStore, PerfStore, RepairStore, DiskStore, DriverStore, LatencyStore, PlannerStore, GpuStore, HardwareStore, ParkingStore, ThermalStore, PrivacyStore, CleanAppsStore, DashboardStore, MainWindow, MatrixStore, StartupStore, GamingStore, PrereqStore, NetworkStore, AboutStore};

struct Harness {
    #[allow(dead_code)]
    sbx: Arc<SandboxEnv>,
    window: MainWindow,
    state: Rc<RefCell<AppState>>,
    bridge: Rc<SlintBridge>,
}

impl Harness {
    /// Sandbox seeded with the full embedded tweak catalog and an initial scan completed.
    fn scanned() -> Self {
        i_slint_backend_testing::init_no_event_loop();
        let sbx = Arc::new(SandboxEnv::new());
        sbx.seed_all_tweaks(&load_embedded_rules());
        let mut st = AppState::new_with_backend(sbx.clone());
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
        Self { sbx, window, state, bridge }
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

// ───────────────────────────── Clean & Apps ─────────────────────────────

#[test]
fn clean_apps_sub_tabs_drive_state() {
    let h = Harness::scanned();
    h.goto(ActiveView::CleanAndApps);
    let st = h.window.global::<CleanAppsStore>();
    for i in [1, 2, 0, 7, -3] {
        st.invoke_set_sub_tab(i);
        h.bridge.sync();
        let expect = if (0..=2).contains(&i) { i } else { 0 };
        assert_eq!(st.get_sub_tab(), expect, "tab {i}");
    }
}

#[test]
fn clean_apps_disk_cleaner_scan_toggle_and_clean_in_sandbox() {
    let h = Harness::scanned();
    h.goto(ActiveView::CleanAndApps);
    let st = h.window.global::<CleanAppsStore>();
    st.invoke_scan_targets();
    h.wait("clean scan", |s| !s.clean_apps_busy);
    let n = st.get_targets().row_count();
    assert_eq!(n, h.state.borrow().clean_targets.len());
    if n == 0 {
        return; // sandbox exposes no cleanup targets in this configuration
    }
    st.invoke_deselect_all_targets();
    h.bridge.sync();
    assert!(st.get_targets().iter().all(|t| !t.selected));
    let first = st.get_targets().row_data(0).unwrap();
    st.invoke_toggle_target(first.id.clone());
    h.bridge.sync();
    assert!(st.get_targets().row_data(0).unwrap().selected);
    st.invoke_toggle_target("NotARealTarget".into()); // ignored
    st.invoke_select_all_targets();
    h.bridge.sync();
    assert!(st.get_targets().iter().all(|t| t.selected));
    st.invoke_clean_selected();
    h.wait("clean executed", |s| !s.clean_apps_busy && s.last_clean_report.is_some());
    assert!(st.get_last_freed().contains("MB") || st.get_last_freed().contains("GB"));
}

#[test]
fn clean_apps_appx_filter_protected_core_and_selection() {
    let h = Harness::scanned();
    h.goto(ActiveView::CleanAndApps);
    let st = h.window.global::<CleanAppsStore>();
    st.invoke_set_sub_tab(1);
    st.invoke_scan_appx();
    h.wait("appx scan", |s| !s.clean_apps_busy);
    let rows: Vec<_> = st.get_appx().iter().collect();
    assert_eq!(rows.len(), h.state.borrow().installed_appx.len());
    // Protected-core packages can never be selected.
    if let Some(core) = rows.iter().find(|r| r.protected) {
        st.invoke_toggle_appx(core.family.clone());
        h.bridge.sync();
        assert!(!h.state.borrow().selected_appx.contains(core.family.as_str()));
    }
    if let Some(bloat) = rows.iter().find(|r| !r.protected) {
        st.invoke_toggle_appx(bloat.family.clone());
        h.bridge.sync();
        assert!(h.state.borrow().selected_appx.contains(bloat.family.as_str()));
        st.invoke_deselect_appx();
        h.bridge.sync();
        assert_eq!(st.get_appx_selected(), 0);
        // Filter narrows the list.
        st.invoke_appx_filter_edited(bloat.name.clone());
        h.bridge.sync();
        assert!(st.get_appx().iter().all(|r| r.name.to_lowercase().contains(&bloat.name.to_lowercase()) || r.family.to_lowercase().contains(&bloat.name.to_lowercase())));
    }
    st.invoke_select_safe_ai();
    h.bridge.sync();
    assert!(h.state.borrow().selected_appx.iter().all(|f| !rows.iter().any(|r| r.protected && r.family.as_str() == f)));
}

#[test]
fn clean_apps_winget_catalog_filter_and_selection() {
    let h = Harness::scanned();
    h.goto(ActiveView::CleanAndApps);
    let st = h.window.global::<CleanAppsStore>();
    st.invoke_set_sub_tab(2);
    h.bridge.sync();
    let all = st.get_winget().row_count();
    assert_eq!(all, winblast0::tools::features_apps::CURATED_WINGET_APPS.len());
    let first = st.get_winget().row_data(0).unwrap();
    st.invoke_toggle_winget(first.id.clone());
    h.bridge.sync();
    assert_eq!(st.get_winget_selected(), 1);
    st.invoke_winget_filter_edited("zzzz-no-such-app".into());
    h.bridge.sync();
    assert_eq!(st.get_winget().row_count(), 0);
    st.invoke_winget_filter_edited("".into());
    st.invoke_deselect_winget();
    h.bridge.sync();
    assert_eq!(st.get_winget().row_count(), all);
    assert_eq!(st.get_winget_selected(), 0);
}

// ───────────────────────────── Startup, Tasks & Services ─────────────────────────────

fn startup_ready() -> Harness {
    let h = Harness::scanned();
    h.goto(ActiveView::StartupAndTasks);
    h.window.global::<StartupStore>().invoke_refresh();
    h.wait("startup inventory", |s| !s.startup_tasks_busy);
    // "Hide Microsoft" is on by default and the sandbox seeds Microsoft-signed items: show them.
    if h.state.borrow().hide_microsoft_tasks {
        h.window.global::<StartupStore>().invoke_toggle_hide_microsoft();
        h.bridge.sync();
    }
    h
}

#[test]
fn startup_lists_filters_and_sorts() {
    let h = startup_ready();
    let st = h.window.global::<StartupStore>();
    let entries = h.state.borrow().startup_entries.len();
    assert!(entries > 0, "sandbox seeds startup entries");
    assert_eq!(st.get_rows().row_count(), entries);

    st.invoke_set_status_filter(1);
    h.bridge.sync();
    assert!(st.get_rows().iter().all(|r| r.enabled));
    st.invoke_set_status_filter(2);
    h.bridge.sync();
    assert!(st.get_rows().iter().all(|r| !r.enabled));
    st.invoke_set_status_filter(0);

    let mut labels = std::collections::HashSet::new();
    for _ in 0..5 {
        st.invoke_cycle_sort();
        h.bridge.sync();
        labels.insert(st.get_sort_label().to_string());
    }
    assert_eq!(labels.len(), 5, "five distinct localized sort labels");
    assert_eq!(h.state.borrow().startup_sort_mode, winblast0::app::StartupSortMode::NameAsc);
    let names: Vec<String> = st.get_rows().iter().map(|r| r.name.to_lowercase()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);

    st.invoke_filter_edited("zzzz-nothing".into());
    h.bridge.sync();
    assert_eq!(st.get_rows().row_count(), 0);
    st.invoke_filter_edited("".into());
    let before = h.state.borrow().hide_microsoft_tasks;
    st.invoke_toggle_hide_microsoft();
    h.bridge.sync();
    assert_eq!(st.get_hide_microsoft(), !before);
    let visible_ms = h.state.borrow().startup_entries.iter().filter(|e| e.is_microsoft).count();
    if st.get_hide_microsoft() {
        assert_eq!(st.get_rows().row_count(), entries - visible_ms);
    }
}

#[test]
fn startup_batch_disable_selected_entries_in_sandbox() {
    let h = startup_ready();
    let st = h.window.global::<StartupStore>();
    st.invoke_select_all_filtered();
    h.bridge.sync();
    let selected = h.state.borrow().selected_startup_entries.len();
    assert!(selected > 0);
    assert_eq!(st.get_selected_count() as usize, selected);
    st.invoke_batch_set(false);
    h.wait("batch disable", |s| !s.startup_tasks_busy);
    let s = h.state.borrow();
    let toggleable: Vec<_> = s
        .startup_entries
        .iter()
        .filter(|e| !e.policy_locked && !winblast0::ui_slint::views::startup::is_run_once(e))
        .collect();
    assert!(toggleable.iter().all(|e| !e.enabled), "every toggleable selected entry is disabled");
    drop(s);
    st.invoke_deselect_all();
    h.bridge.sync();
    assert_eq!(st.get_selected_count(), 0);
}

#[test]
fn services_respect_protected_core_and_reject_bad_start_types() {
    let h = startup_ready();
    let st = h.window.global::<StartupStore>();
    st.invoke_set_sub_tab(2);
    h.bridge.sync();
    let rows: Vec<_> = st.get_rows().iter().collect();
    assert_eq!(rows.len(), h.state.borrow().live_services.len());
    if let Some(core) = rows.iter().find(|r| r.locked) {
        let before = h.state.borrow().live_services.iter().find(|v| v.service_name == core.id.as_str()).unwrap().start_type;
        st.invoke_set_service_start(core.id.clone(), 4);
        h.wait("idle", |s| !s.startup_tasks_busy);
        let after = h.state.borrow().live_services.iter().find(|v| v.service_name == core.id.as_str()).unwrap().start_type;
        assert_eq!(before, after, "protected core service must never change");
    }
    if let Some(svc) = rows.iter().find(|r| !r.locked) {
        st.invoke_set_service_start(svc.id.clone(), 99); // invalid start type ignored
        st.invoke_set_service_start(svc.id.clone(), -1);
        h.bridge.sync();
        assert!(!h.state.borrow().startup_tasks_busy);
        st.invoke_set_service_start(svc.id.clone(), 3);
        h.wait("service start type", |s| !s.startup_tasks_busy);
        let now = h.state.borrow().live_services.iter().find(|v| v.service_name == svc.id.as_str()).unwrap().start_type;
        assert_eq!(now, 3);
    }
}

// ───────────────────────────── Backup Center ─────────────────────────────

#[test]
fn backup_restore_point_profile_and_unattend_roundtrip_in_sandbox() {
    let h = Harness::scanned();
    h.goto(ActiveView::BackupCenter);
    let st = h.window.global::<BackupStore>();
    st.invoke_create_restore_point();
    h.wait("restore point", |s| !s.restore_point_inflight);
    st.invoke_refresh_restore_points();
    h.wait("restore list", |s| !s.restore_point_inflight);
    assert_eq!(st.get_restore_points().row_count(), h.state.borrow().restore_points.len());

    // Unattend options toggle both ways and the arch flips.
    let arch0 = st.get_arch();
    st.invoke_toggle_arch();
    h.bridge.sync();
    assert_ne!(st.get_arch(), arch0);
    let read = |st: &BackupStore, opt: &str| match opt {
        "tpm" => st.get_bypass_tpm(),
        "msa" => st.get_bypass_msa(),
        "privacy" => st.get_skip_privacy(),
        "tweaks" => st.get_include_tweaks(),
        _ => st.get_auto_timezone(),
    };
    for opt in ["tpm", "msa", "privacy", "tweaks", "timezone"] {
        let before = read(&st, opt);
        st.invoke_toggle_option(opt.into());
        h.bridge.sync();
        assert_eq!(read(&st, opt), !before, "{opt}");
    }
    st.invoke_toggle_option("bogus".into());
    st.invoke_unattend_preset("bogus".into());
    st.invoke_unattend_preset("privacy".into());
    st.invoke_export_unattend();
    h.bridge.sync();
    let exported = st.get_last_export();
    assert!(!exported.is_empty(), "export path shown");
    assert!(std::path::Path::new(exported.as_str()).exists(), "autounattend.xml written to the sandbox profile dir");
    assert!(exported.to_lowercase().contains("temp") || exported.contains("tmp") || exported.contains("sandbox"), "must stay inside the sandbox: {exported}");

    st.invoke_export_profile();
    h.bridge.sync();
    st.invoke_import_profile();
    h.bridge.sync();
}

#[test]
fn backup_ledger_undo_reverts_an_applied_tweak() {
    let h = Harness::scanned();
    h.window.global::<AppStore>().invoke_set_engine_instant(true);
    h.goto(ActiveView::AdvancedMatrix);
    // A registry-DWORD tweak is always reversible.
    let reg_ids: std::collections::HashSet<String> = h
        .state
        .borrow()
        .tweaks
        .iter()
        .filter(|t| !t.actions.is_empty() && t.actions.iter().all(|a| matches!(a, winblast0::core::risk::TweakAction::RegistryDword { .. })))
        .map(|t| t.id.clone())
        .collect();
    let target = tweak_rows(&h)
        .into_iter()
        .find(|r| !r.not_applicable && !r.active && r.risk_level <= 1 && reg_ids.contains(r.id.as_str()))
        .expect("a low-risk registry tweak");
    h.window.global::<MatrixStore>().invoke_switch_clicked(target.id.clone());
    h.wait("applied", |s| s.in_flight_tweaks.is_empty() && !s.is_applying && s.is_switch_active(target.id.as_str()));
    h.state.borrow_mut().refresh_ledger();
    h.goto(ActiveView::BackupCenter);
    let st = h.window.global::<BackupStore>();
    let entry = st.get_ledger().iter().find(|e| !e.rolled_back && e.reversible).expect("a reversible ledger entry");
    assert!(st.get_active_changes() >= 1);
    st.invoke_rollback("not-a-number".into());
    st.invoke_rollback(entry.id.clone());
    h.wait("rolled back", |s| !s.is_rolling_back);
    h.state.borrow_mut().refresh_ledger();
    h.bridge.sync();
    let after = st.get_ledger().iter().find(|e| e.id == entry.id).unwrap();
    assert!(after.rolled_back, "ledger entry must be marked rolled back");
}

// ───────────────────────────── Privacy Shield ─────────────────────────────

#[test]
fn privacy_stats_list_and_select_all_are_consistent() {
    let h = Harness::scanned();
    h.goto(ActiveView::PrivacyShield);
    let st = h.window.global::<PrivacyStore>();
    let rows: Vec<_> = st.get_rows().iter().collect();
    assert!(!rows.is_empty(), "catalog has privacy/telemetry tweaks");
    let applicable = rows.iter().filter(|r| !r.not_applicable).count() as i32;
    assert!(st.get_optimized() + st.get_not_optimized() + st.get_partial() <= applicable);

    st.invoke_select_all();
    h.bridge.sync();
    let s = h.state.borrow();
    let row_ids: std::collections::HashSet<&str> = rows.iter().map(|r| r.id.as_str()).collect();
    assert!(!s.selected_tweaks.is_empty());
    for id in &s.selected_tweaks {
        assert!(row_ids.contains(id.as_str()), "Select All picked a non-privacy tweak: {id}");
        let t = s.tweaks.iter().find(|t| &t.id == id).unwrap();
        assert!(t.valuation.stability_risk < winblast0::core::risk::RiskLevel::High, "{id} is high risk");
    }
    // Instant mode must not stage transitions.
    if s.engine_mode == winblast0::app::EngineMode::Instant {
        assert!(s.staged_transitions.is_empty());
    }
    drop(s);
    st.invoke_deselect_all();
    h.bridge.sync();
    assert_eq!(st.get_selected_count(), 0);
}

#[test]
fn privacy_audit_and_hosts_block_run_in_sandbox() {
    let h = Harness::scanned();
    h.goto(ActiveView::PrivacyShield);
    let st = h.window.global::<PrivacyStore>();
    st.invoke_run_audit();
    h.wait("audit", |s| s.telemetry_audit.is_some());
    assert!(st.get_audit_available());
    assert!(st.get_hosts_status().contains('✓'));
    st.invoke_block_hosts();
    h.wait("hosts blocked", |s| !s.telemetry_hosts_inflight);
    st.invoke_neutralize_services();
    h.wait("services neutralized", |s| !s.telemetry_services_inflight);
    assert!(!st.get_hosts_inflight() && !st.get_services_inflight());
}

// ───────────────────────────── GPU Tuning ─────────────────────────────

#[test]
fn gpu_adapters_hags_mpo_and_msi_validation() {
    let h = Harness::scanned();
    h.goto(ActiveView::GpuTuning);
    let st = h.window.global::<GpuStore>();
    assert_eq!(st.get_adapters().row_count(), h.state.borrow().gpus.len());

    if st.get_hags_supported() {
        let before = h.state.borrow().hags_enabled;
        st.invoke_toggle_hags(!before);
        h.wait("hags", |s| !s.hags_inflight);
        assert_eq!(h.state.borrow().hags_enabled, !before);
        assert_eq!(st.get_hags_enabled(), !before);
    }
    let mpo = h.state.borrow().mpo_disabled;
    st.invoke_toggle_mpo(!mpo);
    h.wait("mpo", |s| !s.mpo_inflight);
    assert_eq!(h.state.borrow().mpo_disabled, !mpo);

    // Invalid MSI requests are ignored without touching state.
    st.invoke_set_msi("".into(), true, 2);
    st.invoke_set_msi("PCI\\VEN_TEST".into(), true, 0);
    st.invoke_set_msi("PCI\\VEN_TEST".into(), true, 9);
    h.bridge.sync();
    assert!(h.state.borrow().gpu_msi_inflight.is_empty());
    if let Some(a) = st.get_adapters().iter().find(|a| a.pnp_valid) {
        st.invoke_set_msi(a.pnp.clone(), !a.msi, 3);
        h.wait("msi", |s| s.gpu_msi_inflight.is_empty());
    }
}

#[test]
fn gpu_vendor_tweaks_hide_and_unhide() {
    let h = Harness::scanned();
    h.goto(ActiveView::GpuTuning);
    let st = h.window.global::<GpuStore>();
    let n = st.get_tweaks().row_count();
    assert!(n > 0, "catalog has GPU tweaks");
    let vendors = h.state.borrow().detected_gpu_vendors();
    for t in st.get_tweaks().iter() {
        let def = h.state.borrow().tweaks.iter().find(|d| d.id == t.id.as_str()).cloned().unwrap();
        assert_eq!(t.supported, def.is_gpu_vendor_compatible(&vendors), "{}", t.id);
        if !t.supported {
            assert!(!t.required.is_empty());
        }
    }
    let first = st.get_tweaks().row_data(0).unwrap();
    st.invoke_hide_tweak(first.id.clone());
    h.bridge.sync();
    assert_eq!(st.get_tweaks().row_count(), n - 1);
    assert_eq!(st.get_hidden_count(), 1);
    st.invoke_unhide_all();
    h.bridge.sync();
    assert_eq!(st.get_tweaks().row_count(), n);
}

// ───────────────────────────── Hardware ─────────────────────────────

#[test]
fn hardware_cards_reflect_detected_data_only() {
    let h = Harness::scanned();
    h.state.borrow_mut().refresh_platform_inventory();
    h.wait("inventory", |s| !s.power_schemes.is_empty() || s.platform_security.is_some());
    h.goto(ActiveView::HardwareView);
    let st = h.window.global::<HardwareStore>();
    let s = h.state.borrow();
    let cpu_rows: Vec<_> = st.get_cpu_rows().iter().collect();
    match &s.cpu_info {
        Some(c) => assert!(cpu_rows.iter().any(|r| r.v == c.model_name.as_str())),
        None => assert_eq!(cpu_rows.len(), 1),
    }
    assert_eq!(st.get_volumes().row_count(), s.storage_volumes.len());
    assert_eq!(st.get_dimms().row_count(), s.ram_dimms.len());
    assert_eq!(st.get_schemes().row_count(), s.power_schemes.len());
    if let Some(m) = &s.realtime_metrics {
        assert_eq!(st.get_cores().row_count(), m.core_usages.len());
        assert!(st.get_cores().iter().all(|c| (0.0..=1.0).contains(&c.value)));
    }
}

#[test]
fn hardware_power_plan_hibernation_and_usb_in_sandbox() {
    let h = Harness::scanned();
    h.state.borrow_mut().refresh_platform_inventory();
    h.wait("inventory", |s| !s.power_schemes.is_empty());
    h.goto(ActiveView::HardwareView);
    let st = h.window.global::<HardwareStore>();
    st.invoke_set_power_plan("00000000-not-a-plan".into()); // unknown GUID ignored
    h.bridge.sync();
    assert!(!h.state.borrow().power_scheme_inflight);
    if let Some(inactive) = st.get_schemes().iter().find(|p| !p.active) {
        st.invoke_set_power_plan(inactive.id.clone());
        h.wait("plan switch", |s| !s.power_scheme_inflight);
        assert!(h.state.borrow().power_schemes.iter().any(|p| p.guid == inactive.id.as_str() && p.is_active));
    }
    st.invoke_ultimate_plan();
    h.wait("ultimate plan", |s| !s.power_scheme_inflight);
    let hib = h.state.borrow().hibernation_status.hibernation_enabled;
    st.invoke_toggle_hibernation(!hib);
    h.wait("hibernation", |s| !s.hibernation_inflight);
    st.invoke_usb_power_saving(true);
    h.wait("usb", |s| !s.usb_power_inflight);
    assert!(!st.get_power_inflight() && !st.get_hibernation_inflight() && !st.get_usb_inflight());
}

// ───────────────────────────── Core parking / Thermal / Disks ─────────────────────────────

#[test]
fn core_parking_actions_round_trip_in_sandbox() {
    let h = Harness::scanned();
    h.goto(ActiveView::CoreParking);
    let st = h.window.global::<ParkingStore>();
    assert_eq!(st.get_cores().row_count(), h.state.borrow().core_parking.cores.len());
    assert_eq!(st.get_total() as usize, h.state.borrow().core_parking.total_cores);
    st.invoke_unpark_all();
    h.wait("unpark", |s| !s.core_parking_inflight);
    assert!(st.get_fully_unparked() || st.get_parked() == 0 || !h.state.borrow().status_message.is_empty());
    st.invoke_restore_stock();
    h.wait("restore", |s| !s.core_parking_inflight);
    st.invoke_refresh();
    h.wait("refresh", |s| !s.core_parking_inflight);
    assert!(st.get_cores().iter().all(|c| (0.0..=1.0).contains(&c.usage)));
}

#[test]
fn thermal_view_never_invents_temperatures() {
    let h = Harness::scanned();
    h.goto(ActiveView::ThermalMonitor);
    let st = h.window.global::<ThermalStore>();
    st.invoke_refresh();
    h.wait("thermal", |s| !s.is_thermal_scanning && s.thermal_snapshot.is_some());
    // The sandbox has no sensors: every reading must be the honest placeholder.
    assert!(!st.get_sensors_available());
    assert_eq!(st.get_cpu_temp().as_str(), "—");
    assert_eq!(st.get_gpu_temp().as_str(), "—");
    assert_eq!(st.get_fan_text().as_str(), "—");
    for bogus in ["Auto PWM Active", "Smart Fan Curve Active"] {
        assert!(!st.get_fan_text().contains(bogus) && !st.get_fan_detail().contains(bogus));
    }
}

#[test]
fn disk_view_trim_is_async_and_direct_storage_is_not_claimed() {
    let h = Harness::scanned();
    h.goto(ActiveView::DiskHealth);
    let st = h.window.global::<DiskStore>();
    st.invoke_refresh();
    h.wait("disk report", |s| !s.is_disk_scanning && s.disk_report.is_some());
    assert_eq!(st.get_drives().row_count(), 0, "sandbox has no physical disks");
    assert!(!st.get_trim_banner().contains("DirectStorage"), "DirectStorage must only be shown when detected");
    st.invoke_trim_all();
    h.bridge.sync();
    st.invoke_trim_all(); // second click while in flight is ignored
    h.wait("trim done", |s| !s.disk_trim_inflight && !s.is_disk_scanning);
    assert!(h.state.borrow().status_message.contains("ReTrim"), "status: {}", h.state.borrow().status_message);
}

// ───────────────────────────── Driver & BIOS ─────────────────────────────

#[test]
fn drivers_scan_filter_install_and_backup_are_async_and_sandboxed() {
    let h = Harness::scanned();
    h.goto(ActiveView::DriverBios);
    let st = h.window.global::<DriverStore>();
    st.invoke_scan_online();
    h.wait("driver scan", |s| !s.is_driver_scanning);
    assert_eq!(st.get_total() as usize, h.state.borrow().driver_updates.len());
    let id = st.get_drivers().row_data(0).expect("sandbox exposes the virtual test driver").id;

    // Category filter + folded search.
    st.invoke_set_category("Audio".into());
    h.bridge.sync();
    assert!(st.get_drivers().iter().all(|d| d.details.starts_with("Audio") || d.details.contains("Audio")));
    st.invoke_set_category("NotACategory".into());
    h.bridge.sync();
    assert_eq!(st.get_category().as_str(), "");
    st.invoke_search_edited("zzzz-nope".into());
    h.bridge.sync();
    assert_eq!(st.get_drivers().row_count(), 0);
    st.invoke_search_edited("".into());
    h.bridge.sync();

    // Install runs on a worker: the in-flight flag is visible until the worker reports back.
    st.invoke_install(id.clone());
    assert!(h.state.borrow().driver_install_in_flight.is_some(), "install must not block the UI thread");
    st.invoke_install(id.clone()); // ignored while in flight
    h.wait("install", |s| s.driver_install_in_flight.is_none());
    let item = h.state.borrow().driver_updates.iter().find(|d| d.id == id.as_str()).cloned().unwrap();
    assert_eq!(item.state, winblast0::hardware::driver_update::DriverUpdateState::Installed);

    st.invoke_backup(id.clone());
    h.wait("backup", |s| s.driver_install_in_flight.is_none());
    assert!(h.state.borrow().status_message.contains("Backup created"), "{}", h.state.borrow().status_message);

    st.invoke_rescan_bios();
    h.wait("bios", |s| !s.is_bios_scanning);
    assert_eq!(st.get_board_title().as_str(), "Detecting…", "sandbox board info is honest, not invented");
}

// ───────────────────────────── Latency benchmark ─────────────────────────────

#[test]
fn latency_run_baseline_compare_and_clear() {
    let h = Harness::scanned();
    h.goto(ActiveView::LatencyTester);
    let st = h.window.global::<LatencyStore>();
    st.invoke_save_baseline();
    h.wait("baseline bench", |s| !s.is_benchmarking_latency);
    st.invoke_run();
    h.wait("after bench", |s| !s.is_benchmarking_latency && s.latest_latency_bench.is_some());
    assert!(st.get_has_result());
    assert_eq!(st.get_kpis().row_count(), 4);
    if h.state.borrow().latency_before.is_some() && h.state.borrow().latency_after.is_some() {
        assert!(st.get_has_comparison());
        assert_eq!(st.get_comparison().row_count(), 4);
    }
    // KPI strip never prints a negative ping.
    assert!(st.get_kpis().iter().all(|k| !k.now.starts_with('-')));
    st.invoke_clear();
    h.bridge.sync();
    assert!(!st.get_has_comparison());
}

// ───────────────────────────── Goal Planner ─────────────────────────────

#[test]
fn planner_goals_solve_and_stage() {
    let h = Harness::scanned();
    h.goto(ActiveView::GoalPlanner);
    let st = h.window.global::<PlannerStore>();
    for g in ["gaming", "privacy", "balanced"] {
        st.invoke_select_goal(g.into());
        h.bridge.sync();
        assert_eq!(st.get_active_goal().as_str(), g);
        assert_eq!(st.get_metrics().row_count(), 6);
    }
    st.invoke_select_goal("bogus".into());
    h.bridge.sync();
    assert_eq!(st.get_active_goal().as_str(), "balanced");
    st.invoke_solve();
    h.bridge.sync();
    assert!(st.get_has_plan(), "A* must produce a plan for the balanced goal");
    assert!(st.get_steps().iter().all(|s| !s.title.is_empty() && !s.badge.is_empty()));
    let before = h.state.borrow().selected_tweaks.len();
    st.invoke_stage();
    h.bridge.sync();
    assert!(h.state.borrow().selected_tweaks.len() >= before);
}

// ───────────────────────────── Automation Center ─────────────────────────────

#[test]
fn automation_profiles_autotune_trim_health_and_maintenance() {
    let h = Harness::scanned();
    h.goto(ActiveView::AutomationCenter);
    let st = h.window.global::<AutomationStore>();
    for p in ["GamingLowLatency", "ProAudioDpc", "WorkstationStability", "LaptopBatterySaving"] {
        st.invoke_set_profile(p.into());
        h.bridge.sync();
        assert_eq!(st.get_profile().as_str(), p);
        assert!(st.get_plan().row_count() <= 24);
        assert_eq!(st.get_plan_more(), (st.get_plan_count() - 24).max(0));
    }
    st.invoke_set_profile("Nope".into());
    h.bridge.sync();
    assert_eq!(st.get_profile().as_str(), "LaptopBatterySaving");

    st.invoke_run_auto_tune();
    h.wait("auto-tune", |s| !s.is_running_automation && s.auto_tune_result.is_some());
    assert!(st.get_has_result());

    st.invoke_trim_memory();
    assert!(h.state.borrow().is_running_automation, "trim runs on a worker");
    h.wait("trim", |s| !s.is_running_automation && s.last_memory_trim_result.is_some());
    assert!(st.get_has_trim());

    st.invoke_assess_health();
    h.wait("health", |s| !s.is_assessing_health && s.system_health_assessment.is_some());
    assert!(st.get_has_health());

    st.invoke_refresh_maintenance();
    h.wait("maintenance list", |s| !s.maintenance_inflight);
    let rows: Vec<_> = st.get_maintenance().iter().collect();
    assert_eq!(rows.len(), 3);
    let first = rows[0].clone();
    st.invoke_toggle_maintenance(first.id.clone());
    h.wait("maintenance toggle", |s| !s.maintenance_inflight);
    let after = st.get_maintenance().iter().find(|m| m.id == first.id).unwrap();
    assert_eq!(after.installed, !first.installed, "toggle flips the scheduled task (sandbox marker)");
    st.invoke_toggle_maintenance("NotATask".into());
    h.bridge.sync();
    assert!(!h.state.borrow().maintenance_inflight);
}

// ───────────────────────────── Fault diagnostics / Repair / Perf ─────────────────────────────

#[test]
fn fault_scan_is_async_and_restart_needs_confirmation() {
    let h = Harness::scanned();
    assert!(h.state.borrow().fault_report.is_none(), "constructor must not scan the host");
    h.goto(ActiveView::FaultDiagnostics);
    h.wait("fault scan", |s| !s.fault_scanning && s.fault_report.is_some());
    let st = h.window.global::<FaultStore>();
    assert_eq!(st.get_critical(), 0);

    st.invoke_restart_now();
    h.bridge.sync();
    assert!(st.get_confirm_restart(), "first click only arms");
    assert!(!h.state.borrow().fault_action_inflight);
    st.invoke_restart_now();
    h.wait("restart (sandbox)", |s| !s.fault_action_inflight && !s.fault_scanning);
    assert!(h.state.borrow().status_message.contains("RestartSystem"), "{}", h.state.borrow().status_message);
    assert!(!st.get_confirm_restart());

    // Leaving the view disarms a pending confirmation.
    st.invoke_restart_now();
    h.goto(ActiveView::EasyDashboard);
    h.goto(ActiveView::FaultDiagnostics);
    assert!(!st.get_confirm_restart());

    st.invoke_clear_dumps();
    h.wait("clear dumps", |s| !s.fault_action_inflight && !s.fault_scanning);
}

#[test]
fn repair_tasks_run_on_a_worker_and_log_results() {
    let h = Harness::scanned();
    h.goto(ActiveView::WindowsRepair);
    let st = h.window.global::<RepairStore>();
    let n = st.get_tasks().row_count();
    assert!(n > 0);
    let id = st.get_tasks().row_data(0).unwrap().id;
    st.invoke_run("no-such-task".into());
    h.bridge.sync();
    assert!(h.state.borrow().repair_running.is_none());
    st.invoke_run(id.clone());
    assert_eq!(h.state.borrow().repair_running.as_deref(), Some(id.as_str()), "must not block the UI thread");
    st.invoke_run("all".into()); // ignored while busy
    h.wait("single repair", |s| s.repair_running.is_none());
    assert_eq!(st.get_log().row_count(), 1);
    st.invoke_run("all".into());
    h.wait("all repairs", |s| s.repair_running.is_none());
    assert_eq!(st.get_log().row_count(), 1 + n);
    st.invoke_clear_log();
    h.bridge.sync();
    assert_eq!(st.get_log().row_count(), 0);
}

#[test]
fn perf_benchmark_is_guarded_and_never_shows_fake_zeros() {
    let h = Harness::scanned();
    h.goto(ActiveView::PerformanceTests);
    let st = h.window.global::<PerfStore>();
    assert!(!st.get_has_score());
    assert_eq!(st.get_composite().as_str(), "—");
    st.invoke_start();
    h.wait("benchmark", |s| !s.is_benchmarking_perf);
    // Sandbox returns an empty score: placeholders stay, nothing claims a measured 0.
    assert!(!st.get_has_score());
    assert_eq!(st.get_cpu_multi().as_str(), "—");
}

// ───────────────────────────── Gaming hub / Prerequisites / Network ─────────────────────────────

#[test]
fn gaming_hub_tabs_profiles_overlay_and_honest_monitor() {
    let h = Harness::scanned();
    h.goto(ActiveView::GamingStats);
    h.wait("game probe", |s| !s.game_probe_inflight);
    let g = h.window.global::<GamingStore>();
    assert_eq!(g.get_sub_tab(), 0);
    assert!(h.state.borrow().game_telemetry.active_game_detected.is_none());
    assert!(!g.get_detected_game().is_empty(), "shows the localized 'no game' text, not a blank");
    assert_eq!(g.get_fps().row_count(), 0, "no invented stock-vs-optimized FPS figures (C9)");

    // Out-of-range sub-tab is ignored.
    g.invoke_set_sub_tab(99);
    h.bridge.sync();
    assert_eq!(g.get_sub_tab(), 0);

    // Profiles: switching tab loads them on a worker; toggling flips the real (sandbox) state.
    g.invoke_set_sub_tab(2);
    h.wait("profiles", |s| s.game_profile_inflight.is_none() && !s.game_profiles.is_empty());
    assert_eq!(g.get_sub_tab(), 2);
    let first = g.get_profiles().row_data(0).unwrap();
    g.invoke_toggle_profile(first.id.clone());
    assert!(h.state.borrow().game_profile_inflight.is_some(), "apply runs on a worker");
    h.wait("profile toggle", |s| s.game_profile_inflight.is_none());
    let after = g.get_profiles().iter().find(|p| p.id == first.id).unwrap();
    assert_eq!(after.applied, !first.applied);
    g.invoke_toggle_profile("no-such-profile".into());
    h.bridge.sync();
    assert!(h.state.borrow().game_profile_inflight.is_none());

    // Overlay: the switch really changes the (sandbox) registry state.
    g.invoke_set_sub_tab(3);
    h.wait("overlay read", |s| !s.overlay_inflight);
    let before = g.get_game_bar_disabled();
    g.invoke_set_overlay("gamebar".into(), !before);
    h.wait("overlay set", |s| !s.overlay_inflight);
    assert_eq!(g.get_game_bar_disabled(), !before);
    g.invoke_set_overlay("bogus".into(), true);
    h.bridge.sync();
    assert!(!h.state.borrow().overlay_inflight);

    // Suitability is only scored with real CPU / RAM / GPU data.
    g.invoke_set_sub_tab(4);
    h.bridge.sync();
    let has_data = {
        let s = h.state.borrow();
        s.cpu_info.is_some() && s.realtime_metrics.is_some() && !s.gpus.is_empty()
    };
    assert_eq!(g.get_suitability_ready(), has_data);
    if !has_data {
        assert_eq!(g.get_checks().row_count(), 0);
    }
}

#[test]
fn prereqs_scan_filter_and_install_run_on_workers() {
    let h = Harness::scanned();
    h.goto(ActiveView::GamePrerequisites);
    h.wait("prereq scan", |s| !s.is_prereqs_scanning && !s.game_prereqs.is_empty());
    let p = h.window.global::<PrereqStore>();
    let total = p.get_total();
    assert!(total > 0);
    assert_eq!(p.get_installed() + p.get_missing(), total);
    assert_eq!(p.get_items().row_count() as i32, total);

    let first = p.get_items().row_data(0).unwrap();
    p.invoke_filter_edited("zz-no-such-runtime".into());
    h.bridge.sync();
    assert_eq!(p.get_items().row_count(), 0);
    p.invoke_filter_edited(first.name.clone());
    h.bridge.sync();
    assert!(p.get_items().iter().any(|r| r.id == first.id));
    p.invoke_filter_edited("".into());
    h.bridge.sync();

    let installed_before = p.get_installed();
    p.invoke_install(first.id.clone());
    assert!(h.state.borrow().prereq_install_in_flight.is_some(), "install must not block the UI thread");
    p.invoke_install_all(); // ignored while busy
    h.wait("single install", |s| s.prereq_install_in_flight.is_none());
    assert_eq!(p.get_installed(), installed_before + i32::from(!first.installed));

    p.invoke_install_all();
    h.wait("install all", |s| s.prereq_install_in_flight.is_none());
    assert_eq!(p.get_missing(), 0);
    assert!(p.get_items().iter().all(|r| r.installed && r.tone == "success"));
}

#[test]
fn network_probe_and_dns_actions_report_real_outcome() {
    let h = Harness::scanned();
    h.goto(ActiveView::NetworkDiag);
    let n = h.window.global::<NetworkStore>();
    assert!(n.get_probes().row_count() > 0);
    // Nothing measured yet: no invented adapter or ping values.
    assert_eq!(n.get_adapter().as_str(), "—");
    assert!(n.get_probes().iter().all(|r| r.ping.as_str() == "—" && r.tone == "muted"));

    n.invoke_ping();
    assert!(h.state.borrow().network_probe_inflight);
    h.wait("network probe", |s| !s.network_probe_inflight);

    n.invoke_flush_dns();
    assert!(h.state.borrow().network_action_inflight);
    h.wait("flush", |s| !s.network_action_inflight);
    assert!(h.state.borrow().status_message.contains("FlushDns"), "{}", h.state.borrow().status_message);

    // Malformed addresses never reach netsh.
    n.invoke_apply_dns("1.1.1.1; calc".into(), "".into(), "x".into());
    h.bridge.sync();
    assert!(!h.state.borrow().network_action_inflight);
    n.invoke_apply_dns("1.1.1.1".into(), "bad".into(), "x".into());
    h.bridge.sync();
    assert!(!h.state.borrow().network_action_inflight);

    n.invoke_apply_dns(" 1.1.1.1 ".into(), "1.0.0.1".into(), "Cloudflare".into());
    assert!(h.state.borrow().network_action_inflight);
    h.wait("apply dns", |s| !s.network_action_inflight);
    assert!(h.state.borrow().status_message.contains("ApplyDns"), "{}", h.state.borrow().status_message);

    n.invoke_reset_dns();
    h.wait("reset dns", |s| !s.network_action_inflight);
    assert!(h.state.borrow().status_message.contains("ResetDnsDhcp"), "{}", h.state.borrow().status_message);
}

// ───────────────────────────── Support & About ─────────────────────────────

#[test]
fn about_shows_real_version_and_style_studio_switches_style() {
    let h = Harness::scanned();
    h.goto(ActiveView::SupportAbout);
    let app = h.window.global::<AppStore>();
    assert_eq!(app.get_app_version().as_str(), env!("CARGO_PKG_VERSION"));
    let a = h.window.global::<AboutStore>();
    assert!(a.get_repo_url().starts_with("https://github.com/"));
    let styles = app.get_theme_styles();
    assert_eq!(styles.row_count(), 5);
    let target = (0..styles.row_count()).find(|&i| !styles.row_data(i).unwrap().selected).unwrap();
    let before = h.state.borrow().theme_style;
    app.invoke_set_theme_style(target as i32);
    h.bridge.sync();
    assert_ne!(h.state.borrow().theme_style, before);
    assert!(app.get_theme_styles().row_data(target).unwrap().selected);
}


// ───────────────────────────── In-progress HUD ─────────────────────────────

#[test]
fn hud_close_hides_it_and_a_new_operation_shows_it_again() {
    let h = Harness::scanned();
    let app = h.window.global::<AppStore>();
    h.state.borrow_mut().is_disk_scanning = true;
    h.bridge.sync();
    assert!(app.get_hud_visible(), "HUD shows while an operation runs");

    // Close button / outside click / Esc all route to dismiss-hud.
    app.invoke_dismiss_hud();
    h.bridge.sync();
    assert!(!app.get_hud_visible(), "Close must hide the HUD");
    assert!(h.state.borrow().is_disk_scanning, "dismissing never cancels the operation");

    // A different operation starting while the first still runs brings the HUD back.
    h.state.borrow_mut().is_thermal_scanning = true;
    h.bridge.sync();
    assert!(app.get_hud_visible(), "a new operation must show the HUD again");

    // After going idle the dismissal is forgotten, so the same kind of operation shows it again.
    app.invoke_dismiss_hud();
    h.bridge.sync();
    assert!(!app.get_hud_visible());
    {
        let mut st = h.state.borrow_mut();
        st.is_disk_scanning = false;
        st.is_thermal_scanning = false;
        st.drain_worker_events();
        st.is_disk_scanning = true;
    }
    h.bridge.sync();
    assert!(app.get_hud_visible(), "HUD dismissal must not stick after the app went idle");
    h.state.borrow_mut().is_disk_scanning = false;
}

#[test]
fn health_caption_states_the_real_count_not_a_verdict() {
    let h = Harness::scanned();
    h.goto(ActiveView::EasyDashboard);
    let st = h.window.global::<DashboardStore>();
    let (opt, elig) = winblast0::ui_slint::views::dashboard::health_counts(&h.state.borrow().scan_results);
    assert!(elig > 0);
    assert_eq!(st.get_health_caption().as_str(), format!("{opt}/{elig} · Optimized"));
}
