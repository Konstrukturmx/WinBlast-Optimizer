//! Lisans (Windows License & Activation Information) View in Rune.
//!
//! Queries Windows Product Edition, genuine digital activation entitlement, and license channel.

use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let lic = &s.license_info;

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
                    text("Windows Lisans ve Etkinleştirme Bilgisi")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("Sisteminizin Windows sürüm, lisans kanalı ve dijital ürün anahtarı durumu.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Activation Hero Banner
        .child(
            row()
                .w_full()
                .padding(16.0)
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.success)
                .center()
                .gaps(14.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text("Etkinleştirme Durumu")
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        )
                        .child(
                            text(&lic.activation_status)
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        )
                        .child(
                            text("Windows bir Microsoft Hesabına bağlı dijital lisans ile kalıcı olarak etkinleştirildi.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .px(12.0)
                        .py(6.0)
                        .radius(style.badge_radius())
                        .fill(p.success.alpha(0.18))
                        .border(1.0, p.success)
                        .child(
                            text("✔ Lisans Orijinal")
                                .font(rune::tok::type_::LABEL)
                                .color(p.success),
                        ),
                ),
        )
        // License Details Grid
        .child(
            row()
                .w_full()
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(4.0)
                        .child(text("İşletim Sistemi Sürümü").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(&lic.product_name)
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(format!("Yapı: {} (Sürüm {})", lic.build_number, lic.display_version))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(4.0)
                        .child(text("Lisans Kanalı").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(&lic.license_channel)
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text(format!("Kısmi Anahtar: {}", lic.partial_product_key))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
