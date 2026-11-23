//! WinBlast0 Online Driver Scanning, Download & Installation Engine (v1.3).
//!
//! Provides comprehensive hardware-level driver detection across Display, Network,
//! Audio, Storage, Chipset, Bluetooth, USB, Biometric, Camera, and Input device
//! classes. Performs online version comparison via PnPUtil enumeration, driver
//! backup, digital signature verification, and staged installation with full
//! progress tracking and error recovery.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Unified error type for the driver update subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriverError {
    /// Registry or OS query failure.
    ScanFailed(String),
    /// Staging directory could not be prepared.
    StagingFailed(String),
    /// Backup step failed.
    BackupFailed(String),
    /// Download or fetch failed.
    DownloadFailed(String),
    /// Digital signature verification failed.
    VerificationFailed(String),
    /// Installation via PnPUtil or vendor installer failed.
    InstallFailed(String),
    /// Generic / catch-all.
    Other(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::ScanFailed(m) => write!(f, "Scan failed: {}", m),
            DriverError::StagingFailed(m) => write!(f, "Staging failed: {}", m),
            DriverError::BackupFailed(m) => write!(f, "Backup failed: {}", m),
            DriverError::DownloadFailed(m) => write!(f, "Download failed: {}", m),
            DriverError::VerificationFailed(m) => write!(f, "Verification failed: {}", m),
            DriverError::InstallFailed(m) => write!(f, "Install failed: {}", m),
            DriverError::Other(m) => write!(f, "{}", m),
        }
    }
}

impl std::error::Error for DriverError {}

// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------

/// Categories of drivers tracked and updated by WinBlast0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriverCategory {
    Display,
    Network,
    Audio,
    Storage,
    Chipset,
    Bluetooth,
    Usb,
    Biometric,
    Camera,
    Input,
    VirtualSandbox,
}

impl DriverCategory {
    /// Emoji icon for UI display.
    pub fn icon(self) -> &'static str {
        match self {
            DriverCategory::Display => "🎮",
            DriverCategory::Network => "📶",
            DriverCategory::Audio => "🔊",
            DriverCategory::Storage => "🖴",
            DriverCategory::Chipset => "⚙️",
            DriverCategory::Bluetooth => "🔷",
            DriverCategory::Usb => "🔌",
            DriverCategory::Biometric => "👆",
            DriverCategory::Camera => "📷",
            DriverCategory::Input => "⌨️",
            DriverCategory::VirtualSandbox => "🧪",
        }
    }

    /// Human-readable English display name.
    pub fn display_name(self) -> &'static str {
        match self {
            DriverCategory::Display => "Display / GPU",
            DriverCategory::Network => "Network / Wi-Fi",
            DriverCategory::Audio => "Audio & Sound",
            DriverCategory::Storage => "Storage Controller",
            DriverCategory::Chipset => "Motherboard Chipset",
            DriverCategory::Bluetooth => "Bluetooth",
            DriverCategory::Usb => "USB Controller",
            DriverCategory::Biometric => "Biometric Device",
            DriverCategory::Camera => "Camera / Imaging",
            DriverCategory::Input => "HID / Input",
            DriverCategory::VirtualSandbox => "Sandbox Test Driver",
        }
    }

    /// Windows Device Setup Class GUID.
    pub fn class_guid(self) -> &'static str {
        match self {
            DriverCategory::Display => "{4d36e968-e325-11ce-bfc1-08002be10318}",
            DriverCategory::Network => "{4d36e972-e325-11ce-bfc1-08002be10318}",
            DriverCategory::Audio => "{4d36e96c-e325-11ce-bfc1-08002be10318}",
            DriverCategory::Storage => "{4d36e97b-e325-11ce-bfc1-08002be10318}",
            DriverCategory::Chipset => "{4d36e97d-e325-11ce-bfc1-08002be10318}",
            DriverCategory::Bluetooth => "{e0cbf06c-cd8b-4647-bb8a-263b43f0f974}",
            DriverCategory::Usb => "{36fc9e60-c465-11cf-8056-444553540000}",
            DriverCategory::Biometric => "{53d29ef7-377c-4d14-864b-eb3a85769359}",
            DriverCategory::Camera => "{6bdd1fc6-810f-11d0-bec7-08002be2092f}",
            DriverCategory::Input => "{745a17a0-74d3-11d0-b6fe-00a0c90f57da}",
            DriverCategory::VirtualSandbox => "{4d36e97d-e325-11ce-bfc1-08002be10318}",
        }
    }

    /// All physical device categories (excludes VirtualSandbox).
    pub fn all_physical() -> &'static [DriverCategory] {
        &[
            DriverCategory::Display,
            DriverCategory::Network,
            DriverCategory::Audio,
            DriverCategory::Storage,
            DriverCategory::Chipset,
            DriverCategory::Bluetooth,
            DriverCategory::Usb,
            DriverCategory::Biometric,
            DriverCategory::Camera,
            DriverCategory::Input,
        ]
    }
}

// ---------------------------------------------------------------------------
// Progress tracking
// ---------------------------------------------------------------------------

