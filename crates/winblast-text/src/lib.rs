//! WinBlast0 text engine.
//!
//! Turns font glyphs into anti-aliased coverage bitmaps that line up with the physical pixel
//! grid, so text stays sharp and straight at fractional display scales (125 %, 150 %, …):
//!
//! * [`outline`]: glyph outlines from any TrueType/OpenType font, hinted with the font's own
//!   instructions or the automatic hinter (skrifa), flattened to line segments.
//! * [`raster`]: an exact-area coverage rasterizer (signed-area accumulation, non-zero fill).
//! * [`contrast`]: a coverage curve that keeps thin stems dark enough to read.
//! * [`engine`]: ties these together, caches hinting instances and picks the pixel placement:
//!   baselines always land on whole pixels; horizontal positions use quarter-pixel bins.
//! * [`atlas`]: a shelf packer for the GPU glyph texture.
//! * [`config`]: the process-wide settings (render mode and contrast) the UI changes.
//!
//! The Slint GPU renderer (vendored in `vendor/i-slint-renderer-femtovg`) calls into this crate
//! for every plain text run.

pub mod atlas;
pub mod config;
pub mod contrast;
pub mod engine;
pub mod outline;
pub mod raster;

pub use atlas::ShelfPacker;
pub use config::{config, generation, set_config, RenderMode, TextConfig};
pub use engine::{GlyphBitmap, GlyphKey, GlyphPlacement, TextEngine, SUBPIXEL_BINS};
