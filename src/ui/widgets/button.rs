//! Unified theme-aware, contrast-safe interactive UI widgets shared across all WinBlast0 views.
//!
//! Replaces previous per-view duplicated `action_button`, `switch_toggle`, and `kv_row` helpers,
//! fixing Light Mode hover washes, switch knob contrast, and narrow-width flex wrapping.

use rune::color::Color;
use rune::{col, el, row, text, El};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::scanner::TweakState;
use crate::ui::theme::{rgb, ThemePalette};

/// Shared contrast-safe animated action button replacing all per-view copies.
pub fn action_button(
    label: impl Into<String>,
    bg: Color,
    text_color: Color,
    p: &ThemePalette,
    on_click: impl Fn(&mut AppState) + 'static,
) -> El {
    let hover_bg = if bg == p.accent {
        p.accent_hover
    } else if bg == p.bg_card {
        p.bg_card_hover
    } else if bg == p.bg_primary || bg == p.btn_secondary {
        p.btn_secondary_hover
    } else {
        // Keep colored semantic buttons (success/warning/danger) saturated on hover so Color::WHITE stays legible in Light mode!
        bg.alpha(0.84)
    };

    let press_bg = if bg == p.accent {
        p.accent_press
    } else if bg == p.bg_primary || bg == p.bg_card || bg == p.btn_secondary {
        p.btn_secondary_press
    } else {
        bg.alpha(0.70)
    };

    let border_col = if bg == p.bg_primary || bg == p.bg_card || bg == p.btn_secondary {
        p.border
    } else {
        bg.alpha(0.65)
    };

    row()
        .min_w(0.0)
        .shrink()
        .px(11.0)
        .py(7.0)
        .radius(p.btn_radius)
        .fill(bg)
        .hover(hover_bg)
        .press(press_bg)
        .border(p.border_width, border_col)
        .motion(p.motion)
        .center()
        .on_press(on_click)
        .child(
            text(label.into())
                .font(rune::tok::type_::LABEL)
                .color(text_color)
                .shrink()
                .wrap(),
        )
}

/// Contrast-safe animated action button that switches to a muted, non-interactive
/// disabled presentation when `enabled == false` (e.g. during async `in_flight` ops).
pub fn action_button_guarded(
    label: impl Into<String>,
    bg: Color,
    text_color: Color,
    enabled: bool,
    p: &ThemePalette,
    on_click: impl Fn(&mut AppState) + 'static,
) -> El {
    if enabled {
        action_button(label, bg, text_color, p, on_click)
    } else {
        row()
            .min_w(0.0)
            .shrink()
            .px(11.0)
            .py(7.0)
            .radius(p.btn_radius)
            .fill(p.btn_secondary.alpha(0.65))
            .border(p.border_width, p.border.alpha(0.65))
            .center()
            .child(
                text(label.into())
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            )
    }
}

/// Shared spring-animated visual switch core (track + sliding knob), without click binding.
fn switch_visual_shell(active: bool, in_flight: bool, p: &ThemePalette) -> El {
    let knob_fill = if active {
        Color::WHITE
    } else if p.is_dark {
        rgb(0.80, 0.83, 0.88)
    } else {
        p.text_secondary
    };

    let track_fill = if in_flight {
        p.accent.alpha(0.5)
    } else if active {
        p.accent
    } else {
        p.btn_secondary
    };

    let knob = el()
        .size(16.0, 16.0)
        .radius(if p.theme_style == crate::ui::theme::UiThemeStyle::Minimalistic { 1.0 } else { 8.0 })
        .fill(knob_fill)
        .border(1.0, if p.is_dark { Color::TRANSPARENT } else { p.border })
        .absolute(if active { 18.0 } else { 2.0 }, 2.0);

    el()
        .size(36.0, 20.0)
        .radius(if p.theme_style == crate::ui::theme::UiThemeStyle::Minimalistic { 2.0 } else { 10.0 })
        .fill(track_fill)
        .hover(if active { p.accent_hover } else { p.btn_secondary_hover })
        .border(p.border_width, if active { p.border_accent } else { p.border_hover })
        .motion(p.motion)
        .child(knob)
}

