//! Backup Center binder: restore points, profile export/import, Autounattend.xml, ledger undo.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::tools::autounattend::TargetArch;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{BackupStore, LedgerRow, MainWindow, RestorePointRow};

pub struct BackupView {
    points: Rc<VecModel<RestorePointRow>>,
    ledger: Rc<VecModel<LedgerRow>>,
}

impl BackupView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<BackupStore>();
        let (points, a) = new_model();
        let (ledger, b) = new_model();
        st.set_restore_points(a);
        st.set_ledger(b);
        Self { points, ledger }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<BackupStore>();
        let c = ctx.clone();
        st.on_create_restore_point(move || {
            c.act(|s| {
                if !s.restore_point_inflight {
                    s.create_restore_point();
                }
            });
        });
        let c = ctx.clone();
        st.on_refresh_restore_points(move || {
            c.act(|s| s.refresh_restore_points());
        });
        let c = ctx.clone();
        st.on_export_profile(move || {
            c.act(|s| s.export_profile());
        });
        let c = ctx.clone();
        st.on_import_profile(move || {
            c.act(|s| s.import_profile());
        });
        let c = ctx.clone();
        st.on_rollback(move |id| {
            let Ok(id) = id.parse::<i64>() else { return };
            c.act(|s| {
                if !s.is_rolling_back && !s.is_applying && s.in_flight_tweaks.is_empty() {
                    s.rollback_entry(id);
                }
            });
        });
        let c = ctx.clone();
        st.on_unattend_preset(move |p| {
            if matches!(p.as_str(), "minimal" | "privacy" | "gaming") {
                c.act(|s| s.apply_unattend_preset(p.as_str()));
            }
        });
        let c = ctx.clone();
        st.on_toggle_arch(move || {
            c.act(|s| {
                s.unattend_opts.architecture = match s.unattend_opts.architecture {
                    TargetArch::Amd64 => TargetArch::Arm64,
                    TargetArch::Arm64 => TargetArch::Amd64,
                }
            });
        });
        let c = ctx.clone();
        st.on_toggle_option(move |o| {
            c.act(|s| {
                let u = &mut s.unattend_opts;
                match o.as_str() {
                    "tpm" => u.bypass_tpm_ram_requirements = !u.bypass_tpm_ram_requirements,
                    "msa" => u.bypass_msa_oobe = !u.bypass_msa_oobe,
                    "privacy" => u.skip_oobe_privacy_screens = !u.skip_oobe_privacy_screens,
                    "tweaks" => u.include_selected_tweaks = !u.include_selected_tweaks,
                    "timezone" => u.auto_timezone_utc = !u.auto_timezone_utc,
                    _ => {}
                }
            });
        });
        let c = ctx.clone();
        st.on_export_unattend(move || {
            c.act(|s| s.export_autounattend_xml());
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<BackupStore>();
        st.set_restore_inflight(s.restore_point_inflight);
        sync_vec(
            &self.points,
            s.restore_points
                .iter()
                .map(|rp| RestorePointRow {
                    seq: format!("#{}", rp.sequence_number).into(),
                    description: rp.description.as_str().into(),
                    created: rp.creation_time.to_string().into(),
                })
                .collect(),
        );
        sync_vec(
            &self.ledger,
            s.ledger_history
                .iter()
                .map(|e| LedgerRow {
                    id: e.id.to_string().into(),
                    title: s.tweak_title(&e.tweak_id, &e.tweak_id).into(),
                    action: crate::core::status_i18n::localize_ledger_action(s.language, &e.action_type).into(),
                    timestamp: e.timestamp.format("%Y-%m-%d %H:%M:%S").to_string().into(),
                    change: format!(
                        "{} → {}",
                        crate::core::tweak_i18n::localize_ledger_token(s.language, &e.original_value),
                        crate::core::tweak_i18n::localize_ledger_token(s.language, &e.new_value)
                    )
                    .into(),
                    rolled_back: e.rolled_back,
                    reversible: e.is_reversible(),
                })
                .collect(),
        );
        st.set_active_changes(s.ledger_active_count as i32);
        st.set_can_undo(!s.is_rolling_back && !s.is_applying && s.in_flight_tweaks.is_empty());
        st.set_rolling_back(s.is_rolling_back);
        st.set_ledger_path(s.ledger_path.display().to_string().into());
        let u = &s.unattend_opts;
        st.set_arch(u.architecture.as_str().into());
        st.set_bypass_tpm(u.bypass_tpm_ram_requirements);
        st.set_bypass_msa(u.bypass_msa_oobe);
        st.set_skip_privacy(u.skip_oobe_privacy_screens);
        st.set_include_tweaks(u.include_selected_tweaks);
        st.set_auto_timezone(u.auto_timezone_utc);
        st.set_last_export(s.unattend_last_export.clone().map(SharedString::from).unwrap_or_default());
    }
}
