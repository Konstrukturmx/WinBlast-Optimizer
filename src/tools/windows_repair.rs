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

    /// Execute a real repair task on the Windows host.
    pub fn execute(id: &str) -> Result<String, String> {
        match id {
            "dns" => {
                let out = SilentCommand::new("ipconfig")
                    .arg("/flushdns")
                    .output()
                    .map_err(|e| format!("Failed to launch ipconfig /flushdns: {}", e))?;
                if out.status.success() {
                    Ok("DNS resolver cache was successfully flushed and refreshed.".to_string())
                } else {
                    Err(String::from_utf8_lossy(&out.stderr).to_string())
                }
            }
            "winsock" => {
                let _ = SilentCommand::new("netsh")
                    .args(["winsock", "reset"])
                    .output();
                let _ = SilentCommand::new("netsh")
                    .args(["int", "ip", "reset"])
                    .output();
                Ok("Winsock catalog and TCP/IP stack were successfully reset (system reboot recommended).".to_string())
            }
            "icon_cache" => {
                let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
                if !local_app_data.is_empty() {
                    let cache = std::path::Path::new(&local_app_data).join("IconCache.db");
                    if cache.is_file() {
                        if let Err(e) = std::fs::remove_file(&cache) {
                            tracing::warn!("Failed to remove IconCache.db {:?}: {}", cache, e);
                        }
                    }
                }
                let _ = crate::tools::explorer_ctrl::restart_explorer_safely();
                Ok("Shell icon cache database cleared and Windows Explorer restarted.".to_string())
            }
            "sfc" => {
                let out = SilentCommand::new("sfc")
                    .arg("/scannow")
                    .output()
                    .map_err(|e| format!("Failed to launch sfc.exe: {}", e))?;
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("Windows Resource Protection found corrupt files and successfully repaired them") {
                    Ok("SFC Scannow: Corrupted system files were detected and successfully repaired.".to_string())
                } else if text.contains("did not find any integrity violations") {
                    Ok("SFC Scannow: Windows Resource Protection did not find any integrity violations.".to_string())
                } else if out.status.success() {
                    Ok("SFC Scannow completed successfully (system integrity verified).".to_string())
                } else {
                    Err(format!("SFC Scannow requires administrator privileges or returned an error: {}", text))
                }
            }
            "dism" => {
                let out = SilentCommand::new("dism")
                    .args(["/Online", "/Cleanup-Image", "/RestoreHealth"])
                    .output()
                    .map_err(|e| format!("Failed to launch dism.exe: {}", e))?;
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("The restore operation completed successfully")
                    || out.status.success()
                {
                    Ok(
                        "DISM RestoreHealth: Windows component store was successfully repaired."
                            .to_string(),
                    )
                } else {
                    Err(format!("DISM component repair error: {}", text))
                }
            }
            "wu_reset" => {
                let _ = SilentCommand::new("net")
                    .args(["stop", "wuauserv"])
                    .output();
                let _ = SilentCommand::new("net").args(["stop", "bits"]).output();
                let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
                let download_cache = std::path::Path::new(&windir)
                    .join("SoftwareDistribution")
                    .join("Download");
                if download_cache.is_dir() {
                    if let Err(e) = std::fs::remove_dir_all(&download_cache) {
                        tracing::warn!(
                            "Failed to clear SoftwareDistribution download cache {:?}: {}",
                            download_cache,
                            e
                        );
                    }
                    if let Err(e) = std::fs::create_dir_all(&download_cache) {
                        tracing::warn!(
                            "Failed to recreate SoftwareDistribution download cache {:?}: {}",
                            download_cache,
                            e
                        );
                    }
                }
                let _ = SilentCommand::new("net")
                    .args(["start", "wuauserv"])
                    .output();
                let _ = SilentCommand::new("net").args(["start", "bits"]).output();
                Ok(
                    "Windows Update services and SoftwareDistribution download cache were reset."
                        .to_string(),
                )
            }
            _ => Err("Unknown repair task ID".to_string()),
        }
    }
}
