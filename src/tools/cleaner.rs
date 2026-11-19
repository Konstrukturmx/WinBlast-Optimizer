//! Safe, symlink-aware 8-target Disk & Temp Cleanup subsystem.
//!
//! Ported from `optimizerDuck`'s parallel Disk Cleanup + hardened with:
//! - Explicit `%TEMP%\.net\` preservation (avoids breaking running single-file .NET apps)
//! - `FILE_ATTRIBUTE_REPARSE_POINT` guard (never follows directory junctions or symlinks!)
//! - Safe default selections (Prefetch & Windows.old & Update Cache unchecked by default; GPU Shader cache untouched)
//! - Native `SHQueryRecycleBinW` + `SHEmptyRecycleBinW` with empty short-circuit (`0x8000FFFF` safe)

use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::{
    SetFileAttributesW, FILE_ATTRIBUTE_NORMAL, FILE_FLAGS_AND_ATTRIBUTES,
};
use windows::Win32::UI::Shell::{
    SHEmptyRecycleBinW, SHQueryRecycleBinW, SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI,
    SHERB_NOSOUND, SHQUERYRBINFO,
};

use crate::core::error::{Result, WinBlastError};

const FILE_ATTRIBUTE_READONLY: u32 = 0x0001;
const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;
const ERROR_SHARING_VIOLATION: i32 = 32;
const ERROR_LOCK_VIOLATION: i32 = 33;
const ERROR_USER_MAPPED_FILE: i32 = 1224;

struct ComStaGuard(bool);

impl Drop for ComStaGuard {
    fn drop(&mut self) {
        if self.0 {
            unsafe {
                windows::Win32::System::Com::CoUninitialize();
            }
        }
    }
}

fn init_com_sta() -> std::result::Result<ComStaGuard, String> {
    use windows::Win32::Foundation::{RPC_E_CHANGED_MODE, S_FALSE, S_OK};
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

    let hr = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    if hr == S_OK || hr == S_FALSE {
        Ok(ComStaGuard(true))
    } else if hr == RPC_E_CHANGED_MODE {
        Ok(ComStaGuard(false))
    } else {
        Err(format!("CoInitializeEx failed: 0x{:08X}", hr.0 as u32))
    }
}

/// Supported cleanup categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CleanTargetKind {
    UserTemp,
    WindowsTemp,
    WindowsUpdateCache,
    DeliveryOptimizationCache,
    ThumbnailCache,
    WerAndCrashDumps,
    Prefetch,
    RecycleBin,
}

impl CleanTargetKind {
    pub const ALL: &'static [CleanTargetKind] = &[
        CleanTargetKind::UserTemp,
        CleanTargetKind::WindowsTemp,
        CleanTargetKind::ThumbnailCache,
        CleanTargetKind::WerAndCrashDumps,
        CleanTargetKind::RecycleBin,
        CleanTargetKind::WindowsUpdateCache,
        CleanTargetKind::DeliveryOptimizationCache,
        CleanTargetKind::Prefetch,
    ];

    pub const fn default_selected(self) -> bool {
        matches!(
            self,
            CleanTargetKind::UserTemp
                | CleanTargetKind::WindowsTemp
                | CleanTargetKind::ThumbnailCache
                | CleanTargetKind::WerAndCrashDumps
        )
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            CleanTargetKind::UserTemp => "User Temporary Files (%TEMP%)",
            CleanTargetKind::WindowsTemp => "Windows Temporary Files (C:\\Windows\\Temp)",
            CleanTargetKind::WindowsUpdateCache => "Windows Update Download Cache",
            CleanTargetKind::DeliveryOptimizationCache => "Delivery Optimization Cache",
            CleanTargetKind::ThumbnailCache => "Explorer Thumbnail Cache",
            CleanTargetKind::WerAndCrashDumps => "Crash Dumps & Error Reports",
            CleanTargetKind::Prefetch => "Windows Prefetch Cache (Caution)",
            CleanTargetKind::RecycleBin => "Recycle Bin",
        }
    }

    pub const fn id_key(self) -> &'static str {
        match self {
            CleanTargetKind::UserTemp => "user_temp",
            CleanTargetKind::WindowsTemp => "windows_temp",
            CleanTargetKind::WindowsUpdateCache => "wu_cache",
            CleanTargetKind::DeliveryOptimizationCache => "do_cache",
            CleanTargetKind::ThumbnailCache => "thumb_cache",
            CleanTargetKind::WerAndCrashDumps => "wer_dumps",
            CleanTargetKind::Prefetch => "prefetch",
            CleanTargetKind::RecycleBin => "recycle_bin",
        }
    }
}

