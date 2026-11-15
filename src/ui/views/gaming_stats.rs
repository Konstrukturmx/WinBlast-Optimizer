//! Oyun içi izleme (In-Game Monitoring & Telemetry) View in Rune.
//!
//! Real-time frame pacing, microstutter suppression, and background CPU interference monitoring.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let telemetry = &s.game_telemetry;

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
                            text("Oyun İçi İzleme & Kare Zamanlaması")
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text("Real-time frametime consistency, microstutter detection, and background process thread affinity.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    action_button("Yenile", p.accent, Color::WHITE, p, |s| {
                        s.game_telemetry = crate::tools::gaming_stats::LiveGameTelemetry::probe();
                    })
                    .radius(style.btn_radius()),
                ),
        )
        // Telemetry Metrics Grid
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
                        .child(text("Kare Kararlılığı (Frame Pacing)").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("%{}", telemetry.frame_pacing_rating))
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        )
                        .child(
                            text("Mükemmel: Dalgalanma ve drop yaşanmıyor.")
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
                        .child(text("Arka Plan CPU Etkisi").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("%{:.1}", telemetry.background_cpu_impact_pct))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text("Düşük gecikmeli oyun önceliği devrede.")
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
                        .child(text("Mikro-Takılma Oranı").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{}/dakika", telemetry.micro_stutters_per_min))
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        )
                        .child(
                            text(&telemetry.directx_version)
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                ),
        )
        // Active Game Process Optimizer Card
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(6.0)
                .child(
                    text("Aktif Oyun Süreç Önceliği (Process Priority Booster)")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .child(
                    text("Bir oyun çalıştırıldığında WinBlast otomatik olarak oyun sürecine 'High' CPU önceliği atar, Windows Game Mode'u kilitler ve arka plan Windows Update indirmelerini askıya alır.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
