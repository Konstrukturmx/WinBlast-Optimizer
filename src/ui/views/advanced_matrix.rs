//! Advanced Tweak Matrix in Rune — live search input, category pills, multi-axis risk metrics,
//! and responsive wrapping row geometry inside one outer scroll container.

use std::collections::{BTreeMap, BTreeSet};
use rune::color::Color;
use rune::{col, el, row, text, text_input, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::{I18n, TextKey};
use crate::scanner::{ScanResult, TweakState};
use crate::ui::widgets::button::{
    action_button, action_button_guarded, pill_badge, switch_toggle, tweak_state_meta,
};
use crate::ui::widgets::risk_badge;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

/// Renders the categorized matrix of all tweaks with live search & category filter bar.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let cats: Vec<&str> = s
        .scan_results
        .iter()
        .map(|r| r.category.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    // Combine programmatic `search_query` and interactive `search_edit` smoothly (Turkish-safe case folding).
    // Fold once outside the loop and skip per-tweak folding entirely whenever the query is empty.
    let q_raw = if !s.search_edit.text.trim().is_empty() {
        s.search_edit.text.trim()
    } else {
        s.search_query.trim()
    };
    let q = if q_raw.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q_raw)
    };
    let has_query = !q.is_empty();

    let mut filtered_count = 0usize;
    let mut grouped: BTreeMap<&str, Vec<&ScanResult>> = BTreeMap::new();
    for r in &s.scan_results {
        if let Some(ref c) = s.selected_category {
            if &r.category != c {
                continue;
            }
        }
        if has_query {
            let loc_title = s.tweak_title(&r.tweak_id, &r.title);
            let loc_cat = s.tweak_category(&r.category);
            let loc_desc = s.tweak_description(&r.tweak_id, &r.description);
            if !I18n::fold_search_str(&r.title).contains(&q)
                && !I18n::fold_search_str(&r.tweak_id).contains(&q)
                && !I18n::fold_search_str(&r.category).contains(&q)
                && !I18n::fold_search_str(&r.description).contains(&q)
                && !I18n::fold_search_str(loc_title).contains(&q)
                && !I18n::fold_search_str(loc_cat).contains(&q)
                && !I18n::fold_search_str(loc_desc).contains(&q)
            {
                continue;
            }
        }
        filtered_count += 1;
        grouped.entry(r.category.as_str()).or_default().push(r);
    }

    let header_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .motion(p.motion)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(10.0, 12.0)
        .child(
            col()
                .grow(1.0)
                .min_w(180.0)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::AdvancedMatrix))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!(
                        "{} {}  ·  {} {}",
                        filtered_count,
                        s.tr(TextKey::TweaksAvailable),
                        s.selected_tweaks.len(),
                        s.tr(TextKey::SelectedTotal)
                    ))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
                ),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button(
                    s.tr(TextKey::SelectAll),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.select_all(),
                ))
                .child(action_button(
                    s.tr(TextKey::DeselectAll),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.deselect_all(),
                ))
                .child(action_button_guarded(
                    if s.is_scanning {
                        s.tr(TextKey::Scanning)
                    } else {
                        s.tr(TextKey::ScanSystem)
                    },
                    p.btn_secondary,
                    p.text_primary,
                    !s.is_scanning,
                    p,
                    |s| s.start_scan(),
                ))
                .child(action_button_guarded(
                    if s.is_applying {
                        s.tr(TextKey::BusyWorking).to_string()
                    } else if s.selected_tweaks.is_empty() {
                        s.tr(TextKey::ApplySelected).to_string()
                    } else {
                        format!("{} ({})", s.tr(TextKey::ApplySelected), s.selected_tweaks.len())
                    },
                    p.success,
                    Color::WHITE,
                    !s.is_applying && !s.is_rolling_back,
                    p,
                    |s| s.apply_selected(),
                ))
                .child(action_button(
                    s.tr(TextKey::PreviewDiff),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| {
                        if s.show_diff_preview && s.inspected_tweak.is_none() {
                            s.show_diff_preview = false;
                        } else {
                            s.open_diff_preview();
                        }
                    },
                ))
                .child(action_button(
                    s.tr(TextKey::RestartExplorerBtn),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.restart_explorer_safely(),
                )),
        );

    // 1-Click Optimization Presets Card
    let presets_card = row()
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
                .gap(6.0)
                .center()
                .child(text("🎯").font(rune::tok::type_::LABEL))
                .child(
                    text("1-Tık Hızlı Optimizasyon Profilleri")
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                ),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button(
                    s.tr(TextKey::GamingPreset),
                    p.accent,
                    Color::WHITE,
                    p,
                    |s| s.apply_preset(crate::app::PresetKind::Gaming),
                ))
                .child(action_button(
                    s.tr(TextKey::ProAudioPreset),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.apply_preset(crate::app::PresetKind::ProAudio),
                ))
                .child(action_button(
                    s.tr(TextKey::LaptopBatteryPreset),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.apply_preset(crate::app::PresetKind::LaptopBattery),
                )),
        );

    // Search + Category Filter Bar
    let search_box = row()
        .w_full()
        .min_w(0.0)
        .min_h(34.0)
        .px(10.0)
        .py(6.0)
        .radius(p.input_radius)
        .fill(p.bg_primary)
        .border(p.border_width, p.border)
        .items_center()
        .child(
            text_input(&s.search_edit, rune::tok::type_::BODY)
                .key("matrix_search_input")
                .single_line()
                .placeholder(s.tr(TextKey::SearchPlaceholder), p.text_muted)
                .color(p.text_primary)
                .on_edit(|s: &mut AppState| &mut s.search_edit)
                .grow(1.0)
                .min_w(0.0),
        );

    let mut filter_chips = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .gaps(6.0, 6.0)
        .child(action_button(
            s.tr(TextKey::CategoryAll),
            if s.selected_category.is_none() {
                p.accent
            } else {
                p.btn_secondary
            },
            if s.selected_category.is_none() {
                Color::WHITE
            } else {
                p.text_primary
            },
            p,
            |s| s.selected_category = None,
        ));

    for cat in &cats {
        let cat_name = (*cat).to_string();
        let active = s.selected_category.as_deref() == Some(*cat);
        let loc_cat = s.tweak_category(cat);
        filter_chips = filter_chips.child(action_button(
            loc_cat.to_string(),
            if active { p.accent } else { p.btn_secondary },
            if active { Color::WHITE } else { p.text_primary },
            p,
            move |s| s.selected_category = Some(cat_name.clone()),
        ));
    }

    if has_query || s.selected_category.is_some() {
        filter_chips = filter_chips.child(action_button(
            s.tr(TextKey::ClearFiltersBtn),
            p.btn_secondary,
            p.text_secondary,
            p,
            |s| s.clear_advanced_filters(),
        ));
    }

    let sort_label = match s.matrix_sort_mode {
        crate::app::MatrixSortMode::Default => s.tr(TextKey::SortOrderLabel),
        crate::app::MatrixSortMode::NameAsc => s.tr(TextKey::SortNameAsc),
        crate::app::MatrixSortMode::NameDesc => s.tr(TextKey::SortNameDesc),
        crate::app::MatrixSortMode::Category => s.tr(TextKey::SortCategory),
        crate::app::MatrixSortMode::Risk => s.tr(TextKey::SortRisk),
        crate::app::MatrixSortMode::Status => s.tr(TextKey::SortStatus),
    };
    filter_chips = filter_chips.child(action_button(
        format!("↕ {}", sort_label),
        p.btn_secondary,
        p.accent,
        p,
        |s| {
            s.matrix_sort_mode = match s.matrix_sort_mode {
                crate::app::MatrixSortMode::Default => crate::app::MatrixSortMode::NameAsc,
                crate::app::MatrixSortMode::NameAsc => crate::app::MatrixSortMode::NameDesc,
                crate::app::MatrixSortMode::NameDesc => crate::app::MatrixSortMode::Category,
                crate::app::MatrixSortMode::Category => crate::app::MatrixSortMode::Risk,
                crate::app::MatrixSortMode::Risk => crate::app::MatrixSortMode::Status,
                crate::app::MatrixSortMode::Status => crate::app::MatrixSortMode::Default,
            };
        },
    ));

    let filter_card = col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(8.0)
        .child(search_box)
        .child(filter_chips)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gap(6.0)
                .items_center()
                .child(
                    text(format!("{}:", s.tr(TextKey::SortRisk)))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink(),
                )
                .child(action_button(s.tr(TextKey::RiskSafe), p.success.alpha(0.18), p.success, p, |s| {
                    s.search_edit.text = "safe".to_string();
                }))
                .child(action_button(s.tr(TextKey::RiskLow), p.accent.alpha(0.18), p.accent, p, |s| {
                    s.search_edit.text = "low".to_string();
                }))
                .child(action_button(s.tr(TextKey::RiskCaution), p.warning.alpha(0.20), p.warning, p, |s| {
                    s.search_edit.text = "moderate".to_string();
                }))
                .child(action_button(s.tr(TextKey::RiskHigh), p.warning.alpha(0.28), p.warning, p, |s| {
                    s.search_edit.text = "high".to_string();
                }))
                .child(action_button(s.tr(TextKey::RiskExtreme), p.danger.alpha(0.28), p.danger, p, |s| {
                    s.search_edit.text = "extreme".to_string();
                })),
        );

    // Batch Action Toolbar
    let batch_toolbar = row()
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
                .gap(6.0)
                .items_center()
                .child(text("⚡").font(rune::tok::type_::LABEL))
                .child(
                    text(format!("Toplu İşlemler ({} Seçili)", s.selected_tweaks.len()))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                ),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(6.0, 6.0)
                .child(action_button_guarded(
                    if s.is_applying {
                        s.tr(TextKey::BusyWorking).to_string()
                    } else if s.selected_tweaks.is_empty() {
                        s.tr(TextKey::ApplySelected).to_string()
                    } else {
                        format!("{} ({})", s.tr(TextKey::ApplySelected), s.selected_tweaks.len())
                    },
                    p.success,
                    Color::WHITE,
                    !s.is_applying && !s.is_rolling_back && !s.selected_tweaks.is_empty(),
                    p,
                    |s| s.apply_selected(),
                ))
                .child(action_button_guarded(
                    s.tr(TextKey::RevertSelected),
                    p.warning.alpha(0.20),
                    p.warning,
                    !s.selected_tweaks.is_empty() && !s.is_applying,
                    p,
                    |s| {
                        let to_revert: Vec<String> = s.selected_tweaks.iter().cloned().collect();
                        for id in to_revert {
                            if s.is_switch_active(&id) {
                                s.handle_switch_click(id);
                            }
                        }
                    },
                ))
                .child(action_button(
                    s.tr(TextKey::ExportSelected),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| {
                        let count = s.selected_tweaks.len();
                        s.status_message = format!("Exported {} tweaks configuration", count);
                    },
                )),
        );

    let mut matrix_col = col().w_full().min_w(0.0).gap(14.0);

    for (cat, cat_items) in &grouped {
        if cat_items.is_empty() {
            continue;
        }

        let cat_tweaks: Vec<String> = cat_items.iter().map(|r| r.tweak_id.clone()).collect();
        let cat_tweaks_copy = cat_tweaks.clone();
        let all_selected_in_cat = cat_tweaks.iter().all(|id| s.selected_tweaks.contains(id));

        let cat_header = row()
            .w_full()
            .min_w(0.0)
            .py(4.0)
            .flex_wrap()
            .justify_between()
            .items_center()
            .gap(8.0)
            .child(
                row()
                    .items_center()
                    .gap(6.0)
                    .child(
                        text("▾")
                            .font(rune::tok::type_::LABEL)
                            .color(p.accent),
                    )
                    .child(
                        text(format!("📂  {} ({})", s.tweak_category(cat), cat_items.len()))
                            .font(rune::tok::type_::TITLE)
                            .color(p.accent)
                            .shrink()
                            .wrap(),
                    ),
            )
            .child(
                action_button(
                    if all_selected_in_cat { "Deselect Category" } else { s.tr(TextKey::SelectAllCategory) },
                    p.bg_card,
                    if all_selected_in_cat { p.warning } else { p.accent },
                    p,
                    move |s| {
                        if all_selected_in_cat {
                            for id in &cat_tweaks_copy {
                                s.selected_tweaks.remove(id);
                            }
                        } else {
                            for id in &cat_tweaks_copy {
                                s.selected_tweaks.insert(id.clone());
                            }
                        }
                    },
                ),
            );

        let mut items_col = col().w_full().min_w(0.0).gap(6.0);
        let mut sorted_items = cat_items.clone();
        match s.matrix_sort_mode {
            crate::app::MatrixSortMode::NameAsc => {
                sorted_items.sort_by_key(|a| a.title.to_lowercase());
            }
            crate::app::MatrixSortMode::NameDesc => {
                sorted_items.sort_by_key(|a| std::cmp::Reverse(a.title.to_lowercase()));
            }
            crate::app::MatrixSortMode::Risk => {
                sorted_items.sort_by_key(|a| a.risk.stability_risk as u8);
            }
            crate::app::MatrixSortMode::Status => {
                sorted_items.sort_by_key(|a| a.current_state.clone() as u8);
            }
            _ => {}
        }

        for sr in &sorted_items {
            let is_active = s.is_switch_active(&sr.tweak_id);
            let in_flight = s.in_flight_tweaks.contains(&sr.tweak_id);
            let (state_text, state_color) = tweak_state_meta(&sr.current_state, s);
            let loc_title = s.tweak_title(&sr.tweak_id, &sr.title);
            let loc_detail = s.scan_detail_text(sr);

            let broken_features = s.tweak_broken_features(&sr.tweak_id, &sr.risk.broken_features);
            let broken_str = broken_features.join(", ");
            let broken_el = if !broken_str.is_empty() {
                text(format!("{} {}", s.tr(TextKey::BreaksLabel), broken_str))
                    .font(rune::tok::type_::LABEL)
                    .color(p.warning)
                    .shrink()
                    .wrap()
                    .wrap_anywhere()
            } else {
                el()
            };

            let switch = if sr.current_state == TweakState::NotApplicable {
                pill_badge(s.tr(TextKey::StatusNotApplicable), p.text_muted, p)
            } else {
                switch_toggle(is_active, in_flight, sr.tweak_id.clone(), p)
            };
            let badge = risk_badge::view_localized(&sr.risk, p, s);
            let inspect_id = sr.tweak_id.clone();

            let row_card = row()
                .w_full()
                .min_w(0.0)
                .padding(p.card_padding.min(10.0))
                .radius(p.card_radius.min(6.0))
                .fill(if is_active { p.bg_card_hover } else { p.bg_card })
                .border(
                    p.border_width,
                    if is_active { p.accent.alpha(0.35) } else { p.border },
                )
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .flex_wrap()
                .items_center()
                .gaps(8.0, 12.0)
                .child(switch)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(180.0)
                        .gap(2.0)
                        .child(
                            text(loc_title)
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        )
                        .child(broken_el),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gap(8.0)
                        .items_center()
                        .child(
                            text(format!("{} +{}", s.tr(TextKey::PerfShort), sr.risk.performance_gain))
                                .font(rune::tok::type_::LABEL)
                                .color(p.success)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(format!("{} +{}", s.tr(TextKey::PrivShort), sr.risk.privacy_gain))
                                .font(rune::tok::type_::LABEL)
                                .color(p.accent)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(badge)
                .child(
                    col()
                        .shrink()
                        .min_w(105.0)
                        .gap(2.0)
                        .child(
                            text(state_text)
                                .font(rune::tok::type_::LABEL)
                                .color(state_color)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(&loc_detail)
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        ),
                )
                .child(action_button(
                    format!("🔍 {}", s.tr(TextKey::InspectDiff)),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    move |s| s.inspect_tweak(&inspect_id),
                ));

            items_col = items_col.child(row_card);
        }

        matrix_col = matrix_col.child(
            col()
                .w_full()
                .min_w(0.0)
                .gap(6.0)
                .child(cat_header)
                .child(items_col),
        );
    }

    if filtered_count == 0 {
        matrix_col = matrix_col.child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(p.card_padding)
                .radius(p.card_radius)
                .fill(p.bg_card)
                .border(p.border_width, p.border)
                .center()
                .child(
                    text(if s.is_scanning {
                        s.tr(TextKey::ScanningEvaluating)
                    } else {
                        s.tr(TextKey::EmptyMatrixList)
                    })
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
                ),
        );
    }

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    col()
        .key("advanced_matrix_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .pr(14.0)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header_card)
        .child(presets_card)
        .child(batch_toolbar)
        .child(filter_card)
        .child(matrix_col)
        .winblast_scroll(p)
}
