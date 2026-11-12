//! Windows Task Scheduler inspector.
//!
//! Provides batch caching, Windows OEM codepage decoding, multi-language status
//! detection (all 8 WinBlast0 languages + European locales), COM `ITaskService`
//! lookup distinguishing AccessDenied vs NotFound, and Win11 24H2 `" Exp"` suffix resolution.

use std::collections::HashMap;

use crate::core::error::{Result, WinBlastError};

/// In-memory cache of scheduled tasks and their enabled states.
#[derive(Debug, Clone, Default)]
pub struct TaskCache {
    /// Normalized lowercase task path -> is_enabled.
    tasks: HashMap<String, bool>,
    loaded: bool,
    live_os: bool,
}

/// Normalize a task path (`/` -> `\`, trim quotes/leading backslashes, lowercase).
pub(crate) fn normalize_task_path(path: &str) -> String {
    let p = path.replace('/', "\\").to_lowercase();
    let trimmed = p.trim().trim_matches('"');
    trimmed.trim_start_matches('\\').to_string()
}

/// Normalize a task path preserving case with a single leading `\` for `schtasks` / COM.
pub(crate) fn canonical_task_path(path: &str) -> String {
    let p = path.replace('/', "\\");
    let trimmed = p.trim().trim_matches('"').trim_start_matches('\\');
    format!("\\{}", trimmed)
}

/// Extract the final task leaf name from a full or normalized task path.
fn task_leaf_name(path: &str) -> String {
    let trimmed = path.trim_end_matches(['\\', '/']);
    trimmed
        .rsplit(['\\', '/'])
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(path)
        .to_string()
}

/// Decode raw console output bytes using Windows active OEM codepage (`CP_OEMCP = 1`)
/// FIRST when `GetOEMCP() != 65001`, falling back to UTF-8 so localized status strings
/// never corrupt or suffer multi-byte CP850/CP1251 UTF-8 collisions.
pub fn decode_oem_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    #[cfg(windows)]
    {
        use windows::Win32::Globalization::{
            GetOEMCP, MultiByteToWideChar, CP_OEMCP, MB_PRECOMPOSED,
        };
        unsafe {
            if GetOEMCP() != 65001 {
                let needed = MultiByteToWideChar(CP_OEMCP, MB_PRECOMPOSED, bytes, None);
                if needed > 0 {
                    let mut wide = vec![0u16; needed as usize];
                    let written =
                        MultiByteToWideChar(CP_OEMCP, MB_PRECOMPOSED, bytes, Some(&mut wide));
                    if written > 0 {
                        return String::from_utf16_lossy(&wide[..written as usize]);
                    }
                }
            }
        }
    }
    if let Ok(utf8) = std::str::from_utf8(bytes) {
        return utf8.to_string();
    }
    String::from_utf8_lossy(bytes).into_owned()
}

/// Determine whether a raw localized status string indicates that the task is disabled.
pub fn is_disabled_status(s: &str) -> bool {
    let lower = s.trim().trim_matches('"').to_lowercase();
    lower == "disabled"
        || lower == "devre dışı"
        || lower == "devre dışı bırakıldı"
        || lower == "devre disi"
        || lower == "отключено"
        || lower == "отключена"
        || lower == "deaktiviert"
        || lower == "desactivado"
        || lower == "desactivada"
        || lower == "deshabilitado"
        || lower == "deshabilitada"
        || lower == "désactivé"
        || lower == "désactivée"
        || lower == "disabilitato"
        || lower == "disabilitata"
        || lower == "uitgeschakeld"
        || lower == "wyłączone"
        || lower == "wyłączony"
        || lower == "desativado"
        || lower == "desativada"
        || lower == "已禁用"
        || lower == "無効"
        || lower.contains("disable")
        || lower.contains("devre")
}

impl TaskCache {
    /// Construct an explicit cache (used by sandbox environments and unit tests).
    pub fn from_map(tasks: HashMap<String, bool>, loaded: bool) -> Self {
        let normalized = tasks
            .into_iter()
            .map(|(k, v)| (normalize_task_path(&k), v))
            .collect();
        Self {
            tasks: normalized,
            loaded,
            live_os: false,
        }
    }

