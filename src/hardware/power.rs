//! Native Win32 Power Management subsystem (`powrprof.dll`).
//!
//! Eliminates slow, locale-dependent `powercfg.exe` subprocesses by calling
//! Windows Power Policy APIs directly: `PowerGetActiveScheme`, `PowerReadACValueIndex`,
//! `PowerWriteACValueIndex`, `PowerSetActiveScheme`, `PowerDuplicateScheme`, and
//! `CallNtPowerInformation` (hibernation & Modern Standby `AoAc` detection).

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

use windows::core::GUID;
use windows::Win32::Foundation::{LocalFree, HLOCAL, NTSTATUS, WIN32_ERROR};
use windows::Win32::System::Power::{
    CallNtPowerInformation, GetSystemPowerStatus, PowerDuplicateScheme, PowerGetActiveScheme,
    PowerReadACDefaultIndex, PowerReadACValueIndex, PowerReadDCDefaultIndex, PowerReadDCValueIndex,
    PowerReadValueMax, PowerReadValueMin, PowerSetActiveScheme, PowerWriteACValueIndex,
    PowerWriteDCValueIndex, PowerWriteFriendlyName, PowerWriteValueMax, PowerWriteValueMin,
    SystemPowerCapabilities, SystemReserveHiberFile, SYSTEM_POWER_CAPABILITIES,
    SYSTEM_POWER_STATUS,
};

use crate::core::error::{Result, WinBlastError};

/// Metadata describing an installed Windows Power Plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSchemeInfo {
    pub guid: String,
    pub friendly_name: String,
    pub is_active: bool,
}

/// Live hibernation, Fast Startup, and Modern Standby (`S0ix`) state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HibernationStatus {
    pub hibernation_enabled: bool,
    pub hiberfile_present: bool,
    pub fast_startup_enabled: bool,
    pub modern_standby_s0ix: bool,
}

/// Normalize a GUID string (handles optional `{}` braces and casing) to lowercase dashed format.
pub fn normalize_guid_str(s: &str) -> String {
    s.trim().trim_start_matches('{').trim_end_matches('}').to_ascii_lowercase()
}

/// Parse a dashed GUID string (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` with optional `{}`).
pub fn parse_guid(s: &str) -> Result<GUID> {
    let clean = normalize_guid_str(s);
    let parts: Vec<&str> = clean.split('-').collect();
    if parts.len() != 5
        || parts[0].len() != 8
        || parts[1].len() != 4
        || parts[2].len() != 4
        || parts[3].len() != 4
        || parts[4].len() != 12
    {
        return Err(WinBlastError::Hardware(format!("Invalid GUID format: {}", s)));
    }
    let data1 = u32::from_str_radix(parts[0], 16)
        .map_err(|_| WinBlastError::Hardware(format!("Invalid GUID segment: {}", s)))?;
    let data2 = u16::from_str_radix(parts[1], 16)
        .map_err(|_| WinBlastError::Hardware(format!("Invalid GUID segment: {}", s)))?;
    let data3 = u16::from_str_radix(parts[2], 16)
        .map_err(|_| WinBlastError::Hardware(format!("Invalid GUID segment: {}", s)))?;
    let d4_hi = u16::from_str_radix(parts[3], 16)
        .map_err(|_| WinBlastError::Hardware(format!("Invalid GUID segment: {}", s)))?;
    let mut data4 = [0u8; 8];
    data4[0] = (d4_hi >> 8) as u8;
    data4[1] = (d4_hi & 0xFF) as u8;
    for i in 0..6 {
        let byte_str = &parts[4][i * 2..i * 2 + 2];
        data4[2 + i] = u8::from_str_radix(byte_str, 16)
            .map_err(|_| WinBlastError::Hardware(format!("Invalid GUID hex byte: {}", s)))?;
    }
    Ok(GUID::from_values(data1, data2, data3, data4))
}

/// Format a Win32 `GUID` to lowercase canonical string.
pub fn format_guid(g: &GUID) -> String {
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        g.data1,
        g.data2,
        g.data3,
        g.data4[0],
        g.data4[1],
        g.data4[2],
        g.data4[3],
        g.data4[4],
        g.data4[5],
        g.data4[6],
        g.data4[7],
    )
}

/// RAII wrapper ensuring pointers returned by `PowerGetActiveScheme` / `PowerDuplicateScheme`
/// are always freed via `LocalFree`.
struct LocalGuidPtr(*mut GUID);

