//! Oyun içi overlay (Game Overlay & DVR Tuning) View in Rune.
//!
//! Suppresses background DVR gameplay recording, disables Game Bar popups, and forces fullscreen exclusivity.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
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
        .gap(12.0)
        // Header
        .child(
            col()
                .w_full()
                .gap(2.0)
                .child(
                    text("Oyun İçi Overlay ve Game DVR Ayarları")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("Windows Game Bar, Xbox DVR arka plan kayıtları ve oyun içi katmanları yöneterek FPS düşüşlerini önleyin.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Overlay Toggles Card
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
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
                                    text("Windows Game Bar'ı Devre Dışı Bırak")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text("Oyun açılışında Win+G katmanını ve GameBarPresenceWriter arka plan sürecini durdurur.")
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_bar_disabled { "✔ Kapatıldı (Önerilen)" } else { "Açık" },
                                if overlay.game_bar_disabled { p.success } else { p.btn_secondary },
                                if overlay.game_bar_disabled { Color::WHITE } else { p.text_primary },
                                p,
                                |s| {
                                    s.game_overlay.game_bar_disabled = !s.game_overlay.game_bar_disabled;
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
                                    text("Xbox Game DVR Arka Plan Video Kaydı")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text("Oyun sırasında sürekli video kodlamasını engelleyerek GPU kodlayıcı yükünü sıfıra indirir.")
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_dvr_background_recording_disabled { "✔ Engellendi" } else { "Etkin" },
                                if overlay.game_dvr_background_recording_disabled { p.success } else { p.btn_secondary },
                                if overlay.game_dvr_background_recording_disabled { Color::WHITE } else { p.text_primary },
                                p,
                                |s| {
                                    s.game_overlay.game_dvr_background_recording_disabled =
                                        !s.game_overlay.game_dvr_background_recording_disabled;
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
                                    text("Windows Oyun Modu (Game Mode)")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text("Windows'un oyun penceresine donanım önceliği vermesini sağlar.")
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(
                                if overlay.game_mode_enabled { "✔ Aktif" } else { "Devre Dışı" },
                                if overlay.game_mode_enabled { p.success } else { p.btn_secondary },
                                if overlay.game_mode_enabled { Color::WHITE } else { p.text_primary },
                                p,
                                |s| {
                                    s.game_overlay.game_mode_enabled = !s.game_overlay.game_mode_enabled;
                                },
                            )
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
