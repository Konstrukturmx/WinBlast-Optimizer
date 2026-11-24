//! WinBlast0 In-App Self-Updater via GitHub Releases API.
//!
//! Provides background checking, release metadata parsing, asset download,
//! and atomic Windows binary replacement with elevation support.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

/// Primary and fallback GitHub repository targets for updates.
pub const GITHUB_REPO: &str = "Konstrukturmx/WinBlast-Optimizer";
pub const FALLBACK_REPO: &str = "Devinance/WinBlast-Optimizer";

/// Check if a remote version tag is newer than the current application version.
pub fn is_version_newer(remote_tag: &str, current_version: &str) -> bool {
    let clean_remote = remote_tag
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('V');
    let clean_current = current_version
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('V');

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

#[derive(Deserialize)]
struct GhAsset {
    name: String,
    size: u64,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct GhRelease {
    tag_name: String,
    name: Option<String>,
    body: Option<String>,
    html_url: Option<String>,
    published_at: Option<String>,
    assets: Option<Vec<GhAsset>>,
}

/// Helper to fetch URL content via curl.exe or PowerShell.
fn fetch_url_json(url: &str) -> Result<String, String> {
    // 1. Try native curl.exe first (fast, built-in on Win10/Win11)
    if let Ok(output) = SilentCommand::new("curl.exe")
        .args(["-s", "-f", "-H", "User-Agent: WinBlast0-Updater", url])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !s.is_empty() && (s.starts_with('{') || s.starts_with('[')) {
                return Ok(s);
            }
        }
    }

    // 2. Fallback to PowerShell Invoke-RestMethod
    let escaped_url = url.replace('\'', "''");
    let ps_cmd = format!(
        r#"[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
try {{
    $r = Invoke-WebRequest -Uri '{}' -Headers @{{'User-Agent'='WinBlast0-Updater'}} -UseBasicParsing -TimeoutSec 15;
    Write-Output $r.Content
}} catch {{
    Write-Output "ERROR: $($_.Exception.Message)"
}}"#,
        escaped_url
    );

    let output = SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .output()
        .map_err(|e| format!("Failed to spawn PowerShell: {}", e))?;

    let out_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if out_str.starts_with("ERROR:") || out_str.is_empty() {
        return Err(if out_str.is_empty() {
            "Empty response from update server".to_string()
        } else {
            out_str
        });
    }
    Ok(out_str)
}

/// Query GitHub Releases API for the latest published release.
pub fn check_latest_release(repo: &str) -> Result<Option<ReleaseInfo>, String> {
    let repos_to_try = [repo, GITHUB_REPO, FALLBACK_REPO];

    let mut last_err = String::new();

    for r in repos_to_try {
        let latest_url = format!("https://api.github.com/repos/{}/releases/latest", r);
        let all_url = format!("https://api.github.com/repos/{}/releases", r);

        // Try /releases/latest first
        let json_res = fetch_url_json(&latest_url).or_else(|_| fetch_url_json(&all_url));

        let json_str = match json_res {
            Ok(s) => s,
            Err(e) => {
                last_err = e;
                continue;
            }
        };

        // Try parsing single release object or array of releases
        let release: Option<GhRelease> = if json_str.starts_with('[') {
            if let Ok(releases) = serde_json::from_str::<Vec<GhRelease>>(&json_str) {
                releases.into_iter().next()
            } else {
                None
            }
        } else {
            serde_json::from_str::<GhRelease>(&json_str).ok()
        };

        if let Some(rel) = release {
            let tag = rel.tag_name;
            if tag.is_empty() {
                continue;
            }

            let current_ver = env!("CARGO_PKG_VERSION");
            let is_newer = is_version_newer(&tag, current_ver);

            // Find executable asset
            let mut download_url = None;
            let mut asset_name = None;
            let mut download_size_mb = None;

            if let Some(assets) = rel.assets {
                for a in assets {
                    if a.name.ends_with(".exe") || a.name.ends_with(".zip") {
                        download_url = Some(a.browser_download_url);
                        download_size_mb = Some((a.size as f64) / (1024.0 * 1024.0));
                        asset_name = Some(a.name);
                        break;
                    }
                }
            }

            let info = ReleaseInfo {
                tag_name: tag,
                name: rel.name.unwrap_or_else(|| "WinBlast0 Update".to_string()),
                body: rel.body.unwrap_or_default(),
                html_url: rel.html_url.unwrap_or_default(),
                published_at: rel.published_at.unwrap_or_default(),
                download_url,
                asset_name,
                download_size_mb,
            };

            if is_newer {
                return Ok(Some(info));
            } else {
                return Ok(None);
            }
        }
    }

    if last_err.is_empty() {
        Err("Unable to contact GitHub Releases API".to_string())
    } else {
        Err(last_err)
    }
}

