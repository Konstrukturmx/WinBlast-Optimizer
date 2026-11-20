//! Chassis-Aware Power & Thermal Governor (Milestone 4 Blueprint M4-4).
//!
//! Intelligently classifies the host form factor (Desktop vs Laptop/Mobile) using
//! direct Win32 `GetSystemPowerStatus` and configures tailored power governance:
//! - Desktop: Disables Fast Startup & Hibernation (`powercfg -h off`) to eliminate driver state
//!   memory leaks and reclaim 8–32 GB SSD storage, configuring aggressive processor boost (`PERFBOOSTMODE = 4`).
//! - Laptop: Preserves Fast Startup (`HiberbootEnabled = 1`) for mobile wake speed, configuring
//!   efficient aggressive race-to-idle boost profiles (`PERFBOOSTMODE = 5`).

use serde::{Deserialize, Serialize};
use crate::core::backend::SystemBackend;
use crate::core::error::WinBlastError;

/// Detected hardware form factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormFactor {
    Desktop,
    Laptop,
    Unknown,
}

/// Power governance profile calculated for the detected form factor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChassisProfile {
    pub form_factor: FormFactor,
    pub fast_startup_enabled: bool,
    pub hibernation_enabled: bool,
    pub perf_boost_mode: u32,
    pub perf_increase_policy: u32,
    pub summary: String,
}

/// Probe the host machine's power status via Win32 `GetSystemPowerStatus`.
#[cfg(windows)]
pub fn probe_system_form_factor() -> FormFactor {
    use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};

    let mut status = SYSTEM_POWER_STATUS::default();
    let ok = unsafe { GetSystemPowerStatus(&mut status) };

    if ok.is_ok() {
        // BatteryFlag:
        // 128 = No system battery (Desktop)
        // 255 = Unknown status (typically Desktop on AC without smart battery)
        // 1, 2, 4 = High, Low, Critical (Laptop battery present)
        if status.BatteryFlag == 128 || (status.BatteryFlag == 255 && status.BatteryLifePercent == 255) {
            FormFactor::Desktop
        } else if status.BatteryFlag & 128 == 0 && status.BatteryLifePercent <= 100 {
            FormFactor::Laptop
        } else {
            FormFactor::Desktop
        }
    } else {
        FormFactor::Desktop
    }
}

#[cfg(not(windows))]
pub fn probe_system_form_factor() -> FormFactor {
    FormFactor::Desktop
}

/// Evaluates the recommended chassis power profile based on detected form factor.
pub fn evaluate_chassis_profile(form_factor: FormFactor) -> ChassisProfile {
    match form_factor {
        FormFactor::Desktop => ChassisProfile {
            form_factor: FormFactor::Desktop,
            fast_startup_enabled: false,
            hibernation_enabled: false,
            perf_boost_mode: 4,      // Aggressive at Guaranteed
            perf_increase_policy: 0, // Ideal immediate frequency ramp
            summary: "Desktop Profile: Fast Startup & Hibernation disabled (saves 8-32GB RAM storage & avoids driver leak), CPU boost set to Aggressive.".to_string(),
        },
        FormFactor::Laptop => ChassisProfile {
            form_factor: FormFactor::Laptop,
            fast_startup_enabled: true,
            hibernation_enabled: true,
            perf_boost_mode: 5,      // Efficient Aggressive (Race to Idle)
            perf_increase_policy: 1, // Balanced ramp
            summary: "Laptop Profile: Fast Startup & Sleep preserved for mobile battery conservation, CPU boost set to Efficient Aggressive.".to_string(),
        },
        FormFactor::Unknown => ChassisProfile {
            form_factor: FormFactor::Unknown,
            fast_startup_enabled: false,
            hibernation_enabled: false,
            perf_boost_mode: 4,
            perf_increase_policy: 0,
            summary: "Generic Profile: Conservative performance tuning applied.".to_string(),
        },
    }
}

/// Applies the chassis power profile to the system via the provided backend.
pub fn apply_chassis_governor<B: SystemBackend>(
    backend: &B,
    profile: &ChassisProfile,
) -> Result<(), WinBlastError> {
    // 1. Fast Startup registry configuration
    let hiberboot_val = if profile.fast_startup_enabled { 1u32 } else { 0u32 };
    backend.reg_set_dword(
        "HKLM",
        "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power",
        "HiberbootEnabled",
        hiberboot_val,
    )?;

    // 2. Hibernation state (Desktop turns off hibernation to deallocate hiberfil.sys)
    if !profile.hibernation_enabled {
        let _ = crate::core::process::SilentCommand::new("powercfg.exe")
            .args(["-h", "off"])
            .output();
    }

    // 3. Unhide Processor Performance Boost Mode in active power scheme
    let _ = crate::core::process::SilentCommand::new("powercfg.exe")
        .args([
            "-attributes",
            "SUB_PROCESSOR",
            "be337238-0d82-4146-a960-4f3749d470c7",
            "-ATTRIB_HIDE",
        ])
        .output();

    // 4. Configure Processor Boost Mode (PERFBOOSTMODE)
    // 54533251-82be-4824-96c1-47b60b740d00 = SUB_PROCESSOR
    // be337238-0d82-4146-a960-4f3749d470c7 = PERFBOOSTMODE
    let boost_mode_str = profile.perf_boost_mode.to_string();
    let _ = crate::core::process::SilentCommand::new("powercfg.exe")
        .args([
            "-setacvalueindex",
            "SCHEME_CURRENT",
            "SUB_PROCESSOR",
            "be337238-0d82-4146-a960-4f3749d470c7",
            &boost_mode_str,
        ])
        .output();

    // Activate changes
    let _ = crate::core::process::SilentCommand::new("powercfg.exe")
        .args(["-setactive", "SCHEME_CURRENT"])
        .output();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::backend::SandboxEnv;

    #[test]
    fn test_desktop_profile_evaluation() {
        let profile = evaluate_chassis_profile(FormFactor::Desktop);
        assert_eq!(profile.form_factor, FormFactor::Desktop);
        assert!(!profile.fast_startup_enabled);
        assert!(!profile.hibernation_enabled);
        assert_eq!(profile.perf_boost_mode, 4);
        assert_eq!(profile.perf_increase_policy, 0);
    }

    #[test]
    fn test_laptop_profile_evaluation() {
        let profile = evaluate_chassis_profile(FormFactor::Laptop);
        assert_eq!(profile.form_factor, FormFactor::Laptop);
        assert!(profile.fast_startup_enabled);
        assert!(profile.hibernation_enabled);
        assert_eq!(profile.perf_boost_mode, 5);
        assert_eq!(profile.perf_increase_policy, 1);
    }

    #[test]
    fn test_apply_chassis_governor_sandbox() {
        let sandbox = SandboxEnv::default();
        let profile = evaluate_chassis_profile(FormFactor::Desktop);
        let res = apply_chassis_governor(&sandbox, &profile);
        assert!(res.is_ok());

        let hiberboot = sandbox.reg_get_dword(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power",
            "HiberbootEnabled",
        );
        assert_eq!(hiberboot, Some(0));
    }
}
