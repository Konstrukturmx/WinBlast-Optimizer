//! Constrained PowerShell/DISM runner for AppX package removal, scheduled tasks, and hosts mutations.

use std::fs::OpenOptions;
use std::io::Write;

/// Remove an AppX package accurately matching `PackageFamilyName` for current and all users.
pub fn remove_appx_package(package_family: &str) -> crate::core::error::Result<()> {
    let clean = package_family.trim();
    if clean.is_empty() {
        return Ok(());
    }
    if crate::tools::features_apps::classify_appx_package(clean)
        == crate::tools::features_apps::AppxSafetyTier::ProtectedCore
    {
        return Err(crate::core::error::WinBlastError::AppX(format!(
            "Refused to remove protected Windows core package '{}'",
            clean
        )));
    }
    let escaped = clean.replace('\'', "''");
    // Match exact PackageFamilyName cleanly and propagate removal failures instead of swallowing errors
    let script = format!(
        "$pf = '{}'; $errs = @(); \
         foreach ($p in (Get-AppxPackage -AllUsers | Where-Object {{ $_.PackageFamilyName -eq $pf }})) {{ \
           try {{ Remove-AppxPackage -Package $p.PackageFullName -AllUsers -ErrorAction Stop }} catch {{ $errs += $_.Exception.Message }} \
         }}; \
         $pfx = ($pf -split '_')[0]; \
         Get-AppxProvisionedPackage -Online | Where-Object {{ $_.DisplayName -eq $pfx }} | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue | Out-Null; \
         if ($errs.Count -gt 0) {{ Write-Error ($errs -join '; '); exit 1 }}",
        escaped
    );

    let output = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| {
            crate::core::error::WinBlastError::AppX(format!("PowerShell AppX error: {}", e))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::core::error::WinBlastError::AppX(format!(
            "AppX removal failed for {}: {}",
            package_family,
            stderr.trim()
        )));
    }

    Ok(())
}

/// Enable or disable a scheduled task via `schtasks`, distinguishing `AccessDenied` / errors
/// from genuine `NotFound` and resolving Win11 24H2 `" Exp"` suffix variants cleanly.
pub fn set_scheduled_task_enabled(task_path: &str, enable: bool) -> crate::core::error::Result<()> {
    let resolved = crate::scanner::task_scanner::query_single_task_detailed(task_path)?;
    let actual_path = match resolved {
        Some((p, _)) => p,
        None => {
            if !enable {
                return Ok(());
            }
            return Err(crate::core::error::WinBlastError::ScheduledTask(format!(
                "Scheduled task '{}' does not exist on this system",
                task_path
            )));
        }
    };

    let verb = if enable { "/Enable" } else { "/Disable" };
    let output = crate::core::process::SilentCommand::new("schtasks")
        .args(["/Change", "/TN", &actual_path, verb])
        .output()
        .map_err(crate::core::error::WinBlastError::Io)?;

    if !output.status.success() {
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&output.stderr);
        let stdout = crate::scanner::task_scanner::decode_oem_bytes(&output.stdout);
        return Err(crate::core::error::WinBlastError::ScheduledTask(format!(
            "Failed to {} task {}: {} {}",
            if enable { "enable" } else { "disable" },
            actual_path,
            stdout.trim(),
            stderr.trim()
        )));
    }

    Ok(())
}

const HOSTS_BLOCK_HEADER: &str = "# WinBlast0 Telemetry Blocklist";
const HOSTS_BLOCK_FOOTER: &str = "# End WinBlast0 Telemetry Blocklist";

