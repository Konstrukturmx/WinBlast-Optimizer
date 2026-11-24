//! Dynamic theming system for WinBlast0 (framework-agnostic palette model consumed by the Slint bridge).
//!
//! Provides Light & Dark modes x 5 layout styles paired with 4 OKLCH-designed color schemes
//! (every combination is unit-tested against WCAG 2.1 AA):
//! - Cyber Indigo (Default)
//! - Midnight Stealth
//! - Emerald Mint
//! - Crimson Sunset

use serde::{Deserialize, Serialize};

/// Colour in **linear** sRGB with straight alpha (same convention the v1.6.1 palette was authored in).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Same colour with a different alpha.
    pub fn alpha(self, a: f32) -> Color {
        Color { a, ..self }
    }

    /// Encode to gamma-corrected 8-bit sRGB (r, g, b, a) for display toolkits such as Slint.
    pub fn to_srgb8(self) -> [u8; 4] {
        fn enc(c: f32) -> u8 {
            let c = if c.is_finite() { c.clamp(0.0, 1.0) } else { 0.0 };
            let s = if c <= 0.003_130_8 {
                c * 12.92
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            };
            (s * 255.0).round().clamp(0.0, 255.0) as u8
        }
        let a = if self.a.is_finite() { self.a.clamp(0.0, 1.0) } else { 1.0 };
        [enc(self.r), enc(self.g), enc(self.b), (a * 255.0).round() as u8]
    }
}

/// Spring motion description (perceptual duration in seconds + bounce in (-1, 1)).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring {
    pub duration: f32,
    pub bounce: f32,
}

impl Spring {
    pub const fn new(duration: f32, bounce: f32) -> Spring {
        assert!(duration > 0.0 && duration < 1e4, "spring duration must be positive seconds");
        assert!(bounce > -1.0 && bounce < 1.0, "spring bounce must be in (-1, 1)");
        Spring { duration, bounce }
    }
    /// Effectively instant.
    pub const NONE: Spring = Spring::new(1e-6, 0.0);
    pub const SMOOTH: Spring = Spring::new(0.5, 0.0);
    pub const SNAPPY: Spring = Spring::new(0.5, 0.15);
    pub const BOUNCY: Spring = Spring::new(0.5, 0.3);

    /// Duration in whole milliseconds (0 for NONE).
    pub fn duration_ms(self) -> u32 {
        (self.duration * 1000.0).round() as u32
    }
}

pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
}

pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
}

/// Complete UI theme layout and design system (geometry, elevation, animations, surface materials).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum UiThemeStyle {
    #[default]
    Glassy,
    Minimalistic,
    Classic,
    Compact,
    Materialistic,
}

