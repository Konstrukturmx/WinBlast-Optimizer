//! Registry mutation helper (WOW64 64-bit view aware + automatic Windows REG_SZ preservation).

use winreg::enums::*;
use winreg::RegKey;

pub use crate::core::reg_helpers::is_known_numeric_reg_sz;
use crate::core::error::{Result, WinBlastError};
use crate::scanner::registry_scanner::{
    create_subkey_64, open_subkey_flags_64, resolve_hive, resolve_target_paths,
};

const REG_DELETE_RIGHT: u32 = 0x0001_0000;

fn write_dword_or_numeric_sz(
    key: &RegKey,
    resolved_path: &str,
    name: &str,
    value: u32,
) -> Result<()> {
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    // Preserve numeric REG_SZ / REG_EXPAND_SZ format when existing or when Windows mandates REG_SZ
    let raw_opt = key.get_raw_value(norm_name).ok();
    let (existing_numeric_sz, existing_is_expand, existing_was_negative) = match &raw_opt {
        Some(raw) if raw.vtype == REG_SZ || raw.vtype == REG_EXPAND_SZ => {
            let s = winreg::types::FromRegValue::from_reg_value(raw)
                .map(|v: String| v)
                .unwrap_or_default();
            let t = s.trim();
            let is_u32 = t.parse::<u32>().is_ok();
            let neg_i32 = !is_u32 && t.parse::<i32>().is_ok_and(|n| n < 0);
            (
                is_u32 || t.parse::<i32>().is_ok(),
                raw.vtype == REG_EXPAND_SZ,
                neg_i32,
            )
        }
        _ => (false, false, false),
    };
    let known_sz = is_known_numeric_reg_sz(resolved_path, norm_name);
    let write_as_string = existing_numeric_sz || known_sz;

    if write_as_string {
        let str_val = if (existing_was_negative || known_sz) && (value as i32) < 0 {
            (value as i32).to_string()
        } else {
            value.to_string()
        };
        if existing_is_expand {
            let bytes: Vec<u8> = str_val
                .encode_utf16()
                .chain(std::iter::once(0))
                .flat_map(|w| w.to_le_bytes())
                .collect();
            let raw = winreg::RegValue {
                bytes,
                vtype: REG_EXPAND_SZ,
            };
            key.set_raw_value(norm_name, &raw)
                .map_err(WinBlastError::from_reg_err)?;
        } else {
            key.set_value(norm_name, &str_val)
                .map_err(WinBlastError::from_reg_err)?;
        }
    } else {
        key.set_value(norm_name, &value)
            .map_err(WinBlastError::from_reg_err)?;
    }
    Ok(())
}

/// Set a DWORD (or numeric REG_SZ) value in the registry, creating subkeys with 64-bit view
/// and resolving active `<NET_INTERFACE_GUID>` network adapters.
pub fn set_dword(hive: &str, path: &str, name: &str, value: u32) -> Result<()> {
    set_dword_for_vendor(hive, path, name, value, None)
}

/// Set a string value in the registry.
pub fn set_string(hive: &str, path: &str, name: &str, value: &str) -> Result<()> {
    set_string_for_vendor(hive, path, name, value, false, None)
}

/// Set a `REG_SZ` or `REG_EXPAND_SZ` string value using `(Default)` normalization & vendor GPU routing.
pub fn set_string_for_vendor(
    hive: &str,
    path: &str,
    name: &str,
    value: &str,
    expand_sz: bool,
    gpu_vendor: Option<&str>,
) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, false)?;
    let root = resolve_hive(hive)?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let (is_net, resolved_paths) = resolve_target_paths(hive, path, gpu_vendor, false)?;
    if is_net && resolved_paths.is_empty() {
        return Err(WinBlastError::Hardware(
            "No active TCP/IP network interface found for <NET_INTERFACE_GUID>".to_string(),
        ));
    }
    for resolved_path in resolved_paths {
        let key = create_subkey_64(&root, &resolved_path).map_err(WinBlastError::from_reg_err)?;

        if expand_sz {
            let mut bytes: Vec<u8> = value
                .encode_utf16()
                .chain(std::iter::once(0))
                .flat_map(|w| w.to_le_bytes())
                .collect();
            if bytes.is_empty() {
                bytes = vec![0, 0];
            }
            let raw = winreg::RegValue {
                bytes,
                vtype: REG_EXPAND_SZ,
            };
            key.set_raw_value(norm_name, &raw)
                .map_err(WinBlastError::from_reg_err)?;
        } else {
            key.set_value(norm_name, &value)
                .map_err(WinBlastError::from_reg_err)?;
        }
    }
    Ok(())
}

/// Set a DWORD registry value using explicit GPU silicon vendor routing and active `<NET_INTERFACE_GUID>` resolution.
pub fn set_dword_for_vendor(
    hive: &str,
    path: &str,
    name: &str,
    value: u32,
    gpu_vendor: Option<&str>,
) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, false)?;
    let root = resolve_hive(hive)?;
    let (is_net, resolved_paths) = resolve_target_paths(hive, path, gpu_vendor, false)?;
    if is_net && resolved_paths.is_empty() {
        return Err(WinBlastError::Hardware(
            "No active TCP/IP network interface found for <NET_INTERFACE_GUID>".to_string(),
        ));
    }
    for resolved_path in resolved_paths {
        let key = create_subkey_64(&root, &resolved_path).map_err(WinBlastError::from_reg_err)?;
        write_dword_or_numeric_sz(&key, &resolved_path, name, value)?;
    }
    Ok(())
}

