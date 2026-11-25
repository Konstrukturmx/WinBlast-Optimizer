//! Glyph rasterization with pixel-grid placement.

use std::collections::HashMap;

use skrifa::instance::{LocationRef, NormalizedCoord, Size};
use skrifa::outline::{DrawSettings, Engine, HintingInstance, HintingOptions, SmoothMode, Target};
use skrifa::raw::types::Tag;
use skrifa::{FontRef, GlyphId, MetadataProvider};

use crate::config::RenderMode;
use crate::contrast;
use crate::outline::SegmentPen;
use crate::raster::{Point, Rasterizer};

/// Horizontal positions are rounded to 1/SUBPIXEL_BINS of a pixel (except in Crisp mode).
pub const SUBPIXEL_BINS: u8 = 4;

/// Largest glyph size (pixels per em) the engine draws; bigger text is left to the vector path.
pub const MAX_PX: f32 = 160.0;
/// Smallest glyph size the engine draws.
pub const MIN_PX: f32 = 3.0;

/// Identifies one rasterized glyph image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    pub font: u64,
    pub coords: u64,
    pub glyph: u32,
    /// Pixel size in 1/16 px.
    pub size_q: u32,
    pub bin: u8,
    pub mode: RenderMode,
    pub contrast: u8,
}

/// An 8-bit coverage image of one glyph.
#[derive(Clone, Debug, PartialEq)]
pub struct GlyphBitmap {
    pub width: u32,
    pub height: u32,
    /// Left edge in pixels, relative to the glyph's origin pixel column.
    pub left: i32,
    /// Top edge in pixels above the baseline row (the image's top row is `baseline - top`).
    pub top: i32,
    pub alpha: Vec<u8>,
}

impl GlyphBitmap {
    pub fn coverage_at(&self, x: u32, y: u32) -> u8 {
        self.alpha[(y * self.width + x) as usize]
    }
}

/// Where a glyph goes on the pixel grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlyphPlacement {
    /// Pixel column of the glyph origin.
    pub x: i32,
    /// Pixel row of the baseline: always whole, so every glyph of a line shares it.
    pub y: i32,
    /// Sub-pixel offset of the origin in 1/SUBPIXEL_BINS px.
    pub bin: u8,
}

/// Snap a device-space glyph origin to the pixel grid.
pub fn place(device_x: f32, device_y: f32, mode: RenderMode) -> GlyphPlacement {
    let y = device_y.round() as i32;
    if mode == RenderMode::Crisp {
        return GlyphPlacement { x: device_x.round() as i32, y, bin: 0 };
    }
    let floor = device_x.floor();
    let mut x = floor as i32;
    let mut bin = ((device_x - floor) * SUBPIXEL_BINS as f32).round() as u8;
    if bin >= SUBPIXEL_BINS {
        x += 1;
        bin = 0;
    }
    GlyphPlacement { x, y, bin }
}

pub fn quantize_size(px: f32) -> u32 {
    (px * 16.0).round().max(1.0) as u32
}

/// Stable key for a font file + face index.
pub fn font_key(blob_id: u64, index: u32) -> u64 {
    blob_id.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ u64::from(index).rotate_left(17)
}

/// Stable key for a set of normalized variation coordinates (FNV-1a).
pub fn coords_key(coords: &[i16]) -> u64 {
    coords.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, c| {
        let h = (h ^ (*c as u16 as u64 & 0xff)).wrapping_mul(0x100_0000_01b3);
        (h ^ (*c as u16 as u64 >> 8)).wrapping_mul(0x100_0000_01b3)
    })
}

type HinterKey = (u64, u64, u32, RenderMode);

/// Rasterizes glyphs. Keeps hinting instances (the font program has run once per font, size
/// and variation) and contrast tables between calls.
#[derive(Default)]
pub struct TextEngine {
    hinters: HashMap<HinterKey, Option<HintingInstance>>,
    luts: HashMap<u8, [u8; 256]>,
    color_fonts: HashMap<u64, bool>,
}

