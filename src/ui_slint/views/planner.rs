//! Goal Planner binder (A* plan, Raven conflict resolution, Dijkstra execution order).

use std::rc::Rc;

use slint::{ComponentHandle, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::graph::astar::OptimizationGoal;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{MainWindow, PlanMetric, PlanStep, PlannerStore};

pub fn goal_id(name: &str) -> &'static str {
    if name.contains("Gaming") {
        "gaming"
    } else if name.contains("Privacy") {
        "privacy"
    } else if name.contains("Balanced") {
        "balanced"
    } else {
        ""
    }
}

pub struct PlannerView {
    metrics: Rc<VecModel<PlanMetric>>,
    steps: Rc<VecModel<PlanStep>>,
}

impl PlannerView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<PlannerStore>();
        let (metrics, a) = new_model();
        let (steps, b) = new_model();
        st.set_metrics(a);
        st.set_steps(b);
        Self { metrics, steps }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<PlannerStore>();
        let c = ctx.clone();
        st.on_solve(move || {
            c.act(|s| s.solve_current_goal());
        });
        let c = ctx.clone();
        st.on_stage(move || {
            c.act(|s| {
                if s.planned_path.is_some() {
                    s.stage_planned_goal();
                }
            });
        });
        let c = ctx.clone();
        st.on_select_goal(move |g| {
            let goal = match g.as_str() {
                "gaming" => OptimizationGoal::ultra_gaming(),
                "privacy" => OptimizationGoal::stealth_privacy(),
                "balanced" => OptimizationGoal::balanced_daily(),
                _ => return,
            };
            c.act(|s| {
                s.active_goal = goal;
                s.solve_current_goal();
            });
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<PlannerStore>();
        st.set_active_goal(goal_id(&s.active_goal.name).into());
        st.set_has_plan(s.planned_path.is_some());
        let selected = s.planned_path.as_ref().map_or(0, |p| p.selected_tweak_ids.len());
        let perf = s.planned_path.as_ref().map_or(0, |p| p.achieved_perf_gain);
        let privacy = s.planned_path.as_ref().map_or(0, |p| p.achieved_privacy_gain);
        let cost = s.scheduled_order.as_ref().map_or(0, |o| o.total_cost);
        let reboots = s.scheduled_order.as_ref().map_or(0, |o| o.total_reboots_required);
        let m = |k: TextKey, v: String, tone: &str| PlanMetric { label: s.tr(k).into(), value: v.into(), tone: tone.into() };
        sync_vec(
            &self.metrics,
            vec![
                m(TextKey::GoalMetricTweaks, selected.to_string(), "accent"),
                m(TextKey::GoalMetricPerf, format!("{}/{}", perf, s.active_goal.min_perf_gain), "warning"),
                m(TextKey::GoalMetricPrivacy, format!("{}/{}", privacy, s.active_goal.min_privacy_gain), "accent"),
                m(TextKey::GoalMetricCost, cost.to_string(), "neutral"),
                m(TextKey::GoalMetricReboots, reboots.to_string(), if reboots > 0 { "danger" } else { "success" }),
                m(TextKey::GoalMetricConflicts, s.raven_resolutions.len().to_string(), "accent"),
            ],
        );
        sync_vec(
            &self.steps,
            s.scheduled_order
                .as_ref()
                .map(|o| {
                    o.steps
                        .iter()
                        .enumerate()
                        .map(|(i, step)| {
                            let def = s.tweaks.iter().find(|t| t.id == step.tweak_id);
                            let title = def.map_or(step.tweak_id.as_str(), |t| s.tweak_title(&t.id, &t.title));
                            let cat = def.map_or("Optimization", |t| s.tweak_category(&t.category));
                            let (badge, tone) = if step.requires_reboot {
                                (s.tr(TextKey::GoalStepReboot), "danger")
                            } else if step.touches_services {
                                (s.tr(TextKey::GoalStepService), "warning")
                            } else {
                                (s.tr(TextKey::GoalStepInstant), "success")
                            };
                            PlanStep {
                                index: format!("#{}", i + 1).into(),
                                title: title.into(),
                                sub: format!("{cat} | Cost: {}", step.cumulative_cost).into(),
                                badge: badge.into(),
                                tone: tone.into(),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
        );
    }
}
