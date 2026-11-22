//! Dynamic theming system for WinBlast0 (framework-agnostic palette model consumed by the Slint bridge).
//!
//! Provides Light & Dark modes paired with 4 distinct WCAG AA compliant color schemes:
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
    EmeraldMint,
    CrimsonSunset,
    SolarAmber,
    ArcticAurora,
}

impl ColorScheme {
    #[allow(dead_code)]
    pub const ALL: [ColorScheme; 6] = [
        ColorScheme::CyberIndigo,
        ColorScheme::MidnightStealth,
        ColorScheme::EmeraldMint,
        ColorScheme::CrimsonSunset,
        ColorScheme::SolarAmber,
        ColorScheme::ArcticAurora,
    ];

    pub const fn text_key(self) -> crate::core::i18n::TextKey {
        match self {
            ColorScheme::CyberIndigo => crate::core::i18n::TextKey::ColorSchemeCyber,
            ColorScheme::MidnightStealth => crate::core::i18n::TextKey::ColorSchemeMidnight,
            ColorScheme::EmeraldMint => crate::core::i18n::TextKey::ColorSchemeEmerald,
            ColorScheme::CrimsonSunset => crate::core::i18n::TextKey::ColorSchemeCrimson,
            ColorScheme::SolarAmber => crate::core::i18n::TextKey::ColorSchemeSolar,
            ColorScheme::ArcticAurora => crate::core::i18n::TextKey::ColorSchemeArctic,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            ColorScheme::CyberIndigo => "🔵 Cyber Indigo",
            ColorScheme::MidnightStealth => "⚫ Midnight Stealth",
            ColorScheme::EmeraldMint => "🟢 Emerald Mint",
            ColorScheme::CrimsonSunset => "🔴 Crimson Sunset",
            ColorScheme::SolarAmber => "🟡 Solar Amber",
            ColorScheme::ArcticAurora => "❄️ Arctic Aurora",
        }
    }
}

