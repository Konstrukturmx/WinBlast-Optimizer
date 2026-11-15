//! Sürücü ve BIOS (Driver & BIOS Diagnostics) View in Rune.
//!
//! Visualizes Motherboard SMBIOS tables, UEFI firmware security, and installed GPU display driver stacks.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::hardware::bios::BiosSystemInfo;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let bios = s.bios_info.clone().unwrap_or_else(BiosSystemInfo::probe);

    let sb_badge_color = if bios.secure_boot { p.success } else { p.warning };
    let sb_text = if bios.secure_boot { "✔ Secure Boot Active" } else { "⚠ Secure Boot Disabled / Setup Mode" };

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .gap(12.0)
        // Header
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text("Sürücü ve BIOS Teşhisi")
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text("Motherboard hardware layout, UEFI platform firmware, and active GPU display driver stacks.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    action_button("Yeniden Tara", p.accent, Color::WHITE, p, |s| {
                        s.bios_info = Some(crate::hardware::bios::BiosSystemInfo::probe());
                    })
                    .motion(style.motion())
                    .radius(style.btn_radius()),
                ),
        )
        // Motherboard & BIOS Grid
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    // Motherboard Card
                    col()
                        .grow(1.0)
                        .min_w(280.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(6.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("🎛").font(rune::tok::type_::LABEL))
                                .child(
                                    text("Anakart (Motherboard)")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            text(format!("{} {}", bios.motherboard_vendor, bios.motherboard_product))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(format!("Rev: {} • SMBIOS {}", bios.motherboard_version, bios.smbios_version))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_secondary),
                        ),
                )
                .child(
                    // BIOS / Firmware Card
                    col()
                        .grow(1.0)
                        .min_w(280.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(6.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("💾").font(rune::tok::type_::LABEL))
                                .child(
                                    text("BIOS & UEFI Firmware")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            text(format!("{} ver. {}", bios.bios_vendor, bios.bios_version))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(format!("Yayın Tarihi: {} • UEFI Boot: {}", bios.bios_release_date, if bios.uefi_boot { "Active" } else { "Legacy" }))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_secondary),
                        )
                        .child(
                            row()
                                .px(8.0)
                                .py(3.0)
                                .radius(style.badge_radius())
                                .fill(sb_badge_color.alpha(0.18))
                                .border(1.0, sb_badge_color)
                                .child(
                                    text(sb_text)
                                        .font(rune::tok::type_::LABEL)
                                        .color(sb_badge_color),
                                ),
                        ),
                ),
        )
        // GPU Display Drivers Section
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(8.0)
                .child(
                    row()
                        .gap(6.0)
                        .center()
                        .child(text("🎮").font(rune::tok::type_::LABEL))
                        .child(
                            text("Yüklü Ekran Kartı Sürücüleri (Display Drivers)")
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary),
                        ),
                )
                .children(bios.display_drivers.iter().map(|drv| {
                    row()
                        .w_full()
                        .min_w(0.0)
                        .padding(10.0)
                        .radius(style.card_radius().min(6.0))
                        .fill(p.bg_card_hover)
                        .border(1.0, p.border)
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(220.0)
                                .child(
                                    text(&drv.gpu_name)
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(format!("Sağlayıcı: {} • INF: {}", drv.provider, drv.inf_path))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            col()
                                .min_w(140.0)
                                .child(
                                    text(format!("Sürüm: {}", drv.driver_version))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.success),
                                )
                                .child(
                                    text(format!("Tarih: {}", drv.driver_date))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_secondary),
                                ),
                        )
                })),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