impl TextEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drop cached hinting state (fonts or settings changed).
    pub fn clear(&mut self) {
        self.hinters.clear();
        self.luts.clear();
    }

    /// True when the font has colour glyphs (emoji); those are left to the renderer's own path.
    pub fn is_color_font(&mut self, font: u64, data: &[u8], index: u32) -> bool {
        *self.color_fonts.entry(font).or_insert_with(|| {
            FontRef::from_index(data, index).is_ok_and(|f| {
                [b"COLR", b"CBDT", b"sbix", b"SVG "].iter().any(|t| f.table_data(Tag::new(t)).is_some())
            })
        })
    }

    /// Rasterize one glyph. `coords` are the run's normalized variation coordinates (F2Dot14).
    /// Returns None for empty glyphs (spaces) and glyphs the font cannot draw.
    #[allow(clippy::too_many_arguments)]
    pub fn rasterize(
        &mut self,
        data: &[u8],
        index: u32,
        font: u64,
        coords: &[i16],
        glyph: u32,
        size_q: u32,
        bin: u8,
        mode: RenderMode,
        contrast_level: u8,
    ) -> Option<GlyphBitmap> {
        let face = FontRef::from_index(data, index).ok()?;
        let outlines = face.outline_glyphs();
        let outline = outlines.get(GlyphId::new(glyph))?;
        let norm: Vec<NormalizedCoord> = coords.iter().map(|&c| NormalizedCoord::from_bits(c)).collect();
        let location = LocationRef::new(&norm);
        let size = Size::new(size_q as f32 / 16.0);

        let smooth = match mode {
            RenderMode::Sharp => Some(SmoothMode::Light),
            RenderMode::Crisp => Some(SmoothMode::Normal),
            RenderMode::Smooth | RenderMode::Classic => None,
        };
        let hinter = smooth.and_then(|smooth_mode| {
            self.hinters
                .entry((font, coords_key(coords), size_q, mode))
                .or_insert_with(|| {
                    let options = HintingOptions {
                        engine: Engine::AutoFallback,
                        target: Target::Smooth {
                            mode: smooth_mode,
                            symmetric_rendering: true,
                            // Sharp keeps the font's advances so layout (done by Slint with
                            // unhinted metrics) and drawing agree exactly.
                            preserve_linear_metrics: mode == RenderMode::Sharp,
                        },
                    };
                    HintingInstance::new(&outlines, size, location, options).ok()
                })
                .as_ref()
        });

        let mut pen = SegmentPen::default();
        let drawn = match hinter {
            Some(h) => outline.draw(DrawSettings::hinted(h, false), &mut pen).is_ok(),
            None => false,
        };
        if !drawn {
            pen = SegmentPen::default();
            outline.draw(DrawSettings::unhinted(size, location), &mut pen).ok()?;
        }

        let shift = bin as f32 / SUBPIXEL_BINS as f32;
        let (min_x, min_y, max_x, max_y) = pen.bounds()?;
        let left = (min_x + shift).floor() as i32;
        let right = (max_x + shift).ceil() as i32;
        let top = max_y.ceil() as i32;
        let bottom = min_y.floor() as i32;
        let (width, height) = ((right - left).max(0) as u32, (top - bottom).max(0) as u32);
        if width == 0 || height == 0 || width > 1024 || height > 1024 {
            return None;
        }

        let mut r = Rasterizer::new(width as usize, height as usize);
        let to_bitmap = |p: Point| Point::new(p.x + shift - left as f32, top as f32 - p.y);
        for (a, b) in &pen.segments {
            r.line(to_bitmap(*a), to_bitmap(*b));
        }
        let lut = self.luts.entry(contrast_level).or_insert_with(|| contrast::lut(contrast_level));
        Some(GlyphBitmap { width, height, left, top, alpha: r.alpha(lut) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTER: &[u8] = include_bytes!("../../../ui/fonts/Inter-UI.ttf");
    const FONTS: [(&str, &[u8]); 6] = [
        ("Inter", INTER),
        ("JetBrains Mono", include_bytes!("../../../ui/fonts/JetBrainsMono-UI.ttf")),
        ("Atkinson Hyperlegible Next", include_bytes!("../../../ui/fonts/AtkinsonHyperlegibleNext-UI.ttf")),
        ("IBM Plex Sans", include_bytes!("../../../ui/fonts/IBMPlexSans-UI.ttf")),
        ("Noto Sans", include_bytes!("../../../ui/fonts/NotoSans-UI.ttf")),
        ("Source Sans 3", include_bytes!("../../../ui/fonts/SourceSans3-UI.ttf")),
    ];

    fn gid(data: &[u8], ch: char) -> u32 {
        FontRef::new(data).unwrap().charmap().map(ch).unwrap_or_else(|| panic!("no glyph for {ch}")).to_u32()
    }

    fn draw(e: &mut TextEngine, data: &[u8], ch: char, px: f32, bin: u8, mode: RenderMode) -> GlyphBitmap {
        e.rasterize(data, 0, 1, &[], gid(data, ch), quantize_size(px), bin, mode, 0).unwrap()
    }

    #[test]
    fn placement_snaps_baselines_to_whole_pixels() {
        // Every glyph of a line has the same device y, so they share one baseline row.
        for y in [10.0f32, 10.2, 10.49, 10.51, 10.8] {
            let row = place(3.3, y, RenderMode::Sharp).y;
            assert_eq!(row, y.round() as i32);
        }
        assert_eq!(place(7.1, 0.0, RenderMode::Sharp), GlyphPlacement { x: 7, y: 0, bin: 0 });
        assert_eq!(place(7.3, 0.0, RenderMode::Sharp), GlyphPlacement { x: 7, y: 0, bin: 1 });
        assert_eq!(place(7.6, 0.0, RenderMode::Smooth), GlyphPlacement { x: 7, y: 0, bin: 2 });
        assert_eq!(place(7.9, 0.0, RenderMode::Sharp), GlyphPlacement { x: 8, y: 0, bin: 0 });
        assert_eq!(place(7.6, 0.0, RenderMode::Crisp), GlyphPlacement { x: 8, y: 0, bin: 0 });
    }

    #[test]
    fn flat_bottomed_letters_sit_on_the_baseline_row() {
        // The bitmap's bottom edge is the baseline (top - height == 0) for x, n, H, so a line of
        // text has one straight bottom edge whatever its fractional position was.
        let mut e = TextEngine::new();
        for mode in [RenderMode::Sharp, RenderMode::Crisp, RenderMode::Smooth] {
            for px in [11.0, 12.5, 13.75, 15.0, 17.5] {
                for ch in ['x', 'n', 'H', 'z'] {
                    let b = draw(&mut e, INTER, ch, px, 1, mode);
                    assert_eq!(b.top - b.height as i32, 0, "{ch} at {px}px ({mode:?})");
                    let last_row = (b.height - 1) * b.width;
                    let ink: u32 = b.alpha[last_row as usize..].iter().map(|&a| a as u32).sum();
                    assert!(ink > 0, "{ch} at {px}px has an empty bottom row");
                }
            }
        }
    }

    /// Darkest horizontal bar of a glyph: the mean coverage of its darkest row (over inked
    /// columns). A bar that straddles two pixel rows reads as two grey rows; hinted, it is one
    /// dark row.
    fn darkest_row(b: &GlyphBitmap) -> f64 {
        (0..b.height)
            .map(|y| {
                let cols: Vec<f64> = (0..b.width).map(|x| b.coverage_at(x, y) as f64).filter(|&a| a > 12.0).collect();
                cols.iter().sum::<f64>() / cols.len().max(1) as f64
            })
            .fold(0.0, f64::max)
    }

    #[test]
    fn hinting_puts_horizontal_strokes_on_whole_pixel_rows() {
        let mut e = TextEngine::new();
        let mut stats = HashMap::new();
        for mode in [RenderMode::Smooth, RenderMode::Sharp, RenderMode::Crisp] {
            let rows: Vec<f64> = ['E', 'H', 'T', 'z', 'F', 'L']
                .into_iter()
                .flat_map(|ch| (0..12).map(move |q| (ch, 10.0 + q as f32 * 0.375)))
                .map(|(ch, px)| darkest_row(&draw(&mut e, INTER, ch, px, 1, mode)))
                .collect();
            let mean = rows.iter().sum::<f64>() / rows.len() as f64;
            let min = rows.iter().cloned().fold(255.0, f64::min);
            stats.insert(mode, (mean, min));
        }
        let (smooth, sharp, crisp) = (stats[&RenderMode::Smooth], stats[&RenderMode::Sharp], stats[&RenderMode::Crisp]);
        assert!(sharp.0 > smooth.0 + 10.0, "mean darkest bar: sharp {:.1} vs smooth {:.1}", sharp.0, smooth.0);
        assert!(sharp.1 > smooth.1 + 15.0, "worst bar: sharp {:.1} vs smooth {:.1}", sharp.1, smooth.1);
        assert!(crisp.0 > smooth.0 + 10.0, "mean darkest bar: crisp {:.1} vs smooth {:.1}", crisp.0, smooth.0);
    }

    #[test]
    fn crisp_mode_puts_vertical_stems_on_full_pixels() {
        let mut e = TextEngine::new();
        let mid_row = |b: &GlyphBitmap| (0..b.width).map(|x| b.coverage_at(x, b.height / 2)).collect::<Vec<u8>>();
        // Share of the row's ink in its darkest pixel: high when the stem sits on whole pixels.
        let concentration = |row: &[u8]| *row.iter().max().unwrap() as f64 / row.iter().map(|&a| a as f64).sum::<f64>();
        // Small UI sizes, where a stem is about one pixel wide; at 16 px and up both modes are
        // already solid.
        for px in [10.0, 11.0, 12.0, 13.0, 14.0] {
            let crisp = mid_row(&draw(&mut e, INTER, 'l', px, 0, RenderMode::Crisp));
            // Unhinted and half a pixel off the grid, the same stem is split over two columns.
            let soft = mid_row(&draw(&mut e, INTER, 'l', px, 2, RenderMode::Smooth));
            let darkest = *crisp.iter().max().unwrap();
            assert!(darkest >= 225, "'l' at {px}px: darkest stem pixel only {darkest}");
            assert!(
                concentration(&crisp) >= concentration(&soft),
                "'l' at {px}px: stem pixels crisp {crisp:?} vs half-pixel smooth {soft:?}"
            );
        }
    }

    #[test]
    fn subpixel_bins_shift_the_ink() {
        let mut e = TextEngine::new();
        let centroid = |b: &GlyphBitmap| {
            let (mut s, mut w) = (0.0f64, 0.0f64);
            for y in 0..b.height {
                for x in 0..b.width {
                    let a = b.coverage_at(x, y) as f64;
                    s += a * (x as f64 + 0.5 + b.left as f64);
                    w += a;
                }
            }
            s / w
        };
        let c: Vec<f64> = (0..SUBPIXEL_BINS).map(|bin| centroid(&draw(&mut e, INTER, 'o', 14.0, bin, RenderMode::Smooth))).collect();
        for pair in c.windows(2) {
            let step = pair[1] - pair[0];
            assert!((step - 0.25).abs() < 0.05, "bins move ink by {step:.3} px, expected 0.25");
        }
    }

    #[test]
    fn variable_weight_adds_ink() {
        let mut e = TextEngine::new();
        let g = gid(INTER, 'H');
        let ink = |coords: &[i16], e: &mut TextEngine| -> u64 {
            e.rasterize(INTER, 0, 1, coords, g, quantize_size(20.0), 0, RenderMode::Smooth, 0)
                .unwrap()
                .alpha
                .iter()
                .map(|&a| a as u64)
                .sum()
        };
        let regular = ink(&[], &mut e);
        let bold = ink(&[16384], &mut e); // wght axis at its maximum (800)
        assert!(bold as f64 > regular as f64 * 1.3, "bold {bold} vs regular {regular}");
    }

    #[test]
    fn every_bundled_font_draws_turkish_text_in_every_mode() {
        let mut e = TextEngine::new();
        for (name, data) in FONTS {
            for mode in [RenderMode::Sharp, RenderMode::Crisp, RenderMode::Smooth] {
                for ch in "ğĞıİşŞçÇöÖüÜaZ09".chars() {
                    let b = e.rasterize(data, 0, 7, &[], gid(data, ch), quantize_size(13.0), 0, mode, 35);
                    let b = b.unwrap_or_else(|| panic!("{name}: no bitmap for {ch} ({mode:?})"));
                    assert!(b.alpha.iter().any(|&a| a > 200), "{name}: {ch} has no solid pixel ({mode:?})");
                }
            }
            assert!(!e.is_color_font(name.len() as u64, data, 0), "{name} is not a colour font");
        }
    }

    #[test]
    fn chinese_font_and_grafted_symbols_rasterize() {
        let sc: &[u8] = include_bytes!("../../../ui/fonts/NotoSansSC-UI.ttf");
        let mut e = TextEngine::new();
        for ch in "设置简体中文⌄↩↺✗ⓘ".chars() {
            for mode in [RenderMode::Sharp, RenderMode::Crisp] {
                let b = e.rasterize(sc, 0, 9, &[], gid(sc, ch), quantize_size(14.0), 0, mode, 35);
                let b = b.unwrap_or_else(|| panic!("no bitmap for {ch} ({mode:?})"));
                // ⓘ is a thin ring: some ink, not necessarily a solid pixel at 14 px.
                let darkest = *b.alpha.iter().max().unwrap();
                assert!(darkest > 120, "{ch} is nearly empty (darkest pixel {darkest})");
            }
        }
    }

    #[test]
    fn spaces_and_missing_glyphs_produce_nothing() {
        let mut e = TextEngine::new();
        assert!(e.rasterize(INTER, 0, 1, &[], gid(INTER, ' '), quantize_size(14.0), 0, RenderMode::Sharp, 0).is_none());
        assert!(e.rasterize(INTER, 0, 1, &[], 60_000, quantize_size(14.0), 0, RenderMode::Sharp, 0).is_none());
        assert!(e.rasterize(b"not a font", 0, 2, &[], 5, quantize_size(14.0), 0, RenderMode::Sharp, 0).is_none());
    }

    #[test]
    fn contrast_darkens_thin_strokes() {
        let mut e = TextEngine::new();
        let g = gid(INTER, 'i');
        let sum = |c: u8, e: &mut TextEngine| -> u64 {
            e.rasterize(INTER, 0, 1, &[], g, quantize_size(11.0), 2, RenderMode::Smooth, c).unwrap().alpha.iter().map(|&a| a as u64).sum()
        };
        assert!(sum(100, &mut e) > sum(0, &mut e));
    }

    #[test]
    fn keys_are_stable_and_distinguish_inputs() {
        assert_eq!(coords_key(&[1, 2]), coords_key(&[1, 2]));
        assert_ne!(coords_key(&[1, 2]), coords_key(&[2, 1]));
        assert_ne!(font_key(5, 0), font_key(5, 1));
        assert_eq!(quantize_size(13.0), 208);
    }
}
