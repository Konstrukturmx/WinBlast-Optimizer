//! Dynamic theming system for WinBlast0 in Rune.
//!
//! Provides Light & Dark modes paired with 4 distinct WCAG AA compliant color schemes:
//! - Cyber Indigo (Default)
//! - Midnight Stealth
//! - Emerald Mint
//! - Crimson Sunset

use rune::color::Color;
use serde::{Deserialize, Serialize};

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
    pub const fn motion(self) -> rune_motion::Spring {
        match self {
            UiThemeStyle::Minimalistic => rune_motion::Spring::NONE,
            UiThemeStyle::Classic => rune_motion::Spring::new(0.15, 0.0),
            UiThemeStyle::Compact => rune_motion::Spring::SNAPPY,
            UiThemeStyle::Glassy => rune_motion::Spring::BOUNCY,
            UiThemeStyle::Materialistic => rune_motion::Spring::SMOOTH,
        }
    }
}

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
}

impl ColorScheme {
    #[allow(dead_code)]
    pub const ALL: [ColorScheme; 4] = [
        ColorScheme::CyberIndigo,
        ColorScheme::MidnightStealth,
        ColorScheme::EmeraldMint,
        ColorScheme::CrimsonSunset,
    ];

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

/// Dynamic theme palette holding all UI color tokens.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ThemePalette {
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
        match style {
            UiThemeStyle::Glassy => {
                if self.is_dark {
                    rgba(0.024, 0.030, 0.046, 0.72)
                } else {
                    rgba(1.0, 1.0, 1.0, 0.82)
                }
            }
            UiThemeStyle::Minimalistic => {
                if self.is_dark {
                    rgb(0.016, 0.018, 0.026)
                } else {
                    rgb(0.99, 0.99, 1.00)
                }
            }
            UiThemeStyle::Classic => {
                if self.is_dark {
                    rgb(0.042, 0.048, 0.062)
                } else {
                    rgb(0.93, 0.94, 0.96)
                }
            }
            UiThemeStyle::Compact => {
                if self.is_dark {
                    rgb(0.022, 0.026, 0.036)
                } else {
                    rgb(0.96, 0.97, 0.98)
                }
            }
            UiThemeStyle::Materialistic => {
                if self.is_dark {
                    rgb(0.032, 0.038, 0.054)
                } else {
                    rgb(0.95, 0.96, 0.99)
                }
            }
        }
    }

    /// Border stroke color customized for the active UI theme style.
    pub fn border_styled(&self, style: UiThemeStyle) -> Color {
        match style {
            UiThemeStyle::Glassy => self.accent.alpha(0.35),
            UiThemeStyle::Minimalistic => {
                if self.is_dark {
                    rgba(1.0, 1.0, 1.0, 0.12)
                } else {
                    rgba(0.0, 0.0, 0.0, 0.12)
                }
            }
            UiThemeStyle::Classic => {
                if self.is_dark {
                    rgba(1.0, 1.0, 1.0, 0.28)
                } else {
                    rgba(0.0, 0.0, 0.0, 0.30)
                }
            }
            UiThemeStyle::Compact => self.border,
            UiThemeStyle::Materialistic => self.accent.alpha(0.20),
        }
    }

    /// Generate a theme palette given mode and scheme, tuned for WCAG 2.1 AA compliance.
    pub fn new(mode: ThemeMode, scheme: ColorScheme) -> Self {
        match mode {
            ThemeMode::Dark => Self::build_dark(scheme),
            ThemeMode::Light => Self::build_light(scheme),
        }
    }

    fn build_dark(scheme: ColorScheme) -> Self {
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
        };

        Self {
            bg_primary: rgb(0.012, 0.014, 0.022),
            bg_sidebar: rgb(0.018, 0.021, 0.032),
            bg_card: rgb(0.028, 0.033, 0.048),
            bg_card_hover: rgb(0.045, 0.054, 0.078),
            border: rgba(1.0, 1.0, 1.0, 0.08),
            border_hover: rgba(1.0, 1.0, 1.0, 0.20),
            border_accent: accent.alpha(0.50),

            accent,
            accent_hover,
            accent_press,

            btn_secondary: rgb(0.05, 0.065, 0.095),
            btn_secondary_hover: rgb(0.08, 0.10, 0.15),
            btn_secondary_press: rgb(0.035, 0.045, 0.07),

            sparkline_cpu: rgb(0.20, 0.70, 0.95),
            sparkline_ram: rgb(0.85, 0.45, 0.95),
            sparkline_bg: rgba(1.0, 1.0, 1.0, 0.04),

            text_primary: rgb(0.95, 0.96, 0.98),
            text_secondary: rgb(0.70, 0.72, 0.76),
            text_muted: rgb(0.48, 0.50, 0.55),

            success: rgb(0.09, 0.56, 0.30),
            warning: rgb(0.95, 0.65, 0.15),
            danger: rgb(0.84, 0.18, 0.22),
            neutral_badge: rgb(0.25, 0.27, 0.32),

            is_dark: true,
        }
    }

    fn build_light(scheme: ColorScheme) -> Self {
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
        };

        Self {
            bg_primary: rgb(0.97, 0.98, 0.99), // Clean Slate 50
            bg_sidebar: rgb(0.94, 0.95, 0.97), // Slate 100
            bg_card: rgb(1.00, 1.00, 1.00),    // Pure White
            bg_card_hover: rgb(0.91, 0.93, 0.96),
            border: rgba(0.0, 0.0, 0.0, 0.12),
            border_hover: rgba(0.0, 0.0, 0.0, 0.24),
            border_accent: accent.alpha(0.45),

            accent,
            accent_hover,
            accent_press,

            btn_secondary: rgb(0.91, 0.93, 0.96),
            btn_secondary_hover: rgb(0.85, 0.88, 0.92),
            btn_secondary_press: rgb(0.78, 0.82, 0.88),

            sparkline_cpu: rgb(0.10, 0.55, 0.85),
            sparkline_ram: rgb(0.65, 0.25, 0.80),
            sparkline_bg: rgba(0.0, 0.0, 0.0, 0.04),

            text_primary: rgb(0.06, 0.09, 0.16),   // Slate 900
            text_secondary: rgb(0.20, 0.25, 0.33), // Slate 700
            text_muted: rgb(0.31, 0.37, 0.46),     // Slate 600 (WCAG AA on Light surfaces)

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