impl Drop for LocalGuidPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                let _ = LocalFree(Some(HLOCAL(self.0.cast())));
            }
        }
    }
}

fn map_win32_err(op: &str, code: WIN32_ERROR) -> WinBlastError {
    if code.0 == 5 {
        WinBlastError::ElevationRequired(format!("{}: Access denied (ERROR_ACCESS_DENIED)", op))
    } else {
        WinBlastError::Hardware(format!("{}: Win32 error 0x{:08X} ({})", op, code.0, code.0))
    }
}

/// Query whether the system is currently running on AC mains power via `GetSystemPowerStatus`.
/// Defaults to `true` on desktops (`ACLineStatus == 255`) or API failure.
pub fn is_on_ac_power() -> bool {
    let mut status = SYSTEM_POWER_STATUS::default();
    if unsafe { GetSystemPowerStatus(&mut status) }.is_ok() {
        status.ACLineStatus != 0
    } else {
        true
    }
}

/// Unlock a numeric power setting's `[ValueMin, ValueMax]` metadata range if `target` falls
/// outside the advertised bounds (e.g. `CPMINCORES` clamped to `ValueMax = 0` on Win11 24H2).
///
/// Returns `Ok(false)` without mutating anything for enumerated settings where
/// `PowerReadValueMin`/`PowerReadValueMax` fail (e.g., `ERROR_FILE_NOT_FOUND`).
fn unlock_power_setting_range_if_clamped(
    subgroup: &GUID,
    setting: &GUID,
    target: u32,
) -> Result<bool> {
    let mut min_val = 0u32;
    let mut max_val = 0u32;
    let min_rc = unsafe { PowerReadValueMin(None, Some(subgroup), Some(setting), &mut min_val) };
    let max_rc = unsafe { PowerReadValueMax(None, Some(subgroup), Some(setting), &mut max_val) };
    if min_rc != WIN32_ERROR(0) || max_rc != WIN32_ERROR(0) {
        return Ok(false);
    }

    let mut changed = false;
    if target < min_val {
        let rc = unsafe { PowerWriteValueMin(None, Some(subgroup), Some(setting), target) };
        if rc != WIN32_ERROR(0) {
            return Err(map_win32_err("PowerWriteValueMin", rc));
        }
        changed = true;
    }
    if target > max_val {
        let new_max = if target <= 100 && max_val <= 100 {
            100
        } else {
            target
        };
        let rc = unsafe { PowerWriteValueMax(None, Some(subgroup), Some(setting), new_max) };
        if rc != WIN32_ERROR(0) {
            return Err(map_win32_err("PowerWriteValueMax", rc));
        }
        changed = true;
    }
    if changed {
        tracing::info!(
            "Unlocked clamped power setting range {}/{} (was [{}, {}], target {})",
            format_guid(subgroup),
            format_guid(setting),
            min_val,
            max_val,
            target
        );
    }
    Ok(changed)
}

/// Query the currently active Windows power scheme GUID via `PowerGetActiveScheme`.
pub fn get_active_power_scheme() -> Result<String> {
    let mut ptr: *mut GUID = std::ptr::null_mut();
    let status = unsafe { PowerGetActiveScheme(None, &mut ptr) };
    if status != WIN32_ERROR(0) || ptr.is_null() {
        return Err(map_win32_err("PowerGetActiveScheme", status));
    }
    let guard = LocalGuidPtr(ptr);
    let guid = unsafe { *guard.0 };
    Ok(format_guid(&guid))
}

/// Resolve the target scheme GUID, defaulting to the currently active scheme when `None`.
fn resolve_scheme(scheme_guid: Option<&str>) -> Result<(GUID, String, bool)> {
    let active_str = get_active_power_scheme()?;
    match scheme_guid {
        Some(s) if !s.trim().is_empty() && !s.eq_ignore_ascii_case("SCHEME_CURRENT") => {
            let norm = normalize_guid_str(s);
            let guid = parse_guid(&norm)?;
            let is_active = norm == active_str;
            Ok((guid, norm, is_active))
        }
        _ => {
            let guid = parse_guid(&active_str)?;
            Ok((guid, active_str, true))
        }
    }
}

