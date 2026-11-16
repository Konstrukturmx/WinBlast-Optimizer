//! Clean & Apps View in Rune — Disk Cleaner (8 targets), Dynamic AppX Bloat &
//! Optional Features (DISM) + OneDrive/Edge safe controls, and WinGet App Installer.

use std::rc::Rc;

use rune::color::Color;
use rune::{col, row, text, text_input, El, GrowthSpec, ScrollSpec};

use crate::app::{AppState, CleanAppsSubTab};
use crate::core::i18n::{I18n, TextKey};
use crate::core::risk::FeatureInstallState;
use crate::tools::cleaner::CleanTargetKind;
use crate::tools::features_apps::{AppxSafetyTier, CURATED_WINGET_APPS};
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::{
    action_button, action_button_guarded, pill_badge, switch_indicator,
};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

fn sub_tab_btn(
    key: &'static str,
    label: &str,
    active: bool,
    p: &ThemePalette,
    tab: CleanAppsSubTab,
) -> El {
    row()
        .key(key)
        .min_w(0.0)
        .shrink()
        .px(10.0)
        .py(6.0)
        .radius(p.btn_radius)
        .fill(if active { p.accent } else { p.bg_primary })
        .hover(if active { p.accent_hover } else { p.bg_card_hover })
        .border(p.border_width, if active { p.accent } else { p.border })
        .motion(p.motion)
        .center()
        .on_press(move |s: &mut AppState| s.clean_sub_tab = tab)
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(if active { Color::WHITE } else { p.text_primary })
                .shrink()
                .wrap(),
        )
}

fn format_mb(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb >= 1024.0 {
        format!("{:.2} GB", mb / 1024.0)
    } else {
        format!("{:.1} MB", mb)
    }
}

fn clean_target_label(s: &AppState, kind: CleanTargetKind) -> &'static str {
    match kind {
        CleanTargetKind::UserTemp => s.tr(TextKey::CleanTempUser),
        CleanTargetKind::WindowsTemp => s.tr(TextKey::CleanTempWin),
        CleanTargetKind::WindowsUpdateCache => s.tr(TextKey::CleanWinUpdate),
        CleanTargetKind::DeliveryOptimizationCache => s.tr(TextKey::CleanDeliveryOpt),
        CleanTargetKind::ThumbnailCache => s.tr(TextKey::CleanThumbnail),
        CleanTargetKind::WerAndCrashDumps => s.tr(TextKey::CleanCrashDumps),
        CleanTargetKind::Prefetch => s.tr(TextKey::CleanPrefetch),
        CleanTargetKind::RecycleBin => s.tr(TextKey::CleanRecycleBin),
    }
}

fn empty_card(msg: &str, p: &ThemePalette) -> El {
    row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .center()
        .child(
            text(msg)
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        )
}

