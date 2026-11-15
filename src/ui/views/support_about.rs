//! Destek ve iletişim (Support, Release 1.1 & About) View in Rune.
//!
//! Release metadata, GitHub repository links, and diagnostic report exporter.

use rune::color::Color;
use rune::{col, el, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
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
                    text(s.tr_v(ViewTextKey::AboutTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::AboutSubtitle))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Project Overview Card
        .child(
            col()
                .w_full()
                .padding(16.0)
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(8.0)
                .child(
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
                                    text(s.tr_v(ViewTextKey::AboutRepoLabel))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_secondary),
                                )
                                .child(
                                    text("Konstrukturmx / WinBlast-Optimizer")
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.accent),
                                )
                                .child(
                                    text("https://github.com/Konstrukturmx/WinBlast-Optimizer")
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::AboutOpenGithub), p.accent, Color::WHITE, p, |_| {
                                let _ = crate::core::process::SilentCommand::new("rundll32")
                                    .args(["url.dll,FileProtocolHandler", "https://github.com/Konstrukturmx/WinBlast-Optimizer"])
                                    .spawn();
                            })
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    text(s.tr_v(ViewTextKey::AboutLicense))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                ),
        )
        // Release 1.1 Features Card
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(6.0)
                .child(
                    text(s.tr_v(ViewTextKey::AboutReleaseNotesTitle))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .child(text(s.tr_v(ViewTextKey::AboutBullet1)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet2)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet3)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet4)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet5)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet6)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet7)).font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text(s.tr_v(ViewTextKey::AboutBullet8)).font(rune::tok::type_::BODY).color(p.text_secondary)),
        )
        // Interactive Theme Studio Card
        .child({
            let mut theme_cards = row().w_full().min_w(0.0).gap(8.0).flex_wrap();
            for &t_style in &crate::ui::theme::UiThemeStyle::ALL {
                let is_active = s.theme_style == t_style;
                let bg_fill = if is_active { p.accent.alpha(0.22) } else { p.card_fill_styled(t_style) };
                let border_col = if is_active { p.accent } else { p.border_styled(t_style) };

                theme_cards = theme_cards.child(
                    col()
                        .grow(1.0)
                        .min_w(170.0)
                        .padding(t_style.card_padding())
                        .radius(t_style.card_radius())
                        .fill(bg_fill)
                        .border(t_style.border_width(), border_col)
                        .hover(if is_active { p.accent.alpha(0.32) } else { p.bg_card_hover })
                        .motion(t_style.motion())
                        .on_press(move |s: &mut AppState| s.set_theme_style(t_style))
                        .gap(4.0)
                        .child(
                            row()
                                .w_full()
                                .justify_between()
                                .items_center()
                                .child(
                                    text(t_style.label())
                                        .font(rune::tok::type_::LABEL)
                                        .color(if is_active { p.accent } else { p.text_primary })
                                        .shrink()
                                        .wrap(),
                                )
                                .child(
                                    if is_active {
                                        row()
                                            .px(5.0)
                                            .py(1.0)
                                            .radius(t_style.badge_radius())
                                            .fill(p.accent)
                                            .child(
                                                text(s.tr_v(ViewTextKey::AboutActiveBadge))
                                                    .font(rune::tok::type_::LABEL)
                                                    .color(Color::WHITE),
                                            )
                                    } else {
                                        el()
                                    },
                                ),
                        )
                        .child(
                            text(format!(
                                "Radius: {:.0}px • Pad: {:.0}px",
                                t_style.card_radius(),
                                t_style.card_padding(),
                            ))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted),
                        ),
                );
            }

            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(8.0)
                .child(
                    text(s.tr_v(ViewTextKey::AboutStudioTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::AboutStudioSubtitle))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
                .child(theme_cards)
        })
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
