//! Responsive Task Manager style sparkline waveform widget.
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

    let start_idx = history.len().saturating_sub(max_bars);
    for &val in &history[start_idx..] {
        let clean_val = if val.is_finite() { val } else { 0.0 };
        let pct = (clean_val / 100.0).clamp(0.02, 1.0);
        let bar_h = (pct * chart_height).max(2.0);

        bars.push(
            el()
                .grow(1.0)
                .min_w(2.0)
                .max_w(12.0)
                .h(bar_h)
                .radius(1.0)
                .fill(chart_color)
                .motion(rune::tok::motion::SNAPPY),
        );
    }

    col()
        .w_full()
        .min_w(0.0)
        .gap(6.0)
        .padding(10.0)
        .radius(6.0)
        .fill(p.bg_card)
        .border(1.0, p.border)
        .hover(p.bg_card_hover)
        .motion(rune::tok::motion::SNAPPY)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap(6.0)
                .child(
                    text(title.to_string())
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .grow(1.0)
                        .min_w(120.0)
                        .shrink()
                        .truncate(),
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
