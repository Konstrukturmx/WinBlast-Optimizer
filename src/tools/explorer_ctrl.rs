//! Safe Windows shell refresh, non-elevated Explorer restart, and reboot helpers.

use core::ffi::c_void;

use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, LPARAM, WPARAM};
use windows::Win32::Security::{
    DuplicateTokenEx, SecurityImpersonation, TokenPrimary, TOKEN_ADJUST_DEFAULT,
    TOKEN_ADJUST_SESSIONID, TOKEN_ASSIGN_PRIMARY, TOKEN_DUPLICATE, TOKEN_QUERY,
};
use windows::Win32::System::Environment::{CreateEnvironmentBlock, DestroyEnvironmentBlock};
use windows::Win32::System::Threading::{
    CreateProcessWithTokenW, OpenProcess, OpenProcessToken, TerminateProcess, WaitForSingleObject,
    CREATE_UNICODE_ENVIRONMENT, LOGON_WITH_PROFILE, PROCESS_INFORMATION,
    PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE, STARTUPINFOW,
};
use windows::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNF_FLUSH};
use windows::Win32::UI::WindowsAndMessaging::{
    GetShellWindow, GetWindowThreadProcessId, SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG,
    WM_SETTINGCHANGE,
};

use crate::core::elevation;
use crate::core::error::{Result, WinBlastError};
use crate::core::process::SilentCommand;
use crate::scanner::task_scanner::decode_oem_bytes;

struct HandleCloser(HANDLE);

impl Drop for HandleCloser {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

struct EnvBlockGuard(*mut c_void);

impl Drop for EnvBlockGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = DestroyEnvironmentBlock(self.0 as *const c_void);
            }
        }
    }
}

/// Broadcast `WM_SETTINGCHANGE` and `SHCNE_ASSOCCHANGED` so live Explorer, Taskbar, Dark Mode,
/// File Extensions, and Desktop icon settings refresh without killing user Explorer windows.
pub fn broadcast_shell_settings_changed() {
    let policy_w: Vec<u16> = "TraySettings\0".encode_utf16().collect();
    let env_w: Vec<u16> = "ImmersiveColorSet\0".encode_utf16().collect();
    unsafe {
        let mut _res = 0usize;
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(policy_w.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            300,
            Some(&mut _res),
        );
        let _ = SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            WPARAM(0),
            LPARAM(env_w.as_ptr() as isize),
            SMTO_ABORTIFHUNG,
            300,
            Some(&mut _res),
        );
        SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_FLUSH, None, None);
    }
}

