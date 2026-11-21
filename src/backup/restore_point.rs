//! WMI SystemRestore checkpoint creation & trigger.
//!
//! Pairs Windows System Restore Points (via VSS / WMI Checkpoint-Computer)
//! to provide OS-level disaster recovery before applying tweaks.

use serde::Deserialize;

/// System restore point details for display in Backup Center.
#[derive(Debug, Clone, Deserialize)]
pub struct RestorePoint {
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: u32,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "CreationTime")]
    pub creation_time: String,
}

/// Create a System Restore point with the given description.
pub fn create_restore_point(description: &str) -> crate::core::error::Result<()> {
    let script = format!(
        "Checkpoint-Computer -Description '{}' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction Stop",
        description.replace('\'', "''")
    );

    let output = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| {
            crate::core::error::WinBlastError::Backup(format!("Failed to run PowerShell: {}", e))
        })?;

    if !output.status.success() {
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&output.stderr);
        let stdout = crate::scanner::task_scanner::decode_oem_bytes(&output.stdout);
        let detail = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        return Err(crate::core::error::WinBlastError::Backup(format!(
            "Failed to create restore point: {}",
            detail
        )));
    }

    tracing::info!(
        "System Restore point '{}' created successfully",
        description
    );
    Ok(())
}

/// List all available system restore points on the machine.
pub fn list_restore_points() -> crate::core::error::Result<Vec<RestorePoint>> {
    let script = "Get-ComputerRestorePoint | Select-Object SequenceNumber, Description, @{N='CreationTime';E={$_.ConvertToDateTime($_.CreationTime).ToString('yyyy-MM-dd HH:mm:ss')}} | ConvertTo-Json";

    let output = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| {
            crate::core::error::WinBlastError::Backup(format!("Failed to run PowerShell: {}", e))
        })?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = crate::scanner::task_scanner::decode_oem_bytes(&output.stdout);
    let trimmed = stdout.trim().trim_start_matches('\u{FEFF}');
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    if trimmed.starts_with('[') {
        let list: Vec<RestorePoint> = serde_json::from_str(trimmed)
            .map_err(crate::core::error::WinBlastError::Serialization)?;
        Ok(list)
    } else {
        let point: RestorePoint = serde_json::from_str(trimmed)
            .map_err(crate::core::error::WinBlastError::Serialization)?;
        Ok(vec![point])
    }
}

/// Check if any system restore point exists that was created within `max_age`.
pub fn is_recent_restore_point_present_with(
    backend: &dyn crate::core::backend::SystemBackend,
    max_age: std::time::Duration,
) -> crate::core::error::Result<bool> {
    let points = backend.list_restore_points()?;
    if points.is_empty() {
        return Ok(false);
    }

    let now_utc = chrono::Utc::now();
    let max_age_chrono = chrono::Duration::from_std(max_age).unwrap_or(chrono::Duration::hours(24));

    for point in &points {
        if let Some(created_at) = parse_restore_point_time(&point.creation_time) {
            let diff = now_utc.signed_duration_since(created_at);
            // Allow small negative clock skew (-5 minutes) and verify within max_age
            if diff >= -chrono::Duration::minutes(5) && diff <= max_age_chrono {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Helper to parse various datetime formats from WMI / PowerShell / SandboxEnv
pub fn parse_restore_point_time(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }

    // 1. Try RFC3339 (e.g. 2026-11-17T14:30:00Z)
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(trimmed) {
        return Some(dt.with_timezone(&chrono::Utc));
    }

    // 2. Try "%Y-%m-%d %H:%M:%S"
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S") {
        return Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            naive,
            chrono::Utc,
        ));
    }

    // 3. Try "%Y-%m-%dT%H:%M:%S"
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S") {
        return Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
            naive,
            chrono::Utc,
        ));
    }

    // 4. Try WMI DMTF format: "yyyymmddHHMMSS.xxxxxx+UUU" (e.g. "20261117143000.000000+000")
    if trimmed.len() >= 14 && trimmed.chars().take(14).all(|c| c.is_ascii_digit()) {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&trimmed[..14], "%Y%m%d%H%M%S") {
            return Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                naive,
                chrono::Utc,
            ));
        }
    }

    None
}

/// Ensure an automated pre-flight system restore point exists within the last 24 hours.
/// Returns `Ok(false)` if a restore point already exists within 24 hours (skipping creation).
/// Returns `Ok(true)` if a new restore point was created.
pub fn ensure_preflight_restore_point_with(
    backend: &dyn crate::core::backend::SystemBackend,
    description: &str,
) -> crate::core::error::Result<bool> {
    backend.ensure_preflight_restore_point(description)
}

/// Automated pre-flight VSS restore point hook using the live `RealBackend`.
pub fn ensure_preflight_restore_point(description: &str) -> crate::core::error::Result<bool> {
    ensure_preflight_restore_point_with(&crate::core::backend::RealBackend, description)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::backend::{SandboxEnv, SystemBackend};

    #[test]
    fn test_parse_restore_point_time_formats() {
        assert!(parse_restore_point_time("2026-11-17 14:30:00").is_some());
        assert!(parse_restore_point_time("2026-11-17T14:30:00Z").is_some());
        assert!(parse_restore_point_time("20261117143000.000000+000").is_some());
        assert!(parse_restore_point_time("invalid date").is_none());
        assert!(parse_restore_point_time("").is_none());
    }

    #[test]
    fn test_ensure_preflight_restore_point_sandbox() {
        let sandbox = SandboxEnv::default();
        let initial_count = sandbox.list_restore_points().unwrap().len();

        // 1. Old restore point is >24h old -> preflight creates a new one
        let created = ensure_preflight_restore_point_with(&sandbox, "Pre-optimization test")
            .expect("Preflight failed");
        assert!(
            created,
            "Expected restore point to be created when no recent point exists"
        );
        assert_eq!(
            sandbox.list_restore_points().unwrap().len(),
            initial_count + 1
        );

        // 2. Second invocation within 24h -> preflight skips creation
        let created_again =
            ensure_preflight_restore_point_with(&sandbox, "Pre-optimization test 2")
                .expect("Preflight failed");
        assert!(
            !created_again,
            "Expected preflight to skip creation within 24h"
        );
        assert_eq!(
            sandbox.list_restore_points().unwrap().len(),
            initial_count + 1
        );
    }
}
