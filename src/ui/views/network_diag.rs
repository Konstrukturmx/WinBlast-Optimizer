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

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let net = s.network_diag.clone().unwrap_or_default();

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .gap(12.0)
        // Header
        .child(
            row()
                .w_full()
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
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
                        .gap(8.0)
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagPingBtn), p.accent, Color::WHITE, p, |s| {
                                s.network_diag = Some(crate::tools::network_diag::NetworkDiagnostics::probe_live());
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::NetDiagFlushDnsBtn), p.btn_secondary, p.text_primary, p, |_| {
                                let _ = crate::tools::network_diag::NetworkDiagnostics::flush_dns();
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
                        .gap(4.0)
                        .child(text(s.tr_v(ViewTextKey::NetDiagTcpAckLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(if net.tcp_ack_frequency_optimized { "✔ (Low Latency)" } else { "Default" })
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
                .gap(8.0)
                .child(
                    text(s.tr_v(ViewTextKey::NetDiagDnsProbesTitle))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .children(net.probes.iter().map(|probe| {
                    let ping_text = probe.ping_ms.map(|ms| format!("{} ms", ms)).unwrap_or_else(|| "Timeout".to_string());
                    let ping_color = match probe.ping_ms {
                        Some(ms) if ms < 20 => p.success,
                        Some(ms) if ms < 50 => p.warning,
                        _ => p.danger,
                    };

                    row()
                        .w_full()
                        .padding(10.0)
                        .radius(style.card_radius().min(6.0))
                        .fill(p.bg_card_hover)
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(200.0)
                                .child(text(&probe.name).font(rune::tok::type_::LABEL).color(p.text_primary))
                                .child(text(&probe.host).font(rune::tok::type_::BODY).color(p.text_muted)),
                        )
                        .child(
                            col()
                                .min_w(130.0)
                                .child(text(ping_text).font(rune::tok::type_::TITLE).color(ping_color))
                                .child(text(&probe.status).font(rune::tok::type_::BODY).color(ping_color)),
                        )
                })),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
