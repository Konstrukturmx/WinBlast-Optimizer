//! Arıza teşhisi (Fault Diagnostics) View in Rune.
//!
//! Inspects BSOD kernel crash dumps, Windows CBS pending reboot locks, and component store integrity.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::{ActiveView, AppState};
use crate::tools::diagnostics::FaultDiagnosticsReport;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let report = s.fault_report.clone().unwrap_or_else(FaultDiagnosticsReport::scan);

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .gap(12.0)
        // Header with Alert Badge
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .child(
                    row()
                        .gap(8.0)
                        .center()
                        .child(
                            text("Arıza Teşhisi ve Hata Analizi")
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            row()
                                .px(8.0)
                                .py(2.0)
                                .radius(style.badge_radius())
                                .fill(if report.total_critical_faults > 0 { p.danger } else { p.success })
                                .child(
                                    text(if report.total_critical_faults > 0 {
                                        format!("⚠ {} Arıza Tespit Edildi", report.total_critical_faults)
                                    } else {
                                        "✔ Sistem Kararlı (0 Arıza)".to_string()
                                    })
                                    .font(rune::tok::type_::LABEL)
                                    .color(Color::WHITE),
                                ),
                        ),
                )
                .child(
                    row()
                        .gap(8.0)
                        .child(
                            action_button("Yeniden Tara", p.accent, Color::WHITE, p, |s| {
                                s.fault_report = Some(crate::tools::diagnostics::FaultDiagnosticsReport::scan());
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button("Minidumpları Temizle", p.btn_secondary, p.text_primary, p, |s| {
                                if let Some(ref r) = s.fault_report {
                                    let _ = r.clear_minidumps();
                                }
                                s.fault_report = Some(crate::tools::diagnostics::FaultDiagnosticsReport::scan());
                            })
                            .radius(style.btn_radius()),
                        ),
                ),
        )
        // Pending Reboot Warning Card
        .child(
            if report.pending_reboot.is_pending {
                col()
                    .w_full()
                    .padding(style.card_padding())
                    .radius(style.card_radius())
                    .fill(p.warning.alpha(0.15))
                    .border(1.0, p.warning)
                    .gap(4.0)
                    .child(
                        text("⚠ Sistem Yeniden Başlatma Bekliyor (Pending Reboot Active)")
                            .font(rune::tok::type_::LABEL)
                            .color(p.warning),
                    )
                    .child(
                        text(format!(
                            "CBS Servicing: {} • Windows Update: {} • Bekleyen Dosya Değişimi: {} adet",
                            if report.pending_reboot.cbs_reboot { "Evet" } else { "Hayır" },
                            if report.pending_reboot.windows_update_reboot { "Evet" } else { "Hayır" },
                            report.pending_reboot.pending_file_count
                        ))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_primary),
                    )
            } else {
                col()
                    .w_full()
                    .padding(10.0)
                    .radius(style.card_radius())
                    .fill(p.success.alpha(0.12))
                    .border(1.0, p.success)
                    .child(
                        text("✔ Bekleyen Sistem Yeniden Başlatması Yok (Reboot lock temiz)")
                            .font(rune::tok::type_::LABEL)
                            .color(p.success),
                    )
            },
        )
        // BSOD Minidumps List Card
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(8.0)
                .child(
                    row()
                        .gap(6.0)
                        .center()
                        .child(text("🛡️").font(rune::tok::type_::LABEL))
                        .child(
                            text("Mavi Ekran (BSOD) Minidump Dosyaları (%SystemRoot%\\Minidump)")
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary),
                        ),
                )
                .child(if report.minidumps.is_empty() {
                    text("Temiz: Sisteminizde kayıtlı mavi ekran minidump dosyası bulunamadı.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted)
                } else {
                    col()
                        .gap(6.0)
                        .children(report.minidumps.iter().map(|dump| {
                            row()
                                .w_full()
                                .padding(8.0)
                                .radius(style.card_radius().min(4.0))
                                .fill(p.bg_card_hover)
                                .center()
                                .child(
                                    col()
                                        .grow(1.0)
                                        .child(
                                            text(&dump.file_name)
                                                .font(rune::tok::type_::LABEL)
                                                .color(p.danger),
                                        )
                                        .child(
                                            text(format!("Boyut: {} KB • Tarih: {}", dump.size_kb, dump.created_at))
                                                .font(rune::tok::type_::BODY)
                                                .color(p.text_muted),
                                        ),
                                )
                        }))
                }),
        )
        // Windows Component Store Recommendation Card
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(6.0)
                .child(
                    text("Sistem Dosya Bütünlüğü & DISM Onarımı")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .child(
                    text("Windows bileşen deposunda veya sistem dosyalarında bozulma şüphesi varsa DISM RestoreHealth ve SFC araçlarını çalıştırabilirsiniz.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
                .child(
                    action_button("Windows Onarım Aracını Aç", p.accent, Color::WHITE, p, |s| {
                        s.navigate(ActiveView::WindowsRepair);
                    })
                    .radius(style.btn_radius()),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
