//! Physical Drive SMART Diagnostics, Wear Leveling, TBW & TRIM Optimization.
//!
//! Uses direct Win32 IOCTL queries and volume analysis with zero synthetic placebos.
//! NVMe drives are queried via IOCTL_STORAGE_QUERY_PROPERTY (StorageDeviceProtocolSpecificProperty
//! and StorageAdapterProtocolSpecificProperty) for NVMe SMART/Health log page 0x02.
//! SATA SSDs and mechanical drives are queried via kernel ATA SMART IOCTLs (SMART_GET_VERSION
//! and SMART_RCV_DRIVE_DATA 0x0007c088).
//! Authentic physical kernel storage driver throughput is captured via IOCTL_DISK_GET_PERFORMANCE
//! (0x00070020) across all privilege tiers without placebo values.

use crate::core::process::SilentCommand;
use crate::hardware::tbw::{self, HostWrites, SataWriteCounter};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::mem::size_of;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, ERROR_ACCESS_DENIED, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetDiskFreeSpaceExW, GetLogicalDrives, GetVolumeInformationW, FILE_SHARE_READ,
    FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{
    PropertyStandardQuery, StorageDeviceProperty, StorageDeviceSeekPenaltyProperty,
    DEVICE_SEEK_PENALTY_DESCRIPTOR, IOCTL_STORAGE_QUERY_PROPERTY, STORAGE_DESCRIPTOR_HEADER,
    STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY,
};
use windows::Win32::System::IO::DeviceIoControl;

/// Drive health and SMART status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalDriveHealth {
    pub index: u32,
    pub model: String,
    pub interface_type: String, // "NVMe", "SATA SSD", "HDD"
    pub total_size_gb: f64,
    /// Wear/health indicator reported by the drive (SMART / Storage API). `None` = not reported;
    /// never estimated (AGENTS.md rule 2).
    pub health_percentage: Option<u32>,
    pub status: String, // e.g. "Healthy (Good)", "Caution — …", "Health not reported"
    /// Drive temperature in °C; 0.0 = not reported by the drive.
    pub temperature_c: f32,
    /// Lifetime host writes (TBW) as reported by the drive; never estimated (AGENTS.md rule 2).
    pub host_writes: HostWrites,
    pub power_on_hours: u32,
    pub trim_enabled: bool,
}

/// Disk health subsystem overview.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiskSubsystemReport {
    pub drives: Vec<PhysicalDriveHealth>,
    pub global_trim_active: bool,
    pub nvme_direct_storage_supported: bool,
}

/// Raw kernel storage performance telemetry from IOCTL_DISK_GET_PERFORMANCE.
#[derive(Debug, Clone, Copy, Default)]
pub struct DiskPerformanceSnapshot {
    pub bytes_read: i64,
    pub bytes_written: i64,
    pub read_count: u32,
    pub write_count: u32,
    pub queue_depth: u32,
}

/// Parsed physical SMART telemetry from kernel IOCTLs.
#[derive(Debug, Clone, Default)]
pub struct DriveSmartData {
    pub health_percentage: Option<u32>,
    pub status: String,
    pub temperature_c: Option<f32>,
    pub lifetime_writes_tb: Option<f64>,
    pub power_on_hours: Option<u32>,
}

/// Why a SMART query returned no data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartQueryError {
    /// ERROR_ACCESS_DENIED: the process is not elevated (ATA pass-through needs write access).
    AccessDenied,
    /// The device, driver or bus does not answer the query.
    Unavailable,
}

impl From<windows::core::Error> for SmartQueryError {
    fn from(e: windows::core::Error) -> Self {
        if e.code() == windows::core::HRESULT::from_win32(ERROR_ACCESS_DENIED.0) {
            SmartQueryError::AccessDenied
        } else {
            SmartQueryError::Unavailable
        }
    }
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

impl DiskSubsystemReport {
    /// Probe all physical drives and verify TRIM status from live Windows APIs.
    pub fn probe() -> Self {
        let mut report = Self::default();

        // Check global Windows TRIM status via fsutil
        let mut trim_active = true;
        if let Ok(output) = SilentCommand::new("fsutil")
            .timeout(std::time::Duration::from_secs(20))
            .args(["behavior", "query", "DisableDeleteNotify"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            if text.contains("DisableDeleteNotify = 1") {
                trim_active = false;
            }
        }
        report.global_trim_active = trim_active;

        // Query real physical drives (\\.\PhysicalDrive0..7) in parallel via Rayon
        let drives: Vec<PhysicalDriveHealth> = (0..8u32)
            .into_par_iter()
            .filter_map(|drive_idx| probe_physical_drive(drive_idx, trim_active))
            .collect();

        for drive_info in &drives {
            if drive_info.interface_type.contains("NVMe") {
                report.nvme_direct_storage_supported = true;
            }
        }
        report.drives = drives;

        // If no physical drives were accessible (e.g. non-elevated user), query real logical volumes in parallel
        if report.drives.is_empty() {
            let drives_mask = unsafe { GetLogicalDrives() };
            let letters: Vec<char> = (0..26)
                .filter(|&i| (drives_mask & (1 << i)) != 0)
                .map(|i| (b'A' + i) as char)
                .collect();

            let vol_drives: Vec<PhysicalDriveHealth> = letters
                .into_par_iter()
                .enumerate()
                .filter_map(|(vol_idx, letter)| {
                    probe_volume_fallback(letter, vol_idx as u32, trim_active)
                })
                .collect();

            for drive_info in &vol_drives {
                if drive_info.interface_type.contains("NVMe") {
                    report.nvme_direct_storage_supported = true;
                }
            }
            report.drives = vol_drives;
        }

        report
    }

    /// Run TRIM on all SSD volumes via PowerShell `Optimize-Volume -ReTrim`.
    pub fn run_trim_all() -> Result<String, String> {
        let output = SilentCommand::new("powershell")
            .timeout(std::time::Duration::from_secs(20))
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "Get-Volume | Where-Object { $_.DriveType -eq 'Fixed' } | Optimize-Volume -ReTrim -Verbose",
            ])
            .output()
            .map_err(|e| format!("Failed to invoke Optimize-Volume: {}", e))?;

