//! GPU Tuning binder.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::risk::TweakDefinition;
use crate::scanner::TweakState;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{GpuAdapterRow, GpuStore, GpuTweakRow, MainWindow};
use super::matrix::{risk_index, risk_label, state_meta};

/// Curated display order for the vendor silicon cards (v1.6.1).
const PREFERRED_GPU_ORDER: &[&str] = &[
    "4433ce93-b632-4d67-ae71-0241c31099c0",
    "8a4b4c4d-000e-42db-8bd7-293067f8fae7",
    "fa58802f-b98c-4e98-aa67-067c5a394296",
    "df8739ff-e322-4958-b369-df584bb9b0c1",
    "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054",
    "a3836587-c461-4beb-9c4c-88493b36c138",
    "db379bb5-fc8c-4ad3-be32-11be09e76fba",
    "9e7702be-9e03-4c96-a316-c4e435cb993e",
    "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c",
];

pub fn gpu_tweaks(s: &AppState) -> Vec<&TweakDefinition> {
    let mut v: Vec<&TweakDefinition> = s
        .tweaks
        .iter()
        .filter(|t| (t.category == "GPU Tuning" || t.gpu_vendor.is_some()) && !s.hidden_tweaks.contains(&t.id))
        .collect();
    v.sort_by_key(|t| PREFERRED_GPU_ORDER.iter().position(|&id| id == t.id).unwrap_or(999));
    v
}

pub struct GpuView {
    adapters: Rc<VecModel<GpuAdapterRow>>,
    tweaks: Rc<VecModel<GpuTweakRow>>,
}

impl GpuView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<GpuStore>();
        let (adapters, a) = new_model();
        let (tweaks, b) = new_model();
        st.set_adapters(a);
        st.set_tweaks(b);
        Self { adapters, tweaks }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<GpuStore>();
        let c = ctx.clone();
        st.on_toggle_hags(move |enable| {
            c.act(|s| {
                let supported = s.os_info.as_ref().is_none_or(|o| o.supports_hags());
                if supported && !s.hags_inflight && !s.is_rolling_back {
                    s.toggle_hags(enable);
                }
            });
        });
        let c = ctx.clone();
        st.on_toggle_mpo(move |disable| {
            c.act(|s| {
                if !s.mpo_inflight && !s.is_rolling_back {
                    s.toggle_mpo(disable);
                }
            });
        });
        let c = ctx.clone();
        st.on_set_msi(move |pnp, enable, priority| {
            let Ok(priority) = u32::try_from(priority) else { return };
            if !(1..=3).contains(&priority) || pnp.trim().is_empty() {
                return;
            }
            c.act(|s| {
                if !s.gpu_msi_inflight.contains(pnp.as_str()) && !s.is_rolling_back {
                    s.toggle_gpu_msi(pnp.to_string(), enable, priority);
                }
            });
        });
        let c = ctx.clone();
        st.on_hide_tweak(move |id| {
            c.act(|s| s.toggle_hide_tweak(id.to_string()));
        });
        let c = ctx.clone();
        st.on_unhide_all(move || {
            c.act(|s| s.hidden_tweaks.clear());
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<GpuStore>();
        sync_vec(
            &self.adapters,
            s.gpus
                .iter()
                .enumerate()
                .map(|(i, g)| {
                    let valid = !g.pnp_instance_id.trim().is_empty();
                    GpuAdapterRow {
                        index: i as i32,
                        name: g.name.as_str().into(),
                        vendor: g.vendor.as_str().into(),
                        pnp: if valid { g.pnp_instance_id.as_str().into() } else { s.tr(TextKey::StatusNotApplicable).into() },
                        pnp_valid: valid,
                        msi: g.msi_enabled,
                        priority: g.interrupt_priority as i32,
                        busy: s.gpu_msi_inflight.contains(&g.pnp_instance_id),
                    }
                })
                .collect(),
        );
        st.set_hags_supported(s.os_info.as_ref().is_none_or(|o| o.supports_hags()));
        st.set_hags_enabled(s.hags_enabled);
        st.set_hags_inflight(s.hags_inflight);
        st.set_mpo_disabled(s.mpo_disabled);
        st.set_mpo_inflight(s.mpo_inflight);
        st.set_rolling_back(s.is_rolling_back);
        st.set_hidden_count(s.hidden_tweaks.len() as i32);

        let vendors = s.detected_gpu_vendors();
        sync_vec(
            &self.tweaks,
            gpu_tweaks(s)
                .into_iter()
                .map(|t| {
                    let supported = t.is_gpu_vendor_compatible(&vendors);
                    let scan = s.get_tweak_scan(&t.id);
                    let state_text = match scan {
                        Some(sr) => state_meta(s, &sr.current_state).0,
                        None => s.tr(TextKey::StatusReady),
                    };
                    GpuTweakRow {
                        id: t.id.as_str().into(),
                        title: s.tweak_title(&t.id, &t.title).into(),
                        desc: s.tweak_description(&t.id, &t.description).into(),
                        vendor: t.gpu_vendor.as_deref().unwrap_or("GPU").into(),
                        sub_category: t.sub_category.as_deref().map(|sc| s.tweak_sub_category(sc)).unwrap_or("GPU").into(),
                        supported,
                        required: if supported {
                            SharedString::new()
                        } else {
                            format!(
                                "{} {}",
                                t.gpu_vendor.as_deref().unwrap_or_else(|| s.tr(TextKey::CompatibleLabel)),
                                s.tr(TextKey::GpuRequiredAlert)
                            )
                            .into()
                        },
                        active: s.is_switch_active(&t.id),
                        in_flight: s.in_flight_tweaks.contains(&t.id),
                        not_applicable: scan.is_some_and(|sr| sr.current_state == TweakState::NotApplicable),
                        state_text: state_text.into(),
                        risk_level: risk_index(t.valuation.stability_risk),
                        risk_label: risk_label(s, t.valuation.stability_risk).into(),
                    }
                })
                .collect(),
        );
    }
}
