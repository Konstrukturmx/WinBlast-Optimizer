//! Gaming hub (live monitor, FPS reference table, profiles, overlay, suitability),
//! Game Prerequisites and Network diagnostics binders.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::{AppState, GamingSubTab};
use crate::core::i18n::{I18n, TextKey};
use crate::core::view_i18n::ViewTextKey;
use crate::hardware::sys_inventory::DriveMediaKind;
use crate::tools::game_overlay::{GameSuitabilityReport, OverlaySetting};
use crate::tools::game_prereqs::PrereqInstallState;
use crate::tools::network_diag::NetAction;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{CheckRow, DnsRow, GamingStore, MainWindow, NetworkStore, PrereqRow, PrereqStore, ProfileRow};

pub const SUB_TABS: [GamingSubTab; 5] = [
    GamingSubTab::LiveMonitor,
    GamingSubTab::FpsBenchmark,
    GamingSubTab::GameProfiles,
    GamingSubTab::GameOverlay,
    GamingSubTab::GameSuitability,
];

fn dns_category<'a>(s: &'a AppState, cat: &'a str) -> &'a str {
    match cat {
        "Ultra-Low Latency & Privacy" => s.tr_v(ViewTextKey::NetDiagCatUltraLowLatency),
        "Global Anycast & Reliability" => s.tr_v(ViewTextKey::NetDiagCatAnycast),
        "Malware Threat Shield & Zero Logs" => s.tr_v(ViewTextKey::NetDiagCatMalwareThreat),
        "Ad & Tracker Blocking" => s.tr_v(ViewTextKey::NetDiagCatAdTracker),
        "Enterprise Security & Phishing Filter" => s.tr_v(ViewTextKey::NetDiagCatEnterprisePhishing),
        "High-Performance Anycast" => s.tr_v(ViewTextKey::NetDiagCatHighPerfAnycast),
        "Tier-1 Backbone Routing" => s.tr_v(ViewTextKey::NetDiagCatTier1Routing),
        "Real-Time Cyber Threat Filter" => s.tr_v(ViewTextKey::NetDiagCatCyberThreat),
        "Security & Malware Filtering" => s.tr_v(ViewTextKey::NetDiagCatSecurityMalware),
        "Zero-Log Strict Privacy" => s.tr_v(ViewTextKey::NetDiagCatZeroLogPrivacy),
        _ => cat,
    }
}

fn dns_status<'a>(s: &'a AppState, status: &'a str) -> &'a str {
    if status.starts_with("Fast") {
        s.tr_v(ViewTextKey::NetDiagFastStatus)
    } else if status.starts_with("Good") {
        s.tr_v(ViewTextKey::NetDiagGoodStatus)
    } else if status.starts_with("Moderate") {
        s.tr_v(ViewTextKey::NetDiagModerateStatus)
    } else if status.starts_with("Timeout") {
        s.tr_v(ViewTextKey::NetDiagTimeoutStatus)
    } else {
        status
    }
}

/// Suitability is only evaluated once real CPU, RAM and GPU data exist (v1.6.1 silently
/// assumed 4 cores / 8 GB / "GPU" and scored the machine against those).
pub fn suitability(s: &AppState) -> Option<GameSuitabilityReport> {
    let cores = s.cpu_info.as_ref()?.physical_cores as u32;
    let ram = s.realtime_metrics.as_ref()?.total_ram_mb;
    let gpu = s.gpus.first()?.name.clone();
    let nvme = s
        .platform_security
        .as_ref()
        .is_some_and(|ps| ps.volumes.iter().any(|v| v.media_kind == DriveMediaKind::Nvme));
    Some(GameSuitabilityReport::evaluate(cores, ram, &gpu, s.hags_enabled, nvme))
}

pub struct GamingView {
    profiles: Rc<VecModel<ProfileRow>>,
    checks: Rc<VecModel<CheckRow>>,
    prereqs: Rc<VecModel<PrereqRow>>,
    dns: Rc<VecModel<DnsRow>>,
}

