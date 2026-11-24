//! Regression guard for symbols that neither the embedded Inter font nor Windows' font fallback
//! draws (they rendered as blank gaps, e.g. the "✕" in "✕ Close" and every "✔" status mark).
//! Verified with the offline render probe in tests/ui_render_preview.rs (WINBLAST_GLYPH_PROBE).

use std::path::Path;

/// Characters that render blank, with the replacement that does render.
const BLANK: &[(char, &str)] = &[
    ('\u{2714}', "✓"),
    ('\u{2715}', "×"),
    ('\u{2718}', "✗"),
    ('\u{25BE}', "⌄ / ▼"),
    ('\u{25B4}', "⌃ / ▲"),
    ('\u{2794}', "→"),
    ('\u{27F2}', "↺"),
    ('\u{27F3}', "↻"),
    ('\u{2610}', "□"),
    ('\u{2611}', "■"),
    ('\u{2630}', "≡"),
    ('\u{2139}', "ⓘ"),
    ('\u{2699}', "⚙ + U+FE0F"),
    ('\u{23F1}', "⏱ + U+FE0F"),
    ('\u{2696}', "⚖ + U+FE0F"),
    ('\u{2709}', "✉ + U+FE0F"),
    ('\u{2744}', "❄ + U+FE0F"),
];

fn scan(dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            scan(&p, out);
            continue;
        }
        if !matches!(p.extension().and_then(|x| x.to_str()), Some("rs") | Some("slint")) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        for (no, line) in text.lines().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for (i, c) in chars.iter().enumerate() {
                let emoji_form = chars.get(i + 1) == Some(&'\u{FE0F}');
                if let Some((_, rep)) = BLANK.iter().find(|(b, _)| b == c) {
                    if !emoji_form {
                        out.push(format!("{}:{}: U+{:04X} renders blank, use {rep}", p.display(), no + 1, *c as u32));
                    }
                }
            }
        }
    }
}

#[test]
fn ui_strings_only_use_glyphs_that_render() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    scan(&root.join("src"), &mut found);
    scan(&root.join("ui"), &mut found);
    // This file lists the blank characters on purpose.
    found.retain(|f| !f.contains("glyph_coverage.rs"));
    assert!(found.is_empty(), "{} unrenderable glyphs:\n{}", found.len(), found.join("\n"));
}
