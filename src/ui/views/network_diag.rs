//! İnternet ve ağ (Network & Latency Diagnostics) View in Rune.
//!
//! Live DNS round-trip ping prober, MTU diagnostics, and gaming TCP stack tuning.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

fn localize_dns_category<'a>(s: &'a AppState, cat: &'a str) -> &'a str {
    match cat {
        "Ultra-Low Latency & Privacy" => s.tr_v(ViewTextKey::NetDiagCatUltraLowLatency),
        "Global Anycast & Reliability" => s.tr_v(ViewTextKey::NetDiagCatAnycast),
        "Malware Threat Shield & Zero Logs" => s.tr_v(ViewTextKey::NetDiagCatMalwareThreat),
        "Ad & Tracker Blocking" => s.tr_v(ViewTextKey::NetDiagCatAdTracker),
        "Enterprise Security & Phishing Filter" => {
            s.tr_v(ViewTextKey::NetDiagCatEnterprisePhishing)
        }
        "High-Performance Anycast" => s.tr_v(ViewTextKey::NetDiagCatHighPerfAnycast),
        "Tier-1 Backbone Routing" => s.tr_v(ViewTextKey::NetDiagCatTier1Routing),
        "Real-Time Cyber Threat Filter" => s.tr_v(ViewTextKey::NetDiagCatCyberThreat),
        "Security & Malware Filtering" => s.tr_v(ViewTextKey::NetDiagCatSecurityMalware),
        "Zero-Log Strict Privacy" => s.tr_v(ViewTextKey::NetDiagCatZeroLogPrivacy),
        _ => cat,
    }
}

fn localize_dns_status<'a>(s: &'a AppState, status: &'a str) -> &'a str {
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

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let net = s.network_diag.clone().unwrap_or_default();

    col()
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(style.card_padding())
        .pr(16.0)
        .gap(12.0)
        // Header
        .child(
            row()
                .w_full()
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .justify_between()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text(s.tr_v(ViewTextKey::NetDiagTitle))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text(s.tr_v(ViewTextKey::NetDiagSubtitle))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(8.0, 8.0)
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagPingBtn), p.accent, Color::WHITE, p, |s| {
                                s.network_diag = Some(crate::tools::network_diag::NetworkDiagnostics::probe_live());
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagResetDnsBtn), p.btn_secondary, p.text_primary, p, |s| {
                                let _ = crate::tools::network_diag::NetworkDiagnostics::reset_dns_dhcp();
                                s.set_status("DNS configuration restored to Automatic (DHCP).");
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagFlushDnsBtn), p.btn_secondary, p.text_primary, p, |s| {
                                let _ = crate::tools::network_diag::NetworkDiagnostics::flush_dns();
                                s.set_status("Flushed Windows DNS Resolver Cache.");
                            })
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        // Adapter & TCP Status Card
        .child(
            row()
                .w_full()
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .hover(p.bg_card_hover)
                        .motion(p.motion)
                        .gap(4.0)
                        .child(text(s.tr_v(ViewTextKey::NetDiagAdapterLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(text(&net.adapter_name).font(rune::tok::type_::TITLE).color(p.accent))
                        .child(
                            text(format!("IP: {} • {}: {} B", net.ip_address, s.tr_v(ViewTextKey::NetDiagMtuLabel), net.mtu_size))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .hover(p.bg_card_hover)
                        .motion(p.motion)
                        .gap(4.0)
                        .child(text(s.tr_v(ViewTextKey::NetDiagTcpAckLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(if net.tcp_ack_frequency_optimized {
                                format!("✔ ({})", s.tr_v(ViewTextKey::NetDiagLowLatencyBadge))
                            } else {
                                s.tr_v(ViewTextKey::NetDiagDefaultLatencyBadge).to_string()
                            })
                            .font(rune::tok::type_::TITLE)
                            .color(p.success),
                        )
                        .child(
                            text(s.tr_v(ViewTextKey::NetDiagTcpNoDelayLabel))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                ),
        )
        // Live DNS Ping Probes Grid
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(10.0)
                .child(
                    row()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .child(
                            row()
                                .gap(6.0)
                                .items_center()
                                .child(text("🌐").font(rune::tok::type_::TITLE))
                                .child(
                                    text(s.tr_v(ViewTextKey::NetDiagDnsBenchmarkTitle))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child({
                            let tracked_tmpl = s.tr_v(ViewTextKey::NetDiagDnsTrackedLabel);
                            text(tracked_tmpl.replace("{}", &net.probes.len().to_string()))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary)
                        }),
                )
                .children(net.probes.iter().map(|probe| {
                    let ping_text = probe.ping_ms.map(|ms| format!("{} ms", ms)).unwrap_or_else(|| "Timeout".to_string());
                    let ping_color = match probe.ping_ms {
                        Some(ms) if ms < 20 => p.success,
                        Some(ms) if ms < 50 => p.warning,
                        _ => p.danger,
                    };

                    let p_ip = probe.primary_ip.clone();
                    let s_ip = probe.secondary_ip.clone();
                    let prov_name = probe.name.clone();

                    let cat_label = localize_dns_category(s, &probe.category);
                    let status_label = localize_dns_status(s, &probe.status);

                    row()
                        .w_full()
                        .padding(10.0)
                        .radius(style.card_radius().min(6.0))
                        .fill(p.bg_card_hover)
                        .border(1.0, p.border)
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(220.0)
                                .gap(2.0)
                                .child(
                                    row()
                                        .gap(8.0)
                                        .items_center()
                                        .child(
                                            text(&probe.name)
                                                .font(rune::tok::type_::TITLE)
                                                .color(p.text_primary),
                                        )
                                        .child(
                                            row()
                                                .px(6.0)
                                                .py(1.0)
                                                .radius(style.badge_radius())
                                                .fill(p.accent.alpha(0.12))
                                                .border(1.0, p.accent.alpha(0.35))
                                                .child(
                                                    text(cat_label)
                                                        .font(rune::tok::type_::LABEL)
                                                        .color(p.accent),
                                                ),
                                        ),
                                )
                                .child(
                                    text(format!("Primary: {} • Secondary: {}", probe.primary_ip, probe.secondary_ip))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            row()
                                .min_w(130.0)
                                .gap(8.0)
                                .items_center()
                                .child(
                                    col()
                                        .child(text(ping_text).font(rune::tok::type_::TITLE).color(ping_color))
                                        .child(text(status_label).font(rune::tok::type_::BODY).color(ping_color)),
                                ),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagSetDnsBtn), p.btn_secondary, p.text_primary, p, move |s| {
                                let _ = crate::tools::network_diag::NetworkDiagnostics::apply_dns_servers(&p_ip, &s_ip);
                                s.set_status(format!("Configured {} ({}) as active DNS.", prov_name, p_ip));
                            })
                            .radius(style.btn_radius()),
                        )
                })),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
