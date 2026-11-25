//! WinBlast0 in-app updater (GitHub Releases).
//!
//! Security model (F-2): an update is installed only when all of these hold —
//! * metadata and the file come from the one release repository, over HTTPS only (curl
//!   `--proto =https --proto-redir =https`, no PowerShell fallback, no downgrade on redirects);
//! * the asset is a `.exe` under that repository's `releases/download/` URL;
//! * it is staged in an admin-only folder (`%ProgramData%\WinBlast0\update`, Administrators and
//!   SYSTEM only), so a standard user cannot swap it between download and install;
//! * its Authenticode signature is intact and was made with the pinned release certificate
//!   ([`RELEASE_SIGNER_THUMBPRINT`]) — checked after download and again right before the swap;
//! * the user clicks Restart; nothing is installed automatically.

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

/// The only repository updates are taken from (the former fallback repository was removed).
pub const GITHUB_REPO: &str = "Konstrukturmx/WinBlast-Optimizer";

/// SHA-1 thumbprint of the certificate that signs releases (CN=Konstrukturmx, valid to
/// 2031-11-23). When the certificate is renewed, ship one release signed with the old
/// certificate that carries the new thumbprint here.
pub const RELEASE_SIGNER_THUMBPRINT: &str = "070A70E017B5BB8F7A345688C82D8A578DAC404D";

/// Release assets must be downloaded from below this URL.
pub const DOWNLOAD_URL_PREFIX: &str = "https://github.com/Konstrukturmx/WinBlast-Optimizer/releases/download/";

/// Split `v1.7.0-rc.1` into numeric parts and a pre-release flag.
fn parse_version(v: &str) -> (Vec<u64>, bool) {
    let clean = v.trim().trim_start_matches(['v', 'V']);
    let (core, pre) = match clean.split_once(['-', '+']) {
        Some((c, rest)) => (c, !rest.is_empty()),
        None => (clean, false),
    };
    let nums = core.split('.').map(|p| p.trim().parse::<u64>().unwrap_or(0)).collect();
    (nums, pre)
}

