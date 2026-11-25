# Vendored crates

## `i-slint-renderer-femtovg` 1.19.0

Slint's GPU renderer, copied from crates.io and patched in through `[patch.crates-io]` in the
root `Cargo.toml`. Licence: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR
LicenseRef-Slint-Software-3.0 (see `LICENSES/`). WinBlast0 uses it under the Royalty-free
Desktop licence, which allows modified copies.

WinBlast0 changes (everything else is upstream 1.19.0):

- `winblast_glyphs.rs` (new): draws plain text runs with the `winblast-text` engine
  (`crates/winblast-text`): hinted glyphs, whole-pixel baselines, contrast curve, Gray8 atlas.
- `itemrenderer.rs`, `draw_glyph_run`: tries `winblast_glyphs::try_draw` first.
- `lib.rs`: declares the module and forgets the glyph atlas when the canvas is replaced.
- `Cargo.toml`: path dependency on `winblast-text`; `Cargo.lock` removed.

When updating Slint, copy the new version of the crate here and re-apply these changes.
