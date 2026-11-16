//! Token privilege verification for Administrator / UAC elevation.
//!
//! Uses Win32 `OpenProcessToken` + `GetTokenInformation` to inspect the
//! `TOKEN_ELEVATION` flag of the current process.

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Check if the current process is running with elevated (Administrator) privileges.
pub fn is_elevated() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();
        let process = GetCurrentProcess();
        if OpenProcessToken(process, TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut return_length = 0u32;
        let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut return_length,
        );

        let _ = CloseHandle(token_handle);
        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

/// Returns an error if the process is not elevated.
///
/// Call this early in startup to fail fast with a clear message.
#[allow(dead_code)]
pub fn check_elevation() -> crate::core::error::Result<()> {
    if !is_elevated() {
        return Err(crate::core::error::WinBlastError::ElevationRequired(
            "WinBlast0 must be run as Administrator to apply system tweaks.".to_string(),
        ));
    }
    Ok(())
}

/// Relaunches the current WinBlast0 application with elevated (Administrator) UAC privileges.
///
/// Triggers Windows User Account Control (UAC) elevation prompt and cleanly exits current standard instance.
pub fn relaunch_as_admin() -> Result<(), String> {
    let current_exe = std::env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))?;
    let exe_str = current_exe.to_str().ok_or_else(|| "Invalid UTF-8 executable path".to_string())?;

    let cmd = format!("Start-Process -FilePath '{}' -Verb RunAs", exe_str);
    let status = std::process::Command::new("powershell")
        .args(["-WindowStyle", "Hidden", "-Command", &cmd])
        .status()
        .map_err(|e| format!("Failed to invoke PowerShell RunAs: {}", e))?;

    if status.success() {
        std::process::exit(0);
    } else {
        Err("UAC elevation request was canceled or rejected by the user".to_string())
    }
}