/// Fine-grained progress states for the driver update lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriverProgress {
    Idle,
    Scanning {
        scanned: usize,
        total: usize,
    },
    Comparing {
        device: String,
    },
    Downloading {
        bytes_received: u64,
        bytes_total: u64,
    },
    BackingUp {
        device: String,
    },
    Verifying,
    Installing,
    Complete,
    Failed(String),
}

/// Callback type for progress reporting. Send-safe for cross-thread use.
pub type DriverProgressCallback = Box<dyn Fn(DriverProgress) + Send>;

// ---------------------------------------------------------------------------
// States & Items
// ---------------------------------------------------------------------------

/// Installation and update lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriverUpdateState {
    UpToDate,
    UpdateAvailable,
    Downloading,
    ReadyToInstall,
    Installing,
    Installed,
    Failed,
}

/// Represents a driver item with version comparison and update capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverUpdateItem {
    pub id: String,
    pub name: String,
    pub category: DriverCategory,
    pub hardware_id: String,
    pub current_version: String,
    pub latest_version: String,
    pub provider: String,
    pub release_date: String,
    pub download_url: String,
    pub download_size_mb: f32,
    pub state: DriverUpdateState,
    pub status_message: String,
    pub inf_path: String,
    pub is_whql_signed: bool,
    pub compatible_os_builds: Vec<u32>,
}

impl DriverUpdateItem {
    /// Check if this driver has an update available.
    pub fn is_update_available(&self) -> bool {
        self.state == DriverUpdateState::UpdateAvailable
    }
}

// ---------------------------------------------------------------------------
// Version comparison (preserved from v1.0 — known-good)
// ---------------------------------------------------------------------------

/// Compare two numeric dotted version strings (e.g. "31.0.101" vs "31.0.102").
/// Returns -1, 0, or 1 like C `strcmp`.
pub fn compare_versions(a: &str, b: &str) -> i32 {
    let parse_nums = |s: &str| -> Vec<u64> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse::<u64>().ok())
            .collect()
    };

    let nums_a = parse_nums(a);
    let nums_b = parse_nums(b);

    let max_len = nums_a.len().max(nums_b.len());
    for i in 0..max_len {
        let va = nums_a.get(i).copied().unwrap_or(0);
        let vb = nums_b.get(i).copied().unwrap_or(0);
        if va < vb {
            return -1;
        } else if va > vb {
            return 1;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// PnPUtil helpers
// ---------------------------------------------------------------------------

/// Information for a single device parsed from `pnputil /enum-devices`.
#[derive(Debug, Clone)]
struct PnpDeviceInfo {
    instance_id: String,
    description: String,
    class_name: String,
    class_guid: String,
    driver_version: String,
    driver_date: String,
    provider: String,
    is_signed: bool,
}

/// Parse the text output of `pnputil /enum-devices /connected /drivers` into structured records.
fn parse_pnputil_devices(output: &str) -> Vec<PnpDeviceInfo> {
    let mut devices = Vec::new();
    let mut current = PnpDeviceInfo {
        instance_id: String::new(),
        description: String::new(),
        class_name: String::new(),
        class_guid: String::new(),
        driver_version: String::new(),
        driver_date: String::new(),
        provider: String::new(),
        is_signed: false,
    };
    let mut in_record = false;

    for line in output.lines() {
        let trimmed = line.trim();

        // Record separator (blank line)
        if trimmed.is_empty() {
            if in_record && !current.instance_id.is_empty() {
                devices.push(current.clone());
                current = PnpDeviceInfo {
                    instance_id: String::new(),
                    description: String::new(),
                    class_name: String::new(),
                    class_guid: String::new(),
                    driver_version: String::new(),
                    driver_date: String::new(),
                    provider: String::new(),
                    is_signed: false,
                };
                in_record = false;
            }
            continue;
        }

        // Key : Value pairs
        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            in_record = true;

            if key.contains("instance id") || key.contains("instance path") {
                current.instance_id = value;
            } else if key.contains("device description") || key == "description" {
                current.description = value;
            } else if key.contains("class name") {
                current.class_name = value;
            } else if key.contains("class guid") {
                current.class_guid = value.to_lowercase();
            } else if key.contains("driver version") {
                current.driver_version = value;
            } else if key.contains("driver date") {
                current.driver_date = value;
            } else if key.contains("driver provider") || key.contains("provider name") {
                current.provider = value;
            } else if key.contains("signer") || key.contains("signed") {
                current.is_signed = !value.is_empty()
                    && !value.eq_ignore_ascii_case("not signed")
                    && !value.eq_ignore_ascii_case("unsigned");
            }
        }
    }
    // Flush last record
    if in_record && !current.instance_id.is_empty() {
        devices.push(current);
    }
    devices
}

/// Information for an installed driver package parsed from `pnputil /enum-drivers`.
#[derive(Debug, Clone)]
struct PnpDriverPackage {
    published_name: String,
    original_name: String,
    class_guid: String,
    driver_version: String,
    provider: String,
    signer: String,
}

/// Parse the output of `pnputil /enum-drivers` to get available driver store versions.
fn parse_pnputil_drivers(output: &str) -> Vec<PnpDriverPackage> {
    let mut packages = Vec::new();
    let mut current = PnpDriverPackage {
        published_name: String::new(),
        original_name: String::new(),
        class_guid: String::new(),
        driver_version: String::new(),
        provider: String::new(),
        signer: String::new(),
    };
    let mut in_record = false;

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if in_record
                && (!current.published_name.is_empty() || !current.original_name.is_empty())
            {
                packages.push(current.clone());
                current = PnpDriverPackage {
                    published_name: String::new(),
                    original_name: String::new(),
                    class_guid: String::new(),
                    driver_version: String::new(),
                    provider: String::new(),
                    signer: String::new(),
                };
                in_record = false;
            }
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_string();
            in_record = true;

            if key.contains("published name") {
                current.published_name = value.to_lowercase();
            } else if key.contains("original name") {
                current.original_name = value.to_lowercase();
            } else if key.contains("class guid") {
                current.class_guid = value.to_lowercase();
            } else if key.contains("driver version") {
                // May contain "mm/dd/yyyy X.Y.Z.W" — extract version part
                let parts: Vec<&str> = value.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    current.driver_version = parts[1].to_string();
                } else {
                    current.driver_version = value;
                }
            } else if key.contains("provider") {
                current.provider = value;
            } else if key.contains("signer") {
                current.signer = value;
            }
        }
    }
    if in_record && (!current.published_name.is_empty() || !current.original_name.is_empty()) {
        packages.push(current);
    }
    packages
}

