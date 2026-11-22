//! Persistent user interface preferences (language, theme, zoom, telemetry cadence).
//!
//! Stored as JSON at `%LOCALAPPDATA%\WinBlast0\settings.json` (override with the
//! `WINBLAST_SETTINGS_PATH` environment variable, used by tests). Writes are atomic
//! (temp file + rename) so a crash mid-write can never leave a truncated file, and every
//! field is validated on load so a hand-edited or corrupt file falls back to safe defaults.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::core::i18n::Language;
use crate::core::theme::{ColorScheme, ThemeMode, UiThemeStyle};

/// Minimum / maximum UI zoom factor accepted by the Slint `Scale` global.
pub const MIN_ZOOM: f32 = 0.75;
pub const MAX_ZOOM: f32 = 2.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettings {
    pub language: Language,
    pub theme_mode: ThemeMode,
    pub color_scheme: ColorScheme,
    pub theme_style: UiThemeStyle,
    pub ui_zoom: f32,
    pub telemetry_interval_ms: u64,
    pub sidebar_open: bool,
    pub reduced_motion: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            language: Language::default(),
            theme_mode: ThemeMode::default(),
            color_scheme: ColorScheme::default(),
            theme_style: UiThemeStyle::default(),
            ui_zoom: 1.0,
            telemetry_interval_ms: 1000,
            sidebar_open: true,
            reduced_motion: false,
        }
    }
}

impl UserSettings {
    /// Clamp every numeric field into its supported range (NaN/inf fall back to defaults).
    pub fn sanitized(mut self) -> Self {
        self.ui_zoom = if self.ui_zoom.is_finite() {
            self.ui_zoom.clamp(MIN_ZOOM, MAX_ZOOM)
        } else {
            1.0
        };
        self.telemetry_interval_ms = self.telemetry_interval_ms.clamp(250, 10_000);
        self
    }

    /// Resolve the settings file location.
    pub fn path() -> Option<PathBuf> {
        if let Some(p) = std::env::var_os("WINBLAST_SETTINGS_PATH") {
            return Some(PathBuf::from(p));
        }
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|d| d.join("WinBlast0").join("settings.json"))
    }

    /// Load from disk; any error (missing, unreadable, malformed) yields defaults.
    pub fn load() -> Self {
        Self::path()
            .and_then(|p| std::fs::read(p).ok())
            .map(|bytes| Self::from_json_bytes(&bytes))
            .unwrap_or_default()
    }

    /// Parse JSON bytes leniently: unknown fields are ignored, missing fields default.
    pub fn from_json_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice::<UserSettings>(bytes)
            .map(UserSettings::sanitized)
            .unwrap_or_default()
    }

    /// Persist atomically. Returns the path written.
    pub fn save(&self) -> std::io::Result<PathBuf> {
        let path = Self::path().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "LOCALAPPDATA is not set")
        })?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let json = serde_json::to_vec_pretty(&self.clone().sanitized())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, &path)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrupt_json_falls_back_to_defaults() {
        assert_eq!(UserSettings::from_json_bytes(b"{not json"), UserSettings::default());
        assert_eq!(UserSettings::from_json_bytes(&[0xff, 0xfe, 0x00]), UserSettings::default());
    }

    #[test]
    fn out_of_range_values_are_clamped() {
        let s = UserSettings::from_json_bytes(br#"{"ui_zoom": 9.5, "telemetry_interval_ms": 5}"#);
        assert_eq!(s.ui_zoom, MAX_ZOOM);
        assert_eq!(s.telemetry_interval_ms, 250);
        let s = UserSettings { ui_zoom: f32::NAN, ..Default::default() }.sanitized();
        assert_eq!(s.ui_zoom, 1.0);
    }

    #[test]
    fn partial_json_keeps_other_defaults() {
        let s = UserSettings::from_json_bytes(br#"{"language": "German", "unknown_future_field": 1}"#);
        assert_eq!(s.language, Language::German);
        assert_eq!(s.ui_zoom, 1.0);
    }

    #[test]
    fn roundtrip_via_env_override() {
        let dir = std::env::temp_dir().join(format!("wb0-settings-{}", std::process::id()));
        let path = dir.join("settings.json");
        // SAFETY: test-local env var; this is the only test touching WINBLAST_SETTINGS_PATH.
        unsafe { std::env::set_var("WINBLAST_SETTINGS_PATH", &path) };
        let s = UserSettings {
            language: Language::Japanese,
            ui_zoom: 1.25,
            color_scheme: ColorScheme::EmeraldMint,
            ..Default::default()
        };
        s.save().expect("save settings");
        assert_eq!(UserSettings::load(), s);
        unsafe { std::env::remove_var("WINBLAST_SETTINGS_PATH") };
        let _ = std::fs::remove_dir_all(dir);
    }
}
