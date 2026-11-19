//! Centered "In Progress" HUD overlay widget for WinBlast0.
//!
//! Pure-Rust Rune UI widget featuring a full-screen modal backdrop, an 8-dot
//! circular radar spinner with smooth alpha-eased trailing decay, a cleanly
//! wrapped/truncated status message, and an animated indeterminate progress bar.

use std::f32::consts::{FRAC_PI_2, PI, TAU};

use crate::ui::theme::ThemePalette;
use rune::color::Color;
use rune::{col, row, text, El};

const CARD_WIDTH: f32 = 420.0;
const CARD_PADDING: f32 = 24.0;
const CARD_RADIUS: f32 = 12.0;

const RADAR_CANVAS_SIZE: f32 = 76.0;
const RADAR_ORBIT_RADIUS: f32 = 25.0;
const RADAR_DOT_COUNT: usize = 8;
const RADAR_DOT_MIN_DIAMETER: f32 = 6.0;
const RADAR_DOT_MAX_DIAMETER: f32 = 10.5;

const MINI_BAR_WIDTH: f32 = 260.0;
const MINI_BAR_HEIGHT: f32 = 4.0;
const MINI_BAR_PILL_WIDTH: f32 = 76.0;

const MAX_STATUS_CHARS_PER_LINE: usize = 44;
const MAX_STATUS_LINES: usize = 3;
const DEFAULT_STATUS_MESSAGE: &str = "Executing system operation...";

/// Renders the centered full-screen "In Progress" HUD overlay.
///
/// # Parameters
/// - `now_secs`: Monotonic animation clock in seconds. Non-finite or negative
///   values are clamped safely to `0.0`.
/// - `status_message`: Dynamic description of the active operation. Sanitized,
///   word-wrapped, and truncated cleanly to prevent card overflow.
/// - `palette`: Active WinBlast0 theme color tokens.
pub fn in_progress_hud(
    now_secs: f64,
    status_message: &str,
    palette: &ThemePalette,
) -> El {
    let wrapped_secs = sanitize_time_secs(now_secs);
    let radar = build_circular_radar(wrapped_secs, palette);
    let status_lines = wrap_and_truncate_status(
        status_message,
        MAX_STATUS_CHARS_PER_LINE,
        MAX_STATUS_LINES,
    );
    let mini_progress = build_mini_progress_bar(wrapped_secs, palette);

    let mut status_block = col().gap(4.0).center();
    for line in status_lines {
        status_block = status_block.child(
            text(line)
                .font(rune::tok::type_::LABEL)
                .color(palette.text_secondary),
        );
    }

    let title_row = row().center().child(
        text("⚡ Operation in Progress")
            .font(rune::tok::type_::TITLE)
            .color(palette.text_primary),
    );

    let modal_card = col()
        .w(CARD_WIDTH)
        .padding(CARD_PADDING)
        .radius(CARD_RADIUS)
        .fill(palette.bg_card)
        .border(1.0, palette.border)
        .gap(16.0)
        .center()
        .child(radar)
        .child(title_row)
        .child(status_block)
        .child(mini_progress)
        .child(
            row()
                .center()
                .pt(4.0)
                .child(
                    crate::ui::widgets::button::action_button(
                        "Run in Background",
                        Color::TRANSPARENT,
                        palette.text_muted,
                        palette,
                        |s| {
                            s.dismiss_progress_hud = true;
                        },
                    ),
                ),
        );

    col()
        .key("in_progress_hud_overlay")
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .fill(Color::BLACK.alpha(0.45))
        .modal(true, |s: &mut crate::app::AppState| s.dismiss_progress_hud = true)
        .center()
        .child(modal_card)
}

/// Keeps the animation clock finite, non-negative, and bounded within a 4-second
/// harmonic period so floating-point precision never degrades across long uptimes.
fn sanitize_time_secs(now_secs: f64) -> f64 {
    if now_secs.is_finite() && now_secs >= 0.0 {
        now_secs.rem_euclid(4.0)
    } else {
        0.0
    }
}

