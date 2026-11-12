//! Registry value scanner — multi-hive, 64-bit Wow64-aware registry auditor.

use winreg::enums::*;
use winreg::RegKey;

use crate::core::error::{Result, WinBlastError};

const NET_INTERFACES_LOWER: &str =
    "system\\currentcontrolset\\services\\tcpip\\parameters\\interfaces";
const NET_PLACEHOLDER_LOWER: &str = "<net_interface_guid>";
const DISPLAY_CLASS_LOWER: &str =
    "system\\currentcontrolset\\control\\class\\{4d36e968-e325-11ce-bfc1-08002be10318}";

/// Open a registry subkey with explicit flags + `KEY_WOW64_64KEY`, falling back without
/// `KEY_WOW64_64KEY` ONLY on `ERROR_INVALID_PARAMETER` (87).
pub(crate) fn open_subkey_flags_64(root: &RegKey, path: &str, flags: u32) -> std::io::Result<RegKey> {
    match root.open_subkey_with_flags(path, flags | KEY_WOW64_64KEY) {
        Ok(k) => Ok(k),
        Err(e) if e.raw_os_error() == Some(87) => root.open_subkey_with_flags(path, flags),
        Err(e) => Err(e),
    }
}

/// Open a registry subkey read-only in the 64-bit registry view (never silently redirects absent keys).
pub(crate) fn open_subkey_read_64(root: &RegKey, path: &str) -> std::io::Result<RegKey> {
    open_subkey_flags_64(root, path, KEY_READ)
}

/// Create or open a registry subkey with read/write in the 64-bit registry view, falling back
/// without `KEY_WOW64_64KEY` ONLY on `ERROR_INVALID_PARAMETER` (87).
pub(crate) fn create_subkey_64(root: &RegKey, path: &str) -> std::io::Result<RegKey> {
    match root.create_subkey_with_flags(path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY) {
        Ok((k, _)) => Ok(k),
        Err(e) if e.raw_os_error() == Some(87) => root.create_subkey(path).map(|(k, _)| k),
        Err(e) => Err(e),
    }
}

/// Replace an ASCII case-insensitive placeholder inside a registry path.
pub(crate) fn replace_placeholder_ci(path: &str, placeholder: &str, replacement: &str) -> String {
    let lower = path.to_ascii_lowercase();
    let needle = placeholder.to_ascii_lowercase();
    if let Some(idx) = lower.find(&needle) {
        format!("{}{}{}", &path[..idx], replacement, &path[idx + needle.len()..])
    } else {
        path.to_string()
    }
}

fn has_active_ip(key: &RegKey, value_name: &str) -> bool {
    let is_valid = |s: &str| {
        let t = s.trim();
        !t.is_empty() && t != "0.0.0.0" && t != "::"
    };
    if let Ok(multi) = key.get_value::<Vec<String>, _>(value_name) {
        return multi.iter().any(|s| is_valid(s));
    }
    if let Ok(single) = key.get_value::<String, _>(value_name) {
        return is_valid(&single);
    }
    false
}

/// Resolve `<NET_INTERFACE_GUID>` to active TCP/IP interface subkeys (having `IPAddress` or `DhcpIPAddress`).
pub fn resolve_net_interface_paths(hive: &str, path: &str, include_inactive_for_delete: bool) -> (bool, Vec<String>) {
    let is_hklm = hive.trim().eq_ignore_ascii_case("HKLM")
        || hive.trim().eq_ignore_ascii_case("HKEY_LOCAL_MACHINE");
    let lower = path.trim().to_ascii_lowercase();
    if !is_hklm || !lower.contains(NET_PLACEHOLDER_LOWER) {
        return (false, vec![path.to_string()]);
    }

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut active = Vec::new();
    let mut all_guids = Vec::new();

    if let Ok(ifaces_key) = open_subkey_read_64(&hklm, NET_INTERFACES_LOWER) {
        for sub_name in ifaces_key.enum_keys().filter_map(|k| k.ok()) {
            let resolved = replace_placeholder_ci(path, "<NET_INTERFACE_GUID>", &sub_name);
            if sub_name.starts_with('{') && sub_name.ends_with('}') {
                all_guids.push(resolved.clone());
            }
            if let Ok(sub_key) = open_subkey_read_64(&ifaces_key, &sub_name) {
                if has_active_ip(&sub_key, "IPAddress") || has_active_ip(&sub_key, "DhcpIPAddress") {
                    active.push(resolved);
                }
            }
        }
    }

    if !active.is_empty() {
        (true, active)
    } else if include_inactive_for_delete {
        (true, all_guids)
    } else {
        (true, Vec::new())
    }
}