/// Restart Windows Explorer safely without privilege escalation.
///
/// Verifies elevation, captures the non-elevated desktop shell token (`GetShellWindow`) and
/// user environment block BEFORE terminating `explorer.exe`, then restarts `%WINDIR%\explorer.exe`
/// under that standard medium-integrity token via `CreateProcessWithTokenW` so the desktop shell
/// does NOT inherit Admin privileges and the user is never stranded without a shell.
pub fn restart_explorer_safely() -> Result<String> {
    broadcast_shell_settings_changed();

    if !elevation::is_elevated() {
        return Err(WinBlastError::ElevationRequired(
            "Safe Explorer restart via CreateProcessWithTokenW requires Administrator privileges; shell settings were broadcast without restarting Explorer."
                .to_string(),
        ));
    }

    let shell_hwnd = unsafe { GetShellWindow() };
    let mut shell_pid = 0u32;
    if !shell_hwnd.0.is_null() {
        unsafe {
            GetWindowThreadProcessId(shell_hwnd, Some(&mut shell_pid));
        }
    }

    if shell_pid == 0 {
        return Err(WinBlastError::TweakFailed(
            "Cannot capture non-elevated desktop shell token (no active Shell window); sign out/in to apply shell changes safely."
                .to_string(),
        ));
    }

    let proc_handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, shell_pid) }
        .map_err(|e| WinBlastError::Hardware(format!("OpenProcess(shell) failed: {}", e)))?;
    let _proc_guard = HandleCloser(proc_handle);

    let mut shell_token = HANDLE::default();
    unsafe { OpenProcessToken(proc_handle, TOKEN_DUPLICATE | TOKEN_QUERY, &mut shell_token) }
        .map_err(|e| WinBlastError::Hardware(format!("OpenProcessToken(shell) failed: {}", e)))?;
    let _tok_guard = HandleCloser(shell_token);

    let mut dup_token = HANDLE::default();
    unsafe {
        DuplicateTokenEx(
            shell_token,
            TOKEN_DUPLICATE
                | TOKEN_QUERY
                | TOKEN_ASSIGN_PRIMARY
                | TOKEN_ADJUST_DEFAULT
                | TOKEN_ADJUST_SESSIONID,
            None,
            SecurityImpersonation,
            TokenPrimary,
            &mut dup_token,
        )
    }
    .map_err(|e| WinBlastError::Hardware(format!("DuplicateTokenEx failed: {}", e)))?;
    let _dup_guard = HandleCloser(dup_token);

    let mut env_block: *mut c_void = std::ptr::null_mut();
    unsafe { CreateEnvironmentBlock(&mut env_block, Some(dup_token), false) }
        .map_err(|e| WinBlastError::Hardware(format!("CreateEnvironmentBlock failed: {}", e)))?;
    let _env_guard = EnvBlockGuard(env_block);

    let term_handle = unsafe { OpenProcess(PROCESS_TERMINATE | PROCESS_SYNCHRONIZE, false, shell_pid) }
        .map_err(|e| {
            WinBlastError::Hardware(format!(
                "OpenProcess(terminate shell PID {}) failed before stopping Explorer: {}",
                shell_pid, e
            ))
        })?;
    let _term_guard = HandleCloser(term_handle);

    // Terminate ONLY this desktop session's shell PID with ExitCode=0 (avoids Winlogon AutoRestartShell race)
    // and wait cleanly for process exit before spawning the replacement explorer.exe.
    unsafe { TerminateProcess(term_handle, 0) }
        .map_err(|e| WinBlastError::Hardware(format!("TerminateProcess(shell) failed: {}", e)))?;
    unsafe {
        let _ = WaitForSingleObject(term_handle, 5000);
    }

    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    let explorer_exe = format!("{}\\explorer.exe", windir.trim_end_matches('\\'));
    let mut cmdline_w: Vec<u16> = format!("\"{}\"\0", explorer_exe).encode_utf16().collect();
    let app_w: Vec<u16> = explorer_exe.encode_utf16().chain(std::iter::once(0)).collect();

    let si = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };
    let mut pi = PROCESS_INFORMATION::default();

    let launched = unsafe {
        CreateProcessWithTokenW(
            dup_token,
            LOGON_WITH_PROFILE,
            PCWSTR(app_w.as_ptr()),
            Some(PWSTR(cmdline_w.as_mut_ptr())),
            CREATE_UNICODE_ENVIRONMENT,
            Some(env_block as *const c_void),
            None,
            &si,
            &mut pi,
        )
    };

    match launched {
        Ok(()) => {
            let _thread_guard = HandleCloser(pi.hThread);
            let _process_guard = HandleCloser(pi.hProcess);
            Ok("Windows Explorer restarted safely under standard desktop token.".to_string())
        }
        Err(e) => Err(WinBlastError::TweakFailed(format!(
            "Failed to relaunch non-elevated explorer.exe ({}). Press Ctrl+Shift+Esc -> File -> Run new task -> explorer.exe",
            e
        ))),
    }
}

/// Schedule a safe 10-second graceful OS reboot (never instant `shutdown /r /t 0` without grace window).
pub fn request_system_reboot() -> Result<()> {
    let out = SilentCommand::new("shutdown")
        .args([
            "/r",
            "/t",
            "10",
            "/c",
            "WinBlast0 requested a restart to finalize kernel/driver optimizations.",
        ])
        .output()
        .map_err(|e| WinBlastError::Hardware(format!("Failed to schedule reboot: {}", e)))?;

    if !out.status.success() {
        let stderr = decode_oem_bytes(&out.stderr);
        let stdout = decode_oem_bytes(&out.stdout);
        let msg = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        return Err(WinBlastError::Hardware(msg.to_string()));
    }
    Ok(())
}
