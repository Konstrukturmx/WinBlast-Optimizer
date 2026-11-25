# Embedded UI fonts

All fonts are licensed under the SIL Open Font License 1.1 (see the LICENSE files here) and are
compiled into the executable through `import "fonts/..."` in `ui/main.slint`. The UI font is
chosen in **Settings → Text** (`src/core/text_render.rs`); Chinese always uses Noto Sans SC.

| File | Family | Source | Changes |
|------|--------|--------|---------|
| `Inter-UI.ttf` | Inter Variable | Inter 4.1 `InterVariable.ttf` (https://github.com/rsms/inter) | Subset (ranges below); `opsz` pinned to 14; `wght` 400-800 |
| `JetBrainsMono-UI.ttf` | JetBrains Mono | JetBrains Mono 2.304 `JetBrainsMono[wght].ttf` | Same subset; `wght` 400-800 |
| `AtkinsonHyperlegibleNext-UI.ttf` | Atkinson Hyperlegible Next | google/fonts `ofl/atkinsonhyperlegiblenext` | Same subset; `wght` 400-800. No Cyrillic (Russian falls back to a Windows font) |
| `IBMPlexSans-UI.ttf` | IBM Plex Sans | google/fonts `ofl/ibmplexsans` | Same subset; `wght` 400-700, `wdth` 100 |
| `NotoSans-UI.ttf` | Noto Sans | google/fonts `ofl/notosans` | Same subset; `wght` 400-800, `wdth` 100 |
| `SourceSans3-UI.ttf` | Source Sans 3 | google/fonts `ofl/sourcesans3` | Same subset; `wght` 400-800 |
| `NotoSansSC-UI.ttf` | Noto Sans SC | google/fonts `ofl/notosanssc` | Same subset + every CJK character used in `src/` and `ui/` (about 1,450) + U+2460-24FF; `wght` 400-800; HVAR/VVAR dropped; ⌄ ↩ ↵ ↺ ↻ ⌨ ✗ grafted from Inter (scaled to 1000 UPM) |

Name tables are normalised so the family name (ID 1) is the one in the table above.

Why Noto Sans SC: Windows falls back to a Japanese font for Han text, which has no
Simplified-only characters (简, 设, 优 …) and no ⌄ ↩ ↺, so the Chinese UI showed gaps and boxes.
`core::text_render` tests fail when a new Chinese string uses a character the subset lacks.

Rebuild (fonttools in a virtualenv):

```
RANGES="U+0000-024F,U+0259,U+02B0-02FF,U+0300-036F,U+0370-03FF,U+0400-052F,U+1E00-1EFF,U+2000-206F,U+20A0-20CF,U+2100-214F,U+2190-21FF,U+2200-22FF,U+2300-23FF,U+25A0-25FF,U+2600-26FF,U+2700-27BF,U+FB00-FB06,U+FEFF,U+FFFD"
pyftsubset Source.ttf --unicodes="$RANGES" --layout-features='*' --name-IDs='*' --hinting --output-file=Subset.ttf
fonttools varLib.instancer Subset.ttf wght=400:800 [wdth=100] -o Family-UI.ttf
# then set name IDs 1/2/4/6 to the family above and drop IDs 16/17/21/22
```

For Noto Sans SC add the CJK characters found in `src/**/*.rs` and `ui/**/*.slint` (U+2E80-9FFF,
U+3000-30FF, U+F900-FAFF, U+FF00-FFEF) and U+2460-24FF to the unicodes, then copy the outlines of
the symbols it lacks from `Inter-UI.ttf` (wght 400, scaled by 1000/2048) into new glyphs.
