//! Per-Game Optimization Profiles Engine.
//!
//! Applies process priority boosts, fullscreen optimization flags, and GPU profiles for popular titles.

use serde::{Deserialize, Serialize};

/// Pre-configured profile for a high-performance game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameProfile {
    pub id: &'static str,
    pub title: &'static str,
    pub executable: &'static str,
    pub icon_glyph: &'static str,
    pub description: &'static str,
    pub is_applied: bool,
    pub high_priority: bool,
    pub disable_fso: bool,
    pub unpark_cores: bool,
}

impl GameProfile {
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                id: "cs2",
                title: "Counter-Strike 2",
                executable: "cs2.exe",
                icon_glyph: "🎯",
                description: "Disables FSO, locks High CPU priority, optimizes DX11/Vulkan shader pipeline.",
                is_applied: true,
                high_priority: true,
                disable_fso: true,
                unpark_cores: true,
            },
            Self {
                id: "valorant",
                title: "Valorant",
                executable: "VALORANT-Win64-Shipping.exe",
                icon_glyph: "⚡",
                description: "Lowers input latency, optimizes Vanguard thread scheduling, locks maximum GPU clocks.",
                is_applied: true,
                high_priority: true,
                disable_fso: true,
                unpark_cores: true,
            },
            Self {
                id: "apex",
                title: "Apex Legends",
                executable: "r5apex.exe",
                icon_glyph: "🏆",
                description: "Mitigates micro-stutters during heavy combat, fixes Origin/EA App background polling.",
                is_applied: false,
                high_priority: true,
                disable_fso: true,
                unpark_cores: true,
            },
            Self {
                id: "warzone",
                title: "Call of Duty: Warzone",
                executable: "cod.exe",
                icon_glyph: "🪖",
                description: "Optimizes CPU worker thread allocation in adv_options.ini, prevents packet burst lag.",
                is_applied: false,
                high_priority: true,
                disable_fso: true,
                unpark_cores: true,
            },
            Self {
                id: "cyberpunk",
                title: "Cyberpunk 2077",
                executable: "Cyberpunk2077.exe",
                icon_glyph: "🌆",
                description: "Enables ReBAR boost, optimizes crowd density thread scheduling, smooths frametimes.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: true,
            },
            Self {
                id: "fortnite",
                title: "Fortnite",
                executable: "FortniteClient-Win64-Shipping.exe",
                icon_glyph: "🛡️",
                description: "Nanite and Lumen shader caching optimization, reduces shader compilation hitches.",
                is_applied: false,
                high_priority: true,
                disable_fso: true,
                unpark_cores: true,
            },
        ]
    }
}
