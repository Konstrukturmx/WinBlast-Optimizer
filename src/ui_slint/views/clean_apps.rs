//! Clean & Apps binder: disk cleaner, AppX bloat + optional features + OneDrive/Edge, WinGet.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::{AppState, CleanAppsSubTab};
use crate::core::i18n::{I18n, TextKey};
use crate::core::risk::FeatureInstallState;
use crate::tools::cleaner::CleanTargetKind;
use crate::tools::features_apps::{AppxSafetyTier, CURATED_WINGET_APPS};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{AppxRow, CleanAppsStore, CleanTargetRow, FeatureRow, MainWindow, WingetRow};

pub fn format_mb(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb >= 1024.0 {
        format!("{:.2} GB", mb / 1024.0)
    } else {
        format!("{mb:.1} MB")
    }
}

pub fn target_id(kind: CleanTargetKind) -> String {
    format!("{kind:?}")
}

pub fn target_label(s: &AppState, kind: CleanTargetKind) -> &'static str {
    s.tr(match kind {
        CleanTargetKind::UserTemp => TextKey::CleanTempUser,
        CleanTargetKind::WindowsTemp => TextKey::CleanTempWin,
        CleanTargetKind::WindowsUpdateCache => TextKey::CleanWinUpdate,
        CleanTargetKind::DeliveryOptimizationCache => TextKey::CleanDeliveryOpt,
        CleanTargetKind::ThumbnailCache => TextKey::CleanThumbnail,
        CleanTargetKind::WerAndCrashDumps => TextKey::CleanCrashDumps,
        CleanTargetKind::Prefetch => TextKey::CleanPrefetch,
        CleanTargetKind::RecycleBin => TextKey::CleanRecycleBin,
    })
}

fn folded(q: &str) -> String {
    let q = q.trim();
    if q.is_empty() {
        String::new()
    } else {
        I18n::fold_search_str(q)
    }
}

pub struct CleanAppsView {
    targets: Rc<VecModel<CleanTargetRow>>,
    appx: Rc<VecModel<AppxRow>>,
    features: Rc<VecModel<FeatureRow>>,
    winget: Rc<VecModel<WingetRow>>,
}

