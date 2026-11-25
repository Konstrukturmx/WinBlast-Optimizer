//! UI fonts and text-rendering settings (Settings → Text).
//!
//! The fonts are embedded in the executable (`ui/fonts/`, SIL OFL 1.1) except Segoe UI, which
//! ships with Windows. Rendering goes through the `winblast-text` engine in the patched Slint
//! GPU renderer; see `vendor/README.md`.

pub use winblast_text::{RenderMode, TextConfig};

/// A selectable UI font: the family name Slint resolves and the label shown in Settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiFont {
    pub family: &'static str,
    pub label: &'static str,
    /// Embedded in the exe (false: provided by Windows).
    pub embedded: bool,
    /// Has Cyrillic letters (Russian falls back to a Windows font otherwise).
    pub cyrillic: bool,
}

pub const UI_FONTS: [UiFont; 6] = [
    UiFont { family: "Inter Variable", label: "Inter", embedded: true, cyrillic: true },
    UiFont { family: "Atkinson Hyperlegible Next", label: "Atkinson Hyperlegible", embedded: true, cyrillic: false },
    UiFont { family: "IBM Plex Sans", label: "IBM Plex Sans", embedded: true, cyrillic: true },
    UiFont { family: "Noto Sans", label: "Noto Sans", embedded: true, cyrillic: true },
    UiFont { family: "Source Sans 3", label: "Source Sans 3", embedded: true, cyrillic: true },
    UiFont { family: "Segoe UI", label: "Segoe UI", embedded: false, cyrillic: true },
];

pub const DEFAULT_UI_FONT: &str = "Inter Variable";
pub const DEFAULT_TEXT_RENDERING: &str = "sharp";
pub const DEFAULT_TEXT_CONTRAST: u8 = winblast_text::config::DEFAULT_CONTRAST;

/// Index of a font family in [`UI_FONTS`] (case-insensitive).
pub fn font_index(family: &str) -> Option<usize> {
    UI_FONTS.iter().position(|f| f.family.eq_ignore_ascii_case(family))
}

/// The family to use for a saved setting; unknown names fall back to Inter.
pub fn resolve_font(family: &str) -> &'static str {
    font_index(family).map_or(DEFAULT_UI_FONT, |i| UI_FONTS[i].family)
}

/// Render mode for a saved setting; unknown ids fall back to Sharp.
pub fn resolve_mode(id: &str) -> RenderMode {
    RenderMode::from_id(id).unwrap_or_default()
}

/// `WINBLAST_TEXT_RENDER=sharp|crisp|smooth|classic` overrides the saved mode for this run
/// (used for before/after screenshots); it is never saved.
pub fn env_override() -> Option<RenderMode> {
    std::env::var("WINBLAST_TEXT_RENDER").ok().and_then(|v| RenderMode::from_id(v.trim()))
}

/// Embedded Simplified Chinese font, used for the whole UI in Chinese: Windows falls back to a
/// Japanese font for Han text, and that has no Simplified-only characters (简, 设, 优 …).
pub const CJK_SC_FAMILY: &str = "Noto Sans SC";

/// The family the UI uses: Noto Sans SC in Chinese, otherwise the chosen font (or the
/// `WINBLAST_UI_FONT` override).
pub fn effective_family(chinese: bool, chosen: usize) -> &'static str {
    if chinese {
        return CJK_SC_FAMILY;
    }
    font_override().unwrap_or(UI_FONTS[chosen.min(UI_FONTS.len() - 1)].family)
}

/// `WINBLAST_UI_FONT=<family>` overrides the saved font for this run (screenshots); never saved.
pub fn font_override() -> Option<&'static str> {
    std::env::var("WINBLAST_UI_FONT").ok().and_then(|v| font_index(v.trim())).map(|i| UI_FONTS[i].family)
}

/// What the text engine is using right now.
pub fn current() -> TextConfig {
    winblast_text::config()
}