fn build_disk_cleaner_body(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let total_reclaimable: u64 = s.clean_targets.iter().map(|t| t.reclaimable_bytes).sum();
    let selected_reclaimable: u64 = s
        .clean_targets
        .iter()
        .filter(|t| t.selected)
        .map(|t| t.reclaimable_bytes)
        .sum();
    let last_freed = s
        .last_clean_report
        .as_ref()
        .map(|r| format_mb(r.total_freed_bytes))
        .unwrap_or_else(|| "0.0 MB".to_string());

    let toolbar = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(10.0))
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
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
                    s.tr(TextKey::ScanCleanupTargets),
                    p.btn_secondary,
                    p.text_primary,
                    !s.clean_apps_busy,
                    p,
                    |s| s.scan_clean_targets_async(),
                ))
                .child(action_button_guarded(
                    format!("{} ({})", s.tr(TextKey::CleanSelectedNow), format_mb(selected_reclaimable)),
                    p.accent,
                    Color::WHITE,
                    !s.clean_apps_busy,
                    p,
                    |s| s.execute_clean_selected(),
                )),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 10.0)
                .child(
                    text(format!("{}: {}", s.tr(TextKey::TotalReclaimable), format_mb(total_reclaimable)))
                        .font(rune::tok::type_::LABEL)
                        .color(p.accent)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{}: {}", s.tr(TextKey::LastCleaned), last_freed))
                        .font(rune::tok::type_::LABEL)
                        .color(p.success)
                        .shrink()
                        .wrap(),
                ),
        );

    let mut list_col = col().w_full().min_w(0.0).gap(6.0);
    if let Some(ref report) = s.last_clean_report {
        let errs: Vec<String> = report
            .results
            .iter()
            .filter_map(|r| {
                r.error
                    .as_ref()
                    .map(|e| format!("{}: {}", clean_target_label(s, r.kind), e))
            })
            .collect();
        if !errs.is_empty() {
            list_col = list_col.child(
                col()
                    .w_full()
                    .min_w(0.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(p.warning.alpha(0.12))
                    .border(1.0, p.warning.alpha(0.45))
                    .gap(4.0)
                    .child(
                        text(format!("{}: {}", s.tr(TextKey::CleanErrorLabel), errs.join(" · ")))
                            .font(rune::tok::type_::LABEL)
                            .color(p.warning)
                            .shrink()
                            .wrap()
                            .wrap_anywhere(),
                    ),
            );
        }
    }
    if s.clean_targets.is_empty() {
        list_col = list_col.child(empty_card(
            if s.clean_apps_busy {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyCleanTargets)
            },
            p,
        ));
    } else {
        for t in &s.clean_targets {
            let kind = t.kind;
            let path_preview = t.paths.first().cloned().unwrap_or_default();
            let row_key = format!("clean_tgt_{}", kind.id_key());
            let sw_key = format!("clean_sw_{}", kind.id_key());

            let mut info_col = col()
                .grow(1.0)
                .min_w(140.0)
                .gap(2.0)
                .child(
                    text(clean_target_label(s, kind))
                        .font(rune::tok::type_::SECTION)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(path_preview)
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                );
            if let Some(ref err) = t.last_error {
                info_col = info_col.child(
                    text(format!("{}: {}", s.tr(TextKey::CleanErrorLabel), err))
                        .font(rune::tok::type_::LABEL)
                        .color(p.warning)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                );
            }

            let mut card = row()
                .key(row_key)
                .w_full()
                .min_w(0.0)
                .padding(10.0)
                .radius(6.0)
                .fill(if t.selected { p.bg_card_hover } else { p.bg_card })
                .border(1.0, if t.selected { p.accent.alpha(0.35) } else { p.border })
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .flex_wrap()
                .items_center()
                .justify_between()
                .gaps(8.0, 12.0)
                .on_press(move |s: &mut AppState| s.toggle_clean_target(kind))
                .child(
                    row()
                        .min_w(0.0)
                        .grow(1.0)
                        .items_center()
                        .gap(10.0)
                        .child(switch_indicator(t.selected, s.clean_apps_busy, sw_key, p))
                        .child(info_col),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .items_center()
                        .gaps(6.0, 8.0)
                        .child(
                            text(format!(
                                "{} · {} {}",
                                format_mb(t.reclaimable_bytes),
                                t.file_count,
                                s.tr(TextKey::FilesUnit)
                            ))
                            .font(rune::tok::type_::LABEL)
                            .color(if t.reclaimable_bytes > 0 { p.accent } else { p.text_muted })
                            .shrink()
                            .wrap(),
                        ),
                );
            if t.requires_elevation {
                card = card.child(pill_badge(s.tr(TextKey::AdminBadge), p.warning, p));
            }
            list_col = list_col.child(card);
        }
    }

    let list_container = if tier == LayoutTier::Compact {
        list_col
    } else {
        col()
            .key("clean_targets_scroll")
            .w_full()
            .grow(1.0)
            .min_w(0.0)
            .min_h(100.0)
            .pr(14.0)
            .scroll(ScrollSpec { stick_bottom: false })
            .child(list_col)
            .winblast_scroll(p)
    };

    let mut wrapper = col()
        .w_full()
        .min_w(0.0)
        .min_h(0.0)
        .gap(10.0)
        .child(toolbar)
        .child(list_container);
    if tier != LayoutTier::Compact {
        wrapper = wrapper.grow(1.0);
    }
    wrapper
}

