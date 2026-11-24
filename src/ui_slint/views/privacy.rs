//! Privacy Shield binder.

use std::rc::Rc;

use slint::{ComponentHandle, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::tweak_i18n::is_privacy_or_telemetry_category;
use crate::scanner::TweakState;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{MainWindow, MatrixRow, PrivacyStore};
use super::matrix::{risk_index, risk_label, state_meta};

pub struct PrivacyView {
    rows: Rc<VecModel<MatrixRow>>,
}

impl PrivacyView {
    pub fn new(window: &MainWindow) -> Self {
        let (rows, rc) = new_model();
        window.global::<PrivacyStore>().set_rows(rc);
        Self { rows }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<PrivacyStore>();
        let c = ctx.clone();
        st.on_run_audit(move || {
            c.act(|s| s.run_telemetry_audit());
        });
        let c = ctx.clone();
        st.on_block_hosts(move || {
            c.act(|s| {
                if !s.telemetry_hosts_inflight && !s.is_rolling_back {
                    s.block_telemetry_hosts();
                }
            });
        });
        let c = ctx.clone();
        st.on_neutralize_services(move || {
            c.act(|s| {
                if !s.telemetry_services_inflight && !s.is_rolling_back {
                    s.neutralize_telemetry_services();
                }
            });
        });
        let c = ctx.clone();
        st.on_select_all(move || {
            c.act(|s| s.select_privacy_tweaks());
        });
        let c = ctx.clone();
        st.on_deselect_all(move || {
            c.act(|s| s.deselect_all());
        });
        let c = ctx.clone();
        st.on_apply_selected(move || {
            c.act(|s| {
                if !s.is_applying && !s.is_rolling_back {
                    s.apply_selected();
                }
            });
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<PrivacyStore>();
        let privacy: Vec<_> = s.scan_results.iter().filter(|r| is_privacy_or_telemetry_category(&r.category)).collect();
        let count = |state: TweakState| privacy.iter().filter(|r| r.current_state == state).count() as i32;
        st.set_optimized(count(TweakState::AlreadyOptimized));
        st.set_not_optimized(count(TweakState::NotOptimized));
        st.set_partial(count(TweakState::PartiallyOptimized));
        match &s.telemetry_audit {
            Some(a) => {
                st.set_audit_available(true);
                st.set_hosts_status(
                    format!("{} {} ✓ / {} ⚠", s.tr(TextKey::HostsLabel), a.blocked_hosts.len(), a.unblocked_hosts.len()).into(),
                );
                st.set_services_status(
                    format!(
                        "{}: {} {} / {} {}",
                        s.tr(TextKey::SubTabServices),
                        a.active_services.len(),
                        s.tr(TextKey::AuditActiveWord),
                        a.disabled_services.len(),
                        s.tr(TextKey::StateDisabled)
                    )
                    .into(),
                );
            }
            None => st.set_audit_available(false),
        }
        st.set_hosts_inflight(s.telemetry_hosts_inflight);
        st.set_services_inflight(s.telemetry_services_inflight);
        st.set_busy(s.is_rolling_back);
        st.set_is_applying(s.is_applying);
        st.set_is_scanning(s.is_scanning);
        st.set_selected_count(s.selected_tweaks.len() as i32);
        sync_vec(
            &self.rows,
            privacy
                .iter()
                .map(|sr| {
                    let (state_text, tone) = state_meta(s, &sr.current_state);
                    MatrixRow {
                        id: sr.tweak_id.as_str().into(),
                        title: s.tweak_title(&sr.tweak_id, &sr.title).into(),
                        category_label: s.tweak_category(&sr.category).into(),
                        risk_level: risk_index(sr.risk.stability_risk),
                        risk_label: risk_label(s, sr.risk.stability_risk).into(),
                        state_text: state_text.into(),
                        state_tone: tone.into(),
                        detail: s.scan_detail_text(sr).into(),
                        active: s.is_switch_active(&sr.tweak_id),
                        in_flight: s.in_flight_tweaks.contains(&sr.tweak_id),
                        not_applicable: sr.current_state == TweakState::NotApplicable,
                        ..Default::default()
                    }
                })
                .collect(),
        );
    }
}