impl UiThemeStyle {
    pub const ALL: [UiThemeStyle; 5] = [
        UiThemeStyle::Glassy,
        UiThemeStyle::Minimalistic,
        UiThemeStyle::Classic,
        UiThemeStyle::Compact,
        UiThemeStyle::Materialistic,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            UiThemeStyle::Glassy => "💎 Glassy (Mica / Acrylic)",
            UiThemeStyle::Minimalistic => "📐 Minimalistic (Flat Swiss)",
            UiThemeStyle::Classic => "🖥️ Classic (Win2000 Retro)",
            UiThemeStyle::Compact => "⚡ Compact (High Density)",
            UiThemeStyle::Materialistic => "🎨 Materialistic (MD3 Expressive)",
        }
    }

    pub const fn short_label(self) -> &'static str {
        match self {
            UiThemeStyle::Glassy => "Glassy",
            UiThemeStyle::Minimalistic => "Minimal",
            UiThemeStyle::Classic => "Classic",
            UiThemeStyle::Compact => "Compact",
            UiThemeStyle::Materialistic => "Material",
        }
    }

    /// Corner radius for standard card containers.
    pub const fn card_radius(self) -> f32 {
        match self {
            UiThemeStyle::Minimalistic => 0.0,
            UiThemeStyle::Classic => 2.0,
            UiThemeStyle::Compact => 4.0,
            UiThemeStyle::Glassy => 12.0,
            UiThemeStyle::Materialistic => 18.0,
        }
    }

    /// Corner radius for interactive buttons.
    pub const fn btn_radius(self) -> f32 {
        match self {
            UiThemeStyle::Minimalistic => 2.0,
            UiThemeStyle::Classic => 2.0,
            UiThemeStyle::Compact => 4.0,
            UiThemeStyle::Glassy => 8.0,
            UiThemeStyle::Materialistic => 24.0,
        }
    }

    /// Corner radius for input text boxes.
    pub const fn input_radius(self) -> f32 {
        match self {
            UiThemeStyle::Minimalistic => 1.0,
            UiThemeStyle::Classic => 2.0,
            UiThemeStyle::Compact => 4.0,
            UiThemeStyle::Glassy => 8.0,
            UiThemeStyle::Materialistic => 12.0,
        }
    }

    /// Corner radius for status tags and pill badges.
    pub const fn badge_radius(self) -> f32 {
        match self {
            UiThemeStyle::Minimalistic => 2.0,
            UiThemeStyle::Classic => 2.0,
            UiThemeStyle::Compact => 3.0,
            UiThemeStyle::Glassy => 8.0,
            UiThemeStyle::Materialistic => 16.0,
        }
    }

    /// Outer border stroke width for cards.
    pub const fn border_width(self) -> f32 {
        match self {
            UiThemeStyle::Minimalistic => 1.0,
            UiThemeStyle::Classic => 2.0,
            UiThemeStyle::Compact => 1.0,
            UiThemeStyle::Glassy => 1.2,
            UiThemeStyle::Materialistic => 1.0,
        }
    }

    /// Internal content padding for cards.
    pub const fn card_padding(self) -> f32 {
        match self {
            UiThemeStyle::Compact => 8.0,
            UiThemeStyle::Classic => 10.0,
            UiThemeStyle::Minimalistic => 12.0,
            UiThemeStyle::Glassy => 14.0,
            UiThemeStyle::Materialistic => 18.0,
        }
    }

    /// Preferred sidebar width.
    pub const fn sidebar_width(self) -> f32 {
        match self {
            UiThemeStyle::Compact => 195.0,
            UiThemeStyle::Classic => 210.0,
            UiThemeStyle::Minimalistic => 215.0,
            UiThemeStyle::Glassy => 235.0,
            UiThemeStyle::Materialistic => 245.0,
        }
    }

    /// Motion spring for interactive transitions and element animations.
    pub const fn motion(self) -> Spring {
        match self {
            UiThemeStyle::Minimalistic => Spring::NONE,
            UiThemeStyle::Classic => Spring::new(0.15, 0.0),
            UiThemeStyle::Compact => Spring::SNAPPY,
            UiThemeStyle::Glassy => Spring::BOUNCY,
            UiThemeStyle::Materialistic => Spring::SMOOTH,
        }
    }

    /// Smooth transition spring easing constant.
    pub const fn easing_spring(self) -> Spring {
        self.motion()
    }

    /// Easing duration in milliseconds for style transitions.
    pub const fn easing_duration_ms(self) -> u32 {
        match self {
            UiThemeStyle::Minimalistic => 0,
            UiThemeStyle::Classic => 100,
            UiThemeStyle::Compact => 140,
            UiThemeStyle::Glassy => 240,
            UiThemeStyle::Materialistic => 300,
        }
    }

    /// Standard CSS transition timing curve identifier.
    pub const fn easing_curve(self) -> &'static str {
        match self {
            UiThemeStyle::Minimalistic => "linear",
            UiThemeStyle::Classic => "ease-in-out",
            UiThemeStyle::Compact => "cubic-bezier(0.2, 0.0, 0.0, 1.0)",
            UiThemeStyle::Glassy => "cubic-bezier(0.34, 1.56, 0.64, 1.0)",
            UiThemeStyle::Materialistic => "cubic-bezier(0.4, 0.0, 0.2, 1.0)",
        }
    }
}

