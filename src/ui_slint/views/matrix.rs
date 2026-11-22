//! Advanced Tweak Matrix binder (all 230 tweaks): search, category chips, sort modes, risk
//! filter chips, presets, batch actions, per-tweak switch / inspect diff.

use std::collections::BTreeMap;
use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::{AppState, MatrixQuery, MatrixSortMode, PresetKind};
use crate::core::i18n::TextKey;
use crate::core::risk::RiskLevel;
use crate::scanner::{ScanResult, TweakState};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{MainWindow, MatrixCategory, MatrixRow, MatrixStore};

pub fn risk_index(level: RiskLevel) -> i32 {
    match level {
        RiskLevel::Safe => 0,
        RiskLevel::Low => 1,
        RiskLevel::Moderate => 2,
        RiskLevel::High => 3,
        RiskLevel::Extreme => 4,
    }
}

pub fn risk_label(s: &AppState, level: RiskLevel) -> &'static str {
    s.tr(match level {
        RiskLevel::Safe => TextKey::RiskSafe,
        RiskLevel::Low => TextKey::RiskLow,
        RiskLevel::Moderate => TextKey::RiskCaution,
        RiskLevel::High => TextKey::RiskHigh,
        RiskLevel::Extreme => TextKey::RiskExtreme,
    })
}

pub fn state_meta(s: &AppState, state: &TweakState) -> (&'static str, &'static str) {
    match state {
        TweakState::AlreadyOptimized => (s.tr(TextKey::StatusOptimized), "success"),
        TweakState::NotOptimized => (s.tr(TextKey::StatusNotApplied), "danger"),
        TweakState::PartiallyOptimized => (s.tr(TextKey::StatusPartial), "warning"),
        TweakState::NotApplicable => (s.tr(TextKey::StatusNotApplicable), "muted"),
    }
}

/// The effective query: interactive edit text wins over the programmatic query (v1.6.1 rule).
pub fn effective_query(s: &AppState) -> &str {
    let edit = s.search_edit.text.trim();
    if !edit.is_empty() {
        edit
    } else {
        s.search_query.trim()
    }
}

/// True if `r` passes the category + query filter (same matcher as Select/Deselect All).
pub fn matches(s: &AppState, r: &ScanResult, q: &MatrixQuery) -> bool {
    if let Some(ref c) = s.selected_category {
        if &r.category != c {
            return false;
        }
    }
    s.tweak_matches_query(q, &r.tweak_id, &r.title, &r.category, &r.description, r.risk.stability_risk)
}

/// Filter, group by category (sorted), sort within groups and flatten into rows.
pub fn build_rows(s: &AppState) -> (Vec<MatrixRow>, usize) {
    let q = MatrixQuery::new(effective_query(s));

    let mut grouped: BTreeMap<&str, Vec<&ScanResult>> = BTreeMap::new();
    let mut filtered = 0usize;
    for r in &s.scan_results {
        if matches(s, r, &q) {
            filtered += 1;
            grouped.entry(r.category.as_str()).or_default().push(r);
        }
    }

    let mut rows = Vec::with_capacity(filtered + grouped.len());
    for (cat, mut items) in grouped {
        match s.matrix_sort_mode {
            MatrixSortMode::NameAsc => items.sort_by_key(|a| a.title.to_lowercase()),
            MatrixSortMode::NameDesc => items.sort_by_key(|a| std::cmp::Reverse(a.title.to_lowercase())),
            MatrixSortMode::Risk => items.sort_by_key(|a| risk_index(a.risk.stability_risk)),
            MatrixSortMode::Status => items.sort_by_key(|a| a.current_state.clone() as u8),
            MatrixSortMode::Default | MatrixSortMode::Category => {}
        }
        let all_selected = items.iter().all(|r| s.selected_tweaks.contains(&r.tweak_id));
        rows.push(MatrixRow {
            is_header: true,
            category_raw: cat.into(),
            category_label: s.tweak_category(cat).into(),
            category_count: items.len() as i32,
            all_selected,
            ..Default::default()
        });
        for sr in items {
            let (state_text, tone) = state_meta(s, &sr.current_state);
            let broken = s.tweak_broken_features(&sr.tweak_id, &sr.risk.broken_features).join(", ");
            rows.push(MatrixRow {
                is_header: false,
                category_raw: cat.into(),
                id: sr.tweak_id.as_str().into(),
                title: s.tweak_title(&sr.tweak_id, &sr.title).into(),
                broken: if broken.is_empty() {
                    SharedString::new()
                } else {
                    format!("{} {}", s.tr(TextKey::BreaksLabel), broken).into()
                },
                perf_gain: sr.risk.performance_gain as i32,
                priv_gain: sr.risk.privacy_gain as i32,
                risk_level: risk_index(sr.risk.stability_risk),
                risk_label: risk_label(s, sr.risk.stability_risk).into(),
                state_text: state_text.into(),
                state_tone: tone.into(),
                detail: s.scan_detail_text(sr).into(),
                active: s.is_switch_active(&sr.tweak_id),
                in_flight: s.in_flight_tweaks.contains(&sr.tweak_id),
                not_applicable: sr.current_state == TweakState::NotApplicable,
                ..Default::default()
            });
        }
    }
    (rows, filtered)
}

