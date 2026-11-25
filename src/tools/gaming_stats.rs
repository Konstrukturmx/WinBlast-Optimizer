//! In-game monitoring: detects a running supported game and reports what can be measured.

use serde::{Deserialize, Serialize};

// The former "stock vs optimized FPS" catalog (GameFpsComparison) was removed: its numbers were
// not measured on any machine (AGENTS.md rule 2, checklist C9). Frame rates can only be shown once
// frame-time capture (PresentMon / ETW) exists.

/// Active in-game telemetry tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveGameTelemetry {
    pub active_game_detected: Option<String>,
    pub frame_pacing_rating: u32, // 0..100%
    pub background_cpu_impact_pct: f32,
    pub micro_stutters_per_min: u32,
    pub directx_version: String,
}

impl Default for LiveGameTelemetry {
    fn default() -> Self {
        Self {
            // Frame pacing / stutter capture is not implemented (needs ETW PresentMon); v1.6.1
            // displayed these invented constants as live telemetry. Zero/empty = not measured.
            active_game_detected: None,
            frame_pacing_rating: 0,
            background_cpu_impact_pct: 0.0,
            micro_stutters_per_min: 0,
            directx_version: String::new(),
        }
    }
}

impl LiveGameTelemetry {
    /// Probe currently running processes to detect active games.
    pub fn probe() -> Self {
        let mut telemetry = Self::default();

        let known_games = [
            ("cs2.exe", "Counter-Strike 2"),
            ("VALORANT-Win64-Shipping.exe", "Valorant"),
            ("r5apex.exe", "Apex Legends"),
            ("cod.exe", "Call of Duty: Warzone"),
            ("Cyberpunk2077.exe", "Cyberpunk 2077"),
            ("FortniteClient-Win64-Shipping.exe", "Fortnite"),
        ];

        if let Ok(output) = crate::core::process::SilentCommand::new("tasklist")
            .args(["/NH", "/FO", "CSV"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            for (exe, title) in known_games {
                if text.to_lowercase().contains(&exe.to_lowercase()) {
                    telemetry.active_game_detected = Some(title.to_string());
                    break;
                }
            }
        }

        telemetry
    }
}
