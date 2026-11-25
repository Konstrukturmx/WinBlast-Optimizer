//! System Fault Diagnostics, BSOD Minidump Inspector, Pending Reboot Detector & Component Store Health.
//!
//! Provides non-intrusive scanning of `%SystemRoot%\Minidump`, CBS reboot locks,
//! Windows Update pending operations, and critical system faults.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

/// Individual BSOD Crash Dump metadata.
/// Why WinBlast no longer deletes Windows servicing state (F-22).
pub const SERVICING_STATE_REFUSAL: &str = "WinBlast no longer deletes Windows' pending-restart records: that can leave an update half-installed. Restart Windows to finish the pending work instead.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinidumpEntry {
    pub file_name: String,
    pub path: PathBuf,
    pub size_kb: u64,
    pub created_at: String,
}

/// Pending reboot trigger reasons detected in Windows.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PendingRebootStatus {
    pub is_pending: bool,
    pub cbs_reboot: bool,
    pub windows_update_reboot: bool,
    pub pending_file_rename: bool,
    pub pending_file_count: usize,
    pub pending_files: Vec<String>,
}

/// Remediation actions offered by the Fault Diagnostics view (all mutating).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultAction {
    ClearMinidumps,
    ClearPendingRenames,
    ClearCbsRebootFlags,
    RestartSystem,
}

/// Diagnostic health assessment summary.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FaultDiagnosticsReport {
    pub minidumps: Vec<MinidumpEntry>,
    pub pending_reboot: PendingRebootStatus,
    pub dcom_fault_count: u32,
    pub component_store_corrupted: bool,
    pub dism_check_recommended: bool,
    pub total_critical_faults: usize,
}

impl FaultDiagnosticsReport {
    /// Perform a fast, non-blocking fault inspection of the Windows subsystem.
    pub fn scan() -> Self {
        let mut report = Self::default();

        // 1. Scan for BSOD minidumps in %SystemRoot%\Minidump
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
        let minidump_dir = Path::new(&system_root).join("Minidump");

        if minidump_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&minidump_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|s| s.eq_ignore_ascii_case("dmp"))
                        == Some(true)
                    {
                        if let Ok(meta) = entry.metadata() {
                            let size_kb = meta.len() / 1024;
                            let created = meta
                                .modified()
                                .ok()
                                .map(|t| {
                                    let dt: chrono::DateTime<chrono::Local> = t.into();
                                    dt.format("%Y-%m-%d %H:%M").to_string()
                                })
                                .unwrap_or_else(|| "Recent".to_string());

                            report.minidumps.push(MinidumpEntry {
                                file_name: path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("crash.dmp")
                                    .to_string(),
                                path: path.clone(),
                                size_kb,
                                created_at: created,
                            });
                        }
                    }
                }
            }
        }

        // Check for full kernel memory dump
        let memory_dmp = Path::new(&system_root).join("MEMORY.DMP");
        if memory_dmp.is_file() {
            if let Ok(meta) = memory_dmp.metadata() {
                report.minidumps.push(MinidumpEntry {
                    file_name: "MEMORY.DMP (Full Kernel Dump)".to_string(),
                    path: memory_dmp,
                    size_kb: meta.len() / 1024,
                    created_at: "System Memory Dump".to_string(),
                });
            }
        }

        // 2. Check Pending Reboot flags
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // CBS Reboot Pending
        if let Ok(_cbs_key) = hklm.open_subkey(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Component Based Servicing\\RebootPending",
        ) {
            report.pending_reboot.cbs_reboot = true;
            report.pending_reboot.is_pending = true;
        }

        // Windows Update Reboot Required
        if let Ok(_wu_key) = hklm.open_subkey(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\WindowsUpdate\\Auto Update\\RebootRequired",
        ) {
            report.pending_reboot.windows_update_reboot = true;
            report.pending_reboot.is_pending = true;
        }

        // Pending File Rename Operations (Session Manager)
        if let Ok(sm_key) = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager")
        {
            let non_empty: Vec<String> = if let Ok(list) =
                sm_key.get_value::<Vec<String>, _>("PendingFileRenameOperations")
            {
                list.into_iter().filter(|s| !s.trim().is_empty()).collect()
            } else if let Ok(val) = sm_key.get_raw_value("PendingFileRenameOperations") {
                parse_multi_sz_lossless(&val.bytes)
            } else {
                Vec::new()
            };

            if !non_empty.is_empty() {
                report.pending_reboot.pending_file_rename = true;
                report.pending_reboot.pending_file_count = non_empty.len();
                report.pending_reboot.pending_files = non_empty;
                report.pending_reboot.is_pending = true;
            }
        }

        // 3. Component Store / DISM check recommendation
        report.dism_check_recommended =
            report.pending_reboot.is_pending || !report.minidumps.is_empty();

        // 4. Calculate badge count (Total critical faults)
        let mut faults = 0;
        if !report.minidumps.is_empty() {
            faults += report.minidumps.len();
        }
        if report.pending_reboot.is_pending {
            faults += 1;
        }
        if report.component_store_corrupted {
            faults += 1;
        }

        report.total_critical_faults = faults;
        report
    }

    /// Clear minidump files from Windows directory.
    pub fn clear_minidumps(&self) -> Result<usize, String> {
        let mut count = 0;
        for dump in &self.minidumps {
            if std::fs::remove_file(&dump.path).is_ok() {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Removed (B6 / F-22). Deleting `PendingFileRenameOperations` drops file replacements that
    /// an update or installer scheduled for the next boot and can leave it half-installed; the
    /// only safe way to clear it is to restart. Kept so old callers get a clear answer.
    pub fn clear_pending_renames() -> Result<(), String> {
        Err(SERVICING_STATE_REFUSAL.to_string())
    }

    /// Removed (B6 / F-22): deleting the CBS `RebootPending` / Windows Update `RebootRequired`
    /// keys hides an unfinished servicing operation instead of finishing it.
    pub fn clear_cbs_reboot_flags() -> Result<(), String> {
        Err(SERVICING_STATE_REFUSAL.to_string())
    }

    /// Restart Windows to finalize pending installations.
    pub fn restart_system() -> Result<(), String> {
        crate::core::process::SilentCommand::new("shutdown.exe")
            .args([
                "/r",
                "/t",
                "5",
                "/c",
                "WinBlast0: Rebooting to finalize updates",
            ])
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to initiate restart: {e}"))
    }
}

/// Parse a REG_MULTI_SZ buffer safely even if corrupted, with odd byte length,
/// missing terminal nulls, or lone surrogate characters.
pub fn parse_multi_sz_lossless(raw: &[u8]) -> Vec<String> {
    if raw.is_empty() {
        return Vec::new();
    }
    // Truncate to even number of bytes if corrupted with odd length
    let even_len = raw.len() - (raw.len() % 2);
    let mut u16_chars = Vec::with_capacity(even_len / 2);
    for i in (0..even_len).step_by(2) {
        u16_chars.push(u16::from_le_bytes([raw[i], raw[i + 1]]));
    }

    let mut result = Vec::new();
    let mut current = Vec::new();

    for ch in u16_chars {
        if ch == 0 {
            if !current.is_empty() {
                let s = String::from_utf16_lossy(&current);
                let trimmed = s.trim();
                if !trimmed.is_empty() {
                    result.push(trimmed.to_string());
                }
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        let s = String::from_utf16_lossy(&current);
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            result.push(trimmed.to_string());
        }
    }

    result
}
