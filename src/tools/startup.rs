//! Unified 4-Source Startup Inspector, Live Windows Services Browser (with Critical OS/Driver
//! Protection Blocklist), and Live Scheduled Tasks Inspector.
//!
//! Reuses `TaskCache` (OEM-codepage & 8-language status aware) and hardcodes a kernel/core-OS
//! service protection list so system-critical services and kernel drivers can never be disabled.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::types::FromRegValue;
use winreg::{RegKey, RegValue, HKEY};

use crate::core::error::{Result, WinBlastError};
use crate::core::process::SilentCommand;
use crate::scanner::task_scanner::TaskCache;

/// Immutable list of Windows core services that must NEVER be disabled or stopped blindly.
pub const CRITICAL_CORE_SERVICES: &[&str] = &[
    "rpcss",
    "dcomlaunch",
    "rpceptmapper",
    "lsm",
    "samss",
    "wininit",
    "plugplay",
    "power",
    "bfe",
    "mpssvc",
    "profsvc",
    "cryptsvc",
    "winmgmt",
    "eventlog",
    "schedule",
    "dhcp",
    "dnscache",
    "nsi",
    "coremessagingregistrar",
    "brokerinfrastructure",
    "systemeventsbroker",
    "windefend",
    "wdnissvc",
    "sense",
    "securityhealthservice",
    "wscsvc",
    "trustedinstaller",
    "wuauserv",
    "usosvc",
    "waasmedicsvc",
    "gpsvc",
    "vaultsvc",
    "keyiso",
    "usermanager",
    "appinfo",
    "themes",
    "audiosrv",
    "audioendpointbuilder",
    "staterepository",
    "appxsvc",
    "camsvc",
    "timebrokersvc",
    "nlasvc",
    "netprofm",
    "deviceinstall",
    "sppsvc",
    "clipsvc",
    "bits",
];

pub fn is_protected_core_service(service_name: &str) -> bool {
    let lower = service_name.trim().to_ascii_lowercase();
    CRITICAL_CORE_SERVICES
        .iter()
        .any(|c| c.eq_ignore_ascii_case(&lower))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StartupSource {
    RegistryHkcuRun,
    RegistryHklmRun,
    StartupFolderAllUsers,
    StartupFolderCurrentUser,
    ScheduledTaskLogon,
    UwpStartupTask,
}

impl StartupSource {
    pub const fn as_label(self) -> &'static str {
        match self {
            StartupSource::RegistryHkcuRun => "HKCU\\Run",
            StartupSource::RegistryHklmRun => "HKLM\\Run",
            StartupSource::StartupFolderAllUsers => "Common Startup",
            StartupSource::StartupFolderCurrentUser => "User Startup",
            StartupSource::ScheduledTaskLogon => "Logon Task",
            StartupSource::UwpStartupTask => "Store App Task",
        }
    }
}

/// Discovered Windows startup entry across Registry Run (64/32-bit), Startup folders, UWP tasks & Logon tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub source: StartupSource,
    pub enabled: bool,
    pub location: String,
    pub hive: String,
    pub reg_path: String,
    pub approved_subkey: Option<String>,
    pub is_wow64: bool,
    pub policy_locked: bool,
    pub is_microsoft: bool,
}

/// Live Windows Win32 service entry (drivers excluded for OS safety).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveServiceInfo {
    pub service_name: String,
    pub display_name: String,
    pub start_type: u32,
    pub is_running: bool,
    pub binary_path: String,
    pub is_protected_core: bool,
    pub is_microsoft: bool,
}

/// Live Windows Scheduled Task entry (populated from cached `TaskCache` + status metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveScheduledTaskInfo {
    pub task_path: String,
    pub task_name: String,
    pub folder: String,
    pub enabled: bool,
    pub status: String,
    pub is_microsoft: bool,
}

