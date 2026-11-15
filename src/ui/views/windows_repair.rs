//! Windows onarımı (Windows Component Repair Suite) View in Rune.
//!
//! Provides one-click execution of SFC, DISM RestoreHealth, Winsock reset, and Windows Update repair routines.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::tools::windows_repair::RepairTask;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let tasks = &s.repair_tasks;

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
                    text(s.tr_v(ViewTextKey::RepairTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::RepairSubtitle))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Repair Tasks Grid
        .children(tasks.iter().map(|task| {
            let task_id = task.id;
            col()
                .w_full()
                .padding(14.0)
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
                            row()
                                .gap(8.0)
                                .center()
                                .grow(1.0)
                                .min_w(200.0)
                                .child(text(task.icon_glyph).font(rune::tok::type_::TITLE))
                                .child(
                                    col()
                                        .child(
                                            text(task.title)
                                                .font(rune::tok::type_::TITLE)
                                                .color(p.text_primary),
                                        )
                                        .child(
                                            text(format!("{}: {}", s.tr_v(ViewTextKey::RepairCommandLabel), task.command_preview))
                                                .font(rune::tok::type_::BODY)
                                                .color(p.text_muted),
                                        ),
                                 ),
                        )
                        .child(
                            action_button(s.tr_v(ViewTextKey::RepairRunBtn), p.accent, Color::WHITE, p, move |s| {
                                match RepairTask::execute(task_id) {
                                    Ok(msg) => {
                                        s.repair_log.push(format!("✔ [{}] {}", task_id, msg));
                                        s.status_message = format!("{}: {}", s.tr_v(ViewTextKey::RepairDonePrefix), task_id);
                                    }
                                    Err(err) => {
                                        s.repair_log.push(format!("✖ [{}] Error: {}", task_id, err));
                                        s.status_message = format!("{}: {}", s.tr_v(ViewTextKey::RepairErrPrefix), err);
                                    }
                                }
                            })
                            .radius(style.btn_radius()),
                        ),
                )
                .child(
                    text(task.description)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
        }))
        // Live Execution Output Console
        .child(
            col()
                .w_full()
                .padding(12.0)
                .radius(style.card_radius())
                .fill(if p.is_dark { crate::ui::theme::rgb(0.01, 0.012, 0.018) } else { crate::ui::theme::rgb(0.92, 0.93, 0.95) })
                .border(1.0, p.border)
                .gap(6.0)
                .child(
                    text(s.tr_v(ViewTextKey::RepairConsoleTitle))
                        .font(rune::tok::type_::LABEL)
                        .color(p.accent),
                )
                .child(if s.repair_log.is_empty() {
                    text(s.tr_v(ViewTextKey::RepairReadyStatus))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted)
                } else {
                    col()
                        .gap(4.0)
                        .children(s.repair_log.iter().map(|line| {
                            text(line)
                                .font(rune::tok::type_::BODY)
                                .color(if line.starts_with('✔') { p.success } else { p.danger })
                        }))
                }),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
