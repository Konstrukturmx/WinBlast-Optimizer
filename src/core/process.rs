//! Silent subprocess launcher for WinBlast0.
//!
//! Enforces `CREATE_NO_WINDOW` on Windows for all subprocess invocations, and starts known
//! Windows tools by their full path (F-20): `CreateProcess` searches the exe's own folder and the
//! current directory before System32, so a `powershell.exe` planted next to a portable copy
//! would otherwise run elevated.

use std::ffi::OsStr;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::time::{Duration, Instant};

/// Wrapper constructing [`TimedCommand`]s with stealth execution flags.
pub struct SilentCommand;

impl SilentCommand {
    /// A command for `program` (resolved by [`resolve_program`]) with `CREATE_NO_WINDOW` and a
    /// timeout ([`default_timeout`]); `output()` / `status()` stop the process tree when it
    /// expires (D1 / F-9: one hung PowerShell, DISM or curl call used to freeze its feature).
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> TimedCommand {
        let resolved = resolve_program(program.as_ref());
        let name = resolved
            .file_stem()
            .map(|s| s.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        let mut cmd = Command::new(&resolved);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            // CREATE_NO_WINDOW: 0x0800_0000
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        TimedCommand { timeout: default_timeout(&name), cmd, program: name }
    }
}

/// How long a tool may run before it is stopped.
pub fn default_timeout(program: &str) -> Duration {
    match program {
        "sfc" | "dism" => Duration::from_secs(3 * 3600),
        "winget" | "choco" => Duration::from_secs(3600),
        "powershell" | "curl" | "pnputil" => Duration::from_secs(30 * 60),
        _ => Duration::from_secs(10 * 60),
    }
}

/// A `Command` whose `output()` and `status()` give up after a timeout.
pub struct TimedCommand {
    cmd: Command,
    timeout: Duration,
    program: String,
}

impl TimedCommand {
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.cmd.arg(arg);
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd.args(args);
        self
    }

    pub fn env<K: AsRef<OsStr>, V: AsRef<OsStr>>(&mut self, key: K, val: V) -> &mut Self {
        self.cmd.env(key, val);
        self
    }

    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) -> &mut Self {
        self.cmd.current_dir(dir);
        self
    }

    /// Override the timeout for this call (e.g. short probes on the monitor thread).
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Start without waiting (the caller owns the child; no timeout applies).
    pub fn spawn(&mut self) -> std::io::Result<Child> {
        self.cmd.spawn()
    }

    /// Run to completion, capturing stdout/stderr, or stop it after the timeout.
    pub fn output(&mut self) -> std::io::Result<Output> {
        self.cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = self.cmd.spawn()?;
        let mut out_pipe = child.stdout.take();
        let mut err_pipe = child.stderr.take();
        let out_reader = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(p) = out_pipe.as_mut() {
                let _ = p.read_to_end(&mut buf);
            }
            buf
        });
        let err_reader = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(p) = err_pipe.as_mut() {
                let _ = p.read_to_end(&mut buf);
            }
            buf
        });
        let status = self.wait_or_kill(&mut child)?;
        Ok(Output {
            status,
            stdout: out_reader.join().unwrap_or_default(),
            stderr: err_reader.join().unwrap_or_default(),
        })
    }

    /// Run to completion without capturing output, or stop it after the timeout.
    pub fn status(&mut self) -> std::io::Result<ExitStatus> {
        self.cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        let mut child = self.cmd.spawn()?;
        self.wait_or_kill(&mut child)
    }

    fn wait_or_kill(&self, child: &mut Child) -> std::io::Result<ExitStatus> {
        let start = Instant::now();
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            if start.elapsed() >= self.timeout {
                kill_process_tree(child.id());
                let _ = child.kill();
                let _ = child.wait();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    format!(
                        "{} did not finish within {} s and was stopped",
                        self.program,
                        self.timeout.as_secs()
                    ),
                ));
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

/// Stop a process and everything it started (`taskkill /T /F`), e.g. PowerShell's children.
pub fn kill_process_tree(pid: u32) {
    let mut cmd = Command::new(system32_dir().join("taskkill.exe"));
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    let _ = cmd
        .args(["/T", "/F", "/PID", &pid.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
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
    fn hung_command_is_stopped_after_its_timeout() {
        let start = Instant::now();
        let res = SilentCommand::new("powershell")
            .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
            .timeout(Duration::from_secs(2))
            .output();
        let err = res.expect_err("must time out");
        assert_eq!(err.kind(), std::io::ErrorKind::TimedOut);
        assert!(start.elapsed() < Duration::from_secs(15), "took {:?}", start.elapsed());
    }

    #[test]
    fn output_is_captured() {
        let out = SilentCommand::new("cmd").args(["/c", "echo hello"]).output().expect("cmd");
        assert!(out.status.success());
        assert!(String::from_utf8_lossy(&out.stdout).contains("hello"));
    }

    #[test]
    fn paths_and_unknown_names_are_unchanged() {
        assert_eq!(resolve_program(OsStr::new(r"C:\x\y.exe")), PathBuf::from(r"C:\x\y.exe"));
        assert_eq!(resolve_program(OsStr::new("my-tool")), PathBuf::from("my-tool"));
    }
}
