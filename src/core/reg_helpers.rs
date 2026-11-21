//! Shared lossless Registry value representations, hex encoding, safe subtree
//! path depth guards, and `CompositeStringKey` (`;`-delimited `Key=Val`) helpers.

use serde::{Deserialize, Serialize};

use crate::core::error::{Result, WinBlastError};
use crate::core::risk::normalize_reg_value_name;

/// String registry value with explicit `REG_SZ` vs `REG_EXPAND_SZ` kind tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegStringValue {
    pub value: String,
    pub expand_sz: bool,
}

/// Lossless binary snapshot of any Windows registry value (`REG_*` type + lowercase hex bytes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegRawSnapshot {
    pub reg_type: u32,
    pub hex_bytes: String,
}

/// Recursive registry key tree snapshot used for lossless `RegistryDeleteTree` rollback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegTreeSnapshot {
    pub subkey: String,
    pub values: Vec<(String, RegRawSnapshot)>,
    pub children: Vec<RegTreeSnapshot>,
}

/// Encode raw bytes as compact lowercase hexadecimal.
pub fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0F) as usize] as char);
    }
    out
}

/// Decode lowercase/uppercase hexadecimal bytes, rejecting odd lengths or invalid nibbles.
pub fn decode_hex(s: &str) -> Result<Vec<u8>> {
    let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if !clean.len().is_multiple_of(2) {
        return Err(WinBlastError::Serialization(serde_json::Error::io(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Odd hex byte length"),
        )));
    }
    let mut out = Vec::with_capacity(clean.len() / 2);
    for i in (0..clean.len()).step_by(2) {
        let byte = u8::from_str_radix(&clean[i..i + 2], 16).map_err(|_| {
            WinBlastError::TweakFailed(format!("Invalid hex byte '{}'", &clean[i..i + 2]))
        })?;
        out.push(byte);
    }
    Ok(out)
}

/// Guard `RegistryDeleteTree` and `RegistryDelete` against shallow/root/HKCR disasters.
///
/// Rejects empty/whitespace paths, `..`, NULs, `"HKCR"` writes (merged elevation trap), and shallow/critical OS roots.
pub fn validate_registry_mutation_path(hive: &str, path: &str, is_tree_delete: bool) -> Result<()> {
    let h_clean = hive.trim().to_ascii_uppercase();
    if h_clean == "HKCR" || h_clean == "HKEY_CLASSES_ROOT" {
        return Err(WinBlastError::TweakFailed(format!(
            "Refused unsafe HKCR target '{}': use explicit HKCU\\Software\\Classes or HKLM\\SOFTWARE\\Classes",
            path
        )));
    }
    let trimmed_path = path.trim().trim_matches(['\\', '/']);
    if trimmed_path.is_empty()
        || path.contains('\0')
        || path.split(['\\', '/']).any(|s| s.trim() == "..")
    {
        return Err(WinBlastError::TweakFailed(format!(
            "Invalid registry subkey path: '{}'",
            path
        )));
    }
    if is_tree_delete {
        let segments: Vec<&str> = trimmed_path
            .split(['\\', '/'])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if segments.len() < 3 {
            return Err(WinBlastError::TweakFailed(format!(
                "Refused shallow RegistryDeleteTree on '{}': minimum depth is 3 segments",
                path
            )));
        }
        let lower_first2 = format!(
            "{}\\{}",
            segments[0].to_ascii_lowercase(),
            segments[1].to_ascii_lowercase()
        );
        let protected_roots = [
            "software\\microsoft",
            "software\\policies",
            "system\\currentcontrolset",
        ];
        if protected_roots.contains(&lower_first2.as_str()) && segments.len() < 4 {
            return Err(WinBlastError::TweakFailed(format!(
                "Refused shallow RegistryDeleteTree under OS protected branch '{}'",
                path
            )));
        }
        let lower_first3 = format!("{}\\{}", lower_first2, segments[2].to_ascii_lowercase());
        let critical_3seg_branches = [
            "software\\microsoft\\windows",
            "software\\microsoft\\windows nt",
            "system\\currentcontrolset\\services",
            "system\\currentcontrolset\\control",
        ];
        if critical_3seg_branches.contains(&lower_first3.as_str()) && segments.len() < 5 {
            return Err(WinBlastError::TweakFailed(format!(
                "Refused shallow RegistryDeleteTree under critical OS branch '{}'",
                path
            )));
        }
        if segments.len() >= 4 {
            let lower_first4 = format!("{}\\{}", lower_first3, segments[3].to_ascii_lowercase());
            if lower_first4 == "software\\policies\\microsoft\\windows" && segments.len() < 5 {
                return Err(WinBlastError::TweakFailed(format!(
                    "Refused shallow RegistryDeleteTree under critical policy branch '{}'",
                    path
                )));
            }
        }
        if segments.len() >= 5 {
            let lower_first5 = format!(
                "{}\\{}\\{}",
                lower_first3,
                segments[3].to_ascii_lowercase(),
                segments[4].to_ascii_lowercase()
            );
            if lower_first5 == "software\\microsoft\\windows\\currentversion\\policies"
                && segments.len() < 6
            {
                return Err(WinBlastError::TweakFailed(format!(
                    "Refused shallow RegistryDeleteTree on critical policy root '{}'",
                    path
                )));
            }
        }
    }
    Ok(())
}

