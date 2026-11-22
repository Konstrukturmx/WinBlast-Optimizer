//! Live localization for Slint: resolves `Tr.s("Key")` against the Rust string catalogs.

use std::str::FromStr;

use slint::{ComponentHandle, SharedString};

use crate::core::i18n::{I18n, Language, TextKey};
use crate::core::view_i18n::{tr_view, ViewTextKey};

use super::ctx::Ctx;
use super::{MainWindow, Tr};

/// Resolve a catalog key name (TextKey first, then ViewTextKey). Unknown keys echo the key so a
/// missing translation is visible instead of silently blank.
pub fn lookup(lang: Language, key: &str) -> SharedString {
    if let Ok(k) = TextKey::from_str(key) {
        return I18n::get(lang, k).into();
    }
    if let Ok(k) = ViewTextKey::from_str(key) {
        return tr_view(lang, k).into();
    }
    key.into()
}

/// True if `key` names an entry in either catalog.
pub fn is_known_key(key: &str) -> bool {
    TextKey::from_str(key).is_ok() || ViewTextKey::from_str(key).is_ok()
}

pub fn install(window: &MainWindow, ctx: &Ctx) {
    let lang = ctx.lang.clone();
    window.global::<Tr>().on_lookup(move |key, _rev| lookup(lang.get(), key.as_str()));
}

/// Switch the UI language: updates the lookup cell and bumps the revision so every bound
/// string re-evaluates on the next frame.
pub fn apply_language(window: &MainWindow, ctx: &Ctx, lang: Language) {
    if ctx.lang.get() != lang {
        ctx.lang.set(lang);
        let tr = window.global::<Tr>();
        tr.set_rev(tr.get_rev().wrapping_add(1));
    }
}
