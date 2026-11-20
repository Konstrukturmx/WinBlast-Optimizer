//! Hardware-Adaptive Storage & Memory Tuning (Milestone 4 Blueprint M4-2).
//!
//! Eliminates disk I/O contention and frametime microstutters by:
//! 1. Disabling `SysMain` (Superfetch) and prelaunch memory agents (`Disable-MMAgent -ApplicationPreLaunch`)
//!    on SSDs when physical RAM >= 16 GB, preventing background SSD write amplification.
//! 2. Setting a fixed static pagefile (InitialSize == MaximumSize) on the primary drive,
//!    completely eliminating dynamic NTFS pagefile expansion hitches during gaming.

use serde::{Deserialize, Serialize};
use crate::core::backend::SystemBackend;
use crate::core::error::WinBlastError;
use crate::hardware::storage::StorageMediumInfo;

/// Physical RAM statistics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_ram_gb: f64,
    pub total_ram_mb: u64,
}

/// Query total physical RAM via Win32 `GlobalMemoryStatusEx`.
#[cfg(windows)]
pub fn query_physical_ram() -> MemoryStats {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    use std::mem::size_of;

    let mut mem = MEMORYSTATUSEX {
        dwLength: size_of::<MEMORYSTATUSEX>() as u32,
        ..Default::default()
    };

    let ok = unsafe { GlobalMemoryStatusEx(&mut mem) };
    if ok.is_ok() && mem.ullTotalPhys > 0 {
        let mb = mem.ullTotalPhys / (1024 * 1024);
        let gb = (mem.ullTotalPhys as f64) / (1024.0 * 1024.0 * 1024.0);
        MemoryStats {
            total_ram_gb: (gb * 10.0).round() / 10.0,
            total_ram_mb: mb,
        }
    } else {
        MemoryStats {
            total_ram_gb: 16.0,
            total_ram_mb: 16384,
        }
    }
}

#[cfg(not(windows))]
pub fn query_physical_ram() -> MemoryStats {
    MemoryStats {
        total_ram_gb: 16.0,
        total_ram_mb: 16384,
    }
}

/// Recommended adaptive tuning parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveTuningConfig {
    pub sysmain_disable_recommended: bool,
    pub disable_mmagent_prelaunch: bool,
    pub recommended_static_pagefile_mb: u32,
    pub primary_drive_is_ssd: bool,
    pub summary: String,
}

/// Evaluates optimal adaptive storage and memory tuning parameters.
pub fn evaluate_adaptive_tuning(storage: &StorageMediumInfo, memory: &MemoryStats) -> AdaptiveTuningConfig {
    // Rule 1: SysMain & MMAgent
    // If Solid State Drive AND RAM >= 16 GB -> Disable SysMain and MMAgent prelaunch to eliminate write amplification.
    // If HDD or RAM < 16 GB -> Keep SysMain in delayed-auto mode.
    let sysmain_disable = storage.is_solid_state && memory.total_ram_gb >= 15.5;

    // Rule 2: Static Pagefile Sizing (Initial == Maximum)
    // Prevents NTFS thread stalls from dynamic paging file growth.
    let pagefile_mb = if memory.total_ram_gb >= 31.5 {
        8192 // 8 GB fixed pagefile
    } else if memory.total_ram_gb >= 15.5 {
        12288 // 12 GB fixed pagefile
    } else {
        16384 // 16 GB fixed pagefile
    };

    let summary = format!(
        "Adaptive Tuning: SysMain Disable = {}, Static Pagefile = {} MB on {}: (RAM: {:.1} GB, SSD: {})",
        sysmain_disable, pagefile_mb, storage.drive_letter, memory.total_ram_gb, storage.is_solid_state
    );

    AdaptiveTuningConfig {
        sysmain_disable_recommended: sysmain_disable,
        disable_mmagent_prelaunch: sysmain_disable,
        recommended_static_pagefile_mb: pagefile_mb,
        primary_drive_is_ssd: storage.is_solid_state,
        summary,
    }
}

