//! Windows 10/11 build detection, edition, and feature checks.
//!
//! Reads `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` to determine
//! the running OS version, edition, and whether specific Win11 features
//! (like HAGS or Recall) are available.

use winreg::enums::*;
use winreg::RegKey;

/// Detected Windows OS information.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WindowsInfo {
    /// NT build number (e.g., 19041, 22000, 22631, 26100).
    pub build_number: u32,
    /// Display version string (e.g., "24H2", "23H2").
    pub display_version: String,
    /// Edition (e.g., "Pro", "Home", "Enterprise").
    pub edition: String,
    /// True if build >= 22000 (Windows 11).
    pub is_win11: bool,
}

impl WindowsInfo {
    /// Detect the current Windows version from the registry.
    pub fn detect() -> crate::core::error::Result<Self> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey_path = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";
        let nt_key = match hklm.open_subkey_with_flags(subkey_path, KEY_READ | KEY_WOW64_64KEY) {
            Ok(k) => k,
            Err(e) if e.raw_os_error() == Some(87) => hklm
                .open_subkey(subkey_path)
                .map_err(crate::core::error::WinBlastError::Registry)?,
            Err(e) => return Err(crate::core::error::WinBlastError::Registry(e)),
        };

        let build_str: String = nt_key
            .get_value("CurrentBuildNumber")
            .unwrap_or_else(|_| "0".to_string());
        let build_number: u32 = build_str.trim().parse().unwrap_or(0);

        let display_version: String = nt_key
            .get_value::<String, _>("DisplayVersion")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                nt_key
                    .get_value::<String, _>("ReleaseId")
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
            .unwrap_or_else(|| "Unknown".to_string());

        let edition: String = nt_key
            .get_value::<String, _>("EditionID")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown".to_string());

        let is_win11 = build_number >= 22000;

        Ok(Self {
            build_number,
            display_version,
            edition,
            is_win11,
        })
    }

    /// Check whether this build supports Hardware Accelerated GPU Scheduling.
    #[allow(dead_code)]
    pub fn supports_hags(&self) -> bool {
        self.build_number >= 19041
    }

    /// Check whether this build has the Recall feature (Win11 24H2+).
    #[allow(dead_code)]
    pub fn has_recall(&self) -> bool {
        self.build_number >= 26100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_info_detect_host() {
        let info = WindowsInfo::detect().expect("WindowsInfo::detect should succeed");
        assert!(info.build_number >= 19041);
        assert_ne!(info.display_version, "Unknown");
        assert_ne!(info.edition, "Unknown");
    }
}