/// Resolve dynamic `<NET_INTERFACE_GUID>` and multi-GPU Display Class registry paths,
/// failing explicitly if a vendor-targeted GPU tweak has no matching GPU adapter installed.
pub(crate) fn resolve_target_paths(
    hive: &str,
    path: &str,
    gpu_vendor: Option<&str>,
    delete_mode: bool,
) -> Result<(bool, Vec<String>)> {
    let (is_net, net_paths) = resolve_net_interface_paths(hive, path, delete_mode);
    if is_net {
        return Ok((true, net_paths));
    }

    let is_hklm = hive.trim().eq_ignore_ascii_case("HKLM")
        || hive.trim().eq_ignore_ascii_case("HKEY_LOCAL_MACHINE");
    let lower = path.trim().to_ascii_lowercase();
    if is_hklm && lower.starts_with(DISPLAY_CLASS_LOWER) {
        if let Some(wanted) = crate::hardware::gpu::parse_target_vendor(gpu_vendor) {
            let gpus = crate::hardware::gpu::detect_gpus();
            if !gpus
                .iter()
                .any(|g| g.vendor == wanted && !g.class_subkey.is_empty())
            {
                return Err(WinBlastError::Hardware(format!(
                    "No {} GPU adapter detected on this system",
                    wanted.as_str()
                )));
            }
        }
        return Ok((
            false,
            crate::hardware::gpu::resolve_gpu_class_path_for_vendor(hive, path, gpu_vendor),
        ));
    }

    Ok((false, vec![path.to_string()]))
}

/// Resolve hive string to predefined RegKey.
pub fn resolve_hive(hive: &str) -> Result<RegKey> {
    match hive.trim().to_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => Ok(RegKey::predef(HKEY_LOCAL_MACHINE)),
        "HKCU" | "HKEY_CURRENT_USER" => Ok(RegKey::predef(HKEY_CURRENT_USER)),
        "HKU" | "HKEY_USERS" => Ok(RegKey::predef(HKEY_USERS)),
        "HKCR" | "HKEY_CLASSES_ROOT" => Ok(RegKey::predef(HKEY_CLASSES_ROOT)),
        _ => Err(WinBlastError::TweakFailed(format!(
            "Unknown registry hive: {}",
            hive
        ))),
    }
}

/// Check if a registry DWORD value matches the expected optimized value.
#[allow(dead_code)]
pub fn check_dword(hive: &str, path: &str, name: &str, expected: u32) -> bool {
    let (is_opt, _) = evaluate_dword(hive, path, name, expected, 0);
    is_opt
}

/// Shared pure evaluation helper for DWORD-coercible registry values with structured detail kind.
///
/// Returns `(is_optimized, status_description, detail_kind)`.
pub fn evaluate_dword_value_with_kind(
    current: Option<u32>,
    expected_opt: u32,
    default_val: u32,
) -> (bool, String, crate::scanner::ScanDetailKind) {
    use crate::scanner::ScanDetailKind;

    match current {
        Some(val) if val == expected_opt => (
            true,
            format!("Optimized (value: {})", val),
            ScanDetailKind::DwordOptimized { value: val },
        ),
        Some(val) if val == default_val => (
            false,
            format!("Default (value: {})", val),
            ScanDetailKind::DwordDefault { value: val },
        ),
        Some(val) => (
            false,
            format!("Custom (value: {}, expected: {})", val, expected_opt),
            ScanDetailKind::DwordCustom {
                value: val,
                expected: expected_opt,
            },
        ),
        None => {
            if expected_opt == default_val {
                (
                    true,
                    "Default (not configured)".to_string(),
                    ScanDetailKind::DwordDefaultNotConfigured,
                )
            } else {
                (
                    false,
                    format!(
                        "Not configured (default: {}, expected: {})",
                        default_val, expected_opt
                    ),
                    ScanDetailKind::DwordNotConfigured {
                        default_val,
                        expected: expected_opt,
                    },
                )
            }
        }
    }
}