/// Known Windows registry settings stored as decimal `REG_SZ` strings rather than `REG_DWORD`.
///
/// Ensures fresh creation matches native Windows subsystem expectations across Win10/Win11 Build 26300.
pub fn is_known_numeric_reg_sz(path: &str, name: &str) -> bool {
    let p = path.trim().trim_matches('\\').to_ascii_lowercase();
    let n = name.trim().to_ascii_lowercase();
    (p == "control panel\\desktop"
        && matches!(
            n.as_str(),
            "menushowdelay"
                | "autoendtasks"
                | "waittokillapptimeout"
                | "hungapptimeout"
                | "lowlevelhookstimeout"
                | "dragfullwindows"
                | "fontsmoothing"
                | "screensaveactive"
                | "wheelscrolllines"
                | "wheelscrollchars"
        ))
        || (p == "control panel\\desktop\\windowmetrics"
            && matches!(
                n.as_str(),
                "minanimate" | "borderwidth" | "paddedborderwidth"
            ))
        || (p == "control panel\\keyboard"
            && matches!(
                n.as_str(),
                "keyboarddelay" | "keyboardspeed" | "initialkeyboardindicators"
            ))
        || (p == "control panel\\mouse"
            && matches!(
                n.as_str(),
                "mousespeed"
                    | "mousethreshold1"
                    | "mousethreshold2"
                    | "mousehovertime"
                    | "mousesensitivity"
                    | "doubleclickspeed"
                    | "mousetrails"
                    | "snaptodefaultbutton"
                    | "swapmousebuttons"
            ))
        || (p.starts_with("control panel\\accessibility\\")
            && matches!(
                n.as_str(),
                "flags" | "autorepeatdelay" | "autorepeatrate" | "bouncetime"
            ))
}

/// Parse delimited composite key-value pairs (e.g. `DirectXUserGlobalSettings`).
pub fn parse_composite_pairs(
    raw: &str,
    pair_sep: &str,
    kv_sep: &str,
) -> Vec<(String, Option<String>)> {
    if pair_sep.is_empty() || kv_sep.is_empty() {
        return Vec::new();
    }
    raw.split(pair_sep)
        .filter_map(|seg| {
            let trimmed = seg.trim();
            if trimmed.is_empty() {
                return None;
            }
            if let Some((k, v)) = trimmed.split_once(kv_sep) {
                Some((k.trim().to_string(), Some(v.trim().to_string())))
            } else {
                Some((trimmed.to_string(), None))
            }
        })
        .collect()
}

/// Read a single key's value from a composite string (case-insensitive key match).
pub fn read_composite_entry(
    raw: &str,
    entry_key: &str,
    pair_sep: &str,
    kv_sep: &str,
) -> Option<String> {
    parse_composite_pairs(raw, pair_sep, kv_sep)
        .into_iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(entry_key))
        .and_then(|(_, v)| v)
}

