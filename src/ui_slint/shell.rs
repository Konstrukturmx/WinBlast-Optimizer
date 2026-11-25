//! Shell binder: top bar, sidebar, progress strip, changelog, footer, dropdowns, HUD, diff modal.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Instant;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::app::{ActiveView, AppMode, AppState, EngineMode};
use crate::core::i18n::{I18n, Language, TextKey};
use crate::core::settings::UserSettings;
use crate::core::status_i18n::StatusMessageSpec;
use crate::core::text_render::{self, RenderMode};
use crate::core::theme::{ColorScheme, ThemeMode, UiThemeStyle};
use crate::scanner::TweakState;

use super::ctx::Ctx;
use super::models::{new_model, sync_vec};
use super::{i18n, palette, theme, AppStore, ChangelogEntry, ChoiceItem, DiffRowData, Fx, MainWindow, PaletteItem, Scale, ToastItem, Typo};

/// Most toasts kept on screen at once (oldest is dropped first).
pub const MAX_TOASTS: usize = 3;

/// Text settings held by the shell (saved with the other UI preferences).
pub struct TextSettings {
    pub font: Cell<usize>,
    pub mode: Cell<RenderMode>,
    pub contrast: Cell<u8>,
    /// The UI language is Simplified Chinese: the UI font is then Noto Sans SC.
    pub chinese: Cell<bool>,
}

impl TextSettings {
    fn from_settings(settings: &UserSettings) -> Self {
        TextSettings {
            font: Cell::new(text_render::font_index(&settings.ui_font).unwrap_or(0)),
            mode: Cell::new(text_render::resolve_mode(&settings.text_rendering)),
            contrast: Cell::new(settings.text_contrast.min(100)),
            chinese: Cell::new(settings.language == Language::Chinese),
        }
    }

    /// Push font, mode and contrast to Slint and the text engine, then repaint.
    pub fn apply(&self, window: &MainWindow) {
        let family = text_render::effective_family(self.chinese.get(), self.font.get());
        window.global::<Typo>().set_ui(family.into());
        text_render::apply(self.mode.get(), self.contrast.get());
        window.window().request_redraw();
    }
}

/// Toast tone for a finished status message, or `None` when the message is progress/selection
/// chatter that the footer already shows (no toast).
pub fn toast_tone(spec: &StatusMessageSpec, text: &str) -> Option<&'static str> {
    let dbg = format!("{spec:?}");
    let name = dbg.split(['(', ' ', '{']).next().unwrap_or_default();
    if text.trim().is_empty() || name.ends_with("ing") || name == "ScanningStart" {
        return None;
    }
    if matches!(name, "StagedCount" | "SelectedCount" | "DeselectedAll" | "LanguageChanged") {
        return None;
    }
    if ["Fail", "Error", "Panic", "Refused", "NotFound"].iter().any(|k| name.contains(k)) {
        return Some("danger");
    }
    if matches!(name, "NothingStaged" | "NoCleanupSelected" | "LatencyRunTestFirst") {
        return Some("warning");
    }
    if matches!(name, "Raw" | "ActionMessage" | "WinGetResult") {
        let t = text.trim_end();
        if t.ends_with("...") || t.ends_with('…') {
            return None; // free-form progress line
        }
        return Some("info");
    }
    if name.starts_with("ModeSwitched") {
        return Some("info");
    }
    Some("success")
}

pub const THEME_STYLES: [(UiThemeStyle, &str); 5] = [
    (UiThemeStyle::Glassy, "💎 Glassy (Mica / Acrylic)"),
    (UiThemeStyle::Minimalistic, "📐 Minimalistic (Flat Swiss)"),
    (UiThemeStyle::Classic, "🖥️ Classic (Win2000 Retro)"),
    (UiThemeStyle::Compact, "⚡ Compact (High Density)"),
    (UiThemeStyle::Materialistic, "🎨 Materialistic (MD3 Expressive)"),
];

pub const COLOR_SCHEMES: [ColorScheme; 4] = ColorScheme::ALL;

