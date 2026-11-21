//! Fast native Win32 Hardware, Memory DIMM, Storage Volume, and Security Posture Inventory.
//!
//! Uses direct Win32 kernel/SMBIOS/IOCTL APIs (`GlobalMemoryStatusEx`, `GetSystemFirmwareTable(RSMB)`,
//! `GetFirmwareType`, `GetDiskFreeSpaceExW`, `IOCTL_STORAGE_QUERY_PROPERTY`) with zero PowerShell or
//! blocking WMI loops.

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDrives, GetVolumeInformationW,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{
    PropertyStandardQuery, StorageDeviceProperty, StorageDeviceSeekPenaltyProperty,
    DEVICE_SEEK_PENALTY_DESCRIPTOR, IOCTL_STORAGE_QUERY_PROPERTY, STORAGE_DESCRIPTOR_HEADER,
    STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY,
};
use windows::Win32::System::SystemInformation::{
    FirmwareTypeBios, FirmwareTypeUefi, GetFirmwareType, GetSystemFirmwareTable, GetTickCount64,
    GlobalMemoryStatusEx, MEMORYSTATUSEX, RSMB,
};
use windows::Win32::System::IO::DeviceIoControl;

/// Individual RAM DIMM module parsed from SMBIOS Type 17.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamDimmInfo {
    pub locator: String,
    pub manufacturer: String,
    pub ddr_type: String,
    pub speed_mts: u32,
    pub capacity_mb: u64,
}

/// Storage media classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriveMediaKind {
    Nvme,
    Ssd,
    Hdd,
    Removable,
    Unknown,
}

impl DriveMediaKind {
    pub const fn as_label(self) -> &'static str {
        match self {
            DriveMediaKind::Nvme => "NVMe",
            DriveMediaKind::Ssd => "SSD",
            DriveMediaKind::Hdd => "HDD",
            DriveMediaKind::Removable => "Removable",
            DriveMediaKind::Unknown => "Disk",
        }
    }
}

/// Logical fixed storage volume summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageVolumeInfo {
    pub drive_letter: String,
    pub label: String,
    pub file_system: String,
    pub total_gb: f64,
    pub free_gb: f64,
    pub used_pct: f32,
    pub media_kind: DriveMediaKind,
    pub is_system_drive: bool,
}

/// Firmware & platform security telemetry (UEFI, Secure Boot, VBS, HVCI, Motherboard, Uptime).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlatformSecurityInfo {
    pub firmware_mode: String,
    pub secure_boot_enabled: Option<bool>,
    pub vbs_enabled: bool,
    pub hvci_enabled: bool,
    pub motherboard: String,
    pub bios_version: String,
    pub total_physical_ram_kb: u64,
    pub ram_summary: String,
    pub ram_dimms: Vec<RamDimmInfo>,
    pub volumes: Vec<StorageVolumeInfo>,
    pub uptime_hours: f64,
}

struct HandleGuard(HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

/// Query exact total physical RAM in KiB via `GlobalMemoryStatusEx`.
pub fn total_physical_ram_kb() -> u64 {
    let mut mem = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        ..Default::default()
    };
    if unsafe { GlobalMemoryStatusEx(&mut mem) }.is_ok() {
        mem.ullTotalPhys / 1024
    } else {
        16 * 1024 * 1024
    }
}

fn read_smbios_string(table_data: &[u8], strings_offset: usize, index: u8) -> String {
    if index == 0 || strings_offset >= table_data.len() {
        return String::new();
    }
    let mut cur = strings_offset;
    let mut idx = 1u8;
    while cur < table_data.len() {
        if table_data[cur] == 0 {
            break;
        }
        let end = table_data[cur..]
            .iter()
            .position(|&b| b == 0)
            .map(|rel| cur + rel)
            .unwrap_or(table_data.len());
        if idx == index {
            return String::from_utf8_lossy(&table_data[cur..end])
                .trim()
                .to_string();
        }
        cur = end + 1;
        idx = idx.saturating_add(1);
    }
    String::new()
}

fn smbios_mem_type_name(code: u8) -> &'static str {
    match code {
        0x12 => "DDR",
        0x13 => "DDR2",
        0x18 => "DDR3",
        0x1A => "DDR4",
        0x1B => "LPDDR",
        0x1C => "LPDDR2",
        0x1D => "LPDDR3",
        0x1E => "LPDDR4",
        0x22 => "DDR5",
        0x23 => "LPDDR5",
        _ => "DRAM",
    }
}

