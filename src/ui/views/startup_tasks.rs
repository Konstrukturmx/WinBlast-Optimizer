//! Startup Apps, Scheduled Tasks & Live Services View in Rune.
//!
//! Uses `El::virtual_rows` directly under `.scroll(...)` viewports so 300+ OS services
//! and scheduled tasks render at 60 FPS with O(visible) element materialization.

use std::rc::Rc;

use rune::color::Color;
use rune::{col, row, text, text_input, El, GrowthSpec, ScrollSpec};

use crate::app::{AppState, StartupTasksSubTab};
use crate::core::i18n::{I18n, TextKey};
use crate::tools::startup::StartupSource;
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::{
    action_button, action_button_guarded, pill_badge, switch_toggle_with,
};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

fn sub_tab_btn(
    key: &'static str,
    label: &str,
    active: bool,
    p: &ThemePalette,
    tab: StartupTasksSubTab,
) -> El {
    row()
        .key(key)
        .min_w(0.0)
        .shrink()
        .px(10.0)
        .py(6.0)
        .radius(6.0)
        .fill(if active { p.accent } else { p.bg_primary })
        .hover(if active { p.accent_hover } else { p.bg_card_hover })
        .border(1.0, if active { p.accent } else { p.border })
        .motion(rune::tok::motion::SNAPPY)
        .center()
        .on_press(move |s: &mut AppState| s.startup_sub_tab = tab)
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(if active { Color::WHITE } else { p.text_primary })
                .shrink()
                .wrap(),
        )
}

fn empty_card(msg: &str, p: &ThemePalette) -> El {
    row()
        .w_full()
        .min_w(0.0)
        .padding(18.0)
        .radius(8.0)
        .fill(p.bg_card)
        .border(1.0, p.border)
        .center()
        .child(
            text(msg)
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        )
}

fn startup_source_label(s: &AppState, src: StartupSource) -> &'static str {
    match src {
        StartupSource::RegistryHkcuRun => s.tr(TextKey::StartupSrcHkcuRun),
        StartupSource::RegistryHklmRun => s.tr(TextKey::StartupSrcHklmRun),
        StartupSource::StartupFolderAllUsers => s.tr(TextKey::SrcStartupAllUsers),
        StartupSource::StartupFolderCurrentUser => s.tr(TextKey::SrcStartupCurrentUser),
        StartupSource::ScheduledTaskLogon => s.tr(TextKey::SrcLogonTask),
        StartupSource::UwpStartupTask => s.tr(TextKey::SrcUwpStartup),
    }
}

