//! Silent subprocess launcher for WinBlast0.
//!
//! Enforces `CREATE_NO_WINDOW` on Windows for all subprocess invocations, and starts known
//! Windows tools by their full path (F-20): `CreateProcess` searches the exe's own folder and the
//! current directory before System32, so a `powershell.exe` planted next to a portable copy
//! would otherwise run elevated.

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

/// Wrapper constructing standard library `Command` instances with stealth execution flags.
pub struct SilentCommand;

impl SilentCommand {
    /// Create a new `Command` for `program` (resolved by [`resolve_program`]) with
    /// `CREATE_NO_WINDOW` on Windows.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        let mut cmd = Command::new(resolve_program(program.as_ref()));
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            // CREATE_NO_WINDOW: 0x0800_0000
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd
    }
}

/// Windows tools that live directly in System32.
const SYSTEM32_TOOLS: &[&str] = &[
    "bcdedit", "certutil", "cleanmgr", "cmd", "curl", "dism", "fsutil", "icacls", "ipconfig", "net",
    "netsh", "pnputil", "powercfg", "reg", "rundll32", "sc", "schtasks", "sfc", "shutdown",
    "taskkill", "tasklist", "wevtutil", "where", "whoami", "wmic", "w32tm", "manage-bde",
];

/// The System32 folder from `GetSystemDirectoryW` (not from environment variables, which a
/// user can change).
pub fn system32_dir() -> PathBuf {
    #[cfg(windows)]
    {
        let mut buf = [0u16; 260];
        let n = unsafe { windows::Win32::System::SystemInformation::GetSystemDirectoryW(Some(&mut buf)) } as usize;
        if n > 0 && n < buf.len() {
            return PathBuf::from(String::from_utf16_lossy(&buf[..n]));
        }
    }
    PathBuf::from(r"C:\Windows\System32")
}

#[cfg(windows)]
fn known_folder(id: &windows::core::GUID) -> Option<PathBuf> {
    use windows::Win32::UI::Shell::{SHGetKnownFolderPath, KNOWN_FOLDER_FLAG};
    let p = unsafe { SHGetKnownFolderPath(id, KNOWN_FOLDER_FLAG(0), None) }.ok()?;
    let s = unsafe { p.to_string() }.ok();
    unsafe { windows::Win32::System::Com::CoTaskMemFree(Some(p.0 as *const _)) };
    s.map(PathBuf::from)
}

/// `%ProgramData%` from the shell's known-folder API.
pub fn known_program_data_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        known_folder(&windows::Win32::UI::Shell::FOLDERID_ProgramData)
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Open an `https://` link in the user's default browser **without** admin rights (B7).
/// `rundll32 url.dll` from this elevated process would start the browser elevated; handing the
/// link to `explorer.exe` lets the already-running, non-elevated shell open it.
pub fn open_url_unelevated(url: &str) -> std::io::Result<()> {
    if !url.starts_with("https://") || url.contains(['"', ' ', '\n', '\r']) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "only plain https links are opened"));
    }
    let explorer = system32_dir()
        .parent()
        .map(|w| w.join("explorer.exe"))
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows\explorer.exe"));
    Command::new(explorer).arg(url).spawn().map(|_| ())
}

/// Full path for a bare tool name. Paths (anything with a separator) and unknown names are
/// returned unchanged.
pub fn resolve_program(program: &OsStr) -> PathBuf {
    let s = program.to_string_lossy();
    if s.contains(['\\', '/']) {
        return PathBuf::from(program);
    }
    let lower = s.to_ascii_lowercase();
    let base = lower.strip_suffix(".exe").unwrap_or(&lower);
    let sys = system32_dir();
    if SYSTEM32_TOOLS.contains(&base) {
        return sys.join(format!("{base}.exe"));
    }
    match base {
        "powershell" => sys.join("WindowsPowerShell").join("v1.0").join("powershell.exe"),
        "nvidia-smi" => {
            let in_sys = sys.join("nvidia-smi.exe");
            if in_sys.is_file() {
                in_sys
            } else {
                PathBuf::from(r"C:\Program Files\NVIDIA Corporation\NVSMI\nvidia-smi.exe")
            }
        }
        #[cfg(windows)]
        "winget" => known_folder(&windows::Win32::UI::Shell::FOLDERID_LocalAppData)
            .map(|d| d.join("Microsoft").join("WindowsApps").join("winget.exe"))
            .unwrap_or_else(|| PathBuf::from(program)),
        #[cfg(windows)]
        "choco" => known_program_data_dir()
            .map(|d| d.join("chocolatey").join("bin").join("choco.exe"))
            .unwrap_or_else(|| PathBuf::from(program)),
        _ => PathBuf::from(program),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_tools_resolve_to_system32() {
        let sys = system32_dir();
        assert!(sys.join("kernel32.dll").is_file(), "{} is not System32", sys.display());
        for tool in ["sc", "schtasks.exe", "PowerShell", "curl.exe", "icacls"] {
            let p = resolve_program(OsStr::new(tool));
            assert!(p.is_absolute(), "{tool} -> {}", p.display());
            assert!(p.starts_with(&sys), "{tool} -> {}", p.display());
            assert!(p.is_file(), "{tool} -> {} does not exist", p.display());
        }
    }

    #[test]
    fn paths_and_unknown_names_are_unchanged() {
        assert_eq!(resolve_program(OsStr::new(r"C:\x\y.exe")), PathBuf::from(r"C:\x\y.exe"));
        assert_eq!(resolve_program(OsStr::new("my-tool")), PathBuf::from("my-tool"));
    }
}