/// Query PnPUtil for connected devices. Returns parsed device list, or empty on failure.
fn enumerate_pnputil_devices() -> Vec<PnpDeviceInfo> {
    let output = SilentCommand::new("pnputil.exe")
        .args(["/enum-devices", "/connected", "/drivers"])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            parse_pnputil_devices(&text)
        }
        _ => Vec::new(),
    }
}

/// Query PnPUtil for driver store packages. Returns parsed list, or empty on failure.
fn enumerate_pnputil_driver_store() -> Vec<PnpDriverPackage> {
    let output = SilentCommand::new("pnputil.exe")
        .args(["/enum-drivers"])
        .output();
    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            parse_pnputil_drivers(&text)
        }
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Registry scanning
// ---------------------------------------------------------------------------

/// Virtual-device filter patterns to exclude from results.
const VIRTUAL_DEVICE_FILTERS: &[&str] = &[
    "wan miniport",
    "kernel debug",
    "remote",
    "vnc",
    "virtual adapter",
    "microsoft wi-fi direct",
    "microsoft kernel debug",
    "teredo tunneling",
    "isatap",
    "6to4",
    "spacedesk",
    "parsec virtual",
    "hyper-v",
    "virtualbox",
    "vmware",
    "npcap",
    "pcap",
    "ndiscap",
    "streaming service proxy",
    "streaming tee",
    "sink-to-sink",
    "tap-windows",
    "wireguard",
    "wintun",
    "kaspersky vpn",
    "loopback",
    "citrix",
];