fn read_startup_approved_enabled(hive: HKEY, subkey: &str, name: &str) -> bool {
    let root = RegKey::predef(hive);
    let path = format!(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\{}",
        subkey
    );
    let Ok(k) = root
        .open_subkey_with_flags(&path, KEY_READ | KEY_WOW64_64KEY)
        .or_else(|_| root.open_subkey(&path))
    else {
        return true;
    };

    let name_lower = name.to_lowercase();
    for (val_name, reg_val) in k.enum_values().filter_map(|v| v.ok()) {
        if val_name.to_lowercase() == name_lower {
            if let Some(&first_byte) = reg_val.bytes.first() {
                // In Windows StartupApproved 12-byte struct, bit 0 set (e.g. 0x03, 0x01, 0x05) => Disabled
                return (first_byte & 0x01) == 0;
            }
        }
    }
    true
}

/// Build the standard Windows Task Manager 12-byte `StartupApproved` binary payload.
pub fn encode_startup_approved_bytes(enable: bool) -> Vec<u8> {
    if enable {
        vec![
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    } else {
        // 0x03 disabled + current UTC Windows FILETIME (8 bytes LE)
        let unix_secs = chrono::Utc::now().timestamp().max(0) as u64;
        let filetime = (unix_secs + 11_644_473_600) * 10_000_000;
        let mut buf = vec![0x03, 0x00, 0x00, 0x00];
        buf.extend_from_slice(&filetime.to_le_bytes());
        buf
    }
}

fn replace_ascii_ci(input: &str, needle: &str, replacement: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let needle_lower = needle.to_ascii_lowercase();
    let mut out = String::with_capacity(input.len());
    let mut last = 0usize;
    for (idx, _) in lower.match_indices(&needle_lower) {
        out.push_str(&input[last..idx]);
        out.push_str(replacement);
        last = idx + needle_lower.len();
    }
    out.push_str(&input[last..]);
    out
}

pub(crate) fn expand_system_root_vars(raw: &str) -> String {
    let windir = std::env::var("WINDIR")
        .or_else(|_| std::env::var("SystemRoot"))
        .unwrap_or_else(|_| "C:\\Windows".to_string());
    let windir_clean = windir.trim().trim_end_matches(['\\', '/']);
    let trimmed = raw.trim().trim_matches('"');
    let mut expanded = replace_ascii_ci(trimmed, "%SystemRoot%", windir_clean);
    expanded = replace_ascii_ci(&expanded, "%windir%", windir_clean);
    let norm = expanded.replace('/', "\\");
    if norm.len() >= 12 && norm[..12].eq_ignore_ascii_case("\\systemroot\\") {
        format!("{}\\{}", windir_clean, &norm[12..]).to_lowercase()
    } else {
        norm.to_lowercase()
    }
}

fn is_microsoft_run_command(name: &str, cmd: &str) -> bool {
    let exp = expand_system_root_vars(cmd);
    let windir_lower = std::env::var("WINDIR")
        .or_else(|_| std::env::var("SystemRoot"))
        .unwrap_or_else(|_| "C:\\Windows".to_string())
        .trim_end_matches(['\\', '/'])
        .to_lowercase();
    let name_lower = name.to_lowercase();
    (!exp.contains("\\driverstore\\")
        && (exp.contains(&format!("{}\\", windir_lower))
            || exp.contains("\\microsoft\\onedrive\\")
            || exp.contains("\\microsoft\\edge\\")
            || exp.contains("\\windows defender\\")))
        || name_lower.contains("microsoft")
        || name_lower == "onedrive"
        || name_lower == "onedrivesetup"
        || name_lower == "securityhealth"
}

fn scan_run_key(
    entries: &mut Vec<StartupEntry>,
    hkey: HKEY,
    hive_name: &str,
    subpath: &str,
    approved_subkey: Option<&str>,
    source: StartupSource,
    is_wow64: bool,
) {
    let root = RegKey::predef(hkey);
    let flags = KEY_READ
        | if is_wow64 {
            KEY_WOW64_32KEY
        } else {
            KEY_WOW64_64KEY
        };
    let Ok(k) = root.open_subkey_with_flags(subpath, flags) else {
        return;
    };

    let is_run_once = approved_subkey.is_none();
    let id_bucket = approved_subkey.unwrap_or(if is_wow64 { "RunOnce32" } else { "RunOnce" });

    for (name, val) in k.enum_values().filter_map(|v| v.ok()) {
        let cmd = String::from_reg_value(&val).unwrap_or_default();
        if cmd.trim().is_empty() {
            continue;
        }
        let enabled = match approved_subkey {
            Some(sub) => read_startup_approved_enabled(hkey, sub, &name),
            None => true,
        };
        let is_ms = is_microsoft_run_command(&name, &cmd);

        entries.push(StartupEntry {
            id: format!("{}|{}|{}", hive_name, id_bucket, name),
            name: name.clone(),
            command: cmd,
            source,
            enabled,
            location: format!(
                "{}\\{}{}",
                hive_name,
                subpath,
                if is_wow64 && is_run_once {
                    " (WOW6432)"
                } else {
                    ""
                }
            ),
            hive: hive_name.to_string(),
            reg_path: subpath.to_string(),
            approved_subkey: approved_subkey.map(str::to_string),
            is_wow64,
            policy_locked: is_run_once,
            is_microsoft: is_ms,
        });
    }
}

fn scan_startup_folder(
    entries: &mut Vec<StartupEntry>,
    dir: &Path,
    source: StartupSource,
    hive_for_approved: HKEY,
    hive_name: &str,
) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for ent in rd.flatten() {
        let p = ent.path();
        if !p.is_file() {
            continue;
        }
        let fname = p
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        if fname.is_empty() || fname.eq_ignore_ascii_case("desktop.ini") {
            continue;
        }
        let enabled = read_startup_approved_enabled(hive_for_approved, "StartupFolder", &fname);
        entries.push(StartupEntry {
            id: format!("folder|{}|{}", hive_name, fname),
            name: fname.clone(),
            command: p.display().to_string(),
            source,
            enabled,
            location: dir.display().to_string(),
            hive: hive_name.to_string(),
            reg_path: String::new(),
            approved_subkey: Some("StartupFolder".to_string()),
            is_wow64: false,
            policy_locked: false,
            is_microsoft: fname.to_lowercase().contains("microsoft"),
        });
    }
}