fn sort_label(s: &AppState) -> &'static str {
    s.tr(match s.matrix_sort_mode {
        MatrixSortMode::Default => TextKey::SortOrderLabel,
        MatrixSortMode::NameAsc => TextKey::SortNameAsc,
        MatrixSortMode::NameDesc => TextKey::SortNameDesc,
        MatrixSortMode::Category => TextKey::SortCategory,
        MatrixSortMode::Risk => TextKey::SortRisk,
        MatrixSortMode::Status => TextKey::SortStatus,
    })
}

pub fn next_sort(m: MatrixSortMode) -> MatrixSortMode {
    match m {
        MatrixSortMode::Default => MatrixSortMode::NameAsc,
        MatrixSortMode::NameAsc => MatrixSortMode::NameDesc,
        MatrixSortMode::NameDesc => MatrixSortMode::Category,
        MatrixSortMode::Category => MatrixSortMode::Risk,
        MatrixSortMode::Risk => MatrixSortMode::Status,
        MatrixSortMode::Status => MatrixSortMode::Default,
    }
}

pub struct MatrixView {
    rows: Rc<VecModel<MatrixRow>>,
    cats: Rc<VecModel<MatrixCategory>>,
}

impl MatrixView {
    pub fn new(window: &MainWindow) -> Self {
        let store = window.global::<MatrixStore>();
        let (rows, r_rc) = new_model();
        let (cats, c_rc) = new_model();
        store.set_rows(r_rc);
        store.set_categories(c_rc);
        Self { rows, cats }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<MatrixStore>();
        let c = ctx.clone();
        st.on_query_edited(move |text| {
            c.act(|s| s.search_edit.text = text.to_string());
        });
        let c = ctx.clone();
        st.on_select_filtered(move || {
            c.act(|s| {
                let q = effective_query(s).to_string();
                let cat = s.selected_category.clone();
                s.select_filtered_tweaks(&q, cat.as_deref());
            });
        });
        let c = ctx.clone();
        st.on_deselect_filtered(move || {
            c.act(|s| {
                let q = effective_query(s).to_string();
                let cat = s.selected_category.clone();
                s.deselect_filtered_tweaks(&q, cat.as_deref());
            });
        });
        let c = ctx.clone();
        st.on_apply_selected(move || {
            c.act(|s| {
                if !s.is_applying && !s.is_rolling_back {
                    s.apply_selected();
                }
            });
        });
        let c = ctx.clone();
        st.on_revert_defaults(move || {
            c.act(|s| {
                if !s.is_applying && !s.is_rolling_back && !s.selected_tweaks.is_empty() {
                    s.revert_selected();
                }
            });
        });
        let c = ctx.clone();
        st.on_revert_selected_active(move || {
            c.act(|s| {
                if s.selected_tweaks.is_empty() || s.is_applying {
                    return;
                }
                let mut ids: Vec<String> = s.selected_tweaks.iter().cloned().collect();
                ids.sort();
                for id in ids {
                    if s.is_switch_active(&id) {
                        s.handle_switch_click(id);
                    }
                }
            });
        });
        let c = ctx.clone();
        st.on_export_selected(move || {
            c.act(|s| s.export_profile());
        });
        let c = ctx.clone();
        st.on_toggle_diff_preview(move || {
            c.act(|s| {
                if s.show_diff_preview && s.inspected_tweak.is_none() {
                    s.show_diff_preview = false;
                } else {
                    s.open_diff_preview();
                }
            });
        });
        let c = ctx.clone();
        st.on_restart_explorer(move || {
            c.act(|s| s.restart_explorer_safely());
        });
        let c = ctx.clone();
        st.on_apply_preset(move |kind| {
            let preset = match kind.as_str() {
                "gaming" => PresetKind::Gaming,
                "pro_audio" => PresetKind::ProAudio,
                "laptop_battery" => PresetKind::LaptopBattery,
                other => {
                    tracing::warn!("unknown preset {other}");
                    return;
                }
            };
            c.act(|s| s.apply_preset(preset));
        });
        let c = ctx.clone();
        st.on_set_category(move |cat| {
            c.act(|s| s.selected_category = if cat.is_empty() { None } else { Some(cat.to_string()) });
        });
        let c = ctx.clone();
        st.on_clear_filters(move || {
            c.act(|s| s.clear_advanced_filters());
        });
        let c = ctx.clone();
        st.on_cycle_sort(move || {
            c.act(|s| s.matrix_sort_mode = next_sort(s.matrix_sort_mode));
        });
        let c = ctx.clone();
        st.on_set_risk_query(move |kw| {
            c.act(|s| s.search_edit.text = kw.to_string());
        });
        let c = ctx.clone();
        st.on_toggle_category(move |cat, select| {
            c.act(|s| {
                let ids: Vec<String> =
                    s.scan_results.iter().filter(|r| r.category == cat.as_str()).map(|r| r.tweak_id.clone()).collect();
                s.toggle_category_selection(&ids, select);
            });
        });
        let c = ctx.clone();
        st.on_switch_clicked(move |id| {
            c.act(|s| {
                if !s.in_flight_tweaks.contains(id.as_str()) {
                    s.handle_switch_click(id.to_string());
                }
            });
        });
        let c = ctx.clone();
        st.on_inspect(move |id| {
            c.act(|s| s.inspect_tweak(id.as_str()));
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<MatrixStore>();
        if st.get_query().as_str() != s.search_edit.text {
            st.set_query(s.search_edit.text.as_str().into());
        }
        let sel_cat: SharedString = s.selected_category.clone().unwrap_or_default().into();
        st.set_selected_category(sel_cat);

        let mut cats: Vec<&str> = s.scan_results.iter().map(|r| r.category.as_str()).collect();
        cats.sort_unstable();
        cats.dedup();
        sync_vec(
            &self.cats,
            cats.into_iter()
                .map(|c| MatrixCategory {
                    raw: c.into(),
                    label: s.tweak_category(c).into(),
                    active: s.selected_category.as_deref() == Some(c),
                })
                .collect(),
        );

        let (rows, filtered) = build_rows(s);
        sync_vec(&self.rows, rows);
        st.set_filtered_count(filtered as i32);
        st.set_selected_count(s.selected_tweaks.len() as i32);
        st.set_summary(
            format!(
                "{} {}  ·  {} {}",
                filtered,
                s.tr(TextKey::TweaksAvailable),
                s.selected_tweaks.len(),
                s.tr(TextKey::SelectedTotal)
            )
            .into(),
        );
        st.set_sort_label(format!("↕ {}", sort_label(s)).into());
        st.set_has_filter(!effective_query(s).is_empty() || s.selected_category.is_some());
        st.set_is_scanning(s.is_scanning);
        st.set_is_applying(s.is_applying);
        st.set_busy(s.is_applying || s.is_rolling_back);
    }
}
