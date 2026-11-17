//! System Fault Diagnostics, BSOD Minidump Inspector, Pending Reboot Detector & Component Store Health.
//!
//! Provides non-intrusive scanning of `%SystemRoot%\Minidump`, CBS reboot locks,
//! Windows Update pending operations, and critical system faults.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

/// Individual BSOD Crash Dump metadata.
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
        if let Ok(sm_key) =
            hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Session Manager")
        {
            let files: Result<Vec<String>, _> = sm_key.get_value("PendingFileRenameOperations");
            if let Ok(list) = files {
                let non_empty: Vec<String> = list.into_iter().filter(|s| !s.trim().is_empty()).collect();
                if !non_empty.is_empty() {
                    report.pending_reboot.pending_file_rename = true;
                    report.pending_reboot.pending_file_count = non_empty.len();
                    report.pending_reboot.pending_files = non_empty;
                    report.pending_reboot.is_pending = true;
                }
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

    /// Clear pending file rename operations in Windows registry (requires elevation).
    pub fn clear_pending_renames() -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let sm_key = hklm.open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
            KEY_SET_VALUE,
        ).map_err(|e| format!("Failed to open Session Manager key: {e}"))?;
        sm_key.delete_value("PendingFileRenameOperations")
            .map_err(|e| format!("Failed to clear PendingFileRenameOperations: {e}"))?;
        Ok(())
    }

    /// Clear CBS / Windows Update reboot pending flags.
    pub fn clear_cbs_reboot_flags() -> Result<(), String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let _ = hklm.delete_subkey_all(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Component Based Servicing\\RebootPending",
        );
        let _ = hklm.delete_subkey_all(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\WindowsUpdate\\Auto Update\\RebootRequired",
        );
        Ok(())
    }

    /// Restart Windows to finalize pending installations.
    pub fn restart_system() -> Result<(), String> {
        crate::core::process::SilentCommand::new("shutdown.exe")
            .args(["/r", "/t", "5", "/c", "WinBlast0: Rebooting to finalize updates"])
            .spawn()
            .map(|_| ())
            .map_err(|e| format!("Failed to initiate restart: {e}"))
    }
}