/// True when `remote_tag` is a newer version than `current_version`. A pre-release
/// (`1.7.0-rc.1`) is older than the release with the same number (B8).
pub fn is_version_newer(remote_tag: &str, current_version: &str) -> bool {
    let (r, r_pre) = parse_version(remote_tag);
    let (c, c_pre) = parse_version(current_version);
    let len = r.len().max(c.len());
    for i in 0..len {
        let (a, b) = (r.get(i).copied().unwrap_or(0), c.get(i).copied().unwrap_or(0));
        if a != b {
            return a > b;
        }
    }
    c_pre && !r_pre
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

/// curl arguments that refuse anything but HTTPS, also after redirects.
const CURL_HTTPS_ONLY: [&str; 4] = ["--proto", "=https", "--proto-redir", "=https"];

fn fetch_url_json(url: &str) -> Result<String, String> {
    if !url.starts_with("https://api.github.com/") {
        return Err(format!("Refusing to query {url}"));
    }
    let output = SilentCommand::new("curl.exe")
        .args(CURL_HTTPS_ONLY)
        .args(["-s", "-f", "--max-time", "20", "-H", "User-Agent: WinBlast0-Updater", url])
        .output()
        .map_err(|e| format!("curl.exe could not be started: {e}"))?;
    if !output.status.success() {
        return Err(format!("Update server request failed (curl exit {:?})", output.status.code()));
    }
    let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if s.starts_with('{') || s.starts_with('[') {
        Ok(s)
    } else {
        Err("Unexpected response from the update server".to_string())
    }
}

/// True for an asset this updater may install: a plain `.exe` name under the release URL.
pub fn is_acceptable_asset(name: &str, url: &str) -> bool {
    let safe_name = !name.is_empty()
        && name.to_ascii_lowercase().ends_with(".exe")
        && !name.contains(['\\', '/', ':', '"', '\''])
        && !name.contains("..");
    safe_name && url.starts_with(DOWNLOAD_URL_PREFIX) && url.ends_with(&format!("/{name}"))
}

/// Query GitHub for the latest release. `Ok(None)` = up to date.
pub fn check_latest_release() -> Result<Option<ReleaseInfo>, String> {
    let latest_url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
    let json_str = fetch_url_json(&latest_url)?;
    let rel: GhRelease = serde_json::from_str(&json_str).map_err(|e| format!("Bad release data: {e}"))?;
    if rel.tag_name.trim().is_empty() || !is_version_newer(&rel.tag_name, env!("CARGO_PKG_VERSION")) {
        return Ok(None);
    }
    let asset = rel
        .assets
        .unwrap_or_default()
        .into_iter()
        .find(|a| is_acceptable_asset(&a.name, &a.browser_download_url));
    Ok(Some(ReleaseInfo {
        tag_name: rel.tag_name,
        name: rel.name.unwrap_or_else(|| "WinBlast0 Update".to_string()),
        body: rel.body.unwrap_or_default(),
        html_url: rel.html_url.unwrap_or_default(),
        published_at: rel.published_at.unwrap_or_default(),
        download_url: asset.as_ref().map(|a| a.browser_download_url.clone()),
        asset_name: asset.as_ref().map(|a| a.name.clone()),
        download_size_mb: asset.as_ref().map(|a| a.size as f64 / (1024.0 * 1024.0)),
    }))
}

/// `%ProgramData%\WinBlast0\update`, emptied and restricted to Administrators and SYSTEM.
fn secure_staging_dir() -> Result<PathBuf, String> {
    if !crate::core::elevation::is_elevated() {
        return Err("Installing updates needs administrator rights; restart WinBlast0 as administrator.".to_string());
    }
    let program_data = crate::core::process::known_program_data_dir()
        .ok_or_else(|| "The ProgramData folder could not be located".to_string())?;
    let dir = program_data.join("WinBlast0").join("update");
    if dir.exists() {
        let meta = std::fs::symlink_metadata(&dir).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() {
            return Err(format!("{} is a link; refusing to stage an update there", dir.display()));
        }
        std::fs::remove_dir_all(&dir).map_err(|e| format!("Could not clear {}: {e}", dir.display()))?;
    }
    std::fs::create_dir_all(&dir).map_err(|e| format!("Could not create {}: {e}", dir.display()))?;
    let dir_s = dir.to_string_lossy().to_string();
    let out = SilentCommand::new("icacls.exe")
        .args([
            dir_s.as_str(),
            "/inheritance:r",
            "/grant:r",
            "*S-1-5-32-544:(OI)(CI)F",
            "/grant:r",
            "*S-1-5-18:(OI)(CI)F",
        ])
        .output()
        .map_err(|e| format!("icacls could not be started: {e}"))?;
    if !out.status.success() {
        return Err(format!("Could not restrict {} to administrators", dir.display()));
    }
    Ok(dir)
}

/// Outcome of an Authenticode check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureCheck {
    /// `Get-AuthenticodeSignature` status name (Valid, UnknownError, NotSigned, HashMismatch …).
    pub status: String,
    /// Upper-case SHA-1 thumbprint of the signing certificate, empty when unsigned.
    pub signer_thumbprint: String,
}

impl SignatureCheck {
    /// Intact signature by the pinned release certificate. A self-signed certificate reports
    /// `UnknownError`/`NotTrusted` (untrusted root) — accepted only together with the pinned
    /// thumbprint; `HashMismatch` (tampered file) and `NotSigned` never are.
    pub fn is_trusted_release(&self, pinned: &str) -> bool {
        matches!(self.status.as_str(), "Valid" | "UnknownError" | "NotTrusted")
            && !self.signer_thumbprint.is_empty()
            && self.signer_thumbprint.eq_ignore_ascii_case(pinned)
    }
}