/// Per-target scan metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanTargetScan {
    pub kind: CleanTargetKind,
    pub paths: Vec<String>,
    pub reclaimable_bytes: u64,
    pub file_count: u64,
    pub selected: bool,
    pub requires_elevation: bool,
    pub last_error: Option<String>,
}

/// Per-target deletion outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanTargetResult {
    pub kind: CleanTargetKind,
    pub freed_bytes: u64,
    pub deleted_files: u64,
    pub skipped_locked: u64,
    pub error: Option<String>,
}

/// Aggregated cleanup execution summary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanExecutionReport {
    pub results: Vec<CleanTargetResult>,
    pub total_freed_bytes: u64,
    pub total_deleted_files: u64,
    pub total_skipped_locked: u64,
}

fn is_reparse_point(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| (m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0)
        .unwrap_or(true)
}

fn dir_meta_if_real(dir: &Path) -> std::result::Result<bool, std::io::Error> {
    let meta = fs::symlink_metadata(dir)?;
    if (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0 {
        return Ok(false);
    }
    Ok(meta.is_dir())
}

fn record_dir_io_error(slot: &mut Option<String>, dir: &Path, err: &std::io::Error) {
    if err.kind() == std::io::ErrorKind::NotFound {
        return;
    }
    if slot.is_none() {
        *slot = Some(format!("{}: {}", dir.display(), err));
    }
}

fn resolve_target_paths(kind: CleanTargetKind) -> Vec<PathBuf> {
    let windir = PathBuf::from(std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string()));
    let local_app = std::env::var("LOCALAPPDATA").ok().map(PathBuf::from);
    let prog_data = PathBuf::from(
        std::env::var("ProgramData").unwrap_or_else(|_| "C:\\ProgramData".to_string()),
    );

    match kind {
        CleanTargetKind::UserTemp => vec![std::env::temp_dir()],
        CleanTargetKind::WindowsTemp => vec![windir.join("Temp")],
        CleanTargetKind::WindowsUpdateCache => {
            vec![windir.join("SoftwareDistribution").join("Download")]
        }
        CleanTargetKind::DeliveryOptimizationCache => vec![windir
            .join("ServiceProfiles")
            .join("NetworkService")
            .join("AppData")
            .join("Local")
            .join("Microsoft")
            .join("Windows")
            .join("DeliveryOptimization")
            .join("Cache")],
        CleanTargetKind::ThumbnailCache => match local_app {
            Some(la) => vec![la.join("Microsoft").join("Windows").join("Explorer")],
            None => Vec::new(),
        },
        CleanTargetKind::WerAndCrashDumps => {
            let mut v = Vec::new();
            if let Some(la) = local_app {
                v.push(la.join("CrashDumps"));
            }
            let wer = prog_data.join("Microsoft").join("Windows").join("WER");
            v.push(wer.join("ReportQueue"));
            v.push(wer.join("ReportArchive"));
            v.push(wer.join("Temp"));
            v
        }
        CleanTargetKind::Prefetch => vec![windir.join("Prefetch")],
        CleanTargetKind::RecycleBin => vec![PathBuf::from("Shell::RecycleBin")],
    }
}

fn should_skip_entry(kind: CleanTargetKind, path: &Path, is_dir: bool) -> bool {
    let name_lower = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    match kind {
        CleanTargetKind::UserTemp if is_dir && name_lower == ".net" => true,
        CleanTargetKind::ThumbnailCache => is_dir || !name_lower.starts_with("thumbcache_"),
        CleanTargetKind::Prefetch => is_dir || name_lower == "layout.ini",
        CleanTargetKind::WindowsUpdateCache => name_lower == "datastore.edb",
        _ => false,
    }
}

fn measure_dir(
    kind: CleanTargetKind,
    dir: &Path,
    total_bytes: &mut u64,
    total_files: &mut u64,
    first_error: &mut Option<String>,
) {
    match dir_meta_if_real(dir) {
        Ok(true) => {}
        Ok(false) => return,
        Err(e) => {
            record_dir_io_error(first_error, dir, &e);
            return;
        }
    }
    if is_reparse_point(dir) {
        return;
    }
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            record_dir_io_error(first_error, dir, &e);
            return;
        }
    };
    for entry in rd.flatten() {
        let p = entry.path();
        let Ok(meta) = fs::symlink_metadata(&p) else {
            continue;
        };
        if (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0 {
            continue;
        }
        let is_dir = meta.is_dir();
        if should_skip_entry(kind, &p, is_dir) {
            continue;
        }
        if is_dir {
            measure_dir(kind, &p, total_bytes, total_files, first_error);
        } else if meta.is_file() {
            *total_bytes = total_bytes.saturating_add(meta.len());
            *total_files = total_files.saturating_add(1);
        }
    }
}

