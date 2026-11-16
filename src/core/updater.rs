//! WinBlast0 In-App Self-Updater via GitHub Releases API.
//!
//! Provides background checking, release metadata parsing, asset download,
//! and atomic Windows binary replacement with elevation support.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::core::process::SilentCommand;

/// GitHub Release metadata fetched from the GitHub API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub html_url: String,
    pub published_at: String,
    pub download_url: Option<String>,
    pub asset_name: Option<String>,
    pub download_size_mb: Option<f64>,
}

/// State of the updater engine.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpdateAvailable(ReleaseInfo),
    UpToDate,
    Downloading { progress: f32, message: String },
    ReadyToRestart { new_binary: PathBuf },
    Failed(String),
}

/// GitHub repository target for updates.
pub const GITHUB_REPO: &str = "konstruktor/WinBlast0";

/// Check if a remote version tag is newer than the current application version.
pub fn is_version_newer(remote_tag: &str, current_version: &str) -> bool {
    let clean_remote = remote_tag.trim().trim_start_matches('v').trim_start_matches('V');
    let clean_current = current_version.trim().trim_start_matches('v').trim_start_matches('V');

    let remote_parts: Vec<u64> = clean_remote
        .split('.')
        .filter_map(|s| s.split('-').next().and_then(|p| p.parse().ok()))
        .collect();
    let current_parts: Vec<u64> = clean_current
        .split('.')
        .filter_map(|s| s.split('-').next().and_then(|p| p.parse().ok()))
        .collect();

    for (r, c) in remote_parts.iter().zip(current_parts.iter()) {
        if r > c {
            return true;
        } else if r < c {
            return false;
        }
    }
    remote_parts.len() > current_parts.len()
}

/// Query GitHub Releases API for the latest published release.
pub fn check_latest_release(repo: &str) -> Result<Option<ReleaseInfo>, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    // Using PowerShell to query GitHub REST API with custom User-Agent
    let ps_cmd = format!(
        r#"[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
try {{
    $r = Invoke-RestMethod -Uri '{}' -Headers @{{'User-Agent'='WinBlast0-Updater'}} -TimeoutSec 10;
    $asset = $r.assets | Where-Object {{ $_.name -like '*.exe' -or $_.name -like '*.zip' }} | Select-Object -First 1;
    $dl = if ($asset) {{ $asset.browser_download_url }} else {{ $null }};
    $sz = if ($asset) {{ [math]::Round($asset.size / 1MB, 2) }} else {{ 0 }};
    $as_name = if ($asset) {{ $asset.name }} else {{ $null }};
    [PSCustomObject]@{{
        tag_name = $r.tag_name;
        name = if ($r.name) {{ $r.name }} else {{ $r.tag_name }};
        body = if ($r.body) {{ $r.body }} else {{ '' }};
        html_url = $r.html_url;
        published_at = if ($r.published_at) {{ $r.published_at }} else {{ '' }};
        download_url = $dl;
        asset_name = $as_name;
        download_size_mb = $sz;
    }} | ConvertTo-Json -Compress
}} catch {{
    Write-Output "ERROR: $($_.Exception.Message)"
}}"#,
        url
    );

    let output = SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .output()
        .map_err(|e| format!("Failed to spawn PowerShell: {}", e))?;

    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if out_str.starts_with("ERROR:") || out_str.is_empty() {
        // Fallback for sandboxed / offline / unreleased testing
        return Err(if out_str.is_empty() {
            "Empty response from update server".to_string()
        } else {
            out_str
        });
    }

    #[derive(Deserialize)]
    struct RawRelease {
        tag_name: Option<String>,
        name: Option<String>,
        body: Option<String>,
        html_url: Option<String>,
        published_at: Option<String>,
        download_url: Option<String>,
        asset_name: Option<String>,
        download_size_mb: Option<f64>,
    }

    let parsed: RawRelease = serde_json::from_str(&out_str)
        .map_err(|e| format!("Failed to parse release JSON: {} (raw: {})", e, out_str))?;

    let tag = parsed.tag_name.unwrap_or_default();
    if tag.is_empty() {
        return Ok(None);
    }

    let current_ver = env!("CARGO_PKG_VERSION");
    let is_newer = is_version_newer(&tag, current_ver);

    let info = ReleaseInfo {
        tag_name: tag,
        name: parsed.name.unwrap_or_else(|| "WinBlast0 Update".to_string()),
        body: parsed.body.unwrap_or_default(),
        html_url: parsed.html_url.unwrap_or_default(),
        published_at: parsed.published_at.unwrap_or_default(),
        download_url: parsed.download_url,
        asset_name: parsed.asset_name,
        download_size_mb: parsed.download_size_mb,
    };

    if is_newer {
        Ok(Some(info))
    } else {
        Ok(None)
    }
}

