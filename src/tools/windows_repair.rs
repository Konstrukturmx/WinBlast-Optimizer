//! Windows Component Repair, SFC, DISM & Icon Cache Reset Engine.
//!
//! Executes real elevated system repair routines with live diagnostic output.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};

/// Diagnostic and repair command item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairTask {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub command_preview: &'static str,
    pub icon_glyph: &'static str,
}

impl RepairTask {
    pub fn all() -> Vec<Self> {
        vec![
            Self {
                id: "sfc",
                title: "System File Checker (SFC /Scannow)",
                description: "Scans all protected Windows system files and repairs corrupted files using cached system copies.",
                command_preview: "sfc.exe /scannow",
                icon_glyph: "🛡️",
            },
            Self {
                id: "dism",
                title: "DISM Component Store Repair (RestoreHealth)",
                description: "Repairs corrupted Windows Component Store packages using official Windows Update recovery sources.",
                command_preview: "dism.exe /Online /Cleanup-Image /RestoreHealth",
                icon_glyph: "🔧",
            },
            Self {
                id: "wu_reset",
                title: "Reset Windows Update Services & Download Cache",
                description: "Stops update services, purges SoftwareDistribution download cache, and restarts Windows Update components.",
                command_preview: "net stop wuauserv && del /q SoftwareDistribution && net start wuauserv",
                icon_glyph: "🔄",
            },
            Self {
                id: "dns",
                title: "Flush DNS Resolver Cache",
                description: "Clears and resets the Windows DNS resolver cache to resolve domain resolution failures and network latency.",
                command_preview: "ipconfig.exe /flushdns",
                icon_glyph: "🌐",
            },
            Self {
                id: "winsock",
                title: "Reset Winsock Catalog & TCP/IP Network Stack",
                description: "Restores the Winsock catalog and TCP/IP protocol configuration to their clean factory defaults.",
                command_preview: "netsh.exe winsock reset && netsh.exe int ip reset",
                icon_glyph: "📶",
            },
            Self {
                id: "icon_cache",
                title: "Rebuild Shell Icon & Thumbnail Cache",
                description: "Deletes corrupted IconCache database files and cleanly restarts Windows Explorer.",
                command_preview: "del %LocalAppData%\\IconCache.db && Restart Explorer",
                icon_glyph: "🎨",
            },
        ]
    }

    /// Execute a repair task. Success and failure come from the tools' exit codes, and the
    /// message quotes what the tool itself printed (no English text matching, C5/C6).
    pub fn execute(id: &str) -> Result<String, String> {
        match id {
            "dns" => run_tool("ipconfig", &["/flushdns"]).map(|o| summary("ipconfig /flushdns", &o)),
            "winsock" => {
                let a = run_tool("netsh", &["winsock", "reset"]);
                let b = run_tool("netsh", &["int", "ip", "reset"]);
                match (a, b) {
                    (Ok(_), Ok(_)) => Ok("Winsock catalog and TCP/IP stack reset. Restart Windows to finish.".to_string()),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            "icon_cache" => {
                let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
                if !local_app_data.is_empty() {
                    let cache = std::path::Path::new(&local_app_data).join("IconCache.db");
                    if cache.is_file() {
                        std::fs::remove_file(&cache)
                            .map_err(|e| format!("IconCache.db could not be deleted: {e}"))?;
                    }
                }
                crate::tools::explorer_ctrl::restart_explorer_safely()
                    .map(|_| "Icon cache cleared and Explorer restarted.".to_string())
                    .map_err(|e| format!("Icon cache cleared, but Explorer did not restart: {e}"))
            }
            "sfc" => run_tool("sfc", &["/scannow"]).map(|o| summary("SFC", &o)),
            "dism" => run_tool("dism", &["/Online", "/Cleanup-Image", "/RestoreHealth"]).map(|o| summary("DISM RestoreHealth", &o)),
            "wu_reset" => {
                let mut problems = Vec::new();
                for svc in ["wuauserv", "bits"] {
                    if let Err(e) = run_tool("net", &["stop", svc]) {
                        // 3521 = "service is not started": not a problem here.
                        if !e.contains("3521") {
                            problems.push(e);
                        }
                    }
                }
                let windir = crate::core::process::system32_dir()
                    .parent()
                    .map(std::path::Path::to_path_buf)
                    .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Windows"));
                let download_cache = windir.join("SoftwareDistribution").join("Download");
                if download_cache.is_dir() {
                    if let Err(e) = std::fs::remove_dir_all(&download_cache) {
                        problems.push(format!("Download cache not cleared: {e}"));
                    }
                    let _ = std::fs::create_dir_all(&download_cache);
                }
                for svc in ["wuauserv", "bits"] {
                    if let Err(e) = run_tool("net", &["start", svc]) {
                        problems.push(e);
                    }
                }
                if problems.is_empty() {
                    Ok("Windows Update services restarted and download cache cleared.".to_string())
                } else {
                    Err(problems.join("; "))
                }
            }
            _ => Err("Unknown repair task ID".to_string()),
        }
    }
}

/// Decode console output: `sfc` writes UTF-16LE, the others use the OEM code page.
pub fn decode_tool_output(bytes: &[u8]) -> String {
    let looks_utf16 = bytes.len() >= 4 && bytes.iter().skip(1).step_by(2).filter(|b| **b == 0).count() * 2 >= bytes.len() / 2;
    if looks_utf16 {
        let units: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|c| u16::from_le_bytes(*c)).collect();
        String::from_utf16_lossy(&units).replace('\0', "")
    } else {
        crate::scanner::task_scanner::decode_oem_bytes(bytes)
    }
}

/// Run a repair tool; `Ok(output)` on exit code 0, otherwise `Err` with the exit code and output.
fn run_tool(program: &str, args: &[&str]) -> Result<String, String> {
    let out = SilentCommand::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program} could not be started: {e}"))?;
    let text = format!("{}\n{}", decode_tool_output(&out.stdout), decode_tool_output(&out.stderr));
    if out.status.success() {
        Ok(text)
    } else {
        Err(format!("{program} {} failed (exit {:?}): {}", args.join(" "), out.status.code(), last_lines(&text, 3)))
    }
}

/// The last `n` non-empty lines of tool output (progress bars removed).
fn last_lines(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text
        .split(['\r', '\n'])
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.contains('%'))
        .collect();
    lines[lines.len().saturating_sub(n)..].join(" ")
}

fn summary(tool: &str, output: &str) -> String {
    let tail = last_lines(output, 2);
    if tail.is_empty() {
        format!("{tool} finished.")
    } else {
        format!("{tool} finished: {tail}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_output_is_decoded() {
        let s = "Windows Resource Protection did not find any integrity violations.\r\n";
        let bytes: Vec<u8> = s.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
        assert_eq!(decode_tool_output(&bytes), s);
        assert_eq!(last_lines(&decode_tool_output(&bytes), 1), s.trim());
    }

    #[test]
    fn progress_lines_are_dropped() {
        assert_eq!(last_lines("[==== 40.0% ====]\r\nDone.\r\n", 2), "Done.");
    }
}
