// Copyright © WinBlast0 contributors
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! WinBlast0 addition (not part of upstream Slint): draw plain text runs with the
//! `winblast-text` engine instead of femtovg's own glyph renderer.
//!
//! femtovg places glyph images at fractional device positions, so at 125 % or 150 % display
//! scaling every text item is resampled between pixels and lines come out soft and slightly
//! uneven. Here each run is placed in device pixels: the baseline is rounded to a whole row,
//! glyphs are hinted by `winblast-text`, and the images are drawn 1:1 from a Gray8 atlas with
//! nearest sampling. Rotated or skewed text, colour (emoji) fonts and very large sizes keep
//! femtovg's path.

use std::cell::RefCell;
use std::collections::HashMap;

use femtovg::{Canvas, DrawCommand, GlyphDrawCommands, ImageFlags, ImageId, Paint, PixelFormat, PositionedGlyph, Quad, Renderer};
use i_slint_core::textlayout::sharedparley::parley;
use winblast_text::engine::{coords_key, font_key, place, quantize_size, MAX_PX, MIN_PX};
use winblast_text::{GlyphKey, RenderMode, ShelfPacker, TextEngine};

const ATLAS_SIZE: u32 = 1024;

#[derive(Clone, Copy)]
struct Slot {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    left: i32,
    top: i32,
}

struct Atlas {
    canvas: usize,
    image: ImageId,
    packer: ShelfPacker,
    slots: HashMap<GlyphKey, Option<Slot>>,
    generation: u32,
}

thread_local! {
    static ENGINE: RefCell<TextEngine> = RefCell::new(TextEngine::new());
    static ATLAS: RefCell<Option<Atlas>> = const { RefCell::new(None) };
}

/// Forget the atlas (the canvas it lives on was replaced).
pub(crate) fn reset() {
    ATLAS.with(|a| a.borrow_mut().take());
}

/// Draw a run of glyphs. Returns false when the run must be drawn by femtovg instead.
pub(crate) fn try_draw<R: Renderer>(
    canvas: &mut Canvas<R>,
    font: &parley::FontData,
    font_size: f32,
    coords: &[i16],
    glyphs: &[PositionedGlyph],
    paint: &Paint,
) -> bool {
    let cfg = winblast_text::config();
    if cfg.mode == RenderMode::Classic || glyphs.is_empty() {
        return false;
    }
    let t = canvas.transform();
    let (a, b, c, d, e, f) = (t[0], t[1], t[2], t[3], t[4], t[5]);
    // Only translation and uniform scale map glyph images 1:1 onto device pixels.
    if b.abs() > 1e-4 || c.abs() > 1e-4 || a <= 0.0 || (a - d).abs() > 1e-4 {
        return false;
    }
    let px = font_size * a;
    if !(MIN_PX..=MAX_PX).contains(&px) {
        return false;
    }
    let data = font.data.data();
    let fkey = font_key(font.data.id(), font.index);
    let size_q = quantize_size(px);
    let ckey = coords_key(coords);
    let canvas_id = canvas as *const Canvas<R> as usize;
    let generation = winblast_text::generation();

    ENGINE.with(|engine| {
        ATLAS.with(|atlas| {
            let mut engine = engine.borrow_mut();
            if engine.is_color_font(fkey, data, font.index) {
                return false;
            }
            let mut atlas = atlas.borrow_mut();
            let stale = atlas.as_ref().is_none_or(|at| at.canvas != canvas_id || at.generation != generation);
            if stale {
                if let Some(old) = atlas.take() {
                    if old.canvas == canvas_id {
                        canvas.delete_image(old.image);
                    }
                    if old.generation != generation {
                        engine.clear();
                    }
                }
                let Ok(image) = canvas.create_image_empty(
                    ATLAS_SIZE as usize,
                    ATLAS_SIZE as usize,
                    PixelFormat::Gray8,
                    ImageFlags::NEAREST,
                ) else {
                    return false;
                };
                *atlas = Some(Atlas {
                    canvas: canvas_id,
                    image,
                    packer: ShelfPacker::new(ATLAS_SIZE, ATLAS_SIZE),
                    slots: HashMap::new(),
                    generation,
                });
            }
            let Some(at) = atlas.as_mut() else { return false };

            let inv = 1.0 / ATLAS_SIZE as f32;
            let mut quads = Vec::with_capacity(glyphs.len());
            for g in glyphs {
                let p = place(a * g.x + e, d * g.y + f, cfg.mode);
                let key = GlyphKey {
                    font: fkey,
                    coords: ckey,
                    glyph: u32::from(g.glyph_id),
                    size_q,
                    bin: p.bin,
                    mode: cfg.mode,
                    contrast: cfg.contrast,
                };
                let slot = match at.slots.get(&key) {
                    Some(slot) => *slot,
                    None => {
                        let bitmap = engine.rasterize(data, font.index, fkey, coords, key.glyph, size_q, p.bin, cfg.mode, cfg.contrast);
                        let slot = match bitmap {
                            None => None,
                            Some(bm) => {
                                let Some((x, y)) = at.packer.allocate(bm.width, bm.height) else {
                                    // Atlas full: draw this run with femtovg and start a fresh
                                    // atlas on the next run.
                                    at.generation = 0;
                                    return false;
                                };
                                let pixels: Vec<rgb::alt::Gray<u8>> = bm.alpha.iter().map(|&v| rgb::alt::Gray(v)).collect();
                                let img = imgref::Img::new(pixels.as_slice(), bm.width as usize, bm.height as usize);
                                if canvas.update_image(at.image, img, x as usize, y as usize).is_err() {
                                    at.generation = 0;
                                    return false;
                                }
                                Some(Slot { x, y, w: bm.width, h: bm.height, left: bm.left, top: bm.top })
                            }
                        };
                        at.slots.insert(key, slot);
                        slot
                    }
                };
                if let Some(s) = slot {
                    // Device-pixel rectangle, mapped back to the canvas's local coordinates because
                    // draw_glyph_commands applies the current transform again.
                    let x0 = (p.x + s.left) as f32;
                    let y0 = (p.y - s.top) as f32;
                    quads.push(Quad {
                        x0: (x0 - e) / a,
                        y0: (y0 - f) / d,
                        x1: (x0 + s.w as f32 - e) / a,
                        y1: (y0 + s.h as f32 - f) / d,
                        s0: s.x as f32 * inv,
                        t0: s.y as f32 * inv,
                        s1: (s.x + s.w) as f32 * inv,
                        t1: (s.y + s.h) as f32 * inv,
                    });
                }
            }
            if !quads.is_empty() {
                canvas.draw_glyph_commands(
                    GlyphDrawCommands { alpha_glyphs: vec![DrawCommand { image_id: at.image, quads }], color_glyphs: vec![] },
                    paint,
                );
            }
            true
        })
    })
}