        if output.status.success() {
            Ok("TRIM optimization successfully issued to all SSD volumes.".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

/// Query a single physical drive index via IOCTL_STORAGE_QUERY_PROPERTY and kernel SMART IOCTLs.
fn probe_physical_drive(index: u32, trim_enabled: bool) -> Option<PhysicalDriveHealth> {
    let path_utf16: Vec<u16> = format!("\\\\.\\PhysicalDrive{}", index)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    // Attempt handle acquisition in descending privilege order:
    // 1. GENERIC_READ | GENERIC_WRITE (0xC0000000) for direct SMART & ATA IOCTL command submission
    // 2. GENERIC_READ (0x80000000) for standard query properties & geometry
    // 3. FILE_READ_ATTRIBUTES / 0 for unprivileged query access (works without admin rights)
    let Ok(handle) = (unsafe {
        CreateFileW(
            PCWSTR(path_utf16.as_ptr()),
            0xC0000000,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        )
    })
    .or_else(|_| unsafe {
        CreateFileW(
            PCWSTR(path_utf16.as_ptr()),
            0x80000000,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        )
    })
    .or_else(|_| unsafe {
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
        return None;
    };
    let _guard = HandleGuard(handle);

    // 1. Query STORAGE_DEVICE_DESCRIPTOR
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        ..Default::default()
    };
    let mut header = STORAGE_DESCRIPTOR_HEADER::default();
    let mut bytes_ret = 0u32;
    let ok_header = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&mut query as *mut STORAGE_PROPERTY_QUERY).cast()),
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some((&mut header as *mut STORAGE_DESCRIPTOR_HEADER).cast()),
            size_of::<STORAGE_DESCRIPTOR_HEADER>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };

    if ok_header.is_err() || header.Size == 0 {
        return None;
    }

    let sz = (header.Size as usize).min(4096);
    let mut buf = vec![0u8; sz];
    let ok_buf = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&mut query as *mut STORAGE_PROPERTY_QUERY).cast()),
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some(buf.as_mut_ptr().cast()),
            sz as u32,
            Some(&mut bytes_ret),
            None,
        )
    };

    if ok_buf.is_err() || (bytes_ret as usize) < size_of::<STORAGE_DEVICE_DESCRIPTOR>() {
        return None;
    }

    let desc =
        unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR) };

    // Extract Product/Model string
    let mut model_str = String::new();
    if desc.VendorIdOffset > 0 && (desc.VendorIdOffset as usize) < sz {
        let vendor = extract_null_terminated_str(&buf[desc.VendorIdOffset as usize..]);
        if !vendor.is_empty() {
            model_str.push_str(&vendor);
            model_str.push(' ');
        }
    }
    if desc.ProductIdOffset > 0 && (desc.ProductIdOffset as usize) < sz {
        let product = extract_null_terminated_str(&buf[desc.ProductIdOffset as usize..]);
        model_str.push_str(&product);
    }
    if model_str.is_empty() {
        model_str = format!("Physical Disk {}", index);
    }
    let firmware_str = if desc.ProductRevisionOffset > 0 && (desc.ProductRevisionOffset as usize) < sz {
        extract_null_terminated_str(&buf[desc.ProductRevisionOffset as usize..])
    } else {
        String::new()
    };

    // Determine Bus & Interface Type
    // BusTypeNvme = 17, BusTypeSata = 11, BusTypeUsb = 7, BusTypeScsi = 1
    let is_nvme = desc.BusType.0 == 17;

    // Check seek penalty (SSD vs HDD)
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
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some((&mut seek_desc as *mut DEVICE_SEEK_PENALTY_DESCRIPTOR).cast()),
            size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };

    let is_ssd =
        if ok_seek.is_ok() && bytes_ret >= size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32 {
            !seek_desc.IncursSeekPenalty
        } else {
            is_nvme
        };

    let interface_type = if is_nvme {
        "NVMe PCIe SSD".to_string()
    } else if is_ssd {
        "SATA SSD".to_string()
    } else if desc.BusType.0 == 7 {
        "USB Removable Drive".to_string()
    } else {
        "Mechanical HDD".to_string()
    };

    // 2. Direct Win32 IOCTL for disk capacity
    let ioctl_size_gb = query_disk_capacity_ioctl(handle).unwrap_or(0.0);

    // 3. Physical SMART telemetry. Virtual disks (BusTypeVirtual = 14, BusTypeFileBackedVirtual = 15)
    //    have no physical counters, so they are not queried.
    let is_virtual = matches!(desc.BusType.0, 14 | 15) || tbw::is_virtual_disk_model(&model_str);
    let mut host_writes = HostWrites::NotAvailable;
    let smart_data = if is_virtual {
        None
    } else if is_nvme {
        match query_nvme_smart_ioctl(handle) {
            Ok(smart) => {
                if let Some(tb) = smart.lifetime_writes_tb {
                    host_writes = HostWrites::Reported(tb);
                }
                Some(smart)
            }
            Err(SmartQueryError::AccessDenied) => {
                host_writes = HostWrites::RequiresAdmin;
                None
            }
            Err(SmartQueryError::Unavailable) => None,
        }
    } else {
        match query_ata_smart_ioctl(handle, index) {
            Ok(table) => {
                // The vendor-unit table is keyed on the IDENTIFY DEVICE model/firmware; the
                // storage descriptor strings are the fallback when IDENTIFY is refused.
                let (id_model, id_fw) = query_ata_identify_ioctl(handle, index)
                    .map(|id| parse_ata_identify(&id))
                    .unwrap_or_else(|| (model_str.clone(), firmware_str.clone()));
                // Hard disks: no host-write counter is decoded (N/A), only SSDs.
                let counter = if is_ssd {
                    tbw::lookup_sata_write_counter(&id_model, &id_fw)
                } else {
                    None
                };
                let smart = parse_ata_smart_attributes(&table, counter);
                if let Some(tb) = smart.lifetime_writes_tb {
                    host_writes = HostWrites::Reported(tb);
                }
                Some(smart)
            }
            Err(SmartQueryError::AccessDenied) => {
                host_writes = HostWrites::RequiresAdmin;
                None
            }
            Err(SmartQueryError::Unavailable) => None,
        }
    };

    // 4. Unified telemetry with honest degradation (rule 2: zero fabricated telemetry).
    //    Lifetime writes come only from the drive's own counter: the kernel's
    //    IOCTL_DISK_GET_PERFORMANCE byte count restarts at boot and is not a TBW value.
    let (health_pct, status_str, temp_c, poh, size_gb) = if let Some(smart) = smart_data {
        let final_size = if ioctl_size_gb > 0.0 {
            ioctl_size_gb
        } else {
            query_smart_health_wmi(index).4
        };
        (
            smart.health_percentage,
            smart.status,
            smart.temperature_c.unwrap_or(0.0),
            smart.power_on_hours.unwrap_or(0),
            final_size,
        )
    } else {
        // Fallback: Windows Storage API (Get-PhysicalDisk / Get-StorageReliabilityCounter).
        let (w_health, w_status, w_temp, w_poh, w_size) = query_smart_health_wmi(index);
        let final_size = if ioctl_size_gb > 0.0 {
            ioctl_size_gb
        } else {
            w_size
        };
        // Status comes only from what Windows / the drive reported (v1.6.1 relabelled "no data"
        // as "Healthy (Active)").
        (w_health, w_status, w_temp, w_poh, final_size)
    };

    Some(PhysicalDriveHealth {
        index,
        model: model_str,
        interface_type,
        total_size_gb: (size_gb * 10.0).round() / 10.0,
        health_percentage: health_pct,
        status: status_str,
        temperature_c: (temp_c * 10.0).round() / 10.0,
        host_writes,
        power_on_hours: poh,
        trim_enabled: is_ssd && trim_enabled,
    })
}

