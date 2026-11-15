//! Oyun profilleri (Game Profiles Engine) View in Rune.
//!
//! One-click optimization profiles for Counter-Strike 2, Valorant, Apex, Warzone, Cyberpunk, and Fortnite.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;

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
                    text("Özel Oyun Optimizasyon Profilleri")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("Her oyunun kendine özel thread dağılımı, GPU shader önbelleği ve tam ekran optimizasyon ayarlarını uygulayın.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Game Profiles Grid
        .children(s.game_profiles.iter().enumerate().map(|(idx, profile)| {
            let is_active = profile.is_applied;
            col()
                .w_full()
                .padding(14.0)
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), if is_active { p.accent } else { p.border_styled(style) })
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
                                .child(text(profile.icon_glyph).font(rune::tok::type_::TITLE))
                                .child(
                                    col()
                                        .child(
                                            text(profile.title)
                                                .font(rune::tok::type_::TITLE)
                                                .color(p.text_primary),
                                        )
                                        .child(
                                            text(format!("Hedef: {}", profile.executable))
                                                .font(rune::tok::type_::BODY)
                                                .color(p.text_muted),
                                        ),
                                ),
                        )
                        .child(
                            action_button(
                                if is_active { "✔ Profil Aktif" } else { "Profili Uygula" },
                                if is_active { p.success } else { p.accent },
                                Color::WHITE,
                                p,
                                move |s| {
                                    if let Some(prof) = s.game_profiles.get_mut(idx) {
                                        prof.is_applied = !prof.is_applied;
                                    }
                                },
                            )
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    text(profile.description)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
                .child(
                    row()
                        .gaps(12.0, 4.0)
                        .flex_wrap()
                        .child(
                            text("• Yüksek CPU Önceliği (High Priority)")
                                .font(rune::tok::type_::LABEL)
                                .color(if profile.high_priority { p.success } else { p.text_muted }),
                        )
                        .child(
                            text("• Tam Ekran İyileştirmelerini Devre Dışı Bırak (Disable FSO)")
                                .font(rune::tok::type_::LABEL)
                                .color(if profile.disable_fso { p.success } else { p.text_muted }),
                        )
                        .child(
                            text("• Tüm Çekirdekleri Uyandır (Unpark Cores)")
                                .font(rune::tok::type_::LABEL)
                                .color(if profile.unpark_cores { p.success } else { p.text_muted }),
                        ),
                )
        }))
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