fn delete_dir_contents(
    kind: CleanTargetKind,
    dir: &Path,
    remove_self: bool,
    res: &mut CleanTargetResult,
) {
    match dir_meta_if_real(dir) {
        Ok(true) => {}
        Ok(false) => return,
        Err(e) => {
            record_dir_io_error(&mut res.error, dir, &e);
            return;
        }
    }
    if is_reparse_point(dir) {
        return;
    }
    let rd = match fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            record_dir_io_error(&mut res.error, dir, &e);
            return;
        }
    };
    for entry in rd.flatten() {
        let p = entry.path();
        let Ok(meta) = fs::symlink_metadata(&p) else {
            continue;
        };
        if (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0 {
            continue;
        }
        let is_dir = meta.is_dir();
        if should_skip_entry(kind, &p, is_dir) {
            continue;
        }
        if is_dir {
            delete_dir_contents(kind, &p, true, res);
        } else if meta.is_file() {
            let sz = meta.len();
            let attrs = meta.file_attributes();
            if (attrs & FILE_ATTRIBUTE_READONLY) != 0 {
                let cleared = attrs & !FILE_ATTRIBUTE_READONLY;
                let new_attrs = if cleared == 0 {
                    FILE_ATTRIBUTE_NORMAL.0
                } else {
                    cleared
                };
                let wide: Vec<u16> = p
                    .as_os_str()
                    .encode_wide()
                    .chain(std::iter::once(0))
                    .collect();
                let _ = unsafe {
                    SetFileAttributesW(PCWSTR(wide.as_ptr()), FILE_FLAGS_AND_ATTRIBUTES(new_attrs))
                };
            }
            match fs::remove_file(&p) {
                Ok(()) => {
                    res.freed_bytes = res.freed_bytes.saturating_add(sz);
                    res.deleted_files = res.deleted_files.saturating_add(1);
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e)
                    if matches!(
                        e.raw_os_error(),
                        Some(
                            ERROR_SHARING_VIOLATION
                                | ERROR_LOCK_VIOLATION
                                | ERROR_USER_MAPPED_FILE
                        )
                    ) =>
                {
                    res.skipped_locked = res.skipped_locked.saturating_add(1);
                }
                Err(e) => {
                    record_dir_io_error(&mut res.error, &p, &e);
                }
            }
        }
    }
    if remove_self {
        let _ = fs::remove_dir(dir);
    }
}

/// Query total bytes & item count across all local Recycle Bins via `SHQueryRecycleBinW` (COM STA guarded).
fn query_recycle_bin() -> (u64, u64, Option<String>) {
    let _com = match init_com_sta() {
        Ok(g) => g,
        Err(e) => return (0, 0, Some(e)),
    };
    let mut info = SHQUERYRBINFO {
        cbSize: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        i64Size: 0,
        i64NumItems: 0,
    };
    match unsafe { SHQueryRecycleBinW(None, &mut info) } {
        Ok(()) => (
            info.i64Size.max(0) as u64,
            info.i64NumItems.max(0) as u64,
            None,
        ),
        Err(e) => (0, 0, Some(format!("SHQueryRecycleBinW: {}", e))),
    }
}

/// Empty Recycle Bin via `SHEmptyRecycleBinW` (COM STA guarded) and measure freed bytes by diffing before/after.
fn empty_recycle_bin() -> CleanTargetResult {
    let mut r = CleanTargetResult {
        kind: CleanTargetKind::RecycleBin,
        freed_bytes: 0,
        deleted_files: 0,
        skipped_locked: 0,
        error: None,
    };

    let _com = match init_com_sta() {
        Ok(g) => g,
        Err(e) => {
            r.error = Some(e);
            return r;
        }
    };

    let (before_bytes, before_items, q_err) = query_recycle_bin();
    if let Some(err) = q_err {
        r.error = Some(err);
        return r;
    }
    if before_items == 0 && before_bytes == 0 {
        return r;
    }

    let flags = SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND;
    if let Err(e) = unsafe { SHEmptyRecycleBinW(None, None, flags) } {
        // 0x8000FFFF (E_UNEXPECTED) occurs if bin was emptied concurrently
        if e.code().0 as u32 != 0x8000FFFF {
            r.error = Some(format!("SHEmptyRecycleBinW: {}", e));
        }
    }

    let (after_bytes, after_items, after_err) = query_recycle_bin();
    if after_err.is_none() {
        r.freed_bytes = before_bytes.saturating_sub(after_bytes);
        r.deleted_files = before_items.saturating_sub(after_items);
    } else if r.error.is_none() {
        r.error = after_err;
    }
    r
}

