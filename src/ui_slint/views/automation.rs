//! Automation Center binder: auto-tune profiles + plan preview, memory trim, health
//! assessment and scheduled maintenance tasks.

use std::rc::Rc;

use slint::{ComponentHandle, VecModel};

use crate::app::AppState;
use crate::scanner::TweakState;
use crate::tools::automation::{create_auto_tune_plan, AutoTuneProfile, MaintenanceTaskKind};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{AutomationStore, ChipItem, MainWindow, MaintenanceRow};

pub const PROFILES: [(AutoTuneProfile, &str, &str); 4] = [
    (AutoTuneProfile::GamingLowLatency, "GamingLowLatency", "🎮 Ultra Gaming"),
    (AutoTuneProfile::ProAudioDpc, "ProAudioDpc", "🎵 Pro Audio / DAW"),
    (AutoTuneProfile::WorkstationStability, "WorkstationStability", "🛡 Privacy & Workstation"),
    (AutoTuneProfile::LaptopBatterySaving, "LaptopBatterySaving", "🔋 Laptop Battery"),
];

pub struct AutomationView {
    plan: Rc<VecModel<ChipItem>>,
    maintenance: Rc<VecModel<MaintenanceRow>>,
}

impl AutomationView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<AutomationStore>();
        let (plan, a) = new_model();
        let (maintenance, b) = new_model();
        st.set_plan(a);
        st.set_maintenance(b);
        Self { plan, maintenance }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<AutomationStore>();
        let c = ctx.clone();
        st.on_run_auto_tune(move || {
            c.act(|s| {
                let p = s.selected_auto_tune_profile;
                s.execute_auto_tune_profile(p);
            });
        });
        let c = ctx.clone();
        st.on_trim_memory(move || {
            c.act(|s| s.execute_memory_trim());
        });
        let c = ctx.clone();
        st.on_set_profile(move |id| {
            if let Some((p, _, _)) = PROFILES.iter().find(|(_, pid, _)| *pid == id.as_str()) {
                let p = *p;
                c.act(|s| s.selected_auto_tune_profile = p);
            }
        });
        let c = ctx.clone();
        st.on_assess_health(move || {
            c.act(|s| s.assess_system_health());
        });
        let c = ctx.clone();
        st.on_refresh_maintenance(move || {
            c.act(|s| s.refresh_maintenance_tasks());
        });
        let c = ctx.clone();
        st.on_toggle_maintenance(move |id| {
            if let Some(kind) = MaintenanceTaskKind::all().iter().copied().find(|k| k.task_name() == id.as_str()) {
                c.act(|s| s.toggle_maintenance_task(kind));
            }
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<AutomationStore>();
        st.set_running(s.is_running_automation);
        st.set_assessing(s.is_assessing_health);
        st.set_maintenance_busy(s.maintenance_inflight);
        let (_, pid, plabel) = PROFILES.iter().find(|(p, _, _)| *p == s.selected_auto_tune_profile).copied().unwrap_or(PROFILES[0]);
        st.set_profile(pid.into());
        st.set_profile_label(plabel.into());

        let plan = create_auto_tune_plan(&s.tweaks, s.selected_auto_tune_profile);
        st.set_plan_count(plan.recommended_tweak_ids.len() as i32);
        st.set_plan_more(plan.recommended_tweak_ids.len().saturating_sub(24) as i32);
        sync_vec(
            &self.plan,
            plan.recommended_tweak_ids
                .iter()
                .take(24)
                .filter_map(|id| s.tweaks.iter().find(|t| &t.id == id))
                .map(|t| {
                    let opt = s.get_tweak_scan(&t.id).is_some_and(|r| r.current_state == TweakState::AlreadyOptimized);
                    ChipItem {
                        id: t.id.as_str().into(),
                        label: format!("{} {}", if opt { "✓" } else { "•" }, s.tweak_title(&t.id, &t.title)).into(),
                        active: opt,
                    }
                })
                .collect(),
        );

        match &s.auto_tune_result {
            Some(r) => {
                st.set_has_result(true);
                st.set_result_message(r.message.as_str().into());
                st.set_result_duration(format!("{} ms", r.duration_ms).into());
                st.set_result_applied(r.tweaks_applied as i32);
                st.set_result_failed(r.tweaks_failed as i32);
            }
            None => st.set_has_result(false),
        }
        match &s.last_memory_trim_result {
            Some(t) => {
                st.set_has_trim(true);
                st.set_trim_text(
                    if t.success {
                        format!("Freed ≈{:.1} MB • {} Processes Trimmed", t.freed_mb_estimated, t.processes_trimmed)
                    } else {
                        t.message.clone()
                    }
                    .into(),
                );
            }
            None => st.set_has_trim(false),
        }
        match &s.system_health_assessment {
            Some(h) => {
                st.set_has_health(true);
                st.set_health_grade(format!("{} (Score: {}/100)", h.overall_grade.badge_text(), h.score).into());
                st.set_health_detail(
                    format!("Telemetry: {} • Latency: {} • Services: {}", h.telemetry_leak_grade, h.dpc_latency_grade, h.bloat_services_grade).into(),
                );
            }
            None => st.set_has_health(false),
        }
        sync_vec(
            &self.maintenance,
            s.maintenance_tasks
                .iter()
                .map(|t| MaintenanceRow {
                    id: t.kind.task_name().into(),
                    name: t.task_name.as_str().into(),
                    desc: t.kind.description().into(),
                    schedule: t.schedule.as_str().into(),
                    installed: t.is_installed,
                })
                .collect(),
        );
    }
}