impl CleanAppsView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<CleanAppsStore>();
        let (targets, a) = new_model();
        let (appx, b) = new_model();
        let (features, c) = new_model();
        let (winget, d) = new_model();
        st.set_targets(a);
        st.set_appx(b);
        st.set_features(c);
        st.set_winget(d);
        Self { targets, appx, features, winget }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<CleanAppsStore>();
        macro_rules! on {
            ($setter:ident, |$s:ident| $body:expr) => {{
                let c = ctx.clone();
                st.$setter(move || {
                    c.act(|$s| $body);
                });
            }};
        }
        let c = ctx.clone();
        st.on_set_sub_tab(move |i| {
            let tab = match i {
                1 => CleanAppsSubTab::AppxFeatures,
                2 => CleanAppsSubTab::WinGetInstaller,
                _ => CleanAppsSubTab::DiskCleaner,
            };
            c.act(|s| s.clean_sub_tab = tab);
        });
        on!(on_select_all_targets, |s| s.select_all_clean_targets());
        on!(on_deselect_all_targets, |s| s.deselect_all_clean_targets());
        on!(on_scan_targets, |s| if !s.clean_apps_busy { s.scan_clean_targets_async() });
        on!(on_clean_selected, |s| if !s.clean_apps_busy { s.execute_clean_selected() });
        on!(on_scan_appx, |s| if !s.clean_apps_busy { s.scan_appx_and_features_async() });
        on!(on_remove_appx, |s| if !s.clean_apps_busy { s.remove_selected_appx_async() });
        on!(on_select_filtered_appx, |s| {
            let q = s.appx_filter_edit.text.clone();
            s.select_filtered_appx(&q);
        });
        on!(on_select_safe_ai, |s| s.select_safe_and_ai_appx());
        on!(on_deselect_appx, |s| s.deselect_all_appx());
        on!(on_remove_onedrive, |s| if !s.clean_apps_busy { s.uninstall_onedrive_async() });
        on!(on_remove_edge, |s| if !s.clean_apps_busy { s.remove_edge_async() });
        on!(on_deselect_winget, |s| s.deselect_all_winget());
        on!(on_install_winget, |s| if !s.clean_apps_busy { s.install_selected_winget_async() });

        let c = ctx.clone();
        st.on_toggle_target(move |id| {
            c.act(|s| {
                if s.clean_apps_busy {
                    return;
                }
                if let Some(kind) = s.clean_targets.iter().map(|t| t.kind).find(|k| target_id(*k) == id.as_str()) {
                    s.toggle_clean_target(kind);
                }
            });
        });
        let c = ctx.clone();
        st.on_appx_filter_edited(move |t| {
            c.act(|s| s.appx_filter_edit.text = t.to_string());
        });
        let c = ctx.clone();
        st.on_toggle_appx(move |fam| {
            c.act(|s| {
                let protected = s
                    .installed_appx
                    .iter()
                    .any(|p| p.family_name == fam.as_str() && p.safety_tier == AppxSafetyTier::ProtectedCore);
                if !protected {
                    s.toggle_appx_selection(fam.to_string());
                }
            });
        });
        let c = ctx.clone();
        st.on_toggle_feature(move |name, enable| {
            c.act(|s| {
                if s.clean_apps_busy {
                    return;
                }
                if let Some(kind) = s.optional_features.iter().find(|f| f.name == name.as_str()).map(|f| f.kind) {
                    s.toggle_optional_feature_async(name.to_string(), kind, enable);
                }
            });
        });
        let c = ctx.clone();
        st.on_winget_filter_edited(move |t| {
            c.act(|s| s.winget_filter_edit.text = t.to_string());
        });
        let c = ctx.clone();
        st.on_toggle_winget(move |id| {
            c.act(|s| s.toggle_winget_selection(id.to_string()));
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<CleanAppsStore>();
        st.set_sub_tab(match s.clean_sub_tab {
            CleanAppsSubTab::DiskCleaner => 0,
            CleanAppsSubTab::AppxFeatures => 1,
            CleanAppsSubTab::WinGetInstaller => 2,
        });
        st.set_busy(s.clean_apps_busy);

        // Disk cleaner
        let total: u64 = s.clean_targets.iter().map(|t| t.reclaimable_bytes).sum();
        let selected: u64 = s.clean_targets.iter().filter(|t| t.selected).map(|t| t.reclaimable_bytes).sum();
        st.set_total_reclaimable(format!("{}: {}", s.tr(TextKey::TotalReclaimable), format_mb(total)).into());
        st.set_selected_reclaimable(format_mb(selected).into());
        st.set_last_freed(
            format!(
                "{}: {}",
                s.tr(TextKey::LastCleaned),
                s.last_clean_report.as_ref().map(|r| format_mb(r.total_freed_bytes)).unwrap_or_else(|| "0.0 MB".into())
            )
            .into(),
        );
        let errs: Vec<String> = s
            .last_clean_report
            .as_ref()
            .map(|r| {
                r.results
                    .iter()
                    .filter_map(|x| x.error.as_ref().map(|e| format!("{}: {}", target_label(s, x.kind), e)))
                    .collect()
            })
            .unwrap_or_default();
        st.set_clean_errors(if errs.is_empty() {
            SharedString::new()
        } else {
            format!("{}: {}", s.tr(TextKey::CleanErrorLabel), errs.join(" · ")).into()
        });
        sync_vec(
            &self.targets,
            s.clean_targets
                .iter()
                .map(|t| CleanTargetRow {
                    id: target_id(t.kind).into(),
                    label: target_label(s, t.kind).into(),
                    path: t.paths.first().cloned().unwrap_or_default().into(),
                    size: format_mb(t.reclaimable_bytes).into(),
                    files: format!("{} {}", t.file_count, s.tr(TextKey::FilesUnit)).into(),
                    selected: t.selected,
                    admin: t.requires_elevation,
                    error: t
                        .last_error
                        .as_ref()
                        .map(|e| format!("{}: {}", s.tr(TextKey::CleanErrorLabel), e))
                        .unwrap_or_default()
                        .into(),
                })
                .collect(),
        );

        // AppX
        if st.get_appx_filter().as_str() != s.appx_filter_edit.text {
            st.set_appx_filter(s.appx_filter_edit.text.as_str().into());
        }
        let q = folded(&s.appx_filter_edit.text);
        let appx: Vec<AppxRow> = s
            .installed_appx
            .iter()
            .filter(|p| q.is_empty() || I18n::fold_search_str(&p.short_name).contains(&q) || I18n::fold_search_str(&p.family_name).contains(&q))
            .map(|p| {
                let protected = p.safety_tier == AppxSafetyTier::ProtectedCore;
                let (txt, tone) = match p.safety_tier {
                    AppxSafetyTier::SafeBloat => (s.tr(TextKey::SafetySafe), "success"),
                    AppxSafetyTier::AiBloat => (s.tr(TextKey::SafetyAi), "accent"),
                    AppxSafetyTier::Caution => (s.tr(TextKey::SafetyCaution), "warning"),
                    AppxSafetyTier::ProtectedCore => (s.tr(TextKey::ProtectedCoreBadge), "danger"),
                };
                AppxRow {
                    family: p.family_name.as_str().into(),
                    name: p.short_name.as_str().into(),
                    tier_text: txt.into(),
                    tier_tone: tone.into(),
                    protected,
                    selected: !protected && s.selected_appx.contains(&p.family_name),
                }
            })
            .collect();
        st.set_appx_total(appx.len() as i32);
        sync_vec(&self.appx, appx);
        st.set_appx_selected(s.selected_appx.len() as i32);
        st.set_kfm_warning(match &s.onedrive_kfm_warning {
            Some(k) => format!(
                "{} ({})",
                s.tr(TextKey::OneDriveKfmWarn),
                crate::core::status_i18n::localize_kfm_detail(s.language, k)
            )
            .into(),
            None => SharedString::new(),
        });
        sync_vec(
            &self.features,
            s.optional_features
                .iter()
                .map(|f| {
                    let (name, desc) = crate::core::tweak_i18n::localize_optional_feature(s.language, &f.name)
                        .unwrap_or((f.display_name.as_str(), f.description.as_str()));
                    FeatureRow {
                        name: f.name.as_str().into(),
                        title: name.into(),
                        desc: desc.into(),
                        enabled: matches!(f.state, FeatureInstallState::Enabled | FeatureInstallState::EnablePending),
                    }
                })
                .collect(),
        );

        // WinGet
        if st.get_winget_filter().as_str() != s.winget_filter_edit.text {
            st.set_winget_filter(s.winget_filter_edit.text.as_str().into());
        }
        let wq = folded(&s.winget_filter_edit.text);
        sync_vec(
            &self.winget,
            CURATED_WINGET_APPS
                .iter()
                .filter(|a| {
                    wq.is_empty()
                        || [a.name, a.id, a.category, a.description].iter().any(|f| I18n::fold_search_str(f).contains(&wq))
                })
                .map(|a| {
                    let loc = crate::core::tweak_i18n::localize_winget(s.language, a.id);
                    WingetRow {
                        id: a.id.into(),
                        name: a.name.into(),
                        category: loc.map(|(c, _)| c).unwrap_or(a.category).into(),
                        desc: loc.map(|(_, d)| d).unwrap_or(a.description).into(),
                        selected: s.selected_winget.contains(a.id),
                    }
                })
                .collect(),
        );
        st.set_winget_selected(s.selected_winget.len() as i32);
    }
}