/// Read the Authenticode status and signer thumbprint of `path`.
pub fn read_signature(path: &Path) -> Result<SignatureCheck, String> {
    let p = path.to_string_lossy().replace('\'', "''");
    let script = format!(
        "$s = Get-AuthenticodeSignature -LiteralPath '{p}'; $t = ''; if ($s.SignerCertificate) {{ $t = $s.SignerCertificate.Thumbprint }}; Write-Output (\"{{0}}|{{1}}\" -f $s.Status, $t)"
    );
    let out = SilentCommand::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| format!("PowerShell could not be started: {e}"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.lines().map(str::trim).rfind(|l| l.contains('|')).unwrap_or("");
    let (status, thumb) = line
        .split_once('|')
        .ok_or_else(|| "The signature of the downloaded file could not be read".to_string())?;
    Ok(SignatureCheck {
        status: status.trim().to_string(),
        signer_thumbprint: thumb.trim().to_ascii_uppercase(),
    })
}

/// Reject anything not signed by the release certificate.
pub fn verify_release_signature(path: &Path) -> Result<(), String> {
    let check = read_signature(path)?;
    if check.is_trusted_release(RELEASE_SIGNER_THUMBPRINT) {
        Ok(())
    } else {
        Err(format!(
            "The downloaded file is not signed by the WinBlast0 release certificate (status {}, signer {}); it was deleted.",
            check.status,
            if check.signer_thumbprint.is_empty() { "none" } else { &check.signer_thumbprint }
        ))
    }
}

/// Download a release asset into the admin-only staging folder and verify its signature.
pub fn download_release_asset(download_url: &str, asset_name: &str) -> Result<PathBuf, String> {
    if !is_acceptable_asset(asset_name, download_url) {
        return Err(format!("Refusing to download {asset_name} from {download_url}"));
    }
    let dir = secure_staging_dir()?;
    let target_file = dir.join(asset_name);
    let target = target_file.to_string_lossy().to_string();
    let out = SilentCommand::new("curl.exe")
        .args(CURL_HTTPS_ONLY)
        .args(["-L", "-s", "-f", "--max-time", "900", "-H", "User-Agent: WinBlast0-Updater", "-o"])
        .arg(&target)
        .arg(download_url)
        .output()
        .map_err(|e| format!("curl.exe could not be started: {e}"))?;
    if !out.status.success() || !target_file.is_file() {
        let _ = std::fs::remove_file(&target_file);
        return Err(format!("Download failed (curl exit {:?})", out.status.code()));
    }
    if let Err(e) = verify_release_signature(&target_file) {
        let _ = std::fs::remove_file(&target_file);
        return Err(e);
    }
    Ok(target_file)
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
    // Verified at download; checked again so nothing swapped in since then gets installed.
    verify_release_signature(new_binary)?;
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
        assert!(is_version_newer("1.7.1", "1.7"));
        assert!(!is_version_newer("1.7", "1.7.0"));
    }

    #[test]
    fn prerelease_is_older_than_release() {
        assert!(!is_version_newer("1.7.0-rc.1", "1.7.0"));
        assert!(is_version_newer("1.7.0", "1.7.0-rc.1"));
        assert!(is_version_newer("1.7.1-rc.1", "1.7.0"));
        assert!(!is_version_newer("1.7.0-rc.2", "1.7.0-rc.1"), "pre-release order is not compared");
    }

    #[test]
    fn only_release_exe_assets_are_accepted() {
        let ok = format!("{DOWNLOAD_URL_PREFIX}v1.8.0/winblast0.exe");
        assert!(is_acceptable_asset("winblast0.exe", &ok));
        assert!(!is_acceptable_asset("winblast0.zip", &format!("{DOWNLOAD_URL_PREFIX}v1.8.0/winblast0.zip")));
        assert!(!is_acceptable_asset("winblast0.exe", "http://github.com/Konstrukturmx/WinBlast-Optimizer/releases/download/v1/winblast0.exe"));
        assert!(!is_acceptable_asset("winblast0.exe", "https://github.com/Devinance/WinBlast-Optimizer/releases/download/v1/winblast0.exe"));
        assert!(!is_acceptable_asset("winblast0.exe", &format!("{DOWNLOAD_URL_PREFIX}v1.8.0/other.exe")));
        assert!(!is_acceptable_asset("..\\evil.exe", &format!("{DOWNLOAD_URL_PREFIX}v1/..\\evil.exe")));
    }

    #[test]
    fn signature_policy() {
        let pin = RELEASE_SIGNER_THUMBPRINT;
        let check = |status: &str, thumb: &str| SignatureCheck { status: status.into(), signer_thumbprint: thumb.into() };
        assert!(check("Valid", pin).is_trusted_release(pin));
        assert!(check("UnknownError", &pin.to_ascii_lowercase()).is_trusted_release(pin));
        assert!(!check("HashMismatch", pin).is_trusted_release(pin), "tampered file");
        assert!(!check("NotSigned", "").is_trusted_release(pin));
        assert!(!check("Valid", "0000000000000000000000000000000000000000").is_trusted_release(pin), "other signer");
    }

    #[test]
    fn unsigned_file_is_rejected() {
        let p = std::env::temp_dir().join(format!("wb_unsigned_{}.exe", std::process::id()));
        std::fs::write(&p, b"MZ not a real program").expect("write");
        assert!(verify_release_signature(&p).is_err());
        let _ = std::fs::remove_file(&p);
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
        assert_eq!(clean_exe_path(r"C:\Tools\winblast0.exe"), r"C:\Tools\winblast0.exe");
        assert_eq!(clean_exe_path(r#""C:\Tools\winblast0.exe""#), r"C:\Tools\winblast0.exe");
        assert_eq!(clean_exe_path("  'C:/Tools/winblast0.exe'  "), r"C:\Tools\winblast0.exe");
    }
}