const LANGUAGE_EN_NAMES: [&str; 8] = [
    "English", "Turkish", "German", "French", "Spanish", "Russian", "Simplified Chinese", "Japanese",
];

/// Resolve a view id string ("dashboard", "matrix", ...) to an ActiveView.
pub fn view_from_name(name: &str) -> Option<ActiveView> {
    ActiveView::ALL_VIEWS.iter().copied().find(|v| v.name() == name)
}

/// UI-side cached state for the shell.
pub struct Shell {
    changelog: Rc<VecModel<ChangelogEntry>>,
    diff_rows: Rc<VecModel<DiffRowData>>,
    languages: Rc<VecModel<ChoiceItem>>,
    styles: Rc<VecModel<ChoiceItem>>,
    schemes: Rc<VecModel<ChoiceItem>>,
    hud_since: Cell<Option<Instant>>,
    applied_theme: Cell<Option<(ThemeMode, ColorScheme, UiThemeStyle, bool)>>,
    zoom: Rc<Cell<f32>>,
    reduced_motion: Rc<Cell<bool>>,
    /// Settings → Text: index into UI_FONTS, render mode and contrast.
    text: Rc<TextSettings>,
    ui_fonts: Rc<VecModel<ChoiceItem>>,
    palette: Rc<VecModel<PaletteItem>>,
    toasts: Rc<VecModel<ToastItem>>,
    /// Last status spec seen by `sync` (None until the first sync, which never toasts).
    last_status: RefCell<Option<StatusMessageSpec>>,
    toast_seq: Cell<i32>,
}

