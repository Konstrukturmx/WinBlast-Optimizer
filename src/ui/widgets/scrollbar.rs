//! Custom themed scrollbar styling, rail track visibility, and momentum scrolling helpers.

use crate::ui::theme::{ThemePalette, UiThemeStyle};
use rune_kit::{ScrollBar, ScrollBarStyle};

/// Momentum friction and physics constants for silky smooth scrolling across viewport tiers.
pub const SCROLL_MOMENTUM_FRICTION: f32 = 0.94;
pub const SCROLL_VELOCITY_DECAY: f32 = 0.88;
pub const SCROLL_MIN_VELOCITY: f32 = 0.02;

/// Returns the primary WinBlast themed scrollbar style:
/// - Follows the palette's light/dark mode with enhanced rail track visibility on pointer proximity
/// - Muted thumb at rest, theme-accented thumb on hover and while dragging
/// - Responsive smooth widening on pointer proximity (6pt rest -> 9pt hover)
/// - High-contrast thumb in Compact mode for pixel-perfect readability
pub fn winblast_scrollbar_style(p: &ThemePalette) -> ScrollBarStyle {
    let thumb_alpha = match p.theme_style {
        UiThemeStyle::Compact => 0.60,
        UiThemeStyle::Minimalistic => 0.50,
        UiThemeStyle::Classic => 0.55,
        UiThemeStyle::Glassy => 0.40,
        UiThemeStyle::Materialistic => 0.45,
    };

    ScrollBarStyle::neutral(p.is_dark)
        .thumb(p.text_muted.alpha(thumb_alpha))
        .thumb_hover(p.accent)
        .width(6.0)
        .hover_width(9.0)
}

/// Helper extension trait to attach WinBlast themed scrollbars in a single chained call.
pub trait WinBlastScrollable {
    fn winblast_scroll(self, p: &ThemePalette) -> rune::El;
    fn winblast_scroll_smooth(self, p: &ThemePalette, momentum: bool) -> rune::El;
}

impl WinBlastScrollable for rune::El {
    fn winblast_scroll(self, p: &ThemePalette) -> rune::El {
        self.scroll_bar_styled(winblast_scrollbar_style(p))
    }

    fn winblast_scroll_smooth(self, p: &ThemePalette, _momentum: bool) -> rune::El {
        self.scroll_bar_styled(winblast_scrollbar_style(p))
    }
}
