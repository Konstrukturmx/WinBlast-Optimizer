//! Service Control Manager state auditor.
//!
//! State-of-the-art registry inspection of Windows services start types and states.

use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

use crate::scanner::registry_scanner::open_subkey_read_64;

fn start_type_name(st: u32) -> &'static str {
    match st {
        0 => "Boot",
        1 => "System",
        2 => "Automatic",
        3 => "Manual",
        4 => "Disabled",
        _ => "Unknown",
    }
}

/// Pure evaluation helper for a service start type value with structured detail kind.
pub fn evaluate_service_value_with_kind(
    current: Option<u32>,
    target_start: u32,
) -> (bool, String, crate::scanner::ScanDetailKind) {
    use crate::scanner::ScanDetailKind;

    match current {
        Some(val) if val == target_start => (
            true,
            format!("Optimized ({})", start_type_name(val)),
            ScanDetailKind::ServiceOptimized { mode: val },
        ),
        Some(val) => (
            false,
            format!(
                "Current: {} (Target: {})",
                start_type_name(val),
                start_type_name(target_start)
            ),
            ScanDetailKind::ServiceCurrent {
                current: val,
                target: target_start,
            },
        ),
        None => {
            if target_start == 3 || target_start == 4 {
                (
                    true,
                    "Service not installed".to_string(),
                    ScanDetailKind::ServiceNotInstalled,
                )
            } else {
                (
                    false,
                    "Service absent from system".to_string(),
                    ScanDetailKind::ServiceAbsent,
                )
            }
        }
    }
}

/// Pure evaluation helper for a service start type value.
pub fn evaluate_service_value(current: Option<u32>, target_start: u32) -> (bool, String) {
    let (is_opt, msg, _) = evaluate_service_value_with_kind(current, target_start);
    (is_opt, msg)
}

/// State-of-the-art service evaluation against live Windows Registry.
pub fn evaluate_service(
    service_name: &str,
    target_start: u32,
    _default_start: u32,
) -> (bool, String) {
    evaluate_service_value(get_service_start_type(service_name), target_start)
}

/// Check if a service's start type matches the expected value.
#[allow(dead_code)]
pub fn check_service_start_type(service_name: &str, expected_start_type: u32) -> bool {
    evaluate_service(service_name, expected_start_type, 2).0
}

/// Read the current start type of a service from the registry.
pub fn get_service_start_type(service_name: &str) -> Option<u32> {
    let clean = service_name.trim();
    if clean.is_empty() || clean.contains(['\\', '/', '\0']) {
        return None;
    }
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SYSTEM\\CurrentControlSet\\Services\\{}", clean);
    let key = open_subkey_read_64(&hklm, &path).ok()?;
    key.get_value::<u32, _>("Start").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_absent_optional_service() {
        assert_eq!(
            evaluate_service_value(None, 3),
            (true, "Service not installed".to_string())
        );
        assert_eq!(
            evaluate_service_value(None, 4),
            (true, "Service not installed".to_string())
        );
        assert!(!evaluate_service_value(None, 2).0);
    }

    #[test]
    fn test_evaluate_service_value_with_kind_all_branches() {
        use crate::scanner::ScanDetailKind;

        assert_eq!(
            evaluate_service_value_with_kind(Some(4), 4),
            (
                true,
                "Optimized (Disabled)".to_string(),
                ScanDetailKind::ServiceOptimized { mode: 4 },
            )
        );
        assert_eq!(
            evaluate_service_value_with_kind(Some(2), 4),
            (
                false,
                "Current: Automatic (Target: Disabled)".to_string(),
                ScanDetailKind::ServiceCurrent {
                    current: 2,
                    target: 4,
                },
            )
        );
        assert_eq!(
            evaluate_service_value_with_kind(None, 4),
            (
                true,
                "Service not installed".to_string(),
                ScanDetailKind::ServiceNotInstalled,
            )
        );
        assert_eq!(
            evaluate_service_value_with_kind(None, 2),
            (
                false,
                "Service absent from system".to_string(),
                ScanDetailKind::ServiceAbsent,
            )
        );
    }
}