/// Read both AC and DC value indices for a power setting via `powrprof.dll`,
/// falling back to `PowerReadACDefaultIndex`/`PowerReadDCDefaultIndex` for the target
/// scheme before mirroring or failing.
pub fn read_power_setting_index(
    scheme_guid: Option<&str>,
    subgroup_guid: &str,
    setting_guid: &str,
) -> Result<(u32, u32)> {
    let (scheme, _, _) = resolve_scheme(scheme_guid)?;
    let subgroup = parse_guid(subgroup_guid)?;
    let setting = parse_guid(setting_guid)?;

    let mut ac_val = 0u32;
    let mut dc_val = 0u32;
    let ac_err = unsafe {
        PowerReadACValueIndex(None, Some(&scheme), Some(&subgroup), Some(&setting), &mut ac_val)
    };
    let dc_err = unsafe {
        PowerReadDCValueIndex(None, Some(&scheme), Some(&subgroup), Some(&setting), &mut dc_val)
    };

    let mut ac_ok = ac_err == WIN32_ERROR(0);
    let mut dc_ok = dc_err == 0;

    if !ac_ok {
        let def_rc = unsafe {
            PowerReadACDefaultIndex(None, &scheme, Some(&subgroup), &setting, &mut ac_val)
        };
        if def_rc == 0 {
            ac_ok = true;
        }
    }
    if !dc_ok {
        let def_rc = unsafe {
            PowerReadDCDefaultIndex(None, &scheme, Some(&subgroup), &setting, &mut dc_val)
        };
        if def_rc == 0 {
            dc_ok = true;
        }
    }

    if !ac_ok && !dc_ok {
        let primary_err = if ac_err != WIN32_ERROR(0) {
            ac_err
        } else {
            WIN32_ERROR(dc_err)
        };
        return Err(map_win32_err("PowerReadAC/DCValueIndex", primary_err));
    }
    if !ac_ok {
        ac_val = dc_val;
    } else if !dc_ok {
        dc_val = ac_val;
    }
    Ok((ac_val, dc_val))
}

/// Write AC and/or DC power setting indices and immediately commit via `PowerSetActiveScheme`
/// whenever the target scheme is currently active. Optionally clears `ATTRIB_HIDE` (`0x1`).
/// Automatically unlocks clamped `[ValueMin, ValueMax]` metadata ranges on `ERROR_INVALID_DATA (13)`.
pub fn write_power_setting_index(
    scheme_guid: Option<&str>,
    subgroup_guid: &str,
    setting_guid: &str,
    ac_value: Option<u32>,
    dc_value: Option<u32>,
    unhide_attribute: bool,
) -> Result<()> {
    let (scheme, _, is_active) = resolve_scheme(scheme_guid)?;
    let subgroup = parse_guid(subgroup_guid)?;
    let setting = parse_guid(setting_guid)?;

    if unhide_attribute {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let sub_norm = normalize_guid_str(subgroup_guid);
        let set_norm = normalize_guid_str(setting_guid);
        let path = format!(
            "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerSettings\\{}\\{}",
            sub_norm, set_norm
        );
        if let Ok(k) = hklm.open_subkey_with_flags(&path, KEY_READ | KEY_SET_VALUE | KEY_WOW64_64KEY) {
            let cur_attr: u32 = k.get_value("Attributes").unwrap_or(1);
            let cleared = cur_attr & !0x01;
            if cleared != cur_attr {
                let _ = k.set_value("Attributes", &cleared);
            }
        }
    }

    if let Some(ac) = ac_value {
        let mut rc = unsafe { PowerWriteACValueIndex(None, &scheme, Some(&subgroup), Some(&setting), ac) };
        if rc == WIN32_ERROR(13) && unlock_power_setting_range_if_clamped(&subgroup, &setting, ac)? {
            rc = unsafe { PowerWriteACValueIndex(None, &scheme, Some(&subgroup), Some(&setting), ac) };
        }
        if rc != WIN32_ERROR(0) {
            return Err(map_win32_err("PowerWriteACValueIndex", rc));
        }
    }
    if let Some(dc) = dc_value {
        let mut rc = unsafe { PowerWriteDCValueIndex(None, &scheme, Some(&subgroup), Some(&setting), dc) };
        if rc == 13 && unlock_power_setting_range_if_clamped(&subgroup, &setting, dc)? {
            rc = unsafe { PowerWriteDCValueIndex(None, &scheme, Some(&subgroup), Some(&setting), dc) };
        }
        if rc != 0 {
            return Err(map_win32_err("PowerWriteDCValueIndex", WIN32_ERROR(rc)));
        }
    }

    if is_active {
        let rc = unsafe { PowerSetActiveScheme(None, Some(&scheme)) };
        if rc != WIN32_ERROR(0) {
            return Err(map_win32_err("PowerSetActiveScheme", rc));
        }
    }

    Ok(())
}