/// Applies adaptive hardware tuning via the provided backend.
pub fn apply_adaptive_tuning<B: SystemBackend>(
    backend: &B,
    config: &AdaptiveTuningConfig,
    drive_letter: char,
) -> Result<(), WinBlastError> {
    // 1. SysMain Service configuration
    if config.sysmain_disable_recommended {
        // Set SysMain start to Disabled (4)
        let _ = backend.set_service_start_type("SysMain", 4);
        backend.reg_set_dword(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services\\SysMain",
            "Start",
            4,
        )?;

        // Run PowerShell Disable-MMAgent -ApplicationPreLaunch
        let _ = crate::core::process::SilentCommand::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", "Disable-MMAgent -ApplicationPreLaunch -ErrorAction SilentlyContinue"])
            .output();
    } else {
        // Delayed auto-start
        let _ = backend.set_service_start_type("SysMain", 2);
        backend.reg_set_dword(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services\\SysMain",
            "Start",
            2,
        )?;
        backend.reg_set_dword(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services\\SysMain",
            "DelayedAutoStart",
            1,
        )?;
    }

    // 2. Configure Static Anti-Stutter Pagefile (Initial == Maximum)
    // Multi-SZ PagingFiles entry in HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management
    let pagefile_spec = format!("{}:\\pagefile.sys {} {}", drive_letter, config.recommended_static_pagefile_mb, config.recommended_static_pagefile_mb);
    backend.reg_set_string(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "PagingFiles",
        &pagefile_spec,
        false,
        None,
    )?;

    // Disable automatic paging file sizing (AutomaticManagedPagefile = 0)
    backend.reg_set_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
        "AutomaticManagedPagefile",
        0,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::backend::SandboxEnv;
    use crate::hardware::storage::StorageMedium;

    #[test]
    fn test_evaluate_adaptive_tuning_high_spec_nvme() {
        let storage = StorageMediumInfo {
            medium: StorageMedium::Nvme,
            incurs_seek_penalty: false,
            is_solid_state: true,
            drive_letter: 'C',
        };
        let memory = MemoryStats {
            total_ram_gb: 32.0,
            total_ram_mb: 32768,
        };

        let config = evaluate_adaptive_tuning(&storage, &memory);
        assert!(config.sysmain_disable_recommended);
        assert!(config.disable_mmagent_prelaunch);
        assert_eq!(config.recommended_static_pagefile_mb, 8192);
    }

    #[test]
    fn test_evaluate_adaptive_tuning_low_spec_hdd() {
        let storage = StorageMediumInfo {
            medium: StorageMedium::MechanicalHdd,
            incurs_seek_penalty: true,
            is_solid_state: false,
            drive_letter: 'C',
        };
        let memory = MemoryStats {
            total_ram_gb: 8.0,
            total_ram_mb: 8192,
        };

        let config = evaluate_adaptive_tuning(&storage, &memory);
        assert!(!config.sysmain_disable_recommended);
        assert_eq!(config.recommended_static_pagefile_mb, 16384);
    }

    #[test]
    fn test_apply_adaptive_tuning_sandbox() {
        let sandbox = SandboxEnv::default();
        let storage = StorageMediumInfo {
            medium: StorageMedium::Nvme,
            incurs_seek_penalty: false,
            is_solid_state: true,
            drive_letter: 'C',
        };
        let memory = MemoryStats {
            total_ram_gb: 16.0,
            total_ram_mb: 16384,
        };
        let config = evaluate_adaptive_tuning(&storage, &memory);

        let res = apply_adaptive_tuning(&sandbox, &config, 'C');
        assert!(res.is_ok());

        let pfiles = sandbox.reg_get_string(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management",
            "PagingFiles",
            None,
        ).map(|s| s.value);
        assert_eq!(pfiles, Some("C:\\pagefile.sys 12288 12288".to_string()));
    }
}