/// Builds the 8-dot circular radar indicator with smoothstep alpha interpolation
/// between `palette.accent` (1.0) and `palette.accent.alpha(0.2)`.
fn build_circular_radar(wrapped_secs: f64, palette: &ThemePalette) -> El {
    let phase = wrapped_secs * 4.0;
    let active_idx = (phase as usize) % RADAR_DOT_COUNT;
    let frac = (phase - phase.floor()) as f32;
    let center = RADAR_CANVAS_SIZE * 0.5;

    let mut canvas = col().w(RADAR_CANVAS_SIZE).h(RADAR_CANVAS_SIZE);

    for idx in 0..RADAR_DOT_COUNT {
        let alpha = compute_dot_alpha(idx, active_idx, frac);
        // Normalize alpha back into [0.0, 1.0] to subtly pulse dot diameter.
        let glow_factor = ((alpha - 0.2) / 0.8).clamp(0.0, 1.0);
        let diameter =
            RADAR_DOT_MIN_DIAMETER + (RADAR_DOT_MAX_DIAMETER - RADAR_DOT_MIN_DIAMETER) * glow_factor;
        let radius = diameter * 0.5;

        let angle = (idx as f32 / RADAR_DOT_COUNT as f32) * TAU - FRAC_PI_2;
        let x = (center + RADAR_ORBIT_RADIUS * angle.cos() - radius).max(0.0);
        let y = (center + RADAR_ORBIT_RADIUS * angle.sin() - radius).max(0.0);

        let dot = col()
            .key(format!("radar_dot_{}", idx))
            .w(diameter)
            .h(diameter)
            .radius(radius)
            .fill(palette.accent.alpha(alpha))
            .pin(Some(y), None, None, Some(x));

        canvas = canvas.child(dot);
    }

    canvas
}

/// Computes the smooth alpha value in `[0.2, 1.0]` for dot `idx` given the active
/// head dot `active_idx` and sub-step fraction `frac` in `[0.0, 1.0)`.
fn compute_dot_alpha(idx: usize, active_idx: usize, frac: f32) -> f32 {
    let steps_behind = (active_idx + RADAR_DOT_COUNT - (idx % RADAR_DOT_COUNT)) % RADAR_DOT_COUNT;
    let trail_pos = (steps_behind as f32 + frac.clamp(0.0, 1.0)) % (RADAR_DOT_COUNT as f32);
    let linear_intensity = (1.0 - (trail_pos / RADAR_DOT_COUNT as f32)).clamp(0.0, 1.0);
    let smoothstep = linear_intensity * linear_intensity * (3.0 - 2.0 * linear_intensity);
    0.2 + 0.8 * smoothstep
}

/// Builds a sleek indeterminate sliding progress bar with eased oscillation.
fn build_mini_progress_bar(wrapped_secs: f64, palette: &ThemePalette) -> El {
    // 2-second full sweep cycle with smooth ease-in-out at both track edges.
    let oscillation = 0.5 - 0.5 * ((wrapped_secs as f32) * PI).cos();
    let max_travel = (MINI_BAR_WIDTH - MINI_BAR_PILL_WIDTH).max(0.0);
    let pill_left = max_travel * oscillation.clamp(0.0, 1.0);
    let track_radius = MINI_BAR_HEIGHT * 0.5;

    let sliding_pill = col()
        .key("mini_pill")
        .w(MINI_BAR_PILL_WIDTH)
        .h(MINI_BAR_HEIGHT)
        .radius(track_radius)
        .fill(palette.accent)
        .pin(Some(0.0), None, Some(0.0), Some(pill_left));

    let track = col()
        .key("mini_track")
        .w(MINI_BAR_WIDTH)
        .h(MINI_BAR_HEIGHT)
        .radius(track_radius)
        .fill(palette.accent.alpha(0.2))
        .child(sliding_pill);

    row().center().child(track)
}

