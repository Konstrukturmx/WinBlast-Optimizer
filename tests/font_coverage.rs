//! Embedded font invariants (ui/fonts/README.md):
//! * every letter used by the Latin- and Cyrillic-script UI languages (Turkish included) has a
//!   glyph in Inter-UI.ttf, so no label falls back to a system font with different metrics;
//! * the Turkish alphabet and ASCII are in JetBrainsMono-UI.ttf (logs, telemetry, paths);
//! * the variable `wght` axis spans exactly the weights the .slint files use, no more, no less.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const TURKISH: &str = "ÇçĞğİıÖöŞşÜü";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn font(name: &str) -> Vec<u8> {
    std::fs::read(root().join("ui/fonts").join(name)).expect("embedded font file")
}

fn files(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).expect("readable dir").flatten() {
        let p = e.path();
        if p.is_dir() {
            files(&p, ext, out);
        } else if p.extension().is_some_and(|x| x == ext) {
            out.push(p);
        }
    }
}

/// Latin, Greek and Cyrillic letters used in UI text sources. CJK (Chinese/Japanese use the Windows
/// system fonts) and symbol letters such as ⓘ (covered by tests/glyph_coverage.rs) are excluded.
fn ui_letters() -> BTreeSet<char> {
    let mut paths = Vec::new();
    files(&root().join("src/core"), "rs", &mut paths);
    files(&root().join("ui"), "slint", &mut paths);
    let mut set = BTreeSet::new();
    for p in paths.iter().filter(|p| {
        p.extension().is_some_and(|x| x == "slint") || p.to_string_lossy().contains("i18n")
    }) {
        let text = std::fs::read_to_string(p).expect("utf-8 source");
        set.extend(text.chars().filter(|c| {
            let cp = *c as u32;
            c.is_alphabetic() && (cp < 0x0530 || (0x1E00..0x1F00).contains(&cp))
        }));
    }
    set.extend(TURKISH.chars());
    set
}

fn missing(face: &ttf_parser::Face, chars: impl IntoIterator<Item = char>) -> Vec<char> {
    chars.into_iter().filter(|c| face.glyph_index(*c).is_none()).collect()
}

#[test]
fn inter_covers_all_non_cjk_ui_letters() {
    let data = font("Inter-UI.ttf");
    let face = ttf_parser::Face::parse(&data, 0).expect("valid TTF");
    let letters = ui_letters();
    assert!(letters.len() > 100, "scan found too few letters: {}", letters.len());
    let gaps = missing(&face, letters);
    assert!(gaps.is_empty(), "Inter-UI.ttf lacks glyphs for {gaps:?}");
}

#[test]
fn mono_covers_ascii_and_turkish() {
    let data = font("JetBrainsMono-UI.ttf");
    let face = ttf_parser::Face::parse(&data, 0).expect("valid TTF");
    let gaps = missing(&face, (' '..='~').chain(TURKISH.chars()));
    assert!(gaps.is_empty(), "JetBrainsMono-UI.ttf lacks glyphs for {gaps:?}");
}

/// Numeric literals after `font-weight:` in the .slint sources (including both arms of ternaries).
fn used_weights() -> BTreeSet<u16> {
    let mut paths = Vec::new();
    files(&root().join("ui"), "slint", &mut paths);
    let mut set = BTreeSet::new();
    for p in paths {
        let text = std::fs::read_to_string(&p).expect("utf-8 source");
        for line in text.lines().filter(|l| l.contains("font-weight:")) {
            let rhs = line.split_once("font-weight:").map(|(_, r)| r).unwrap_or("");
            let rhs = rhs.split(';').next().unwrap_or("");
            set.extend(
                rhs.split(|c: char| !c.is_ascii_digit())
                    .filter_map(|n| n.parse::<u16>().ok())
                    .filter(|w| (100..=900).contains(w) && w % 100 == 0),
            );
        }
    }
    set
}

#[test]
fn weight_axes_match_used_weights() {
    let used = used_weights();
    let (lo, hi) = (*used.first().expect("weights used"), *used.last().expect("weights used"));
    for name in ["Inter-UI.ttf", "JetBrainsMono-UI.ttf"] {
        let data = font(name);
        let face = ttf_parser::Face::parse(&data, 0).expect("valid TTF");
        let wght = face
            .variation_axes()
            .into_iter()
            .find(|a| a.tag == ttf_parser::Tag::from_bytes(b"wght"))
            .unwrap_or_else(|| panic!("{name} has no wght axis"));
        assert_eq!(
            (wght.min_value as u16, wght.max_value as u16),
            (lo, hi),
            "{name}: wght axis should span exactly the used weights {used:?}"
        );
    }
}
