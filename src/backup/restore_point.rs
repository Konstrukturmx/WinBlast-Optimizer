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
        .map_err(|e| crate::core::error::WinBlastError::Backup(format!("Failed to run PowerShell: {}", e)))?;

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

    tracing::info!("System Restore point '{}' created successfully", description);
    Ok(())
}

/// List all available system restore points on the machine.
pub fn list_restore_points() -> crate::core::error::Result<Vec<RestorePoint>> {
    let script = "Get-ComputerRestorePoint | Select-Object SequenceNumber, Description, @{N='CreationTime';E={$_.ConvertToDateTime($_.CreationTime).ToString('yyyy-MM-dd HH:mm:ss')}} | ConvertTo-Json";

    let output = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| crate::core::error::WinBlastError::Backup(format!("Failed to run PowerShell: {}", e)))?;

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