    /// Iterate all cached normalized task paths and enabled states.
    pub fn all_entries(&self) -> impl Iterator<Item = (&String, &bool)> {
        self.tasks.iter()
    }

    /// Load all scheduled tasks on the system using a single fast `schtasks /Query /FO CSV /NH` call.
    pub fn load() -> Self {
        let mut tasks = HashMap::new();

        let output = crate::core::process::SilentCommand::new("schtasks")
            .args(["/Query", "/FO", "CSV", "/NH"])
            .output();

        let loaded = match output {
            Ok(out) if out.status.success() => {
                let stdout = decode_oem_bytes(&out.stdout);
                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty()
                        || line.starts_with("INFO:")
                        || line.starts_with("WARNING:")
                        || line.starts_with("Folder:")
                    {
                        continue;
                    }
                    let columns = parse_csv_line(line);
                    if columns.len() >= 3 {
                        let task_name = normalize_task_path(&columns[0]);
                        let status_col = &columns[columns.len() - 1];
                        let is_disabled = is_disabled_status(status_col);
                        tasks.insert(task_name, !is_disabled);
                    }
                }
                true
            }
            _ => false,
        };

        Self {
            tasks,
            loaded,
            live_os: true,
        }
    }

    /// Evaluate a scheduled task returning `(is_optimized, diagnostic_description, detail_kind)`.
    pub fn evaluate_task_with_kind(
        &self,
        task_path: &str,
        expected_enabled: bool,
    ) -> (bool, String, crate::scanner::ScanDetailKind) {
        use crate::scanner::ScanDetailKind;

        let norm = normalize_task_path(task_path);

        // 1. Exact path match
        if let Some(&enabled) = self.tasks.get(&norm) {
            let is_opt = enabled == expected_enabled;
            let desc = if enabled { "Active (Running/Ready)" } else { "Disabled" };
            return (
                is_opt,
                format!("Task found: {}", desc),
                ScanDetailKind::TaskFound { active: enabled },
            );
        }

        // 2. Explicit Win11 24H2 suffix match (e.g. "<norm> Exp") — deterministic, avoids prefix collision
        let space_variant = format!("{} ", norm);
        let mut variant_hits: Vec<(&String, &bool)> = self
            .tasks
            .iter()
            .filter(|(name, _)| name.starts_with(&space_variant))
            .collect();
        variant_hits.sort_by(|a, b| a.0.cmp(b.0));

        if let Some((name, &enabled)) = variant_hits.first() {
            let is_opt = enabled == expected_enabled;
            let desc = if enabled { "Active (Running/Ready)" } else { "Disabled" };
            let leaf = task_leaf_name(name);
            return (
                is_opt,
                format!("Matched ({}): {}", name, desc),
                ScanDetailKind::TaskMatched {
                    leaf,
                    active: enabled,
                },
            );
        }

        // 3. Fallback single task query when batch cache failed OR on live OS (distinguishes AccessDenied vs NotFound)
        if !self.loaded || self.live_os {
            match query_single_task_detailed(task_path) {
                Ok(Some((actual_path, enabled))) => {
                    let is_opt = enabled == expected_enabled;
                    let desc = if enabled { "Active" } else { "Disabled" };
                    let leaf = task_leaf_name(&actual_path);
                    return (
                        is_opt,
                        format!("Direct query ({}): {}", actual_path, desc),
                        ScanDetailKind::TaskDirect {
                            leaf,
                            active: enabled,
                        },
                    );
                }
                Err(WinBlastError::ElevationRequired(_)) => {
                    return (
                        false,
                        "Task requires Administrator privileges to query".to_string(),
                        ScanDetailKind::TaskRequiresAdmin,
                    );
                }
                Err(e) => {
                    let err = e.to_string();
                    return (
                        false,
                        format!("Task query error: {}", err),
                        ScanDetailKind::TaskQueryError { err },
                    );
                }
                Ok(None) => {}
            }
        }

        // 4. Task genuinely absent from the system
        if !expected_enabled {
            (
                true,
                "Optimized (Task absent / not registered)".to_string(),
                ScanDetailKind::TaskAbsentOptimized,
            )
        } else {
            (
                false,
                "Task absent from system".to_string(),
                ScanDetailKind::TaskAbsentStock,
            )
        }
    }

    /// Evaluate a scheduled task returning `(is_optimized, diagnostic_description)`.
    pub fn evaluate_task(&self, task_path: &str, expected_enabled: bool) -> (bool, String) {
        let (is_opt, msg, _) = self.evaluate_task_with_kind(task_path, expected_enabled);
        (is_opt, msg)
    }

    /// Check if a scheduled task's state matches the expected state.
    #[allow(dead_code)]
    pub fn is_task_enabled(&self, task_path: &str, expected_enabled: bool) -> bool {
        self.evaluate_task(task_path, expected_enabled).0
    }
}