/// Shared pure evaluation helper for DWORD-coercible registry values.
///
/// Returns `(is_optimized, status_description)`.
pub fn evaluate_dword_value(
    current: Option<u32>,
    expected_opt: u32,
    default_val: u32,
) -> (bool, String) {
    let (is_opt, msg, _) = evaluate_dword_value_with_kind(current, expected_opt, default_val);
    (is_opt, msg)
}

/// State-of-the-art registry evaluation against the live Windows registry.
pub fn evaluate_dword(
    hive: &str,
    path: &str,
    name: &str,
    expected_opt: u32,
    default_val: u32,
) -> (bool, String) {
    evaluate_dword_value(get_dword(hive, path, name), expected_opt, default_val)
}

fn read_dword_from_key(key: &RegKey, name: &str) -> Option<u32> {
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let raw = key.get_raw_value(norm_name).ok()?;
    match raw.vtype {
        RegType::REG_DWORD => {
            if raw.bytes.len() >= 4 {
                Some(u32::from_le_bytes([
                    raw.bytes[0],
                    raw.bytes[1],
                    raw.bytes[2],
                    raw.bytes[3],
                ]))
            } else {
                None
            }
        }
        RegType::REG_DWORD_BIG_ENDIAN => {
            if raw.bytes.len() >= 4 {
                Some(u32::from_be_bytes([
                    raw.bytes[0],
                    raw.bytes[1],
                    raw.bytes[2],
                    raw.bytes[3],
                ]))
            } else {
                None
            }
        }
        RegType::REG_QWORD => {
            if raw.bytes.len() >= 8 {
                let val = u64::from_le_bytes([
                    raw.bytes[0],
                    raw.bytes[1],
                    raw.bytes[2],
                    raw.bytes[3],
                    raw.bytes[4],
                    raw.bytes[5],
                    raw.bytes[6],
                    raw.bytes[7],
                ]);
                if val <= u32::MAX as u64 {
                    Some(val as u32)
                } else {
                    None
                }
            } else {
                None
            }
        }
        RegType::REG_SZ | RegType::REG_EXPAND_SZ => {
            let s: String = winreg::types::FromRegValue::from_reg_value(&raw).ok()?;
            let trimmed = s.trim();
            if let Ok(u) = trimmed.parse::<u32>() {
                Some(u)
            } else if let Ok(i) = trimmed.parse::<i32>() {
                Some(i as u32)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Read a DWORD value from the live registry, handling Wow64 redirection, QWORD, numeric REG_SZ,
/// and active `<NET_INTERFACE_GUID>` network adapters.
pub fn get_dword(hive: &str, path: &str, name: &str) -> Option<u32> {
    get_dword_for_vendor(hive, path, name, None)
}

/// Read a string value from the registry.
#[allow(dead_code)]
pub fn get_string(hive: &str, path: &str, name: &str) -> Option<String> {
    get_string_for_vendor(hive, path, name, None).map(|v| v.value)
}

/// Read a string (`REG_SZ` or `REG_EXPAND_SZ`) registry value using explicit vendor GPU routing and `(Default)` normalization.
pub fn get_string_for_vendor(
    hive: &str,
    path: &str,
    name: &str,
    gpu_vendor: Option<&str>,
) -> Option<crate::core::reg_helpers::RegStringValue> {
    let root = resolve_hive(hive).ok()?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let (_, resolved_paths) = resolve_target_paths(hive, path, gpu_vendor, false).ok()?;
    for resolved_path in resolved_paths {
        if let Ok(k) = open_subkey_read_64(&root, &resolved_path) {
            if let Ok(raw) = k.get_raw_value(norm_name) {
                if raw.vtype == winreg::enums::REG_SZ || raw.vtype == winreg::enums::REG_EXPAND_SZ {
                    let expand_sz = raw.vtype == winreg::enums::REG_EXPAND_SZ;
                    let s = winreg::types::FromRegValue::from_reg_value(&raw).ok()?;
                    return Some(crate::core::reg_helpers::RegStringValue { value: s, expand_sz });
                }
            }
        }
    }
    None
}

/// Read a DWORD registry value using explicit GPU silicon vendor targeting and active `<NET_INTERFACE_GUID>` resolution.
pub fn get_dword_for_vendor(
    hive: &str,
    path: &str,
    name: &str,
    gpu_vendor: Option<&str>,
) -> Option<u32> {
    let root = resolve_hive(hive).ok()?;
    let (_, resolved_paths) = resolve_target_paths(hive, path, gpu_vendor, false).ok()?;
    if resolved_paths.is_empty() {
        return None;
    }

    let mut consensus: Option<u32> = None;
    for resolved_path in &resolved_paths {
        let val = open_subkey_read_64(&root, resolved_path)
            .ok()
            .and_then(|k| read_dword_from_key(&k, name))?;
        match consensus {
            None => consensus = Some(val),
            Some(prev) if prev == val => {}
            Some(_) => return None,
        }
    }
    consensus
}

/// Read a raw `REG_BINARY` value cleanly.
pub fn get_binary(hive: &str, path: &str, name: &str) -> Option<Vec<u8>> {
    let root = resolve_hive(hive).ok()?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let key = open_subkey_read_64(&root, path).ok()?;
    let raw = key.get_raw_value(norm_name).ok()?;
    if raw.vtype == RegType::REG_BINARY {
        Some(raw.bytes)
    } else {
        None
    }
}

/// Check whether a registry subkey exists.
pub fn key_exists(hive: &str, path: &str) -> bool {
    let Ok(root) = resolve_hive(hive) else {
        return false;
    };
    open_subkey_read_64(&root, path).is_ok()
}

fn reg_type_to_u32(rt: &winreg::enums::RegType) -> u32 {
    use winreg::enums::RegType::*;
    match rt {
        REG_NONE => 0,
        REG_SZ => 1,
        REG_EXPAND_SZ => 2,
        REG_BINARY => 3,
        REG_DWORD => 4,
        REG_DWORD_BIG_ENDIAN => 5,
        REG_LINK => 6,
        REG_MULTI_SZ => 7,
        REG_RESOURCE_LIST => 8,
        REG_FULL_RESOURCE_DESCRIPTOR => 9,
        REG_RESOURCE_REQUIREMENTS_LIST => 10,
        REG_QWORD => 11,
    }
}

/// Read lossless raw value bytes + `REG_*` type code.
pub fn get_raw(hive: &str, path: &str, name: &str) -> Option<crate::core::reg_helpers::RegRawSnapshot> {
    let root = resolve_hive(hive).ok()?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let key = open_subkey_read_64(&root, path).ok()?;
    let raw = key.get_raw_value(norm_name).ok()?;
    Some(crate::core::reg_helpers::RegRawSnapshot {
        reg_type: reg_type_to_u32(&raw.vtype),
        hex_bytes: crate::core::reg_helpers::encode_hex(&raw.bytes),
    })
}

fn snapshot_key_recursive(
    key: &RegKey,
    subkey_name: &str,
    depth: usize,
) -> Result<crate::core::reg_helpers::RegTreeSnapshot> {
    if depth > 32 {
        return Err(WinBlastError::Registry(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Exceeded maximum registry tree snapshot depth (32)",
            ),
        ));
    }
    let mut values = Vec::new();
    for (vname, rval) in key.enum_values().filter_map(|v| v.ok()) {
        values.push((
            vname,
            crate::core::reg_helpers::RegRawSnapshot {
                reg_type: reg_type_to_u32(&rval.vtype),
                hex_bytes: crate::core::reg_helpers::encode_hex(&rval.bytes),
            },
        ));
    }
    let mut children = Vec::new();
    for cname in key.enum_keys().filter_map(|k| k.ok()) {
        if let Ok(ck) = open_subkey_read_64(key, &cname) {
            children.push(snapshot_key_recursive(&ck, &cname, depth + 1)?);
        }
    }
    Ok(crate::core::reg_helpers::RegTreeSnapshot {
        subkey: subkey_name.to_string(),
        values,
        children,
    })
}

/// Capture an entire registry subtree losslessly before `RegistryDeleteTree`.
pub fn snapshot_tree(
    hive: &str,
    path: &str,
) -> Result<Option<crate::core::reg_helpers::RegTreeSnapshot>> {
    let root = resolve_hive(hive)?;
    let key = match open_subkey_read_64(&root, path) {
        Ok(k) => k,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(WinBlastError::Registry(e)),
    };
    Ok(Some(snapshot_key_recursive(&key, "", 0)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    struct ScratchCleanup(String);
    impl Drop for ScratchCleanup {
        fn drop(&mut self) {
            let _ = RegKey::predef(HKEY_CURRENT_USER).delete_subkey_all(&self.0);
        }
    }

    #[test]
    fn test_strict_reg_type_dispatch_and_negative_numeric_sz() {
        let subkey = format!("Software\\WinBlast0_TypeTest_{}", Uuid::new_v4());
        let _cleanup = ScratchCleanup(subkey.clone());
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (key, _) = hkcu.create_subkey(&subkey).unwrap();

        // Non-numeric REG_SZ >= 4 UTF-16LE bytes must NEVER corrupt into u32
        key.set_value("NonNumeric", &"auto").unwrap();
        assert_eq!(get_dword("HKCU", &subkey, "NonNumeric"), None);
        assert_eq!(get_binary("HKCU", &subkey, "NonNumeric"), None);

        // Negative i32 stored as REG_SZ parses cleanly and preserves REG_SZ + sign on update
        key.set_value("SignedSz", &"-15").unwrap();
        assert_eq!(get_dword("HKCU", &subkey, "SignedSz"), Some((-15i32) as u32));
        crate::executor::reg_mutator::set_dword("HKCU", &subkey, "SignedSz", (-60i32) as u32).unwrap();
        assert_eq!(key.get_value::<String, _>("SignedSz").unwrap(), "-60");

        // REG_QWORD within u32 vs overflowing u32
        key.set_raw_value(
            "SmallQword",
            &winreg::RegValue {
                bytes: 42u64.to_le_bytes().to_vec(),
                vtype: RegType::REG_QWORD,
            },
        )
        .unwrap();
        key.set_raw_value(
            "HugeQword",
            &winreg::RegValue {
                bytes: (u32::MAX as u64 + 1).to_le_bytes().to_vec(),
                vtype: RegType::REG_QWORD,
            },
        )
        .unwrap();
        assert_eq!(get_dword("HKCU", &subkey, "SmallQword"), Some(42));
        assert_eq!(get_dword("HKCU", &subkey, "HugeQword"), None);

        // REG_DWORD_BIG_ENDIAN
        key.set_raw_value(
            "BigEndianDword",
            &winreg::RegValue {
                bytes: 0x12345678u32.to_be_bytes().to_vec(),
                vtype: RegType::REG_DWORD_BIG_ENDIAN,
            },
        )
        .unwrap();
        assert_eq!(get_dword("HKCU", &subkey, "BigEndianDword"), Some(0x12345678));

        // get_binary enforces REG_BINARY strictly
        key.set_value("DwordNotBin", &123u32).unwrap();
        assert_eq!(get_binary("HKCU", &subkey, "DwordNotBin"), None);
        crate::executor::reg_mutator::set_binary("HKCU", &subkey, "RealBin", &[1, 2, 3]).unwrap();
        assert_eq!(get_binary("HKCU", &subkey, "RealBin"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_evaluate_dword_value_with_kind_all_branches() {
        use crate::scanner::ScanDetailKind;

        assert_eq!(
            evaluate_dword_value_with_kind(Some(1), 1, 0),
            (
                true,
                "Optimized (value: 1)".to_string(),
                ScanDetailKind::DwordOptimized { value: 1 },
            )
        );
        assert_eq!(
            evaluate_dword_value_with_kind(Some(0), 1, 0),
            (
                false,
                "Default (value: 0)".to_string(),
                ScanDetailKind::DwordDefault { value: 0 },
            )
        );
        assert_eq!(
            evaluate_dword_value_with_kind(Some(42), 1, 0),
            (
                false,
                "Custom (value: 42, expected: 1)".to_string(),
                ScanDetailKind::DwordCustom {
                    value: 42,
                    expected: 1,
                },
            )
        );
        assert_eq!(
            evaluate_dword_value_with_kind(None, 0, 0),
            (
                true,
                "Default (not configured)".to_string(),
                ScanDetailKind::DwordDefaultNotConfigured,
            )
        );
        assert_eq!(
            evaluate_dword_value_with_kind(None, 1, 0),
            (
                false,
                "Not configured (default: 0, expected: 1)".to_string(),
                ScanDetailKind::DwordNotConfigured {
                    default_val: 0,
                    expected: 1,
                },
            )
        );
        assert_eq!(
            evaluate_dword_value(Some(42), 1, 0),
            (false, "Custom (value: 42, expected: 1)".to_string())
        );
    }
}