impl Shell {
    pub fn new(window: &MainWindow, settings: &UserSettings) -> Self {
        let store = window.global::<AppStore>();
        let (changelog, c_rc) = new_model();
        let (diff_rows, d_rc) = new_model();
        let (languages, l_rc) = new_model();
        let (styles, s_rc) = new_model();
        let (schemes, sc_rc) = new_model();
        let (palette, p_rc) = new_model();
        let (toasts, t_rc) = new_model();
        let (ui_fonts, uf_rc) = new_model();
        store.set_ui_fonts(uf_rc);
        let text = Rc::new(TextSettings::from_settings(settings));
        text.apply(window);
        store.set_palette_items(p_rc);
        store.set_toasts(t_rc);
        store.set_changelog(c_rc);
        store.set_diff_rows(d_rc);
        store.set_languages(l_rc);
        store.set_theme_styles(s_rc);
        store.set_color_schemes(sc_rc);
        store.set_app_version(env!("CARGO_PKG_VERSION").into());
        let zoom = settings.ui_zoom;
        window.global::<Scale>().set_factor(zoom);
        store.set_zoom(zoom);
        Self {
            changelog,
            diff_rows,
            languages,
            styles,
            schemes,
            hud_since: Cell::new(None),
            applied_theme: Cell::new(None),
            zoom: Rc::new(Cell::new(zoom)),
            reduced_motion: Rc::new(Cell::new(settings.reduced_motion)),
            text,
            ui_fonts,
            palette,
            toasts,
            last_status: RefCell::new(None),
            toast_seq: Cell::new(0),
        }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let store = window.global::<AppStore>();

        let c = ctx.clone();
        store.on_navigate(move |name| {
            if let Some(view) = view_from_name(name.as_str()) {
                c.act(|s| s.navigate(view));
            } else {
                tracing::warn!("navigate: unknown view id {name}");
            }
        });
        let c = ctx.clone();
        store.on_set_mode_easy(move |easy| {
            c.act(|s| {
                if easy {
                    s.mode = AppMode::Easy;
                    s.navigate(ActiveView::EasyDashboard);
                } else {
                    s.mode = AppMode::Advanced;
                    s.navigate(ActiveView::AdvancedMatrix);
                }
            });
        });
        let c = ctx.clone();
        store.on_set_engine_instant(move |instant| {
            c.act(|s| {
                let want = if instant { EngineMode::Instant } else { EngineMode::Normal };
                if s.engine_mode != want {
                    s.toggle_engine_mode();
                }
            });
        });
        let c = ctx.clone();
        store.on_toggle_sidebar(move || {
            c.act(|s| s.toggle_sidebar());
        });
        let c = ctx.clone();
        store.on_set_language(move |idx| {
            if let Some(lang) = usize::try_from(idx).ok().and_then(|i| Language::ALL.get(i).copied()) {
                c.act(|s| s.set_language(lang));
            }
        });
        let c = ctx.clone();
        store.on_toggle_theme_mode(move || {
            c.act(|s| s.toggle_theme_mode());
        });
        let c = ctx.clone();
        store.on_set_theme_style(move |idx| {
            if let Some((style, _)) = usize::try_from(idx).ok().and_then(|i| THEME_STYLES.get(i).copied()) {
                c.act(|s| s.set_theme_style(style));
            }
        });
        let c = ctx.clone();
        store.on_set_color_scheme(move |idx| {
            if let Some(cs) = usize::try_from(idx).ok().and_then(|i| COLOR_SCHEMES.get(i).copied()) {
                c.act(|s| s.set_color_scheme(cs));
            }
        });
        let c = ctx.clone();
        store.on_cycle_color_scheme(move || {
            c.act(|s| {
                let i = COLOR_SCHEMES.iter().position(|x| *x == s.color_scheme).unwrap_or(0);
                s.set_color_scheme(COLOR_SCHEMES[(i + 1) % COLOR_SCHEMES.len()]);
            });
        });
        let c = ctx.clone();
        store.on_cycle_theme_style(move || {
            c.act(|s| {
                let i = THEME_STYLES.iter().position(|(x, _)| *x == s.theme_style).unwrap_or(0);
                s.set_theme_style(THEME_STYLES[(i + 1) % THEME_STYLES.len()].0);
            });
        });
        let (c, zoom, w) = (ctx.clone(), self.zoom.clone(), window.as_weak());
        store.on_set_zoom(move |z| {
            let z = if z.is_finite() { z.clamp(crate::core::settings::MIN_ZOOM, crate::core::settings::MAX_ZOOM) } else { 1.0 };
            let z = (z * 100.0).round() / 100.0;
            zoom.set(z);
            if let Some(w) = w.upgrade() {
                w.global::<Scale>().set_factor(z);
                w.global::<AppStore>().set_zoom(z);
            }
            c.dirty.set(true);
        });
        let c = ctx.clone();
        store.on_relaunch_elevated(move || {
            c.act(|s| s.relaunch_elevated());
        });
        let c = ctx.clone();
        store.on_nav_prev(move || {
            c.act(|s| s.nav_prev_view());
        });
        let c = ctx.clone();
        store.on_nav_next(move || {
            c.act(|s| s.nav_next_view());
        });
        let c = ctx.clone();
        store.on_start_scan(move || {
            c.act(|s| {
                if !s.is_scanning {
                    s.start_scan();
                }
            });
        });
        let c = ctx.clone();
        store.on_toggle_changelog(move || {
            c.act(|s| s.toggle_changelog());
        });
        let (lang, model) = (ctx.lang.clone(), self.palette.clone());
        store.on_palette_search(move |query| {
            sync_vec(&model, palette::search(lang.get(), query.as_str()));
        });
        let model = self.toasts.clone();
        store.on_dismiss_toast(move |id| {
            if let Some(i) = model.iter().position(|t| t.id == id) {
                model.remove(i);
            }
        });
        let (c, t, w) = (ctx.clone(), self.text.clone(), window.as_weak());
        store.on_set_ui_font(move |idx| {
            if let (Ok(i), Some(win)) = (usize::try_from(idx), w.upgrade()) {
                if i < text_render::UI_FONTS.len() {
                    t.font.set(i);
                    t.apply(&win);
                    c.dirty.set(true);
                }
            }
        });
        let (c, t, w) = (ctx.clone(), self.text.clone(), window.as_weak());
        store.on_set_text_mode(move |id| {
            if let (Some(mode), Some(win)) = (RenderMode::from_id(id.as_str()), w.upgrade()) {
                t.mode.set(mode);
                t.apply(&win);
                c.dirty.set(true);
            }
        });
        let (c, t, w) = (ctx.clone(), self.text.clone(), window.as_weak());
        store.on_set_text_contrast(move |v| {
            if let Some(win) = w.upgrade() {
                t.contrast.set(v.clamp(0, 100) as u8);
                t.apply(&win);
                c.dirty.set(true);
            }
        });
        let (c, rm) = (ctx.clone(), self.reduced_motion.clone());
        store.on_toggle_reduced_motion(move || {
            rm.set(!rm.get());
            c.dirty.set(true); // next sync re-applies the theme and persists the setting
        });
        let c = ctx.clone();
        store.on_clear_changelog(move || {
            c.act(|s| s.clear_changelog());
        });
        let c = ctx.clone();
        store.on_rollback(move |id| match id.parse::<i64>() {
            Ok(entry_id) => {
                c.act(|s| {
                    if !s.is_rolling_back {
                        s.rollback_entry(entry_id);
                    }
                });
            }
            Err(_) => tracing::warn!("rollback: invalid ledger id {id}"),
        });
        let c = ctx.clone();
        store.on_open_bug_report(move || {
            c.act(|s| s.open_bug_report());
        });
        let c = ctx.clone();
        store.on_dismiss_hud(move || {
            c.act(|s| s.dismiss_progress_hud());
        });
        let c = ctx.clone();
        store.on_close_diff(move || {
            c.act(|s| {
                s.show_diff_preview = false;
                s.inspected_tweak = None;
            });
        });
        store.on_nav_matches(|label, query| {
            let q = I18n::fold_search_str(query.trim());
            q.is_empty() || I18n::fold_search_str(label.as_str()).contains(&q)
        });
    }

