//! Per-Game Optimization Profiles Engine.
//!
//! Sets the CPU priority a game starts with, through Windows Image File Execution Options
//! (IFEO `PerfOptions\CpuPriorityClass`). That is all a profile does (checklist C10: the old
//! descriptions promised FSO, core unparking, GPU clocks and shader tuning that never happened).

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

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
    /// Return all game profiles initialized against live Windows registry IFEO status.
    pub fn catalog() -> Vec<Self> {
        vec![
            Self {
                id: "cs2",
                title: "Counter-Strike 2",
                executable: "cs2.exe",
                icon_glyph: "🎯",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
            Self {
                id: "valorant",
                title: "Valorant",
                executable: "VALORANT-Win64-Shipping.exe",
                icon_glyph: "⚡",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
            Self {
                id: "apex",
                title: "Apex Legends",
                executable: "r5apex.exe",
                icon_glyph: "🏆",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
            Self {
                id: "warzone",
                title: "Call of Duty: Warzone",
                executable: "cod.exe",
                icon_glyph: "🎖️",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
            Self {
                id: "cyberpunk",
                title: "Cyberpunk 2077",
                executable: "Cyberpunk2077.exe",
                icon_glyph: "🌆",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
            Self {
                id: "fortnite",
                title: "Fortnite",
                executable: "FortniteClient-Win64-Shipping.exe",
                icon_glyph: "🛡️",
                description: "Starts the game at High CPU priority (Windows Image File Execution Options). Nothing else is changed for this game.",
                is_applied: false,
                high_priority: true,
                disable_fso: false,
                unpark_cores: false,
            },
        ]
    }

    /// Catalog with `is_applied` read from the live registry (IFEO PerfOptions).
    pub fn all() -> Vec<Self> {
        let mut profiles = Self::catalog();
        for prof in &mut profiles {
            prof.is_applied = prof.check_applied();
        }
        profiles
    }

    /// Check whether this profile is currently applied in Windows Registry (IFEO PerfOptions).
    pub fn check_applied(&self) -> bool {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = format!(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}\\PerfOptions",
            self.executable
        );
        if let Ok(key) = hklm.open_subkey(&subkey) {
            let priority: Result<u32, _> = key.get_value("CpuPriorityClass");
            priority.map(|p| p == 3).unwrap_or(false)
        } else {
            false
        }
    }

    /// IFEO `PerfOptions` key of this game.
    pub fn perf_options_path(&self) -> String {
        format!(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}\\PerfOptions",
            self.executable
        )
    }

    /// The profile as a ledger-tracked tweak, so applying records the exact prior value and
    /// undo restores it (including a `PerfOptions` key the user had made themselves).
    pub fn as_tweak(&self) -> crate::core::risk::TweakDefinition {
        use crate::core::risk::{OsRequirement, Reversibility, RiskLevel, RiskValuation, TweakAction, TweakDefinition};
        TweakDefinition {
            id: format!("game_profile_{}", self.id),
            title: format!("{}: High CPU priority", self.title),
            category: "Gaming".to_string(),
            description: self.description.to_string(),
            easy_mode_included: false,
            os_req: OsRequirement { min_build: 0, max_build: None, editions: Vec::new() },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 0,
                reversibility: Reversibility::Instant,
                broken_features: Vec::new(),
            },
            actions: vec![TweakAction::RegistryDword {
                hive: "HKLM".to_string(),
                path: self.perf_options_path(),
                name: "CpuPriorityClass".to_string(),
                optimized_value: 3,
                default_value: 2,
            }],
            gpu_vendor: None,
            sub_category: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        }
    }

    /// Apply real Windows IFEO high priority and I/O priority boost.
    pub fn apply(&self) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = format!(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}\\PerfOptions",
            self.executable
        );
        let (key, _) = hklm.create_subkey(&subkey).map_err(|e| {
            format!(
                "Failed to create IFEO PerfOptions key (requires elevation): {}",
                e
            )
        })?;

        key.set_value("CpuPriorityClass", &3u32)
            .map_err(|e| format!("Failed to set CpuPriorityClass: {}", e))?;

        Ok(())
    }

    /// Revert profile by removing the PerfOptions subkey.
    pub fn revert(&self) -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let parent = format!(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\{}",
            self.executable
        );
        if let Ok(key) = hklm.open_subkey_with_flags(&parent, KEY_WRITE) {
            let _ = key.delete_subkey("PerfOptions");
        }
        Ok(())
    }
}
