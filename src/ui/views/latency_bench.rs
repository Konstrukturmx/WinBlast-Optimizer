//! System and Network Latency Benchmark & Before/After Comparative View in Rune.
//!
//! Pure `.flex_wrap()` KPI strips and target cards adapt smoothly across all resolutions.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::hardware::net_latency::LatencyBenchmarkSnapshot;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

/// Comparison delta card between Before and After snapshots.
fn before_after_comparison_card(
    before: &LatencyBenchmarkSnapshot,
    after: &LatencyBenchmarkSnapshot,
    s: &AppState,
) -> El {
    let p = s.palette();
    let _style = s.theme_style;

    let dpc_color = if after.dpc.peak_jitter_us <= before.dpc.peak_jitter_us {
        p.success
    } else {
        p.warning
    };
    let timer_color = if after.dpc.timer_resolution_ms <= before.dpc.timer_resolution_ms {
        p.success
    } else {
        p.warning
    };
    let is_offline = after.overall_avg_ping_ms < 0.0;
    let ping_color = if is_offline {
        p.danger
    } else if after.overall_avg_ping_ms <= before.overall_avg_ping_ms {
        p.success
    } else {
        p.warning
    };
    let jitter_color = if after.overall_avg_jitter_ms < 0.0 {
        p.danger
    } else if after.overall_avg_jitter_ms <= before.overall_avg_jitter_ms {
        p.success
    } else {
        p.warning
    };

    row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.accent.alpha(0.4))
        .flex_wrap()
        .gaps(10.0, 12.0)
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::DpcJitterPeak))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{} µs", after.dpc.peak_jitter_us))
                        .font(rune::tok::type_::TITLE)
                        .color(dpc_color),
                )
                .child(
                    text(format!(
                        "({} {} µs)",
                        s.tr(TextKey::WasLabel),
                        before.dpc.peak_jitter_us
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::TimerResolution))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{:.3} ms", after.dpc.timer_resolution_ms))
                        .font(rune::tok::type_::TITLE)
                        .color(timer_color),
                )
                .child(
                    text(format!(
                        "({} {:.3} ms)",
                        s.tr(TextKey::WasLabel),
                        before.dpc.timer_resolution_ms
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::AvgPingDns))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(if is_offline {
                        "Offline (100% Loss)".to_string()
                    } else {
                        format!("{:.1} ms", after.overall_avg_ping_ms)
                    })
                    .font(rune::tok::type_::TITLE)
                    .color(ping_color),
                )
                .child(
                    text(format!(
                        "({} {:.1} ms)",
                        s.tr(TextKey::WasLabel),
                        before.overall_avg_ping_ms
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::AvgPingJitter))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{:.2} ms", after.overall_avg_jitter_ms))
                        .font(rune::tok::type_::TITLE)
                        .color(jitter_color),
                )
                .child(
                    text(format!(
                        "({} {:.2} ms)",
                        s.tr(TextKey::WasLabel),
                        before.overall_avg_jitter_ms
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted),
                ),
        )
}