/// Sanitizes control characters and wraps/truncates `raw` safely along Unicode
/// scalar boundaries so arbitrary status messages never overflow or panic.
fn wrap_and_truncate_status(
    raw: &str,
    max_chars_per_line: usize,
    max_lines: usize,
) -> Vec<String> {
    if max_chars_per_line == 0 || max_lines == 0 {
        return Vec::new();
    }

    let cleaned: String = raw
        .chars()
        .map(|ch| if ch.is_control() { ' ' } else { ch })
        .collect();

    let words: Vec<&str> = cleaned.split_whitespace().collect();
    if words.is_empty() {
        return vec![DEFAULT_STATUS_MESSAGE.to_string()];
    }

    let mut lines: Vec<String> = Vec::with_capacity(max_lines);
    let mut current_line = String::new();
    let mut current_len: usize = 0;
    let mut truncated = false;

    'outer: for word in words {
        let mut remaining_chars = word.chars().peekable();

        while remaining_chars.peek().is_some() {
            let chunk: String = remaining_chars.by_ref().take(max_chars_per_line).collect();
            let chunk_len = chunk.chars().count();

            if current_len == 0 {
                current_line.push_str(&chunk);
                current_len = chunk_len;
            } else if current_len + 1 + chunk_len <= max_chars_per_line {
                current_line.push(' ');
                current_line.push_str(&chunk);
                current_len += 1 + chunk_len;
            } else {
                lines.push(current_line);
                if lines.len() >= max_lines {
                    truncated = true;
                    current_line = String::new();
                    break 'outer;
                }
                current_line = chunk;
                current_len = chunk_len;
            }
        }
    }

    if !current_line.is_empty() && lines.len() < max_lines {
        lines.push(current_line);
    }

    if truncated {
        if let Some(last_line) = lines.last_mut() {
            let char_count = last_line.chars().count();
            if char_count >= max_chars_per_line {
                let keep = max_chars_per_line.saturating_sub(1);
                *last_line = last_line.chars().take(keep).collect();
            }
            while last_line.ends_with(' ') {
                last_line.pop();
            }
            last_line.push('…');
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_time_handles_edge_values() {
        assert_eq!(sanitize_time_secs(f64::NAN), 0.0);
        assert_eq!(sanitize_time_secs(f64::INFINITY), 0.0);
        assert_eq!(sanitize_time_secs(f64::NEG_INFINITY), 0.0);
        assert_eq!(sanitize_time_secs(-5.0), 0.0);
        assert!((sanitize_time_secs(4.5) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn dot_alpha_stays_within_expected_bounds() {
        for active in 0..RADAR_DOT_COUNT {
            for idx in 0..RADAR_DOT_COUNT {
                let alpha = compute_dot_alpha(idx, active, 0.0);
                assert!((0.2..=1.0).contains(&alpha));
            }
            let head_alpha = compute_dot_alpha(active, active, 0.0);
            assert!((head_alpha - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn status_wrapping_handles_empty_unicode_and_overflow() {
        assert_eq!(
            wrap_and_truncate_status("   \n\t ", 44, 3),
            vec![DEFAULT_STATUS_MESSAGE.to_string()]
        );

        let long_msg = "Scanning NVMe SMART telemetry blocks across PCIe Gen4 controllers and validating firmware descriptors for thermal safety";
        let wrapped = wrap_and_truncate_status(long_msg, 24, 2);
        assert_eq!(wrapped.len(), 2);
        assert!(wrapped.iter().all(|line| line.chars().count() <= 24));
        assert!(wrapped.last().is_some_and(|line| line.ends_with('…')));

        let unicode_token = "🔥".repeat(30);
        let wrapped_unicode = wrap_and_truncate_status(&unicode_token, 10, 2);
        assert_eq!(wrapped_unicode.len(), 2);
        assert!(wrapped_unicode.iter().all(|line| line.chars().count() <= 10));
    }
}