impl GamingView {
    pub fn new(window: &MainWindow) -> Self {
        let g = window.global::<GamingStore>();
        let (profiles, b) = new_model();
        let (checks, c) = new_model();
        g.set_profiles(b);
        g.set_checks(c);
        let (prereqs, d) = new_model();
        window.global::<PrereqStore>().set_items(d);
        let (dns, e) = new_model();
        window.global::<NetworkStore>().set_probes(e);
        Self { profiles, checks, prereqs, dns }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let g = window.global::<GamingStore>();
        let c = ctx.clone();
        g.on_set_sub_tab(move |i| {
            if let Some(tab) = usize::try_from(i).ok().and_then(|i| SUB_TABS.get(i).copied()) {
                c.act(|s| {
                    s.gaming_sub_tab = tab;
                    match tab {
                        GamingSubTab::GameProfiles => s.refresh_game_profiles(),
                        GamingSubTab::GameOverlay => s.refresh_game_overlay(),
                        _ => {}
                    }
                });
            }
        });
        let c = ctx.clone();
        g.on_refresh(move || {
            c.act(|s| s.probe_game_telemetry_async());
        });
        let c = ctx.clone();
        g.on_toggle_profile(move |id| {
            c.act(|s| s.toggle_game_profile(id.as_str()));
        });
        let c = ctx.clone();
        g.on_set_overlay(move |which, value| {
            let setting = match which.as_str() {
                "gamebar" => OverlaySetting::GameBarDisabled,
                "dvr" => OverlaySetting::DvrDisabled,
                "gamemode" => OverlaySetting::GameModeEnabled,
                _ => return,
            };
            c.act(|s| s.set_game_overlay_async(setting, value));
        });

        let p = window.global::<PrereqStore>();
        let c = ctx.clone();
        p.on_scan(move || {
            c.act(|s| s.scan_game_prereqs());
        });
        let c = ctx.clone();
        p.on_install(move |id| {
            c.act(|s| s.install_game_prereq(id.to_string()));
        });
        let c = ctx.clone();
        p.on_install_all(move || {
            c.act(|s| s.install_all_missing_prereqs());
        });
        let c = ctx.clone();
        p.on_filter_edited(move |t| {
            c.act(|s| s.prereq_filter_edit.text = t.to_string());
        });

        let n = window.global::<NetworkStore>();
        let c = ctx.clone();
        n.on_ping(move || {
            c.act(|s| s.probe_network_async());
        });
        let c = ctx.clone();
        n.on_reset_dns(move || {
            c.act(|s| s.network_action_async(NetAction::ResetDnsDhcp));
        });
        let c = ctx.clone();
        n.on_flush_dns(move || {
            c.act(|s| s.network_action_async(NetAction::FlushDns));
        });
        let c = ctx.clone();
        n.on_apply_dns(move |primary, secondary, _name| {
            let (primary, secondary) = (primary.trim().to_string(), secondary.trim().to_string());
            if primary.parse::<std::net::IpAddr>().is_err() || (!secondary.is_empty() && secondary.parse::<std::net::IpAddr>().is_err()) {
                return; // only well-formed addresses ever reach netsh
            }
            c.act(|s| s.network_action_async(NetAction::ApplyDns { primary, secondary }));
        });
    }

    pub fn sync_gaming(&self, window: &MainWindow, s: &AppState) {
        let g = window.global::<GamingStore>();
        g.set_sub_tab(SUB_TABS.iter().position(|t| *t == s.gaming_sub_tab).unwrap_or(0) as i32);
        g.set_probing(s.game_probe_inflight);
        g.set_detected_game(s.game_telemetry.active_game_detected.clone().map(SharedString::from).unwrap_or_else(|| s.tr(TextKey::GamingNoGameDetected).into()));
        g.set_hags_on(s.hags_enabled);
        g.set_hags_text(if s.hags_enabled { s.tr(TextKey::HagsActiveStatus).into() } else { s.tr(TextKey::HagsDisabledStatus).into() });
        g.set_profile_busy(s.game_profile_inflight.is_some());
        sync_vec(
            &self.profiles,
            s.game_profiles
                .iter()
                .map(|p| ProfileRow {
                    id: p.id.into(),
                    icon: p.icon_glyph.into(),
                    title: p.title.into(),
                    target: format!("{}: {}", s.tr_v(ViewTextKey::GameProfilesTarget), p.executable).into(),
                    desc: p.description.into(),
                    applied: p.is_applied,
                    busy: s.game_profile_inflight.as_deref() == Some(p.id),
                    high_priority: p.high_priority,
                    disable_fso: p.disable_fso,
                    unpark: p.unpark_cores,
                })
                .collect(),
        );
        g.set_overlay_busy(s.overlay_inflight);
        g.set_game_bar_disabled(s.game_overlay.game_bar_disabled);
        g.set_dvr_disabled(s.game_overlay.game_dvr_background_recording_disabled);
        g.set_game_mode(s.game_overlay.game_mode_enabled);
        g.set_fse(s.game_overlay.fullscreen_exclusive_optimized);
        match suitability(s) {
            Some(r) => {
                g.set_suitability_ready(true);
                g.set_tier_label(r.tier_label.into());
                g.set_score(r.overall_score_pct as i32);
                sync_vec(
                    &self.checks,
                    r.checks
                        .iter()
                        .map(|c| CheckRow { name: c.feature_name.into(), ok: c.is_supported, desc: c.description.as_str().into(), recommendation: c.recommendation.into() })
                        .collect(),
                );
            }
            None => {
                g.set_suitability_ready(false);
                sync_vec(&self.checks, Vec::new());
            }
        }
    }

