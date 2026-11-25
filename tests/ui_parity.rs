//! Slint 1.19 shell parity & integrity suite (headless, `i-slint-backend-testing`).
//!
//! Every test runs against a sandboxed backend (`SandboxEnv`): nothing here touches the host
//! registry, services or files, and the bridge is created in ephemeral mode (no settings file).

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use slint::ComponentHandle;
use winblast0::app::{ActiveView, AppMode, AppState};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::i18n::{I18n, Language, TextKey};
use winblast0::core::theme::{ColorScheme, ThemeMode, ThemePalette, UiThemeStyle};
use winblast0::ui_slint::bridge::SlintBridge;
use winblast0::ui_slint::{AppStore, MainWindow, Scale, Theme, Tr};

fn harness() -> (MainWindow, Rc<RefCell<AppState>>, Rc<SlintBridge>) {
    i_slint_backend_testing::init_no_event_loop();
    let window = MainWindow::new().expect("create MainWindow");
    let state = Rc::new(RefCell::new(AppState::new_with_backend(Arc::new(SandboxEnv::new()))));
    let bridge = SlintBridge::new_ephemeral(window.clone_strong(), state.clone());
    (window, state, bridge)
}

/// Collect every `Tr.s("Key")` literal used under ui/.
fn slint_tr_keys() -> Vec<(String, String)> {
    fn walk(dir: &std::path::Path, out: &mut Vec<(String, String)>) {
        for entry in std::fs::read_dir(dir).expect("read ui dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "slint") {
                let text = std::fs::read_to_string(&path).expect("read slint file");
                // Literal lookups plus catalog keys stored in data tables (sidebar nav, tabs).
                for marker in ["Tr.s(\"", "key: \"", "title-key: \""] {
                    let mut rest = text.as_str();
                    while let Some(i) = rest.find(marker) {
                        rest = &rest[i + marker.len()..];
                        if let Some(end) = rest.find('"') {
                            let candidate = &rest[..end];
                            // Data-table "key:" fields are only catalog keys when CamelCase identifiers.
                            let is_identifier = candidate.chars().next().is_some_and(|c| c.is_ascii_uppercase())
                                && candidate.chars().all(|c| c.is_ascii_alphanumeric());
                            if marker.starts_with("Tr.") || is_identifier {
                                out.push((path.display().to_string(), candidate.to_string()));
                            }
                            rest = &rest[end..];
                        }
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    walk(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"), &mut out);
    out
}

#[test]
fn every_slint_translation_key_exists_in_the_catalogs() {
    let keys = slint_tr_keys();
    assert!(keys.len() > 40, "expected the shell to use many localized keys, found {}", keys.len());
    let missing: Vec<_> = keys
        .iter()
        .filter(|(_, k)| !winblast0::ui_slint::i18n::is_known_key(k))
        .collect();
    assert!(missing.is_empty(), "unknown Tr keys: {missing:?}");
}

#[test]
fn text_and_view_catalog_key_names_do_not_collide() {
    use strum::VariantArray;
    let text: std::collections::HashSet<&'static str> =
        TextKey::VARIANTS.iter().map(|k| <&'static str>::from(*k)).collect();
    let clashes: Vec<&'static str> = winblast0::core::view_i18n::ViewTextKey::VARIANTS
        .iter()
        .map(|k| <&'static str>::from(*k))
        .filter(|n| text.contains(n))
        .collect();
    assert!(clashes.is_empty(), "ambiguous Tr keys present in both catalogs: {clashes:?}");
}

#[test]
fn language_switch_relocalizes_live_without_restart() {
    let (window, state, bridge) = harness();
    let tr = window.global::<Tr>();
    let en = tr.invoke_lookup("NavHardware".into(), tr.get_rev());
    assert_eq!(en.as_str(), I18n::get(Language::English, TextKey::NavHardware));
    let rev0 = tr.get_rev();

    window.global::<AppStore>().invoke_set_language(2); // German
    bridge.sync();
    assert_eq!(state.borrow().language, Language::German);
    assert_ne!(tr.get_rev(), rev0, "revision must bump so bindings re-evaluate");
    let de = tr.invoke_lookup("NavHardware".into(), tr.get_rev());
    assert_eq!(de.as_str(), I18n::get(Language::German, TextKey::NavHardware));

    // Every language resolves every shell key to a non-empty string.
    for (i, lang) in Language::ALL.iter().enumerate() {
        window.global::<AppStore>().invoke_set_language(i as i32);
        bridge.sync();
        assert_eq!(state.borrow().language, *lang);
        for (_, key) in slint_tr_keys() {
            assert!(!tr.invoke_lookup(key.as_str().into(), tr.get_rev()).is_empty(), "{lang:?} {key}");
        }
    }
    // Out-of-range language index is ignored.
    window.global::<AppStore>().invoke_set_language(99);
    window.global::<AppStore>().invoke_set_language(-1);
    bridge.sync();
    assert_eq!(state.borrow().language, Language::Japanese);
}

#[test]
fn every_theme_combination_reaches_slint() {
    let (window, state, bridge) = harness();
    let theme = window.global::<Theme>();
    let mut accents = std::collections::HashSet::new();
    for mode in [ThemeMode::Dark, ThemeMode::Light] {
        for scheme in winblast0::ui_slint::shell::COLOR_SCHEMES {
            for (style, _) in winblast0::ui_slint::shell::THEME_STYLES {
                {
                    let mut s = state.borrow_mut();
                    s.set_theme_mode(mode);
                    s.set_color_scheme(scheme);
                    s.set_theme_style(style);
                }
                bridge.sync();
                let p = ThemePalette::new(mode, scheme, style);
                assert_eq!(theme.get_dark(), p.is_dark);
                assert_eq!(theme.get_accent(), winblast0::ui_slint::theme::to_slint(p.accent));
                assert_eq!(theme.get_bg_primary(), winblast0::ui_slint::theme::to_slint(p.bg_primary));
                assert!((theme.get_card_radius() - p.card_radius).abs() < f32::EPSILON);
                assert_eq!(theme.get_motion(), p.theme_style.easing_duration_ms() as i64);
                accents.insert(format!("{:?}", theme.get_accent()));
            }
        }
    }
    assert!(accents.len() >= 6, "color schemes must produce distinct accents");
}

#[test]
fn linear_to_srgb_conversion_is_exact_at_the_endpoints() {
    use winblast0::core::theme::{rgb, rgba};
    assert_eq!(rgb(0.0, 0.0, 0.0).to_srgb8(), [0, 0, 0, 255]);
    assert_eq!(rgb(1.0, 1.0, 1.0).to_srgb8(), [255, 255, 255, 255]);
    assert_eq!(rgba(f32::NAN, 2.0, -1.0, 0.5).to_srgb8(), [0, 255, 0, 128]);
    // Linear 0.214 is ~sRGB 128 (mid grey).
    let mid = rgb(0.214, 0.214, 0.214).to_srgb8();
    assert!((127..=129).contains(&mid[0]), "{mid:?}");
}

#[test]
fn navigation_reaches_all_views_through_the_store() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    assert_eq!(ActiveView::ALL_VIEWS.len(), 28);
    for view in ActiveView::ALL_VIEWS {
        store.invoke_navigate(view.name().into());
        bridge.sync();
        // The FPS page was merged into Gaming (checklist E1).
        let expected = if view == ActiveView::FpsBenchmark { ActiveView::GamingStats } else { view };
        assert_eq!(state.borrow().active_view, expected);
        assert_eq!(store.get_active_view().as_str(), expected.name());
    }
    store.invoke_navigate("no-such-view".into());
    bridge.sync();
    assert_eq!(state.borrow().active_view, ActiveView::SupportAbout);
    // Prev / Next wrap around.
    store.invoke_nav_next();
    bridge.sync();
    assert_eq!(state.borrow().active_view, ActiveView::SupportAbout.next());
    store.invoke_nav_prev();
    bridge.sync();
    assert_eq!(state.borrow().active_view, ActiveView::SupportAbout);
}

#[test]
fn easy_advanced_and_engine_mode_switches() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    store.invoke_set_mode_easy(false);
    bridge.sync();
    assert_eq!(state.borrow().mode, AppMode::Advanced);
    assert_eq!(state.borrow().active_view, ActiveView::AdvancedMatrix);
    assert!(!store.get_mode_easy());
    store.invoke_set_mode_easy(true);
    bridge.sync();
    assert_eq!(state.borrow().active_view, ActiveView::EasyDashboard);

    let before = store.get_engine_instant();
    store.invoke_set_engine_instant(!before);
    bridge.sync();
    assert_eq!(store.get_engine_instant(), !before);
    store.invoke_set_engine_instant(!before); // idempotent
    bridge.sync();
    assert_eq!(store.get_engine_instant(), !before);
}

#[test]
fn zoom_is_clamped_and_drives_scale_factor() {
    let (window, _state, _bridge) = harness();
    let store = window.global::<AppStore>();
    let scale = window.global::<Scale>();
    store.invoke_set_zoom(1.5);
    assert!((scale.get_factor() - 1.5).abs() < 1e-6);
    store.invoke_set_zoom(9.0);
    assert!((scale.get_factor() - 2.0).abs() < 1e-6);
    store.invoke_set_zoom(0.1);
    assert!((scale.get_factor() - 0.75).abs() < 1e-6);
    store.invoke_set_zoom(f32::NAN);
    assert!((scale.get_factor() - 1.0).abs() < 1e-6);
    assert!(scale.get_font_md() > 0.0);
}

#[test]
fn hud_follows_background_work_and_can_be_dismissed() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    // A fresh AppState starts in "initial scan" mode (setup() kicks the scan off immediately).
    assert!(store.get_hud_visible(), "startup scan must show the HUD");
    {
        let mut s = state.borrow_mut();
        s.is_scanning = false;
        s.progress_message = None;
    }
    bridge.sync();
    assert!(!store.get_hud_visible());
    state.borrow_mut().is_scanning = true;
    bridge.sync();
    assert!(store.get_hud_visible());
    assert!(!store.get_hud_status().is_empty());
    store.invoke_dismiss_hud();
    bridge.sync();
    assert!(!store.get_hud_visible());
}

#[test]
fn diff_modal_shows_inspected_tweak_and_closes() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    let actions = state.borrow().tweaks.iter().find(|t| !t.actions.is_empty()).map(|t| (t.title.clone(), t.actions.clone()));
    let (title, actions) = actions.expect("catalog has tweaks with actions");
    state.borrow_mut().inspected_tweak = Some((title.clone(), actions.clone()));
    bridge.sync();
    assert!(store.get_diff_visible());
    assert!(store.get_diff_title().contains(title.as_str()));
    assert_eq!(slint::Model::row_count(&store.get_diff_rows()), actions.len());
    store.invoke_close_diff();
    bridge.sync();
    assert!(!store.get_diff_visible());
    assert!(state.borrow().inspected_tweak.is_none());
}

#[test]
fn sidebar_search_folds_case_and_diacritics() {
    let (window, _s, _b) = harness();
    let store = window.global::<AppStore>();
    assert!(store.invoke_nav_matches("Türkçe".into(), "turkce".into()));
    assert!(store.invoke_nav_matches("Hardware".into(), "HARD".into()));
    assert!(store.invoke_nav_matches("Anything".into(), "   ".into()));
    assert!(!store.invoke_nav_matches("Hardware".into(), "zzz".into()));
}

#[test]
fn reentrant_callbacks_never_panic_on_borrowed_state() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    let guard = state.borrow_mut();
    // State is borrowed: actions must be skipped, sync must be a no-op, nothing may panic.
    store.invoke_navigate("matrix".into());
    store.invoke_toggle_sidebar();
    bridge.sync();
    bridge.tick();
    drop(guard);
    bridge.sync();
    assert_ne!(state.borrow().active_view, ActiveView::AdvancedMatrix);
}

#[test]
fn changelog_toggle_and_rollback_id_validation() {
    let (window, state, bridge) = harness();
    let store = window.global::<AppStore>();
    store.invoke_toggle_changelog();
    bridge.sync();
    assert!(store.get_show_changelog());
    assert!(state.borrow().show_changelog);
    store.invoke_rollback("not-a-number".into()); // must be rejected without panicking
    store.invoke_toggle_changelog();
    bridge.sync();
    assert!(!store.get_show_changelog());
}

#[test]
fn shell_reports_version_and_progress_text() {
    let (window, s, bridge) = harness();
    {
        let mut st = s.borrow_mut();
        st.is_scanning = false;
        st.progress_message = None;
    }
    bridge.sync();
    let store = window.global::<AppStore>();
    assert_eq!(store.get_app_version().as_str(), env!("CARGO_PKG_VERSION"));
    assert!(store.get_progress_text().contains('%'));
    let v = store.get_progress_value();
    assert!((0.0..=1.0).contains(&v), "{v}");
    assert!(store.get_total_tweaks() > 200, "embedded catalog must load");
    let _ = UiThemeStyle::default();
    let _ = ColorScheme::default();
}
