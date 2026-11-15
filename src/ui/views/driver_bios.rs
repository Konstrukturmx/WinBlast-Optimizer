//! Sürücü ve BIOS (Driver & BIOS Diagnostics) View in Rune.
//!
//! Visualizes Motherboard SMBIOS tables, UEFI firmware security, installed GPU display driver stacks,
//! and provides full online driver scanning, version verification, download, and installation.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::hardware::bios::BiosSystemInfo;
use crate::hardware::driver_update::DriverUpdateState;
use crate::ui::widgets::button::{action_button, action_button_guarded};
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
        // Top Header
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
                            text(s.tr(TextKey::NavDriverBios))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text("Motherboard hardware layout, UEFI platform firmware, and online driver lifecycle manager.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .gap(8.0)
                        .flex_wrap()
                        .child(
                            action_button_guarded(
                                if s.is_driver_scanning {
                                    s.tr(TextKey::Scanning)
                                } else {
                                    s.tr(TextKey::DriverScanOnline)
                                },
                                p.accent,
                                Color::WHITE,
                                !s.is_driver_scanning,
                                p,
                                |s| s.scan_online_drivers(),
                            )
                            .motion(style.motion())
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button_guarded(
                                s.tr(TextKey::DriverTestInstallSandbox),
                                p.btn_secondary,
                                p.text_primary,
                                s.driver_install_in_flight.is_none(),
                                p,
                                |s| s.test_sandbox_driver_install(),
                            )
                            .motion(style.motion())
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button("Yeniden Tara", p.btn_secondary, p.text_primary, p, |s| {
                                s.bios_info = Some(crate::hardware::bios::BiosSystemInfo::probe());
                            })
                            .motion(style.motion())
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        // Online Driver Update Center Card
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(10.0)
                .child(
                    row()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .flex_wrap()
                        .gaps(8.0, 6.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("🌐").font(rune::tok::type_::LABEL))
                                .child(
                                    text("Çevrimiçi Sürücü Güncelleme & Doğrulama")
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            text(format!("{} Cihaz Taranmış", s.driver_updates.len()))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        ),
                )
                .child(
                    if s.driver_updates.is_empty() {
                        row()
                            .w_full()
                            .padding(14.0)
                            .center()
                            .child(
                                text(if s.is_driver_scanning {
                                    "Çevrimiçi sürücü katalogları taranıyor..."
                                } else {
                                    "Sürücüleri taramak için 'Çevrimiçi Sürücüleri Tara' veya 'Test Sürücüsü' butonuna tıklayın."
                                })
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                            )
                    } else {
                        let mut driver_list = col().w_full().min_w(0.0).gap(6.0);
                        for item in &s.driver_updates {
                            let is_busy = s.driver_install_in_flight.as_deref() == Some(&item.id);
                            let (badge_text, badge_color) = match item.state {
                                DriverUpdateState::Installed => (s.tr(TextKey::DriverInstalledSuccess), p.success),
                                DriverUpdateState::UpdateAvailable => (s.tr(TextKey::DriverUpdateAvailable), p.warning),
                                DriverUpdateState::UpToDate => (s.tr(TextKey::DriverUpToDate), p.text_secondary),
                                DriverUpdateState::Downloading => ("İndiriliyor...", p.accent),
                                DriverUpdateState::Installing => (s.tr(TextKey::DriverInstalling), p.accent),
                                DriverUpdateState::Failed => ("Hata ✘", p.danger),
                                DriverUpdateState::ReadyToInstall => ("Kuruluma Hazır", p.accent),
                            };

                            let driver_id = item.id.clone();
                            let row_card = row()
                                .w_full()
                                .min_w(0.0)
                                .padding(10.0)
                                .radius(style.card_radius().min(6.0))
                                .fill(p.bg_card_hover)
                                .border(1.0, p.border)
                                .center()
                                .gaps(10.0, 8.0)
                                .flex_wrap()
                                .child(
                                    row()
                                        .gap(6.0)
                                        .items_center()
                                        .child(text(item.category.icon()).font(rune::tok::type_::TITLE))
                                        .child(
                                            col()
                                                .grow(1.0)
                                                .min_w(200.0)
                                                .child(
                                                    text(&item.name)
                                                        .font(rune::tok::type_::LABEL)
                                                        .color(p.text_primary),
                                                )
                                                .child(
                                                    text(format!("{} • Sağlayıcı: {} • {}", item.category.display_name(), item.provider, item.status_message))
                                                        .font(rune::tok::type_::BODY)
                                                        .color(p.text_muted),
                                                ),
                                        ),
                                )
                                .child(
                                    col()
                                        .min_w(160.0)
                                        .child(
                                            text(format!("Mevcut: {} ➔ Son: {}", item.current_version, item.latest_version))
                                                .font(rune::tok::type_::LABEL)
                                                .color(if item.is_update_available() { p.warning } else { p.text_secondary }),
                                        )
                                        .child(
                                            text(format!("Tarih: {} ({:.1} MB)", item.release_date, item.download_size_mb))
                                                .font(rune::tok::type_::BODY)
                                                .color(p.text_muted),
                                        ),
                                )
                                .child(
                                    row()
                                        .px(8.0)
                                        .py(3.0)
                                        .radius(style.badge_radius())
                                        .fill(badge_color.alpha(0.18))
                                        .border(1.0, badge_color)
                                        .child(
                                            text(badge_text)
                                                .font(rune::tok::type_::LABEL)
                                                .color(badge_color),
                                        ),
                                )
                                .child(
                                    action_button_guarded(
                                        if is_busy {
                                            s.tr(TextKey::DriverInstalling)
                                        } else if item.is_update_available() {
                                            s.tr(TextKey::DriverDownloadInstall)
                                        } else {
                                            "Doğrula / Kur"
                                        },
                                        if item.is_update_available() { p.accent } else { p.btn_secondary },
                                        if item.is_update_available() { Color::WHITE } else { p.text_primary },
                                        !is_busy && s.driver_install_in_flight.is_none(),
                                        p,
                                        move |s| s.install_driver_update(driver_id.clone()),
                                    )
                                    .motion(style.motion())
                                    .radius(style.btn_radius()),
                                );

                            driver_list = driver_list.child(row_card);
                        }
                        driver_list
                    },
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