/// Delete a value from a registry key (64-bit view aware, with `(Default)` normalization & GPU routing).
pub fn delete_value(hive: &str, path: &str, name: &str) -> Result<()> {
    delete_value_for_vendor(hive, path, name, None)
}

/// Delete a value using explicit GPU vendor class routing and `(Default)` normalization.
pub fn delete_value_for_vendor(
    hive: &str,
    path: &str,
    name: &str,
    gpu_vendor: Option<&str>,
) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, false)?;
    let root = resolve_hive(hive)?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let (_, resolved_paths) = resolve_target_paths(hive, path, gpu_vendor, true)?;
    for resolved_path in resolved_paths {
        let key = match open_subkey_flags_64(&root, &resolved_path, KEY_SET_VALUE) {
            Ok(k) => k,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(WinBlastError::from_reg_err(e)),
        };
        match key.delete_value(norm_name) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(WinBlastError::from_reg_err(e)),
        }
    }
    Ok(())
}

/// Write a `REG_BINARY` value cleanly.
pub fn set_binary(hive: &str, path: &str, name: &str, bytes: &[u8]) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, false)?;
    let root = resolve_hive(hive)?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let key = create_subkey_64(&root, path).map_err(WinBlastError::from_reg_err)?;
    let raw = winreg::RegValue {
        bytes: bytes.to_vec(),
        vtype: REG_BINARY,
    };
    key.set_raw_value(norm_name, &raw)
        .map_err(WinBlastError::from_reg_err)?;
    Ok(())
}

fn u32_to_reg_type(code: u32) -> winreg::enums::RegType {
    use winreg::enums::RegType::*;
    match code {
        0 => REG_NONE,
        1 => REG_SZ,
        2 => REG_EXPAND_SZ,
        3 => REG_BINARY,
        4 => REG_DWORD,
        5 => REG_DWORD_BIG_ENDIAN,
        6 => REG_LINK,
        7 => REG_MULTI_SZ,
        8 => REG_RESOURCE_LIST,
        9 => REG_FULL_RESOURCE_DESCRIPTOR,
        10 => REG_RESOURCE_REQUIREMENTS_LIST,
        11 => REG_QWORD,
        _ => REG_BINARY,
    }
}

/// Restore a lossless [`RegRawSnapshot`] with exact original `REG_*` type and raw bytes.
pub fn set_raw(
    hive: &str,
    path: &str,
    name: &str,
    snap: &crate::core::reg_helpers::RegRawSnapshot,
) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, false)?;
    let root = resolve_hive(hive)?;
    let norm_name = crate::core::risk::normalize_reg_value_name(name);
    let key = create_subkey_64(&root, path).map_err(WinBlastError::from_reg_err)?;
    let bytes = crate::core::reg_helpers::decode_hex(&snap.hex_bytes)?;
    let raw = winreg::RegValue {
        bytes,
        vtype: u32_to_reg_type(snap.reg_type),
    };
    key.set_raw_value(norm_name, &raw)
        .map_err(WinBlastError::from_reg_err)?;
    Ok(())
}

/// Delete a registry subkey tree safely using Win32 `RegDeleteTreeW` on a 64-bit view parent handle,
/// refusing shallow paths or `"HKCR"` targets. Idempotent on `NotFound`.
pub fn delete_tree(hive: &str, path: &str) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, true)?;
    let clean = path.trim().trim_matches(['\\', '/']);
    let (parent_path, leaf_name) = clean.rsplit_once('\\').ok_or_else(|| {
        WinBlastError::TweakFailed(format!("Invalid tree path '{}'", path))
    })?;
    let root = resolve_hive(hive)?;
    let parent_key = match open_subkey_flags_64(
        &root,
        parent_path,
        KEY_READ | KEY_WRITE | REG_DELETE_RIGHT,
    ) {
        Ok(k) => k,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(WinBlastError::from_reg_err(e)),
    };

    let leaf_w: Vec<u16> = leaf_name.encode_utf16().chain(std::iter::once(0)).collect();
    let status = unsafe {
        windows::Win32::System::Registry::RegDeleteTreeW(
            windows::Win32::System::Registry::HKEY(parent_key.raw_handle() as _),
            windows::core::PCWSTR(leaf_w.as_ptr()),
        )
    };
    if status.0 == 0 || status.0 == 2 || status.0 == 3 {
        Ok(())
    } else {
        Err(WinBlastError::registry(
            status.0 as i32,
            format!("RegDeleteTreeW failed with Win32 error {}", status.0),
        ))
    }
}

fn restore_tree_node(
    parent_key: &RegKey,
    path: &str,
    node: &crate::core::reg_helpers::RegTreeSnapshot,
) -> Result<()> {
    let k = create_subkey_64(parent_key, path).map_err(WinBlastError::from_reg_err)?;
    for (vname, snap) in &node.values {
        let bytes = crate::core::reg_helpers::decode_hex(&snap.hex_bytes)?;
        let raw = winreg::RegValue {
            bytes,
            vtype: u32_to_reg_type(snap.reg_type),
        };
        k.set_raw_value(vname, &raw)
            .map_err(WinBlastError::from_reg_err)?;
    }
    for child in &node.children {
        restore_tree_node(&k, &child.subkey, child)?;
    }
    Ok(())
}

/// Losslessly restore a previously deleted registry subtree from [`RegTreeSnapshot`].
pub fn restore_tree(
    hive: &str,
    path: &str,
    snapshot: &crate::core::reg_helpers::RegTreeSnapshot,
) -> Result<()> {
    crate::core::reg_helpers::validate_registry_mutation_path(hive, path, true)?;
    let root = resolve_hive(hive)?;
    restore_tree_node(&root, path, snapshot)
}