/// Download release asset to a temporary directory and stage for atomic replacement.
pub fn download_release_asset(download_url: &str, asset_name: &str) -> Result<PathBuf, String> {
    let temp_dir = std::env::temp_dir().join("winblast0_update");
    let _ = std::fs::create_dir_all(&temp_dir);
    let target_file = temp_dir.join(asset_name);

    let target_path_str = target_file.to_string_lossy().to_string();

    // 1. Try curl.exe -L first (handles GitHub redirects seamlessly)
    let curl_res = SilentCommand::new("curl.exe")
        .args([
            "-L",
            "-s",
            "-f",
            "-H",
            "User-Agent: WinBlast0-Updater",
            "-o",
            &target_path_str,
            download_url,
        ])
        .output();

    if let Ok(c_out) = curl_res {
        if c_out.status.success() && target_file.is_file() {
            if let Ok(meta) = target_file.metadata() {
                if meta.len() > 1_000_000 {
                    return Ok(target_file);
                }
            }
        }
    }

    // 2. Fallback to PowerShell Invoke-WebRequest with redirect support
    let escaped_url = download_url.replace('\'', "''");
    let escaped_target = target_path_str.replace('\'', "''");
    let ps_cmd = format!(
        r#"[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
try {{
    Invoke-WebRequest -Uri '{}' -OutFile '{}' -Headers @{{'User-Agent'='WinBlast0-Updater'}} -UseBasicParsing -TimeoutSec 180;
    if (Test-Path '{}') {{
        Write-Output "SUCCESS"
    }} else {{
        Write-Output "ERROR: Target file not found after download"
    }}
}} catch {{
    Write-Output "ERROR: $($_.Exception.Message)"
}}"#,
        escaped_url, escaped_target, escaped_target
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
/// Normalizes a Windows executable path by stripping verbatim namespaces (`\\?\` and `\\?\UNC\`),
/// trimming spurious whitespace/quotes, and normalizing directory separators.
pub fn clean_exe_path(path_str: &str) -> String {
    let trimmed = path_str.trim().trim_matches('"').trim_matches('\'');
    let normalized = if let Some(stripped) = trimmed.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{}", stripped)
    } else if let Some(stripped) = trimmed.strip_prefix(r"\\?\") {
        stripped.to_string()
    } else {
        trimmed.to_string()
    };
    normalized.replace('/', "\\")
}

/// Applies a staged update binary by swapping the running executable and restarting.
///
/// Procedure:
/// 1. Rename running `winblast0.exe` -> `winblast0.exe.old`.
/// 2. Copy staged `new_binary` into `winblast0.exe` and flush to disk.
/// 3. Launch the new executable using native Win32 `ShellExecuteW` (with RunAs elevation if required).
/// 4. Fallback to detached process spawn if ShellExecute returns an error.
/// 5. Exit current process cleanly.
pub fn apply_and_restart(new_binary: &Path) -> Result<(), String> {
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let backup_exe = current_exe.with_extension("exe.old");

    // Clean any prior backup
    if let Err(e) = std::fs::remove_file(&backup_exe) {
        if e.kind() != std::io::ErrorKind::NotFound {
            tracing::warn!(
                "Failed to remove old backup executable {:?}: {}",
                backup_exe,
                e
            );
        }
    }

    // 1. Rename running exe to .old
    std::fs::rename(&current_exe, &backup_exe)
        .map_err(|e| format!("Failed to rename running executable: {}", e))?;

    // 2. Copy new binary to current exe location
    if let Err(e) = std::fs::copy(new_binary, &current_exe) {
        // Rollback rename if copy fails
        if let Err(restore) = std::fs::rename(&backup_exe, &current_exe) {
            tracing::error!("Could not restore {:?} after a failed update: {}", backup_exe, restore);
            return Err(format!(
                "Failed to install new binary: {e}; restoring the previous executable also failed: {restore} (it is kept at {})",
                backup_exe.display()
            ));
        }
        return Err(format!("Failed to install new binary: {}", e));
    }

    // Ensure new binary is completely flushed to storage
    if let Ok(file) = std::fs::File::open(&current_exe) {
        let _ = file.sync_all();
    }

    // 3. Normalize executable path
    let raw_exe_str = current_exe.to_string_lossy();
    let clean_exe_str = clean_exe_path(&raw_exe_str);
    let is_elevated = crate::core::elevation::is_elevated();

    // 4. Launch via native Win32 ShellExecuteW (avoids cmd.exe/powershell string escaping & UNC '\\' errors)
    #[cfg(windows)]
    {
        let file_w: Vec<u16> = clean_exe_str
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let verb = if is_elevated { "runas" } else { "open" };
        let verb_w: Vec<u16> = verb.encode_utf16().chain(std::iter::once(0)).collect();

        let hinstance = unsafe {
            windows::Win32::UI::Shell::ShellExecuteW(
                None,
                windows::core::PCWSTR(verb_w.as_ptr()),
                windows::core::PCWSTR(file_w.as_ptr()),
                None,
                None,
                windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
            )
        };

        let result_code = hinstance.0 as usize;
        if result_code > 32 {
            tracing::info!(
                "Successfully launched updated binary via Win32 ShellExecuteW (code: {})",
                result_code
            );
            std::process::exit(0);
        } else {
            tracing::warn!(
                "Win32 ShellExecuteW failed with code {}, attempting detached process spawn",
                result_code
            );
        }
    }

    // Fallback: spawn via standard library Command with detached flags
    let mut cmd = std::process::Command::new(&clean_exe_str);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NEW_PROCESS_GROUP (0x200) | DETACHED_PROCESS (0x8)
        cmd.creation_flags(0x00000200 | 0x00000008);
    }
    if let Err(e) = cmd.spawn() {
        tracing::error!("Direct detached spawn fallback failed: {}", e);
        return Err(format!("Failed to launch new binary: {}", e));
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison_logic() {
        assert!(is_version_newer("v1.6.0", "1.5.0"));
        assert!(is_version_newer("1.5.1", "1.5.0"));
        assert!(is_version_newer("2.0.0", "1.9.9"));
        assert!(!is_version_newer("v1.5.0", "1.5.0"));
        assert!(!is_version_newer("1.4.9", "1.5.0"));
        assert!(!is_version_newer("v1.0.0", "1.5.0"));
    }

    #[test]
    fn test_release_info_structure() {
        let r = ReleaseInfo {
            tag_name: "v1.6.0".to_string(),
            name: "WinBlast0 v1.6.0".to_string(),
            body: "Performance release".to_string(),
            html_url: "https://github.com/Konstrukturmx/WinBlast-Optimizer".to_string(),
            published_at: "2026-11-17".to_string(),
            download_url: Some("https://example.com/winblast0.exe".to_string()),
            asset_name: Some("winblast0.exe".to_string()),
            download_size_mb: Some(30.0),
        };
        assert_eq!(r.tag_name, "v1.6.0");
        assert!(is_version_newer(&r.tag_name, "1.5.0"));
    }

    #[test]
    fn test_clean_windows_exe_path() {
        assert_eq!(
            clean_exe_path(r"\\?\C:\Program Files\WinBlast0\winblast0.exe"),
            r"C:\Program Files\WinBlast0\winblast0.exe"
        );
        assert_eq!(
            clean_exe_path(r"\\?\UNC\server\share\winblast0.exe"),
            r"\\server\share\winblast0.exe"
        );
        assert_eq!(
            clean_exe_path(r"C:\Tools\winblast0.exe"),
            r"C:\Tools\winblast0.exe"
        );
        assert_eq!(
            clean_exe_path(r#""C:\Tools\winblast0.exe""#),
            r"C:\Tools\winblast0.exe"
        );
        assert_eq!(
            clean_exe_path("  'C:/Tools/winblast0.exe'  "),
            r"C:\Tools\winblast0.exe"
        );
    }
}
