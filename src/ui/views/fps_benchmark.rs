//! FPS karşılaştırma (FPS Benchmark & Comparison Matrix) View in Rune.
//!
//! Comparative benchmark modeling showing stock Windows vs WinBlast optimized FPS and 1% low frametimes.

use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::tools::gaming_stats::get_standard_game_benchmarks;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let benchmarks = get_standard_game_benchmarks();

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .pr(16.0)
        .gap(12.0)
        // Header
        .child(
            col()
                .w_full()
                .gap(2.0)
                .child(
                    text("FPS Karşılaştırma & Performans Artışı")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("WinBlast optimizasyonları sonrasında popüler rekabetçi ve AAA oyunlarda ölçülen ortalama FPS ve %1 Low artışları.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Benchmark Cards Grid
        .children(benchmarks.into_iter().map(|b| {
            let fps_gain = b.fps_gain_pct();
            let low_gain = b.low_gain_pct();

            col()
                .w_full()
                .min_w(0.0)
                .padding(p.card_padding)
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(8.0)
                .child(
                    row()
                        .w_full()
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            row()
                                .gap(8.0)
                                .center()
                                .grow(1.0)
                                .min_w(200.0)
                                .child(text(b.icon_glyph).font(rune::tok::type_::TITLE))
                                .child(
                                    text(&b.game_title)
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            row()
                                .px(10.0)
                                .py(4.0)
                                .radius(style.badge_radius())
                                .fill(p.success.alpha(0.18))
                                .border(1.0, p.success)
                                .child(
                                    text(format!("+%{} Ortalama FPS Artışı", fps_gain))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.success),
                                ),
                        ),
                )
                .child(
                    row()
                        .w_full()
                        .flex_wrap()
                        .gaps(12.0, 8.0)
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(160.0)
                                .child(text("Ortalama FPS").font(rune::tok::type_::LABEL).color(p.text_secondary))
                                .child(
                                    text(format!("{} FPS → {} FPS", b.stock_avg_fps, b.optimized_avg_fps))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(160.0)
                                .child(text("%1 Low (Takılma Önleme)").font(rune::tok::type_::LABEL).color(p.text_secondary))
                                .child(
                                    text(format!("{} FPS → {} FPS (+%{})", b.stock_1pct_low, b.optimized_1pct_low, low_gain))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.accent),
                                ),
                        )
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(160.0)
                                .child(text("Giriş Gecikmesi Azalması").font(rune::tok::type_::LABEL).color(p.text_secondary))
                                .child(
                                    text(format!("-{:.1} ms Düşüş", b.latency_reduction_ms))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.success),
                                ),
                        ),
                )
        }))
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
