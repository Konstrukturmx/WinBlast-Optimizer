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
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    let raw_str = current_exe.to_string_lossy();
    let clean_str = crate::core::updater::clean_exe_path(&raw_str);

    // Native "runas" verb: no PowerShell console flash, no command-line quoting of the path.
    let file_w: Vec<u16> = clean_str.encode_utf16().chain(std::iter::once(0)).collect();
    let verb_w: Vec<u16> = "runas".encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: both buffers are NUL-terminated UTF-16 strings that outlive the call.
    let hinstance = unsafe {
        windows::Win32::UI::Shell::ShellExecuteW(
            None,
            windows::core::PCWSTR(verb_w.as_ptr()),
            windows::core::PCWSTR(file_w.as_ptr()),
            None,
            None,
            windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
        )
    };
    // ShellExecuteW returns a value greater than 32 on success; a declined UAC prompt
    // (ERROR_CANCELLED) or a missing file comes back as a small error code.
    let code = hinstance.0 as usize;
    if code > 32 {
        std::process::exit(0);
    }
    Err(format!("UAC elevation request was canceled or failed (ShellExecute code {code})"))
}