    /// Raise a toast when a finished action changed the status (not on the first sync, not for
    /// re-localization of the same message, not for progress chatter).
    fn push_toast_if_new(&self, s: &AppState) {
        let mut last = self.last_status.borrow_mut();
        let first = last.is_none();
        if last.as_ref() == Some(&s.status_spec) {
            return;
        }
        *last = Some(s.status_spec.clone());
        if first {
            return;
        }
        let Some(tone) = toast_tone(&s.status_spec, &s.status_message) else {
            return;
        };
        let id = self.toast_seq.get().wrapping_add(1);
        self.toast_seq.set(id);
        while self.toasts.row_count() >= MAX_TOASTS {
            self.toasts.remove(0);
        }
        self.toasts.push(ToastItem { id, text: s.status_message.as_str().into(), tone: tone.into() });
    }

    /// Current persisted-preference snapshot.
    pub fn settings_snapshot(&self, s: &AppState) -> UserSettings {
        UserSettings {
            language: s.language,
            theme_mode: s.theme_mode,
            color_scheme: s.color_scheme,
            theme_style: s.theme_style,
            ui_zoom: self.zoom.get(),
            telemetry_interval_ms: s.telemetry_interval_ms(),
            sidebar_open: s.sidebar_open,
            reduced_motion: self.reduced_motion.get(),
            ui_font: text_render::UI_FONTS[self.text.font.get()].family.to_string(),
            text_rendering: self.text.mode.get().id().to_string(),
            text_contrast: self.text.contrast.get(),
        }
        .sanitized()
    }

