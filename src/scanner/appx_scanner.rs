//! Installed UWP/AppX bloatware detector.
//!
//! High-speed direct registry enumeration across User, Machine, and AllUserStore repositories
//! with sub-millisecond execution and accurate PackageFamilyName matching.

use std::collections::HashSet;
use winreg::enums::*;
use winreg::RegKey;

/// Split a normalized `PackageFamilyName` (`Name_PublisherId`) into `(name, Some(pub_id))`
/// when the trailing underscore segment is a 13-character alphanumeric PublisherId;
/// otherwise return `(clean, None)` so underscore names without a publisher suffix stay intact.
pub(crate) fn split_appx_name_and_publisher(clean: &str) -> (&str, Option<&str>) {
    if let Some((name, pub_id)) = clean.rsplit_once('_') {
        if !name.is_empty()
            && pub_id.len() == 13
            && pub_id.bytes().all(|b| b.is_ascii_alphanumeric())
        {
            return (name, Some(pub_id));
        }
    }
    (clean, None)
}

/// Match an installed lowercase AppX package identifier (full or family name) against a
/// normalized lowercase target family `(family_norm, name_part, pub_part)`.
pub(crate) fn appx_package_matches_family(
    pkg_lower: &str,
    family_norm: &str,
    name_part: &str,
    pub_part: Option<&str>,
) -> bool {
    if pkg_lower == family_norm {
        return true;
    }
    let pkg_norm = crate::tools::features_apps::normalize_appx_family_name(pkg_lower);
    if pkg_norm == family_norm {
        return true;
    }
    if pkg_norm != pkg_lower {
        // Real 5-part PackageFullName already normalized to `Name_PublisherId`:
        // exact family match was checked above; only allow match when query had no publisher ID.
        return pub_part.is_none() && split_appx_name_and_publisher(&pkg_norm).0 == name_part;
    }
    if !pkg_lower.starts_with(name_part) {
        return false;
    }
    if pkg_lower.len() > name_part.len() && pkg_lower.as_bytes()[name_part.len()] != b'_' {
        return false;
    }
    if let Some(pub_id) = pub_part {
        pkg_lower.ends_with(&format!("__{}", pub_id)) || pkg_lower.ends_with(&format!("_{}", pub_id))
    } else {
        true
    }
}

/// In-memory cache of installed package names across user and machine repositories.
#[derive(Debug, Clone, Default)]
pub struct AppxCache {
    /// Lowercase package subkey names / family names from HKCU and HKLM repositories.
    installed_packages: HashSet<String>,
    /// Explicitly tracks whether this cache was populated (prevents accidental PowerShell fallbacks).
    loaded: bool,
}

impl AppxCache {
    /// Construct an explicit pre-populated cache (used by isolated test sandboxes).
    pub fn from_packages(packages: HashSet<String>) -> Self {
        Self {
            installed_packages: packages.into_iter().map(|s| s.trim().to_lowercase()).collect(),
            loaded: true,
        }
    }

    /// Iterate all cached installed AppX package names / families.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.installed_packages.iter()
    }

    /// Load all installed package keys from HKCU, HKLM, and AppxAllUserStore into memory.
    pub fn load() -> Self {
        let mut packages = HashSet::new();
        let mut any_opened = false;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(key) = hkcu
            .open_subkey_with_flags(
                "Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\Repository\\Packages",
                KEY_READ | KEY_WOW64_64KEY,
            )
            .or_else(|_| hkcu.open_subkey("Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\Repository\\Packages"))
        {
            any_opened = true;
            for name in key.enum_keys().filter_map(|r| r.ok()) {
                packages.insert(name.to_lowercase());
            }
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        for sub in [
            "SOFTWARE\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\PackageRepository\\Packages",
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\Applications",
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\InboxApplications",
        ] {
            if let Ok(key) = hklm
                .open_subkey_with_flags(sub, KEY_READ | KEY_WOW64_64KEY)
                .or_else(|_| hklm.open_subkey(sub))
            {
                any_opened = true;
                for name in key.enum_keys().filter_map(|r| r.ok()) {
                    packages.insert(name.to_lowercase());
                }
            }
        }

        Self {
            installed_packages: packages,
            loaded: any_opened,
        }
    }

    /// Evaluate a `PackageFamilyName` (e.g. `"Microsoft.BingWeather_8wekyb3d8bbwe"`) returning `(is_removed, diagnostic, detail_kind)`.
    pub fn evaluate_package_with_kind(
        &self,
        package_family: &str,
    ) -> (bool, String, crate::scanner::ScanDetailKind) {
        use crate::scanner::ScanDetailKind;

        let family_clean = package_family.trim().to_lowercase();
        if family_clean.is_empty() {
            return (
                true,
                "No package specified".to_string(),
                ScanDetailKind::AppxNoPackage,
            );
        }

        if !self.loaded {
            let removed = check_package_removed_ps(&family_clean);
            return if removed {
                (
                    true,
                    "Removed / Not Installed".to_string(),
                    ScanDetailKind::AppxRemovedPs,
                )
            } else {
                (
                    false,
                    "Installed (Detected via PowerShell)".to_string(),
                    ScanDetailKind::AppxInstalledPs,
                )
            };
        }

        let family_norm = crate::tools::features_apps::normalize_appx_family_name(&family_clean)
            .to_ascii_lowercase();
        let (name_part, publisher_part) = split_appx_name_and_publisher(&family_norm);

        let matched_package = self
            .installed_packages
            .iter()
            .find(|pkg| {
                appx_package_matches_family(pkg.as_str(), &family_norm, name_part, publisher_part)
            })
            .cloned();

        if let Some(pkg_name) = matched_package {
            (
                false,
                format!("Installed ({})", pkg_name),
                ScanDetailKind::AppxInstalled { package: pkg_name },
            )
        } else {
            (
                true,
                "Removed / Not Installed (Optimized)".to_string(),
                ScanDetailKind::AppxRemovedOptimized,
            )
        }
    }

    /// Evaluate a `PackageFamilyName` (e.g. `"Microsoft.BingWeather_8wekyb3d8bbwe"`) returning `(is_removed, diagnostic)`.
    pub fn evaluate_package(&self, package_family: &str) -> (bool, String) {
        let (is_removed, msg, _) = self.evaluate_package_with_kind(package_family);
        (is_removed, msg)
    }

    /// Returns `true` if the package is NOT installed (removed/optimized).
    pub fn is_package_removed(&self, package_family: &str) -> bool {
        self.evaluate_package(package_family).0
    }
}