/// Dynamic theme palette holding all UI color and geometry tokens.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub theme_style: UiThemeStyle,
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
    /// Card fill color customized for the active UI theme style (acrylic translucent, flat Swiss, retro beveled, compact, or tonal Material 3).
    pub fn card_fill_styled(&self, style: UiThemeStyle) -> Color {
        if style == self.theme_style {
            return self.bg_card;
        }
        match style {
            UiThemeStyle::Glassy => {
                if self.is_dark {
                    rgba(0.032, 0.040, 0.062, 0.70)
                } else {
                    rgba(1.0, 1.0, 1.0, 0.78)
                }
            }
            UiThemeStyle::Minimalistic => {
                if self.is_dark {
                    rgb(0.022, 0.022, 0.026)
                } else {
                    rgb(1.0, 1.0, 1.0)
                }
            }
            UiThemeStyle::Classic => {
                if self.is_dark {
                    rgb(0.065, 0.074, 0.095)
                } else {
                    rgb(0.93, 0.94, 0.96)
                }
            }
            UiThemeStyle::Compact => {
                if self.is_dark {
                    rgb(0.032, 0.040, 0.058)
                } else {
                    rgb(1.0, 1.0, 1.0)
                }
            }
            UiThemeStyle::Materialistic => {
                if self.is_dark {
                    rgb(0.044, 0.050, 0.070)
                } else {
                    rgb(0.99, 1.0, 1.0)
                }
            }
        }
    }

    /// Border stroke color customized for the active UI theme style.
    pub fn border_styled(&self, style: UiThemeStyle) -> Color {
        if style == self.theme_style {
            return self.border;
        }
        match style {
            UiThemeStyle::Glassy => self.accent.alpha(0.30),
            UiThemeStyle::Minimalistic => {
                if self.is_dark {
                    rgba(1.0, 1.0, 1.0, 0.12)
                } else {
                    rgba(0.0, 0.0, 0.0, 0.14)
                }
            }
            UiThemeStyle::Classic => {
                if self.is_dark {
                    rgba(1.0, 1.0, 1.0, 0.24)
                } else {
                    rgba(0.0, 0.0, 0.0, 0.35)
                }
            }
            UiThemeStyle::Compact => self.border,
            UiThemeStyle::Materialistic => self.accent.alpha(0.22),
        }
    }

    /// Generate a theme palette given mode, scheme, and layout/geometry style, tuned for WCAG 2.1 AA compliance.
    pub fn new(mode: ThemeMode, scheme: ColorScheme, style: UiThemeStyle) -> Self {
        match mode {
            ThemeMode::Dark => Self::build_dark(scheme, style),
            ThemeMode::Light => Self::build_light(scheme, style),
        }
    }

    /// Fallback constructor with default theme style.
    pub fn default_for(mode: ThemeMode, scheme: ColorScheme) -> Self {
        Self::new(mode, scheme, UiThemeStyle::default())
    }

    fn build_dark(scheme: ColorScheme, style: UiThemeStyle) -> Self {
        let (accent, accent_hover, accent_press) = match scheme {
            ColorScheme::CyberIndigo => (
                rgb(0.20, 0.42, 0.95), // #336BFF
                rgb(0.28, 0.52, 1.00),
                rgb(0.14, 0.32, 0.80),
            ),
            ColorScheme::MidnightStealth => (
                rgb(0.40, 0.46, 0.58), // Slate 500
                rgb(0.52, 0.58, 0.70),
                rgb(0.30, 0.35, 0.45),
            ),
            ColorScheme::EmeraldMint => (
                rgb(0.03, 0.54, 0.37), // High-contrast Emerald for WCAG AA button labels
                rgb(0.05, 0.64, 0.44),
                rgb(0.02, 0.42, 0.28),
            ),
            ColorScheme::CrimsonSunset => (
                rgb(0.82, 0.14, 0.27), // Accessible Rose 600 (>= 4.8:1 contrast with white)
                rgb(0.92, 0.22, 0.35),
                rgb(0.68, 0.10, 0.21),
            ),
            ColorScheme::SolarAmber => (
                rgb(0.96, 0.62, 0.04), // Amber 500 #F59E0B
                rgb(0.98, 0.72, 0.15),
                rgb(0.82, 0.50, 0.02),
            ),
            ColorScheme::ArcticAurora => (
                rgb(0.00, 0.831, 0.667), // #00d4aa
                rgb(0.12, 0.92, 0.76),
                rgb(0.00, 0.68, 0.54),
            ),
        };

        let (
            bg_primary,
            bg_sidebar,
            bg_card,
            bg_card_hover,
            border,
            border_hover,
            border_accent,
            btn_secondary,
            btn_secondary_hover,
            btn_secondary_press,
        ) = match style {
            UiThemeStyle::Glassy => (
                rgb(0.012, 0.015, 0.024),
                rgba(0.018, 0.022, 0.035, 0.88),
                rgba(0.032, 0.040, 0.062, 0.70),
                rgba(0.052, 0.066, 0.098, 0.82),
                accent.alpha(0.30),
                accent.alpha(0.60),
                accent.alpha(0.80),
                rgba(0.06, 0.08, 0.12, 0.75),
                rgba(0.10, 0.13, 0.19, 0.85),
                rgba(0.04, 0.05, 0.08, 0.90),
            ),
            UiThemeStyle::Minimalistic => (
                rgb(0.008, 0.008, 0.010),
                rgb(0.014, 0.014, 0.016),
                rgb(0.022, 0.022, 0.026),
                rgb(0.035, 0.035, 0.042),
                rgba(1.0, 1.0, 1.0, 0.12),
                rgba(1.0, 1.0, 1.0, 0.35),
                accent,
                rgb(0.04, 0.04, 0.05),
                rgb(0.07, 0.07, 0.08),
                rgb(0.02, 0.02, 0.03),
            ),
            UiThemeStyle::Classic => (
                rgb(0.040, 0.045, 0.058),
                rgb(0.050, 0.058, 0.075),
                rgb(0.065, 0.074, 0.095),
                rgb(0.085, 0.096, 0.122),
                rgba(1.0, 1.0, 1.0, 0.24),
                rgba(1.0, 1.0, 1.0, 0.48),
                accent,
                rgb(0.08, 0.09, 0.12),
                rgb(0.12, 0.14, 0.18),
                rgb(0.05, 0.06, 0.08),
            ),
            UiThemeStyle::Compact => (
                rgb(0.016, 0.019, 0.028),
                rgb(0.022, 0.026, 0.038),
                rgb(0.032, 0.040, 0.058),
                rgb(0.052, 0.062, 0.088),
                rgba(1.0, 1.0, 1.0, 0.18),
                rgba(1.0, 1.0, 1.0, 0.35),
                accent.alpha(0.60),
                rgb(0.065, 0.078, 0.110),
                rgb(0.095, 0.115, 0.165),
                rgb(0.048, 0.058, 0.082),
            ),
            UiThemeStyle::Materialistic => (
                rgb(0.024, 0.027, 0.038),
                rgb(0.032, 0.037, 0.052),
                rgb(0.044, 0.050, 0.070),
                rgb(0.062, 0.070, 0.098),
                accent.alpha(0.22),
                accent.alpha(0.44),
                accent.alpha(0.70),
                rgb(0.07, 0.085, 0.12),
                rgb(0.10, 0.12, 0.17),
                rgb(0.05, 0.06, 0.09),
            ),
        };

        let (bg_primary, bg_card, bg_sidebar, bg_card_hover) =
            if scheme == ColorScheme::ArcticAurora {
                (
                    rgb(0.039, 0.086, 0.157), // #0a1628
                    rgb(0.071, 0.122, 0.200), // #121f33
                    rgba(0.051, 0.102, 0.180, 0.90),
                    rgb(0.102, 0.165, 0.260),
                )
            } else {
                (bg_primary, bg_card, bg_sidebar, bg_card_hover)
            };

        let (text_primary, text_secondary, text_muted) = if scheme == ColorScheme::ArcticAurora {
            (
                rgb(0.878, 0.941, 1.0), // #e0f0ff
                rgb(0.70, 0.82, 0.92),
                rgb(0.48, 0.60, 0.72),
            )
        } else if style == UiThemeStyle::Compact {
            // Enhanced contrast for Compact high-density mode
            (
                rgb(0.98, 0.99, 1.00),
                rgb(0.82, 0.85, 0.90),
                rgb(0.62, 0.65, 0.72),
            )
        } else {
            (
                rgb(0.95, 0.96, 0.98),
                rgb(0.70, 0.72, 0.76),
                rgb(0.48, 0.50, 0.55),
            )
        };

        Self {
            theme_style: style,
            card_radius: style.card_radius(),
            btn_radius: style.btn_radius(),
            input_radius: style.input_radius(),
            badge_radius: style.badge_radius(),
            card_padding: style.card_padding(),
            border_width: style.border_width(),
            sidebar_width: style.sidebar_width(),
            motion: style.motion(),

            bg_primary,
            bg_sidebar,
            bg_card,
            bg_card_hover,
            border,
            border_hover,
            border_accent,

            accent,
            accent_hover,
            accent_press,

            btn_secondary,
            btn_secondary_hover,
            btn_secondary_press,

            sparkline_cpu: if scheme == ColorScheme::ArcticAurora {
                rgb(0.00, 0.831, 0.667)
            } else {
                rgb(0.20, 0.70, 0.95)
            },
            sparkline_ram: if scheme == ColorScheme::ArcticAurora {
                rgb(0.40, 0.85, 0.95)
            } else {
                rgb(0.85, 0.45, 0.95)
            },
            sparkline_bg: rgba(1.0, 1.0, 1.0, 0.04),

            text_primary,
            text_secondary,
            text_muted,

            success: rgb(0.09, 0.56, 0.30),
            warning: rgb(0.95, 0.65, 0.15),
            danger: rgb(0.84, 0.18, 0.22),
            neutral_badge: rgb(0.25, 0.27, 0.32),

            is_dark: true,
        }
    }

    fn build_light(scheme: ColorScheme, style: UiThemeStyle) -> Self {
        let (accent, accent_hover, accent_press) = match scheme {
            ColorScheme::CyberIndigo => (
                rgb(0.11, 0.31, 0.85), // Deep Blue #1D4ED8 (WCAG AAA on white)
                rgb(0.15, 0.38, 0.95),
                rgb(0.08, 0.22, 0.68),
            ),
            ColorScheme::MidnightStealth => (
                rgb(0.20, 0.25, 0.33), // Slate 700 #334155
                rgb(0.28, 0.34, 0.44),
                rgb(0.12, 0.16, 0.22),
            ),
            ColorScheme::EmeraldMint => (
                rgb(0.02, 0.47, 0.34), // Emerald 700 #047857 (>5:1 contrast)
                rgb(0.03, 0.58, 0.42),
                rgb(0.01, 0.35, 0.25),
            ),
            ColorScheme::CrimsonSunset => (
                rgb(0.75, 0.07, 0.24), // Rose 700 #BE123C (>6:1 contrast)
                rgb(0.88, 0.12, 0.30),
                rgb(0.58, 0.04, 0.18),
            ),
            ColorScheme::SolarAmber => (
                rgb(0.85, 0.45, 0.02),
                rgb(0.95, 0.55, 0.10),
                rgb(0.70, 0.35, 0.01),
            ),
            ColorScheme::ArcticAurora => (
                rgb(0.00, 0.659, 0.541), // #00a88a
                rgb(0.05, 0.76, 0.62),
                rgb(0.00, 0.52, 0.42),
            ),
        };

        let (
            bg_primary,
            bg_sidebar,
            bg_card,
            bg_card_hover,
            border,
            border_hover,
            border_accent,
            btn_secondary,
            btn_secondary_hover,
            btn_secondary_press,
        ) = match style {
            UiThemeStyle::Glassy => (
                rgb(0.95, 0.96, 0.98),
                rgba(0.93, 0.95, 0.97, 0.88),
                rgba(1.0, 1.0, 1.0, 0.78),
                rgba(0.94, 0.96, 1.0, 0.90),
                accent.alpha(0.28),
                accent.alpha(0.55),
                accent.alpha(0.75),
                rgba(0.90, 0.93, 0.97, 0.85),
                rgba(0.84, 0.88, 0.93, 0.92),
                rgba(0.78, 0.82, 0.88, 0.95),
            ),
            UiThemeStyle::Minimalistic => (
                rgb(0.99, 0.99, 1.00),
                rgb(0.96, 0.96, 0.97),
                rgb(1.0, 1.0, 1.0),
                rgb(0.93, 0.93, 0.95),
                rgba(0.0, 0.0, 0.0, 0.14),
                rgba(0.0, 0.0, 0.0, 0.40),
                accent,
                rgb(0.92, 0.92, 0.94),
                rgb(0.85, 0.85, 0.88),
                rgb(0.78, 0.78, 0.82),
            ),
            UiThemeStyle::Classic => (
                rgb(0.87, 0.88, 0.90),
                rgb(0.83, 0.84, 0.87),
                rgb(0.93, 0.94, 0.96),
                rgb(0.85, 0.87, 0.90),
                rgba(0.0, 0.0, 0.0, 0.35),
                rgba(0.0, 0.0, 0.0, 0.65),
                accent,
                rgb(0.82, 0.83, 0.86),
                rgb(0.75, 0.77, 0.80),
                rgb(0.68, 0.70, 0.74),
            ),
            UiThemeStyle::Compact => (
                rgb(0.96, 0.97, 0.98),
                rgb(0.92, 0.94, 0.96),
                rgb(1.0, 1.0, 1.0),
                rgb(0.90, 0.93, 0.96),
                rgba(0.0, 0.0, 0.0, 0.20),
                rgba(0.0, 0.0, 0.0, 0.38),
                accent.alpha(0.55),
                rgb(0.89, 0.92, 0.95),
                rgb(0.83, 0.86, 0.90),
                rgb(0.76, 0.80, 0.85),
            ),
            UiThemeStyle::Materialistic => (
                rgb(0.96, 0.97, 1.00),
                rgb(0.92, 0.94, 0.98),
                rgb(0.99, 1.0, 1.0),
                rgb(0.91, 0.93, 0.98),
                accent.alpha(0.20),
                accent.alpha(0.42),
                accent.alpha(0.65),
                rgb(0.90, 0.92, 0.97),
                rgb(0.84, 0.87, 0.93),
                rgb(0.77, 0.81, 0.88),
            ),
        };

        let (bg_primary, bg_card, bg_sidebar, bg_card_hover) =
            if scheme == ColorScheme::ArcticAurora {
                (
                    rgb(0.941, 0.973, 1.0), // #f0f8ff
                    rgb(1.0, 1.0, 1.0),     // #ffffff
                    rgba(0.92, 0.96, 0.99, 0.92),
                    rgb(0.91, 0.95, 0.98),
                )
            } else {
                (bg_primary, bg_card, bg_sidebar, bg_card_hover)
            };

        let (text_primary, text_secondary, text_muted) = if scheme == ColorScheme::ArcticAurora {
            (
                rgb(0.102, 0.165, 0.227), // #1a2a3a
                rgb(0.20, 0.30, 0.40),
                rgb(0.32, 0.42, 0.52),
            )
        } else if style == UiThemeStyle::Compact {
            (
                rgb(0.03, 0.05, 0.10),
                rgb(0.12, 0.16, 0.22),
                rgb(0.22, 0.26, 0.34),
            )
        } else {
            (
                rgb(0.06, 0.09, 0.16),
                rgb(0.20, 0.25, 0.33),
                rgb(0.31, 0.37, 0.46),
            )
        };

        Self {
            theme_style: style,
            card_radius: style.card_radius(),
            btn_radius: style.btn_radius(),
            input_radius: style.input_radius(),
            badge_radius: style.badge_radius(),
            card_padding: style.card_padding(),
            border_width: style.border_width(),
            sidebar_width: style.sidebar_width(),
            motion: style.motion(),

            bg_primary,
            bg_sidebar,
            bg_card,
            bg_card_hover,
            border,
            border_hover,
            border_accent,

            accent,
            accent_hover,
            accent_press,

            btn_secondary,
            btn_secondary_hover,
            btn_secondary_press,

            sparkline_cpu: if scheme == ColorScheme::ArcticAurora {
                rgb(0.00, 0.659, 0.541)
            } else {
                rgb(0.10, 0.55, 0.85)
            },
            sparkline_ram: if scheme == ColorScheme::ArcticAurora {
                rgb(0.20, 0.60, 0.80)
            } else {
                rgb(0.65, 0.25, 0.80)
            },
            sparkline_bg: rgba(0.0, 0.0, 0.0, 0.04),

            text_primary,
            text_secondary,
            text_muted,

            success: rgb(0.02, 0.47, 0.34),
            warning: rgb(0.70, 0.33, 0.04), // Dark Amber #B45309
            danger: rgb(0.73, 0.11, 0.11),  // Dark Red #B91C1C
            neutral_badge: rgb(0.88, 0.90, 0.94),

            is_dark: false,
        }
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