struct ComGuard(bool);

impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.0 {
            unsafe {
                windows::Win32::System::Com::CoUninitialize();
            }
        }
    }
}

fn query_task_via_com(canonical_path: &str) -> Option<Result<Option<(String, bool)>>> {
    use windows::core::BSTR;
    use windows::Win32::Foundation::{E_ACCESSDENIED, RPC_E_CHANGED_MODE, S_FALSE, S_OK};
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
    };
    use windows::Win32::System::TaskScheduler::{ITaskService, TaskScheduler};
    use windows::Win32::System::Variant::VARIANT;

    const HRESULT_FILE_NOT_FOUND: i32 = 0x8007_0002u32 as i32;
    const HRESULT_PATH_NOT_FOUND: i32 = 0x8007_0003u32 as i32;

    let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
    let should_uninit = hr == S_OK || hr == S_FALSE;
    if hr != S_OK && hr != S_FALSE && hr != RPC_E_CHANGED_MODE {
        return None;
    }
    let _guard = ComGuard(should_uninit);

    let service: ITaskService =
        match unsafe { CoCreateInstance(&TaskScheduler, None, CLSCTX_INPROC_SERVER) } {
            Ok(s) => s,
            Err(_) => return None,
        };
    if unsafe {
        service.Connect(
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
        )
    }
    .is_err()
    {
        return None;
    }

    let root_folder = match unsafe { service.GetFolder(&BSTR::from("\\")) } {
        Ok(f) => f,
        Err(e) => {
            return Some(Err(WinBlastError::ScheduledTask(format!(
                "Task Scheduler root folder error: {}",
                e
            ))));
        }
    };

    let try_get = |p: &str| -> std::result::Result<Option<(String, bool)>, windows::core::Error> {
        match unsafe { root_folder.GetTask(&BSTR::from(p)) } {
            Ok(task) => {
                let enabled = unsafe { task.Enabled() }.map(|v| v.as_bool()).unwrap_or(true);
                Ok(Some((p.to_string(), enabled)))
            }
            Err(e)
                if e.code().0 == HRESULT_FILE_NOT_FOUND || e.code().0 == HRESULT_PATH_NOT_FOUND =>
            {
                Ok(None)
            }
            Err(e) => Err(e),
        }
    };

    match try_get(canonical_path) {
        Ok(Some(hit)) => return Some(Ok(Some(hit))),
        Err(e) if e.code() == E_ACCESSDENIED => {
            return Some(Err(WinBlastError::ElevationRequired(format!(
                "Access denied querying scheduled task '{}'",
                canonical_path
            ))));
        }
        Err(e) => {
            return Some(Err(WinBlastError::ScheduledTask(format!(
                "Task query failed for '{}': {}",
                canonical_path, e
            ))));
        }
        Ok(None) => {}
    }

    // Check Win11 24H2 `" Exp"` suffix variant and space-suffixed task names in same folder
    let exp_variant = format!("{} Exp", canonical_path);
    match try_get(&exp_variant) {
        Ok(Some(hit)) => return Some(Ok(Some(hit))),
        Err(e) if e.code() == E_ACCESSDENIED => {
            return Some(Err(WinBlastError::ElevationRequired(format!(
                "Access denied querying scheduled task '{}'",
                exp_variant
            ))));
        }
        Err(e) => {
            return Some(Err(WinBlastError::ScheduledTask(format!(
                "Task query failed for '{}': {}",
                exp_variant, e
            ))));
        }
        Ok(None) => {}
    }

    if let Some((parent, leaf)) = canonical_path.rsplit_once('\\') {
        let folder_path = if parent.is_empty() { "\\" } else { parent };
        if let Ok(folder) = unsafe { service.GetFolder(&BSTR::from(folder_path)) } {
            if let Ok(collection) = unsafe { folder.GetTasks(1) } {
                let count = unsafe { collection.Count() }.unwrap_or(0);
                let space_prefix = format!("{} ", leaf.to_ascii_lowercase());
                let mut candidates: Vec<(String, bool)> = Vec::new();
                for idx in 1..=count {
                    if let Ok(t) = unsafe { collection.get_Item(&VARIANT::from(idx)) } {
                        if let Ok(name_bstr) = unsafe { t.Name() } {
                            let t_name = name_bstr.to_string();
                            if t_name.to_ascii_lowercase().starts_with(&space_prefix) {
                                let t_path = unsafe { t.Path() }
                                    .map(|b| b.to_string())
                                    .unwrap_or_else(|_| {
                                        if folder_path == "\\" {
                                            format!("\\{}", t_name)
                                        } else {
                                            format!("{}\\{}", folder_path, t_name)
                                        }
                                    });
                                let enabled =
                                    unsafe { t.Enabled() }.map(|v| v.as_bool()).unwrap_or(true);
                                candidates.push((t_path, enabled));
                            }
                        }
                    }
                }
                candidates.sort_by(|a, b| a.0.cmp(&b.0));
                if let Some(first) = candidates.into_iter().next() {
                    return Some(Ok(Some(first)));
                }
            }
        }
    }

    Some(Ok(None))
}

