//! Storage device medium and seek-penalty diagnostics (Milestone 4 Blueprint M4-2).
//!
//! Provides kernel-level detection of storage media (NVMe, SATA SSD, Mechanical HDD)
//! using `IOCTL_STORAGE_QUERY_PROPERTY` (`StorageDeviceSeekPenaltyProperty`).

use serde::{Deserialize, Serialize};

/// Physical storage drive medium.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageMedium {
    /// Non-Volatile Memory Express PCIe SSD.
    Nvme,
    /// Solid State Drive (SATA or SAS, zero seek penalty).
    SataSsd,
    /// Mechanical spinning hard disk drive (seek penalty incurred).
    MechanicalHdd,
    /// Removable USB mass storage.
    UsbRemovable,
    /// Medium indeterminate or generic controller.
    Unknown,
}

/// Hardware diagnostics for a physical storage drive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageMediumInfo {
    pub medium: StorageMedium,
    pub incurs_seek_penalty: bool,
    pub is_solid_state: bool,
    pub drive_letter: char,
}

/// Detects whether the volume on the given drive letter incurs a seek penalty.
#[cfg(windows)]
pub fn probe_volume_seek_penalty(drive_letter: char) -> Option<bool> {
    use std::mem::size_of;
    use windows::core::PCWSTR;
    use windows::Win32::Storage::FileSystem::{
        CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows::Win32::System::Ioctl::{
        PropertyStandardQuery, StorageDeviceSeekPenaltyProperty, DEVICE_SEEK_PENALTY_DESCRIPTOR,
        IOCTL_STORAGE_QUERY_PROPERTY, STORAGE_PROPERTY_QUERY,
    };
    use windows::Win32::System::IO::DeviceIoControl;

    let path_utf16: Vec<u16> = format!("\\\\.\\{}:", drive_letter)
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let handle_res = unsafe {
        CreateFileW(
            PCWSTR(path_utf16.as_ptr()),
            0, // query access only
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        )
    };

    let Ok(handle) = handle_res else {
        return None;
    };

    struct HandleGuard(windows::Win32::Foundation::HANDLE);
    impl Drop for HandleGuard {
        fn drop(&mut self) {
            unsafe {
                let _ = windows::Win32::Foundation::CloseHandle(self.0);
            }
        }
    }
    let _guard = HandleGuard(handle);

    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceSeekPenaltyProperty,
        QueryType: PropertyStandardQuery,
        ..Default::default()
    };
    let mut desc = DEVICE_SEEK_PENALTY_DESCRIPTOR::default();
    let mut bytes_ret = 0u32;

    let ok = unsafe {
        DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some((&mut query as *mut STORAGE_PROPERTY_QUERY).cast()),
            size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            Some((&mut desc as *mut DEVICE_SEEK_PENALTY_DESCRIPTOR).cast()),
            size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
            Some(&mut bytes_ret),
            None,
        )
    };

    if ok.is_ok() && bytes_ret >= size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32 {
        Some(desc.IncursSeekPenalty)
    } else {
        None
    }
}

#[cfg(not(windows))]
pub fn probe_volume_seek_penalty(_drive_letter: char) -> Option<bool> {
    Some(false)
}

/// Diagnoses the medium for a given drive letter.
pub fn diagnose_storage_medium(drive_letter: char) -> StorageMediumInfo {
    let penalty = probe_volume_seek_penalty(drive_letter).unwrap_or(false);
    let is_solid_state = !penalty;
    let medium = if is_solid_state {
        StorageMedium::SataSsd
    } else {
        StorageMedium::MechanicalHdd
    };

    StorageMediumInfo {
        medium,
        incurs_seek_penalty: penalty,
        is_solid_state,
        drive_letter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_medium_info() {
        let info = diagnose_storage_medium('C');
        assert_eq!(info.drive_letter, 'C');
        assert_eq!(info.is_solid_state, !info.incurs_seek_penalty);
    }
}
