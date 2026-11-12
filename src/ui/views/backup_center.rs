//! Backup Center in Rune — 100% localized VSS restore points, `.wb0` profile export/import,
//! and responsive SQLite transaction ledger with 1-click Undo.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::LayoutTier;

/// Renders the responsive, fully localized Backup Center view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();

    let header = col()
        .w_full()
        .min_w(0.0)
        .gap(4.0)
        .child(
            text(s.tr(TextKey::BackupTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        )
        .child(
            text(s.tr(TextKey::BackupSub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );

    let top_actions = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(8.0, 10.0)
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button_guarded(
                    if s.restore_point_inflight {
                        s.tr(TextKey::BusyWorking)
                    } else {
                        s.tr(TextKey::CreateRestorePointBtn)
                    },
                    p.accent,
                    Color::WHITE,
                    !s.restore_point_inflight,
                    p,
                    |s| s.create_restore_point(),
                ))
                .child(action_button(
                    s.tr(TextKey::RefreshRestorePointsBtn),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.refresh_restore_points(),
                )),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button(
                    s.tr(TextKey::ExportProfileBtn),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.export_profile(),
                ))
                .child(action_button(
                    s.tr(TextKey::ImportProfileBtn),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.import_profile(),
                )),
        );

    let mut rp_col = col().gap(6.0).w_full().min_w(0.0);
    if s.restore_points.is_empty() {
        rp_col = rp_col.child(
            text(s.tr(TextKey::NoRestorePoints))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );
    } else {
        for rp in &s.restore_points {
            let row_el = row()
                .w_full()
                .min_w(0.0)
                .padding(8.0)
                .radius(6.0)
                .fill(p.bg_primary)
                .border(1.0, p.border)
                .flex_wrap()
                .items_center()
                .gaps(6.0, 10.0)
                .child(
                    text(format!("#{}", rp.sequence_number))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted),
                )
                .child(
                    text(&rp.description)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_primary)
                        .grow(1.0)
                        .min_w(140.0)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                )
                .child(
                    text(&rp.creation_time)
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                );

            rp_col = rp_col.child(row_el);
        }
    }

    let rp_card = col()
        .w_full()
        .min_w(0.0)
        .padding(14.0)
        .radius(8.0)
        .fill(p.bg_card)
        .border(1.0, p.border)
        .gap(10.0)
        .child(
            text(s.tr(TextKey::RestorePointsTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.accent)
                .shrink()
                .wrap(),
        )
        .child(rp_col);

    let active_count = s.ledger_history.iter().filter(|e| !e.rolled_back).count();
    let ledger_header = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(6.0, 10.0)
        .child(
            text(s.tr(TextKey::SqlLedgerTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.accent)
                .grow(1.0)
                .min_w(160.0)
                .shrink()
                .wrap(),
        )
        .child(
            text(format!("{} {}", active_count, s.tr(TextKey::ActiveChanges)))
                .font(rune::tok::type_::LABEL)
                .color(p.success)
                .shrink()
                .wrap(),
        );

    let mut ledger_col = col().gap(6.0).w_full().min_w(0.0);
    if s.ledger_history.is_empty() {
        ledger_col = ledger_col.child(
            text(s.tr(TextKey::ChangelogEmpty))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );
    } else {
        for entry in &s.ledger_history {
            let entry_id = entry.id;
            let is_rolled_back = entry.rolled_back;
            let ts = entry.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();
            let display_title = s.tweak_title(&entry.tweak_id, &entry.tweak_id);
            let action_label = crate::core::status_i18n::localize_ledger_action(s.language, &entry.action_type);
            let orig_val = crate::core::tweak_i18n::localize_ledger_token(s.language, &entry.original_value);
            let new_val = crate::core::tweak_i18n::localize_ledger_token(s.language, &entry.new_value);

            let row_item = row()
                .w_full()
                .min_w(0.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .flex_wrap()
                .items_center()
                .gaps(8.0, 12.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(180.0)
                        .gap(2.0)
                        .child(
                            text(display_title)
                                .font(rune::tok::type_::SECTION)
                                .color(p.text_primary)
                                .shrink()
                                .wrap_anywhere(),
                        )
                        .child(
                            row()
                                .flex_wrap()
                                .gap(10.0)
                                .child(
                                    text(&action_label)
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_muted)
                                        .shrink()
                                        .wrap_anywhere(),
                                )
                                .child(
                                    text(ts)
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_muted),
                                ),
                        ),
                )
                .child(
                    col()
                        .shrink()
                        .min_w(105.0)
                        .gap(2.0)
                        .child(
                            text(format!("{} → {}", orig_val, new_val))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        )
                        .child(
                            text(if is_rolled_back {
                                s.tr(TextKey::RolledBackBadge)
                            } else {
                                s.tr(TextKey::AppliedBadge)
                            })
                            .font(rune::tok::type_::LABEL)
                            .color(if is_rolled_back { p.text_muted } else { p.success })
                            .shrink()
                            .wrap(),
                        ),
                )
                .child(if is_rolled_back {
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(8.0)
                        .py(5.0)
                        .radius(4.0)
                        .fill(p.btn_secondary)
                        .child(
                            text(s.tr(TextKey::RolledBackBadge))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                } else if !entry.is_reversible() {
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(8.0)
                        .py(5.0)
                        .radius(4.0)
                        .fill(p.btn_secondary)
                        .child(
                            text(s.tr(TextKey::NonReversibleBadge))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                } else {
                    action_button_guarded(
                        if s.is_rolling_back {
                            s.tr(TextKey::BusyWorking)
                        } else {
                            s.tr(TextKey::UndoBtn)
                        },
                        p.warning.alpha(0.16),
                        p.warning,
                        !s.is_rolling_back && !s.is_applying && s.in_flight_tweaks.is_empty(),
                        p,
                        move |s| s.rollback_entry(entry_id),
                    )
                });

            ledger_col = ledger_col.child(row_item);
        }
    }

    let unattend_card = {
        let presets_row = row()
            .w_full()
            .min_w(0.0)
            .flex_wrap()
            .items_center()
            .gaps(6.0, 6.0)
            .child(
                text(s.tr(TextKey::UnattendPresetsLabel))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            )
            .child(action_button(
                s.tr(TextKey::UnattendPresetMinimal),
                p.btn_secondary,
                p.text_primary,
                p,
                |s| s.apply_unattend_preset("minimal"),
            ))
            .child(action_button(
                s.tr(TextKey::UnattendPresetPrivacy),
                p.btn_secondary,
                p.text_primary,
                p,
                |s| s.apply_unattend_preset("privacy"),
            ))
            .child(action_button(
                s.tr(TextKey::UnattendPresetGaming),
                p.btn_secondary,
                p.text_primary,
                p,
                |s| s.apply_unattend_preset("gaming"),
            ))
            .child(action_button(
                format!(
                    "{} {}",
                    s.tr(TextKey::UnattendArchToggle),
                    s.unattend_opts.architecture.as_str()
                ),
                p.btn_secondary,
                p.accent,
                p,
                |s| {
                    use crate::tools::autounattend::TargetArch;
                    s.unattend_opts.architecture = match s.unattend_opts.architecture {
                        TargetArch::Amd64 => TargetArch::Arm64,
                        TargetArch::Arm64 => TargetArch::Amd64,
                    };
                },
            ));

        let mut opts_row = row()
            .w_full()
            .min_w(0.0)
            .flex_wrap()
            .gaps(6.0, 6.0)
            .child(action_button(
                format!(
                    "{} {}",
                    if s.unattend_opts.bypass_tpm_ram_requirements { "☑" } else { "☐" },
                    s.tr(TextKey::UnattendBypassTpm)
                ),
                if s.unattend_opts.bypass_tpm_ram_requirements { p.accent } else { p.btn_secondary },
                if s.unattend_opts.bypass_tpm_ram_requirements { Color::WHITE } else { p.text_primary },
                p,
                |s| s.unattend_opts.bypass_tpm_ram_requirements = !s.unattend_opts.bypass_tpm_ram_requirements,
            ))
            .child(action_button(
                format!(
                    "{} {}",
                    if s.unattend_opts.bypass_msa_oobe { "☑" } else { "☐" },
                    s.tr(TextKey::UnattendBypassMsa)
                ),
                if s.unattend_opts.bypass_msa_oobe { p.accent } else { p.btn_secondary },
                if s.unattend_opts.bypass_msa_oobe { Color::WHITE } else { p.text_primary },
                p,
                |s| s.unattend_opts.bypass_msa_oobe = !s.unattend_opts.bypass_msa_oobe,
            ))
            .child(action_button(
                format!(
                    "{} {}",
                    if s.unattend_opts.skip_oobe_privacy_screens { "☑" } else { "☐" },
                    s.tr(TextKey::UnattendSkipPrivacy)
                ),
                if s.unattend_opts.skip_oobe_privacy_screens { p.accent } else { p.btn_secondary },
                if s.unattend_opts.skip_oobe_privacy_screens { Color::WHITE } else { p.text_primary },
                p,
                |s| s.unattend_opts.skip_oobe_privacy_screens = !s.unattend_opts.skip_oobe_privacy_screens,
            ))
            .child(action_button(
                format!(
                    "{} {}",
                    if s.unattend_opts.include_selected_tweaks { "☑" } else { "☐" },
                    s.tr(TextKey::UnattendIncludeTweaks)
                ),
                if s.unattend_opts.include_selected_tweaks { p.accent } else { p.btn_secondary },
                if s.unattend_opts.include_selected_tweaks { Color::WHITE } else { p.text_primary },
                p,
                |s| s.unattend_opts.include_selected_tweaks = !s.unattend_opts.include_selected_tweaks,
            ))
            .child(action_button(
                format!(
                    "{} {}",
                    if s.unattend_opts.auto_timezone_utc { "☑" } else { "☐" },
                    s.tr(TextKey::UnattendAutoTimezone)
                ),
                if s.unattend_opts.auto_timezone_utc { p.accent } else { p.btn_secondary },
                if s.unattend_opts.auto_timezone_utc { Color::WHITE } else { p.text_primary },
                p,
                |s| s.unattend_opts.auto_timezone_utc = !s.unattend_opts.auto_timezone_utc,
            ));

        opts_row = opts_row.child(action_button(
            s.tr(TextKey::UnattendExportBtn),
            p.success,
            Color::WHITE,
            p,
            |s| s.export_autounattend_xml(),
        ));

        let mut u_col = col()
            .w_full()
            .min_w(0.0)
            .padding(14.0)
            .radius(8.0)
            .fill(p.bg_card)
            .border(1.0, p.border)
            .gap(8.0)
            .child(
                text(s.tr(TextKey::UnattendTitle))
                    .font(rune::tok::type_::TITLE)
                    .color(p.accent)
                    .shrink()
                    .wrap(),
            )
            .child(
                text(s.tr(TextKey::UnattendSub))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            )
            .child(presets_row)
            .child(opts_row)
            .child(
                text(format!("{} {}", s.tr(TextKey::LedgerDbLabel), s.ledger_path.display()))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap()
                    .wrap_anywhere(),
            );

        if let Some(ref pth) = s.unattend_last_export {
            u_col = u_col.child(
                text(format!("{} {}", s.tr(TextKey::ExportedLabel), pth))
                    .font(rune::tok::type_::LABEL)
                    .color(p.success)
                    .shrink()
                    .wrap()
                    .wrap_anywhere(),
            );
        }
        u_col
    };

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    col()
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header)
        .child(top_actions)
        .child(rp_card)
        .child(unattend_card)
        .child(ledger_header)
        .child(ledger_col)
}