/// Global transition easing spring constants.
pub const EASING_GLASSY: Spring = Spring::BOUNCY;
pub const EASING_MINIMALISTIC: Spring = Spring::NONE;
pub const EASING_CLASSIC: Spring = Spring::new(0.15, 0.0);
pub const EASING_COMPACT: Spring = Spring::SNAPPY;
pub const EASING_MATERIALISTIC: Spring = Spring::SMOOTH;

/// Mode determining baseline dark vs light tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    pub const fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "🌙 Dark",
            ThemeMode::Light => "☀️ Light",
        }
    }
}

/// Color scheme defining accent chromaticity and branding tones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorScheme {
    #[default]
    CyberIndigo,
    MidnightStealth,
    /// `ArcticAurora` (removed in 1.7.0) loads as its nearest replacement.
    #[serde(alias = "ArcticAurora")]
    EmeraldMint,
    /// `SolarAmber` (removed in 1.7.0) loads as its nearest replacement.
    #[serde(alias = "SolarAmber")]
    CrimsonSunset,
}

impl ColorScheme {
    pub const ALL: [ColorScheme; 4] = [
        ColorScheme::CyberIndigo,
        ColorScheme::MidnightStealth,
        ColorScheme::EmeraldMint,
        ColorScheme::CrimsonSunset,
    ];

    /// Colour-theory recipe: indigo (split-complementary magenta data hue), graphite (near-mono
    /// steel), emerald-teal (analogous cyan) and raspberry rose (analogous sunset orange).
    const fn spec(self) -> SchemeSpec {
        match self {
            ColorScheme::CyberIndigo => SchemeSpec { hue: 268.0, accent_chroma: 0.16, tint: 0.018, hue2: 330.0, accent_l_dark: 0.835, accent_l_light: 0.475 },
            ColorScheme::MidnightStealth => SchemeSpec { hue: 245.0, accent_chroma: 0.045, tint: 0.006, hue2: 200.0, accent_l_dark: 0.815, accent_l_light: 0.465 },
            ColorScheme::EmeraldMint => SchemeSpec { hue: 172.0, accent_chroma: 0.13, tint: 0.014, hue2: 220.0, accent_l_dark: 0.80, accent_l_light: 0.435 },
            ColorScheme::CrimsonSunset => SchemeSpec { hue: 355.0, accent_chroma: 0.16, tint: 0.014, hue2: 45.0, accent_l_dark: 0.87, accent_l_light: 0.485 },
        }
    }

    pub const fn text_key(self) -> crate::core::i18n::TextKey {
        match self {
            ColorScheme::CyberIndigo => crate::core::i18n::TextKey::ColorSchemeCyber,
            ColorScheme::MidnightStealth => crate::core::i18n::TextKey::ColorSchemeMidnight,
            ColorScheme::EmeraldMint => crate::core::i18n::TextKey::ColorSchemeEmerald,
            ColorScheme::CrimsonSunset => crate::core::i18n::TextKey::ColorSchemeCrimson,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            ColorScheme::CyberIndigo => "🔵 Cyber Indigo",
            ColorScheme::MidnightStealth => "⚫ Midnight Stealth",
            ColorScheme::EmeraldMint => "🟢 Emerald Mint",
            ColorScheme::CrimsonSunset => "🔴 Crimson Sunset",
        }
    }
}

