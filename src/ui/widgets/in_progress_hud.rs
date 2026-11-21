//! Centered "In Progress" HUD overlay widget for WinBlast0.
//!
//! Fluent / Windows 11-style glassmorphic modal overlay featuring:
//! - 10-dot clockwise cubic-eased orbital radar spinner with glowing trail.
//! - Pulsing center lightning badge (`⚡`) driven by `(elapsed_secs * 3.0).sin()`.
//! - Smooth indeterminate sliding progress bar with stretch-and-glide easing.
//! - Clean typography with elapsed time display and word-wrapped status text.
//!
//! Layout invariants:
//! - `El::pin(left, top, right, bottom)` always maps screen `x` to `left` and `y` to `top`.
//! - All orbital dots, halos, and the pulsing core are positioned relative to
//!   `(SPINNER_CENTER, SPINNER_CENTER)` so scaling never introduces positional jitter.

use std::f64::consts::{FRAC_PI_2, TAU};

use rune::color::Color;
use rune::{col, row, text, El};

use crate::ui::theme::ThemePalette;

const CARD_WIDTH: f32 = 360.0;
const SPINNER_SIZE: f32 = 88.0;
const SPINNER_CENTER: f32 = SPINNER_SIZE * 0.5;
const ORBIT_RADIUS: f32 = 34.0;

const DOT_COUNT: usize = 10;
const ORBIT_CYCLE_SECS: f64 = 1.85;
const ORBIT_DRIFT_SECS: f64 = 5.2;
const DOT_PHASE_STEP: f64 = 0.064;

const TRACK_WIDTH: f32 = 280.0;
const TRACK_HEIGHT: f32 = 4.0;
const PROGRESS_CYCLE_SECS: f64 = 1.65;

const MAX_STATUS_CHARS_PER_LINE: usize = 42;
const MAX_STATUS_LINES: usize = 2;

/// Renders a Fluent / Windows 11-style glassmorphic "In Progress" HUD modal.
///
/// Displays a clockwise cubic-eased orbital radar spinner, a pulsing central
/// lightning badge driven by `(elapsed_secs * 3.0).sin()`, crisp action/timer
/// typography, and a smooth indeterminate sliding progress pill.
pub fn in_progress_hud(now_secs: f64, status_message: &str, palette: &ThemePalette) -> El {
    let elapsed = sanitize_elapsed(now_secs);
    let display_action = if status_message.trim().is_empty() {
        "Optimizing Windows..."
    } else {
        status_message.trim()
    };

    let spinner = build_orbital_spinner(elapsed, palette);
    let header_block = build_typography_block(display_action, elapsed, palette);
    let progress_track = build_indeterminate_track(elapsed, palette);

    let background_btn = row()
        .center()
        .pt(4.0)
        .child(crate::ui::widgets::button::action_button(
            "Run in Background",
            Color::TRANSPARENT,
            palette.text_muted,
            palette,
            |s| {
                s.dismiss_progress_hud = true;
            },
        ));

    // Floating Acrylic/Mica-style card
    let modal_card = col()
        .w(CARD_WIDTH)
        .padding(26.0)
        .gap(18.0)
        .center()
        .fill(palette.bg_card)
        .radius(16.0)
        .border(1.0, palette.border)
        .child(spinner)
        .child(header_block)
        .child(progress_track)
        .child(background_btn);

    // Full-viewport translucent modal overlay centered on the card
    col()
        .key("in_progress_hud_overlay")
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        .fill(Color::BLACK.alpha(0.50))
        .modal(true, |s: &mut crate::app::AppState| {
            s.dismiss_progress_hud = true
        })
        .center()
        .child(modal_card)
}