/// Set the active Windows power plan by GUID.
pub fn set_active_power_scheme(scheme_guid: &str) -> Result<()> {
    let guid = parse_guid(scheme_guid)?;
    let rc = unsafe { PowerSetActiveScheme(None, Some(&guid)) };
    if rc != WIN32_ERROR(0) {
        return Err(map_win32_err("PowerSetActiveScheme", rc));
    }
    Ok(())
}

/// Resolve indirect MUI `@dll,-id` resource strings via Win32 `SHLoadIndirectString`,
/// falling back cleanly when a resource DLL cannot be loaded.
fn resolve_indirect_string(raw: &str) -> Option<String> {
    use windows::core::PCWSTR;
    use windows::Win32::UI::Shell::SHLoadIndirectString;

    let src: Vec<u16> = raw.encode_utf16().chain(std::iter::once(0)).collect();
    let mut out = [0u16; 256];
    if unsafe { SHLoadIndirectString(PCWSTR(src.as_ptr()), &mut out, None) }.is_ok() {
        let end = out.iter().position(|&c| c == 0).unwrap_or(out.len());
        let resolved = String::from_utf16_lossy(&out[..end]).trim().to_string();
        if !resolved.is_empty() && !resolved.starts_with('@') {
            return Some(resolved);
        }
    }
    None
}

/// Strip `@dll,-id` prefix from an unresolved MUI power plan string cleanly.
fn strip_indirect_prefix(raw_name: &str) -> String {
    let trimmed = raw_name.trim().trim_start_matches('@');
    if let Some((_dll, after_dll)) = trimmed.split_once(',') {
        let rest = after_dll.trim_start();
        if let Some((id_part, label_part)) = rest.split_once(',') {
            let id_clean = id_part.trim();
            let label_clean = label_part.trim();
            if !label_clean.is_empty()
                && id_clean.trim_start_matches('-').chars().all(|c| c.is_ascii_digit())
            {
                return label_clean.to_string();
            }
        }
    }
    trimmed.to_string()
}

/// Known stock scheme names fallback (in case `FriendlyName` contains an indirect `@` resource ref).
fn fallback_scheme_name(guid: &str, raw_name: &str) -> String {
    let norm = normalize_guid_str(guid);
    if !raw_name.is_empty() && !raw_name.starts_with('@') {
        return raw_name.to_string();
    }
    if raw_name.starts_with('@') {
        if let Some(resolved) = resolve_indirect_string(raw_name) {
            return resolved;
        }
    }
    match norm.as_str() {
        "381b4222-f694-41f0-9685-ff5bb260df2e" => "Balanced".to_string(),
        "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c" => "High Performance".to_string(),
        "a1841308-3541-4fab-bc81-f71556f20b4a" => "Power Saver".to_string(),
        "e9a42b02-d5df-448d-aa00-03f14749eb61" => "Ultimate Performance".to_string(),
        _ => strip_indirect_prefix(raw_name),
    }
}

/// Enumerate all power plans stored under `Control\Power\User\PowerSchemes` in sub-millisecond time.
pub fn list_power_schemes() -> Vec<PowerSchemeInfo> {
    let active = get_active_power_scheme().unwrap_or_default();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut plans = Vec::new();
    let base = "SYSTEM\\CurrentControlSet\\Control\\Power\\User\\PowerSchemes";
    if let Ok(root) = hklm
        .open_subkey_with_flags(base, KEY_READ | KEY_WOW64_64KEY)
        .or_else(|_| hklm.open_subkey(base))
    {
        for sub in root.enum_keys().filter_map(|k| k.ok()) {
            let norm = normalize_guid_str(&sub);
            if parse_guid(&norm).is_err() {
                continue;
            }
            let raw_name: String = root
                .open_subkey_with_flags(&sub, KEY_READ | KEY_WOW64_64KEY)
                .or_else(|_| root.open_subkey(&sub))
                .and_then(|k| k.get_value("FriendlyName"))
                .unwrap_or_else(|_| norm.clone());
            let friendly_name = fallback_scheme_name(&norm, &raw_name);
            plans.push(PowerSchemeInfo {
                is_active: norm == active,
                guid: norm,
                friendly_name,
            });
        }
    }
    plans
}

