//! Driver and BIOS Diagnostics View in Rune.
//!
//! Visualizes Motherboard SMBIOS tables, UEFI firmware security, installed GPU display driver stacks,
//! and provides full online driver scanning, category filtering, search, version verification,
//! backup, and installation within bounded scroll containers.

use rune::color::Color;
use rune::{col, row, text, text_input, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;
use crate::hardware::driver_update::{DriverCategory, DriverUpdateState};
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let is_compact = tier == LayoutTier::Compact;
    let bios = s.bios_info.clone().unwrap_or_default();

    let sb_badge_color = if bios.secure_boot { p.success } else { p.warning };
    let sb_text = if bios.secure_boot { "✔ Secure Boot Active" } else { "⚠ Secure Boot Disabled / Setup Mode" };

    let q_raw = s.driver_search_edit.text.trim().to_lowercase();

    col()
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(style.card_padding())
        .pr(16.0)
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
                            text(s.tr_v(ViewTextKey::DriverBiosSubtitle))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(8.0, 8.0)
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
                            action_button("Rescan BIOS", p.btn_secondary, p.text_primary, p, |s| {
                                s.scan_bios_info_async();
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
                                    text(s.tr_v(ViewTextKey::DriverBiosOnlineUpdateTitle))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            text(format!("{} Devices Scanned", s.driver_updates.len()))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        ),
                )
                // Filter & Search Toolbar
                .child(
                    row()
                        .w_full()
                        .min_w(0.0)
                        .items_center()
                        .gaps(8.0, 8.0)
                        .flex_wrap()
                        .child(
                            row()
                                .grow(1.0)
                                .min_w(220.0)
                                .min_h(32.0)
                                .px(8.0)
                                .py(4.0)
                                .radius(6.0)
                                .fill(p.bg_card)
                                .border(1.0, p.border)
                                .items_center()
                                .child(
                                    text_input(&s.driver_search_edit, rune::tok::type_::BODY)
                                        .key("driver_search_input")
                                        .single_line()
                                        .placeholder(s.tr(TextKey::DriverSearchPlaceholder), p.text_muted)
                                        .color(p.text_primary)
                                        .on_edit(|s: &mut AppState| &mut s.driver_search_edit)
                                        .grow(1.0)
                                        .min_w(0.0),
                                ),
                        ),
                )
                // Category Filter Chips
                .child(
                    row()
                        .w_full()
                        .min_w(0.0)
                        .gap(6.0)
                        .flex_wrap()
                        .child({
                            let is_active = s.driver_category_filter.is_none();
                            action_button(
                                s.tr(TextKey::DriverFilterAll),
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = None,
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Display);
                            action_button(
                                s.tr(TextKey::DriverFilterDisplay),
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Display),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Network);
                            action_button(
                                s.tr(TextKey::DriverFilterNetwork),
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Network),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Audio);
                            action_button(
                                s.tr(TextKey::DriverFilterAudio),
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Audio),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Storage);
                            action_button(
                                s.tr(TextKey::DriverFilterStorage),
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Storage),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Chipset);
                            action_button(
                                "⚙ Chipset",
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Chipset),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Usb);
                            action_button(
                                "🔌 USB",
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Usb),
                            )
                            .radius(style.badge_radius())
                        })
                        .child({
                            let is_active = s.driver_category_filter == Some(DriverCategory::Bluetooth);
                            action_button(
                                "🔷 Bluetooth",
                                if is_active { p.accent } else { p.bg_card },
                                if is_active { Color::WHITE } else { p.text_secondary },
                                p,
                                |s| s.driver_category_filter = Some(DriverCategory::Bluetooth),
                            )
                            .radius(style.badge_radius())
                        }),
                )
                // Driver Cards inside Bounded Scroll Container
                .child(
                    if s.driver_updates.is_empty() {
                        col()
                            .key("driver_updates_scroll")
                            .w_full()
                            .min_w(0.0)
                            .padding(p.card_padding)
                            .center()
                            .gap(8.0)
                            .child(
                                text(if s.is_driver_scanning { "⟳" } else { "🔍" })
                                    .font(rune::tok::type_::TITLE)
                                    .color(if s.is_driver_scanning { p.accent } else { p.text_muted }),
                            )
                            .child(
                                text(if s.is_driver_scanning {
                                    "Scanning online driver catalogs and PnP devices..."
                                } else {
                                    "Click 'Scan Drivers Online' or 'Test Sandbox Driver' to scan system hardware."
                                })
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                            )
                    } else {
                        let filtered_drivers: Vec<_> = s
                            .driver_updates
                            .iter()
                            .filter(|item| {
                                if let Some(cat) = s.driver_category_filter {
                                    if item.category != cat {
                                        return false;
                                    }
                                }
                                if q_raw.is_empty() {
                                    return true;
                                }
                                item.name.to_lowercase().contains(&q_raw)
                                    || item.provider.to_lowercase().contains(&q_raw)
                                    || item.hardware_id.to_lowercase().contains(&q_raw)
                                    || item.category.display_name().to_lowercase().contains(&q_raw)
                                    || item.current_version.to_lowercase().contains(&q_raw)
                                    || item.latest_version.to_lowercase().contains(&q_raw)
                            })
                            .collect();

                        let mut driver_list = col().w_full().min_w(0.0).gap(6.0);

                        if filtered_drivers.is_empty() {
                            driver_list = driver_list.child(
                                col()
                                    .w_full()
                                    .min_w(0.0)
                                    .padding(16.0)
                                    .center()
                                    .child(
                                        text(s.tr_v(ViewTextKey::DriverBiosNoDevicesFound))
                                            .font(rune::tok::type_::BODY)
                                            .color(p.text_muted),
                                    ),
                            );
                        } else {
                            for item in &filtered_drivers {
                                let is_busy = s.driver_install_in_flight.as_deref() == Some(&item.id);
                                let (badge_text, badge_color) = match item.state {
                                    DriverUpdateState::Installed => (s.tr(TextKey::DriverInstalledSuccess), p.success),
                                    DriverUpdateState::UpdateAvailable => (s.tr(TextKey::DriverUpdateAvailable), p.warning),
                                    DriverUpdateState::UpToDate => (s.tr(TextKey::DriverUpToDate), p.text_secondary),
                                    DriverUpdateState::Downloading => ("Downloading...", p.accent),
                                    DriverUpdateState::Installing => (s.tr(TextKey::DriverInstalling), p.accent),
                                    DriverUpdateState::Failed => ("Failed ✘", p.danger),
                                    DriverUpdateState::ReadyToInstall => ("Ready to Install", p.accent),
                                };

                                let driver_id = item.id.clone();
                                let backup_id = item.id.clone();

                                let mut hw_details = format!("{} • Provider: {}", item.category.display_name(), item.provider);
                                if !item.hardware_id.is_empty() {
                                    hw_details.push_str(&format!(" • HW: {}", item.hardware_id));
                                }
                                hw_details.push_str(&format!(" • {}", item.status_message));

                                let (whql_text, whql_color) = if item.is_whql_signed {
                                    ("✔ WHQL", p.success)
                                } else {
                                    ("Unverified", p.text_secondary)
                                };

                                let row_card = row()
                                    .w_full()
                                    .min_w(0.0)
                                    .padding(8.0)
                                    .pr(12.0)
                                    .radius(style.card_radius().min(6.0))
                                    .fill(p.bg_card_hover)
                                    .border(1.0, p.border)
                                    .center()
                                    .gaps(8.0, 6.0)
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
                                                        text(hw_details)
                                                            .font(rune::tok::type_::BODY)
                                                            .color(p.text_muted),
                                                    ),
                                            ),
                                    )
                                    .child(
                                        col()
                                            .min_w(150.0)
                                            .child(
                                                text(format!("Current: {} ➔ Latest: {}", item.current_version, item.latest_version))
                                                    .font(rune::tok::type_::LABEL)
                                                    .color(if item.is_update_available() { p.warning } else { p.text_secondary }),
                                            )
                                            .child(
                                                text(format!("Date: {} ({:.1} MB)", item.release_date, item.download_size_mb))
                                                    .font(rune::tok::type_::BODY)
                                                    .color(p.text_muted),
                                            ),
                                    )
                                    .child(
                                        row()
                                            .px(6.0)
                                            .py(2.0)
                                            .radius(style.badge_radius())
                                            .fill(whql_color.alpha(0.12))
                                            .border(1.0, whql_color.alpha(0.4))
                                            .child(
                                                text(whql_text)
                                                    .font(rune::tok::type_::LABEL)
                                                    .color(whql_color),
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
                                        row()
                                            .gap(6.0)
                                            .items_center()
                                            .child(
                                                action_button_guarded(
                                                    "Backup",
                                                    p.btn_secondary,
                                                    p.text_primary,
                                                    !is_busy,
                                                    p,
                                                    move |s| s.backup_driver(backup_id.clone()),
                                                )
                                                .motion(style.motion())
                                                .radius(style.btn_radius()),
                                            )
                                            .child(
                                                action_button_guarded(
                                                    if is_busy {
                                                        s.tr(TextKey::DriverInstalling)
                                                    } else if item.is_update_available() {
                                                        s.tr(TextKey::DriverDownloadInstall)
                                                    } else {
                                                        "Verify / Update"
                                                    },
                                                    if item.is_update_available() { p.accent } else { p.btn_secondary },
                                                    if item.is_update_available() { Color::WHITE } else { p.text_primary },
                                                    !is_busy && s.driver_install_in_flight.is_none(),
                                                    p,
                                                    move |s| s.install_driver_update(driver_id.clone()),
                                                )
                                                .motion(style.motion())
                                                .radius(style.btn_radius()),
                                            ),
                                    );

                                driver_list = driver_list.child(row_card);
                            }
                        }

                        let list_height = if filtered_drivers.len() <= 3 {
                            (filtered_drivers.len() as f32 * 68.0).max(130.0)
                        } else if is_compact {
                            300.0
                        } else {
                            380.0
                        };

                        col()
                            .key("driver_updates_scroll")
                            .w_full()
                            .min_w(0.0)
                            .h(list_height)
                            .min_h(120.0)
                            .padding(2.0)
                            .pr(8.0)
                            .clip_soft(0.0)
                            .scroll(ScrollSpec { stick_bottom: false })
                            .child(driver_list)
                            .winblast_scroll(p)
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
                                    text(s.tr_v(ViewTextKey::DriverBiosMotherboardSection))
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
                                    text(s.tr_v(ViewTextKey::DriverBiosFirmwareSection))
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
                            text(format!("Release Date: {} • UEFI Boot: {}", bios.bios_release_date, if bios.uefi_boot { "Active" } else { "Legacy" }))
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
                            text(s.tr_v(ViewTextKey::DriverBiosGpuDriversSection))
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
                                    text(format!("Provider: {} • INF: {}", drv.provider, drv.inf_path))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                 ),
                        )
                        .child(
                            col()
                                .min_w(140.0)
                                .child(
                                    text(format!("Version: {}", drv.driver_version))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.success),
                                )
                                .child(
                                    text(format!("Date: {}", drv.driver_date))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_secondary),
                                ),
                        )
                })),
        )
        .scroll(ScrollSpec { stick_bottom: false })
        .winblast_scroll(p)
}