/// Merge (update, append, or remove via `"__ABSENT__"`) a single key inside a composite string
/// without disturbing unrelated flags/tokens. Deduplicates stale duplicate occurrences of `entry_key`.
/// Returns `(updated_string, prior_entry_value)`.
pub fn merge_composite_pair(
    raw: Option<&str>,
    entry_key: &str,
    target_val: &str,
    pair_sep: &str,
    kv_sep: &str,
    trailing_sep: bool,
) -> (String, Option<String>) {
    let pairs = raw
        .map(|s| parse_composite_pairs(s, pair_sep, kv_sep))
        .unwrap_or_default();

    let mut prev: Option<String> = None;
    let mut seen_match = false;
    let mut new_pairs = Vec::with_capacity(pairs.len() + 1);

    for (k, v) in pairs {
        if k.eq_ignore_ascii_case(entry_key) {
            if !seen_match {
                seen_match = true;
                prev = v;
                if target_val != "__ABSENT__" {
                    new_pairs.push((k, Some(target_val.to_string())));
                }
            }
        } else {
            new_pairs.push((k, v));
        }
    }

    if !seen_match && target_val != "__ABSENT__" {
        new_pairs.push((entry_key.to_string(), Some(target_val.to_string())));
    }

    if new_pairs.is_empty() {
        return (String::new(), prev);
    }

    let mut joined = new_pairs
        .into_iter()
        .map(|(k, v)| match v {
            Some(val) => format!("{}{}{}", k, kv_sep, val),
            None => k,
        })
        .collect::<Vec<_>>()
        .join(pair_sep);

    if trailing_sep && !joined.ends_with(pair_sep) {
        joined.push_str(pair_sep);
    }
    (joined, prev)
}

/// Pipe-escaped target formatter (`%`, `|`, and `#` escaped).
pub fn escape_pipe(s: &str) -> String {
    s.replace('%', "%25")
        .replace('|', "%7C")
        .replace('#', "%23")
}

/// Inverse of [`escape_pipe`].
pub fn unescape_pipe(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            match &bytes[i..i + 3] {
                b"%7C" | b"%7c" => {
                    out.push('|');
                    i += 3;
                    continue;
                }
                b"%25" => {
                    out.push('%');
                    i += 3;
                    continue;
                }
                b"%23" => {
                    out.push('#');
                    i += 3;
                    continue;
                }
                _ => {}
            }
        }
        if let Some(ch) = s.get(i..).and_then(|sub| sub.chars().next()) {
            out.push(ch);
            i += ch.len_utf8();
        } else {
            break;
        }
    }
    out
}

/// Format unambiguous pipe-delimited registry target `(hive, path, name)`.
pub fn format_pipe_reg_target(hive: &str, path: &str, name: &str) -> String {
    let disp = crate::core::risk::display_reg_value_name(name);
    format!(
        "{}\\{}|{}",
        hive.trim(),
        escape_pipe(path),
        escape_pipe(disp)
    )
}