fn build_appx_features_body(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let q_raw = s.appx_filter_edit.text.trim();
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let filtered_appx_idxs: Rc<Vec<usize>> = Rc::new(
        s.installed_appx
            .iter()
            .enumerate()
            .filter(|(_, pkg)| {
                q.is_empty()
                    || I18n::fold_search_str(&pkg.short_name).contains(&q)
                    || I18n::fold_search_str(&pkg.family_name).contains(&q)
            })
            .map(|(i, _)| i)
            .collect(),
    );

    let mut controls = col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(10.0))
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(8.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .items_center()
                .justify_between()
                .gaps(8.0, 8.0)
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(action_button_guarded(
                            s.tr(TextKey::ScanAppsFeatures),
                            p.btn_secondary,
                            p.text_primary,
                            !s.clean_apps_busy,
                            p,
                            |s| s.scan_appx_and_features_async(),
                        ))
                        .child(action_button_guarded(
                            format!("{} ({})", s.tr(TextKey::RemoveSelectedAppx), s.selected_appx.len()),
                            p.danger,
                            Color::WHITE,
                            !s.clean_apps_busy,
                            p,
                            |s| s.remove_selected_appx_async(),
                        )),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(action_button(
                            s.tr(TextKey::SelectAll),
                            p.btn_secondary,
                            p.text_primary,
                            p,
                            |s| s.select_safe_and_ai_appx(),
                        ))
                        .child(action_button(
                            s.tr(TextKey::DeselectAllBtn),
                            p.btn_secondary,
                            p.text_secondary,
                            p,
                            |s| s.deselect_all_appx(),
                        ))
                        .child(action_button_guarded(
                            s.tr(TextKey::RemoveOneDriveSafe),
                            p.warning.alpha(0.16),
                            p.warning,
                            !s.clean_apps_busy,
                            p,
                            |s| s.uninstall_onedrive_async(),
                        ))
                        .child(action_button_guarded(
                            s.tr(TextKey::RemoveEdgeSafe),
                            p.warning.alpha(0.16),
                            p.warning,
                            !s.clean_apps_busy,
                            p,
                            |s| s.remove_edge_async(),
                        )),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .min_h(34.0)
                .px(8.0)
                .py(5.0)
                .radius(6.0)
                .fill(p.bg_primary)
                .border(1.0, p.border)
                .items_center()
                .child(
                    text_input(&s.appx_filter_edit, rune::tok::type_::BODY)
                        .key("appx_search_input")
                        .single_line()
                        .placeholder(s.tr(TextKey::SearchPlaceholder), p.text_muted)
                        .color(p.text_primary)
                        .on_edit(|s: &mut AppState| &mut s.appx_filter_edit)
                        .grow(1.0)
                        .min_w(0.0),
                ),
        );

    if let Some(ref kfm) = s.onedrive_kfm_warning {
        let kfm_loc = crate::core::status_i18n::localize_kfm_detail(s.language, kfm);
        controls = controls.child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(8.0)
                .radius(6.0)
                .fill(p.warning.alpha(0.14))
                .border(1.0, p.warning.alpha(0.5))
                .child(
                    text(format!("{} ({})", s.tr(TextKey::OneDriveKfmWarn), kfm_loc))
                        .font(rune::tok::type_::LABEL)
                        .color(p.warning)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                ),
        );
    }

    let k_idxs = Rc::clone(&filtered_appx_idxs);
    let b_idxs = Rc::clone(&filtered_appx_idxs);
    let count = filtered_appx_idxs.len();
    let row_h = if tier == LayoutTier::Compact { 104.0 } else { 72.0 };

    let appx_panel = if count == 0 {
        empty_card(
            if s.clean_apps_busy && s.installed_appx.is_empty() {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyAppxList)
            },
            p,
        )
    } else {
        let mut sc = col()
            .key("appx_scroll")
            .w_full()
            .min_w(0.0)
            .pr(14.0)
            .scroll(ScrollSpec { stick_bottom: false });
        sc = if tier == LayoutTier::Compact {
            sc.h(220.0)
        } else {
            sc.grow(1.0).min_h(120.0)
        };
        sc.child(
            col()
                .key("appx_virtual_rows")
                .w_full()
                .growth(GrowthSpec::default())
                .virtual_rows(
                    count,
                    row_h,
                    move |s: &AppState, i: usize| {
                        format!("appx_{}", s.installed_appx[k_idxs[i]].family_name).into()
                    },
                    move |s: &AppState, i: usize| {
                        let pkg = &s.installed_appx[b_idxs[i]];
                        let p = s.palette();
                        let is_core = pkg.safety_tier == AppxSafetyTier::ProtectedCore;
                        let selected = !is_core && s.selected_appx.contains(&pkg.family_name);
                        let tier_color = match pkg.safety_tier {
                            AppxSafetyTier::SafeBloat => p.success,
                            AppxSafetyTier::AiBloat => p.accent,
                            AppxSafetyTier::Caution => p.warning,
                            AppxSafetyTier::ProtectedCore => p.danger,
                        };
                        let tier_txt = match pkg.safety_tier {
                            AppxSafetyTier::SafeBloat => s.tr(TextKey::SafetySafe),
                            AppxSafetyTier::AiBloat => s.tr(TextKey::SafetyAi),
                            AppxSafetyTier::Caution => s.tr(TextKey::SafetyCaution),
                            AppxSafetyTier::ProtectedCore => s.tr(TextKey::ProtectedCoreBadge),
                        };
                        let fam = pkg.family_name.clone();
                        let sw_key = format!("appx_sw_{}", pkg.family_name);

                        let indicator = if is_core {
                            row()
                                .key(sw_key)
                                .size(36.0, 20.0)
                                .radius(10.0)
                                .fill(p.danger.alpha(0.16))
                                .border(1.0, p.danger.alpha(0.45))
                                .center()
                                .child(
                                    text("🔒")
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.danger),
                                )
                        } else {
                            switch_indicator(selected, s.clean_apps_busy, sw_key, p)
                        };

                        let mut inner_card = row()
                            .w_full()
                            .min_w(0.0)
                            .padding(8.0)
                            .radius(6.0)
                            .fill(if is_core {
                                p.bg_card.alpha(0.75)
                            } else if selected {
                                p.bg_card_hover
                            } else {
                                p.bg_card
                            })
                            .border(
                                1.0,
                                if is_core {
                                    p.danger.alpha(0.28)
                                } else if selected {
                                    p.accent.alpha(0.35)
                                } else {
                                    p.border
                                },
                            )
                            .flex_wrap()
                            .items_center()
                            .justify_between()
                            .gaps(6.0, 10.0)
                            .child(
                                row()
                                    .min_w(0.0)
                                    .grow(1.0)
                                    .items_center()
                                    .gap(8.0)
                                    .child(indicator)
                                    .child(
                                        col()
                                            .grow(1.0)
                                            .min_w(120.0)
                                            .gap(2.0)
                                            .child(
                                                text(&pkg.short_name)
                                                    .font(rune::tok::type_::SECTION)
                                                    .color(p.text_primary)
                                                    .shrink()
                                                    .truncate(),
                                            )
                                            .child(
                                                text(&pkg.family_name)
                                                    .font(rune::tok::type_::LABEL)
                                                    .color(p.text_muted)
                                                    .shrink()
                                                    .truncate(),
                                            ),
                                    ),
                            )
                            .child(pill_badge(tier_txt, tier_color, p));

                        if !is_core {
                            let fam_click = fam.clone();
                            inner_card = inner_card.on_press(move |s: &mut AppState| {
                                s.toggle_appx_selection(fam_click.clone());
                            });
                        }

                        col().w_full().min_w(0.0).pb(6.0).child(inner_card)
                    },
                ),
        )
        .winblast_scroll(p)
    };

    let mut feat_col = col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(10.0))
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(6.0)
        .child(
            text(s.tr(TextKey::OptionalFeaturesTitle))
                .font(rune::tok::type_::SECTION)
                .color(p.accent)
                .shrink()
                .wrap(),
        );

    if s.optional_features.is_empty() {
        feat_col = feat_col.child(
            text(if s.clean_apps_busy {
                s.tr(TextKey::Scanning)
            } else {
                s.tr(TextKey::EmptyFeaturesList)
            })
            .font(rune::tok::type_::BODY)
            .color(p.text_muted)
            .shrink()
            .wrap(),
        );
    } else {
        for feat in &s.optional_features {
            let enabled = matches!(
                feat.state,
                FeatureInstallState::Enabled | FeatureInstallState::EnablePending
            );
            let target_enable = !enabled;
            let f_name = feat.name.clone();
            let f_kind = feat.kind;
            let row_key = format!("feat_{}", feat.name);
            feat_col = feat_col.child(
                row()
                    .key(row_key)
                    .w_full()
                    .min_w(0.0)
                    .padding(8.0)
                    .radius(6.0)
                    .fill(p.bg_primary)
                    .border(1.0, p.border)
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .gaps(6.0, 10.0)
                    .child(
                        col()
                            .grow(1.0)
                            .min_w(160.0)
                            .gap(2.0)
                            .child({
                                let (disp_name, _) = crate::core::tweak_i18n::localize_optional_feature(
                                    s.language,
                                    &feat.name,
                                )
                                .unwrap_or((feat.display_name.as_str(), feat.description.as_str()));
                                text(disp_name)
                                    .font(rune::tok::type_::BODY)
                                    .color(p.text_primary)
                                    .shrink()
                                    .wrap()
                            })
                            .child({
                                let (_, disp_desc) = crate::core::tweak_i18n::localize_optional_feature(
                                    s.language,
                                    &feat.name,
                                )
                                .unwrap_or((feat.display_name.as_str(), feat.description.as_str()));
                                text(disp_desc)
                                    .font(rune::tok::type_::LABEL)
                                    .color(p.text_muted)
                                    .shrink()
                                    .wrap()
                            }),
                    )
                    .child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .items_center()
                            .gap(6.0)
                            .child(pill_badge(
                                if enabled {
                                    s.tr(TextKey::StateEnabled)
                                } else {
                                    s.tr(TextKey::StateDisabled)
                                },
                                if enabled { p.success } else { p.text_muted },
                                p,
                            ))
                            .child(action_button_guarded(
                                if enabled {
                                    s.tr(TextKey::ToggleDisable)
                                } else {
                                    s.tr(TextKey::ToggleEnable)
                                },
                                if enabled { p.btn_secondary } else { p.accent },
                                if enabled { p.text_primary } else { Color::WHITE },
                                !s.clean_apps_busy,
                                p,
                                move |s| s.toggle_optional_feature_async(f_name.clone(), f_kind, target_enable),
                            )),
                    ),
            );
        }
    }

    let features_panel = if tier == LayoutTier::Compact {
        feat_col
    } else {
        col()
            .key("features_scroll")
            .w_full()
            .max_h(170.0)
            .min_w(0.0)
            .pr(14.0)
            .scroll(ScrollSpec { stick_bottom: false })
            .child(feat_col)
            .winblast_scroll(p)
    };

    let mut wrapper = col()
        .w_full()
        .min_w(0.0)
        .min_h(0.0)
        .gap(8.0)
        .child(controls)
        .child(appx_panel)
        .child(features_panel);
    if tier != LayoutTier::Compact {
        wrapper = wrapper.grow(1.0);
    }
    wrapper
}