/// Scan all 8 cleanup targets in parallel using Rayon without deleting anything.
pub fn scan_all_clean_targets() -> Vec<CleanTargetScan> {
    CleanTargetKind::ALL
        .into_par_iter()
        .map(|&kind| {
            let paths = resolve_target_paths(kind);
            let path_strings: Vec<String> = paths.iter().map(|p| p.display().to_string()).collect();
            let requires_elevation = matches!(
                kind,
                CleanTargetKind::WindowsTemp
                    | CleanTargetKind::WindowsUpdateCache
                    | CleanTargetKind::DeliveryOptimizationCache
                    | CleanTargetKind::WerAndCrashDumps
                    | CleanTargetKind::Prefetch
            );

            if kind == CleanTargetKind::RecycleBin {
                let (bytes, files, err) = query_recycle_bin();
                CleanTargetScan {
                    kind,
                    paths: path_strings,
                    reclaimable_bytes: bytes,
                    file_count: files,
                    selected: kind.default_selected(),
                    requires_elevation: false,
                    last_error: err,
                }
            } else {
                let mut total_bytes = 0u64;
                let mut total_files = 0u64;
                let mut first_error = None;
                for dir in &paths {
                    measure_dir(kind, dir, &mut total_bytes, &mut total_files, &mut first_error);
                }
                CleanTargetScan {
                    kind,
                    paths: path_strings,
                    reclaimable_bytes: total_bytes,
                    file_count: total_files,
                    selected: kind.default_selected(),
                    requires_elevation,
                    last_error: first_error,
                }
            }
        })
        .collect()
}

/// Clean selected targets cleanly, returning freed byte/file telemetry.
pub fn execute_clean_targets(kinds: &[CleanTargetKind]) -> Result<CleanExecutionReport> {
    let mut report = CleanExecutionReport::default();

    for &kind in kinds {
        if kind == CleanTargetKind::RecycleBin {
            let r = empty_recycle_bin();
            report.total_freed_bytes = report.total_freed_bytes.saturating_add(r.freed_bytes);
            report.total_deleted_files = report.total_deleted_files.saturating_add(r.deleted_files);
            report.results.push(r);
            continue;
        }

        let mut r = CleanTargetResult {
            kind,
            freed_bytes: 0,
            deleted_files: 0,
            skipped_locked: 0,
            error: None,
        };
        for dir in resolve_target_paths(kind) {
            delete_dir_contents(kind, &dir, false, &mut r);
        }
        report.total_freed_bytes = report.total_freed_bytes.saturating_add(r.freed_bytes);
        report.total_deleted_files = report.total_deleted_files.saturating_add(r.deleted_files);
        report.total_skipped_locked = report.total_skipped_locked.saturating_add(r.skipped_locked);
        report.results.push(r);
    }

    if report.results.is_empty() && !kinds.is_empty() {
        return Err(WinBlastError::TweakFailed("No cleanup targets executed".to_string()));
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_dir_preserves_dot_net_and_reparse_safe() {
        let root = std::env::temp_dir().join(format!("wb0_clean_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(root.join(".net").join("app")).unwrap();
        fs::create_dir_all(root.join("junk_sub")).unwrap();
        fs::write(root.join(".net").join("app").join("runtime.dll"), b"keep").unwrap();
        fs::write(root.join("junk_sub").join("old.tmp"), b"123456").unwrap();

        let mut bytes = 0;
        let mut files = 0;
        let mut first_err = None;
        measure_dir(CleanTargetKind::UserTemp, &root, &mut bytes, &mut files, &mut first_err);
        assert!(first_err.is_none());
        assert_eq!(bytes, 6);
        assert_eq!(files, 1);

        let mut res = CleanTargetResult {
            kind: CleanTargetKind::UserTemp,
            freed_bytes: 0,
            deleted_files: 0,
            skipped_locked: 0,
            error: None,
        };
        delete_dir_contents(CleanTargetKind::UserTemp, &root, false, &mut res);
        assert_eq!(res.freed_bytes, 6);
        assert_eq!(res.deleted_files, 1);
        assert!(root.join(".net").join("app").join("runtime.dll").exists());
        assert!(!root.join("junk_sub").exists());

        let _ = fs::remove_dir_all(&root);
    }
}
