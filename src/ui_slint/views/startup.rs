//! Startup Apps / Scheduled Tasks / Live Services binder (v1.6.1 parity: filters, sorts,
//! hide-Microsoft, batch enable/disable, per-row toggles, run task, service start types).

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::{AppState, StartupSortMode, StartupStatusFilter, StartupTasksSubTab};
use crate::core::i18n::{I18n, TextKey};
use crate::tools::startup::{StartupEntry, StartupSource};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{MainWindow, StartupRow, StartupStore};

pub fn source_label(s: &AppState, src: StartupSource) -> &'static str {
    s.tr(match src {
        StartupSource::RegistryHkcuRun => TextKey::StartupSrcHkcuRun,
        StartupSource::RegistryHklmRun => TextKey::StartupSrcHklmRun,
        StartupSource::StartupFolderAllUsers => TextKey::SrcStartupAllUsers,
        StartupSource::StartupFolderCurrentUser => TextKey::SrcStartupCurrentUser,
        StartupSource::ScheduledTaskLogon => TextKey::SrcLogonTask,
        StartupSource::UwpStartupTask => TextKey::SrcUwpStartup,
    })
}

/// RunOnce entries execute once and cannot be toggled (v1.6.1 rule).
pub fn is_run_once(e: &StartupEntry) -> bool {
    e.approved_subkey.is_none() && (e.id.contains("|RunOnce|") || e.id.contains("|RunOnce32|") || e.reg_path.ends_with("RunOnce"))
}

pub fn next_sort(m: StartupSortMode) -> StartupSortMode {
    match m {
        StartupSortMode::NameAsc => StartupSortMode::NameDesc,
        StartupSortMode::NameDesc => StartupSortMode::StatusActiveFirst,
        StartupSortMode::StatusActiveFirst => StartupSortMode::StatusInactiveFirst,
        StartupSortMode::StatusInactiveFirst => StartupSortMode::Source,
        StartupSortMode::Source => StartupSortMode::NameAsc,
    }
}

fn status_ok(f: StartupStatusFilter, on: bool) -> bool {
    match f {
        StartupStatusFilter::All => true,
        StartupStatusFilter::ActiveOnly => on,
        StartupStatusFilter::InactiveOnly => !on,
    }
}

/// Sort helper shared by the three lists: (name, enabled, source-key).
fn sort_rows(rows: &mut [(StartupRow, String)], mode: StartupSortMode) {
    let lc = |r: &StartupRow| r.name.to_lowercase();
    match mode {
        StartupSortMode::NameAsc => rows.sort_by_key(|(r, _)| lc(r)),
        StartupSortMode::NameDesc => rows.sort_by_key(|(r, _)| std::cmp::Reverse(lc(r))),
        StartupSortMode::StatusActiveFirst => rows.sort_by(|(a, _), (b, _)| b.enabled.cmp(&a.enabled).then_with(|| lc(a).cmp(&lc(b)))),
        StartupSortMode::StatusInactiveFirst => rows.sort_by(|(a, _), (b, _)| a.enabled.cmp(&b.enabled).then_with(|| lc(a).cmp(&lc(b)))),
        StartupSortMode::Source => rows.sort_by(|(a, ka), (b, kb)| ka.cmp(kb).then_with(|| lc(a).cmp(&lc(b)))),
    }
}