fn query_task_via_schtasks(canonical_path: &str) -> Result<Option<(String, bool)>> {
    let query_one = |tn: &str| -> Result<Option<(String, bool)>> {
        let output = crate::core::process::SilentCommand::new("schtasks")
            .args(["/Query", "/TN", tn, "/FO", "CSV", "/NH"])
            .output()
            .map_err(|e| WinBlastError::ScheduledTask(format!("Failed to invoke schtasks: {}", e)))?;

        if output.status.success() {
            let stdout = decode_oem_bytes(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let cols = parse_csv_line(line);
                if cols.len() >= 3 {
                    let is_disabled = is_disabled_status(&cols[cols.len() - 1]);
                    return Ok(Some((tn.to_string(), !is_disabled)));
                }
            }
            return Ok(None);
        }

        let stderr = decode_oem_bytes(&output.stderr);
        let stdout = decode_oem_bytes(&output.stdout);
        let combined = format!("{} {}", stdout.trim(), stderr.trim());
        let lower = combined.to_lowercase();
        if lower.contains("access is denied") || lower.contains("erişim engellendi") || lower.contains("zugriff verweigert") {
            return Err(WinBlastError::ElevationRequired(format!(
                "Access denied querying scheduled task '{}': {}",
                tn,
                combined.trim()
            )));
        }
        if lower.contains("cannot find the file")
            || lower.contains("does not exist")
            || lower.contains("cannot find the path")
        {
            return Ok(None);
        }
        Err(WinBlastError::ScheduledTask(format!(
            "schtasks query failed for '{}': {}",
            tn,
            combined.trim()
        )))
    };

    if let Some(hit) = query_one(canonical_path)? {
        return Ok(Some(hit));
    }
    let exp_variant = format!("{} Exp", canonical_path);
    query_one(&exp_variant)
}

/// Query a single scheduled task distinguishing `NotFound` (`Ok(None)`), `AccessDenied` (`Err(ElevationRequired)`),
/// other errors (`Err(ScheduledTask)`), and resolving Win11 24H2 `" Exp"` task path variants (`Ok(Some((actual_path, enabled)))`).
pub fn query_single_task_detailed(task_path: &str) -> Result<Option<(String, bool)>> {
    let canonical = canonical_task_path(task_path);
    if canonical == "\\" {
        return Err(WinBlastError::ScheduledTask(format!(
            "Invalid scheduled task path '{}'",
            task_path
        )));
    }
    if let Some(com_res) = query_task_via_com(&canonical) {
        return com_res;
    }
    query_task_via_schtasks(&canonical)
}