fn build_startup_apps_list(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let q_raw = s.startup_filter_edit.text.trim();
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let hide_ms = s.hide_microsoft_tasks;
    let idxs: Rc<Vec<usize>> = Rc::new(
        s.startup_entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                (!hide_ms || !e.is_microsoft)
                    && (q.is_empty()
                        || I18n::fold_search_str(&e.name).contains(&q)
                        || I18n::fold_search_str(&e.command).contains(&q))
            })
            .map(|(i, _)| i)
            .collect(),
    );

    let count = idxs.len();
    if count == 0 {
        return empty_card(
            if s.startup_tasks_busy && s.startup_entries.is_empty() {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyStartupList)
            },
            p,
        );
    }

    let k_idxs = Rc::clone(&idxs);
    let b_idxs = Rc::clone(&idxs);
    let row_h = if tier == LayoutTier::Compact { 116.0 } else { 72.0 };

    let mut sc = col()
        .key("startup_scroll")
        .w_full()
        .min_w(0.0)
        .scroll(ScrollSpec { stick_bottom: false });
    sc = if tier == LayoutTier::Compact {
        sc.h(220.0)
    } else {
        sc.grow(1.0).min_h(120.0)
    };

    sc.child(
        col()
            .key("startup_virtual_rows")
            .w_full()
            .growth(GrowthSpec::default())
            .virtual_rows(
                count,
                row_h,
                move |s: &AppState, i: usize| {
                    format!("st_{}", s.startup_entries[k_idxs[i]].id).into()
                },
                move |s: &AppState, i: usize| {
                    let e = &s.startup_entries[b_idxs[i]];
                    let p = s.palette();
                    let target = !e.enabled;
                    let locked = e.policy_locked;
                    let is_run_once = e.approved_subkey.is_none()
                        && (e.id.contains("|RunOnce|")
                            || e.id.contains("|RunOnce32|")
                            || e.reg_path.ends_with("RunOnce"));
                    let id_sw = e.id.clone();
                    let id_btn = e.id.clone();
                    let sw_key = format!("st_sw_{}", e.id);

                    let toggle_ctl = if is_run_once {
                        pill_badge(s.tr(TextKey::RunOnceBadge), p.accent, p)
                    } else if locked {
                        pill_badge(s.tr(TextKey::PolicyLockedBadge), p.warning, p)
                    } else {
                        action_button_guarded(
                            if e.enabled {
                                s.tr(TextKey::ToggleDisable)
                            } else {
                                s.tr(TextKey::ToggleEnable)
                            },
                            if e.enabled { p.btn_secondary } else { p.accent },
                            if e.enabled { p.text_primary } else { Color::WHITE },
                            !s.startup_tasks_busy,
                            p,
                            move |s| s.toggle_startup_entry_async(id_btn.clone(), target),
                        )
                    };

                    col()
                        .w_full()
                        .min_w(0.0)
                        .pb(6.0)
                        .child(
                            row()
                                .w_full()
                                .min_w(0.0)
                                .padding(8.0)
                                .radius(6.0)
                                .fill(p.bg_card)
                                .border(1.0, p.border)
                                .flex_wrap()
                                .items_center()
                                .justify_between()
                                .gaps(6.0, 10.0)
                                .child(
                                    row()
                                        .min_w(0.0)
                                        .grow(1.0)
                                        .items_center()
                                        .gap(8.0)
                                        .child(switch_toggle_with(
                                            e.enabled,
                                            s.startup_tasks_busy,
                                            sw_key,
                                            p,
                                            move |s: &mut AppState| {
                                                if !locked && !is_run_once {
                                                    s.toggle_startup_entry_async(id_sw.clone(), target);
                                                }
                                            },
                                        ))
                                        .child(
                                            col()
                                                .grow(1.0)
                                                .min_w(120.0)
                                                .gap(2.0)
                                                .child(
                                                    text(&e.name)
                                                        .font(rune::tok::type_::SECTION)
                                                        .color(p.text_primary)
                                                        .shrink()
                                                        .truncate(),
                                                )
                                                .child(
                                                    text(&e.command)
                                                        .font(rune::tok::type_::LABEL)
                                                        .color(p.text_muted)
                                                        .shrink()
                                                        .truncate(),
                                                ),
                                        ),
                                )
                                .child(
                                    row()
                                        .min_w(0.0)
                                        .shrink()
                                        .items_center()
                                        .gap(6.0)
                                        .child(pill_badge(startup_source_label(s, e.source), p.accent, p))
                                        .child(toggle_ctl),
                                ),
                        )
                },
            ),
    )
    .winblast_scroll(p)
}

fn build_tasks_list(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let q_raw = s.startup_filter_edit.text.trim();
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let hide_ms = s.hide_microsoft_tasks;
    let idxs: Rc<Vec<usize>> = Rc::new(
        s.scheduled_tasks
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                (!hide_ms || !t.is_microsoft)
                    && (q.is_empty()
                        || I18n::fold_search_str(&t.task_name).contains(&q)
                        || I18n::fold_search_str(&t.task_path).contains(&q))
            })
            .map(|(i, _)| i)
            .collect(),
    );

    let count = idxs.len();
    if count == 0 {
        return empty_card(
            if s.startup_tasks_busy && s.scheduled_tasks.is_empty() {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyTasksList)
            },
            p,
        );
    }

    let k_idxs = Rc::clone(&idxs);
    let b_idxs = Rc::clone(&idxs);
    let row_h = if tier == LayoutTier::Compact { 116.0 } else { 72.0 };

    let mut sc = col()
        .key("tasks_scroll")
        .w_full()
        .min_w(0.0)
        .scroll(ScrollSpec { stick_bottom: false });
    sc = if tier == LayoutTier::Compact {
        sc.h(220.0)
    } else {
        sc.grow(1.0).min_h(120.0)
    };

    sc.child(
        col()
            .key("tasks_virtual_rows")
            .w_full()
            .growth(GrowthSpec::default())
            .virtual_rows(
                count,
                row_h,
                move |s: &AppState, i: usize| {
                    format!("tsk_{}", s.scheduled_tasks[k_idxs[i]].task_path).into()
                },
                move |s: &AppState, i: usize| {
                    let t = &s.scheduled_tasks[b_idxs[i]];
                    let p = s.palette();
                    let target = !t.enabled;
                    let path_toggle = t.task_path.clone();
                    let path_run = t.task_path.clone();

                    col()
                        .w_full()
                        .min_w(0.0)
                        .pb(6.0)
                        .child(
                            row()
                                .w_full()
                                .min_w(0.0)
                                .padding(8.0)
                                .radius(6.0)
                                .fill(p.bg_card)
                                .border(1.0, p.border)
                                .flex_wrap()
                                .items_center()
                                .justify_between()
                                .gaps(6.0, 10.0)
                                .child(
                                    col()
                                        .grow(1.0)
                                        .min_w(140.0)
                                        .gap(2.0)
                                        .child(
                                            text(&t.task_name)
                                                .font(rune::tok::type_::SECTION)
                                                .color(p.text_primary)
                                                .shrink()
                                                .truncate(),
                                        )
                                        .child(
                                            text(&t.task_path)
                                                .font(rune::tok::type_::LABEL)
                                                .color(p.text_muted)
                                                .shrink()
                                                .truncate(),
                                        ),
                                )
                                .child(
                                    row()
                                        .min_w(0.0)
                                        .shrink()
                                        .items_center()
                                        .gap(6.0)
                                        .child(pill_badge(
                                            if t.enabled {
                                                s.tr(TextKey::StateEnabled)
                                            } else {
                                                s.tr(TextKey::StateDisabled)
                                            },
                                            if t.enabled { p.success } else { p.text_muted },
                                            p,
                                        ))
                                        .child(action_button_guarded(
                                            s.tr(TextKey::RunTaskBtn),
                                            p.btn_secondary,
                                            p.text_primary,
                                            !s.startup_tasks_busy,
                                            p,
                                            move |s| s.run_live_task_async(path_run.clone()),
                                        ))
                                        .child(action_button_guarded(
                                            if t.enabled {
                                                s.tr(TextKey::ToggleDisable)
                                            } else {
                                                s.tr(TextKey::ToggleEnable)
                                            },
                                            if t.enabled { p.warning.alpha(0.16) } else { p.accent },
                                            if t.enabled { p.warning } else { Color::WHITE },
                                            !s.startup_tasks_busy,
                                            p,
                                            move |s| s.toggle_live_task_async(path_toggle.clone(), target),
                                        )),
                                ),
                        )
                },
            ),
    )
    .winblast_scroll(p)
}