/// Builds the 10-dot clockwise cubic-eased orbital spinner and pulsing core badge.
fn build_orbital_spinner(elapsed: f64, palette: &ThemePalette) -> El {
    let mut canvas = col().w(SPINNER_SIZE).h(SPINNER_SIZE);

    // 1. Subtle recessed radar track ring
    let ring_diameter = ORBIT_RADIUS * 2.0;
    let ring_offset = SPINNER_CENTER - ORBIT_RADIUS;
    canvas = canvas.child(
        col()
            .key("radar_track_ring")
            .w(ring_diameter)
            .h(ring_diameter)
            .radius(ring_diameter * 0.5)
            .border(1.0, palette.border.alpha(0.24))
            .pin(Some(ring_offset), Some(ring_offset), None, None),
    );

    // 2. Clockwise orbital dots with cubic ease-in-out and alpha gradient [0.15, 1.0]
    let base_drift = (elapsed / ORBIT_DRIFT_SECS) * TAU;
    let denom = (DOT_COUNT.saturating_sub(1)).max(1) as f32;

    // Render tail-to-head so brighter leading dots stack cleanly on top
    for i in (0..DOT_COUNT).rev() {
        let lead_factor = 1.0 - (i as f32 / denom);
        let alpha = 0.15 + 0.85 * lead_factor;
        let dot_size = 3.2 + 3.6 * lead_factor;

        let raw_t = ((elapsed / ORBIT_CYCLE_SECS) - (i as f64) * DOT_PHASE_STEP).rem_euclid(1.0);
        let eased_t = ease_in_out_cubic(raw_t);
        let angle = -FRAC_PI_2 + eased_t * TAU + base_drift;

        let center_x = SPINNER_CENTER + ORBIT_RADIUS * (angle.cos() as f32);
        let center_y = SPINNER_CENTER + ORBIT_RADIUS * (angle.sin() as f32);

        // Soft luminous halo behind the top 4 leading dots
        if i < 4 {
            let halo_size = dot_size + 5.5;
            let halo_left = center_x - halo_size * 0.5;
            let halo_top = center_y - halo_size * 0.5;

            canvas = canvas.child(
                col()
                    .key(format!("radar_halo_{}", i))
                    .w(halo_size)
                    .h(halo_size)
                    .radius(halo_size * 0.5)
                    .fill(palette.accent.alpha(alpha * 0.22))
                    .pin(Some(halo_left), Some(halo_top), None, None),
            );
        }

        let dot_left = center_x - dot_size * 0.5;
        let dot_top = center_y - dot_size * 0.5;

        canvas = canvas.child(
            col()
                .key(format!("radar_dot_{}", i))
                .w(dot_size)
                .h(dot_size)
                .radius(dot_size * 0.5)
                .fill(palette.accent.alpha(alpha))
                .pin(Some(dot_left), Some(dot_top), None, None),
        );
    }

    // 3. Gently pulsing center icon badge driven by (elapsed_secs * 3.0).sin()
    let pulse = ((elapsed * 3.0).sin() as f32) * 0.5 + 0.5; // [0.0, 1.0]

    let core_halo_size = 34.0 + 7.5 * pulse;
    let core_halo_offset = SPINNER_CENTER - core_halo_size * 0.5;
    canvas = canvas.child(
        col()
            .key("core_halo")
            .w(core_halo_size)
            .h(core_halo_size)
            .radius(core_halo_size * 0.5)
            .fill(palette.accent.alpha(0.08 + 0.14 * pulse))
            .pin(Some(core_halo_offset), Some(core_halo_offset), None, None),
    );

    let badge_size = 28.0 + 2.4 * pulse;
    let badge_offset = SPINNER_CENTER - badge_size * 0.5;
    canvas = canvas.child(
        col()
            .key("core_badge")
            .w(badge_size)
            .h(badge_size)
            .radius(badge_size * 0.5)
            .center()
            .fill(palette.accent.alpha(0.14 + 0.10 * pulse))
            .border(1.0, palette.accent.alpha(0.32 + 0.28 * pulse))
            .pin(Some(badge_offset), Some(badge_offset), None, None)
            .child(text("⚡").color(palette.accent.alpha(0.78 + 0.22 * pulse))),
    );

    canvas
}

/// Builds the action title and formatted elapsed timer pill.
fn build_typography_block(action_name: &str, elapsed: f64, palette: &ThemePalette) -> El {
    let pulse_dot_alpha = 0.45 + 0.55 * (((elapsed * 3.0).sin() as f32) * 0.5 + 0.5);
    let elapsed_label = format_elapsed(elapsed);

    let status_row = row()
        .gap(6.0)
        .center()
        .child(
            col()
                .key("pulse_indicator_dot")
                .w(6.0)
                .h(6.0)
                .radius(3.0)
                .fill(palette.accent.alpha(pulse_dot_alpha)),
        )
        .child(
            text("IN PROGRESS")
                .font(rune::tok::type_::LABEL)
                .color(palette.text_secondary),
        )
        .child(
            text("•")
                .font(rune::tok::type_::LABEL)
                .color(palette.text_secondary.alpha(0.6)),
        )
        .child(
            text(elapsed_label)
                .font(rune::tok::type_::LABEL)
                .color(palette.text_secondary),
        );

    let lines = wrap_and_truncate_status(action_name, MAX_STATUS_CHARS_PER_LINE, MAX_STATUS_LINES);

    let mut title_col = col().gap(2.0).center();
    for (idx, line) in lines.iter().enumerate() {
        title_col = title_col.child(
            text(line)
                .font(if idx == 0 {
                    rune::tok::type_::TITLE
                } else {
                    rune::tok::type_::BODY
                })
                .color(palette.text_primary),
        );
    }

    col().gap(6.0).center().child(title_col).child(status_row)
}

