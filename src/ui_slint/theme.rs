//! Pushes the Rust ThemePalette (scheme x style x mode) into the Slint `Theme` global.

use slint::ComponentHandle;

use crate::core::theme::{Color, ThemePalette};

use super::{MainWindow, Theme};

pub fn to_slint(c: Color) -> slint::Color {
    let [r, g, b, a] = c.to_srgb8();
    slint::Color::from_argb_u8(a, r, g, b)
}

pub fn apply(window: &MainWindow, p: &ThemePalette, reduced_motion: bool) {
    let t = window.global::<Theme>();
    t.set_dark(p.is_dark);
    t.set_bg_primary(to_slint(p.bg_primary));
    t.set_bg_sidebar(to_slint(p.bg_sidebar));
    t.set_bg_card(to_slint(p.bg_card));
    t.set_bg_card_hover(to_slint(p.bg_card_hover));
    t.set_border(to_slint(p.border));
    t.set_border_hover(to_slint(p.border_hover));
    t.set_border_accent(to_slint(p.border_accent));
    t.set_accent(to_slint(p.accent));
    t.set_accent_hover(to_slint(p.accent_hover));
    t.set_accent_press(to_slint(p.accent_press));
    t.set_text_on_accent(to_slint(p.text_on_accent));
    t.set_text_on_danger(to_slint(p.text_on_danger));
    t.set_btn_secondary(to_slint(p.btn_secondary));
    t.set_btn_secondary_hover(to_slint(p.btn_secondary_hover));
    t.set_btn_secondary_press(to_slint(p.btn_secondary_press));
    t.set_sparkline_cpu(to_slint(p.sparkline_cpu));
    t.set_sparkline_ram(to_slint(p.sparkline_ram));
    t.set_sparkline_bg(to_slint(p.sparkline_bg));
    t.set_text_primary(to_slint(p.text_primary));
    t.set_text_secondary(to_slint(p.text_secondary));
    t.set_text_muted(to_slint(p.text_muted));
    t.set_success(to_slint(p.success));
    t.set_warning(to_slint(p.warning));
    t.set_danger(to_slint(p.danger));
    t.set_neutral_badge(to_slint(p.neutral_badge));
    t.set_card_radius(p.card_radius);
    t.set_btn_radius(p.btn_radius);
    t.set_input_radius(p.input_radius);
    t.set_badge_radius(p.badge_radius);
    t.set_card_padding(p.card_padding);
    t.set_border_width(p.border_width);
    t.set_sidebar_width(p.sidebar_width);
    t.set_motion(p.theme_style.easing_duration_ms() as i64);
    t.set_reduced_motion(reduced_motion);
}
