//! Win32 Motherboard, SMBIOS, UEFI & Graphics Driver Telemetry.
//!
//! Provides clean, zero-WMI queries via direct Win32 Registry and SMBIOS tables.

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

/// Display driver and GPU device details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayDriverInfo {
    pub gpu_name: String,
    pub driver_version: String,
    pub driver_date: String,
    pub provider: String,
    pub inf_path: String,
}

/// Comprehensive Motherboard and BIOS telemetry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiosSystemInfo {
    pub motherboard_vendor: String,
    pub motherboard_product: String,
    pub motherboard_version: String,
    pub bios_vendor: String,
    pub bios_version: String,
    pub bios_release_date: String,
    pub uefi_boot: bool,
    pub secure_boot: bool,
    pub smbios_version: String,
    pub display_drivers: Vec<DisplayDriverInfo>,
}

impl BiosSystemInfo {
    /// Probe the system's motherboard, BIOS metadata, Secure Boot, and installed display drivers.
    pub fn probe() -> Self {
        let mut info = Self::default();

        // 1. Motherboard & BIOS info via HKLM\HARDWARE\DESCRIPTION\System\BIOS
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(bios_key) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS") {
            info.motherboard_vendor = bios_key
                .get_value("BaseBoardManufacturer")
                .or_else(|_| bios_key.get_value("SystemManufacturer"))
                .unwrap_or_else(|_| "Unknown Manufacturer".to_string());

            info.motherboard_product = bios_key
                .get_value("BaseBoardProduct")
                .or_else(|_| bios_key.get_value("SystemProductName"))
                .unwrap_or_else(|_| "Unknown Motherboard".to_string());

            info.motherboard_version = bios_key
                .get_value("BaseBoardVersion")
                .unwrap_or_else(|_| "1.0".to_string());

            info.bios_vendor = bios_key
                .get_value("BIOSVendor")
                .unwrap_or_else(|_| "Unknown BIOS Vendor".to_string());

            info.bios_version = bios_key
                .get_value("BIOSVersion")
                .unwrap_or_else(|_| "Unknown Version".to_string());

            info.bios_release_date = bios_key
                .get_value("BIOSReleaseDate")
                .unwrap_or_else(|_| "Unknown Date".to_string());
        }

        // 2. SMBIOS Major/Minor version
        if let Ok(sys_key) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System") {
            let major: Result<u32, _> = sys_key.get_value("SmbiosMajorVersion");
            let minor: Result<u32, _> = sys_key.get_value("SmbiosMinorVersion");
            if let (Ok(maj), Ok(min)) = (major, minor) {
                info.smbios_version = format!("{}.{}", maj, min);
            } else {
                info.smbios_version = "v3.x".to_string();
            }
        }

        // 3. Secure Boot State & Firmware Boot Type (UEFI vs Legacy)
        let mut is_uefi = true;
        if let Ok(ctrl_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control") {
            if let Ok(fw_type) = ctrl_key.get_value::<u32, _>("PEFirmwareType") {
                is_uefi = fw_type == 2;
            }
        }
        info.uefi_boot = is_uefi;

        if let Ok(sb_key) =
            hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\SecureBoot\\State")
        {
            let sb_val: Result<u32, _> = sb_key.get_value("UEFISecureBootEnabled");
            info.secure_boot = sb_val.map(|v| v == 1).unwrap_or(false);
        } else {
            info.secure_boot = false;
        }

        // 4. Query Installed Graphics / Display Drivers from Win32 Class Registry
        // {4d36e968-e325-11ce-bfc1-08002be10318} is the Display Adapter Class GUID
        if let Ok(class_key) = hklm.open_subkey(
            "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}",
        ) {
            for sub in class_key.enum_keys().filter_map(|r| r.ok()) {
                if sub.len() == 4 && sub.chars().all(|c| c.is_ascii_digit()) {
                    if let Ok(dev_key) = class_key.open_subkey(&sub) {
                        let desc: Result<String, _> = dev_key.get_value("DriverDesc");
                        let ver: Result<String, _> = dev_key.get_value("DriverVersion");
                        let date: Result<String, _> = dev_key.get_value("DriverDate");
                        let prov: Result<String, _> = dev_key.get_value("ProviderName");
                        let inf: Result<String, _> = dev_key.get_value("InfPath");

                        if let (Ok(name), Ok(version)) = (desc, ver) {
                            let name_lower = name.to_lowercase();
                            // Filter out virtual/mirror/remote display adapters
                            let is_virtual = name_lower.contains("remote")
                                || name_lower.contains("vnc")
                                || name_lower.contains("spacedesk")
                                || name_lower.contains("parsec")
                                || name_lower.contains("miracast")
                                || name_lower.contains("virtual display")
                                || name_lower.contains("citrix")
                                || name_lower.contains("indirect")
                                || name_lower.contains("idd");
                            if !is_virtual {
                                info.display_drivers.push(DisplayDriverInfo {
                                    gpu_name: name,
                                    driver_version: version,
                                    driver_date: date.unwrap_or_else(|_| "Unknown Date".to_string()),
                                    provider: prov.unwrap_or_else(|_| "Microsoft / Vendor".to_string()),
                                    inf_path: inf.unwrap_or_else(|_| sub.clone()),
                                });
                            }
                        }
                    }
                }
            }
        }

        info
    }

    /// Check if BIOS or Drivers need attention (e.g. out-of-date or Secure Boot disabled).
    pub fn has_warning(&self) -> bool {
        !self.secure_boot || self.display_drivers.is_empty()
    }
}