fn build_services_list(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let q_raw = s.startup_filter_edit.text.trim();
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let hide_ms = s.hide_microsoft_tasks;
    let idxs: Rc<Vec<usize>> = Rc::new(
        s.live_services
            .iter()
            .enumerate()
            .filter(|(_, svc)| {
                (!hide_ms || !svc.is_microsoft)
                    && (q.is_empty()
                        || I18n::fold_search_str(&svc.service_name).contains(&q)
                        || I18n::fold_search_str(&svc.display_name).contains(&q))
            })
            .map(|(i, _)| i)
            .collect(),
    );

    let count = idxs.len();
    if count == 0 {
        return empty_card(
            if s.startup_tasks_busy && s.live_services.is_empty() {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyServicesList)
            },
            p,
        );
    }

    let k_idxs = Rc::clone(&idxs);
    let b_idxs = Rc::clone(&idxs);
    let row_h = if tier == LayoutTier::Compact { 140.0 } else { 76.0 };

    let mut sc = col()
        .key("services_scroll")
        .w_full()
        .min_w(0.0)
        .scroll(ScrollSpec { stick_bottom: false });
    sc = if tier == LayoutTier::Compact {
        sc.h(220.0)
    } else {
        sc.grow(1.0).min_h(120.0)
    };

    sc.child(
        col()
            .key("services_virtual_rows")
            .w_full()
            .growth(GrowthSpec::default())
            .virtual_rows(
                count,
                row_h,
                move |s: &AppState, i: usize| {
                    format!("svc_{}", s.live_services[k_idxs[i]].service_name).into()
                },
                move |s: &AppState, i: usize| {
                    let svc = &s.live_services[b_idxs[i]];
                    let p = s.palette();
                    let start_lbl = match svc.start_type {
                        2 => s.tr(TextKey::StartAuto),
                        3 => s.tr(TextKey::StartManual),
                        4 => s.tr(TextKey::StartDisabled),
                        _ => s.tr(TextKey::StartBootSys),
                    };

                    let mut actions = row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .items_center()
                        .gaps(4.0, 4.0)
                        .child(pill_badge(
                            if svc.is_running {
                                s.tr(TextKey::StateRunning)
                            } else {
                                s.tr(TextKey::StateStopped)
                            },
                            if svc.is_running { p.success } else { p.text_muted },
                            p,
                        ))
                        .child(pill_badge(start_lbl, p.accent, p));

                    if svc.is_protected_core {
                        actions = actions.child(pill_badge(s.tr(TextKey::ProtectedCoreBadge), p.danger, p));
                    } else {
                        let s_auto = svc.service_name.clone();
                        let s_man = svc.service_name.clone();
                        let s_dis = svc.service_name.clone();
                        actions = actions
                            .child(action_button_guarded(
                                s.tr(TextKey::StartManual),
                                p.btn_secondary,
                                p.text_primary,
                                !s.startup_tasks_busy,
                                p,
                                move |s| s.set_live_service_start_async(s_man.clone(), 3),
                            ))
                            .child(action_button_guarded(
                                s.tr(TextKey::StartAuto),
                                p.btn_secondary,
                                p.text_primary,
                                !s.startup_tasks_busy,
                                p,
                                move |s| s.set_live_service_start_async(s_auto.clone(), 2),
                            ))
                            .child(action_button_guarded(
                                s.tr(TextKey::ToggleDisable),
                                p.warning.alpha(0.16),
                                p.warning,
                                !s.startup_tasks_busy,
                                p,
                                move |s| s.set_live_service_start_async(s_dis.clone(), 4),
                            ));
                    }

                    col()
                        .w_full()
                        .min_w(0.0)
                        .pb(6.0)
                        .child(
                            row()
                                .w_full()
                                .min_w(0.0)
                                .padding(8.0)
                                .radius(6.0)
                                .fill(p.bg_card)
                                .border(1.0, p.border)
                                .flex_wrap()
                                .items_center()
                                .justify_between()
                                .gaps(6.0, 10.0)
                                .child(
                                    col()
                                        .grow(1.0)
                                        .min_w(140.0)
                                        .gap(2.0)
                                        .child(
                                            text(format!("{} ({})", svc.display_name, svc.service_name))
                                                .font(rune::tok::type_::SECTION)
                                                .color(p.text_primary)
                                                .shrink()
                                                .truncate(),
                                        )
                                        .child(
                                            text(&svc.binary_path)
                                                .font(rune::tok::type_::LABEL)
                                                .color(p.text_muted)
                                                .shrink()
                                                .truncate(),
                                        ),
                                )
                                .child(actions),
                        )
                },
            ),
    )
    .winblast_scroll(p)
}

