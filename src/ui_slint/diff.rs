//! Diff preview model: turns pending/inspected TweakActions into localized before/after rows
//! for the Slint `DiffModal` overlay (port of the v1.6.1 Rune diff_modal widget logic).

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::risk::TweakAction;

/// Colour tone of a diff row's summary line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Success,
    Warning,
    Danger,
}

impl Tone {
    pub fn as_str(self) -> &'static str {
        match self {
            Tone::Success => "success",
            Tone::Warning => "warning",
            Tone::Danger => "danger",
        }
    }
}

struct Tones {
    success: Tone,
    warning: Tone,
    #[allow(dead_code)]
    danger: Tone,
}

const TONES: Tones = Tones { success: Tone::Success, warning: Tone::Warning, danger: Tone::Danger };

/// One rendered diff entry. Pair rows fill `left`/`right`; single rows fill `summary`.
#[derive(Debug, Clone, PartialEq)]
pub struct DiffRow {
    pub header: String,
    pub left: String,
    pub right: String,
    pub summary: String,
    pub tone: Tone,
    pub is_pair: bool,
}

fn diff_pair_card(_p: &Tones, header: String, left: String, right: String) -> DiffRow {
    DiffRow { header, left, right, summary: String::new(), tone: Tone::Success, is_pair: true }
}

fn diff_single_card(_p: &Tones, header: String, summary: String, accent: Tone) -> DiffRow {
    DiffRow { header, left: String::new(), right: String::new(), summary, tone: accent, is_pair: false }
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

/// Title and rows for the diff modal: the inspected tweak if any, else staged items, else pending actions.
pub fn diff_view(s: &AppState) -> (String, Vec<DiffRow>) {
    if let Some((ref title, ref actions)) = s.inspected_tweak {
        let items: Vec<(TweakAction, bool)> = actions.iter().map(|a| (a.clone(), false)).collect();
        (format!("{} — {}", s.tr(TextKey::DiffModalTitle), title), rows_for(&items, s))
    } else if !s.pending_items.is_empty() {
        (s.tr(TextKey::DiffModalTitle).to_string(), rows_for(&s.pending_items, s))
    } else {
        let fallback: Vec<(TweakAction, bool)> =
            s.pending_actions.iter().map(|a| (a.clone(), false)).collect();
        (s.tr(TextKey::DiffModalTitle).to_string(), rows_for(&fallback, s))
    }
}

fn rows_for(pending: &[(TweakAction, bool)], s: &AppState) -> Vec<DiffRow> {
    let p = &TONES;
    let mut items: Vec<DiffRow> = Vec::new();

    {
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
            items.push(row_card);
        }
    }

    items
}
