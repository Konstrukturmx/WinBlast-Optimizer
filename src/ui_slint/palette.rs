//! Command palette (Ctrl+K): accent/diacritic-insensitive, ranked search over every view and a
//! few shell actions. A non-empty query also offers "search tweaks for <query>", which the
//! Slint side hands to the Advanced Matrix search.

use slint::SharedString;

use crate::core::i18n::{I18n, Language};

use super::i18n::lookup;
use super::PaletteItem;

/// (view id, icon, label key, group key). Mirrors the sidebar groups in `ui/shell/sidebar.slint`.
pub const VIEWS: [(&str, &str, &str, &str); 26] = [
    ("dashboard", "⚡", "NavSystemPanel", "NavGroupOverview"),
    ("hardware", "🖥️", "NavHardware", "NavGroupOverview"),
    ("driver_bios", "💾", "NavDriverBios", "NavGroupOverview"),
    ("performance_tests", "📈", "NavPerfTests", "NavGroupDiagnostics"),
    ("fault_diagnostics", "🛡️", "NavFaultDiag", "NavGroupDiagnostics"),
    ("latency", "⏱️", "NavLatency", "NavGroupDiagnostics"),
    ("cpu_parking", "🔲", "NavCoreParking", "NavGroupDiagnostics"),
    ("thermal", "❄️", "NavThermal", "NavGroupDiagnostics"),
    ("disk_health", "💽", "NavDisks", "NavGroupDiagnostics"),
    ("gaming_stats", "🎮", "NavGamingStats", "NavGroupDiagnostics"),
    ("fps_benchmark", "📊", "NavFpsBench", "NavGroupDiagnostics"),
    ("network_diag", "📶", "NavNetwork", "NavGroupDiagnostics"),
    ("matrix", "⚡", "NavFpsOptimization", "NavGroupOptimization"),
    ("automation", "🤖", "NavAutomationCenter", "NavGroupOptimization"),
    ("game_prereqs", "🎮", "NavGamePrereqs", "NavGroupOptimization"),
    ("goal_planner", "🎯", "NavGoalPlanner", "NavGroupOptimization"),
    ("game_profiles", "🕹️", "NavGameProfiles", "NavGroupOptimization"),
    ("game_overlay", "🖥️", "NavGameOverlay", "NavGroupOptimization"),
    ("game_suitability", "🎯", "NavGameSuitability", "NavGroupOptimization"),
    ("clean_apps", "🗑️", "NavCleanApps", "NavGroupOptimization"),
    ("startup_tasks", "⚙️", "NavStartupTasks", "NavGroupOptimization"),
    ("windows_repair", "🔧", "NavWindowsRepair", "NavGroupOptimization"),
    ("backup", "📄", "NavBackupReport", "NavGroupOptimization"),
    ("gpu", "🎛️", "NavGpuMsiHags", "NavGroupOptimization"),
    ("privacy", "🛡️", "NavPrivacyShield", "NavGroupOptimization"),
    ("support_about", "✉️", "NavSupportAbout", "NavGroupAccount"),
];

/// (action id, icon, label key). Executed by `ui/shell/command_palette.slint`.
pub const ACTIONS: [(&str, &str, &str); 4] = [
    ("scan", "🔍", "ScanSystem"),
    ("theme", "🌓", "PaletteToggleTheme"),
    ("motion", "✨", "PaletteToggleMotion"),
    ("changelog", "📜", "PaletteShowChangelog"),
];

/// Maximum number of view/action rows shown (the tweak-search row comes on top of this).
pub const MAX_RESULTS: usize = 10;

/// Strip a leading emoji/symbol prefix that some catalog labels carry; the palette has its own icon column.
fn clean_label(s: &str) -> String {
    s.trim_start_matches(|c: char| !c.is_alphanumeric()).trim().to_string()
}

/// 0 = a word starts with the query, 1 = substring of label or id, None = no match.
fn rank(query: &str, label: &str, id: &str) -> Option<u8> {
    if query.is_empty() {
        return Some(1);
    }
    let folded = I18n::fold_search_str(label);
    if folded.starts_with(query) || folded.split(|c: char| !c.is_alphanumeric()).any(|w| w.starts_with(query)) {
        return Some(0);
    }
    if folded.contains(query) || I18n::fold_search_str(&id.replace('_', " ")).contains(query) {
        return Some(1);
    }
    None
}