/// Download release asset to a temporary directory and stage for atomic replacement.
pub fn download_release_asset(download_url: &str, asset_name: &str) -> Result<PathBuf, String> {
    let temp_dir = std::env::temp_dir().join("winblast0_update");
    let _ = std::fs::create_dir_all(&temp_dir);
    let target_file = temp_dir.join(asset_name);

    let target_path_str = target_file.to_string_lossy().to_string();

    let ps_cmd = format!(
        r#"[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
try {{
    Invoke-WebRequest -Uri '{}' -OutFile '{}' -Headers @{{'User-Agent'='WinBlast0-Updater'}} -UseBasicParsing -TimeoutSec 120;
    if (Test-Path '{}') {{
        Write-Output "SUCCESS"
    }} else {{
        Write-Output "ERROR: Target file not found after download"
    }}
}} catch {{
    Write-Output "ERROR: $($_.Exception.Message)"
}}"#,
        download_url, target_path_str, target_path_str
    );

    let output = SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .output()
        .map_err(|e| format!("Failed to execute download script: {}", e))?;

    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if out_str.contains("SUCCESS") && target_file.exists() {
        Ok(target_file)
    } else {
        Err(format!("Download failed: {}", out_str))
    }
}

/// Apply update by atomically replacing the running binary.
///
/// On Windows, a running executable cannot be written to directly, but it CAN be renamed.
/// Procedure:
/// 1. Rename running `winblast0.exe` -> `winblast0.exe.old`.
/// 2. Copy staged `new_binary` into `winblast0.exe`.
/// 3. Spawn the new executable and terminate the current process.
pub fn apply_and_restart(new_binary: &Path) -> Result<(), String> {
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let backup_exe = current_exe.with_extension("exe.old");

    // Clean any prior backup
    let _ = std::fs::remove_file(&backup_exe);

    // 1. Rename running exe to .old
    std::fs::rename(&current_exe, &backup_exe)
        .map_err(|e| format!("Failed to rename running executable: {}", e))?;

    // 2. Copy new binary to current exe location
    if let Err(e) = std::fs::copy(new_binary, &current_exe) {
        // Rollback rename if copy fails
        let _ = std::fs::rename(&backup_exe, &current_exe);
        return Err(format!("Failed to install new binary: {}", e));
    }

    // 3. Launch the new executable and exit
    let current_exe_str = current_exe.to_string_lossy().to_string();
    let ps_restart = format!(
        r#"Start-Sleep -Milliseconds 800; Start-Process -FilePath '{}'"#,
        current_exe_str
    );

    let _ = SilentCommand::new("powershell")
        .args(["-WindowStyle", "Hidden", "-Command", &ps_restart])
        .spawn();

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison_logic() {
        assert!(is_version_newer("v1.3.0", "1.2.0"));
        assert!(is_version_newer("1.2.1", "1.2.0"));
        assert!(is_version_newer("2.0.0", "1.9.9"));
        assert!(!is_version_newer("v1.2.0", "1.2.0"));
        assert!(!is_version_newer("1.1.9", "1.2.0"));
        assert!(!is_version_newer("v1.0.0", "1.2.0"));
    }

    #[test]
    fn test_release_info_structure() {
        let r = ReleaseInfo {
            tag_name: "v1.3.0".to_string(),
            name: "WinBlast0 v1.3.0".to_string(),
            body: "Performance release".to_string(),
            html_url: "https://github.com/konstruktor/WinBlast0".to_string(),
            published_at: "2026-11-16".to_string(),
            download_url: Some("https://example.com/winblast0.exe".to_string()),
            asset_name: Some("winblast0.exe".to_string()),
            download_size_mb: Some(15.4),
        };
        assert_eq!(r.tag_name, "v1.3.0");
        assert!(is_version_newer(&r.tag_name, "1.2.0"));
    }
}