/// Create (if absent) and activate a Performance power scheme duplicated from Ultimate/High Performance,
/// applying tuned processor & NVMe/USB/PCIe latency indices while respecting Modern Standby (`AoAc`) warning.
pub fn create_and_activate_performance_plan() -> Result<String> {
    let existing = list_power_schemes();
    let scheme = if let Some(found) = existing
        .iter()
        .find(|p| p.friendly_name.eq_ignore_ascii_case("WinBlast0 Ultimate Performance"))
    {
        found.guid.clone()
    } else {
        // Prefer Ultimate Performance template, fallback High Performance, fallback current
        let src_candidates = [
            "e9a42b02-d5df-448d-aa00-03f14749eb61",
            "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c",
        ];
        let active_now = get_active_power_scheme()?;
        let mut created_guid: Option<String> = None;

        for cand in src_candidates
            .iter()
            .copied()
            .chain(std::iter::once(active_now.as_str()))
        {
            if let Ok(src) = parse_guid(cand) {
                let mut dst_ptr: *mut GUID = std::ptr::null_mut();
                let err = unsafe { PowerDuplicateScheme(None, &src, &mut dst_ptr) };
                if err == WIN32_ERROR(0) && !dst_ptr.is_null() {
                    let guard = LocalGuidPtr(dst_ptr);
                    let dst = unsafe { *guard.0 };
                    let name_utf16: Vec<u16> = "WinBlast0 Ultimate Performance\0".encode_utf16().collect();
                    let name_bytes: &[u8] = unsafe {
                        std::slice::from_raw_parts(
                            name_utf16.as_ptr() as *const u8,
                            name_utf16.len() * 2,
                        )
                    };
                    let fn_rc = unsafe {
                        PowerWriteFriendlyName(None, &dst, None, None, name_bytes)
                    };
                    if fn_rc != WIN32_ERROR(0) {
                        tracing::warn!(
                            "PowerWriteFriendlyName failed for scheme {}: Win32 error 0x{:08X}",
                            format_guid(&dst),
                            fn_rc.0
                        );
                    }
                    created_guid = Some(format_guid(&dst));
                    break;
                }
            }
        }

        created_guid.ok_or_else(|| {
            WinBlastError::Hardware(
                "Failed to duplicate a base power plan via PowerDuplicateScheme".to_string(),
            )
        })?
    };

    // Tune key settings even when the plan already existed (ignore unsupported hardware GUIDs cleanly)
    #[allow(clippy::type_complexity)]
    let tuned_settings: &[(&str, &str, Option<u32>, Option<u32>, bool)] = &[
        // Min cores AC/DC = 100 (Class 0)
        ("54533251-82be-4824-96c1-47b60b740d00", "0cc5b647-c1df-4637-891a-dec35c318583", Some(100), Some(100), true),
        // Max cores AC/DC = 100 (Class 0)
        ("54533251-82be-4824-96c1-47b60b740d00", "ea062031-0e34-4ff1-9b6d-eb1059334028", Some(100), Some(100), true),
        // Min cores AC/DC = 100 (Class 1 / P-core on Intel hybrid)
        ("54533251-82be-4824-96c1-47b60b740d00", "0cc5b647-c1df-4637-891a-dec35c318584", Some(100), Some(100), true),
        // Max cores AC/DC = 100 (Class 1 / P-core on Intel hybrid)
        ("54533251-82be-4824-96c1-47b60b740d00", "ea062031-0e34-4ff1-9b6d-eb1059334029", Some(100), Some(100), true),
        // Perf increase policy = Rocket (2)
        ("54533251-82be-4824-96c1-47b60b740d00", "465e1f50-b610-473a-ab58-00d1077dc418", Some(2), Some(2), true),
        // USB selective suspend off AC
        ("2a737441-1930-4402-8d77-b2bebba308a3", "48e6b7a6-50f5-4782-a5d4-53bb8f07e226", Some(0), None, true),
        // PCIe ASPM off AC
        ("501a4d13-42af-4429-9fd1-a8218c268e20", "ee12f906-d277-404b-b6da-e5fa1a576df5", Some(0), None, false),
    ];

    for &(sub, set, ac, dc, unhide) in tuned_settings {
        if let Err(err) = write_power_setting_index(Some(&scheme), sub, set, ac, dc, unhide) {
            if !crate::core::backend::is_unsupported_power_guid_error(&err) {
                tracing::warn!(
                    "Failed to apply tuned setting {}/{} on scheme {}: {}",
                    sub,
                    set,
                    scheme,
                    err
                );
            }
        }
    }

    set_active_power_scheme(&scheme)?;
    Ok(scheme)
}