fn item(kind: &str, id: &str, icon: &str, label: String, hint: String) -> PaletteItem {
    PaletteItem {
        kind: kind.into(),
        id: id.into(),
        icon: icon.into(),
        label: SharedString::from(label),
        hint: SharedString::from(hint),
    }
}

/// Ranked results for `query` in `lang`.
pub fn search(lang: Language, query: &str) -> Vec<PaletteItem> {
    let raw = query.trim();
    let q = I18n::fold_search_str(raw);
    let mut scored: Vec<(u8, usize, PaletteItem)> = Vec::new();
    for (order, (id, icon, key, group)) in VIEWS.iter().enumerate() {
        let label = clean_label(&lookup(lang, key));
        if let Some(r) = rank(&q, &label, id) {
            let hint = clean_label(&lookup(lang, group));
            scored.push((r, order, item("view", id, icon, label, hint)));
        }
    }
    for (order, (id, icon, key)) in ACTIONS.iter().enumerate() {
        let label = clean_label(&lookup(lang, key));
        if let Some(r) = rank(&q, &label, id) {
            // Actions first when browsing, interleaved by rank when searching.
            let order = if q.is_empty() { order } else { VIEWS.len() + order };
            scored.push((r, order, item("action", id, icon, label, "↵".to_string())));
        }
    }
    if q.is_empty() {
        // Browsing: actions, then views in sidebar order.
        scored.sort_by_key(|(_, order, it)| (it.kind != "action", *order));
    } else {
        scored.sort_by_key(|(r, order, _)| (*r, *order));
    }
    let mut out: Vec<PaletteItem> = scored.into_iter().take(MAX_RESULTS).map(|(_, _, it)| it).collect();
    if !q.is_empty() {
        let label = format!("{} “{}”", lookup(lang, "PaletteSearchTweaks"), raw);
        let hint = clean_label(&lookup(lang, "NavFpsOptimization"));
        out.push(item("tweaks", raw, "🎛️", label, hint));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::ActiveView;
    use crate::ui_slint::i18n::is_known_key;
    use crate::ui_slint::shell::view_from_name;

    #[test]
    fn every_view_is_listed_once_and_resolves() {
        assert_eq!(VIEWS.len(), ActiveView::ALL_VIEWS.len());
        for v in ActiveView::ALL_VIEWS {
            assert_eq!(VIEWS.iter().filter(|(id, ..)| *id == v.name()).count(), 1, "view {} missing or duplicated", v.name());
        }
        for (id, _, key, group) in VIEWS {
            assert!(view_from_name(id).is_some(), "unknown view id {id}");
            assert!(is_known_key(key), "unknown label key {key}");
            assert!(is_known_key(group), "unknown group key {group}");
        }
        for (_, _, key) in ACTIONS {
            assert!(is_known_key(key), "unknown action key {key}");
        }
    }

    #[test]
    fn empty_query_lists_actions_first_without_tweak_row() {
        let r = search(Language::English, "");
        assert_eq!(r.len(), MAX_RESULTS);
        assert_eq!(r[0].kind.as_str(), "action");
        assert!(r.iter().all(|i| i.kind.as_str() != "tweaks"));
    }

    #[test]
    fn query_matches_are_ranked_and_end_with_tweak_search() {
        let r = search(Language::English, "  thermal ");
        assert_eq!(r.first().map(|i| i.id.as_str()), Some("thermal"));
        let last = r.last().expect("tweak row");
        assert_eq!(last.kind.as_str(), "tweaks");
        assert_eq!(last.id.as_str(), "thermal");
    }

    #[test]
    fn search_is_accent_insensitive_and_localized() {
        // German "Thermik"/"Thermal" style labels still match a plain ASCII query via the view id.
        let r = search(Language::German, "parking");
        assert!(r.iter().any(|i| i.id.as_str() == "cpu_parking"));
        let none = search(Language::English, "zzzz-no-such-view");
        assert_eq!(none.len(), 1, "only the tweak-search row remains");
    }
}