    pub fn sync(&self, window: &MainWindow, ctx: &Ctx, s: &AppState) {
        let store = window.global::<AppStore>();
        let lang = s.language;
        i18n::apply_language(window, ctx, lang);
        if self.text.chinese.get() != (lang == Language::Chinese) {
            self.text.chinese.set(lang == Language::Chinese);
            self.text.apply(window);
        }

        let key = (s.theme_mode, s.color_scheme, s.theme_style, self.reduced_motion.get());
        if self.applied_theme.get() != Some(key) {
            theme::apply(window, s.palette(), key.3);
            self.applied_theme.set(Some(key));
        }

        store.set_is_elevated(s.is_admin_elevated());
        store.set_os_desc(match &s.os_info {
            Some(os) => format!("Win {} ({})", os.edition, os.build_number).into(),
            None => SharedString::new(),
        });
        if let Some(m) = &s.realtime_metrics {
            store.set_metrics_available(true);
            store.set_cpu_usage(m.cpu_usage);
            store.set_ram_usage(m.ram_percentage);
        } else {
            store.set_metrics_available(false);
        }

        let view_name: SharedString = s.active_view.name().into();
        if store.get_active_view() != view_name {
            // Open the staggered-entrance window for the new view's cards and re-seed the aurora.
            let fx = window.global::<Fx>();
            fx.set_entering(true);
            fx.set_stagger(0);
            fx.set_view_serial(fx.get_view_serial().wrapping_add(1));
            store.set_active_view(view_name);
        }
        store.set_mode_easy(s.mode == AppMode::Easy);
        store.set_engine_instant(s.engine_mode == EngineMode::Instant);
        store.set_sidebar_open(s.sidebar_open);
        store.set_fault_count(s.fault_report.as_ref().map_or(0, |r| r.total_critical_faults) as i32);

        // Selectors
        store.set_language_label(lang.label().into());
        store.set_language_code(lang.code().to_uppercase().into());
        sync_vec(
            &self.languages,
            Language::ALL
                .iter()
                .zip(LANGUAGE_EN_NAMES)
                .map(|(l, en)| ChoiceItem { label: l.label().into(), detail: en.into(), selected: *l == lang })
                .collect(),
        );
        sync_vec(
            &self.ui_fonts,
            text_render::UI_FONTS
                .iter()
                .enumerate()
                .map(|(i, f)| ChoiceItem {
                    label: f.label.into(),
                    detail: f.family.into(),
                    selected: i == self.text.font.get(),
                })
                .collect(),
        );
        store.set_text_mode(self.text.mode.get().id().into());
        store.set_text_contrast(i32::from(self.text.contrast.get()));
        store.set_theme_dark(s.theme_mode == ThemeMode::Dark);
        store.set_theme_style_label(s.theme_style.short_label().into());
        sync_vec(
            &self.styles,
            THEME_STYLES
                .iter()
                .map(|(st, label)| ChoiceItem { label: (*label).into(), detail: SharedString::new(), selected: *st == s.theme_style })
                .collect(),
        );
        store.set_color_scheme_label(s.tr(s.color_scheme.text_key()).into());
        sync_vec(
            &self.schemes,
            COLOR_SCHEMES
                .iter()
                .map(|cs| ChoiceItem { label: s.tr(cs.text_key()).into(), detail: SharedString::new(), selected: *cs == s.color_scheme })
                .collect(),
        );

        // Scan / staging
        store.set_is_scanning(s.is_scanning);
        store.set_staged_count(s.selected_tweaks.len() as i32);
        store.set_total_tweaks(s.tweaks.len() as i32);

        // Progress strip (identical rules to v1.6.1)
        let opt_pct = s.optimization_progress();
        let active_count = s.scan_results.iter().filter(|r| r.current_state == TweakState::AlreadyOptimized).count();
        let total_count = s.scan_results.iter().filter(|r| r.current_state != TweakState::NotApplicable).count();
        let progress_val = if s.is_scanning || s.progress_message.is_some() { s.progress.clamp(0.1, 0.95) } else { opt_pct };
        store.set_progress_value(if progress_val.is_finite() { progress_val } else { 0.0 });
        store.set_progress_tone(
            if s.is_scanning { "warning" } else if opt_pct > 0.65 { "success" } else if opt_pct > 0.35 { "accent" } else { "warning" }.into(),
        );
        store.set_progress_text(match &s.progress_message {
            Some(msg) => msg.as_str().into(),
            None => format!(
                "{}: {:.0}% ({} / {} {})",
                s.tr(TextKey::ProgressBarOptimized),
                opt_pct * 100.0,
                active_count,
                total_count,
                s.tr(TextKey::TweaksGained)
            )
            .into(),
        });

        // Changelog
        store.set_show_changelog(s.show_changelog);
        store.set_is_rolling_back(s.is_rolling_back);
        store.set_can_undo(!s.is_rolling_back && !s.is_applying && s.in_flight_tweaks.is_empty());
        if s.show_changelog {
            sync_vec(
                &self.changelog,
                s.ledger_history
                    .iter()
                    .take(12)
                    .map(|e| ChangelogEntry {
                        id: e.id.to_string().into(),
                        timestamp: e.timestamp.format("%m-%d %H:%M:%S").to_string().into(),
                        action: crate::core::status_i18n::localize_ledger_action(lang, &e.action_type).into(),
                        title: s.tweak_title(&e.tweak_id, &e.tweak_id).into(),
                        rolled_back: e.rolled_back,
                        reversible: e.is_reversible(),
                    })
                    .collect(),
            );
        }

        // Footer
        store.set_status_message(s.status_message.as_str().into());
        self.push_toast_if_new(s);

        // HUD
        let hud = s.is_in_progress();
        store.set_hud_visible(hud);
        if hud {
            let since = match self.hud_since.get() {
                Some(t) => t,
                None => {
                    let t = Instant::now();
                    self.hud_since.set(Some(t));
                    t
                }
            };
            store.set_hud_status(s.in_progress_status().into());
            store.set_hud_elapsed(format_elapsed(since.elapsed().as_secs()).into());
        } else {
            self.hud_since.set(None);
        }

        // Diff modal
        let diff_open = s.show_diff_preview || s.inspected_tweak.is_some();
        store.set_diff_visible(diff_open);
        if diff_open {
            let (title, rows) = super::diff::diff_view(s);
            store.set_diff_title(title.into());
            sync_vec(
                &self.diff_rows,
                rows.into_iter()
                    .map(|r| DiffRowData {
                        header: r.header.into(),
                        left: r.left.into(),
                        right: r.right.into(),
                        summary: r.summary.into(),
                        tone: r.tone.as_str().into(),
                        is_pair: r.is_pair,
                    })
                    .collect(),
            );
        }
    }
}