/// Query the enabled state of a single scheduled task directly (returning `None` on absent OR error).
pub fn get_task_enabled(task_path: &str) -> Option<bool> {
    query_single_task_detailed(task_path)
        .ok()
        .flatten()
        .map(|(_, en)| en)
}

/// Parse a CSV line handling standard double quotes.
pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multilingual_disabled_statuses() {
        assert!(is_disabled_status("Disabled"));
        assert!(is_disabled_status("Devre Dışı"));
        assert!(is_disabled_status("Отключено"));
        assert!(is_disabled_status("Deaktiviert"));
        assert!(is_disabled_status("Deshabilitado"));
        assert!(is_disabled_status("Deshabilitada"));
        assert!(is_disabled_status("Désactivé"));
        assert!(is_disabled_status("已禁用"));
        assert!(is_disabled_status("無効"));
        assert!(!is_disabled_status("Ready"));
        assert!(!is_disabled_status("Hazır"));
    }

    #[test]
    fn test_query_single_task_distinguishes_absent_vs_access_denied() {
        let missing = query_single_task_detailed("\\WinBlast0_Definitely_Absent_Task_12345");
        assert!(matches!(missing, Ok(None)));
    }

    #[test]
    fn test_task_cache_loaded_does_not_cross_match_prefixes() {
        let mut map = HashMap::new();
        map.insert("\\Microsoft\\Foo".to_string(), true);
        map.insert("\\Microsoft\\FooBar".to_string(), false);
        let cache = TaskCache::from_map(map, true);

        // "\Microsoft\Foo" matches exact, never "\Microsoft\FooBar"
        assert!(!cache.is_task_enabled("\\Microsoft\\Foo", false));
        // Unregistered task "\Microsoft\Fo" does not false-match "\Microsoft\Foo"
        assert!(cache.is_task_enabled("\\Microsoft\\Fo", false));
    }

    #[test]
    fn test_evaluate_task_with_kind_all_cache_and_error_branches() {
        use crate::scanner::ScanDetailKind;

        let mut map = HashMap::new();
        map.insert("\\Microsoft\\Windows\\ExactTask".to_string(), false);
        map.insert("\\Microsoft\\Windows\\SuffixTask Exp".to_string(), true);
        let cache = TaskCache::from_map(map, true);

        assert_eq!(
            cache.evaluate_task_with_kind("\\Microsoft\\Windows\\ExactTask", false),
            (
                true,
                "Task found: Disabled".to_string(),
                ScanDetailKind::TaskFound { active: false },
            )
        );
        assert_eq!(
            cache.evaluate_task_with_kind("\\Microsoft\\Windows\\SuffixTask", false),
            (
                false,
                "Matched (microsoft\\windows\\suffixtask exp): Active (Running/Ready)"
                    .to_string(),
                ScanDetailKind::TaskMatched {
                    leaf: "suffixtask exp".to_string(),
                    active: true,
                },
            )
        );
        assert_eq!(
            cache.evaluate_task_with_kind("\\Microsoft\\Windows\\MissingTask", false),
            (
                true,
                "Optimized (Task absent / not registered)".to_string(),
                ScanDetailKind::TaskAbsentOptimized,
            )
        );
        assert_eq!(
            cache.evaluate_task_with_kind("\\Microsoft\\Windows\\MissingTask", true),
            (
                false,
                "Task absent from system".to_string(),
                ScanDetailKind::TaskAbsentStock,
            )
        );

        let unloaded = TaskCache::from_map(HashMap::new(), false);
        let (is_opt, msg, kind) = unloaded.evaluate_task_with_kind("\\", false);
        assert!(!is_opt);
        assert!(msg.starts_with("Task query error: "));
        assert!(matches!(kind, ScanDetailKind::TaskQueryError { .. }));
    }
}