/// Standalone check if an AppX package family has been removed.
#[allow(dead_code)]
pub fn check_package_removed(package_family: &str) -> bool {
    AppxCache::load().is_package_removed(package_family)
}

/// Fallback PowerShell check using exact `PackageFamilyName` property comparison.
fn check_package_removed_ps(package_family: &str) -> bool {
    let escaped = package_family.replace('\'', "''");
    let script = format!(
        "(Get-AppxPackage -AllUsers | Where-Object {{ $_.PackageFamilyName -eq '{}' }} | Measure-Object).Count",
        escaped
    );
    let output = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.trim().parse::<u32>().map(|c| c == 0).unwrap_or(true)
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appx_cache_loaded_empty_no_fallback() {
        let cache = AppxCache::from_packages(HashSet::new());
        let (removed, _) = cache.evaluate_package("Microsoft.BingWeather_8wekyb3d8bbwe");
        assert!(removed);
    }

    #[test]
    fn test_appx_publisher_suffix_matching() {
        let mut pkgs = HashSet::new();
        pkgs.insert("microsoft.bingweather_4.53.0.0_x64__8wekyb3d8bbwe".to_string());
        pkgs.insert("microsoft.bingweather_1.0_x64__wrongpub1234".to_string());
        let cache = AppxCache::from_packages(pkgs);
        assert!(!cache.is_package_removed("Microsoft.BingWeather_8wekyb3d8bbwe"));
        assert!(cache.is_package_removed("Microsoft.BingWeather_otherpub99999"));
    }

    #[test]
    fn test_split_appx_name_and_publisher_with_underscores_and_prefix() {
        assert_eq!(
            split_appx_name_and_publisher("my_cool_app_8wekyb3d8bbwe"),
            ("my_cool_app", Some("8wekyb3d8bbwe"))
        );
        assert_eq!(
            split_appx_name_and_publisher("my_cool_app"),
            ("my_cool_app", None)
        );

        let mut pkgs = HashSet::new();
        pkgs.insert("my_cool_app_1.2.0.0_x64__8wekyb3d8bbwe".to_string());
        pkgs.insert("my_cool_app_pro_1.2.0.0_x64__8wekyb3d8bbwe".to_string());
        let cache = AppxCache::from_packages(pkgs);
        assert!(!cache.is_package_removed("My_Cool_App_8wekyb3d8bbwe"));
        assert!(!cache.is_package_removed("My_Cool_App"));
        assert!(cache.is_package_removed("My_Cool_App_otherpub99999"));
    }

    #[test]
    fn test_evaluate_package_with_kind_all_cache_branches() {
        use crate::scanner::ScanDetailKind;

        let mut pkgs = HashSet::new();
        pkgs.insert("microsoft.bingweather_4.53.0.0_x64__8wekyb3d8bbwe".to_string());
        let cache = AppxCache::from_packages(pkgs);

        assert_eq!(
            cache.evaluate_package_with_kind("   "),
            (true, "No package specified".to_string(), ScanDetailKind::AppxNoPackage)
        );
        assert_eq!(
            cache.evaluate_package_with_kind("Microsoft.BingWeather_8wekyb3d8bbwe"),
            (
                false,
                "Installed (microsoft.bingweather_4.53.0.0_x64__8wekyb3d8bbwe)".to_string(),
                ScanDetailKind::AppxInstalled {
                    package: "microsoft.bingweather_4.53.0.0_x64__8wekyb3d8bbwe".to_string(),
                },
            )
        );
        assert_eq!(
            cache.evaluate_package_with_kind("Microsoft.XboxApp_8wekyb3d8bbwe"),
            (
                true,
                "Removed / Not Installed (Optimized)".to_string(),
                ScanDetailKind::AppxRemovedOptimized,
            )
        );
    }
}