/// Detect Hibernation state (`hiberfil.sys` + Fast Startup) and Modern Standby (`AoAc` / `S0ix`).
pub fn query_hibernation_and_standby() -> HibernationStatus {
    let mut caps = SYSTEM_POWER_CAPABILITIES::default();
    let status: NTSTATUS = unsafe {
        CallNtPowerInformation(
            SystemPowerCapabilities,
            None,
            0,
            Some((&mut caps as *mut SYSTEM_POWER_CAPABILITIES).cast()),
            std::mem::size_of::<SYSTEM_POWER_CAPABILITIES>() as u32,
        )
    };

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hib_reg = hklm
        .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Power", KEY_READ | KEY_WOW64_64KEY)
        .and_then(|k| k.get_value::<u32, _>("HibernateEnabled"))
        .unwrap_or(0)
        == 1;

    let fast_startup_enabled = hklm
        .open_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power",
            KEY_READ | KEY_WOW64_64KEY,
        )
        .and_then(|k| k.get_value::<u32, _>("HiberbootEnabled"))
        .unwrap_or(0)
        == 1;

    if status.0 == 0 {
        HibernationStatus {
            hibernation_enabled: hib_reg || caps.HiberFilePresent,
            hiberfile_present: caps.HiberFilePresent,
            fast_startup_enabled,
            modern_standby_s0ix: caps.AoAc,
        }
    } else {
        let cs_enabled = hklm
            .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Power", KEY_READ | KEY_WOW64_64KEY)
            .and_then(|k| k.get_value::<u32, _>("CsEnabled"))
            .unwrap_or(0)
            == 1;
        HibernationStatus {
            hibernation_enabled: hib_reg,
            hiberfile_present: hib_reg,
            fast_startup_enabled,
            modern_standby_s0ix: cs_enabled,
        }
    }
}

/// Reserve or release `hiberfil.sys` natively via `CallNtPowerInformation(SystemReserveHiberFile)`.
pub fn set_hibernation_enabled(enable: bool) -> Result<HibernationStatus> {
    let mut reserve: u8 = if enable { 1 } else { 0 };
    let status: NTSTATUS = unsafe {
        CallNtPowerInformation(
            SystemReserveHiberFile,
            Some((&mut reserve as *mut u8).cast()),
            std::mem::size_of::<u8>() as u32,
            None,
            0,
        )
    };
    if status.0 != 0 {
        return Err(WinBlastError::Hardware(format!(
            "CallNtPowerInformation(SystemReserveHiberFile) failed with NTSTATUS 0x{:08X}",
            status.0 as u32
        )));
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok((k, _)) = hklm.create_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\Power",
        KEY_READ | KEY_SET_VALUE | KEY_WOW64_64KEY,
    ) {
        let _ = k.set_value("HibernateEnabled", &(if enable { 1u32 } else { 0u32 }));
    }
    if !enable {
        if let Ok((k, _)) = hklm.create_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power",
            KEY_READ | KEY_SET_VALUE | KEY_WOW64_64KEY,
        ) {
            let _ = k.set_value("HiberbootEnabled", &0u32);
        }
    }

    Ok(query_hibernation_and_standby())
}

