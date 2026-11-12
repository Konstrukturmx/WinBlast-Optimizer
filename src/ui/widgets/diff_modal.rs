//! Responsive, localized Diff Preview Modal showing pending mutations with dynamic theme support
//! and a Close button for smooth overlay dismissal.

use rune::{col, row, text, Color, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::risk::TweakAction;
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::action_button;

fn diff_pair_card(p: &ThemePalette, header: String, left: String, right: String) -> El {
    col()
        .w_full()
        .min_w(0.0)
        .padding(8.0)
        .radius(6.0)
        .fill(p.bg_primary)
        .border(1.0, p.border)
        .gap(4.0)
        .child(
            text(header)
                .font(rune::tok::type_::SECTION)
                .color(p.text_primary)
                .shrink()
                .wrap()
                .wrap_anywhere(),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gap(10.0)
                .child(
                    text(left)
                        .color(p.danger)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                )
                .child(text("➔").color(p.text_muted))
                .child(
                    text(right)
                        .color(p.success)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                ),
        )
}

fn diff_single_card(p: &ThemePalette, header: String, summary: String, accent: Color) -> El {
    col()
        .w_full()
        .min_w(0.0)
        .padding(8.0)
        .radius(6.0)
        .fill(p.bg_primary)
        .border(1.0, p.border)
        .gap(4.0)
        .child(
            text(header)
                .font(rune::tok::type_::SECTION)
                .color(p.text_primary)
                .shrink()
                .wrap()
                .wrap_anywhere(),
        )
        .child(text(summary).color(accent).shrink().wrap().wrap_anywhere())
}

fn header_with_dir(s: &AppState, base: String, is_revert: bool) -> String {
    if is_revert {
        format!("[{}] {}", s.tr(TextKey::RevertLabel), base)
    } else {
        base
    }
}

fn directed_pair(
    s: &AppState,
    default_label: TextKey,
    default_val: impl std::fmt::Display,
    target_label: TextKey,
    target_val: impl std::fmt::Display,
    is_revert: bool,
) -> (String, String) {
    let def_str = format!("{} {}", s.tr(default_label), default_val);
    let tgt_str = format!("{} {}", s.tr(target_label), target_val);
    if is_revert {
        (tgt_str, def_str)
    } else {
        (def_str, tgt_str)
    }
}

/// Renders a localized, scrollable modal preview of staged items or a single inspected tweak.
pub fn view_state(s: &AppState, p: &ThemePalette) -> El {
    if let Some((ref title, ref actions)) = s.inspected_tweak {
        let items: Vec<(TweakAction, bool)> = actions.iter().map(|a| (a.clone(), false)).collect();
        let modal_title = format!("{} — {}", s.tr(TextKey::DiffModalTitle), title);
        view_items_with_palette(&modal_title, &items, s, p)
    } else if !s.pending_items.is_empty() {
        view_items_with_palette(s.tr(TextKey::DiffModalTitle), &s.pending_items, s, p)
    } else {
        let fallback: Vec<(TweakAction, bool)> =
            s.pending_actions.iter().map(|a| (a.clone(), false)).collect();
        view_items_with_palette(s.tr(TextKey::DiffModalTitle), &fallback, s, p)
    }
}

/// Legacy slice preview wrapper (assumes forward Apply direction).
#[allow(dead_code)]
pub fn view_with_palette(actions: &[TweakAction], s: &AppState, p: &ThemePalette) -> El {
    let items: Vec<(TweakAction, bool)> = actions.iter().map(|a| (a.clone(), false)).collect();
    view_items_with_palette(s.tr(TextKey::DiffModalTitle), &items, s, p)
}

/// Renders a localized, scrollable modal preview of pending system mutations with Apply/Revert direction.
pub fn view_items_with_palette(
    modal_title: &str,
    pending: &[(TweakAction, bool)],
    s: &AppState,
    p: &ThemePalette,
) -> El {
    let mut items = col().gap(8.0).w_full().min_w(0.0);

    if pending.is_empty() {
        items = items.child(
            text(s.tr(TextKey::DiffModalEmpty))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );
    } else {
        for (action, is_revert) in pending {
            let is_revert = *is_revert;
            let row_card = match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    default_value,
                    optimized_value,
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        default_value,
                        TextKey::DiffTargetLabel,
                        optimized_value,
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!("{}: {}\\{}\\{}", s.tr(TextKey::DiffActionRegDword), hive, path, name),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    default_value,
                    optimized_value,
                    ..
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        default_value,
                        TextKey::DiffTargetLabel,
                        optimized_value,
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!(
                                "{}: {}\\{}\\{}",
                                s.tr(TextKey::DiffActionRegSz),
                                hive,
                                path,
                                if name.is_empty() {
                                    s.tr(TextKey::DiffDefaultValName)
                                } else {
                                    name
                                }
                            ),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::RegistryDelete { hive, path, name } => diff_single_card(
                    p,
                    header_with_dir(
                        s,
                        format!(
                            "{}: {}\\{}\\{}",
                            s.tr(TextKey::DiffActionRegDelete),
                            hive,
                            path,
                            if name.is_empty() {
                                s.tr(TextKey::DiffDefaultValName)
                            } else {
                                name
                            }
                        ),
                        is_revert,
                    ),
                    s.tr(TextKey::DiffActionRemoveVal).to_string(),
                    p.warning,
                ),
                TweakAction::RegistryDeleteTree { hive, path } => diff_single_card(
                    p,
                    header_with_dir(
                        s,
                        format!("{}: {}\\{}", s.tr(TextKey::DiffActionRegTreeDelete), hive, path),
                        is_revert,
                    ),
                    s.tr(TextKey::DiffActionRemoveTree).to_string(),
                    p.warning,
                ),
                TweakAction::RegistryBinaryBit {
                    hive,
                    path,
                    name,
                    byte_index,
                    bit_mask,
                    default_bits,
                    optimized_bits,
                    ..
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        format!("0x{:02X}", default_bits & bit_mask),
                        TextKey::DiffTargetLabel,
                        format!("0x{:02X}", optimized_bits & bit_mask),
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!(
                                "{}[{}] (0x{:02X}): {}\\{}\\{}",
                                s.tr(TextKey::DiffActionRegBinBit),
                                byte_index,
                                bit_mask,
                                hive,
                                path,
                                name
                            ),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::CompositeStringKey {
                    hive,
                    path,
                    name,
                    entry_key,
                    default_entry_value,
                    optimized_entry_value,
                    ..
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        default_entry_value,
                        TextKey::DiffTargetLabel,
                        optimized_entry_value,
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!(
                                "{}: {}\\{}\\{} [{}]",
                                s.tr(TextKey::DiffActionRegComposite),
                                hive,
                                path,
                                name,
                                entry_key
                            ),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::RegistryDynamicDword {
                    hive,
                    path,
                    name,
                    formula,
                    default_value,
                    ..
                } => {
                    let target = crate::scanner::resolve_dynamic_dword(&*s.backend, *formula);
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        default_value,
                        TextKey::DiffTargetLabel,
                        target,
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!(
                                "{}: {}\\{}\\{}",
                                s.tr(TextKey::DiffActionRegDynamic),
                                hive,
                                path,
                                name
                            ),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::PowerCfgSetting {
                    subgroup_guid,
                    setting_guid,
                    default_ac,
                    default_dc,
                    optimized_ac,
                    optimized_dc,
                    ..
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffDefaultLabel,
                        format!("AC/DC {}/{}", default_ac, default_dc),
                        TextKey::DiffTargetLabel,
                        format!("AC/DC {}/{}", optimized_ac, optimized_dc),
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!(
                                "{}: {}\\{}",
                                s.tr(TextKey::DiffActionPowerCfg),
                                subgroup_guid,
                                setting_guid
                            ),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::ServiceState {
                    service_name,
                    default_start_type,
                    target_start_type,
                } => {
                    let (left, right) = directed_pair(
                        s,
                        TextKey::DiffStartLabel,
                        default_start_type,
                        TextKey::DiffTargetLabel,
                        target_start_type,
                        is_revert,
                    );
                    diff_pair_card(
                        p,
                        header_with_dir(
                            s,
                            format!("{}: {}", s.tr(TextKey::DiffActionService), service_name),
                            is_revert,
                        ),
                        left,
                        right,
                    )
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    let effective_enable = if is_revert { !*enable } else { *enable };
                    diff_single_card(
                        p,
                        header_with_dir(
                            s,
                            format!("{}: {}", s.tr(TextKey::DiffActionTask), task_path),
                            is_revert,
                        ),
                        if effective_enable {
                            s.tr(TextKey::DiffStateEnable)
                        } else {
                            s.tr(TextKey::DiffStateDisable)
                        }
                        .to_string(),
                        if effective_enable { p.success } else { p.warning },
                    )
                }
                TweakAction::AppXPackage { package_family } => diff_single_card(
                    p,
                    header_with_dir(
                        s,
                        format!("{}: {}", s.tr(TextKey::DiffActionAppx), package_family),
                        is_revert,
                    ),
                    s.tr(TextKey::DiffActionRemovePkg).to_string(),
                    p.danger,
                ),
                TweakAction::WindowsFeature {
                    feature_name,
                    kind: _,
                    enable,
                } => {
                    let effective_enable = if is_revert { !*enable } else { *enable };
                    diff_single_card(
                        p,
                        header_with_dir(
                            s,
                            format!("{}: {}", s.tr(TextKey::DiffActionWinFeature), feature_name),
                            is_revert,
                        ),
                        if effective_enable {
                            s.tr(TextKey::DiffActionEnable)
                        } else {
                            s.tr(TextKey::DiffActionDisable)
                        }
                        .to_string(),
                        if effective_enable { p.success } else { p.warning },
                    )
                }
            };
            items = items.child(row_card);
        }
    }

    col()
        .w_full()
        .max_w(720.0)
        .h_full()
        .max_h(420.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(14.0)
        .radius(10.0)
        .fill(p.bg_card)
        .border(1.5, p.border_accent)
        .gap(10.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .center_y()
                .gap(8.0)
                .child(
                    text(modal_title)
                        .font(rune::tok::type_::TITLE)
                        .color(p.accent)
                        .grow(1.0)
                        .min_w(0.0)
                        .shrink()
                        .wrap(),
                )
                .child(action_button(
                    s.tr(TextKey::CloseBtn),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| {
                        s.show_diff_preview = false;
                        s.inspected_tweak = None;
                    },
                )),
        )
        .child(
            items
                .grow(1.0)
                .min_h(0.0)
                .scroll(ScrollSpec { stick_bottom: false }),
        )
}