/// Dynamic theme palette holding all UI color and geometry tokens.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub theme_style: UiThemeStyle,
    pub scheme: ColorScheme,
    pub card_radius: f32,
    pub btn_radius: f32,
    pub input_radius: f32,
    pub badge_radius: f32,
    pub card_padding: f32,
    pub border_width: f32,
    pub sidebar_width: f32,
    pub motion: Spring,

    pub bg_primary: Color,
    pub bg_sidebar: Color,
    pub bg_card: Color,
    pub bg_card_hover: Color,
    pub border: Color,
    pub border_hover: Color,
    pub border_accent: Color,

    pub accent: Color,
    pub accent_hover: Color,
    pub accent_press: Color,
    /// Label colour on accent fills (dark on the light dark-mode accents, white in light mode).
    pub text_on_accent: Color,
    /// Label colour on danger fills.
    pub text_on_danger: Color,

    pub btn_secondary: Color,
    pub btn_secondary_hover: Color,
    pub btn_secondary_press: Color,

    pub sparkline_cpu: Color,
    pub sparkline_ram: Color,
    pub sparkline_bg: Color,

    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,

    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub neutral_badge: Color,

    pub is_dark: bool,
}

impl ThemePalette {
    /// Card fill for `style` in this palette's mode and scheme (style studio previews).
    pub fn card_fill_styled(&self, style: UiThemeStyle) -> Color {
        if style == self.theme_style {
            return self.bg_card;
        }
        Self::new(self.mode(), self.scheme, style).bg_card
    }

    /// Card border for `style` in this palette's mode and scheme (style studio previews).
    pub fn border_styled(&self, style: UiThemeStyle) -> Color {
        if style == self.theme_style {
            return self.border;
        }
        Self::new(self.mode(), self.scheme, style).border
    }

