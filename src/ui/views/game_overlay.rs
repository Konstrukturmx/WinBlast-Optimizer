//! Oyun içi overlay (Game Overlay & DVR Tuning) View in Rune.
//!
//! Suppresses background DVR gameplay recording, disables Game Bar popups, and forces fullscreen exclusivity.

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
    let overlay = &s.game_overlay;

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
                    text(s.tr_v(ViewTextKey::OverlayTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::OverlaySubtitle))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Overlay Toggles Card
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(10.0)
                .child(
                    // Game Bar Toggle
                    row()
                        .w_full()
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(220.0)
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayDisableGameBar))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayDisableGameBarDesc))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_bar_disabled {
                                    format!("✔ {}", s.tr_v(ViewTextKey::OverlayActiveState))
                                } else {
                                    s.tr_v(ViewTextKey::OverlayDisabledState).to_string()
                                },
                                if overlay.game_bar_disabled {
                                    p.success
                                } else {
                                    p.btn_secondary
                                },
                                if overlay.game_bar_disabled {
                                    Color::WHITE
                                } else {
                                    p.text_primary
                                },
                                p,
                                |s| {
                                    s.game_overlay.game_bar_disabled =
                                        !s.game_overlay.game_bar_disabled;
                                },
                            )
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    // Game DVR Background Recording Toggle
                    row()
                        .w_full()
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(220.0)
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayDisableDvr))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayDisableDvrDesc))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_dvr_background_recording_disabled {
                                    format!("✔ {}", s.tr_v(ViewTextKey::OverlayActiveState))
                                } else {
                                    s.tr_v(ViewTextKey::OverlayDisabledState).to_string()
                                },
                                if overlay.game_dvr_background_recording_disabled {
                                    p.success
                                } else {
                                    p.btn_secondary
                                },
                                if overlay.game_dvr_background_recording_disabled {
                                    Color::WHITE
                                } else {
                                    p.text_primary
                                },
                                p,
                                |s| {
                                    let next =
                                        !s.game_overlay.game_dvr_background_recording_disabled;
                                    let _ = s.game_overlay.set_game_dvr_disabled(next);
                                },
                            )
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    // Windows Game Mode Toggle
                    row()
                        .w_full()
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(220.0)
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayGameMode))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(s.tr_v(ViewTextKey::OverlayGameModeDesc))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_mode_enabled {
                                    format!("✔ {}", s.tr_v(ViewTextKey::OverlayActiveState))
                                } else {
                                    s.tr_v(ViewTextKey::OverlayDisabledState).to_string()
                                },
                                if overlay.game_mode_enabled {
                                    p.success
                                } else {
                                    p.btn_secondary
                                },
                                if overlay.game_mode_enabled {
                                    Color::WHITE
                                } else {
                                    p.text_primary
                                },
                                p,
                                |s| {
                                    let next = !s.game_overlay.game_mode_enabled;
                                    let _ = s.game_overlay.set_game_mode_enabled(next);
                                },
                            )
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        .scroll(ScrollSpec {
            stick_bottom: false,
        })
        .winblast_scroll(p)
}
