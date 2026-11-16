//! Responsive Task Manager style sparkline waveform widget with telemetry markers and gradient fill.
//!
//! Uses adaptive bar flex width so waveforms fill both narrow and ultrawide cards seamlessly.

use rune::color::Color;
use rune::{col, el, row, text, El};

use crate::ui::theme::ThemePalette;

/// Render a live Task Manager style mini waveform chart using the dynamic theme palette.
pub fn sparkline_with_palette(
    title: &str,
    current_value_label: &str,
    history: &[f32],
    chart_color: Color,
    chart_height: f32,
    max_bars: usize,
    p: &ThemePalette,
) -> El {
    let mut bars = Vec::with_capacity(max_bars);
    let pad_count = max_bars.saturating_sub(history.len());

    for _ in 0..pad_count {
        bars.push(
            el()
                .grow(1.0)
                .min_w(2.0)
                .max_w(12.0)
                .h(2.0)
                .radius(1.0)
                .fill(p.sparkline_bg),
        );
    }

    // Compute min and max telemetry markers across valid history values
    let (min_val, max_val) = history
        .iter()
        .filter(|v| v.is_finite())
        .fold((f32::MAX, f32::MIN), |(mn, mx), &v| (mn.min(v), mx.max(v)));
    let has_range = min_val.is_finite() && max_val.is_finite() && !history.is_empty();

    let start_idx = history.len().saturating_sub(max_bars);
    for &val in &history[start_idx..] {
        let clean_val = if val.is_finite() { val } else { 0.0 };
        let pct = (clean_val / 100.0).clamp(0.02, 1.0);
        let bar_h = (pct * chart_height).max(2.0);
        let is_peak = has_range && (clean_val - max_val).abs() < 0.05 && max_val > 5.0;

        // Gradient simulation: dual layer with semi-transparent base and saturated tip
        let fill_col = if is_peak {
            p.warning
        } else {
            chart_color
        };

        bars.push(
            col()
                .grow(1.0)
                .min_w(2.0)
                .max_w(12.0)
                .h(bar_h)
                .radius(1.0)
                .fill(fill_col.alpha(0.65))
                .hover(fill_col)
                .motion(p.motion)
                .child(
                    el()
                        .w_full()
                        .h((bar_h * 0.35).max(2.0))
                        .radius(1.0)
                        .fill(fill_col),
                ),
        );
    }

    let min_max_row = if has_range {
        row()
            .min_w(0.0)
            .shrink()
            .gap(6.0)
            .items_center()
            .child(
                text(format!("▼ {:.0}%", min_val.max(0.0)))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink(),
            )
            .child(
                text(format!("▲ {:.0}%", max_val.max(0.0)))
                    .font(rune::tok::type_::LABEL)
                    .color(if max_val > 80.0 { p.warning } else { p.text_secondary })
                    .shrink(),
            )
    } else {
        row()
    };

    col()
        .w_full()
        .min_w(0.0)
        .gap(6.0)
        .padding(p.card_padding.min(10.0))
        .radius(p.card_radius.min(6.0))
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .hover(p.bg_card_hover)
        .motion(p.motion)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap(6.0)
                .child(
                    row()
                        .grow(1.0)
                        .min_w(120.0)
                        .items_center()
                        .gap(6.0)
                        .child(
                            text(title.to_string())
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .truncate(),
                        )
                        .child(min_max_row),
                )
                .child(
                    text(current_value_label.to_string())
                        .font(rune::tok::type_::TITLE)
                        .color(chart_color)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .h(chart_height)
                .items_end()
                .gap(2.0)
                .children(bars),
        )
}