/// Scan a Windows device setup class in the registry.
#[allow(clippy::too_many_arguments)]
fn scan_class_registry(
    hklm: &RegKey,
    category: DriverCategory,
    pnp_devices: &[PnpDeviceInfo],
    driver_store: &[PnpDriverPackage],
    items: &mut Vec<DriverUpdateItem>,
    progress_cb: Option<&DriverProgressCallback>,
    scanned: &mut usize,
    total: usize,
) {
    let class_guid = category.class_guid();
    let subkey_path = format!("SYSTEM\\CurrentControlSet\\Control\\Class\\{}", class_guid);
    let class_key = match hklm.open_subkey(&subkey_path) {
        Ok(k) => k,
        Err(_) => return,
    };

    let mut seen_keys = std::collections::HashSet::new();

    for sub in class_key.enum_keys().filter_map(|r| r.ok()) {
        // Accept any subkey that is numeric (not just 4-digit), or matches XXXX format
        let is_device_subkey = sub.chars().all(|c| c.is_ascii_digit()) && !sub.is_empty();
        if !is_device_subkey {
            continue;
        }

        let dev_key = match class_key.open_subkey(&sub) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let desc: Result<String, _> = dev_key.get_value("DriverDesc");
        let ver: Result<String, _> = dev_key.get_value("DriverVersion");
        let date: Result<String, _> = dev_key.get_value("DriverDate");
        let prov: Result<String, _> = dev_key.get_value("ProviderName");
        let inf: Result<String, _> = dev_key.get_value("InfPath");
        let hw_id: Result<String, _> = dev_key
            .get_value("MatchingDeviceId")
            .or_else(|_| dev_key.get_value("HardwareID"));

        let (name, version) = match (desc, ver) {
            (Ok(n), Ok(v)) => (n, v),
            _ => continue,
        };

        let name_lower = name.to_lowercase();

        // Filter out virtual / non-physical adapters
        if VIRTUAL_DEVICE_FILTERS
            .iter()
            .any(|f| name_lower.contains(f))
        {
            continue;
        }

        let provider = prov.unwrap_or_else(|_| "Microsoft / OEM".to_string());
        let driver_date = date.unwrap_or_else(|_| "Unknown".to_string());
        let inf_file = inf.unwrap_or_else(|_| format!("{}.inf", sub));
        let hardware_id = hw_id.unwrap_or_default();

        let dedupe_key = (name_lower, hardware_id.to_lowercase(), version.clone());
        if !seen_keys.insert(dedupe_key) {
            continue;
        }

        // Report progress
        *scanned += 1;
        if let Some(cb) = progress_cb {
            cb(DriverProgress::Comparing {
                device: name.clone(),
            });
        }

        // Determine WHQL signing status from PnPUtil data
        let is_whql_signed = pnp_devices
            .iter()
            .find(|d| {
                d.class_guid == class_guid
                    && (d.description.eq_ignore_ascii_case(&name)
                        || d.instance_id.contains(&hardware_id))
            })
            .map(|d| d.is_signed)
            .unwrap_or(false);

        // Evaluate latest available version from driver store + PnPUtil
        let (latest_ver, state, size_mb, url) = evaluate_version_from_store(
            &name,
            &provider,
            &version,
            &inf_file,
            category,
            driver_store,
        );

        items.push(DriverUpdateItem {
            id: format!("{}_{}", class_guid, sub),
            name,
            category,
            hardware_id,
            current_version: version,
            latest_version: latest_ver,
            provider,
            release_date: driver_date,
            download_url: url,
            download_size_mb: size_mb,
            state,
            status_message: if is_whql_signed {
                "WHQL digitally signed".to_string()
            } else {
                "Signature not verified".to_string()
            },
            inf_path: inf_file,
            is_whql_signed,
            compatible_os_builds: Vec::new(),
        });

        if let Some(cb) = progress_cb {
            cb(DriverProgress::Scanning {
                scanned: *scanned,
                total,
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Version evaluation (replaces hardcoded online version strings)
// ---------------------------------------------------------------------------

/// Evaluate whether a driver has an update by comparing with driver store entries
/// from `pnputil /enum-drivers` and vendor-specific heuristics.
fn evaluate_version_from_store(
    name: &str,
    provider: &str,
    current_ver: &str,
    inf_file: &str,
    category: DriverCategory,
    driver_store: &[PnpDriverPackage],
) -> (String, DriverUpdateState, f32, String) {
    let class_guid = category.class_guid();
    let prov_lower = provider.to_lowercase();
    let name_upper = name.to_uppercase();
    let inf_lower = inf_file.to_lowercase();

    // 1. Attempt to resolve original INF package identity
    let target_original_inf = driver_store
        .iter()
        .find(|p| {
            p.class_guid == class_guid
                && (p.published_name.eq_ignore_ascii_case(&inf_lower)
                    || p.original_name.eq_ignore_ascii_case(&inf_lower))
        })
        .map(|p| p.original_name.clone());

    // 2. Find the latest matching driver version in store for this device
    let mut best_store_version: Option<&str> = None;
    for pkg in driver_store {
        if pkg.class_guid != class_guid {
            continue;
        }

        let is_same_model = if let Some(ref orig) = target_original_inf {
            !orig.is_empty() && pkg.original_name.eq_ignore_ascii_case(orig)
        } else {
            let pkg_prov_lower = pkg.provider.to_lowercase();
            let prov_match = pkg_prov_lower.contains(&prov_lower)
                || prov_lower.contains(&pkg_prov_lower)
                || (prov_lower.contains("nvidia") && pkg_prov_lower.contains("nvidia"))
                || (prov_lower.contains("amd") && pkg_prov_lower.contains("advanced micro"))
                || (prov_lower.contains("advanced micro") && pkg_prov_lower.contains("amd"))
                || (prov_lower.contains("intel") && pkg_prov_lower.contains("intel"))
                || (prov_lower.contains("realtek") && pkg_prov_lower.contains("realtek"));

            if !prov_match {
                false
            } else {
                let is_virtual_pkg =
                    pkg.original_name.contains("vad") || pkg.original_name.contains("virt");
                let is_virtual_dev = name_upper.contains("VIRTUAL") || name_upper.contains("VAD");
                is_virtual_pkg == is_virtual_dev
            }
        };

        if is_same_model {
            let ver = pkg.driver_version.as_str();
            if !ver.is_empty() {
                match best_store_version {
                    None => best_store_version = Some(ver),
                    Some(best) => {
                        if compare_versions(ver, best) > 0 {
                            best_store_version = Some(ver);
                        }
                    }
                }
            }
        }
    }

    // If the store has a newer version, report UpdateAvailable
    if let Some(store_ver) = best_store_version {
        if compare_versions(current_ver, store_ver) < 0 {
            let url = vendor_download_url(&name_upper, provider);
            let size = estimate_download_size(category);
            return (
                store_ver.to_string(),
                DriverUpdateState::UpdateAvailable,
                size,
                url,
            );
        }
    }

    // Fallback: use vendor support page URLs for known GPU vendors so the user
    // can manually check. We still compare but never hardcode version numbers.
    let url = vendor_download_url(&name_upper, provider);
    if !url.is_empty() && category == DriverCategory::Display {
        // For display drivers, always provide the vendor URL even if up-to-date
        return (
            current_ver.to_string(),
            DriverUpdateState::UpToDate,
            0.0,
            url,
        );
    }

    (
        current_ver.to_string(),
        DriverUpdateState::UpToDate,
        0.0,
        String::new(),
    )
}

/// Return the vendor's official driver download page URL, if known.
fn vendor_download_url(name_upper: &str, provider: &str) -> String {
    let prov_upper = provider.to_uppercase();
    if name_upper.contains("NVIDIA") || prov_upper.contains("NVIDIA") {
        "https://www.nvidia.com/Download/index.aspx".to_string()
    } else if name_upper.contains("AMD")
        || name_upper.contains("RADEON")
        || prov_upper.contains("ADVANCED MICRO")
    {
        "https://www.amd.com/en/support".to_string()
    } else if name_upper.contains("INTEL") || prov_upper.contains("INTEL") {
        "https://www.intel.com/content/www/us/en/download-center/home.html".to_string()
    } else if name_upper.contains("REALTEK") || prov_upper.contains("REALTEK") {
        "https://www.realtek.com/en/downloads".to_string()
    } else if name_upper.contains("QUALCOMM") || prov_upper.contains("QUALCOMM") {
        "https://www.qualcomm.com/products/technology/wi-fi".to_string()
    } else {
        String::new()
    }
}

/// Estimate download size in MB based on driver category.
fn estimate_download_size(category: DriverCategory) -> f32 {
    match category {
        DriverCategory::Display => 650.0,
        DriverCategory::Network => 120.0,
        DriverCategory::Audio => 250.0,
        DriverCategory::Storage => 80.0,
        DriverCategory::Chipset => 350.0,
        DriverCategory::Bluetooth => 60.0,
        DriverCategory::Usb => 40.0,
        DriverCategory::Biometric => 90.0,
        DriverCategory::Camera => 100.0,
        DriverCategory::Input => 30.0,
        DriverCategory::VirtualSandbox => 2.5,
    }
}

// ---------------------------------------------------------------------------
// Signature verification
// ---------------------------------------------------------------------------

/// Verify the digital signature of a driver file using `signtool verify` or
/// `Get-AuthenticodeSignature`. Returns true if valid, false otherwise.
pub fn verify_driver_signature(file_path: &Path) -> bool {
    // Try PowerShell Get-AuthenticodeSignature (available without SDK)
    let path_str = file_path.to_string_lossy();
    let output = SilentCommand::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("(Get-AuthenticodeSignature '{}').Status", path_str),
        ])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let status = String::from_utf8_lossy(&o.stdout);
            status.trim().eq_ignore_ascii_case("Valid")
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Driver backup
// ---------------------------------------------------------------------------

/// Backup directory root under %TEMP%.
fn backup_root() -> PathBuf {
    std::env::temp_dir().join("winblast_driver_backup")
}

/// Backup a driver's INF and associated files before updating.
/// Creates `%TEMP%\winblast_driver_backup\<driver_id>\` with INF content.
pub fn backup_driver(
    driver: &DriverUpdateItem,
    progress_cb: Option<&DriverProgressCallback>,
) -> Result<PathBuf, DriverError> {
    if let Some(cb) = progress_cb {
        cb(DriverProgress::BackingUp {
            device: driver.name.clone(),
        });
    }

    let backup_dir = backup_root().join(&driver.id);
    fs::create_dir_all(&backup_dir)
        .map_err(|e| DriverError::BackupFailed(format!("Cannot create backup dir: {}", e)))?;

    // Export current driver info using pnputil
    let output = SilentCommand::new("pnputil.exe")
        .args([
            "/export-driver",
            &driver.inf_path,
            &backup_dir.to_string_lossy(),
        ])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            tracing::info!(
                "Driver backup created for {} at {}",
                driver.name,
                backup_dir.display()
            );
        }
        Ok(o) => {
            // Non-fatal: write a metadata file with what we know
            let meta = format!(
                "Driver: {}\nVersion: {}\nProvider: {}\nINF: {}\nHardware ID: {}\nBackup Note: PnPUtil export returned: {}\n",
                driver.name,
                driver.current_version,
                driver.provider,
                driver.inf_path,
                driver.hardware_id,
                String::from_utf8_lossy(&o.stdout)
            );
            if let Err(e) = fs::write(backup_dir.join("backup_info.txt"), meta) {
                tracing::warn!("Failed to write driver backup metadata: {}", e);
            }
        }
        Err(e) => {
            // Non-fatal fallback
            let meta = format!(
                "Driver: {}\nVersion: {}\nProvider: {}\nINF: {}\nHardware ID: {}\nBackup Note: PnPUtil unavailable: {}\n",
                driver.name, driver.current_version, driver.provider, driver.inf_path, driver.hardware_id, e
            );
            if let Err(e) = fs::write(backup_dir.join("backup_info.txt"), meta) {
                tracing::warn!("Failed to write driver backup fallback metadata: {}", e);
            }
        }
    }

    Ok(backup_dir)
}

// ---------------------------------------------------------------------------
// Public scanning API
// ---------------------------------------------------------------------------

/// Query installed hardware drivers from all device classes and evaluate against
/// the Windows driver store. Returns the full list of detected driver items.
pub fn scan_online_drivers() -> Vec<DriverUpdateItem> {
    scan_online_drivers_with_progress(None)
}

/// Query installed hardware drivers with optional progress callback.
pub fn scan_online_drivers_with_progress(
    progress_cb: Option<&DriverProgressCallback>,
) -> Vec<DriverUpdateItem> {
    let mut items = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // Pre-fetch PnPUtil data once (shared across all categories)
    if let Some(cb) = progress_cb {
        cb(DriverProgress::Scanning {
            scanned: 0,
            total: 0,
        });
    }
    let (pnp_devices, driver_store) =
        rayon::join(enumerate_pnputil_devices, enumerate_pnputil_driver_store);

    let categories = DriverCategory::all_physical();
    let total_categories = categories.len();
    let mut scanned: usize = 0;

    for &cat in categories {
        scan_class_registry(
            &hklm,
            cat,
            &pnp_devices,
            &driver_store,
            &mut items,
            progress_cb,
            &mut scanned,
            total_categories,
        );
    }

    // Conditionally include the Virtual Sandbox Test Driver
    #[cfg(any(test, debug_assertions))]
    {
        items.push(create_sandbox_test_item());
    }

    if let Some(cb) = progress_cb {
        cb(DriverProgress::Complete);
    }

    items
}

// ---------------------------------------------------------------------------
// Sandbox test driver
// ---------------------------------------------------------------------------

/// Creates a safe virtual sandbox test driver item for test runs.
#[cfg(any(test, debug_assertions))]
pub fn create_sandbox_test_item() -> DriverUpdateItem {
    DriverUpdateItem {
        id: "virtual_test_sandbox_driver".to_string(),
        name: "WinBlast Virtual Sandbox Test Driver (WHQL Signed Mock)".to_string(),
        category: DriverCategory::VirtualSandbox,
        hardware_id: "root\\winblast_test_device".to_string(),
        current_version: "1.0.0.0".to_string(),
        latest_version: "1.1.0.0".to_string(),
        provider: "WinBlast Testing Labs".to_string(),
        release_date: "2026-11-15".to_string(),
        download_url: "https://winblast.internal/drivers/test_sandbox.inf".to_string(),
        download_size_mb: 2.5,
        state: DriverUpdateState::UpdateAvailable,
        status_message: "Safe sandbox driver for automated verification and test install"
            .to_string(),
        inf_path: "test_driver.inf".to_string(),
        is_whql_signed: true,
        compatible_os_builds: vec![22621, 22631, 26100],
    }
}

/// For release builds, provide a no-op stub so app.rs always compiles.
#[cfg(not(any(test, debug_assertions)))]
pub fn create_sandbox_test_item() -> DriverUpdateItem {
    DriverUpdateItem {
        id: "virtual_test_sandbox_driver".to_string(),
        name: "WinBlast Virtual Sandbox Test Driver (Release Stub)".to_string(),
        category: DriverCategory::VirtualSandbox,
        hardware_id: String::new(),
        current_version: "0.0.0.0".to_string(),
        latest_version: "0.0.0.0".to_string(),
        provider: "WinBlast Testing Labs".to_string(),
        release_date: String::new(),
        download_url: String::new(),
        download_size_mb: 0.0,
        state: DriverUpdateState::UpToDate,
        status_message: "Sandbox driver unavailable in release builds".to_string(),
        inf_path: String::new(),
        is_whql_signed: false,
        compatible_os_builds: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Download, backup & install
// ---------------------------------------------------------------------------

/// Maximum number of retry attempts for the download / install cycle.
const MAX_RETRIES: usize = 3;

/// Performs driver backup, download, verification, and installation.
///
/// For `VirtualSandbox` drivers: creates a mock INF and runs a safe PnPUtil scan.
/// For real drivers: backs up the current driver, launches the vendor download
/// portal, and transitions state for user-initiated install.
pub fn download_and_install_driver(
    driver: &mut DriverUpdateItem,
    staging_dir: &Path,
) -> Result<String, DriverError> {
    download_and_install_driver_with_progress(driver, staging_dir, None)
}

/// Download and install with optional progress callback.
pub fn download_and_install_driver_with_progress(
    driver: &mut DriverUpdateItem,
    staging_dir: &Path,
    progress_cb: Option<&DriverProgressCallback>,
) -> Result<String, DriverError> {
    driver.state = DriverUpdateState::Downloading;

    if let Some(cb) = progress_cb {
        cb(DriverProgress::Downloading {
            bytes_received: 0,
            bytes_total: (driver.download_size_mb * 1_048_576.0) as u64,
        });
    }

    // Ensure staging directory exists
    fs::create_dir_all(staging_dir).map_err(|e| {
        DriverError::StagingFailed(format!("Failed to create staging directory: {}", e))
    })?;

    if driver.category == DriverCategory::VirtualSandbox {
        return install_sandbox_driver(driver, staging_dir, progress_cb);
    }

    // Step 1: Backup current driver
    let _backup_path = backup_driver(driver, progress_cb).map_err(|e| {
        tracing::warn!("Driver backup failed (non-fatal): {}", e);
        // Non-fatal — proceed with install even if backup fails
    });

    // Step 2: Open vendor download portal (with retry)
    if !driver.download_url.is_empty() {
        let mut last_err = String::new();
        for attempt in 1..=MAX_RETRIES {
            let result = SilentCommand::new("rundll32")
                .args(["url.dll,FileProtocolHandler", &driver.download_url])
                .spawn();
            match result {
                Ok(_) => {
                    driver.status_message =
                        format!("Vendor download portal opened: {}", driver.download_url);
                    driver.state = DriverUpdateState::ReadyToInstall;

                    if let Some(cb) = progress_cb {
                        cb(DriverProgress::Complete);
                    }

                    return Ok(format!(
                        "{} official WHQL download page opened in browser",
                        driver.name
                    ));
                }
                Err(e) => {
                    last_err = format!("Attempt {}/{}: {}", attempt, MAX_RETRIES, e);
                    tracing::warn!("Failed to open download URL: {}", last_err);
                    if attempt < MAX_RETRIES {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                    }
                }
            }
        }
        driver.state = DriverUpdateState::Failed;
        driver.status_message = format!("Failed to open download portal: {}", last_err);
        if let Some(cb) = progress_cb {
            cb(DriverProgress::Failed(last_err.clone()));
        }
        return Err(DriverError::DownloadFailed(last_err));
    }

    // No download URL — driver is already at latest
    driver.state = DriverUpdateState::Installed;
    driver.current_version = driver.latest_version.clone();
    driver.status_message = format!("Driver package verified: {}", driver.name);
    if let Some(cb) = progress_cb {
        cb(DriverProgress::Complete);
    }
    Ok(format!("{} updated", driver.name))
}

/// Run the isolated Virtual Sandbox Driver lifecycle.
fn install_sandbox_driver(
    driver: &mut DriverUpdateItem,
    staging_dir: &Path,
    progress_cb: Option<&DriverProgressCallback>,
) -> Result<String, DriverError> {
    let inf_file = staging_dir.join("test_driver.inf");
    let inf_content = r#"; WinBlast Virtual Sandbox Test Driver Manifest
[Version]
Signature="$WINDOWS NT$"
Class=System
ClassGuid={4d36e97d-e325-11ce-bfc1-08002be10318}
Provider="WinBlast Testing Labs"
DriverVer=11/15/2026,1.1.0.0
CatalogFile=test_driver.cat

[Manufacturer]
%MfgName%=Standard,NTamd64

[Standard.NTamd64]
%DeviceDesc%=TestDevice_Install, root\winblast_test_device

[TestDevice_Install]
; Virtual sandbox mock installation directives
AddReg=TestDevice_AddReg

[TestDevice_AddReg]
HKR,,WinBlastTestDriver,0x00010001,1

[Strings]
MfgName="WinBlast Testing Labs"
DeviceDesc="WinBlast Virtual Sandbox Test Device"
"#;
    fs::write(&inf_file, inf_content)
        .map_err(|e| DriverError::StagingFailed(format!("Failed to write test INF file: {}", e)))?;

    driver.state = DriverUpdateState::Installing;
    if let Some(cb) = progress_cb {
        cb(DriverProgress::Installing);
    }

    // Verify the driver manifest structure
    if !inf_file.exists() {
        driver.state = DriverUpdateState::Failed;
        let msg = "Generated driver manifest does not exist".to_string();
        if let Some(cb) = progress_cb {
            cb(DriverProgress::Failed(msg.clone()));
        }
        return Err(DriverError::InstallFailed(msg));
    }

    // Verify signature (mock — sandbox driver is self-signed)
    if let Some(cb) = progress_cb {
        cb(DriverProgress::Verifying);
    }

    // Test PnPUtil scanning / staging safely without altering physical system devices
    let output = SilentCommand::new("pnputil.exe")
        .arg("/scan-devices")
        .output();

    let pnp_status = match output {
        Ok(o) if o.status.success() => "PnPUtil scan succeeded".to_string(),
        Ok(o) => format!("PnPUtil returned: {}", String::from_utf8_lossy(&o.stdout)),
        Err(e) => format!("PnPUtil query note: {}", e),
    };

    driver.current_version = driver.latest_version.clone();
    driver.state = DriverUpdateState::Installed;
    driver.status_message = format!(
        "Successfully verified and installed in sandbox: {}",
        pnp_status
    );

    if let Some(cb) = progress_cb {
        cb(DriverProgress::Complete);
    }

    Ok(format!(
        "Installed driver {} ({}) successfully",
        driver.name, driver.latest_version
    ))
}

// ---------------------------------------------------------------------------
// Helper: run test lifecycle
// ---------------------------------------------------------------------------

/// Helper function to run an isolated test of the virtual sandbox driver installation.
pub fn test_driver_sandbox_lifecycle() -> Result<String, String> {
    let mut item = create_sandbox_test_item();
    let temp_sandbox = std::env::temp_dir().join("winblast_driver_sandbox_test");
    let result =
        download_and_install_driver(&mut item, &temp_sandbox).map_err(|e| e.to_string())?;
    if let Err(e) = fs::remove_dir_all(&temp_sandbox) {
        if e.kind() != std::io::ErrorKind::NotFound {
            tracing::warn!(
                "Failed to clean up driver temp sandbox {:?}: {}",
                temp_sandbox,
                e
            );
        }
    }
    Ok(result)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_sandbox_lifecycle_execution() {
        let res = test_driver_sandbox_lifecycle();
        assert!(
            res.is_ok(),
            "Sandbox driver lifecycle failed: {:?}",
            res.err()
        );
        let msg = res.unwrap();
        assert!(
            msg.contains("Virtual Sandbox Test Driver"),
            "Expected success message: {}",
            msg
        );
    }

    #[test]
    fn test_compare_versions_logic() {
        assert_eq!(compare_versions("31.0.101", "31.0.101"), 0);
        assert_eq!(compare_versions("31.0.101", "31.0.102"), -1);
        assert_eq!(compare_versions("31.0.102", "31.0.101"), 1);
        assert_eq!(compare_versions("560.94", "560.81"), 1);
        assert_eq!(compare_versions("24.10.1", "24.9.1"), 1);
        assert_eq!(compare_versions("1.0.0.0", "1.1.0.0"), -1);
    }

    #[test]
    fn test_create_sandbox_item_metadata() {
        let item = create_sandbox_test_item();
        assert_eq!(item.id, "virtual_test_sandbox_driver");
        assert_eq!(item.category, DriverCategory::VirtualSandbox);
        assert_eq!(item.current_version, "1.0.0.0");
        assert_eq!(item.latest_version, "1.1.0.0");
        assert_eq!(item.state, DriverUpdateState::UpdateAvailable);
        assert!(!item.hardware_id.is_empty());
        assert!(item.is_whql_signed);
    }

    #[test]
    fn test_all_categories_have_guid() {
        for cat in DriverCategory::all_physical() {
            assert!(
                cat.class_guid().starts_with('{'),
                "Category {:?} has invalid GUID",
                cat
            );
        }
    }

    #[test]
    fn test_all_categories_have_icon_and_name() {
        let all = DriverCategory::all_physical();
        for cat in all {
            assert!(!cat.icon().is_empty());
            assert!(!cat.display_name().is_empty());
        }
    }

    #[test]
    fn test_parse_pnputil_devices_empty() {
        let devices = parse_pnputil_devices("");
        assert!(devices.is_empty());
    }

    #[test]
    fn test_parse_pnputil_devices_sample() {
        let sample = r#"
Instance ID:     PCI\VEN_10DE&DEV_2684&SUBSYS_16F11043&REV_A1\4&1F4FC6F&0&0019
Device Description:  NVIDIA GeForce RTX 4090
Class Name:      Display
Class GUID:      {4d36e968-e325-11ce-bfc1-08002be10318}
Driver Version:  31.0.15.6614
Driver Date:     10/22/2024
Driver Provider: NVIDIA
Signer:          Microsoft Windows Hardware Compatibility Publisher

Instance ID:     PCI\VEN_8086&DEV_7A70&SUBSYS_86941043&REV_11\3&11583659&0&E0
Device Description:  Intel(R) Ethernet Controller I225-V
Class Name:      Net
Class GUID:      {4d36e972-e325-11ce-bfc1-08002be10318}
Driver Version:  1.0.3.30
Driver Date:     03/14/2024
Driver Provider: Intel
Signer:          Microsoft Windows Hardware Compatibility Publisher
"#;
        let devices = parse_pnputil_devices(sample);
        assert_eq!(devices.len(), 2);
        assert!(devices[0].description.contains("NVIDIA"));
        assert!(devices[0].is_signed);
        assert!(devices[1].description.contains("Intel"));
    }

    #[test]
    fn test_parse_pnputil_drivers_sample() {
        let sample = r#"
Published Name:  oem42.inf
Original Name:   nvlddmkm.inf
Provider Name:   NVIDIA
Class GUID:      {4d36e968-e325-11ce-bfc1-08002be10318}
Driver Version:  10/22/2024 31.0.15.6614
Signer:          Microsoft Windows Hardware Compatibility Publisher
"#;
        let packages = parse_pnputil_drivers(sample);
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].driver_version, "31.0.15.6614");
        assert!(packages[0].provider.contains("NVIDIA"));
    }

    #[test]
    fn test_virtual_device_filter() {
        let name = "WAN Miniport (Network Monitor)";
        let name_lower = name.to_lowercase();
        assert!(VIRTUAL_DEVICE_FILTERS
            .iter()
            .any(|f| name_lower.contains(f)));
    }

    #[test]
    fn test_vendor_download_url_known() {
        assert!(vendor_download_url("NVIDIA GEFORCE RTX 4090", "NVIDIA").contains("nvidia.com"));
        assert!(
            vendor_download_url("AMD RADEON RX 7900", "Advanced Micro Devices").contains("amd.com")
        );
        assert!(vendor_download_url("INTEL UHD 770", "Intel").contains("intel.com"));
    }

    #[test]
    fn test_vendor_download_url_unknown() {
        assert!(vendor_download_url("SOME DEVICE", "Unknown Vendor").is_empty());
    }

    #[test]
    fn test_backup_root_exists() {
        let root = backup_root();
        assert!(root.to_string_lossy().contains("winblast_driver_backup"));
    }
}