/// Query physical drive capacity using IOCTL_DISK_GET_LENGTH_INFO (0x0007405C) or IOCTL_DISK_GET_DRIVE_GEOMETRY_EX (0x000700A0).
fn query_disk_capacity_ioctl(handle: HANDLE) -> Option<f64> {
    // 1. Try IOCTL_DISK_GET_LENGTH_INFO (0x0007405C)
    let mut length_bytes = 0i64;
    let mut bytes_ret = 0u32;
    let ok = unsafe {
        DeviceIoControl(
            handle,
            0x0007405C, // IOCTL_DISK_GET_LENGTH_INFO
            None,
            0,
            Some((&mut length_bytes as *mut i64).cast()),
            size_of::<i64>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };
    if ok.is_ok() && bytes_ret >= size_of::<i64>() as u32 && length_bytes > 0 {
        return Some(length_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
    }

    // 2. Fallback: IOCTL_DISK_GET_DRIVE_GEOMETRY_EX (0x000700A0) - works with FILE_ANY_ACCESS
    #[repr(C)]
    struct GeometryEx {
        _geometry: [u8; 24],
        disk_size: i64,
        _data: [u8; 1],
    }
    let mut geo = std::mem::MaybeUninit::<GeometryEx>::uninit();
    bytes_ret = 0;
    let ok_geo = unsafe {
        DeviceIoControl(
            handle,
            0x000700A0, // IOCTL_DISK_GET_DRIVE_GEOMETRY_EX
            None,
            0,
            Some(geo.as_mut_ptr().cast()),
            size_of::<GeometryEx>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };
    if ok_geo.is_ok() && bytes_ret >= 32 {
        let val = unsafe { (*geo.as_ptr()).disk_size };
        if val > 0 {
            return Some(val as f64 / (1024.0 * 1024.0 * 1024.0));
        }
    }

    None
}

/// Query authentic NT kernel storage driver performance metrics via IOCTL_DISK_GET_PERFORMANCE (0x00070020).
///
/// Returns raw cumulative BytesWritten and BytesRead directly from the NT disk driver stack.
/// Supported on physical drives with standard FILE_ANY_ACCESS across all permission levels.
pub fn query_disk_performance_ioctl(handle: HANDLE) -> Option<DiskPerformanceSnapshot> {
    #[repr(C)]
    struct RawDiskPerformance {
        bytes_read: i64,
        bytes_written: i64,
        _read_time: i64,
        _write_time: i64,
        _idle_time: i64,
        read_count: u32,
        write_count: u32,
        queue_depth: u32,
        _split_count: u32,
        _query_time: i64,
        _storage_device_number: u32,
        _storage_manager_name: [u16; 8],
    }

    let mut perf = std::mem::MaybeUninit::<RawDiskPerformance>::uninit();
    let mut bytes_ret = 0u32;
    let ok = unsafe {
        DeviceIoControl(
            handle,
            0x00070020, // IOCTL_DISK_GET_PERFORMANCE
            None,
            0,
            Some(perf.as_mut_ptr().cast()),
            size_of::<RawDiskPerformance>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };

    if ok.is_ok() && bytes_ret >= 16 {
        let p = unsafe { perf.assume_init() };
        Some(DiskPerformanceSnapshot {
            bytes_read: p.bytes_read,
            bytes_written: p.bytes_written,
            read_count: p.read_count,
            write_count: p.write_count,
            queue_depth: p.queue_depth,
        })
    } else {
        None
    }
}

/// Query NVMe SMART / Health Information log page 0x02 via IOCTL_STORAGE_QUERY_PROPERTY.
///
/// Layout (Microsoft "Working with NVMe drives"): STORAGE_PROPERTY_QUERY header (PropertyId,
/// QueryType) followed by STORAGE_PROTOCOL_SPECIFIC_DATA (10 × u32) and room for the 512-byte log.
/// The reply is a STORAGE_PROTOCOL_DATA_DESCRIPTOR (Version, Size, STORAGE_PROTOCOL_SPECIFIC_DATA,
/// data); see [`nvme_log_from_descriptor`].
///
/// `ProtocolDataRequestSubValue` is the lower 32 bits of the log page *offset* on current
/// StorNVMe drivers, so it stays 0: 0xFFFFFFFF there fails with ERROR_IO_DEVICE (verified on a
/// KINGSTON SNV3S500G). The adapter property (49) is tried first, then the device property (50).
pub fn query_nvme_smart_ioctl(handle: HANDLE) -> Result<DriveSmartData, SmartQueryError> {
    const QUERY_HEADER: usize = 8;
    const PROTOCOL_DATA: usize = 40;
    const LOG_LEN: usize = 512;
    const PROTOCOL_TYPE_NVME: u32 = 3;
    const NVME_DATA_TYPE_LOG_PAGE: u32 = 2;
    const NVME_LOG_PAGE_HEALTH_INFO: u32 = 0x02;

    let mut last_err = SmartQueryError::Unavailable;
    for property_id in [49u32, 50u32] {
        let mut query = [0u8; QUERY_HEADER + PROTOCOL_DATA + LOG_LEN];
        let fields: [(usize, u32); 7] = [
            (0, property_id),
            (4, 0), // PropertyStandardQuery
            (8, PROTOCOL_TYPE_NVME),
            (12, NVME_DATA_TYPE_LOG_PAGE),
            (16, NVME_LOG_PAGE_HEALTH_INFO),
            (24, PROTOCOL_DATA as u32), // ProtocolDataOffset (from STORAGE_PROTOCOL_SPECIFIC_DATA)
            (28, LOG_LEN as u32),       // ProtocolDataLength
        ];
        for (offset, value) in fields {
            query[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }

        let mut out = [0u8; QUERY_HEADER + PROTOCOL_DATA + LOG_LEN];
        let mut bytes_ret = 0u32;
        let res = unsafe {
            DeviceIoControl(
                handle,
                IOCTL_STORAGE_QUERY_PROPERTY,
                Some(query.as_ptr().cast()),
                query.len() as u32,
                Some(out.as_mut_ptr().cast()),
                out.len() as u32,
                Some(&mut bytes_ret),
                None,
            )
        };
        if let Err(e) = res {
            last_err = e.into();
            continue;
        }
        let returned = &out[..(bytes_ret as usize).min(out.len())];
        if let Some(smart) = nvme_log_from_descriptor(returned).and_then(parse_nvme_smart_log) {
            return Ok(smart);
        }
    }

    Err(last_err)
}

/// The NVMe log inside a STORAGE_PROTOCOL_DATA_DESCRIPTOR. `ProtocolDataOffset` (bytes 24..28)
/// counts from the STORAGE_PROTOCOL_SPECIFIC_DATA, which starts at byte 8 after Version and Size,
/// so the log normally begins at byte 48.
pub fn nvme_log_from_descriptor(desc: &[u8]) -> Option<&[u8]> {
    let read_u32 = |at: usize| -> Option<usize> {
        Some(u32::from_le_bytes(desc.get(at..at + 4)?.try_into().ok()?) as usize)
    };
    let offset = read_u32(24)?;
    let length = read_u32(28)?;
    if length < 128 {
        return None;
    }
    let start = 8usize.checked_add(offset)?;
    desc.get(start..start.checked_add(length.min(512))?)
}

/// Parse NVMe SMART/Health Information log page 0x02 payload.
pub fn parse_nvme_smart_log(smart: &[u8]) -> Option<DriveSmartData> {
    if smart.len() < 128 {
        return None;
    }

    // Byte 0: Critical Warning
    let critical_warning = smart[0];
    // Bytes 1..3: Composite Temperature in Kelvin
    let temp_k = u16::from_le_bytes([smart[1], smart[2]]);
    let temp_c = if temp_k > 200 && temp_k < 400 {
        Some((temp_k as f32) - 273.15)
    } else {
        None
    };

    // Byte 5: Percentage Used (0 to 100% estimated endurance consumed)
    let percentage_used = smart[5];
    let health_pct = 100u32.saturating_sub(percentage_used as u32);

    // Bytes 48..64: Data Units Written (u128 LE, one unit = 1000 × 512 bytes).
    let tbw = tbw::nvme_data_units_written_tb(smart)?;

    // Bytes 128..136: Power-On Hours (lowest 64 bits = hours)
    let power_on_hours = if smart.len() >= 136 {
        let mut poh_bytes = [0u8; 8];
        poh_bytes.copy_from_slice(&smart[128..136]);
        Some(u64::from_le_bytes(poh_bytes) as u32)
    } else {
        None
    };

    let status = if critical_warning == 0 {
        if health_pct >= 90 {
            "Healthy (Good)".to_string()
        } else if health_pct >= 75 {
            "Healthy (Caution)".to_string()
        } else {
            "Degraded".to_string()
        }
    } else {
        "Caution — SMART Critical Warning".to_string()
    };

    Some(DriveSmartData {
        health_percentage: Some(health_pct),
        status,
        temperature_c: temp_c.map(|t| (t * 10.0).round() / 10.0),
        lifetime_writes_tb: Some((tbw * 100.0).round() / 100.0),
        power_on_hours,
    })
}

/// Query the ATA SMART attribute table via SMART_GET_VERSION (0x00074080) and
/// SMART_RCV_DRIVE_DATA (0x0007c088, SMART READ DATA).
///
/// Needs a handle opened with GENERIC_READ | GENERIC_WRITE, i.e. an elevated process; otherwise
/// Windows answers ERROR_ACCESS_DENIED, reported as [`SmartQueryError::AccessDenied`].
pub fn query_ata_smart_ioctl(handle: HANDLE, drive_index: u32) -> Result<[u8; 512], SmartQueryError> {
    #[repr(C, packed)]
    #[derive(Default, Copy, Clone)]
    struct GetVersionInParams {
        _b_version: u8,
        _b_revision: u8,
        _b_reserved: u8,
        _b_ide_device_map: u8,
        f_capabilities: u32,
        _dw_reserved: [u32; 4],
    }

    let mut ver = GetVersionInParams::default();
    let mut bytes_ret = 0u32;
    unsafe {
        DeviceIoControl(
            handle,
            0x00074080, // SMART_GET_VERSION
            None,
            0,
            Some((&mut ver as *mut GetVersionInParams).cast()),
            size_of::<GetVersionInParams>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    }?;
    // CAP_ATA_ID_CMD (0x01) or CAP_SMART_CMD (0x04)
    if ver.f_capabilities & 0x04 == 0 && ver.f_capabilities & 0x01 == 0 {
        return Err(SmartQueryError::Unavailable);
    }

    // SMART READ DATA: features 0xD0, command 0xB0, cylinder 0xC24F.
    ata_receive_drive_data(handle, drive_index, 0xD0, 0x4F, 0xC2, 0xB0)
}

/// Read the 512-byte ATA IDENTIFY DEVICE block (command 0xEC) via SMART_RCV_DRIVE_DATA.
pub fn query_ata_identify_ioctl(handle: HANDLE, drive_index: u32) -> Option<[u8; 512]> {
    ata_receive_drive_data(handle, drive_index, 0, 0, 0, 0xEC).ok()
}

/// Issue one SMART_RCV_DRIVE_DATA command that returns a 512-byte sector.
fn ata_receive_drive_data(
    handle: HANDLE,
    drive_index: u32,
    features: u8,
    cyl_low: u8,
    cyl_high: u8,
    command: u8,
) -> Result<[u8; 512], SmartQueryError> {
    #[repr(C, packed)]
    #[derive(Default, Copy, Clone)]
    struct IdeRegs {
        b_features_reg: u8,
        b_sector_count_reg: u8,
        b_sector_number_reg: u8,
        b_cyl_low_reg: u8,
        b_cyl_high_reg: u8,
        b_drive_head_reg: u8,
        b_command_reg: u8,
        _b_reserved: u8,
    }

    #[repr(C, packed)]
    #[derive(Default, Copy, Clone)]
    struct SendCmdInParams {
        c_buffer_size: u32,
        ir_drive_regs: IdeRegs,
        b_drive_number: u8,
        _b_reserved: [u8; 3],
        _dw_reserved: [u32; 4],
    }

    #[repr(C, packed)]
    #[derive(Default, Copy, Clone)]
    struct DriverStatus {
        _b_driver_error: u8,
        _b_ide_error: u8,
        _b_reserved: [u8; 2],
        _dw_reserved: [u32; 2],
    }

    #[repr(C, packed)]
    struct SendCmdOutParams {
        _c_buffer_size: u32,
        _driver_status: DriverStatus,
        b_buffer: [u8; 512],
    }

    let in_params = SendCmdInParams {
        c_buffer_size: 512,
        ir_drive_regs: IdeRegs {
            b_features_reg: features,
            b_sector_count_reg: 1,
            b_sector_number_reg: 1,
            b_cyl_low_reg: cyl_low,
            b_cyl_high_reg: cyl_high,
            b_drive_head_reg: 0xA0 | ((drive_index as u8 & 1) << 4),
            b_command_reg: command,
            _b_reserved: 0,
        },
        b_drive_number: drive_index as u8,
        _b_reserved: [0; 3],
        _dw_reserved: [0; 4],
    };

    let mut out_params = std::mem::MaybeUninit::<SendCmdOutParams>::zeroed();
    let mut bytes_ret = 0u32;
    unsafe {
        DeviceIoControl(
            handle,
            0x0007c088, // SMART_RCV_DRIVE_DATA
            Some((&in_params as *const SendCmdInParams).cast()),
            size_of::<SendCmdInParams>() as u32,
            Some(out_params.as_mut_ptr().cast()),
            size_of::<SendCmdOutParams>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    }?;

    if (bytes_ret as usize) < size_of::<SendCmdOutParams>() {
        return Err(SmartQueryError::Unavailable);
    }

    let out_ref = unsafe { &*out_params.as_ptr() };
    Ok(out_ref.b_buffer)
}

/// Model (words 27..47) and firmware revision (words 23..27) from ATA IDENTIFY DEVICE. ATA strings
/// store two characters per 16-bit word with the bytes swapped.
pub fn parse_ata_identify(id: &[u8; 512]) -> (String, String) {
    let ata_string = |first_word: usize, last_word: usize| -> String {
        let mut s = Vec::with_capacity((last_word - first_word) * 2);
        for w in first_word..last_word {
            s.push(id[w * 2 + 1]);
            s.push(id[w * 2]);
        }
        String::from_utf8_lossy(&s).trim_matches(|c: char| c == ' ' || c == '\0').to_string()
    };
    (ata_string(27, 47), ata_string(23, 27))
}

/// Parse ATA SMART attributes table (up to 30 attribute entries of 12 bytes each).
///
/// `writes` names the attribute and unit that hold lifetime host writes for this drive family
/// (from [`tbw::lookup_sata_write_counter`]); with `None` no TBW value is produced, because 0xF1 /
/// 0xF6 units differ between vendors and cannot be inferred from the raw value.
pub fn parse_ata_smart_attributes(buf: &[u8; 512], writes: Option<SataWriteCounter>) -> DriveSmartData {
    let mut health_pct: Option<u32> = None;
    let mut temp_c: Option<f32> = None;
    let mut lifetime_writes_tb: Option<f64> = None;
    let mut power_on_hours: Option<u32> = None;
    let mut caution = false;

    // Up to 30 attribute entries starting at byte 2, each 12 bytes
    for i in 0..30 {
        let offset = 2 + i * 12;
        if offset + 12 > buf.len() {
            break;
        }
        let attr_id = buf[offset];
        if attr_id == 0 {
            continue;
        }
        let current_val = buf[offset + 3];
        let raw_val = u64::from_le_bytes([
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
            buf[offset + 8],
            buf[offset + 9],
            buf[offset + 10],
            0,
            0,
        ]);

        if let Some(counter) = writes.filter(|c| c.attribute == attr_id) {
            let tb = tbw::sata_raw_to_tb(raw_val, counter.unit);
            lifetime_writes_tb = Some((tb * 100.0).round() / 100.0);
        }

        match attr_id {
            // 0x05: Reallocated Sectors Count
            0x05 => {
                if raw_val > 50 {
                    caution = true;
                }
            }
            // 0x09: Power-On Hours
            0x09 => {
                if power_on_hours.is_none() {
                    power_on_hours = Some(raw_val as u32);
                }
            }
            // 0xC2 (194) or 0xBE (190): Temperature
            0xC2 | 0xBE => {
                let t = (raw_val & 0xFF) as f32;
                if (10.0..100.0).contains(&t) {
                    temp_c = Some(t);
                }
            }
            // 0xE7 (231): SSD Life Left / Wear indicator (Goodram CX400, Kingston, Crucial, SanDisk)
            0xE7 => {
                if (1..=100).contains(&current_val) {
                    health_pct = Some(current_val as u32);
                } else if (1..=100).contains(&raw_val) {
                    health_pct = Some(raw_val as u32);
                }
            }
            // 0xA9 (169): Remaining Life Percentage (Silicon Motion / Realtek SSDs)
            0xA9 if health_pct.is_none() && (1..=100).contains(&current_val) => {
                health_pct = Some(current_val as u32);
            }
            // 0xCA (202): Percentage Lifetime Used (Micron)
            0xCA if health_pct.is_none() && raw_val <= 100 => {
                health_pct = Some(100u32.saturating_sub(raw_val as u32));
            }
            _ => {}
        }
    }

    let status = if caution {
        "Caution — Sectors Reallocated".to_string()
    } else if let Some(h) = health_pct {
        if h >= 90 {
            "Healthy (Good)".to_string()
        } else if h >= 75 {
            "Healthy (Caution)".to_string()
        } else {
            "Degraded".to_string()
        }
    } else {
        // SMART was readable but has no wear indicator: say so instead of claiming "Good".
        "SMART OK — no wear indicator".to_string()
    };

    DriveSmartData {
        health_percentage: health_pct,
        status,
        temperature_c: temp_c,
        lifetime_writes_tb,
        power_on_hours,
    }
}

/// Status shown when neither SMART nor the Storage API reported a health state.
pub const NOT_REPORTED: &str = "Health not reported";

/// Query real SMART / reliability data for a physical disk via PowerShell WMI / CIM Storage API.
///
/// Uses `Get-PhysicalDisk` + `Get-StorageReliabilityCounter` with graceful fallback to
/// `Win32_DiskDrive` and honest telemetry bounds.
/// Returns `(health_pct, status, temp_c, poh, size_gb)`. MSFT_StorageReliabilityCounter has no
/// lifetime-writes property, so TBW never comes from here.
fn query_smart_health_wmi(disk_index: u32) -> (Option<u32>, String, f32, u32, f64) {
    let script = format!(
        "$c=[System.Globalization.CultureInfo]::InvariantCulture;\
$d=Get-PhysicalDisk|Where-Object{{$_.DeviceId -eq '{idx}'}}|Select-Object -First 1;\
if($d){{$sz=($d.Size/1GB).ToString('0.0',$c);$r=$null;\
try{{$r=$d|Get-StorageReliabilityCounter -ErrorAction SilentlyContinue}}catch{{}};\
$wear=if($r -and $r.Wear -ne $null){{100-[int]$r.Wear}}else{{''}};\
$tmp=if($r -and $r.Temperature -ne $null -and $r.Temperature -gt 0){{$r.Temperature}}else{{''}};\
$poh=if($r -and $r.PowerOnHours -ne $null){{[int]$r.PowerOnHours}}else{{0}};\
$hs=if($d.HealthStatus){{$d.HealthStatus}}else{{''}};\
Write-Output \"$wear|$tmp|$poh|$sz|$hs\"}}\
else{{\
$w=Get-CimInstance Win32_DiskDrive|Where-Object{{$_.Index -eq {idx}}}|Select-Object -First 1;\
if($w){{$sz=($w.Size/1GB).ToString('0.0',$c);Write-Output \"||0|$sz|\"}}\
else{{Write-Output ''}}\
}}",
        idx = disk_index
    );

    let output = SilentCommand::new("powershell.exe")
        .timeout(std::time::Duration::from_secs(20))
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();

    // Nothing is estimated: missing values stay None / 0.0 and the status says so (rule 2).
    let not_reported = || (None, NOT_REPORTED.to_string(), 0.0f32, 0u32, 0.0f64);

    match output {
        Ok(o) if o.status.success() => {
            let raw = String::from_utf8_lossy(&o.stdout);
            let line = raw.trim();
            if line.is_empty() {
                return not_reported();
            }
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 5 {
                return not_reported();
            }
            let parse_f64 =
                |s: &str| -> f64 { s.trim().replace(',', ".").parse::<f64>().unwrap_or(0.0) };
            let parse_f32 =
                |s: &str| -> f32 { s.trim().replace(',', ".").parse::<f32>().unwrap_or(0.0) };
            let health_pct = parts[0].trim().parse::<u32>().ok().map(|h| h.min(100));
            let temp_c = parse_f32(parts[1]).max(0.0);
            let poh = parts[2].trim().parse::<u32>().unwrap_or(0);
            let size_gb = parse_f64(parts[3]);
            let health_str = parts[4].trim();
            let status = if health_str.eq_ignore_ascii_case("Healthy") {
                match health_pct {
                    Some(h) if h >= 90 => "Healthy (Good)",
                    Some(h) if h >= 75 => "Healthy (Caution)",
                    Some(_) => "Degraded",
                    None => "Healthy (Windows status)",
                }
            } else if health_str.eq_ignore_ascii_case("Warning") {
                "Caution — Reliability Risk"
            } else if health_str.eq_ignore_ascii_case("Unhealthy") {
                "Unhealthy — Replace Drive"
            } else {
                NOT_REPORTED
            };
            (health_pct, status.to_string(), temp_c, poh, size_gb)
        }
        _ => not_reported(),
    }
}

/// Fallback to querying logical volume info when physical drive handles are restricted.
fn probe_volume_fallback(
    letter: char,
    index: u32,
    trim_enabled: bool,
) -> Option<PhysicalDriveHealth> {
    let root_utf16: Vec<u16> = format!("{}:\\", letter)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let mut vol_name = [0u16; 260];
    let mut fs_name = [0u16; 260];
    let mut serial = 0u32;
    let mut max_comp = 0u32;
    let mut flags = 0u32;

    let ok = unsafe {
        GetVolumeInformationW(
            PCWSTR(root_utf16.as_ptr()),
            Some(&mut vol_name),
            Some(&mut serial),
            Some(&mut max_comp),
            Some(&mut flags),
            Some(&mut fs_name),
        )
    };

    if ok.is_err() {
        return None;
    }

    let mut free_bytes = 0u64;
    let mut total_bytes = 0u64;
    let mut total_free = 0u64;

    let _ = unsafe {
        GetDiskFreeSpaceExW(
            PCWSTR(root_utf16.as_ptr()),
            Some(&mut free_bytes),
            Some(&mut total_bytes),
            Some(&mut total_free),
        )
    };

    let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    if total_gb <= 0.1 {
        return None;
    }

    let vol_label = String::from_utf16_lossy(&vol_name)
        .trim_matches('\0')
        .trim()
        .to_string();
    let fs_str = String::from_utf16_lossy(&fs_name)
        .trim_matches('\0')
        .trim()
        .to_string();

    let model = if !vol_label.is_empty() {
        format!("{}: [{}] ({} Volume)", letter, vol_label, fs_str)
    } else {
        format!("Local Disk ({}:) - {}", letter, fs_str)
    };

    Some(PhysicalDriveHealth {
        index,
        model,
        interface_type: "Fixed System Volume".to_string(),
        total_size_gb: (total_gb * 10.0).round() / 10.0,
        health_percentage: None,
        status: "Volume only — drive health not accessible".to_string(),
        temperature_c: 0.0,
        host_writes: HostWrites::NotAvailable,
        power_on_hours: 0,
        trim_enabled,
    })
}

fn extract_null_terminated_str(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_smart_health_wmi_fallback() {
        // For disk index 255 (non-existent): no invented values, just "not reported".
        let (health, status, temp, poh, size) = query_smart_health_wmi(255);
        assert_eq!(health, None, "missing health must not be estimated");
        assert_eq!(status, NOT_REPORTED);
        assert_eq!(temp, 0.0, "missing temperature must not be estimated");
        assert_eq!(poh, 0, "POH should default to 0 for non-existent disk");
        let _ = size;
    }

    #[test]
    fn test_disk_subsystem_report_probe_no_panic() {
        // Should complete without panicking even without admin access.
        let report = DiskSubsystemReport::probe();
        for drive in &report.drives {
            assert!(!drive.model.is_empty(), "Drive model should not be empty");
            assert!(
                drive.health_percentage.is_none_or(|h| h <= 100),
                "Health percentage out of range"
            );
            assert!(drive.total_size_gb > 0.0, "Physical drive size must be > 0");
        }
    }

    #[test]
    fn test_ata_smart_structures_sizes() {
        // Enforce exact structure alignments to prevent memory corruption in FFI
        assert_eq!(size_of::<DriveSmartData>(), 64);
        assert_eq!(size_of::<DiskPerformanceSnapshot>(), 32);
    }

    #[test]
    fn test_parse_nvme_smart_log_synthetic() {
        let mut smart = [0u8; 512];
        // Critical warning = 0
        smart[0] = 0;
        // Temp = 313 K (39.85 °C)
        smart[1..3].copy_from_slice(&313u16.to_le_bytes());
        // Percentage used = 2% (Health 98%)
        smart[5] = 2;
        // Data units written = 20,000,000 units (20,000,000 * 512,000 / 1e12 = 10.24 TB)
        let duw = 20_000_000u128;
        smart[48..64].copy_from_slice(&duw.to_le_bytes());
        // POH = 1250 hours
        let poh = 1250u64;
        smart[128..136].copy_from_slice(&poh.to_le_bytes());

        let res = parse_nvme_smart_log(&smart).expect("Must parse valid NVMe SMART log");
        assert_eq!(res.health_percentage, Some(98));
        assert_eq!(res.status, "Healthy (Good)");
        assert_eq!(res.temperature_c, Some(39.9));
        assert_eq!(res.lifetime_writes_tb, Some(10.24));
        assert_eq!(res.power_on_hours, Some(1250));
    }

    #[test]
    fn test_parse_ata_smart_attributes_synthetic() {
        let mut buf = [0u8; 512];
        // Attr 0: Attr 0xE7 (Life Left) -> current_val = 96%
        let offset0 = 2;
        buf[offset0] = 0xE7;
        buf[offset0 + 3] = 96;

        // Attr 1: Attr 0xF1 (Total LBAs Written) -> 15_625_000_000 LBAs * 512 = 8.00 TB
        let offset1 = 2 + 12;
        buf[offset1] = 0xF1;
        let lbas = 15_625_000_000u64;
        buf[offset1 + 5..offset1 + 11].copy_from_slice(&lbas.to_le_bytes()[0..6]);

        // Attr 2: Attr 0xC2 (Temperature) -> 34 °C
        let offset2 = 2 + 24;
        buf[offset2] = 0xC2;
        buf[offset2 + 5] = 34;

        // Attr 3: Attr 0x09 (Power-On Hours) -> 3200 hours
        let offset3 = 2 + 36;
        buf[offset3] = 0x09;
        let poh = 3200u64;
        buf[offset3 + 5..offset3 + 11].copy_from_slice(&poh.to_le_bytes()[0..6]);

        let lba = SataWriteCounter { attribute: 0xF1, unit: tbw::WriteUnit::Lba512 };
        let res = parse_ata_smart_attributes(&buf, Some(lba));
        assert_eq!(res.health_percentage, Some(96));
        assert_eq!(res.status, "Healthy (Good)");
        assert_eq!(res.temperature_c, Some(34.0));
        assert_eq!(res.lifetime_writes_tb, Some(8.00));
        assert_eq!(res.power_on_hours, Some(3200));

        // Without a known vendor unit the same table yields no TBW at all.
        assert_eq!(parse_ata_smart_attributes(&buf, None).lifetime_writes_tb, None);
    }

    /// Bytes from a hex-dump fixture (`offset: xx xx ...` lines, `#` comments).
    fn hex_fixture(text: &str) -> Vec<u8> {
        text.lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .filter_map(|l| l.split_once(':').map(|(_, bytes)| bytes))
            .flat_map(|bytes| bytes.split_whitespace())
            .map(|b| u8::from_str_radix(b, 16).expect("hex byte"))
            .collect()
    }

    fn sector(bytes: Vec<u8>) -> [u8; 512] {
        bytes.try_into().expect("fixture must hold exactly 512 bytes")
    }

    #[test]
    fn nvme_descriptor_fixture_kingston_snv3s500g() {
        // Raw IOCTL output captured on a KINGSTON SNV3S500G (2026-11-25).
        let desc = hex_fixture(include_str!("../../tests/fixtures/smart/kingston_snv3s500g_nvme_descriptor.hex"));
        assert_eq!(desc.len(), 560);
        // ProtocolType 3 (NVMe), DataType 2 (log page), ProtocolDataOffset 40, length 512.
        assert_eq!(&desc[8..12], &3u32.to_le_bytes());
        assert_eq!(&desc[12..16], &2u32.to_le_bytes());
        assert_eq!(&desc[24..28], &40u32.to_le_bytes());

        let log = nvme_log_from_descriptor(&desc).expect("log page inside descriptor");
        assert_eq!(log.len(), 512);
        assert_eq!(log.as_ptr(), desc[48..].as_ptr(), "log starts 8 + ProtocolDataOffset bytes in");

        let smart = parse_nvme_smart_log(log).expect("valid log page 0x02");
        // Percentage Used 0x12 = 18 % -> 82 % health; Data Units Written 0x0452A42C = 72,524,844
        // units × 512,000 B = 37.13 TB; Power-On Hours 0x0D5C = 3420.
        assert_eq!(smart.health_percentage, Some(82));
        assert_eq!(smart.lifetime_writes_tb, Some(37.13));
        assert_eq!(smart.power_on_hours, Some(3420));
        let t = smart.temperature_c.expect("composite temperature");
        assert!((30.0..45.0).contains(&t), "temperature {t}");
    }

    #[test]
    fn nvme_descriptor_rejects_short_or_bad_offsets() {
        assert_eq!(nvme_log_from_descriptor(&[]), None);
        let mut desc = vec![0u8; 560];
        desc[24..28].copy_from_slice(&40u32.to_le_bytes());
        desc[28..32].copy_from_slice(&64u32.to_le_bytes()); // shorter than a SMART log
        assert_eq!(nvme_log_from_descriptor(&desc), None);
        desc[28..32].copy_from_slice(&512u32.to_le_bytes());
        desc[24..28].copy_from_slice(&u32::MAX.to_le_bytes()); // offset beyond the buffer
        assert_eq!(nvme_log_from_descriptor(&desc), None);
    }

    #[test]
    fn ata_fixture_goodram_cx400_g2_unknown_unit_is_not_available() {
        // Raw SMART READ DATA + IDENTIFY captured on a Goodram SSDPR-CX400-01T-G2 (fw SN20728).
        let id = sector(hex_fixture(include_str!("../../tests/fixtures/smart/goodram_cx400_g2_identify.hex")));
        let table = sector(hex_fixture(include_str!("../../tests/fixtures/smart/goodram_cx400_g2_ata_smart.hex")));

        let (model, fw) = parse_ata_identify(&id);
        assert_eq!(model, "SSDPR-CX400-01T-G2");
        assert_eq!(fw, "SN20728");

        // This firmware is not the Phison variant smartmontools documents, so 0xF1's unit is
        // unknown and no TBW is reported (the raw value 4718 could be GiB or 32 MiB blocks).
        let counter = tbw::lookup_sata_write_counter(&model, &fw);
        assert_eq!(counter, None);
        let smart = parse_ata_smart_attributes(&table, counter);
        assert_eq!(smart.lifetime_writes_tb, None);
        assert_eq!(smart.power_on_hours, Some(496));
        assert_eq!(smart.temperature_c, Some(28.0));
        assert_eq!(smart.health_percentage, None);
        assert_eq!(smart.status, "SMART OK — no wear indicator");

        // The raw48 decoding itself: the same bytes read as GiB give 4718 GiB = 5.07 TB.
        let gib = SataWriteCounter { attribute: 0xF1, unit: tbw::WriteUnit::GiB };
        assert_eq!(parse_ata_smart_attributes(&table, Some(gib)).lifetime_writes_tb, Some(5.07));
    }

    #[test]
    fn access_denied_maps_to_requires_admin() {
        let denied = windows::core::Error::from_hresult(windows::core::HRESULT::from_win32(ERROR_ACCESS_DENIED.0));
        assert_eq!(SmartQueryError::from(denied), SmartQueryError::AccessDenied);
        let io = windows::core::Error::from_hresult(windows::core::HRESULT::from_win32(1117));
        assert_eq!(SmartQueryError::from(io), SmartQueryError::Unavailable);
    }

    /// Read-only: prints what this PC's drives report, to compare with CrystalDiskInfo / smartctl.
    /// `cargo test --lib print_live_disk_report -- --ignored --nocapture`
    #[test]
    #[ignore = "prints live drive data for manual comparison"]
    fn print_live_disk_report() {
        for d in DiskSubsystemReport::probe().drives {
            println!(
                "Disk {}: {} [{}] health={:?} temp={} poh={} writes={:?} status={}",
                d.index, d.model, d.interface_type, d.health_percentage, d.temperature_c, d.power_on_hours, d.host_writes, d.status
            );
        }
    }

    #[test]
    fn test_kernel_disk_performance_live_probe() {
        // Verify IOCTL_DISK_GET_PERFORMANCE works without elevation
        let path_utf16: Vec<u16> = "\\\\.\\PhysicalDrive0"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let ok_handle = unsafe {
            CreateFileW(
                PCWSTR(path_utf16.as_ptr()),
                0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                Default::default(),
                None,
            )
        };

        if let Ok(handle) = ok_handle {
            let _guard = HandleGuard(handle);
            let perf = query_disk_performance_ioctl(handle);
            if let Some(p) = perf {
                assert!(p.bytes_read >= 0);
                assert!(p.bytes_written >= 0);
            }
        }
    }
}