    pub fn sync_prereqs(&self, window: &MainWindow, s: &AppState) {
        let p = window.global::<PrereqStore>();
        let total = s.game_prereqs.len();
        let installed = s.game_prereqs.iter().filter(|i| i.installed).count();
        p.set_scanning(s.is_prereqs_scanning);
        p.set_busy(s.prereq_install_in_flight.is_some());
        p.set_total(total as i32);
        p.set_installed(installed as i32);
        p.set_missing(total.saturating_sub(installed) as i32);
        if p.get_filter().as_str() != s.prereq_filter_edit.text {
            p.set_filter(s.prereq_filter_edit.text.as_str().into());
        }
        let q = s.prereq_filter_edit.text.trim();
        let q = if q.is_empty() { String::new() } else { I18n::fold_search_str(q) };
        sync_vec(
            &self.prereqs,
            s.game_prereqs
                .iter()
                .filter(|i| q.is_empty() || [i.name.as_str(), i.category.as_str(), i.description.as_str(), i.id.as_str()].iter().any(|f| I18n::fold_search_str(f).contains(&q)))
                .map(|i| {
                    let (state, tone) = match i.state {
                        PrereqInstallState::Installed => (s.tr(TextKey::PrereqStateInstalled), "success"),
                        PrereqInstallState::NotInstalled => (s.tr(TextKey::PrereqStateMissing), "warning"),
                        PrereqInstallState::Installing => (s.tr(TextKey::PrereqStateInstalling), "accent"),
                        PrereqInstallState::Failed => (s.tr(TextKey::PrereqStateFailed), "danger"),
                    };
                    PrereqRow {
                        id: i.id.as_str().into(),
                        name: i.name.as_str().into(),
                        sub: format!("{} • {}", i.category, i.description).into(),
                        version: format!("{}: {}", s.tr(TextKey::PrereqVersionLabel), i.installed_version.as_deref().unwrap_or(s.tr(TextKey::PrereqNotDetected))).into(),
                        status: i.status_message.as_str().into(),
                        state: state.into(),
                        tone: tone.into(),
                        installed: i.installed,
                        busy: s.prereq_install_in_flight.as_deref() == Some(i.id.as_str()),
                    }
                })
                .collect(),
        );
    }

    pub fn sync_network(&self, window: &MainWindow, s: &AppState) {
        let n = window.global::<NetworkStore>();
        n.set_probing(s.network_probe_inflight);
        n.set_busy(s.network_action_inflight);
        let net = s.network_diag.clone().unwrap_or_default();
        n.set_loaded(!net.adapter_name.is_empty());
        n.set_adapter(if net.adapter_name.is_empty() { "—".into() } else { net.adapter_name.as_str().into() });
        n.set_ip_mtu(
            format!(
                "IP: {} • {}: {}",
                if net.ip_address.is_empty() { "—" } else { net.ip_address.as_str() },
                s.tr_v(ViewTextKey::NetDiagMtuLabel),
                if net.mtu_size > 0 { format!("{} B", net.mtu_size) } else { "—".into() }
            )
            .into(),
        );
        n.set_ack_optimized(net.tcp_ack_frequency_optimized);
        n.set_nodelay_optimized(net.tcp_no_delay_optimized);
        let tracked = s.tr_v(ViewTextKey::NetDiagDnsTrackedLabel);
        let count = net.probes.len().to_string();
        // Some catalogs carry a "{}" placeholder, others are a bare label: always show the count.
        n.set_tracked(if tracked.contains("{}") { tracked.replace("{}", &count) } else { format!("{count} {tracked}") }.into());
        sync_vec(
            &self.dns,
            net.probes
                .iter()
                .map(|p| DnsRow {
                    name: p.name.as_str().into(),
                    category: dns_category(s, &p.category).into(),
                    ips: format!("{}: {} • {}: {}", s.tr(TextKey::NetPrimaryLabel), p.primary_ip, s.tr(TextKey::NetSecondaryLabel), p.secondary_ip).into(),
                    primary: p.primary_ip.as_str().into(),
                    secondary: p.secondary_ip.as_str().into(),
                    ping: p.ping_ms.map(|ms| format!("{ms} ms")).unwrap_or_else(|| if p.status.starts_with("Ready") { "—".into() } else { s.tr(TextKey::NetTimeout).into() }).into(),
                    status: if p.ping_ms.is_none() && p.status.starts_with("Ready") { s.tr(TextKey::NetNotProbed).into() } else { dns_status(s, &p.status).into() },
                    tone: match p.ping_ms {
                        Some(ms) if ms < 20 => "success",
                        Some(ms) if ms < 50 => "warning",
                        Some(_) => "danger",
                        None if p.status.starts_with("Ready") => "muted",
                        None => "danger",
                    }
                    .into(),
                })
                .collect(),
        );
    }
}