pub fn build_rows(s: &AppState) -> Vec<StartupRow> {
    let q = s.startup_filter_edit.text.trim();
    let q = if q.is_empty() { String::new() } else { I18n::fold_search_str(q) };
    let m = |t: &str| q.is_empty() || I18n::fold_search_str(t).contains(&q);
    let hide = s.hide_microsoft_tasks;
    let f = s.startup_status_filter;
    let mut rows: Vec<(StartupRow, String)> = match s.startup_sub_tab {
        StartupTasksSubTab::StartupApps => s
            .startup_entries
            .iter()
            .filter(|e| !(hide && e.is_microsoft) && status_ok(f, e.enabled) && (m(&e.name) || m(&e.command)))
            .map(|e| {
                (
                    StartupRow {
                        id: e.id.as_str().into(),
                        name: e.name.as_str().into(),
                        sub: e.command.as_str().into(),
                        source: source_label(s, e.source).into(),
                        enabled: e.enabled,
                        selected: s.selected_startup_entries.contains(&e.id),
                        run_once: is_run_once(e),
                        locked: e.policy_locked,
                        start_label: SharedString::new(),
                    },
                    format!("{:?}", e.source),
                )
            })
            .collect(),
        StartupTasksSubTab::ScheduledTasks => s
            .scheduled_tasks
            .iter()
            .filter(|t| !(hide && t.is_microsoft) && status_ok(f, t.enabled) && (m(&t.task_name) || m(&t.task_path)))
            .map(|t| {
                (
                    StartupRow {
                        id: t.task_path.as_str().into(),
                        name: t.task_name.as_str().into(),
                        sub: t.task_path.as_str().into(),
                        source: SharedString::new(),
                        enabled: t.enabled,
                        selected: s.selected_scheduled_tasks.contains(&t.task_path),
                        run_once: false,
                        locked: false,
                        start_label: SharedString::new(),
                    },
                    t.folder.clone(),
                )
            })
            .collect(),
        StartupTasksSubTab::Services => s
            .live_services
            .iter()
            .filter(|v| !(hide && v.is_microsoft) && status_ok(f, v.is_running) && (m(&v.service_name) || m(&v.display_name)))
            .map(|v| {
                (
                    StartupRow {
                        id: v.service_name.as_str().into(),
                        name: format!("{} ({})", v.display_name, v.service_name).into(),
                        sub: v.binary_path.as_str().into(),
                        source: SharedString::new(),
                        enabled: v.is_running,
                        selected: false,
                        run_once: false,
                        locked: v.is_protected_core,
                        start_label: s
                            .tr(match v.start_type {
                                2 => TextKey::StartAuto,
                                3 => TextKey::StartManual,
                                4 => TextKey::StartDisabled,
                                _ => TextKey::StartBootSys,
                            })
                            .into(),
                    },
                    v.service_name.clone(),
                )
            })
            .collect(),
    };
    sort_rows(&mut rows, s.startup_sort_mode);
    rows.into_iter().map(|(r, _)| r).collect()
}

fn sort_label(s: &AppState) -> String {
    format!(
        "↕ {}",
        s.tr(match s.startup_sort_mode {
            StartupSortMode::NameAsc => TextKey::SortNameAsc,
            StartupSortMode::NameDesc => TextKey::SortNameDesc,
            StartupSortMode::StatusActiveFirst => TextKey::SortActiveFirst,
            StartupSortMode::StatusInactiveFirst => TextKey::SortInactiveFirst,
            StartupSortMode::Source => TextKey::SortSource,
        })
    )
}

pub struct StartupView {
    rows: Rc<VecModel<StartupRow>>,
}