/// Renders the responsive System & Network Latency Benchmark view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let _style = s.theme_style;

    let header_card = col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .motion(p.motion)
        .gap(4.0)
        .child(
            text(s.tr(TextKey::LatencyTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        )
        .child(
            text(s.tr(TextKey::LatencySub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );

    let actions_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(12.0))
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .flex_wrap()
        .items_center()
        .gaps(8.0, 10.0)
        .child(action_button_guarded(
            if s.is_benchmarking_latency {
                s.tr(TextKey::RunningTest)
            } else {
                s.tr(TextKey::RunLatencyTest)
            },
            p.accent,
            Color::WHITE,
            !s.is_benchmarking_latency,
            p,
            |s| s.start_latency_benchmark(),
        ))
        .child(action_button_guarded(
            s.tr(TextKey::SaveBeforeBaselineBtn),
            p.btn_secondary,
            p.text_primary,
            !s.is_benchmarking_latency,
            p,
            |s| s.set_latency_baseline_before(),
        ))
        .child(action_button(
            s.tr(TextKey::LatencyClearBtn),
            p.btn_secondary,
            p.text_secondary,
            p,
            |s| s.reset_latency_benchmark(),
        ));

    let mut content_col = col().w_full().min_w(0.0).gap(12.0);

    if let (Some(before), Some(after)) = (&s.latency_before, &s.latency_after) {
        content_col = content_col.child(before_after_comparison_card(before, after, s));
    }

    if let Some(bench) = &s.latest_latency_bench {
        let kpi_strip = row()
            .w_full()
            .min_w(0.0)
            .flex_wrap()
            .gaps(10.0, 10.0)
            .child(
                col()
                    .grow(1.0)
                    .min_w(150.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .gap(2.0)
                    .child(
                        text(s.tr(TextKey::DpcJitterPeak))
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(format!("{} µs", bench.dpc.peak_jitter_us))
                            .font(rune::tok::type_::TITLE)
                            .color(p.accent),
                    ),
            )
            .child(
                col()
                    .grow(1.0)
                    .min_w(150.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .gap(2.0)
                    .child(
                        text(s.tr(TextKey::TimerResolution))
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(format!("{:.3} ms", bench.dpc.timer_resolution_ms))
                            .font(rune::tok::type_::TITLE)
                            .color(p.success),
                    ),
            )
            .child(
                col()
                    .grow(1.0)
                    .min_w(150.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .gap(2.0)
                    .child(
                        text(s.tr(TextKey::AvgPingDns))
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(format!("{:.1} ms", bench.overall_avg_ping_ms))
                            .font(rune::tok::type_::TITLE)
                            .color(p.text_primary),
                    ),
            )
            .child(
                col()
                    .grow(1.0)
                    .min_w(150.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .gap(2.0)
                    .child(
                        text(s.tr(TextKey::AvgPingJitter))
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(format!("{:.2} ms", bench.overall_avg_jitter_ms))
                            .font(rune::tok::type_::TITLE)
                            .color(p.accent),
                    ),
            );

        content_col = content_col.child(kpi_strip);

        let mut target_cards = col().w_full().min_w(0.0).gap(8.0);
        for target in &bench.targets {
            let item = row()
                .w_full()
                .min_w(0.0)
                .padding(12.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .flex_wrap()
                .items_center()
                .gaps(8.0, 14.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(160.0)
                        .child(
                            text(&target.name)
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(format!(
                                "{}: {:.0}%",
                                s.tr(TextKey::LossLabel),
                                target.packet_loss_pct
                            ))
                            .font(rune::tok::type_::LABEL)
                            .color(if target.packet_loss_pct == 0.0 {
                                p.success
                            } else {
                                p.danger
                            }),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(80.0)
                        .child(
                            text(s.tr(TextKey::MinLabel))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted),
                        )
                        .child(
                            text(format!("{:.1} ms", target.rtt_min_ms))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(80.0)
                        .child(
                            text(s.tr(TextKey::AvgLabel))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted),
                        )
                        .child(
                            text(format!("{:.1} ms", target.rtt_avg_ms))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(80.0)
                        .child(
                            text(s.tr(TextKey::MaxLabel))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted),
                        )
                        .child(
                            text(format!("{:.1} ms", target.rtt_max_ms))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(90.0)
                        .child(
                            text(s.tr(TextKey::JitterLabel))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted),
                        )
                        .child(
                            text(format!("{:.2} ms", target.jitter_ms))
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        ),
                );

            target_cards = target_cards.child(item);
        }

        content_col = content_col.child(target_cards);
    } else {
        content_col = content_col.child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(p.card_padding)
                .radius(p.card_radius)
                .fill(p.bg_card)
                .border(p.border_width, p.border)
                .center()
                .child(
                    text(if s.is_benchmarking_latency {
                        s.tr(TextKey::LatencyRunningText)
                    } else {
                        s.tr(TextKey::NoLatencyRunYet)
                    })
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
                ),
        );
    }

    let pad = if tier == LayoutTier::Compact {
        10.0
    } else {
        14.0
    };
    col()
        .key("latency_bench_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .pr(14.0)
        .gap(12.0)
        .scroll(ScrollSpec {
            stick_bottom: false,
        })
        .child(header_card)
        .child(actions_card)
        .child(content_col)
        .winblast_scroll(p)
}
