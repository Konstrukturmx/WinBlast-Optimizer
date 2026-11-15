//! Diskler (Disk Health & TRIM Diagnostics) View in Rune.
//!
//! Evaluates physical storage SMART health, NVMe lifetime TBW, and issues fast TRIM optimizations.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::hardware::disks::DiskSubsystemReport;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let report = s.disk_report.clone().unwrap_or_else(DiskSubsystemReport::probe);

    let trim_color = if report.global_trim_active { p.success } else { p.warning };

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .gap(12.0)
        // Header
        .child(
            row()
                .w_full()
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text("Disk Sağlığı ve TRIM Teşhisi")
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text("Physical drive SMART wear indicators, lifetime TBW writes, and NVMe DirectStorage telemetry.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .gap(8.0)
                        .child(
                            action_button("Yenile", p.btn_secondary, p.text_primary, p, |s| {
                                s.disk_report = Some(crate::hardware::disks::DiskSubsystemReport::probe());
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button("Tüm SSD'lere TRIM Uygula", p.accent, Color::WHITE, p, |s| {
                                let _ = crate::hardware::disks::DiskSubsystemReport::run_trim_all();
                                s.disk_report = Some(crate::hardware::disks::DiskSubsystemReport::probe());
                            })
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        // TRIM Status Banner
        .child(
            row()
                .w_full()
                .padding(10.0)
                .radius(style.card_radius())
                .fill(trim_color.alpha(0.12))
                .border(1.0, trim_color)
                .center()
                .child(
                    text(format!(
                        "Windows TRIM Desteği: {} • DirectStorage 1.2 Bypass: {}",
                        if report.global_trim_active { "AKTİF (SSD Ömrü & Yazma Performansı Korunuyor)" } else { "DEVRE DIŞI" },
                        if report.nvme_direct_storage_supported { "Hazır" } else { "Desteklenmiyor" }
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(trim_color),
                ),
        )
        // Physical Drives List
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(8.0)
                .child(
                    text("Fiziksel Depolama Sürücüleri")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .children(report.drives.iter().map(|drv| {
                    let health_color = if drv.health_percentage >= 90 {
                        p.success
                    } else if drv.health_percentage >= 75 {
                        p.warning
                    } else {
                        p.danger
                    };

                    col()
                        .w_full()
                        .padding(12.0)
                        .radius(style.card_radius().min(6.0))
                        .fill(p.bg_card_hover)
                        .border(1.0, p.border)
                        .gap(6.0)
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
                                            text(format!("Disk {}: {}", drv.index, drv.model))
                                                .font(rune::tok::type_::LABEL)
                                                .color(p.text_primary),
                                        )
                                        .child(
                                            text(format!("Arayüz: {} • Kapasite: {:.1} GB", drv.interface_type, drv.total_size_gb))
                                                .font(rune::tok::type_::BODY)
                                                .color(p.text_muted),
                                        ),
                                )
                                .child(
                                    col()
                                        .min_w(130.0)
                                        .child(
                                            text(format!("Sağlık: %{}", drv.health_percentage))
                                                .font(rune::tok::type_::LABEL)
                                                .color(health_color),
                                        )
                                        .child(
                                            text(&drv.status)
                                                .font(rune::tok::type_::BODY)
                                                .color(health_color),
                                        ),
                                ),
                        )
                        .child(
                            row()
                                .gaps(14.0, 4.0)
                                .flex_wrap()
                                .child(
                                    text(format!("Toplam Yazılan (TBW): {:.1} TB", drv.lifetime_writes_tb))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_secondary),
                                )
                                .child(
                                    text(format!("Çalışma Saati: {} saat", drv.power_on_hours))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_secondary),
                                )
                                .child(
                                    text(format!("Sıcaklık: {:.0}°C", drv.temperature_c))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.accent),
                                ),
                        )
                })),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