impl StartupView {
    pub fn new(window: &MainWindow) -> Self {
        let (rows, rc) = new_model();
        window.global::<StartupStore>().set_rows(rc);
        Self { rows }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<StartupStore>();
        let c = ctx.clone();
        st.on_set_sub_tab(move |i| {
            c.act(|s| {
                s.startup_sub_tab = match i {
                    1 => StartupTasksSubTab::ScheduledTasks,
                    2 => StartupTasksSubTab::Services,
                    _ => StartupTasksSubTab::StartupApps,
                }
            });
        });
        let c = ctx.clone();
        st.on_filter_edited(move |t| {
            c.act(|s| s.startup_filter_edit.text = t.to_string());
        });
        let c = ctx.clone();
        st.on_set_status_filter(move |i| {
            c.act(|s| {
                s.startup_status_filter = match i {
                    1 => StartupStatusFilter::ActiveOnly,
                    2 => StartupStatusFilter::InactiveOnly,
                    _ => StartupStatusFilter::All,
                }
            });
        });
        let c = ctx.clone();
        st.on_cycle_sort(move || {
            c.act(|s| s.startup_sort_mode = next_sort(s.startup_sort_mode));
        });
        let c = ctx.clone();
        st.on_toggle_hide_microsoft(move || {
            c.act(|s| s.hide_microsoft_tasks = !s.hide_microsoft_tasks);
        });
        let c = ctx.clone();
        st.on_refresh(move || {
            c.act(|s| {
                if !s.startup_tasks_busy {
                    s.scan_startup_and_tasks_async();
                }
            });
        });
        let c = ctx.clone();
        st.on_select_all_filtered(move || {
            c.act(|s| {
                let (q, hide, f) = (s.startup_filter_edit.text.clone(), s.hide_microsoft_tasks, s.startup_status_filter);
                match s.startup_sub_tab {
                    StartupTasksSubTab::StartupApps => s.select_all_filtered_startup(&q, hide, f),
                    StartupTasksSubTab::ScheduledTasks => s.select_all_filtered_tasks(&q, hide, f),
                    StartupTasksSubTab::Services => {}
                }
            });
        });
        let c = ctx.clone();
        st.on_deselect_all(move || {
            c.act(|s| match s.startup_sub_tab {
                StartupTasksSubTab::StartupApps => s.deselect_all_startup(),
                StartupTasksSubTab::ScheduledTasks => s.deselect_all_tasks(),
                StartupTasksSubTab::Services => {}
            });
        });
        let c = ctx.clone();
        st.on_batch_set(move |enable| {
            c.act(|s| {
                if s.startup_tasks_busy {
                    return;
                }
                match s.startup_sub_tab {
                    StartupTasksSubTab::StartupApps if !s.selected_startup_entries.is_empty() => {
                        s.batch_toggle_selected_startup_async(enable)
                    }
                    StartupTasksSubTab::ScheduledTasks if !s.selected_scheduled_tasks.is_empty() => {
                        s.batch_toggle_selected_tasks_async(enable)
                    }
                    _ => {}
                }
            });
        });
        let c = ctx.clone();
        st.on_toggle_select(move |id| {
            c.act(|s| match s.startup_sub_tab {
                StartupTasksSubTab::StartupApps => s.toggle_startup_entry_selection(id.to_string()),
                StartupTasksSubTab::ScheduledTasks => s.toggle_scheduled_task_selection(id.to_string()),
                StartupTasksSubTab::Services => {}
            });
        });
        let c = ctx.clone();
        st.on_set_enabled(move |id, enable| {
            c.act(|s| {
                if s.startup_tasks_busy {
                    return;
                }
                match s.startup_sub_tab {
                    StartupTasksSubTab::StartupApps => {
                        let allowed = s.startup_entries.iter().find(|e| e.id == id.as_str()).is_some_and(|e| !e.policy_locked && !is_run_once(e));
                        if allowed {
                            s.toggle_startup_entry_async(id.to_string(), enable);
                        }
                    }
                    StartupTasksSubTab::ScheduledTasks => s.toggle_live_task_async(id.to_string(), enable),
                    StartupTasksSubTab::Services => {}
                }
            });
        });
        let c = ctx.clone();
        st.on_run_task(move |path| {
            c.act(|s| {
                if !s.startup_tasks_busy {
                    s.run_live_task_async(path.to_string());
                }
            });
        });
        let c = ctx.clone();
        st.on_set_service_start(move |name, start| {
            let Ok(start) = u32::try_from(start) else { return };
            if !matches!(start, 2..=4) {
                return;
            }
            c.act(|s| {
                let protected = s.live_services.iter().any(|v| v.service_name == name.as_str() && v.is_protected_core);
                if !s.startup_tasks_busy && !protected {
                    s.set_live_service_start_async(name.to_string(), start);
                }
            });
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<StartupStore>();
        st.set_sub_tab(match s.startup_sub_tab {
            StartupTasksSubTab::StartupApps => 0,
            StartupTasksSubTab::ScheduledTasks => 1,
            StartupTasksSubTab::Services => 2,
        });
        st.set_busy(s.startup_tasks_busy);
        if st.get_filter_text().as_str() != s.startup_filter_edit.text {
            st.set_filter_text(s.startup_filter_edit.text.as_str().into());
        }
        st.set_status_filter(match s.startup_status_filter {
            StartupStatusFilter::All => 0,
            StartupStatusFilter::ActiveOnly => 1,
            StartupStatusFilter::InactiveOnly => 2,
        });
        st.set_sort_label(sort_label(s).into());
        st.set_hide_microsoft(s.hide_microsoft_tasks);
        st.set_selected_count(match s.startup_sub_tab {
            StartupTasksSubTab::StartupApps => s.selected_startup_entries.len(),
            StartupTasksSubTab::ScheduledTasks => s.selected_scheduled_tasks.len(),
            StartupTasksSubTab::Services => 0,
        } as i32);
        let rows = build_rows(s);
        st.set_total_count(rows.len() as i32);
        sync_vec(&self.rows, rows);
    }
}