fn build_winget_body(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let q_raw = s.winget_filter_edit.text.trim();
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let filtered_apps: Vec<_> = CURATED_WINGET_APPS
        .iter()
        .filter(|app| {
            q.is_empty()
                || I18n::fold_search_str(app.name).contains(&q)
                || I18n::fold_search_str(app.id).contains(&q)
                || I18n::fold_search_str(app.category).contains(&q)
                || I18n::fold_search_str(app.description).contains(&q)
        })
        .collect();

    let toolbar = col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(10.0))
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(8.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .items_center()
                .justify_between()
                .gaps(8.0, 10.0)
                .child(
                    text(s.tr(TextKey::WinGetStatusReady))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary)
                        .grow(1.0)
                        .min_w(140.0)
                        .shrink()
                        .wrap(),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(action_button(
                            s.tr(TextKey::DeselectAllBtn),
                            p.btn_secondary,
                            p.text_secondary,
                            p,
                            |s| s.deselect_all_winget(),
                        ))
                        .child(action_button_guarded(
                            format!(
                                "{} ({})",
                                s.tr(TextKey::InstallSelectedWinGet),
                                s.selected_winget.len()
                            ),
                            p.accent,
                            Color::WHITE,
                            !s.clean_apps_busy,
                            p,
                            |s| s.install_selected_winget_async(),
                        )),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .min_h(34.0)
                .px(8.0)
                .py(5.0)
                .radius(6.0)
                .fill(p.bg_primary)
                .border(1.0, p.border)
                .items_center()
                .child(
                    text_input(&s.winget_filter_edit, rune::tok::type_::BODY)
                        .key("winget_search_input")
                        .single_line()
                        .placeholder(s.tr(TextKey::SearchPlaceholder), p.text_muted)
                        .color(p.text_primary)
                        .on_edit(|s: &mut AppState| &mut s.winget_filter_edit)
                        .grow(1.0)
                        .min_w(0.0),
                ),
        );

    let cards_container: El = if filtered_apps.is_empty() {
        empty_card(s.tr(TextKey::EmptyWingetList), p)
    } else {
        let mut cards = row().w_full().min_w(0.0).flex_wrap().gaps(8.0, 8.0);
        for app in filtered_apps {
            let sel = s.selected_winget.contains(app.id);
            let app_id = app.id.to_string();
            let row_key = format!("wg_{}", app.id);
            let sw_key = format!("wg_sw_{}", app.id);
            let app_desc = crate::core::tweak_i18n::localize_winget(
                s.language,
                app.id,
            )
            .map(|(_, d)| d)
            .unwrap_or(app.description);
            let loc_cat = crate::core::tweak_i18n::localize_winget(s.language, app.id)
                .map(|(c, _)| c)
                .unwrap_or(app.category);

            cards = cards.child(
                col()
                    .key(row_key)
                    .grow(1.0)
                    .min_w(220.0)
                    .padding(10.0)
                    .radius(6.0)
                    .fill(if sel { p.bg_card_hover } else { p.bg_card })
                    .border(1.0, if sel { p.accent.alpha(0.4) } else { p.border })
                    .hover(p.bg_card_hover)
                    .motion(p.motion)
                    .gap(6.0)
                    .on_press(move |s: &mut AppState| s.toggle_winget_selection(app_id.clone()))
                    .child(
                        row()
                            .w_full()
                            .justify_between()
                            .items_center()
                            .gap(8.0)
                            .child(
                                row()
                                    .min_w(0.0)
                                    .items_center()
                                    .gap(8.0)
                                    .child(switch_indicator(sel, s.clean_apps_busy, sw_key, p))
                                    .child(
                                        text(app.name)
                                            .font(rune::tok::type_::SECTION)
                                            .color(p.text_primary)
                                            .shrink()
                                            .wrap(),
                                    ),
                            )
                            .child(pill_badge(loc_cat, p.accent, p)),
                    )
                    .child(
                        text(app_desc)
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    ),
            );
        }

        if tier == LayoutTier::Compact {
            cards
        } else {
            col()
                .key("winget_scroll")
                .w_full()
                .grow(1.0)
                .min_w(0.0)
                .min_h(100.0)
                .pr(14.0)
                .scroll(ScrollSpec { stick_bottom: false })
                .child(cards)
                .winblast_scroll(p)
        }
    };

    let mut wrapper = col()
        .w_full()
        .min_w(0.0)
        .min_h(0.0)
        .gap(10.0)
        .child(toolbar)
        .child(cards_container);
    if tier != LayoutTier::Compact {
        wrapper = wrapper.grow(1.0);
    }
    wrapper
}

