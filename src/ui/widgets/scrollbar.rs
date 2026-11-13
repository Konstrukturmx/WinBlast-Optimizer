//! Custom themed scrollbar styling and helpers for WinBlast Optimizer.

use rune_kit::{ScrollBar, ScrollBarStyle};
use crate::ui::theme::ThemePalette;

/// Returns the primary WinBlast themed scrollbar style:
/// - Follows the palette's light/dark mode for its neutral track tokens
/// - Muted thumb at rest, theme-accented thumb on hover and while dragging
/// - Responsive smooth widening on pointer proximity (6pt rest -> 9pt hover)
pub fn winblast_scrollbar_style(p: &ThemePalette) -> ScrollBarStyle {
    ScrollBarStyle::neutral(p.is_dark)
        .thumb(p.text_muted.alpha(0.40))
        .thumb_hover(p.accent)
        .width(6.0)
        .hover_width(9.0)
}

/// Helper extension trait to attach WinBlast themed scrollbars in a single chained call.
pub trait WinBlastScrollable {
    fn winblast_scroll(self, p: &ThemePalette) -> rune::El;
}

impl WinBlastScrollable for rune::El {
    fn winblast_scroll(self, p: &ThemePalette) -> rune::El {
        self.scroll_bar_styled(winblast_scrollbar_style(p))
    }
}