/// Find byte offset of the exact line matching `target` (ignoring leading UTF-8 BOM and surrounding whitespace)
/// starting at or after `start_offset`.
fn find_exact_marker_line(content: &str, target: &str, start_offset: usize) -> Option<usize> {
    let mut offset = start_offset;
    for line in content[start_offset..].split_inclusive('\n') {
        let cleaned = line.trim_start_matches('\u{FEFF}').trim();
        if cleaned == target {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

/// Pure content helper parsing uncommented `0.0.0.0`/`127.0.0.1`/`::1`/`::` rules and appending only genuinely
/// unblocked telemetry hosts with Windows CRLF formatting inside a marked `# WinBlast0 Telemetry Blocklist` section.
pub fn block_telemetry_hosts_in_content(existing: &str, hosts: &[&str]) -> (String, usize) {
    let already_blocked =
        crate::scanner::telemetry_scanner::parse_blocked_hosts_from_content(existing);
    let mut updated = existing.to_string();
    let mut added = 0usize;
    let mut to_append = String::new();

    for host in hosts {
        let is_already_blocked = already_blocked.iter().any(|b| b.eq_ignore_ascii_case(host));
        if !is_already_blocked {
            to_append.push_str(&format!("0.0.0.0 {}\r\n", host));
            added += 1;
        }
    }

    if added > 0 {
        let footer_pos = find_exact_marker_line(&updated, HOSTS_BLOCK_HEADER, 0)
            .and_then(|hdr| find_exact_marker_line(&updated, HOSTS_BLOCK_FOOTER, hdr))
            .or_else(|| find_exact_marker_line(&updated, HOSTS_BLOCK_FOOTER, 0));

        if let Some(footer_idx) = footer_pos {
            let prefix_needs_crlf = footer_idx > 0 && !updated[..footer_idx].ends_with('\n');
            let mut insertion = String::new();
            if prefix_needs_crlf {
                insertion.push_str("\r\n");
            }
            insertion.push_str(&to_append);
            updated.insert_str(footer_idx, &insertion);
        } else {
            if !updated.is_empty() && !updated.ends_with('\n') {
                updated.push_str("\r\n");
            }
            updated.push_str(HOSTS_BLOCK_HEADER);
            updated.push_str("\r\n");
            updated.push_str(&to_append);
            updated.push_str(HOSTS_BLOCK_FOOTER);
            updated.push_str("\r\n");
        }
    }

    (updated, added)
}

/// Remove only WinBlast0's `# WinBlast0 Telemetry Blocklist` section from hosts file content.
pub fn unblock_telemetry_hosts_in_content(existing: &str) -> (String, bool) {
    let Some(header_idx) = find_exact_marker_line(existing, HOSTS_BLOCK_HEADER, 0) else {
        return (existing.to_string(), false);
    };
    let has_exact_footer =
        find_exact_marker_line(existing, HOSTS_BLOCK_FOOTER, header_idx).is_some();

    let mut out = String::with_capacity(existing.len());
    let mut in_section = false;
    let mut removed_any = false;

    for line in existing.split_inclusive('\n') {
        let trimmed = line.trim_start_matches('\u{FEFF}').trim();
        if trimmed == HOSTS_BLOCK_HEADER {
            in_section = true;
            removed_any = true;
            continue;
        }
        if in_section {
            if trimmed == HOSTS_BLOCK_FOOTER {
                in_section = false;
                continue;
            }
            let first_tok = trimmed.split_whitespace().next().unwrap_or("");
            if !has_exact_footer && !trimmed.is_empty() && first_tok != "0.0.0.0" {
                in_section = false;
            } else {
                continue;
            }
        }
        out.push_str(line);
    }

    (out, removed_any)
}

/// Remove WinBlast0's telemetry blocklist section via [`SystemBackend::read_hosts_file`] / `write_hosts_file`.
pub fn unblock_telemetry_hosts_with(
    backend: &dyn crate::core::backend::SystemBackend,
) -> crate::core::error::Result<bool> {
    let Some(existing) = backend.read_hosts_file() else {
        return Ok(false);
    };
    let (updated, changed) = unblock_telemetry_hosts_in_content(&existing);
    if changed {
        backend.write_hosts_file(&updated)?;
    }
    Ok(changed)
}

/// Path of the hosts file (System32 from `GetSystemDirectoryW`).
pub fn hosts_file_path() -> std::path::PathBuf {
    crate::core::process::system32_dir().join("drivers").join("etc").join("hosts")
}

/// Replace the hosts file atomically (F-23): the new content goes to a temporary file next to it,
/// is flushed to disk, and is swapped in with `ReplaceFileW` (which keeps the original's ACL and
/// attributes). A crash or an antivirus lock leaves either the old or the new file, never an
/// empty one.
pub fn write_hosts_file_raw(content: &str) -> crate::core::error::Result<()> {
    let hosts_path = hosts_file_path();
    let tmp_path = hosts_path.with_file_name("hosts.winblast.tmp");
    {
        let mut tmp = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_path)
            .map_err(crate::core::error::WinBlastError::Io)?;
        tmp.write_all(content.as_bytes())
            .map_err(crate::core::error::WinBlastError::Io)?;
        tmp.sync_all().map_err(crate::core::error::WinBlastError::Io)?;
    }
    if !hosts_path.exists() {
        return std::fs::rename(&tmp_path, &hosts_path).map_err(crate::core::error::WinBlastError::Io);
    }
    let wide = |p: &std::path::Path| -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;
        p.as_os_str().encode_wide().chain(std::iter::once(0)).collect()
    };
    let (dst, src) = (wide(&hosts_path), wide(&tmp_path));
    let replaced = unsafe {
        windows::Win32::Storage::FileSystem::ReplaceFileW(
            windows::core::PCWSTR(dst.as_ptr()),
            windows::core::PCWSTR(src.as_ptr()),
            windows::core::PCWSTR::null(),
            windows::Win32::Storage::FileSystem::REPLACE_FILE_FLAGS(0),
            None,
            None,
        )
    };
    if let Err(e) = replaced {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(crate::core::error::WinBlastError::Io(std::io::Error::other(format!(
            "could not replace the hosts file: {e}"
        ))));
    }
    Ok(())
}

/// Hosts from `hosts` that the file does not block yet (what a block would add).
pub fn hosts_not_yet_blocked(existing: &str, hosts: &[&str]) -> Vec<String> {
    let already = crate::scanner::telemetry_scanner::parse_blocked_hosts_from_content(existing);
    hosts
        .iter()
        .filter(|h| !already.iter().any(|b| b.eq_ignore_ascii_case(h)))
        .map(|h| h.to_string())
        .collect()
}

/// Remove only the given hosts from WinBlast's blocklist section (F-23: undoing one block must
/// not remove entries another block added). The markers go once the section is empty.
pub fn unblock_hosts_in_content(existing: &str, hosts: &[String]) -> (String, bool) {
    let mut out = String::with_capacity(existing.len());
    let mut in_section = false;
    let mut removed = false;
    let mut kept_in_section = 0usize;
    let mut lines: Vec<&str> = Vec::new();
    for line in existing.split_inclusive('\n') {
        let trimmed = line.trim_start_matches('\u{FEFF}').trim();
        if trimmed == HOSTS_BLOCK_HEADER {
            in_section = true;
        } else if trimmed == HOSTS_BLOCK_FOOTER {
            in_section = false;
        } else if in_section {
            let mut toks = trimmed.split_whitespace();
            let (ip, host) = (toks.next().unwrap_or(""), toks.next().unwrap_or(""));
            if ip == "0.0.0.0" && hosts.iter().any(|h| h.eq_ignore_ascii_case(host)) {
                removed = true;
                continue;
            }
            if !trimmed.is_empty() {
                kept_in_section += 1;
            }
        }
        lines.push(line);
    }
    for line in lines {
        let trimmed = line.trim_start_matches('\u{FEFF}').trim();
        if kept_in_section == 0 && (trimmed == HOSTS_BLOCK_HEADER || trimmed == HOSTS_BLOCK_FOOTER) {
            continue;
        }
        out.push_str(line);
    }
    (out, removed)
}

/// Block telemetry hostnames in `C:\Windows\System32\drivers\etc\hosts`.
pub fn block_telemetry_hosts(hosts: &[&str]) -> crate::core::error::Result<usize> {
    let hosts_path = hosts_file_path();
    let existing = match std::fs::read_to_string(hosts_path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(crate::core::error::WinBlastError::Io(e)),
    };

    let (updated, added) = block_telemetry_hosts_in_content(&existing, hosts);
    if added > 0 {
        write_hosts_file_raw(&updated)?;
        tracing::info!("Blocked {} telemetry hosts in hosts file", added);
    }

    Ok(added)
}

/// Add an outbound block firewall rule via PowerShell.
#[allow(dead_code)]
pub fn add_firewall_block_rule(
    rule_name: &str,
    remote_address: &str,
) -> crate::core::error::Result<()> {
    let script = format!(
        "New-NetFirewallRule -DisplayName '{}' -Direction Outbound -Action Block -RemoteAddress '{}' -ErrorAction SilentlyContinue",
        rule_name.replace('\'', "''"),
        remote_address.replace('\'', "''")
    );

    let _ = crate::core::process::SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undoing_one_block_keeps_other_blocked_hosts() {
        let base = "127.0.0.1 localhost\r\n";
        let (after_a, _) = block_telemetry_hosts_in_content(base, &["a.example", "b.example"]);
        let (after_b, _) = block_telemetry_hosts_in_content(&after_a, &["c.example"]);
        let (undo_a, changed) =
            unblock_hosts_in_content(&after_b, &["a.example".to_string(), "b.example".to_string()]);
        assert!(changed);
        assert!(undo_a.contains("0.0.0.0 c.example"), "{undo_a}");
        assert!(!undo_a.contains("a.example") && !undo_a.contains("b.example"));
        assert!(undo_a.contains(HOSTS_BLOCK_HEADER), "section stays while it has entries");
        let (empty, _) = unblock_hosts_in_content(&undo_a, &["c.example".to_string()]);
        assert_eq!(empty, base, "markers go with the last entry");
    }

    #[test]
    fn hosts_not_yet_blocked_lists_only_new_hosts() {
        let existing = "0.0.0.0 a.example\r\n";
        assert_eq!(hosts_not_yet_blocked(existing, &["a.example", "B.example"]), vec!["B.example".to_string()]);
    }

    #[test]
    fn test_hosts_blocking_ignores_comments_and_subdomains() {
        let sample = "127.0.0.1 localhost\r\n# 0.0.0.0 vortex.data.microsoft.com\r\n0.0.0.0 sub.telemetry.nvidia.com.local";
        let (out, added) = block_telemetry_hosts_in_content(
            sample,
            &["vortex.data.microsoft.com", "telemetry.nvidia.com"],
        );
        assert_eq!(added, 2);
        assert!(out.contains("0.0.0.0 vortex.data.microsoft.com\r\n"));
        assert!(out.contains("0.0.0.0 telemetry.nvidia.com\r\n"));
    }

    #[test]
    fn test_hosts_bom_ipv6_tabs_and_exact_footer_matching() {
        let bom_hosts = "\u{FEFF}::1\tvortex.data.microsoft.com\r\n::\ttelemetry.nvidia.com\r\n";
        let (_, added) = block_telemetry_hosts_in_content(
            bom_hosts,
            &["vortex.data.microsoft.com", "telemetry.nvidia.com"],
        );
        assert_eq!(added, 0);

        // Stray comment mentioning footer text before/without exact footer must NOT wipe trailing lines
        let tricky = "# note about # End WinBlast0 Telemetry Blocklist\r\n# WinBlast0 Telemetry Blocklist\r\n0.0.0.0\tvortex.data.microsoft.com\r\n# user comment\r\n127.0.0.1 custom.local\r\n";
        let (unblocked, changed) = unblock_telemetry_hosts_in_content(tricky);
        assert!(changed);
        assert!(unblocked.contains("127.0.0.1 custom.local"));
        assert!(unblocked.contains("# user comment"));
        assert!(!unblocked.contains("vortex.data.microsoft.com"));

        // Exact footer line ends the section cleanly while keeping trailing 0.0.0.0 user entries
        let with_exact_footer = "# WinBlast0 Telemetry Blocklist\r\n0.0.0.0 vortex.data.microsoft.com\r\n# End WinBlast0 Telemetry Blocklist\r\n0.0.0.0 custom.blocked.local\r\n";
        let (unblocked2, changed2) = unblock_telemetry_hosts_in_content(with_exact_footer);
        assert!(changed2);
        assert!(!unblocked2.contains("vortex.data.microsoft.com"));
        assert!(unblocked2.contains("0.0.0.0 custom.blocked.local"));
    }
}