fn scan_uwp_startup_tasks(entries: &mut Vec<StartupEntry>) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let base = "Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\SystemAppData";
    let Ok(root) = hkcu.open_subkey_with_flags(base, KEY_READ | KEY_WOW64_64KEY) else {
        return;
    };

    for fam in root.enum_keys().filter_map(|k| k.ok()) {
        let Ok(fam_key) = root.open_subkey_with_flags(&fam, KEY_READ | KEY_WOW64_64KEY) else {
            continue;
        };
        for task_id in fam_key.enum_keys().filter_map(|k| k.ok()) {
            let Ok(tk) = fam_key.open_subkey_with_flags(&task_id, KEY_READ | KEY_WOW64_64KEY)
            else {
                continue;
            };
            let Ok(state): std::result::Result<u32, _> = tk.get_value("State") else {
                continue;
            };
            // State: 0 = Disabled, 1 = DisabledByUser, 2 = Enabled, 3/4/5 = Policy controlled
            let policy_locked = state >= 3;
            let enabled = state == 2 || state == 4;
            let short_name = fam.split('_').next().unwrap_or(&fam).to_string();
            let reg_path = format!("{}\\{}\\{}", base, fam, task_id);
            entries.push(StartupEntry {
                id: format!("uwp|{}|{}", fam, task_id),
                name: format!("{} ({})", short_name, task_id),
                command: format!("shell:AppsFolder\\{}!{}", fam, task_id),
                source: StartupSource::UwpStartupTask,
                enabled,
                location: format!("HKCU\\{}", reg_path),
                hive: "HKCU".to_string(),
                reg_path,
                approved_subkey: None,
                is_wow64: false,
                policy_locked,
                is_microsoft: fam.to_ascii_lowercase().starts_with("microsoft"),
            });
        }
    }
}

