//! Destek ve iletişim (Support, Release 1.1 & About) View in Rune.
//!
//! Release metadata, GitHub repository links, and diagnostic report exporter.

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
                    text("WinBlast Optimizer — Destek ve İletişim")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("Sürüm 1.1.0 • Açık kaynak Windows optimizasyon ve düşük gecikmeli çekirdek motoru.")
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
                                    text("Geliştirici & GitHub Deposu")
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
                            action_button("GitHub Sayfasını Aç", p.accent, Color::WHITE, p, |_| {
                                let _ = std::process::Command::new("rundll32")
                                    .args(["url.dll,FileProtocolHandler", "https://github.com/Konstrukturmx/WinBlast-Optimizer"])
                                    .spawn();
                            })
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    text("WinBlast0, GPU destekli Rune UI çerçevesi ve saf Rust Win32 FFI çekirdeği ile inşa edilmiştir. Lisans: MIT License.")
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
                    text("Sürüm 1.1.0 Yenilikleri")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .child(text("• 5 Tam UI Teması: Minimalistic, Classic, Glassy, Compact ve Materialistic.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Sürücü ve BIOS teşhisi: SMBIOS anakart, UEFI ve GPU display driver analizi.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Arıza teşhisi: BSOD minidump tarayıcı ve CBS bekleyen yeniden başlatma tespit motoru.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Sıcaklık ve Fanlar: Canlı CPU/GPU junction sıcaklıkları ve PROCHOT uyarısı.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Diskler: SMART sağlık yüzdesi, TBW yazma ömrü ve tek tıkla TRIM.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Oyun Profilleri: CS2, Valorant, Apex, Warzone, Cyberpunk için özel öncelik ve shader ayarları.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Windows Onarımı: SFC, DISM, Winsock reset ve DNS flush araçları.") .font(rune::tok::type_::BODY).color(p.text_secondary))
                .child(text("• Kalıcı WinBlast İkonu ve Microsoft SignTool dijital imzalı PE dosyası.") .font(rune::tok::type_::BODY).color(p.text_secondary)),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
