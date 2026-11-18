//! Performans testleri (Performance Benchmark Suite) View in Rune.
//!
//! Multi-core CPU compute, memory bandwidth, and storage throughput benchmarking engine.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::ui::widgets::button::action_button_guarded;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let score = s.perf_score.clone().unwrap_or_default();

    col()
        .w_full()
        .min_w(0.0)
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
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text(s.tr_v(ViewTextKey::PerfTitle))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text(s.tr_v(ViewTextKey::PerfSubtitle))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    action_button_guarded(
                        if s.is_benchmarking_perf {
                            s.tr_v(ViewTextKey::PerfBenchmarkingStatus)
                        } else {
                            s.tr_v(ViewTextKey::PerfStartBtn)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.is_benchmarking_perf,
                        p,
                        |s| s.start_perf_benchmark_async(),
                    )
                    .radius(style.btn_radius()),
                ),
        )
        // Composite Index Hero Card
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.accent.alpha(0.12))
                .border(1.5, p.accent)
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(6.0)
                .child(
                    text(s.tr_v(ViewTextKey::PerfCompositeScoreTitle))
                        .font(rune::tok::type_::LABEL)
                        .color(p.accent),
                )
                .child(
                    text(format!("{} {}", score.total_composite_score, s.tr_v(ViewTextKey::PerfPoints)))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::PerfHeroDesc))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                ),
        )
        // Benchmark Metrics Grid
        .child(
            row()
                .w_full()
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    // CPU Score Card
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
                        .child(text(s.tr_v(ViewTextKey::PerfCpuLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{} {}", score.cpu_multi_score, s.tr_v(ViewTextKey::PerfMultiCoreSuffix)))
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        )
                        .child(
                            text(format!("1-Core: {} {}", score.cpu_single_score, s.tr_v(ViewTextKey::PerfPoints)))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    // RAM Score Card
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
                        .child(text(s.tr_v(ViewTextKey::PerfRamLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{:.1} {}", score.ram_read_gb_s, s.tr_v(ViewTextKey::PerfThroughputSuffix)))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(format!("Latency: {:.1} ns", score.ram_latency_ns))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    // Disk Score Card
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
                        .child(text(s.tr_v(ViewTextKey::PerfDiskLabel)).font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{:.0} {}", score.disk_seq_read_mb_s, s.tr_v(ViewTextKey::PerfIoSuffix)))
                                .font(rune::tok::type_::TITLE)
                                .color(p.warning),
                        )
                        .child(
                            text(s.tr(crate::core::i18n::TextKey::StorageDrivesTitle))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
