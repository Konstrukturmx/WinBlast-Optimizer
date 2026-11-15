//! Silent subprocess launcher for WinBlast0.
//!
//! Enforces `CREATE_NO_WINDOW` on Windows for all subprocess invocations,
//! completely eliminating command prompt, PowerShell, and terminal window flashing.

use std::ffi::OsStr;
use std::process::Command;

/// Wrapper constructing standard library `Command` instances with stealth execution flags.
pub struct SilentCommand;

impl SilentCommand {
    /// Create a new `Command` configured with `CREATE_NO_WINDOW` on Windows.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        let mut cmd = Command::new(program);
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
