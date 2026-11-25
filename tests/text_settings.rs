//! Settings → Text, driven headlessly through the Slint store like a user click.
//! One test in its own binary: the text engine's settings are process-wide.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use slint::{ComponentHandle, Model};
use winblast0::app::{ActiveView, AppState};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::settings::UserSettings;
use winblast0::core::text_render::{self, RenderMode, UI_FONTS};
use winblast0::ui_slint::bridge::SlintBridge;
use winblast0::ui_slint::{AppStore, MainWindow, Typo};

#[test]
fn font_render_mode_and_contrast_apply_and_round_trip() {
    i_slint_backend_testing::init_no_event_loop();
    let st = AppState::new_with_backend(Arc::new(SandboxEnv::new()));
    let window = MainWindow::new().expect("MainWindow");
    let state = Rc::new(RefCell::new(st));
    let bridge = SlintBridge::new_ephemeral(window.clone_strong(), state);
    let store = window.global::<AppStore>();
    store.invoke_navigate(ActiveView::Settings.name().into());
    bridge.sync();

    // Defaults: Inter, Sharp, contrast 35.
    let fonts = store.get_ui_fonts();
    assert_eq!(fonts.row_count(), UI_FONTS.len());
    assert!(fonts.row_data(0).unwrap().selected);
    assert_eq!(window.global::<Typo>().get_ui(), "Inter Variable");
    assert_eq!(store.get_text_mode(), "sharp");
    assert_eq!(text_render::current().mode, RenderMode::Sharp);

    // Every font can be chosen, and the whole UI switches family.
    for (i, f) in UI_FONTS.iter().enumerate() {
        store.invoke_set_ui_font(i as i32);
        bridge.sync();
        assert_eq!(window.global::<Typo>().get_ui(), f.family);
        assert!(store.get_ui_fonts().row_data(i).unwrap().selected, "{} not marked selected", f.label);
    }
    store.invoke_set_ui_font(99); // out of range: ignored
    bridge.sync();
    assert_eq!(window.global::<Typo>().get_ui(), UI_FONTS[UI_FONTS.len() - 1].family);

    for mode in RenderMode::ALL {
        store.invoke_set_text_mode(mode.id().into());
        bridge.sync();
        assert_eq!(store.get_text_mode(), mode.id());
        assert_eq!(text_render::current().mode, mode);
    }
    store.invoke_set_text_mode("nonsense".into());
    bridge.sync();
    assert_eq!(text_render::current().mode, RenderMode::Classic, "unknown ids are ignored");

    store.invoke_set_text_contrast(250);
    bridge.sync();
    assert_eq!(store.get_text_contrast(), 100);
    assert_eq!(text_render::current().contrast, 100);
    store.invoke_set_text_contrast(-5);
    bridge.sync();
    assert_eq!(text_render::current().contrast, 0);

    // Chinese switches the whole UI to the embedded Simplified Chinese font, and back.
    store.invoke_set_ui_font(3); // Noto Sans
    store.invoke_set_language(6); // 简体中文
    bridge.sync();
    assert_eq!(window.global::<Typo>().get_ui(), text_render::CJK_SC_FAMILY);
    store.invoke_set_language(0);
    bridge.sync();
    assert_eq!(window.global::<Typo>().get_ui(), "Noto Sans");

    // Saved settings keep the choice and clean up bad values.
    let saved = UserSettings { ui_font: "IBM Plex Sans".into(), text_rendering: "crisp".into(), text_contrast: 60, ..UserSettings::default() };
    let back: UserSettings = serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
    assert_eq!(back, saved);
    let bad = UserSettings { ui_font: "Wingdings".into(), text_rendering: "blurry".into(), text_contrast: 200, ..UserSettings::default() }.sanitized();
    assert_eq!((bad.ui_font.as_str(), bad.text_rendering.as_str(), bad.text_contrast), ("Inter Variable", "sharp", 100));
    // A settings file from before this feature still loads, with the defaults.
    let old: UserSettings = serde_json::from_str(r#"{"ui_zoom":1.2}"#).unwrap();
    assert_eq!((old.ui_font.as_str(), old.text_rendering.as_str(), old.text_contrast), ("Inter Variable", "sharp", 35));
}