/// Renders the Clean & Apps view cleanly with 3 sub-tabs and responsive viewport geometry.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let _style = s.theme_style;
    let pad = if tier == LayoutTier::Compact { 10.0 } else { p.card_padding };

    let mut header_title_row = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .items_center()
        .justify_between()
        .gaps(6.0, 8.0)
        .child(
            text(s.tr(TextKey::CleanAppsTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        );
    if s.clean_apps_busy {
        header_title_row = header_title_row.child(pill_badge(s.tr(TextKey::BusyWorking), p.accent, p));
    }

    let header = col()
        .w_full()
        .min_w(0.0)
        .gap(6.0)
        .child(header_title_row)
        .child(
            text(s.tr(TextKey::CleanAppsSub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(6.0, 6.0)
                .child(sub_tab_btn(
                    "sub_clean_disk",
                    s.tr(TextKey::SubTabDiskCleaner),
                    s.clean_sub_tab == CleanAppsSubTab::DiskCleaner,
                    p,
                    CleanAppsSubTab::DiskCleaner,
                ))
                .child(sub_tab_btn(
                    "sub_clean_appx",
                    s.tr(TextKey::SubTabAppxFeatures),
                    s.clean_sub_tab == CleanAppsSubTab::AppxFeatures,
                    p,
                    CleanAppsSubTab::AppxFeatures,
                ))
                .child(sub_tab_btn(
                    "sub_clean_winget",
                    s.tr(TextKey::SubTabWinGet),
                    s.clean_sub_tab == CleanAppsSubTab::WinGetInstaller,
                    p,
                    CleanAppsSubTab::WinGetInstaller,
                )),
        );

    let body = match s.clean_sub_tab {
        CleanAppsSubTab::DiskCleaner => build_disk_cleaner_body(s, p, tier),
        CleanAppsSubTab::AppxFeatures => build_appx_features_body(s, p, tier),
        CleanAppsSubTab::WinGetInstaller => build_winget_body(s, p, tier),
    };

    let outer = col()
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(10.0)
        .child(header)
        .child(body);

    if tier == LayoutTier::Compact {
        outer.key("clean_apps_compact_viewport").scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
    } else {
        outer
    }
}