    /// Light or dark mode this palette was built for.
    pub fn mode(&self) -> ThemeMode {
        if self.is_dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        }
    }

    /// Generate a theme palette for mode x scheme x style. Every combination is verified against
    /// WCAG 2.1 AA (4.5:1 text, 3:1 UI graphics) by the unit tests below.
    pub fn new(mode: ThemeMode, scheme: ColorScheme, style: UiThemeStyle) -> Self {
        let dark = mode == ThemeMode::Dark;
        let spec = scheme.spec();
        let surf = SurfaceSpec::of(dark, style);
        let h = spec.hue;
        let tint = spec.tint * surf.tint;
        // Neutral surfaces carry a faint tint of the scheme hue (pure white stays untinted).
        let surface = |l: f32, a: f32| {
            let c = if l > 0.985 { 0.0 } else { tint };
            oklch(l, c, h).alpha(a)
        };
        let hover_alpha = (surf.card_alpha + 0.1).min(1.0);
        let classic = style == UiThemeStyle::Classic;

        let (accent, accent_hover, accent_press, text_on_accent, text_on_danger);
        let (text_primary, text_secondary, text_muted);
        let (success, warning, danger);
        let (btn_secondary, btn_secondary_hover, btn_secondary_press);
        let (sparkline_cpu, sparkline_ram, sparkline_bg, neutral_badge, border, border_hover);
        if dark {
            // Tonal dark theme: light, desaturated accents carry dark labels (Material 3 "on" colours).
            let l = spec.accent_l_dark;
            accent = oklch(l, spec.accent_chroma, h);
            accent_hover = oklch(l + 0.05, spec.accent_chroma * 0.9, h);
            accent_press = oklch(l - 0.07, spec.accent_chroma, h);
            text_on_accent = oklch(0.18, 0.02, h);
            text_on_danger = oklch(0.18, 0.02, 25.0);
            text_primary = oklch(0.97, 0.004, h);
            text_secondary = oklch(0.86, 0.008, h);
            text_muted = oklch(0.74, 0.012, h);
            success = oklch(0.80, 0.14, HUE_SUCCESS);
            warning = oklch(0.82, 0.14, 80.0);
            danger = oklch(0.865, 0.14, 22.0);
            btn_secondary = surface(surf.card + 0.06, 1.0);
            btn_secondary_hover = surface(surf.card + 0.10, 1.0);
            btn_secondary_press = surface(surf.card + 0.02, 1.0);
            sparkline_cpu = oklch(0.76, 0.13, h);
            sparkline_ram = oklch(0.76, 0.13, spec.hue2);
            sparkline_bg = rgba(1.0, 1.0, 1.0, 0.04);
            neutral_badge = oklch(0.60, 0.01, h);
            border = rgba(1.0, 1.0, 1.0, if classic { 0.22 } else { 0.10 });
            border_hover = rgba(1.0, 1.0, 1.0, if classic { 0.40 } else { 0.24 });
        } else {
            let l = spec.accent_l_light;
            accent = oklch(l, spec.accent_chroma, h);
            accent_hover = oklch(l + 0.05, spec.accent_chroma, h);
            accent_press = oklch(l - 0.06, spec.accent_chroma, h);
            text_on_accent = rgb(1.0, 1.0, 1.0);
            text_on_danger = rgb(1.0, 1.0, 1.0);
            text_primary = oklch(0.22, 0.02, h);
            text_secondary = oklch(0.36, 0.02, h);
            text_muted = oklch(0.45, 0.02, h);
            success = oklch(0.45, 0.12, HUE_SUCCESS);
            warning = oklch(0.475, 0.12, 60.0);
            danger = oklch(0.475, 0.18, 25.0);
            btn_secondary = surface(surf.card - 0.05, 1.0);
            btn_secondary_hover = surface(surf.card - 0.08, 1.0);
            btn_secondary_press = surface(surf.card - 0.11, 1.0);
            sparkline_cpu = oklch(0.52, 0.15, h);
            sparkline_ram = oklch(0.52, 0.15, spec.hue2);
            sparkline_bg = rgba(0.0, 0.0, 0.0, 0.04);
            neutral_badge = oklch(0.80, 0.01, h);
            border = rgba(0.0, 0.0, 0.0, if classic { 0.28 } else { 0.12 });
            border_hover = rgba(0.0, 0.0, 0.0, if classic { 0.50 } else { 0.28 });
        }

        Self {
            theme_style: style,
            scheme,
            card_radius: style.card_radius(),
            btn_radius: style.btn_radius(),
            input_radius: style.input_radius(),
            badge_radius: style.badge_radius(),
            card_padding: style.card_padding(),
            border_width: style.border_width(),
            sidebar_width: style.sidebar_width(),
            motion: style.motion(),

            bg_primary: surface(surf.bg, 1.0),
            bg_sidebar: surface(surf.sidebar, surf.sidebar_alpha),
            bg_card: surface(surf.card, surf.card_alpha),
            bg_card_hover: surface(surf.hover, hover_alpha),
            border,
            border_hover,
            border_accent: accent,

            accent,
            accent_hover,
            accent_press,
            text_on_accent,
            text_on_danger,

            btn_secondary,
            btn_secondary_hover,
            btn_secondary_press,

            sparkline_cpu,
            sparkline_ram,
            sparkline_bg,

            text_primary,
            text_secondary,
            text_muted,

            success,
            warning,
            danger,
            neutral_badge,

            is_dark: dark,
        }
    }

    /// Fallback constructor with default theme style.
    pub fn default_for(mode: ThemeMode, scheme: ColorScheme) -> Self {
        Self::new(mode, scheme, UiThemeStyle::default())
    }
}

// ── Backward-Compatible Static Const Defaults (Dark Cyber Indigo) ───────────
#[allow(dead_code)]
pub const BG_PRIMARY: Color = rgb(0.012, 0.014, 0.022);
#[allow(dead_code)]
pub const BG_SIDEBAR: Color = rgb(0.018, 0.021, 0.032);
#[allow(dead_code)]
pub const BG_CARD: Color = rgb(0.028, 0.033, 0.048);
#[allow(dead_code)]
pub const BG_CARD_HOVER: Color = rgb(0.045, 0.054, 0.078);
#[allow(dead_code)]
pub const BORDER: Color = rgba(1.0, 1.0, 1.0, 0.08);
#[allow(dead_code)]
pub const BORDER_HOVER: Color = rgba(1.0, 1.0, 1.0, 0.20);
#[allow(dead_code)]
pub const BORDER_ACCENT: Color = rgba(0.28, 0.52, 1.0, 0.50);

