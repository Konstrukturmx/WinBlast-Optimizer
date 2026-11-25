//! Process-wide text settings, changed by the Settings page and read by the renderer.

use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

/// How glyphs are fitted to the pixel grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum RenderMode {
    /// Vertical hinting only (baseline, x-height and horizontal stems snap to pixels) with
    /// quarter-pixel horizontal positioning. Keeps the font's shapes and spacing. Default.
    #[default]
    Sharp = 0,
    /// Full hinting in both directions and whole-pixel glyph positions: the crispest stems,
    /// slightly less even spacing.
    Crisp = 1,
    /// No hinting, but still pixel-aligned baselines and the WinBlast0 rasterizer: closest to
    /// the font's design, softest edges.
    Smooth = 2,
    /// Slint's built-in text rendering (the engine is bypassed).
    Classic = 3,
}

impl RenderMode {
    pub const ALL: [RenderMode; 4] = [RenderMode::Sharp, RenderMode::Crisp, RenderMode::Smooth, RenderMode::Classic];

    pub fn from_u8(v: u8) -> RenderMode {
        match v {
            1 => RenderMode::Crisp,
            2 => RenderMode::Smooth,
            3 => RenderMode::Classic,
            _ => RenderMode::Sharp,
        }
    }

    /// Stable identifier used in the settings file.
    pub fn id(self) -> &'static str {
        match self {
            RenderMode::Sharp => "sharp",
            RenderMode::Crisp => "crisp",
            RenderMode::Smooth => "smooth",
            RenderMode::Classic => "classic",
        }
    }

    pub fn from_id(id: &str) -> Option<RenderMode> {
        RenderMode::ALL.into_iter().find(|m| m.id() == id)
    }
}

/// Settings for the text engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextConfig {
    pub mode: RenderMode,
    /// 0 = the rasterizer's exact coverage, 100 = strongest darkening of thin stems.
    pub contrast: u8,
}

pub const DEFAULT_CONTRAST: u8 = 35;

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig { mode: RenderMode::Sharp, contrast: DEFAULT_CONTRAST }
    }
}

static MODE: AtomicU8 = AtomicU8::new(RenderMode::Sharp as u8);
static CONTRAST: AtomicU8 = AtomicU8::new(DEFAULT_CONTRAST);
static GENERATION: AtomicU32 = AtomicU32::new(1);

/// Replace the settings. Renderers notice the new [`generation`] and drop cached glyphs.
pub fn set_config(cfg: TextConfig) {
    let old = config();
    MODE.store(cfg.mode as u8, Ordering::Relaxed);
    CONTRAST.store(cfg.contrast.min(100), Ordering::Relaxed);
    if old != config() {
        GENERATION.fetch_add(1, Ordering::Release);
    }
}

pub fn config() -> TextConfig {
    TextConfig { mode: RenderMode::from_u8(MODE.load(Ordering::Relaxed)), contrast: CONTRAST.load(Ordering::Relaxed) }
}

/// Changes every time [`set_config`] changes something.
pub fn generation() -> u32 {
    GENERATION.load(Ordering::Acquire)
}