/// Parse unambiguous pipe-delimited registry target into `(hive, path, normalized_name)`.
pub fn parse_pipe_reg_target(target: &str) -> Result<(String, String, String)> {
    let (left, right) = target.split_once('|').ok_or_else(|| {
        WinBlastError::Backup(format!("Malformed pipe registry target: {}", target))
    })?;
    let (hive, esc_path) = left.split_once('\\').ok_or_else(|| {
        WinBlastError::Backup(format!("Missing hive separator in target: {}", target))
    })?;
    let path = unescape_pipe(esc_path);
    let raw_name = unescape_pipe(right);
    let name = normalize_reg_value_name(&raw_name).to_string();
    Ok((hive.to_string(), path, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_string_merge_preserves_siblings() {
        let initial = "VRROptimizeEnable=0;OtherCustomFlag=42;";
        let (after_swap, prev1) = merge_composite_pair(
            Some(initial),
            "SwapEffectUpgradeEnable",
            "1",
            ";",
            "=",
            true,
        );
        assert_eq!(prev1, None);
        assert_eq!(
            after_swap,
            "VRROptimizeEnable=0;OtherCustomFlag=42;SwapEffectUpgradeEnable=1;"
        );

        let (after_vrr, prev2) =
            merge_composite_pair(Some(&after_swap), "vrroptimizeenable", "1", ";", "=", true);
        assert_eq!(prev2.as_deref(), Some("0"));
        assert_eq!(
            read_composite_entry(&after_vrr, "VRROptimizeEnable", ";", "=").as_deref(),
            Some("1")
        );
        assert_eq!(
            read_composite_entry(&after_vrr, "OtherCustomFlag", ";", "=").as_deref(),
            Some("42")
        );

        // Removing one token keeps siblings
        let (after_del, _) = merge_composite_pair(
            Some(&after_vrr),
            "SwapEffectUpgradeEnable",
            "__ABSENT__",
            ";",
            "=",
            true,
        );
        assert_eq!(after_del, "VRROptimizeEnable=1;OtherCustomFlag=42;");

        // Deduplicates stale duplicate keys cleanly
        let dup_raw = "SwapEffectUpgradeEnable=0;Other=1;swapeffectupgradeenable=2;";
        let (deduped, prev_dup) = merge_composite_pair(
            Some(dup_raw),
            "SwapEffectUpgradeEnable",
            "1",
            ";",
            "=",
            true,
        );
        assert_eq!(prev_dup.as_deref(), Some("0"));
        assert_eq!(deduped, "SwapEffectUpgradeEnable=1;Other=1;");
    }

    #[test]
    fn test_pipe_escape_handles_default_and_backslashes() {
        let tgt = format_pipe_reg_target("HKCU", "Software\\Classes\\CLSID\\{test}", "(Default)");
        let (h, p, n) = parse_pipe_reg_target(&tgt).unwrap();
        assert_eq!(h, "HKCU");
        assert_eq!(p, "Software\\Classes\\CLSID\\{test}");
        assert_eq!(n, "");

        let tgt2 =
            format_pipe_reg_target("HKLM", "SOFTWARE\\Test#Hash", "C:\\Path|Pipe#2\\app.exe");
        assert!(!tgt2.contains('#'));
        let (h2, p2, n2) = parse_pipe_reg_target(&tgt2).unwrap();
        assert_eq!(h2, "HKLM");
        assert_eq!(p2, "SOFTWARE\\Test#Hash");
        assert_eq!(n2, "C:\\Path|Pipe#2\\app.exe");

        let tgt3 = format_pipe_reg_target("HKCU", "Software\\Test", "@");
        let (_, _, n3) = parse_pipe_reg_target(&tgt3).unwrap();
        assert_eq!(n3, "");
    }

    #[test]
    fn test_hkcr_and_shallow_tree_delete_blocked() {
        assert!(validate_registry_mutation_path("HKCR", "*\\shell\\Test", false).is_err());
        assert!(validate_registry_mutation_path("HKCU", "   ", false).is_err());
        assert!(validate_registry_mutation_path("HKCU", "\\\\\\\\", false).is_err());
        assert!(
            validate_registry_mutation_path("HKCU", "Software\\Microsoft\\Windows", true).is_err()
        );
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Services\\Tcpip",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Control",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SYSTEM\\CurrentControlSet\\Control\\Session Manager",
            true
        )
        .is_err());
        assert!(validate_registry_mutation_path(
            "HKCU",
            "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            true
        )
        .is_ok());
        assert!(validate_registry_mutation_path(
            "HKLM",
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\SampleApp",
            true
        )
        .is_ok());
    }

    #[test]
    fn test_known_numeric_reg_sz() {
        assert!(is_known_numeric_reg_sz(
            " Control Panel\\Desktop\\ ",
            "MenuShowDelay"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Desktop",
            "LowLevelHooksTimeout"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Desktop\\WindowMetrics",
            "PaddedBorderWidth"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Keyboard",
            "InitialKeyboardIndicators"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Mouse",
            "DoubleClickSpeed"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Accessibility\\StickyKeys",
            "Flags"
        ));
        assert!(is_known_numeric_reg_sz(
            "Control Panel\\Accessibility\\Keyboard Response",
            "AutoRepeatRate"
        ));
        assert!(!is_known_numeric_reg_sz("Software\\Test", "MenuShowDelay"));
    }
}