#[allow(dead_code)]
pub const ACCENT: Color = rgb(0.20, 0.42, 0.95);
#[allow(dead_code)]
pub const ACCENT_HOVER: Color = rgb(0.28, 0.52, 1.0);
#[allow(dead_code)]
pub const ACCENT_PRESS: Color = rgb(0.14, 0.32, 0.80);

#[allow(dead_code)]
pub const BTN_SECONDARY: Color = rgb(0.05, 0.065, 0.095);
#[allow(dead_code)]
pub const BTN_SECONDARY_HOVER: Color = rgb(0.08, 0.10, 0.15);
#[allow(dead_code)]
pub const BTN_SECONDARY_PRESS: Color = rgb(0.035, 0.045, 0.07);

#[allow(dead_code)]
pub const SPARKLINE_CPU: Color = rgb(0.20, 0.70, 0.95);
#[allow(dead_code)]
pub const SPARKLINE_RAM: Color = rgb(0.85, 0.45, 0.95);
#[allow(dead_code)]
pub const SPARKLINE_BG: Color = rgba(1.0, 1.0, 1.0, 0.04);

#[allow(dead_code)]
pub const TEXT_PRIMARY: Color = rgb(0.95, 0.96, 0.98);
#[allow(dead_code)]
pub const TEXT_SECONDARY: Color = rgb(0.70, 0.72, 0.76);
#[allow(dead_code)]
pub const TEXT_MUTED: Color = rgb(0.48, 0.50, 0.55);

#[allow(dead_code)]
pub const SUCCESS: Color = rgb(0.15, 0.78, 0.42);
#[allow(dead_code)]
pub const WARNING: Color = rgb(0.95, 0.65, 0.15);
#[allow(dead_code)]
pub const DANGER: Color = rgb(0.92, 0.24, 0.28);
#[allow(dead_code)]
pub const NEUTRAL_BADGE: Color = rgb(0.25, 0.27, 0.32);

/// Hue (OKLCH degrees) of the success role, kept clear of every scheme accent.
const HUE_SUCCESS: f32 = 145.0;

/// Colour-theory parameters of one scheme. Accent lightness per mode was solved so that every
/// style passes WCAG AA (see the tests at the end of this file).
struct SchemeSpec {
    hue: f32,
    accent_chroma: f32,
    /// Chroma of the hue tint on neutral surfaces.
    tint: f32,
    /// Second data-series hue (split-complementary / analogous partner of `hue`).
    hue2: f32,
    accent_l_dark: f32,
    accent_l_light: f32,
}

/// OKLab lightness / alpha of the surfaces for one style in one mode.
struct SurfaceSpec {
    bg: f32,
    sidebar: f32,
    card: f32,
    hover: f32,
    tint: f32,
    card_alpha: f32,
    sidebar_alpha: f32,
}

impl SurfaceSpec {
    const fn of(dark: bool, style: UiThemeStyle) -> Self {
        const fn s(bg: f32, sidebar: f32, card: f32, hover: f32, tint: f32, card_alpha: f32, sidebar_alpha: f32) -> SurfaceSpec {
            SurfaceSpec { bg, sidebar, card, hover, tint, card_alpha, sidebar_alpha }
        }
        match (dark, style) {
            (true, UiThemeStyle::Glassy) => s(0.160, 0.185, 0.215, 0.240, 1.2, 0.80, 0.92),
            (true, UiThemeStyle::Minimalistic) => s(0.140, 0.158, 0.180, 0.205, 0.35, 1.0, 1.0),
            (true, UiThemeStyle::Classic) => s(0.185, 0.205, 0.225, 0.250, 0.6, 1.0, 1.0),
            (true, UiThemeStyle::Compact) => s(0.155, 0.178, 0.205, 0.230, 0.9, 1.0, 1.0),
            (true, UiThemeStyle::Materialistic) => s(0.168, 0.192, 0.222, 0.248, 2.0, 1.0, 1.0),
            (false, UiThemeStyle::Glassy) => s(0.955, 0.935, 1.0, 0.965, 1.0, 0.80, 0.90),
            (false, UiThemeStyle::Minimalistic) => s(0.990, 0.970, 1.0, 0.955, 0.3, 1.0, 1.0),
            (false, UiThemeStyle::Classic) => s(0.930, 0.915, 0.970, 0.945, 0.5, 1.0, 1.0),
            (false, UiThemeStyle::Compact) => s(0.970, 0.945, 1.0, 0.955, 0.8, 1.0, 1.0),
            (false, UiThemeStyle::Materialistic) => s(0.965, 0.940, 0.985, 0.950, 1.8, 1.0, 1.0),
        }
    }
}