/// Renders Startup Apps, Scheduled Tasks & Services inside responsive virtualized lists.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };

    let mut title_row = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .items_center()
        .justify_between()
        .gaps(6.0, 8.0)
        .child(
            text(s.tr(TextKey::StartupTasksTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        );
    if s.startup_tasks_busy {
        title_row = title_row.child(pill_badge(s.tr(TextKey::BusyWorking), p.accent, p));
    }

    let header = col()
        .w_full()
        .min_w(0.0)
        .gap(6.0)
        .child(title_row)
        .child(
            text(s.tr(TextKey::StartupTasksSub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(8.0)
                .radius(8.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .flex_wrap()
                .items_center()
                .justify_between()
                .gaps(6.0, 8.0)
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(sub_tab_btn(
                            "sub_startup_apps",
                            s.tr(TextKey::SubTabStartupApps),
                            s.startup_sub_tab == StartupTasksSubTab::StartupApps,
                            p,
                            StartupTasksSubTab::StartupApps,
                        ))
                        .child(sub_tab_btn(
                            "sub_startup_tasks",
                            s.tr(TextKey::SubTabScheduledTasks),
                            s.startup_sub_tab == StartupTasksSubTab::ScheduledTasks,
                            p,
                            StartupTasksSubTab::ScheduledTasks,
                        ))
                        .child(sub_tab_btn(
                            "sub_startup_services",
                            s.tr(TextKey::SubTabServices),
                            s.startup_sub_tab == StartupTasksSubTab::Services,
                            p,
                            StartupTasksSubTab::Services,
                        )),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(action_button(
                            format!(
                                "{} {}",
                                if s.hide_microsoft_tasks { "☑" } else { "☐" },
                                s.tr(TextKey::HideMicrosoftTasks)
                            ),
                            if s.hide_microsoft_tasks { p.accent } else { p.btn_secondary },
                            if s.hide_microsoft_tasks { Color::WHITE } else { p.text_primary },
                            p,
                            |s| s.hide_microsoft_tasks = !s.hide_microsoft_tasks,
                        ))
                        .child(action_button_guarded(
                            s.tr(TextKey::RefreshStartupInventory),
                            p.btn_secondary,
                            p.text_primary,
                            !s.startup_tasks_busy,
                            p,
                            |s| s.scan_startup_and_tasks_async(),
                        )),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .min_h(34.0)
                .px(8.0)
                .py(5.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .items_center()
                .child(
                    text_input(&s.startup_filter_edit, rune::tok::type_::BODY)
                        .key("startup_search_input")
                        .single_line()
                        .placeholder(s.tr(TextKey::SearchPlaceholder), p.text_muted)
                        .color(p.text_primary)
                        .on_edit(|s: &mut AppState| &mut s.startup_filter_edit)
                        .grow(1.0)
                        .min_w(0.0),
                ),
        );

    let body = match s.startup_sub_tab {
        StartupTasksSubTab::StartupApps => build_startup_apps_list(s, p, tier),
        StartupTasksSubTab::ScheduledTasks => build_tasks_list(s, p, tier),
        StartupTasksSubTab::Services => build_services_list(s, p, tier),
    };

    let outer = col()
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(8.0)
        .child(header)
        .child(body);

    if tier == LayoutTier::Compact {
        outer.key("startup_tasks_compact_viewport").scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
    } else {
        outer
    }
}