/// Parse populated SMBIOS Type 17 (Memory Device) DIMMs directly from firmware tables (`GetSystemFirmwareTable`).
pub fn query_ram_dimms() -> Vec<RamDimmInfo> {
    let size = unsafe { GetSystemFirmwareTable(RSMB, 0, None) };
    if size == 0 || size > 2 * 1024 * 1024 {
        return Vec::new();
    }
    let mut buf = vec![0u8; size as usize];
    let written =
        (unsafe { GetSystemFirmwareTable(RSMB, 0, Some(&mut buf)) } as usize).min(buf.len());
    if written < 8 {
        return Vec::new();
    }

    // RawSMBIOSData header is 8 bytes
    let data = &buf[8..written];
    let mut pos = 0usize;
    let mut dimms = Vec::new();

    while pos + 4 <= data.len() {
        let rec_type = data[pos];
        let rec_len = data[pos + 1] as usize;
        if rec_len < 4 || pos + rec_len > data.len() {
            break;
        }

        // Find string-set terminator (\0\0) starting after fixed struct bytes
        let mut s_end = pos + rec_len;
        while s_end + 1 < data.len() {
            if data[s_end] == 0 && data[s_end + 1] == 0 {
                break;
            }
            s_end += 1;
        }
        let next_pos = (s_end + 2).min(data.len());
        let rec = &data[pos..next_pos];

        if rec_type == 17 && rec_len >= 0x1B {
            let raw_size = u16::from_le_bytes([rec[0x0C], rec[0x0D]]);
            let cap_mb: u64 = if raw_size == 0 || raw_size == 0xFFFF {
                0
            } else if raw_size == 0x7FFF && rec_len >= 0x20 {
                u32::from_le_bytes([rec[0x1C], rec[0x1D], rec[0x1E], rec[0x1F]]) as u64
            } else if (raw_size & 0x8000) != 0 {
                // Size in KiB
                ((raw_size & 0x7FFF) as u64) / 1024
            } else {
                raw_size as u64
            };

            if cap_mb > 0 {
                let locator = read_smbios_string(rec, rec_len, rec[0x10]);
                let mem_type_code = rec[0x12];
                let nominal_speed = u16::from_le_bytes([rec[0x15], rec[0x16]]) as u32;
                let manufacturer = if rec_len > 0x17 {
                    read_smbios_string(rec, rec_len, rec[0x17])
                } else {
                    String::new()
                };
                let configured_speed = if rec_len >= 0x22 {
                    let cs = u16::from_le_bytes([rec[0x20], rec[0x21]]) as u32;
                    if cs > 0 && cs != 0xFFFF {
                        cs
                    } else {
                        nominal_speed
                    }
                } else {
                    nominal_speed
                };

                dimms.push(RamDimmInfo {
                    locator: if locator.is_empty() {
                        format!("DIMM {}", dimms.len() + 1)
                    } else {
                        locator
                    },
                    manufacturer: if manufacturer.is_empty() {
                        "Unknown".to_string()
                    } else {
                        manufacturer
                    },
                    ddr_type: smbios_mem_type_name(mem_type_code).to_string(),
                    speed_mts: configured_speed,
                    capacity_mb: cap_mb,
                });
            }
        }

        if rec_type == 127 {
            break;
        }
        pos = next_pos;
    }

    dimms
}

fn probe_drive_media_kind(letter: char) -> DriveMediaKind {
    // Open volume handle without read/write data access (GENERIC_READ=0 metadata query)
    let path_utf16: Vec<u16> = format!("\\\\.\\{}:", letter)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let Ok(handle) = (unsafe {
        CreateFileW(
            PCWSTR(path_utf16.as_ptr()),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        )
    }) else {
        return DriveMediaKind::Unknown;
    };
    let _guard = HandleGuard(handle);

    // 1. Check StorageDeviceProperty BusType (17 = BusTypeNvme)
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        ..Default::default()
    };
    let mut header = STORAGE_DESCRIPTOR_HEADER::default();
    let mut bytes_ret = 0u32;
    let ok_dev = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&mut query as *mut STORAGE_PROPERTY_QUERY).cast()),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some((&mut header as *mut STORAGE_DESCRIPTOR_HEADER).cast()),
            std::mem::size_of::<STORAGE_DESCRIPTOR_HEADER>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };
    if ok_dev.is_ok() && header.Size as usize >= std::mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>() {
        let sz = (header.Size as usize).min(4096);
        let mut buf = vec![0u8; sz];
        let ok_full = unsafe {
            DeviceIoControl(
                handle,
                IOCTL_STORAGE_QUERY_PROPERTY,
                Some((&mut query as *mut STORAGE_PROPERTY_QUERY).cast()),
                std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
                Some(buf.as_mut_ptr().cast()),
                sz as u32,
                Some(&mut bytes_ret),
                None,
            )
        };
        if ok_full.is_ok() && bytes_ret as usize >= std::mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>()
        {
            let desc = unsafe {
                std::ptr::read_unaligned(buf.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR)
            };
            // STORAGE_BUS_TYPE::BusTypeNvme == 17
            if desc.BusType.0 == 17 {
                return DriveMediaKind::Nvme;
            }
        }
    }

    // 2. Check Seek Penalty (false => SSD, true => HDD)
    let mut seek_q = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceSeekPenaltyProperty,
        QueryType: PropertyStandardQuery,
        ..Default::default()
    };
    let mut seek_desc = DEVICE_SEEK_PENALTY_DESCRIPTOR::default();
    bytes_ret = 0;
    let ok_seek = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&mut seek_q as *mut STORAGE_PROPERTY_QUERY).cast()),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some((&mut seek_desc as *mut DEVICE_SEEK_PENALTY_DESCRIPTOR).cast()),
            std::mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };
    if ok_seek.is_ok() && bytes_ret >= std::mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32
    {
        return if seek_desc.IncursSeekPenalty {
            DriveMediaKind::Hdd
        } else {
            DriveMediaKind::Ssd
        };
    }

    DriveMediaKind::Unknown
}