/// Push the settings to the text engine.
pub fn apply(mode: RenderMode, contrast: u8) {
    winblast_text::set_config(TextConfig { mode: env_override().unwrap_or(mode), contrast: contrast.min(100) });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonts_and_modes_resolve_with_safe_fallbacks() {
        assert_eq!(resolve_font("noto sans"), "Noto Sans");
        assert_eq!(resolve_font("Comic Sans MS"), DEFAULT_UI_FONT);
        assert_eq!(resolve_mode("crisp"), RenderMode::Crisp);
        assert_eq!(resolve_mode("bogus"), RenderMode::Sharp);
        assert_eq!(font_index(DEFAULT_UI_FONT), Some(0));
        for m in RenderMode::ALL {
            assert_eq!(RenderMode::from_id(m.id()), Some(m));
        }
    }

    #[test]
    fn noto_sans_sc_covers_every_cjk_character_the_app_uses() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let data = std::fs::read(root.join("ui/fonts/NotoSansSC-UI.ttf")).unwrap();
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        let mut missing = std::collections::BTreeSet::new();
        let mut files = vec![];
        for dir in ["src", "ui"] {
            let mut stack = vec![root.join(dir)];
            while let Some(d) = stack.pop() {
                for e in std::fs::read_dir(d).unwrap().flatten() {
                    let p = e.path();
                    if p.is_dir() {
                        stack.push(p);
                    } else if p.extension().is_some_and(|x| x == "rs" || x == "slint") {
                        files.push(p);
                    }
                }
            }
        }
        let mut seen = 0;
        for p in files {
            for ch in std::fs::read_to_string(&p).unwrap_or_default().chars() {
                let o = ch as u32;
                let cjk = (0x2E80..=0x9FFF).contains(&o) || (0x3000..=0x30FF).contains(&o) || (0xFF00..=0xFFEF).contains(&o) || (0xF900..=0xFAFF).contains(&o);
                if cjk {
                    seen += 1;
                    if face.glyph_index(ch).is_none() {
                        missing.insert(ch);
                    }
                }
            }
        }
        assert!(seen > 1000, "expected the Chinese and Japanese catalogs to be scanned");
        assert!(missing.is_empty(), "rebuild ui/fonts/NotoSansSC-UI.ttf (see ui/fonts/README.md); missing: {missing:?}");
        // Symbols next to Chinese text are looked up in a Japanese fallback font that lacks
        // them, so they are built into the Chinese font (grafted from Inter, see ui/fonts/README.md).
        for ch in "⌄↩↺↻✗ⓘ简设优".chars() {
            assert!(face.glyph_index(ch).is_some(), "Noto Sans SC subset lacks {ch}");
        }
        assert_eq!(effective_family(true, 2), CJK_SC_FAMILY);
    }

    #[test]
    fn every_embedded_font_file_declares_its_family_name() {
        // The family names above must match the name table of the files main.slint imports.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui/fonts");
        let files: Vec<Vec<u8>> = std::fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "ttf"))
            .map(|e| std::fs::read(e.path()).unwrap())
            .collect();
        let main_slint = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui/main.slint")).unwrap();
        for f in UI_FONTS.iter().filter(|f| f.embedded) {
            let found = files.iter().any(|data| {
                let face = ttf_parser::Face::parse(data, 0).unwrap();
                face.names().into_iter().any(|n| n.name_id == ttf_parser::name_id::FAMILY && n.to_string().as_deref() == Some(f.family))
            });
            assert!(found, "no embedded font file has family {:?}", f.family);
        }
        for file in ["Inter-UI", "AtkinsonHyperlegibleNext-UI", "IBMPlexSans-UI", "NotoSans-UI", "SourceSans3-UI", "NotoSansSC-UI"] {
            assert!(main_slint.contains(&format!("import \"fonts/{file}.ttf\";")), "main.slint does not import {file}.ttf");
        }
    }
}
