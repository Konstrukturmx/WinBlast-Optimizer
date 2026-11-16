//! Arıza teşhisi (Fault Diagnostics) View in Rune.
//!
//! Inspects BSOD kernel crash dumps, Windows CBS pending reboot locks, and component store integrity.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::{ActiveView, AppState};
use crate::core::view_i18n::ViewTextKey;
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
        .pr(16.0)
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
                            text(s.tr_v(ViewTextKey::FaultTitle))
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
                                        format!("{}{}{}", s.tr_v(ViewTextKey::FaultAlertBadgePrefix), report.total_critical_faults, s.tr_v(ViewTextKey::FaultAlertBadgeSuffix))
                                    } else {
                                        s.tr_v(ViewTextKey::FaultSystemStable).to_string()
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
                            action_button(s.tr_v(ViewTextKey::FaultRescanBtn), p.accent, Color::WHITE, p, |s| {
                                s.fault_report = Some(crate::tools::diagnostics::FaultDiagnosticsReport::scan());
                            })
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::FaultClearDumpsBtn), p.btn_secondary, p.text_primary, p, |s| {
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
                        text(s.tr_v(ViewTextKey::FaultPendingRebootWarn))
                            .font(rune::tok::type_::LABEL)
                            .color(p.warning),
                    )
                    .child(
                        text(format!(
                            "{}: {} • {}: {} • {}: {} ",
                            s.tr_v(ViewTextKey::FaultPendingRebootCbs),
                            if report.pending_reboot.cbs_reboot { s.tr_v(ViewTextKey::FaultYes) } else { s.tr_v(ViewTextKey::FaultNo) },
                            s.tr_v(ViewTextKey::FaultPendingRebootWu),
                            if report.pending_reboot.windows_update_reboot { s.tr_v(ViewTextKey::FaultYes) } else { s.tr_v(ViewTextKey::FaultNo) },
                            s.tr_v(ViewTextKey::FaultPendingRebootRename),
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
                        text(s.tr_v(ViewTextKey::FaultPendingRebootClean))
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
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(8.0)
                .child(
                    row()
                        .gap(6.0)
                        .center()
                        .child(text("🛡️").font(rune::tok::type_::LABEL))
                        .child(
                            text(s.tr_v(ViewTextKey::FaultMinidumpSectionTitle))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary),
                        ),
                )
                .child(if report.minidumps.is_empty() {
                    text(s.tr_v(ViewTextKey::FaultMinidumpEmpty))
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
                                            text(format!("{}: {} KB • {}: {}", s.tr_v(ViewTextKey::FaultSize), dump.size_kb, s.tr_v(ViewTextKey::FaultCreated), dump.created_at))
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
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(6.0)
                .child(
                    text("System File Integrity & DISM Repair")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .child(
                    text("If component store or system file corruption is suspected, you can execute DISM RestoreHealth and SFC repair tools.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
                .child(
                    action_button("Open Windows Repair Tool", p.accent, Color::WHITE, p, |s| {
                        s.navigate(ActiveView::WindowsRepair);
                    })
                    .radius(style.btn_radius()),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