/// Enumerate all fixed storage volumes (`C:\`, `D:\`, ...) with space stats and NVMe/SSD/HDD type.
pub fn query_storage_volumes() -> Vec<StorageVolumeInfo> {
    let sys_drive_letter = std::env::var("SystemDrive")
        .ok()
        .and_then(|s| s.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .unwrap_or('C');

    let mask = unsafe { GetLogicalDrives() };
    let mut vols = Vec::new();
    const DRIVE_FIXED: u32 = 3;

    for bit in 0..26u32 {
        if (mask & (1u32 << bit)) == 0 {
            continue;
        }
        let letter = (b'A' + bit as u8) as char;
        let root_str = format!("{}:\\", letter);
        let root_w: Vec<u16> = root_str.encode_utf16().chain(std::iter::once(0)).collect();
        if unsafe { GetDriveTypeW(PCWSTR(root_w.as_ptr())) } != DRIVE_FIXED {
            continue;
        }

        let mut free_avail = 0u64;
        let mut total_bytes = 0u64;
        let mut total_free = 0u64;
        let ok_space = unsafe {
            GetDiskFreeSpaceExW(
                PCWSTR(root_w.as_ptr()),
                Some(&mut free_avail),
                Some(&mut total_bytes),
                Some(&mut total_free),
            )
        }
        .is_ok();
        if !ok_space || total_bytes == 0 {
            continue;
        }

        let mut vol_name_buf = [0u16; 128];
        let mut fs_name_buf = [0u16; 32];
        let ok_vol = unsafe {
            GetVolumeInformationW(
                PCWSTR(root_w.as_ptr()),
                Some(&mut vol_name_buf),
                None,
                None,
                None,
                Some(&mut fs_name_buf),
            )
        }
        .is_ok();

        let (label, file_system) = if ok_vol {
            let l_end = vol_name_buf.iter().position(|&c| c == 0).unwrap_or(0);
            let f_end = fs_name_buf.iter().position(|&c| c == 0).unwrap_or(0);
            (
                String::from_utf16_lossy(&vol_name_buf[..l_end]),
                String::from_utf16_lossy(&fs_name_buf[..f_end]),
            )
        } else {
            (String::new(), "NTFS".to_string())
        };

        let gb = 1024.0 * 1024.0 * 1024.0;
        let total_gb = total_bytes as f64 / gb;
        let free_gb = free_avail as f64 / gb;
        let used_pct = (((total_bytes.saturating_sub(free_avail)) as f64 / total_bytes as f64)
            * 100.0)
            .clamp(0.0, 100.0) as f32;

        vols.push(StorageVolumeInfo {
            drive_letter: format!("{}:", letter),
            label: if label.is_empty() {
                "Local Disk".to_string()
            } else {
                label
            },
            file_system,
            total_gb,
            free_gb,
            used_pct,
            media_kind: probe_drive_media_kind(letter),
            is_system_drive: letter == sys_drive_letter,
        });
    }

    vols
}

#[repr(C)]
struct SystemCodeIntegrityInfo {
    length: u32,
    code_integrity_options: u32,
}

#[link(name = "ntdll")]
extern "system" {
    fn NtQuerySystemInformation(
        system_information_class: u32,
        system_information: *mut std::ffi::c_void,
        system_information_length: u32,
        return_length: *mut u32,
    ) -> i32;
}

/// Query live kernel runtime Code Integrity flags (`SystemCodeIntegrityInformation = 103`).
/// Prevents pending-reboot HVCI/VBS registry values from being falsely reported active.
fn query_live_code_integrity_options() -> Option<u32> {
    const SYSTEM_CODE_INTEGRITY_INFORMATION: u32 = 103;
    let mut info = SystemCodeIntegrityInfo {
        length: std::mem::size_of::<SystemCodeIntegrityInfo>() as u32,
        code_integrity_options: 0,
    };
    let mut ret_len = 0u32;
    let status = unsafe {
        NtQuerySystemInformation(
            SYSTEM_CODE_INTEGRITY_INFORMATION,
            (&mut info as *mut SystemCodeIntegrityInfo).cast(),
            std::mem::size_of::<SystemCodeIntegrityInfo>() as u32,
            &mut ret_len,
        )
    };
    if status == 0 {
        Some(info.code_integrity_options)
    } else {
        None
    }
}

/// Collect full platform firmware, security posture, RAM DIMMs, and storage volume snapshot.
pub fn detect_platform_security_info() -> PlatformSecurityInfo {
    let mut fw_type = Default::default();
    let firmware_mode = if unsafe { GetFirmwareType(&mut fw_type) }.is_ok() {
        if fw_type == FirmwareTypeUefi {
            "UEFI".to_string()
        } else if fw_type == FirmwareTypeBios {
            "Legacy BIOS".to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let secure_boot_enabled = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\SecureBoot\\State",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|k| k.get_value::<u32, _>("UEFISecureBootEnabled"))
        .ok()
        .map(|v| v == 1);

    let vbs_configured = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|k| k.get_value::<u32, _>("EnableVirtualizationBasedSecurity"))
        .unwrap_or(0)
        == 1;

    let hvci_configured = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\HypervisorEnforcedCodeIntegrity",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|k| k.get_value::<u32, _>("Enabled"))
        .unwrap_or(0)
        == 1;

    let secure_kernel_running = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\DeviceGuard\\Scenarios\\KeyGuard\\Status",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|k| k.get_value::<u32, _>("IsSecureKernelRunning"))
        .ok()
        .map(|v| v == 1);

    const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 0x0000_0400;
    const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 0x0000_1000;
    const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 0x0000_2000;

    let (vbs_enabled, hvci_enabled) = if let Some(opts) = query_live_code_integrity_options() {
        let hvci_live = (opts
            & (CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED
                | CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED))
            != 0;
        let ium_live = (opts & CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED) != 0;
        let vbs_live = ium_live || hvci_live || secure_kernel_running.unwrap_or(false);
        (vbs_live, hvci_live)
    } else {
        (
            secure_kernel_running.unwrap_or(vbs_configured),
            hvci_configured,
        )
    };

    let (motherboard, bios_version) = hklm
        .open_subkey_with_flags(
            "HARDWARE\\DESCRIPTION\\System\\BIOS",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .map(|k| {
            let mfg: String = k.get_value("BaseBoardManufacturer").unwrap_or_default();
            let prod: String = k.get_value("BaseBoardProduct").unwrap_or_default();
            let bver: String = k.get_value("BIOSVersion").unwrap_or_default();
            let board = format!("{} {}", mfg.trim(), prod.trim()).trim().to_string();
            (
                if board.is_empty() {
                    "Unknown Board".to_string()
                } else {
                    board
                },
                if bver.trim().is_empty() {
                    "Unknown".to_string()
                } else {
                    bver.trim().to_string()
                },
            )
        })
        .unwrap_or_else(|_| ("Unknown Board".to_string(), "Unknown".to_string()));

    let total_kb = total_physical_ram_kb();
    let ram_dimms = query_ram_dimms();
    let ram_summary = if !ram_dimms.is_empty() {
        let total_gb: u64 = ram_dimms.iter().map(|d| d.capacity_mb).sum::<u64>() / 1024;
        let ddr = ram_dimms
            .first()
            .map(|d| d.ddr_type.as_str())
            .unwrap_or("DRAM");
        let speed = ram_dimms.first().map(|d| d.speed_mts).unwrap_or(0);
        if speed > 0 {
            format!(
                "{} GB {} @ {} MT/s ({} DIMMs)",
                total_gb,
                ddr,
                speed,
                ram_dimms.len()
            )
        } else {
            format!("{} GB {} ({} DIMMs)", total_gb, ddr, ram_dimms.len())
        }
    } else {
        format!("{:.1} GB RAM", total_kb as f64 / (1024.0 * 1024.0))
    };

    let volumes = query_storage_volumes();
    let uptime_hours = (unsafe { GetTickCount64() } as f64) / 3_600_000.0;

    PlatformSecurityInfo {
        firmware_mode,
        secure_boot_enabled,
        vbs_enabled,
        hvci_enabled,
        motherboard,
        bios_version,
        total_physical_ram_kb: total_kb,
        ram_summary,
        ram_dimms,
        volumes,
        uptime_hours,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_and_platform_inventory_nonblocking() {
        let kb = total_physical_ram_kb();
        assert!(
            kb > 512 * 1024,
            "Expected > 512 MB physical RAM, got {} KiB",
            kb
        );
        let info = detect_platform_security_info();
        assert!(!info.firmware_mode.is_empty());
        assert!(
            !info.volumes.is_empty(),
            "Expected at least one fixed storage volume"
        );
    }
}