/// OKLCH (lightness 0..1, chroma, hue in degrees) to linear sRGB, clamped into gamut.
pub fn oklch(l: f32, c: f32, hue_deg: f32) -> Color {
    let h = hue_deg.to_radians();
    let (a, b) = (c * h.cos(), c * h.sin());
    let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;
    let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    let clamp = |v: f32| if v.is_finite() { v.clamp(0.0, 1.0) } else { 0.0 };
    rgb(
        clamp(4.076_741_7 * l3 - 3.307_711_6 * m3 + 0.230_969_94 * s3),
        clamp(-1.268_438 * l3 + 2.609_757_4 * m3 - 0.341_319_38 * s3),
        clamp(-0.004_196_086_3 * l3 - 0.703_418_6 * m3 + 1.707_614_7 * s3),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Linear -> 8-bit sRGB -> float sRGB (what the renderer actually stores and blends).
    fn srgb(c: Color) -> [f32; 4] {
        let [r, g, b, a] = c.to_srgb8();
        [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0]
    }

    /// Source-over in gamma-encoded sRGB space, like Slint's renderers.
    fn over(fg: [f32; 4], bg: [f32; 4]) -> [f32; 4] {
        let m = |f: f32, b: f32| f * fg[3] + b * (1.0 - fg[3]);
        [m(fg[0], bg[0]), m(fg[1], bg[1]), m(fg[2], bg[2]), 1.0]
    }

    fn luminance(c: [f32; 4]) -> f32 {
        let d = |v: f32| if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) };
        0.2126 * d(c[0]) + 0.7152 * d(c[1]) + 0.0722 * d(c[2])
    }

    fn contrast(a: [f32; 4], b: [f32; 4]) -> f32 {
        let (x, y) = (luminance(a), luminance(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    fn with_alpha(c: [f32; 4], a: f32) -> [f32; 4] {
        [c[0], c[1], c[2], a]
    }

    /// Alpha of tinted badge backgrounds (`safe-bg`, `high-bg`, ... in ui/tokens/theme.slint).
    const BADGE_ALPHA: f32 = 0.12;

    fn failures(mode: ThemeMode, scheme: ColorScheme, style: UiThemeStyle) -> Vec<String> {
        let p = ThemePalette::new(mode, scheme, style);
        let bg = srgb(p.bg_primary);
        let card = over(srgb(p.bg_card), bg);
        let hover = over(srgb(p.bg_card_hover), bg);
        let sidebar = over(srgb(p.bg_sidebar), bg);
        let btn = over(srgb(p.btn_secondary), bg);
        let btn_hover = over(srgb(p.btn_secondary_hover), bg);
        let mut out = Vec::new();
        let mut check = |name: &str, fg: [f32; 4], back: [f32; 4], min: f32| {
            let v = contrast(over(fg, back), back);
            if v < min {
                out.push(format!("{mode:?}/{scheme:?}/{style:?}: {name} {v:.2} < {min}"));
            }
        };
        let surfaces = [("bg", bg), ("card", card), ("hover", hover), ("sidebar", sidebar), ("btn", btn), ("btn-hover", btn_hover)];
        for (sn, s) in surfaces {
            check(&format!("text-primary/{sn}"), srgb(p.text_primary), s, 4.5);
            check(&format!("text-secondary/{sn}"), srgb(p.text_secondary), s, 4.5);
            check(&format!("text-muted/{sn}"), srgb(p.text_muted), s, 4.5);
        }
        for (sn, s) in &surfaces[..4] {
            for (k, c) in [("accent", p.accent), ("success", p.success), ("warning", p.warning), ("danger", p.danger)] {
                let c = srgb(c);
                check(&format!("{k}/{sn}"), c, *s, 4.5);
                check(&format!("{k}-badge/{sn}"), c, over(with_alpha(c, BADGE_ALPHA), *s), 4.5);
            }
            check(&format!("sparkline-cpu/{sn}"), srgb(p.sparkline_cpu), *s, 3.0);
            check(&format!("sparkline-ram/{sn}"), srgb(p.sparkline_ram), *s, 3.0);
            check(&format!("focus-ring/{sn}"), srgb(p.border_accent), *s, 3.0);
        }
        for (k, c) in [("accent", p.accent), ("accent-hover", p.accent_hover), ("accent-press", p.accent_press)] {
            check(&format!("on-accent/{k}"), srgb(p.text_on_accent), srgb(c), 4.5);
        }
        check("on-danger/danger", srgb(p.text_on_danger), srgb(p.danger), 4.5);
        out
    }

    #[test]
    fn every_theme_combination_meets_wcag_aa() {
        let mut all = Vec::new();
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            for scheme in ColorScheme::ALL {
                for style in UiThemeStyle::ALL {
                    all.extend(failures(mode, scheme, style));
                }
            }
        }
        assert!(all.is_empty(), "WCAG AA contrast failures:\n{}", all.join("\n"));
    }

    #[test]
    fn accent_stays_distinct_from_danger_and_success() {
        // Hue distance in OKLCH degrees between each scheme accent and the semantic roles.
        let dist = |a: f32, b: f32| {
            let d = (a - b).rem_euclid(360.0);
            d.min(360.0 - d)
        };
        for scheme in ColorScheme::ALL {
            let h = scheme.spec().hue;
            assert!(dist(h, 22.0) >= 25.0, "{scheme:?} accent too close to danger");
            assert!(dist(h, HUE_SUCCESS) >= 25.0, "{scheme:?} accent too close to success");
        }
    }

    #[test]
    fn removed_schemes_load_as_their_nearest_replacement() {
        let s: ColorScheme = serde_json::from_str("\"SolarAmber\"").expect("alias");
        assert_eq!(s, ColorScheme::CrimsonSunset);
        let a: ColorScheme = serde_json::from_str("\"ArcticAurora\"").expect("alias");
        assert_eq!(a, ColorScheme::EmeraldMint);
        assert_eq!(serde_json::to_string(&ColorScheme::EmeraldMint).expect("ser"), "\"EmeraldMint\"");
    }

    #[test]
    fn oklch_round_trips_white_black_and_grey() {
        assert_eq!(oklch(1.0, 0.0, 0.0).to_srgb8(), [255, 255, 255, 255]);
        assert_eq!(oklch(0.0, 0.0, 0.0).to_srgb8(), [0, 0, 0, 255]);
        let g = oklch(0.6, 0.0, 120.0).to_srgb8();
        assert!(g[0] == g[1] && g[1] == g[2], "achromatic stays grey: {g:?}");
    }

    #[test]
    fn style_previews_follow_mode_and_scheme() {
        let p = ThemePalette::new(ThemeMode::Light, ColorScheme::EmeraldMint, UiThemeStyle::Glassy);
        let c = p.card_fill_styled(UiThemeStyle::Classic);
        assert_eq!(c, ThemePalette::new(ThemeMode::Light, ColorScheme::EmeraldMint, UiThemeStyle::Classic).bg_card);
        assert_eq!(p.card_fill_styled(UiThemeStyle::Glassy), p.bg_card);
    }
}