/// Builds a Fluent-style indeterminate progress track with a cubic-eased sliding pill.
fn build_indeterminate_track(elapsed: f64, palette: &ThemePalette) -> El {
    let t = (elapsed / PROGRESS_CYCLE_SECS).rem_euclid(1.0);

    // Leading edge starts immediately; trailing edge follows after a 18% delay
    let head = ease_in_out_cubic((t / 0.82).min(1.0)) as f32;
    let tail = ease_in_out_cubic(((t - 0.18).max(0.0) / 0.82).min(1.0)) as f32;

    let pill_left = (tail * TRACK_WIDTH).clamp(0.0, TRACK_WIDTH);
    let pill_right = (head * TRACK_WIDTH).clamp(pill_left, TRACK_WIDTH);
    let pill_width = (pill_right - pill_left).max(0.0);

    let mut track = col()
        .key("indeterminate_track")
        .w(TRACK_WIDTH)
        .h(TRACK_HEIGHT)
        .radius(TRACK_HEIGHT * 0.5)
        .fill(palette.border.alpha(0.3));

    if pill_width > 0.5 {
        // Subtle glow under the active pill
        track = track.child(
            col()
                .key("pill_glow")
                .w(pill_width)
                .h(TRACK_HEIGHT)
                .radius(TRACK_HEIGHT * 0.5)
                .fill(palette.accent.alpha(0.32))
                .pin(Some(pill_left), Some(0.0), None, Some(0.0)),
        );

        // Crisp foreground accent pill
        track = track.child(
            col()
                .key("pill_fg")
                .w(pill_width)
                .h(TRACK_HEIGHT)
                .radius(TRACK_HEIGHT * 0.5)
                .fill(palette.accent)
                .pin(Some(pill_left), Some(0.0), None, Some(0.0)),
        );
    }

    track
}

/// Standard C1-continuous cubic ease-in-out over `[0.0, 1.0]`.
#[inline]
fn ease_in_out_cubic(t: f64) -> f64 {
    let x = t.clamp(0.0, 1.0);
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powi(3) * 0.5
    }
}

/// Guards against `NaN`, infinity, or negative deltas from monotonic clock jitter.
#[inline]
fn sanitize_elapsed(elapsed_secs: f64) -> f64 {
    if elapsed_secs.is_finite() && elapsed_secs > 0.0 {
        elapsed_secs
    } else {
        0.0
    }
}

/// Formats elapsed duration cleanly (`1.2s`, or `1m 04.2s` for long scans).
fn format_elapsed(elapsed_secs: f64) -> String {
    if elapsed_secs < 60.0 {
        format!("{elapsed_secs:.1}s")
    } else {
        let mins = (elapsed_secs / 60.0).floor() as u64;
        let secs = elapsed_secs % 60.0;
        format!("{mins}m {secs:04.1}s")
    }
}

/// Word-wraps and truncates a status message to at most `max_lines` lines of
/// `max_chars_per_line` length each, appending an ellipsis on overflow.
pub fn wrap_and_truncate_status(
    raw: &str,
    max_chars_per_line: usize,
    max_lines: usize,
) -> Vec<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return vec!["Optimizing Windows...".to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in trimmed.split_whitespace() {
        let candidate_len = if current.is_empty() {
            word.chars().count()
        } else {
            current.chars().count() + 1 + word.chars().count()
        };

        if candidate_len <= max_chars_per_line {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        } else {
            if !current.is_empty() {
                lines.push(current);
                current = String::new();
                if lines.len() == max_lines {
                    break;
                }
            }

            if word.chars().count() > max_chars_per_line {
                let chunk: String = word
                    .chars()
                    .take(max_chars_per_line.saturating_sub(1))
                    .collect();
                lines.push(format!("{}…", chunk));
                if lines.len() == max_lines {
                    break;
                }
            } else {
                current.push_str(word);
            }
        }
    }

    if !current.is_empty() && lines.len() < max_lines {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(trimmed.chars().take(max_chars_per_line).collect());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_and_truncate_status() {
        let s =
            "Scanning Windows Registry and removing telemetry tracking components from the host";
        let lines = wrap_and_truncate_status(s, 30, 2);
        assert!(!lines.is_empty());
        assert!(lines.len() <= 2);
    }

    #[test]
    fn test_cubic_ease_in_out() {
        assert!((ease_in_out_cubic(0.0) - 0.0).abs() < 1e-6);
        assert!((ease_in_out_cubic(0.5) - 0.5).abs() < 1e-6);
        assert!((ease_in_out_cubic(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_format_elapsed() {
        assert_eq!(format_elapsed(0.0), "0.0s");
        assert_eq!(format_elapsed(12.34), "12.3s");
        assert_eq!(format_elapsed(65.0), "1m 05.0s");
    }
}