/// "12s" below a minute, otherwise "3m 05s".
pub fn format_elapsed(secs: u64) -> String {
    if secs < 60 {
        format!("{secs}s")
    } else {
        format!("{}m {:02}s", secs / 60, secs % 60)
    }
}

#[allow(dead_code)]
fn _assert_model_types(_: ModelRc<ChangelogEntry>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_view_name_resolves() {
        for v in ActiveView::ALL_VIEWS {
            assert_eq!(view_from_name(v.name()), Some(v));
        }
        assert_eq!(view_from_name("does-not-exist"), None);
    }

    #[test]
    fn elapsed_formatting() {
        assert_eq!(format_elapsed(0), "0s");
        assert_eq!(format_elapsed(59), "59s");
        assert_eq!(format_elapsed(65), "1m 05s");
    }

    #[test]
    fn toast_tone_classifies_outcomes_and_skips_progress() {
        use crate::core::status_i18n::StatusMessageSpec as S;
        assert_eq!(toast_tone(&S::ScanningStart, "Scanning"), None);
        assert_eq!(toast_tone(&S::ScanCompleted(3), "Done"), Some("success"));
        assert_eq!(toast_tone(&S::NothingStaged, "Nothing"), Some("warning"));
        assert_eq!(toast_tone(&S::ScanFailed("x".into()), "Scan failed"), Some("danger"));
        assert_eq!(toast_tone(&S::Raw("Applying tweaks...".into()), "Applying tweaks..."), None);
        assert_eq!(toast_tone(&S::Raw("Exported".into()), "Exported"), Some("info"));
        assert_eq!(toast_tone(&S::Raw(String::new()), ""), None);
        assert_eq!(toast_tone(&S::DeselectedAll, "Deselected"), None);
    }
}