/// Configure USB Root Hub selective suspend & D3 idle settings across enumerated `Enum\USB\ROOT_HUB*` nodes.
pub fn set_usb_root_hub_power_saving(disable_power_saving: bool) -> Result<usize> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let usb_root = hklm
        .open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Enum\\USB", KEY_READ | KEY_WOW64_64KEY)
        .map_err(WinBlastError::Registry)?;

    let mut updated = 0usize;
    let target_val: u32 = if disable_power_saving { 0 } else { 1 };

    for dev_name in usb_root.enum_keys().filter_map(|k| k.ok()) {
        if !dev_name.to_ascii_uppercase().contains("ROOT_HUB") {
            continue;
        }
        if let Ok(dev_key) = usb_root.open_subkey_with_flags(&dev_name, KEY_READ | KEY_WOW64_64KEY) {
            for inst_name in dev_key.enum_keys().filter_map(|k| k.ok()) {
                let param_path = format!("{}\\Device Parameters", inst_name);
                if let Ok(pk) =
                    dev_key.open_subkey_with_flags(&param_path, KEY_READ | KEY_SET_VALUE | KEY_WOW64_64KEY)
                {
                    let ok1 = pk.set_value("EnhancedPowerManagementEnabled", &target_val).is_ok();
                    let ok2 = pk.set_value("AllowIdleIrpInD3", &target_val).is_ok();
                    let ok3 = pk.set_value("DeviceIdleEnabled", &target_val).is_ok();
                    if ok1 || ok2 || ok3 {
                        updated += 1;
                    }
                }
            }
        }
    }

    // Also align active scheme USB selective suspend setting
    if let Err(err) = write_power_setting_index(
        None,
        "2a737441-1930-4402-8d77-b2bebba308a3",
        "48e6b7a6-50f5-4782-a5d4-53bb8f07e226",
        Some(if disable_power_saving { 0 } else { 1 }),
        None,
        true,
    ) {
        if !crate::core::backend::is_unsupported_power_guid_error(&err) {
            tracing::warn!("Failed to align USB selective suspend index: {}", err);
        }
    }

    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_roundtrip_and_normalization() {
        let g = parse_guid("{54533251-82BE-4824-96C1-47B60B740D00}").unwrap();
        assert_eq!(format_guid(&g), "54533251-82be-4824-96c1-47b60b740d00");
        assert_eq!(normalize_guid_str(" {381B4222-F694-41F0-9685-FF5BB260DF2E} "), "381b4222-f694-41f0-9685-ff5bb260df2e");
        assert!(parse_guid("not-a-guid").is_err());
    }

    #[test]
    fn test_power_scheme_enumeration_nonblocking() {
        let active = get_active_power_scheme();
        assert!(active.is_ok(), "PowerGetActiveScheme should work on Windows");
        let schemes = list_power_schemes();
        // In sandboxed or restricted execution environments, HKLM PowerSchemes subkeys may be unreadable;
        // when present, each entry must have valid GUID and non-empty friendly name.
        for s in &schemes {
            assert!(!s.guid.is_empty());
            assert!(!s.friendly_name.is_empty());
        }
    }

    #[test]
    fn test_ac_power_status_and_enumerated_range_noop() {
        // Must never panic on desktop or laptop
        let _on_ac = is_on_ac_power();

        // Enumerated setting (IDLEDISABLE) has no ValueMin/ValueMax -> must return Ok(false)
        let sub = parse_guid("54533251-82be-4824-96c1-47b60b740d00").unwrap();
        let idle_disable = parse_guid("5d76a2ca-e8c0-402f-a133-2158492d58ad").unwrap();
        let unlocked = unlock_power_setting_range_if_clamped(&sub, &idle_disable, 1).unwrap();
        assert!(!unlocked, "Enumerated setting must not attempt ValueMin/ValueMax write");

        // Reading CPMINCORES on live host succeeds, while a bogus GUID fails cleanly
        assert!(read_power_setting_index(
            None,
            "54533251-82be-4824-96c1-47b60b740d00",
            "0cc5b647-c1df-4637-891a-dec35c318583",
        )
        .is_ok());
        assert!(read_power_setting_index(
            None,
            "54533251-82be-4824-96c1-47b60b740d00",
            "00000000-0000-0000-0000-000000000000",
        )
        .is_err());
    }

    #[test]
    fn test_fallback_scheme_name_indirect_stripping() {
        assert_eq!(
            fallback_scheme_name("00000000-0000-0000-0000-000000000001", "@nonexistent_oem.dll,-42,OEM Turbo Plan"),
            "OEM Turbo Plan"
        );
        assert_eq!(
            fallback_scheme_name("381b4222-f694-41f0-9685-ff5bb260df2e", "@nonexistent.dll,-10"),
            "Balanced"
        );
        assert_eq!(
            fallback_scheme_name("00000000-0000-0000-0000-000000000002", "Custom Plan"),
            "Custom Plan"
        );
    }
}
