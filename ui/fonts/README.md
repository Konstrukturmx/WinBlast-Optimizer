# Embedded UI fonts

Both fonts are licensed under the SIL Open Font License 1.1 (see the LICENSE files here) and are
compiled into the executable through `import "fonts/..."` in `ui/main.slint`.

| File | Source | Changes |
|------|--------|---------|
| `Inter-UI.ttf` | Inter 4.1 `InterVariable.ttf` (https://github.com/rsms/inter) | Subset to Latin, Latin Extended, Greek, Cyrillic, punctuation and symbols; `opsz` pinned to 14; `wght` limited to 400-800 |
| `JetBrainsMono-UI.ttf` | JetBrains Mono 2.304 `JetBrainsMono[wght].ttf` (https://github.com/JetBrains/JetBrainsMono) | Same subset; `wght` limited to 400-800 |

Chinese and Japanese glyphs are not in these files; Slint falls back to the Windows system fonts for them.

Rebuild (fonttools):

```
pyftsubset InterVariable.ttf --unicodes="U+0000-024F,U+0259,U+02B0-02FF,U+0300-036F,U+0370-03FF,U+0400-052F,U+1E00-1EFF,U+2000-206F,U+20A0-20CF,U+2100-214F,U+2190-21FF,U+2200-22FF,U+2300-23FF,U+25A0-25FF,U+2600-26FF,U+2700-27BF,U+FB00-FB06,U+FEFF,U+FFFD" --layout-features='*' --output-file=Inter-Subset.ttf
fonttools varLib.instancer Inter-Subset.ttf opsz=14 wght=400:800 -o Inter-UI.ttf
```
