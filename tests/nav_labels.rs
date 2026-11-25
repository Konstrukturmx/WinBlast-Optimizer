//! E5: sidebar and page-tab labels must not carry their own emoji. The sidebar draws an icon
//! next to every label, so an emoji inside the translated text showed twice ("🤖 🤖 Automation").

use winblast0::core::i18n::Language;
use winblast0::ui_slint::i18n::{is_known_key, lookup};

fn nav_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for src in [include_str!("../ui/shell/sidebar.slint"), include_str!("../ui/stores/nav.slint")] {
        let mut rest = src;
        while let Some(i) = rest.find("key: \"") {
            rest = &rest[i + 6..];
            if let Some(end) = rest.find('"') {
                keys.push(rest[..end].to_string());
                rest = &rest[end..];
            }
        }
    }
    keys.sort();
    keys.dedup();
    keys
}

#[test]
fn nav_labels_exist_and_start_with_a_letter() {
    let keys = nav_keys();
    assert!(keys.len() >= 30, "found only {} nav keys", keys.len());
    let mut bad = Vec::new();
    for key in &keys {
        assert!(is_known_key(key), "unknown nav key {key}");
        for lang in Language::ALL {
            let label = lookup(lang, key).to_string();
            let first = label.chars().next().unwrap_or(' ');
            if !first.is_alphanumeric() {
                bad.push(format!("{lang:?} {key}: {label:?}"));
            }
        }
    }
    assert!(bad.is_empty(), "labels starting with a symbol:\n{}", bad.join("\n"));
}