/// Shared spring-animated toggle switch bound to catalog tweak ID (`handle_switch_click`).
pub fn switch_toggle(active: bool, in_flight: bool, id: String, p: &ThemePalette) -> El {
    switch_visual_shell(active, in_flight, p)
        .on_press(move |s: &mut AppState| s.handle_switch_click(id.clone()))
}

/// Interactive spring-animated toggle switch with a custom press handler (does NOT pollute `selected_tweaks`).
pub fn switch_toggle_with(
    active: bool,
    in_flight: bool,
    key: impl Into<String>,
    p: &ThemePalette,
    on_press: impl Fn(&mut AppState) + 'static,
) -> El {
    switch_visual_shell(active, in_flight, p)
        .key(key.into())
        .on_press(on_press)
}

/// Pure non-interactive keyed switch indicator for embedding inside an already-clickable row card.
pub fn switch_indicator(
    active: bool,
    in_flight: bool,
    key: impl Into<String>,
    p: &ThemePalette,
) -> El {
    switch_visual_shell(active, in_flight, p).key(key.into())
}

/// Adaptive key-value row (stacks vertically on `Compact` width and wraps long hardware/PnP strings).
pub fn kv_row(key: impl Into<String>, value: impl Into<String>, compact: bool, p: &ThemePalette) -> El {
    let k = key.into();
    let v = value.into();

    if compact {
        col()
            .w_full()
            .min_w(0.0)
            .gap(2.0)
            .child(
                text(k)
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            )
            .child(
                text(v)
                    .font(rune::tok::type_::BODY)
                    .color(p.text_primary)
                    .shrink()
                    .wrap()
                    .wrap_anywhere(),
            )
    } else {
        row()
            .w_full()
            .min_w(0.0)
            .gap(8.0)
            .items_start()
            .child(
                text(k)
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .w(150.0)
                    .wrap(),
            )
            .child(
                text(v)
                    .font(rune::tok::type_::BODY)
                    .color(p.text_primary)
                    .grow(1.0)
                    .min_w(0.0)
                    .shrink()
                    .wrap()
                    .wrap_anywhere(),
            )
    }
}

/// Shared elevated card container with hover wash & dynamic spring motion.
pub fn section_card(title: impl Into<String>, body: El, p: &ThemePalette) -> El {
    col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .hover(p.bg_card_hover)
        .motion(p.motion)
        .gap(10.0)
        .child(
            text(title.into())
                .font(rune::tok::type_::TITLE)
                .color(p.accent)
                .shrink()
                .wrap()
                .wrap_anywhere(),
        )
        .child(body)
}

/// Map `TweakState` to its localized status text and semantic palette color.
pub fn tweak_state_meta(state: &TweakState, s: &AppState) -> (&'static str, Color) {
    let p = s.palette();
    match state {
        TweakState::AlreadyOptimized => (s.tr(TextKey::StatusOptimized), p.success),
        TweakState::NotOptimized => (s.tr(TextKey::StatusNotApplied), p.danger),
        TweakState::PartiallyOptimized => (s.tr(TextKey::StatusPartial), p.warning),
        TweakState::NotApplicable => (s.tr(TextKey::StatusNotApplicable), p.text_muted),
    }
}

/// Compact rounded status pill badge with high-contrast tinted background.
pub fn pill_badge(label: impl Into<String>, color: Color, p: &ThemePalette) -> El {
    row()
        .min_w(0.0)
        .shrink()
        .px(7.0)
        .py(3.0)
        .radius(p.badge_radius)
        .fill(color.alpha(0.16))
        .border(p.border_width, color.alpha(0.45))
        .child(
            text(label.into())
                .font(rune::tok::type_::LABEL)
                .color(color)
                .shrink()
                .wrap(),
        )
}
