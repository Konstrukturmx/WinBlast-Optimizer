//! A* Goal Planner, Raven Pareto Conflict Evaluator & Dijkstra Scheduler View in Rune.
//!
//! Provides an interactive interface for goal-driven optimization planning,
//! evaluating multi-criteria tradeoffs and displaying dependency-ordered execution steps.

use rune::color::Color;
use rune::{col, el, row, text, El};

use crate::app::AppState;
use crate::graph::astar::OptimizationGoal;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

/// Renders the Goal-Driven Graph Planner view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();

    // 1. Header Card
    let header_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .motion(p.motion)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(10.0, 16.0)
        .child(
            col()
                .grow(1.0)
                .min_w(200.0)
                .gap(4.0)
                .child(
                    text("A* Goal Planner & Raven Optimizer")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text("Heuristic goal-driven solver with Pareto conflict resolution and Dijkstra execution sequencing.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            row()
                .gap(8.0)
                .child(
                    action_button(
                        "⚡ Solve A* Path",
                        p.accent,
                        Color::WHITE,
                        p,
                        |st| st.solve_current_goal(),
                    )
                )
                .child(
                    action_button_guarded(
                        "📥 Stage Plan Tweaks",
                        p.success,
                        Color::WHITE,
                        s.planned_path.is_some(),
                        p,
                        |st| st.stage_planned_goal(),
                    )
                )
        );

    // 2. Goal Profiles Selector Row
    let is_gaming = s.active_goal.name.contains("Gaming");
    let is_privacy = s.active_goal.name.contains("Privacy");
    let is_balanced = s.active_goal.name.contains("Balanced");

    let goal_selector = row()
        .w_full()
        .gap(12.0)
        .flex_wrap()
        .child(
            col()
                .grow(1.0)
                .min_w(220.0)
                .padding(p.card_padding.min(12.0))
                .radius(p.card_radius)
                .fill(if is_gaming { p.bg_card_hover } else { p.bg_card })
                .border(if is_gaming { p.border_width + 1.0 } else { p.border_width }, if is_gaming { p.accent } else { p.border })
                .motion(p.motion)
                .gap(6.0)
                .child(
                    text("⚡ Ultra Gaming Rig")
                        .font(rune::tok::type_::TITLE)
                        .color(if is_gaming { p.accent } else { p.text_primary }),
                )
                .child(
                    text("Min Perf Gain: 25 | Max Risk: Moderate | Reboots Allowed")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted),
                )
                .child(
                    action_button(
                        if is_gaming { "✓ Active Goal" } else { "Select Gaming" },
                        if is_gaming { p.accent } else { p.btn_secondary },
                        if is_gaming { Color::WHITE } else { p.text_primary },
                        p,
                        |st| {
                            st.active_goal = OptimizationGoal::ultra_gaming();
                            st.solve_current_goal();
                        },
                    )
                )
        )
        .child(
            col()
                .grow(1.0)
                .min_w(220.0)
                .padding(p.card_padding.min(12.0))
                .radius(p.card_radius)
                .fill(if is_privacy { p.bg_card_hover } else { p.bg_card })
                .border(if is_privacy { p.border_width + 1.0 } else { p.border_width }, if is_privacy { p.accent } else { p.border })
                .motion(p.motion)
                .gap(6.0)
                .child(
                    text("🛡️ Stealth Privacy Fortress")
                        .font(rune::tok::type_::TITLE)
                        .color(if is_privacy { p.accent } else { p.text_primary }),
                )
                .child(
                    text("Min Privacy Gain: 30 | Max Risk: Low | Zero Reboots")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted),
                )
                .child(
                    action_button(
                        if is_privacy { "✓ Active Goal" } else { "Select Privacy" },
                        if is_privacy { p.accent } else { p.btn_secondary },
                        if is_privacy { Color::WHITE } else { p.text_primary },
                        p,
                        |st| {
                            st.active_goal = OptimizationGoal::stealth_privacy();
                            st.solve_current_goal();
                        },
                    )
                )
        )
        .child(
            col()
                .grow(1.0)
                .min_w(220.0)
                .padding(p.card_padding.min(12.0))
                .radius(p.card_radius)
                .fill(if is_balanced { p.bg_card_hover } else { p.bg_card })
                .border(if is_balanced { p.border_width + 1.0 } else { p.border_width }, if is_balanced { p.accent } else { p.border })
                .motion(p.motion)
                .gap(6.0)
                .child(
                    text("⚖️ Balanced Daily Driver")
                        .font(rune::tok::type_::TITLE)
                        .color(if is_balanced { p.accent } else { p.text_primary }),
                )
                .child(
                    text("Min Gain: 15 / 15 | 100% Safe Only | Zero Reboots")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted),
                )
                .child(
                    action_button(
                        if is_balanced { "✓ Active Goal" } else { "Select Balanced" },
                        if is_balanced { p.accent } else { p.btn_secondary },
                        if is_balanced { Color::WHITE } else { p.text_primary },
                        p,
                        |st| {
                            st.active_goal = OptimizationGoal::balanced_daily();
                            st.solve_current_goal();
                        },
                    )
                )
        );

    // 3. Metrics Summary Bar
    let selected_count = s.planned_path.as_ref().map(|p| p.selected_tweak_ids.len()).unwrap_or(0);
    let total_perf = s.planned_path.as_ref().map(|p| p.achieved_perf_gain).unwrap_or(0);
    let total_priv = s.planned_path.as_ref().map(|p| p.achieved_privacy_gain).unwrap_or(0);
    let total_cost = s.scheduled_order.as_ref().map(|o| o.total_cost).unwrap_or(0);
    let total_reboots = s.scheduled_order.as_ref().map(|o| o.total_reboots_required).unwrap_or(0);
    let conflicts_resolved = s.raven_resolutions.len();

    let metrics_bar = row()
        .w_full()
        .gap(10.0)
        .flex_wrap()
        .child(
            metric_pill("Optimal Tweaks Found", &format!("{}", selected_count), p.accent, p)
        )
        .child(
            metric_pill("Total Perf Gain", &format!("{}/{}", total_perf, s.active_goal.min_perf_gain), p.warning, p)
        )
        .child(
            metric_pill("Total Privacy Gain", &format!("{}/{}", total_priv, s.active_goal.min_privacy_gain), p.accent, p)
        )
        .child(
            metric_pill("Dijkstra Disruption Cost", &format!("{}", total_cost), p.text_primary, p)
        )
        .child(
            metric_pill("Reboots Needed", &format!("{}", total_reboots), if total_reboots > 0 { p.danger } else { p.success }, p)
        )
        .child(
            metric_pill("Conflicts Resolved", &format!("{}", conflicts_resolved), p.accent, p)
        );

    // 4. Scheduled Execution Sequence Timeline
    let mut schedule_col = col().w_full().gap(6.0);

    if let Some(order) = &s.scheduled_order {
        if order.steps.is_empty() {
            schedule_col = schedule_col.child(
                text("Click '⚡ Solve A* Path' to calculate the optimal execution sequence.")
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .py(20.0),
            );
        } else {
            for (idx, step) in order.steps.iter().enumerate() {
                let tweak_title = s.tweaks
                    .iter()
                    .find(|t| t.id == step.tweak_id)
                    .map(|t| t.title.as_str())
                    .unwrap_or(&step.tweak_id);

                let cat = s.tweaks
                    .iter()
                    .find(|t| t.id == step.tweak_id)
                    .map(|t| t.category.as_str())
                    .unwrap_or("Optimization");

                let item = row()
                    .w_full()
                    .padding(8.0)
                    .radius(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .items_center()
                    .justify_between()
                    .gap(10.0)
                    .child(
                        row()
                            .items_center()
                            .gap(10.0)
                            .child(
                                el()
                                    .w(28.0)
                                    .h(28.0)
                                    .radius(14.0)
                                    .fill(p.bg_card_hover)
                                    .border(1.0, p.border)
                                    .center()
                                    .child(
                                        text(format!("#{}", idx + 1))
                                            .font(rune::tok::type_::LABEL)
                                            .color(p.accent),
                                    )
                            )
                            .child(
                                col()
                                    .gap(2.0)
                                    .child(
                                        text(tweak_title)
                                            .font(rune::tok::type_::BODY)
                                            .color(p.text_primary),
                                    )
                                    .child(
                                        text(format!("{} | Cost: {}", cat, step.cumulative_cost))
                                            .font(rune::tok::type_::LABEL)
                                            .color(p.text_muted),
                                    )
                            )
                    )
                    .child(
                        row()
                            .gap(6.0)
                            .items_center()
                            .child(
                                if step.requires_reboot {
                                    text("⚠️ Reboot Required")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.warning)
                                } else if step.touches_services {
                                    text("🔄 Service Restart")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.accent)
                                } else {
                                    text("⚡ Instant")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.success)
                                }
                            )
                    );

                schedule_col = schedule_col.child(item);
            }
        }
    } else {
        schedule_col = schedule_col.child(
            text("Click '⚡ Solve A* Path' to calculate the optimal execution sequence.")
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .py(20.0),
        );
    }

    let timeline_card = col()
        .w_full()
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(10.0)
        .child(
            text("Topological Execution Schedule (Dijkstra Shortest Disruption Path)")
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary),
        )
        .child(schedule_col);

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    col()
        .key("goal_planner_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(14.0)
        .scroll(rune::ScrollSpec { stick_bottom: false })
        .child(header_card)
        .child(goal_selector)
        .child(metrics_bar)
        .child(timeline_card)
        .winblast_scroll(p)
}

fn metric_pill(label: &str, value: &str, accent_color: Color, p: &crate::ui::theme::ThemePalette) -> El {
    col()
        .grow(1.0)
        .min_w(130.0)
        .padding(10.0)
        .radius(6.0)
        .fill(p.bg_card)
        .border(1.0, p.border)
        .gap(2.0)
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted),
        )
        .child(
            text(value)
                .font(rune::tok::type_::TITLE)
                .color(accent_color),
        )
}
