//! Themed animated progress bar widget for WinBlast0 across all 5 theme styles.
//!
//! Supports standard percentage bars, labeled bars with active readout,
//! animated indeterminate mode for async scanning/benchmarking, and multi-segment bars.

use rune::color::Color;
use rune::{col, el, row, text, El};

use crate::ui::theme::{rgb, rgba, ThemePalette, UiThemeStyle};

/// Standard themed progress bar with corner radius adapted to the active theme style.
pub fn progress_bar(val: f32, color: Color, p: &ThemePalette) -> El {
    progress_bar_styled(val, color, p, 8.0, false)
}

/// Labeled progress bar with a top row showing category title and formatted percentage text.
pub fn progress_bar_labeled(val: f32, label: &str, color: Color, p: &ThemePalette) -> El {
    let clean_val = val.clamp(0.0, 1.0);
    let pct_text = format!("{:.0}%", clean_val * 100.0);

    col()
        .w_full()
        .min_w(0.0)
        .gap(4.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap(6.0)
                .child(
                    text(label.to_string())
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_secondary)
                        .shrink()
                        .truncate(),
                )
                .child(
                    text(pct_text)
                        .font(rune::tok::type_::LABEL)
                        .color(color)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(progress_bar_styled(val, color, p, 7.0, false))
}

/// Indeterminate progress bar with an animated scanning pulse for async operations.
pub fn progress_bar_indeterminate(label: &str, color: Color, p: &ThemePalette) -> El {
    let radius = match p.theme_style {
        UiThemeStyle::Minimalistic => 0.0,
        UiThemeStyle::Classic => 1.0,
        UiThemeStyle::Compact => 2.0,
        UiThemeStyle::Glassy => 4.0,
        UiThemeStyle::Materialistic => 8.0,
    };

    col()
        .w_full()
        .min_w(0.0)
        .gap(4.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap(6.0)
                .child(
                    text(label.to_string())
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .truncate(),
                )
                .child(
                    text("⟳")
                        .font(rune::tok::type_::LABEL)
                        .color(color)
                        .shrink(),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .h(6.0)
                .radius(radius)
                .fill(p.bg_primary)
                .border(p.border_width, p.border)
                .items_center()
                .child(
                    el().w_frac(0.40)
                        .h_full()
                        .radius(radius)
                        .fill(color)
                        .motion(p.motion),
                ),
        )
}

/// Multi-segment progress bar distributing 100% capacity across colored categories.
///
/// `segments`: slice of `(fraction_0_to_1, color, label)`.
pub fn progress_bar_segmented(segments: &[(f32, Color, &'static str)], p: &ThemePalette) -> El {
    let radius = match p.theme_style {
        UiThemeStyle::Minimalistic => 0.0,
        UiThemeStyle::Classic => 1.0,
        UiThemeStyle::Compact => 2.0,
        UiThemeStyle::Glassy => 4.0,
        UiThemeStyle::Materialistic => 8.0,
    };

    let mut segment_bars = Vec::with_capacity(segments.len());
    let mut legend_items = Vec::with_capacity(segments.len());

    for &(frac, col_segment, seg_label) in segments {
        let clean_frac = frac.clamp(0.0, 1.0);
        if clean_frac > 0.0 {
            segment_bars.push(
                el().w_frac(clean_frac)
                    .h_full()
                    .fill(col_segment)
                    .motion(p.motion),
            );
        }

        legend_items.push(
            row()
                .min_w(0.0)
                .shrink()
                .items_center()
                .gap(4.0)
                .child(
                    el().size(8.0, 8.0)
                        .radius(if p.theme_style == UiThemeStyle::Minimalistic {
                            0.0
                        } else {
                            4.0
                        })
                        .fill(col_segment),
                )
                .child(
                    text(format!("{}: {:.0}%", seg_label, clean_frac * 100.0))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        );
    }

    col()
        .w_full()
        .min_w(0.0)
        .gap(6.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .h(8.0)
                .radius(radius)
                .fill(p.bg_primary)
                .border(p.border_width, p.border)
                .children(segment_bars),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gap(8.0)
                .children(legend_items),
        )
}

/// Low-level configurable styled bar conforming to theme geometry and motion tokens.
pub fn progress_bar_styled(
    val: f32,
    color: Color,
    p: &ThemePalette,
    height: f32,
    show_glow: bool,
) -> El {
    let clean_val = val.clamp(0.0, 1.0);
    let radius = match p.theme_style {
        UiThemeStyle::Minimalistic => 0.0,
        UiThemeStyle::Classic => 1.0,
        UiThemeStyle::Compact => 2.0,
        UiThemeStyle::Glassy => (height / 2.0).min(6.0),
        UiThemeStyle::Materialistic => height / 2.0,
    };

    let track_bg = if p.is_dark {
        p.bg_primary
    } else {
        rgb(0.92, 0.94, 0.96)
    };

    let fill_el = if show_glow && p.theme_style == UiThemeStyle::Glassy {
        el().w_frac(clean_val)
            .h_full()
            .radius(radius)
            .fill(color)
            .border(1.0, rgba(1.0, 1.0, 1.0, 0.4))
            .motion(p.motion)
    } else {
        el().w_frac(clean_val)
            .h_full()
            .radius(radius)
            .fill(color)
            .motion(p.motion)
    };

    row()
        .w_full()
        .min_w(0.0)
        .h(height)
        .radius(radius)
        .fill(track_bg)
        .border(p.border_width, p.border)
        .child(fill_el)
}