/// Enumerate all 4 Windows startup sources fast via Win32 Registry & `TaskCache`.
pub fn list_startup_entries() -> Result<Vec<StartupEntry>> {
    let mut entries = Vec::new();
    let run_path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let run_once_path = "Software\\Microsoft\\Windows\\CurrentVersion\\RunOnce";

    scan_run_key(
        &mut entries,
        HKEY_CURRENT_USER,
        "HKCU",
        run_path,
        Some("Run"),
        StartupSource::RegistryHkcuRun,
        false,
    );
    scan_run_key(
        &mut entries,
        HKEY_CURRENT_USER,
        "HKCU",
        run_once_path,
        None,
        StartupSource::RegistryHkcuRun,
        false,
    );
    scan_run_key(
        &mut entries,
        HKEY_LOCAL_MACHINE,
        "HKLM",
        run_path,
        Some("Run"),
        StartupSource::RegistryHklmRun,
        false,
    );
    scan_run_key(
        &mut entries,
        HKEY_LOCAL_MACHINE,
        "HKLM",
        run_path,
        Some("Run32"),
        StartupSource::RegistryHklmRun,
        true,
    );
    scan_run_key(
        &mut entries,
        HKEY_LOCAL_MACHINE,
        "HKLM",
        run_once_path,
        None,
        StartupSource::RegistryHklmRun,
        false,
    );
    scan_run_key(
        &mut entries,
        HKEY_LOCAL_MACHINE,
        "HKLM",
        run_once_path,
        None,
        StartupSource::RegistryHklmRun,
        true,
    );

    if let Ok(appdata) = std::env::var("APPDATA") {
        let user_dir = PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Startup");
        scan_startup_folder(
            &mut entries,
            &user_dir,
            StartupSource::StartupFolderCurrentUser,
            HKEY_CURRENT_USER,
            "HKCU",
        );
    }
    if let Ok(progdata) = std::env::var("ProgramData") {
        let common_dir = PathBuf::from(progdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("StartUp");
        scan_startup_folder(
            &mut entries,
            &common_dir,
            StartupSource::StartupFolderAllUsers,
            HKEY_LOCAL_MACHINE,
            "HKLM",
        );
    }

    scan_uwp_startup_tasks(&mut entries);

    entries.sort_by_key(|a| a.name.to_lowercase());
    Ok(entries)
}

/// Toggle a startup entry cleanly using Task Manager's 12-byte `StartupApproved` binary value or UWP `State`.
pub fn set_startup_entry_enabled(entry: &StartupEntry, enable: bool) -> Result<()> {
    if entry.policy_locked {
        return Err(WinBlastError::TweakFailed(format!(
            "Startup entry '{}' is policy-locked or read-only.",
            entry.name
        )));
    }

    match entry.source {
        StartupSource::UwpStartupTask => {
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let (k, _) = hkcu
                .create_subkey_with_flags(&entry.reg_path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
                .map_err(WinBlastError::from_reg_err)?;
            let state: u32 = if enable { 2 } else { 1 };
            k.set_value("State", &state)
                .map_err(WinBlastError::from_reg_err)?;
            Ok(())
        }
        StartupSource::ScheduledTaskLogon => {
            crate::executor::ps_runner::set_scheduled_task_enabled(&entry.location, enable)
        }
        _ => {
            let Some(subkey) = entry.approved_subkey.as_deref() else {
                return Err(WinBlastError::TweakFailed(format!(
                    "Startup entry '{}' does not support StartupApproved toggling (RunOnce / policy item).",
                    entry.name
                )));
            };
            if subkey.eq_ignore_ascii_case("RunOnce")
                || entry.reg_path.to_ascii_lowercase().ends_with("\\runonce")
            {
                return Err(WinBlastError::TweakFailed(format!(
                    "RunOnce entry '{}' does not use StartupApproved.",
                    entry.name
                )));
            }
            let root_hkey = if entry.hive.eq_ignore_ascii_case("HKCU") {
                HKEY_CURRENT_USER
            } else {
                HKEY_LOCAL_MACHINE
            };
            let root = RegKey::predef(root_hkey);
            let path = format!(
                "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\{}",
                subkey
            );
            let (k, _) = root
                .create_subkey_with_flags(&path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
                .map_err(WinBlastError::from_reg_err)?;
            let bytes = encode_startup_approved_bytes(enable);
            let val = RegValue {
                bytes,
                vtype: REG_BINARY,
            };
            k.set_raw_value(&entry.name, &val)
                .map_err(WinBlastError::from_reg_err)?;
            Ok(())
        }
    }
}

/// Toggle a startup entry by its unique `StartupEntry::id` (e.g. `"HKCU|Run|OneDrive"`).
pub fn set_startup_entry_enabled_by_id(id: &str, enable: bool) -> Result<()> {
    let entries = list_startup_entries()?;
    let entry = entries.into_iter().find(|e| e.id == id).ok_or_else(|| {
        WinBlastError::TweakFailed(format!(
            "Startup entry '{}' was not found on the system.",
            id
        ))
    })?;
    set_startup_entry_enabled(&entry, enable)
}

struct ScHandleGuard(windows::Win32::System::Services::SC_HANDLE);

impl Drop for ScHandleGuard {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = windows::Win32::System::Services::CloseServiceHandle(self.0);
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ScmServiceRecord {
    is_running: bool,
    display_name: String,
}

fn enum_scm_services_by_type(
    scm: windows::Win32::System::Services::SC_HANDLE,
    svc_type: windows::Win32::System::Services::ENUM_SERVICE_TYPE,
) -> std::result::Result<HashMap<String, ScmServiceRecord>, windows::core::Error> {
    use windows::Win32::Foundation::ERROR_MORE_DATA;
    use windows::Win32::System::Services::{
        EnumServicesStatusExW, ENUM_SERVICE_STATUS_PROCESSW, SC_ENUM_PROCESS_INFO, SERVICE_RUNNING,
        SERVICE_STATE_ALL,
    };

    let mut map = HashMap::new();
    let mut bytes_needed = 0u32;
    let mut services_returned = 0u32;
    let mut resume_handle = 0u32;

    if let Err(e) = unsafe {
        EnumServicesStatusExW(
            scm,
            SC_ENUM_PROCESS_INFO,
            svc_type,
            SERVICE_STATE_ALL,
            None,
            &mut bytes_needed,
            &mut services_returned,
            Some(&mut resume_handle),
            None,
        )
    } {
        if e.code() != ERROR_MORE_DATA.to_hresult() {
            return Err(e);
        }
    }

    if bytes_needed == 0 {
        return Ok(map);
    }

    let initial_bytes = (bytes_needed as usize).saturating_add(4096);
    let mut aligned_buf = vec![0u64; initial_bytes.div_ceil(8)];
    resume_handle = 0;

    for _ in 0..128 {
        services_returned = 0;
        let buf_len = aligned_buf.len() * 8;
        let byte_slice =
            unsafe { std::slice::from_raw_parts_mut(aligned_buf.as_mut_ptr() as *mut u8, buf_len) };
        let res = unsafe {
            EnumServicesStatusExW(
                scm,
                SC_ENUM_PROCESS_INFO,
                svc_type,
                SERVICE_STATE_ALL,
                Some(byte_slice),
                &mut bytes_needed,
                &mut services_returned,
                Some(&mut resume_handle),
                None,
            )
        };

        let count = services_returned as usize;
        let max_count = buf_len / std::mem::size_of::<ENUM_SERVICE_STATUS_PROCESSW>();
        let entries = unsafe {
            std::slice::from_raw_parts(
                byte_slice.as_ptr() as *const ENUM_SERVICE_STATUS_PROCESSW,
                count.min(max_count),
            )
        };
        for item in entries {
            if !item.lpServiceName.is_null() {
                if let Ok(svc_name) = unsafe { item.lpServiceName.to_string() } {
                    let disp = if !item.lpDisplayName.is_null() {
                        unsafe { item.lpDisplayName.to_string() }.unwrap_or_default()
                    } else {
                        String::new()
                    };
                    map.insert(
                        svc_name.trim().to_ascii_lowercase(),
                        ScmServiceRecord {
                            is_running: item.ServiceStatusProcess.dwCurrentState == SERVICE_RUNNING,
                            display_name: disp,
                        },
                    );
                }
            }
        }

        match res {
            Ok(()) => break,
            Err(e) if e.code() == ERROR_MORE_DATA.to_hresult() => {
                if services_returned == 0 {
                    let needed = (bytes_needed as usize).saturating_add(4096);
                    if needed > buf_len {
                        aligned_buf.resize(needed.div_ceil(8), 0);
                    } else {
                        break;
                    }
                }
            }
            Err(_) => break,
        }
    }

    Ok(map)
}

fn query_scm_win32_services(
    scm: windows::Win32::System::Services::SC_HANDLE,
) -> HashMap<String, ScmServiceRecord> {
    use windows::Win32::Foundation::ERROR_INVALID_PARAMETER;
    use windows::Win32::System::Services::{ENUM_SERVICE_TYPE, SERVICE_WIN32};

    // Include SERVICE_USER_SERVICE (0x00000040) alongside SERVICE_WIN32; fall back to SERVICE_WIN32 on older hosts.
    const SERVICE_USER_SERVICE_FLAG: u32 = 0x0000_0040;
    let combined = ENUM_SERVICE_TYPE(SERVICE_WIN32.0 | SERVICE_USER_SERVICE_FLAG);
    match enum_scm_services_by_type(scm, combined) {
        Ok(m) => m,
        Err(e) if e.code() == ERROR_INVALID_PARAMETER.to_hresult() => {
            enum_scm_services_by_type(scm, SERVICE_WIN32).unwrap_or_default()
        }
        Err(_) => HashMap::new(),
    }
}

fn query_scm_service_config(
    scm: windows::Win32::System::Services::SC_HANDLE,
    service_name: &str,
) -> Option<(u32, String)> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
    use windows::Win32::System::Services::{
        OpenServiceW, QueryServiceConfigW, QUERY_SERVICE_CONFIGW, SERVICE_QUERY_CONFIG,
    };

    let svc_w: Vec<u16> = service_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let svc = unsafe { OpenServiceW(scm, PCWSTR(svc_w.as_ptr()), SERVICE_QUERY_CONFIG) }.ok()?;
    let _svc_guard = ScHandleGuard(svc);

    let mut needed = 0u32;
    if let Err(e) = unsafe { QueryServiceConfigW(svc, None, 0, &mut needed) } {
        if e.code() != ERROR_INSUFFICIENT_BUFFER.to_hresult() {
            return None;
        }
    }
    if needed == 0 {
        return None;
    }
    let mut aligned = vec![0u64; (needed as usize).div_ceil(8)];
    let ptr = aligned.as_mut_ptr() as *mut QUERY_SERVICE_CONFIGW;
    unsafe { QueryServiceConfigW(svc, Some(ptr), (aligned.len() * 8) as u32, &mut needed) }.ok()?;
    let cfg = unsafe { &*ptr };
    let bin_path = if !cfg.lpBinaryPathName.is_null() {
        unsafe { cfg.lpBinaryPathName.to_string() }.unwrap_or_default()
    } else {
        String::new()
    };
    Some((cfg.dwStartType.0, bin_path))
}

fn resolve_indirect_display_string(raw: &str) -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::SHLoadIndirectString;

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if !trimmed.starts_with('@') {
        return Some(trimmed.to_string());
    }
    let src_w: Vec<u16> = trimmed.encode_utf16().chain(std::iter::once(0)).collect();
    let mut out_buf = [0u16; 512];
    unsafe { SHLoadIndirectString(PCWSTR(src_w.as_ptr()), &mut out_buf, None) }.ok()?;
    let nul_pos = out_buf
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(out_buf.len());
    let resolved = String::from_utf16_lossy(&out_buf[..nul_pos])
        .trim()
        .to_string();
    if resolved.is_empty() || resolved.starts_with('@') {
        None
    } else {
        Some(resolved)
    }
}

pub(crate) fn is_microsoft_service_binary(service_name: &str, bin_path: &str) -> bool {
    if is_protected_core_service(service_name) {
        return true;
    }
    let name_lower = service_name.trim().to_ascii_lowercase();
    if matches!(
        name_lower.as_str(),
        "edgeupdate" | "edgeupdatem" | "microsoftedgeelevationservice"
    ) || name_lower.starts_with("microsoftedge")
    {
        return true;
    }

    let expanded = expand_system_root_vars(bin_path);
    if expanded.is_empty() || expanded.contains("\\driverstore\\") {
        return false;
    }
    let windir_lower = std::env::var("WINDIR")
        .or_else(|_| std::env::var("SystemRoot"))
        .unwrap_or_else(|_| "C:\\Windows".to_string())
        .trim_end_matches(['\\', '/'])
        .to_lowercase();

    let exe_token = expanded.split_whitespace().next().unwrap_or(&expanded);
    let exe_clean = exe_token.trim_matches('"');

    exe_clean.ends_with("\\svchost.exe")
        || exe_clean == "svchost.exe"
        || exe_clean.starts_with(&format!("{}\\system32\\", windir_lower))
        || exe_clean.starts_with(&format!("{}\\syswow64\\", windir_lower))
        || exe_clean.starts_with(&format!("{}\\servicing\\", windir_lower))
        || exe_clean.starts_with(&format!("{}\\winsxs\\", windir_lower))
        || exe_clean.starts_with("system32\\")
        || exe_clean.contains("\\windows defender\\")
        || exe_clean.contains("\\microsoft\\edge\\")
        || exe_clean.contains("\\windowsapps\\microsoft.")
}

/// Enumerate Win32 & per-user services (`Type & 0x30 != 0`, filtering out kernel drivers `Type <= 8`)
/// via `HKLM\SYSTEM\CurrentControlSet\Services` + Win32 SCM (`EnumServicesStatusExW` with `SERVICE_USER_SERVICE`).
/// Preserves ACL-restricted protected services even when unelevated and resolves `@resource,-id` display strings.
pub fn list_live_services() -> Result<Vec<LiveServiceInfo>> {
    use windows::Win32::System::Services::{OpenSCManagerW, SC_MANAGER_ENUMERATE_SERVICE};

    let scm_opt = unsafe { OpenSCManagerW(None, None, SC_MANAGER_ENUMERATE_SERVICE) }.ok();
    let scm_guard = scm_opt.map(ScHandleGuard);
    let scm_map = scm_opt.map(query_scm_win32_services).unwrap_or_default();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let svc_root = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Services",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .map_err(WinBlastError::from_reg_err)?;

    let mut list = Vec::new();
    for name in svc_root.enum_keys().filter_map(|k| k.ok()) {
        let lower = name.to_ascii_lowercase();
        let scm_rec = scm_map.get(&lower);

        let sk = match svc_root.open_subkey_with_flags(&name, KEY_READ | KEY_WOW64_64KEY) {
            Ok(k) => k,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                // Preserve ACL-restricted protected/PPL Win32 services on unelevated hosts without pulling drivers
                if scm_rec.is_none() && !is_protected_core_service(&name) {
                    continue;
                }
                let (cfg_start, cfg_bin) = scm_opt
                    .and_then(|scm| query_scm_service_config(scm, &name))
                    .unwrap_or((2, String::new()));
                let scm_disp = scm_rec
                    .map(|r| r.display_name.as_str())
                    .filter(|s| !s.is_empty())
                    .unwrap_or(&name);
                let display_name = resolve_indirect_display_string(scm_disp)
                    .unwrap_or_else(|| scm_disp.to_string());
                let is_protected = is_protected_core_service(&name);
                let is_ms = is_protected || is_microsoft_service_binary(&name, &cfg_bin);
                list.push(LiveServiceInfo {
                    is_running: scm_rec.is_some_and(|r| r.is_running),
                    is_protected_core: is_protected,
                    service_name: name,
                    display_name,
                    start_type: cfg_start,
                    binary_path: cfg_bin,
                    is_microsoft: is_ms,
                });
                continue;
            }
            Err(_) => continue,
        };

        let svc_type: u32 = sk.get_value("Type").unwrap_or(0);
        // Only Win32 / per-user services (0x10 / 0x20 / 0x50 / 0x60 / 0x100+); skip kernel/FS drivers
        if (svc_type & 0x30) == 0 {
            continue;
        }
        let Ok(start_type): std::result::Result<u32, _> = sk.get_value("Start") else {
            continue;
        };
        let display_raw: String = sk.get_value("DisplayName").unwrap_or_else(|_| name.clone());
        let display_name = resolve_indirect_display_string(&display_raw)
            .or_else(|| {
                scm_rec
                    .map(|r| r.display_name.trim())
                    .filter(|s| !s.is_empty() && !s.starts_with('@'))
                    .map(str::to_string)
            })
            .unwrap_or_else(|| name.clone());
        let bin_path: String = sk.get_value("ImagePath").unwrap_or_default();
        let is_ms = is_microsoft_service_binary(&name, &bin_path);

        list.push(LiveServiceInfo {
            is_running: scm_rec.is_some_and(|r| r.is_running),
            is_protected_core: is_protected_core_service(&name),
            service_name: name,
            display_name,
            start_type,
            binary_path: bin_path,
            is_microsoft: is_ms,
        });
    }

    drop(scm_guard);
    list.sort_by_key(|a| a.service_name.to_ascii_lowercase());
    Ok(list)
}

/// Convert loaded `TaskCache` (fast, OEM/locale safe) into rich `LiveScheduledTaskInfo` list.
pub fn list_live_tasks_from_cache(cache: &TaskCache) -> Vec<LiveScheduledTaskInfo> {
    let mut out: Vec<LiveScheduledTaskInfo> = cache
        .all_entries()
        .map(|(path, enabled)| {
            let folder = path
                .rsplit_once('\\')
                .map(|(f, _)| f.to_string())
                .unwrap_or_else(|| "\\".to_string());
            let task_name = path
                .rsplit_once('\\')
                .map(|(_, n)| n.to_string())
                .unwrap_or_else(|| path.clone());
            let is_ms = path
                .trim_start_matches('\\')
                .to_ascii_lowercase()
                .starts_with("microsoft\\");
            LiveScheduledTaskInfo {
                task_path: path.clone(),
                task_name,
                folder,
                enabled: *enabled,
                status: if *enabled {
                    "Ready / Enabled".to_string()
                } else {
                    "Disabled".to_string()
                },
                is_microsoft: is_ms,
            }
        })
        .collect();
    out.sort_by_key(|a| a.task_path.to_ascii_lowercase());
    out
}

/// Trigger immediate execution of a scheduled task (`schtasks /Run`) silently.
pub fn run_scheduled_task(task_path: &str) -> Result<()> {
    let out = SilentCommand::new("schtasks")
        .args(["/Run", "/TN", task_path])
        .output()
        .map_err(|e| {
            WinBlastError::ScheduledTask(format!("Failed to run task {}: {}", task_path, e))
        })?;
    if !out.status.success() {
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
        return Err(WinBlastError::ScheduledTask(format!(
            "Failed to run task {}: {}",
            task_path,
            stderr.trim()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_critical_service_blocklist_protects_core_services() {
        assert!(is_protected_core_service("RpcSs"));
        assert!(is_protected_core_service("DcomLaunch"));
        assert!(is_protected_core_service("wuauserv"));
        assert!(is_protected_core_service("WinDefend"));
        assert!(!is_protected_core_service("DiagTrack"));
        assert!(!is_protected_core_service("Fax"));
    }

    #[test]
    fn test_startup_approved_12_byte_encoding() {
        let en = encode_startup_approved_bytes(true);
        let dis = encode_startup_approved_bytes(false);
        assert_eq!(en.len(), 12);
        assert_eq!(dis.len(), 12);
        assert_eq!(en[0] & 0x01, 0);
        assert_eq!(dis[0] & 0x01, 1);
    }
}
