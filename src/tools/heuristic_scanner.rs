//! `heuristic_scanner.rs` — Zero-allocation static heuristic bug scanner for WinBlast0.
//! Edition: Rust 2024 | Dependencies: std only.

use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

/// Severity ranking for a static heuristic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Heuristic rule violated in scanned Rust source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BugRule {
    /// Unchecked `.unwrap()` or `.expect(...)` outside test code.
    UncheckedUnwrapOrExpect,
    /// Direct slice/vec indexing (`[i]`, `[0]`) without a nearby bounds check.
    OutOfBoundsIndexing,
    /// Narrowing primitive integer cast (`as u8/u16/u32/i8/i16/i32/usize/isize`).
    IntegerTruncationCast,
    /// Unbuffered blocking file or Windows Registry call inside a loop.
    UnbufferedBlockingLoop,
}

impl BugRule {
    /// Default severity associated with each heuristic rule.
    #[must_use]
    pub const fn default_severity(self) -> Severity {
        match self {
            Self::UncheckedUnwrapOrExpect | Self::OutOfBoundsIndexing => Severity::High,
            Self::IntegerTruncationCast => Severity::Medium,
            Self::UnbufferedBlockingLoop => Severity::Critical,
        }
    }
}

/// Zero-allocation borrowed diagnostic emitted by [`HeuristicBugScanner::scan_source_with`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Finding<'a> {
    pub file: &'a Path,
    pub line: usize,
    pub column: usize,
    pub rule: BugRule,
    pub severity: Severity,
    pub snippet: &'a str,
}

/// Owned diagnostic stored inside [`ScanReport`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedFinding {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub rule: BugRule,
    pub severity: Severity,
    pub snippet: String,
}

impl<'a> Finding<'a> {
    #[must_use]
    pub fn to_owned_finding(self) -> OwnedFinding {
        OwnedFinding {
            file: self.file.to_path_buf(),
            line: self.line,
            column: self.column,
            rule: self.rule,
            severity: self.severity,
            snippet: self.snippet.to_owned(),
        }
    }
}

/// Summary counters produced during scanning.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ScanStats {
    pub files_scanned: usize,
    pub lines_scanned: usize,
    pub findings_count: usize,
}

/// Structured report returned by owned scanning APIs.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScanReport {
    pub stats: ScanStats,
    pub findings: Vec<OwnedFinding>,
}

/// Configuration knobs for [`HeuristicBugScanner`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScannerConfig {
    /// Number of preceding lines to inspect for bounds checks (`.len()`, `.get(`, etc.).
    pub bounds_check_lookback: usize,
    /// Skip `target` and `.git` directories during recursive scans.
    pub skip_build_dirs: bool,
    /// Initial byte capacity for the reused file-reading buffer during directory scans.
    pub scratch_buffer_capacity: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            bounds_check_lookback: 4,
            skip_build_dirs: true,
            scratch_buffer_capacity: 32 * 1024,
        }
    }
}

/// High-throughput, zero-allocation static heuristic scanner for Rust codebases.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeuristicBugScanner {
    config: ScannerConfig,
}

/// Stack-allocated per-file lexical and scope tracker.
#[derive(Debug, Default)]
struct FileScanState {
    block_comment_depth: u32,
    in_string: bool,
    raw_string_hashes: Option<u8>,
    brace_depth: u32,
    pending_test_attr: bool,
    test_scope_depth: Option<u32>,
    pending_loop: bool,
    loop_scope_depth: Option<u32>,
    last_bounds_check_line: Option<usize>,
}

impl FileScanState {
    #[inline]
    const fn in_test_code(&self) -> bool {
        self.test_scope_depth.is_some()
    }

    #[inline]
    const fn in_loop(&self) -> bool {
        self.loop_scope_depth.is_some()
    }

    #[inline]
    fn has_recent_bounds_check(&self, current_line: usize, lookback: usize) -> bool {
        self.last_bounds_check_line
            .is_some_and(|prev| current_line.saturating_sub(prev) <= lookback)
    }
}

const NON_INDEXING_KEYWORDS: &[&[u8]] = &[
    b"let", b"mut", b"return", b"in", b"match", b"if", b"else", b"const", b"static", b"type",
    b"pub", b"fn", b"where", b"as", b"break", b"ref", b"move", b"yield",
];

const TRUNCATING_INT_TYPES: &[&[u8]] = &[
    b"u8", b"u16", b"u32", b"i8", b"i16", b"i32", b"usize", b"isize",
];

const BOUNDS_CHECK_MARKERS: &[&[u8]] = &[
    b".len()",
    b".is_empty()",
    b".get(",
    b".get_mut(",
    b"assert!(",
    b"debug_assert!(",
    b".chunks(",
    b".chunks_exact(",
    b".windows(",
    b".split_at(",
];

const BLOCKING_FILE_CALLS: &[&[u8]] = &[
    b"File::open(",
    b"File::create(",
    b"OpenOptions::new(",
    b"fs::read(",
    b"fs::read_to_string(",
    b"fs::write(",
    b"fs::copy(",
    b"fs::metadata(",
];

const BLOCKING_REGISTRY_CALLS: &[&[u8]] = &[
    b"RegOpenKeyEx",
    b"RegQueryValueEx",
    b"RegSetValueEx",
    b"RegEnumKeyEx",
    b"RegEnumValue",
    b"RegCreateKeyEx",
    b"RegGetValue",
    b"RegDeleteKey",
    b"RegDeleteValue",
    b".open_subkey(",
    b".get_value(",
    b".set_value(",
];

impl HeuristicBugScanner {
    /// Creates a scanner with default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: ScannerConfig {
                bounds_check_lookback: 4,
                skip_build_dirs: true,
                scratch_buffer_capacity: 32 * 1024,
            },
        }
    }

    /// Creates a scanner with custom configuration.
    #[must_use]
    pub const fn with_config(config: ScannerConfig) -> Self {
        Self { config }
    }

    /// Scans an in-memory Rust source string with **zero heap allocations**.
    ///
    /// Borrowed [`Finding`] values are streamed directly to `on_finding`.
    pub fn scan_source_with<'a, F>(
        &self,
        file: &'a Path,
        source: &'a str,
        mut on_finding: F,
    ) -> ScanStats
    where
        F: FnMut(Finding<'a>),
    {
        let mut state = FileScanState::default();
        let mut stats = ScanStats {
            files_scanned: 1,
            lines_scanned: 0,
            findings_count: 0,
        };

        for (line_idx, raw_line) in source.lines().enumerate() {
            let line_no = line_idx + 1;
            stats.lines_scanned += 1;
            let trimmed = raw_line.trim();
            self.scan_line(
                file,
                raw_line,
                trimmed,
                line_no,
                &mut state,
                &mut stats,
                &mut on_finding,
            );
        }

        stats
    }

    /// Scans an in-memory Rust source string and collects owned findings into a [`ScanReport`].
    #[must_use]
    pub fn scan_source(&self, file_label: impl AsRef<Path>, source: &str) -> ScanReport {
        let mut findings = Vec::new();
        let stats = self.scan_source_with(file_label.as_ref(), source, |finding| {
            findings.push(finding.to_owned_finding());
        });
        ScanReport { stats, findings }
    }

    /// Recursively scans all `.rs` files in `root`, reusing one scratch buffer across files.
    ///
    /// # Errors
    /// Returns an `io::Error` if the root path cannot be read.
    pub fn scan_directory(&self, root: &Path) -> io::Result<ScanReport> {
        let mut report = ScanReport::default();
        let mut scratch = String::with_capacity(self.config.scratch_buffer_capacity);
        let mut dir_stack = Vec::with_capacity(32);

        if root.is_file() {
            self.scan_single_file(root, &mut scratch, &mut report)?;
            return Ok(report);
        }

        dir_stack.push(root.to_path_buf());

        while let Some(dir) = dir_stack.pop() {
            let entries = fs::read_dir(&dir)?;
            for entry_res in entries {
                let entry = entry_res?;
                let path = entry.path();
                let file_type = entry.file_type()?;

                if file_type.is_dir() {
                    if self.config.skip_build_dirs && is_skipped_dir(&path) {
                        continue;
                    }
                    dir_stack.push(path);
                } else if file_type.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                    self.scan_single_file(&path, &mut scratch, &mut report)?;
                }
            }
        }

        Ok(report)
    }

    fn scan_single_file(
        &self,
        path: &Path,
        scratch: &mut String,
        report: &mut ScanReport,
    ) -> io::Result<()> {
        scratch.clear();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_string(scratch)?;

        let stats = self.scan_source_with(path, scratch, |finding| {
            report.findings.push(finding.to_owned_finding());
        });

        report.stats.files_scanned += stats.files_scanned;
        report.stats.lines_scanned += stats.lines_scanned;
        report.stats.findings_count += stats.findings_count;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn scan_line<'a, F>(
        &self,
        file: &'a Path,
        line: &'a str,
        trimmed: &'a str,
        line_no: usize,
        state: &mut FileScanState,
        stats: &mut ScanStats,
        on_finding: &mut F,
    ) where
        F: FnMut(Finding<'a>),
    {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0usize;

        let line_is_impl_header = starts_with_keyword(trimmed.as_bytes(), b"impl");
        let line_has_buffered_wrapper =
            contains_bytes(bytes, b"BufReader") || contains_bytes(bytes, b"BufWriter");

        while i < len {
            // 1. Continue multiline block comment.
            if state.block_comment_depth > 0 {
                if starts_with_at(bytes, i, b"/*") {
                    state.block_comment_depth += 1;
                    i += 2;
                } else if starts_with_at(bytes, i, b"*/") {
                    state.block_comment_depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
                continue;
            }

            // 2. Continue multiline raw string literal.
            if let Some(hashes) = state.raw_string_hashes {
                if bytes.get(i) == Some(&b'"') && has_n_hashes(bytes, i + 1, hashes) {
                    state.raw_string_hashes = None;
                    i += 1 + usize::from(hashes);
                } else {
                    i += 1;
                }
                continue;
            }

            // 3. Continue multiline normal string literal.
            if state.in_string {
                match bytes.get(i) {
                    Some(&b'\\') => i += 2,
                    Some(&b'"') => {
                        state.in_string = false;
                        i += 1;
                    }
                    _ => i += 1,
                }
                continue;
            }

            // 4. Enter comments.
            if starts_with_at(bytes, i, b"//") {
                break;
            }
            if starts_with_at(bytes, i, b"/*") {
                state.block_comment_depth += 1;
                i += 2;
                continue;
            }

            // 5. Enter raw string (`r#"..."#` / `br#"..."#`).
            if let Some((next_idx, hashes)) = try_enter_raw_string(bytes, i) {
                state.raw_string_hashes = Some(hashes);
                i = next_idx;
                continue;
            }

            // 6. Enter normal string literal.
            if bytes.get(i) == Some(&b'"') {
                state.in_string = true;
                i += 1;
                continue;
            }

            // 7. Skip character literals (preserving lifetimes like `'a`).
            if bytes.get(i) == Some(&b'\'') {
                if let Some(next_idx) = try_skip_char_literal(line, bytes, i) {
                    i = next_idx;
                    continue;
                }
            }

            // 8. Parse attributes (`#[...]` / `#![...]`) and detect test markers.
            if starts_with_at(bytes, i, b"#[") || starts_with_at(bytes, i, b"#![") {
                if let Some(close_bracket) = find_matching_bracket(bytes, i) {
                    if let Some(attr_slice) = bytes.get(i..=close_bracket) {
                        if is_test_attribute(attr_slice) {
                            state.pending_test_attr = true;
                        }
                    }
                    i = close_bracket + 1;
                    continue;
                }
            }

            // 9. Track scope depth transitions.
            match bytes.get(i) {
                Some(&b'{') => {
                    state.brace_depth += 1;
                    if state.pending_test_attr {
                        state.test_scope_depth = Some(state.brace_depth);
                        state.pending_test_attr = false;
                    }
                    if state.pending_loop {
                        if state.loop_scope_depth.is_none() {
                            state.loop_scope_depth = Some(state.brace_depth);
                        }
                        state.pending_loop = false;
                    }
                    i += 1;
                    continue;
                }
                Some(&b'}') => {
                    if state.test_scope_depth == Some(state.brace_depth) {
                        state.test_scope_depth = None;
                    }
                    if state.loop_scope_depth == Some(state.brace_depth) {
                        state.loop_scope_depth = None;
                    }
                    state.brace_depth = state.brace_depth.saturating_sub(1);
                    i += 1;
                    continue;
                }
                Some(&b';') => {
                    state.pending_test_attr = false;
                    state.pending_loop = false;
                    i += 1;
                    continue;
                }
                _ => {}
            }

            // Skip all diagnostic checks inside `#[cfg(test)]` / `#[test]` scopes.
            if state.in_test_code() {
                i += 1;
                continue;
            }

            // 10. Detect loop keywords (`for`, `while`, `loop`).
            if (!line_is_impl_header && matches_keyword_at(bytes, i, b"for"))
                || matches_keyword_at(bytes, i, b"while")
                || matches_keyword_at(bytes, i, b"loop")
            {
                state.pending_loop = true;
            }

            // 11. Track bounds-checking markers to suppress false positives on `[i]`.
            if BOUNDS_CHECK_MARKERS
                .iter()
                .any(|marker| starts_with_at(bytes, i, marker))
            {
                state.last_bounds_check_line = Some(line_no);
            }

            // Rule 1: Unchecked `.unwrap()` / `.expect(`.
            if starts_with_at(bytes, i, b".unwrap(") || starts_with_at(bytes, i, b".expect(") {
                emit_finding(
                    file,
                    line_no,
                    i + 1,
                    BugRule::UncheckedUnwrapOrExpect,
                    trimmed,
                    stats,
                    on_finding,
                );
            }

            // Rule 2: Out-of-bounds indexing hazard (`arr[i]`, `arr[0]`).
            if bytes.get(i) == Some(&b'[')
                && is_indexing_hazard(bytes, i)
                && !state.has_recent_bounds_check(line_no, self.config.bounds_check_lookback)
            {
                emit_finding(
                    file,
                    line_no,
                    i + 1,
                    BugRule::OutOfBoundsIndexing,
                    trimmed,
                    stats,
                    on_finding,
                );
            }

            // Rule 3: Integer truncation cast (`as u32`, `as usize`, etc.).
            if matches_truncating_cast(bytes, i) {
                emit_finding(
                    file,
                    line_no,
                    i + 1,
                    BugRule::IntegerTruncationCast,
                    trimmed,
                    stats,
                    on_finding,
                );
            }

            // Rule 4: Unbuffered blocking file / Windows registry calls inside loops.
            if state.in_loop() {
                let blocking_file = !line_has_buffered_wrapper
                    && BLOCKING_FILE_CALLS
                        .iter()
                        .any(|call| starts_with_at(bytes, i, call));
                let blocking_registry = BLOCKING_REGISTRY_CALLS
                    .iter()
                    .any(|call| starts_with_at(bytes, i, call));

                if blocking_file || blocking_registry {
                    emit_finding(
                        file,
                        line_no,
                        i + 1,
                        BugRule::UnbufferedBlockingLoop,
                        trimmed,
                        stats,
                        on_finding,
                    );
                }
            }

            i += 1;
        }
    }
}

#[inline]
fn emit_finding<'a, F>(
    file: &'a Path,
    line: usize,
    column: usize,
    rule: BugRule,
    snippet: &'a str,
    stats: &mut ScanStats,
    on_finding: &mut F,
) where
    F: FnMut(Finding<'a>),
{
    stats.findings_count += 1;
    on_finding(Finding {
        file,
        line,
        column,
        rule,
        severity: rule.default_severity(),
        snippet,
    });
}

#[inline]
fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[inline]
fn starts_with_at(bytes: &[u8], idx: usize, needle: &[u8]) -> bool {
    bytes
        .get(idx..)
        .is_some_and(|tail| tail.starts_with(needle))
}

#[inline]
fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}

#[inline]
fn starts_with_keyword(trimmed: &[u8], kw: &[u8]) -> bool {
    trimmed.starts_with(kw)
        && trimmed
            .get(kw.len())
            .is_none_or(|&next| !is_ident_byte(next))
}

#[inline]
fn matches_keyword_at(bytes: &[u8], idx: usize, kw: &[u8]) -> bool {
    let prev_ok = idx == 0 || bytes.get(idx - 1).is_none_or(|&b| !is_ident_byte(b));
    if !prev_ok || !starts_with_at(bytes, idx, kw) {
        return false;
    }
    bytes
        .get(idx + kw.len())
        .is_none_or(|&next| !is_ident_byte(next))
}

#[inline]
fn has_n_hashes(bytes: &[u8], start: usize, count: u8) -> bool {
    let count_usize = usize::from(count);
    bytes
        .get(start..start + count_usize)
        .is_some_and(|slice| slice.iter().all(|&b| b == b'#'))
}

fn try_enter_raw_string(bytes: &[u8], idx: usize) -> Option<(usize, u8)> {
    if bytes.get(idx) != Some(&b'r') {
        return None;
    }
    // Preceded by non-identifier byte, or by `b` (for `br#"..."#`) that itself is standalone.
    let valid_prefix = if idx == 0 {
        true
    } else if bytes.get(idx - 1) == Some(&b'b') {
        idx == 1 || bytes.get(idx - 2).is_none_or(|&b| !is_ident_byte(b))
    } else {
        bytes.get(idx - 1).is_none_or(|&b| !is_ident_byte(b))
    };

    if !valid_prefix {
        return None;
    }

    let mut j = idx + 1;
    let mut hashes = 0u8;
    while bytes.get(j) == Some(&b'#') {
        hashes = hashes.saturating_add(1);
        j += 1;
    }

    if bytes.get(j) == Some(&b'"') {
        Some((j + 1, hashes))
    } else {
        None
    }
}

fn try_skip_char_literal(line: &str, bytes: &[u8], idx: usize) -> Option<usize> {
    if bytes.get(idx + 1) == Some(&b'\\') {
        let mut j = idx + 3;
        let limit = (idx + 14).min(bytes.len());
        while j < limit {
            if bytes.get(j) == Some(&b'\'') {
                return Some(j + 1);
            }
            j += 1;
        }
        return None;
    }

    let ch = line.get(idx + 1..)?.chars().next()?;
    let closing_idx = idx + 1 + ch.len_utf8();
    if bytes.get(closing_idx) == Some(&b'\'') {
        Some(closing_idx + 1)
    } else {
        None
    }
}

fn find_matching_bracket(bytes: &[u8], start: usize) -> Option<usize> {
    let mut depth = 0u32;
    let mut i = start;
    while let Some(&b) = bytes.get(i) {
        if b == b'[' {
            depth += 1;
        } else if b == b']' {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn is_test_attribute(attr: &[u8]) -> bool {
    contains_bytes(attr, b"cfg(test)")
        || contains_bytes(attr, b"#[test]")
        || contains_bytes(attr, b"::test]")
        || contains_bytes(attr, b"::test(")
}

fn is_indexing_hazard(bytes: &[u8], open_idx: usize) -> bool {
    // Locate preceding non-whitespace token.
    let mut prev_idx = open_idx;
    while prev_idx > 0 {
        prev_idx -= 1;
        if let Some(&b) = bytes.get(prev_idx) {
            if !b.is_ascii_whitespace() {
                break;
            }
        }
    }

    let Some(&prev_byte) = bytes.get(prev_idx) else {
        return false;
    };
    if prev_idx == open_idx || prev_byte.is_ascii_whitespace() {
        return false;
    }

    if is_ident_byte(prev_byte) {
        let mut ident_start = prev_idx;
        while ident_start > 0
            && bytes
                .get(ident_start - 1)
                .is_some_and(|&b| is_ident_byte(b))
        {
            ident_start -= 1;
        }
        let ident = bytes.get(ident_start..=prev_idx).unwrap_or(&[]);
        if NON_INDEXING_KEYWORDS.contains(&ident) {
            return false;
        }
    } else if prev_byte != b')' && prev_byte != b']' {
        // Excludes array literals (`= [0]`), slice types (`&[u8]`), attributes (`#[...]`).
        return false;
    }

    // Inspect content inside `[...]`.
    let Some(close_idx) = find_matching_bracket(bytes, open_idx) else {
        return false;
    };
    let inside = bytes.get(open_idx + 1..close_idx).unwrap_or(&[]);
    let trimmed_inside = trim_ascii_bytes(inside);

    // Exclude empty brackets, array type syntax `[T; N]`, and full-range slices `[..]`.
    if trimmed_inside.is_empty() || trimmed_inside == b".." || trimmed_inside.contains(&b';') {
        return false;
    }

    true
}

fn matches_truncating_cast(bytes: &[u8], idx: usize) -> bool {
    if !matches_keyword_at(bytes, idx, b"as") {
        return false;
    }

    let mut j = idx + 2;
    let mut saw_space = false;
    while bytes.get(j).is_some_and(u8::is_ascii_whitespace) {
        saw_space = true;
        j += 1;
    }
    if !saw_space {
        return false;
    }

    let ty_start = j;
    while bytes.get(j).is_some_and(|&b| is_ident_byte(b)) {
        j += 1;
    }

    let ty_slice = bytes.get(ty_start..j).unwrap_or(&[]);
    TRUNCATING_INT_TYPES.contains(&ty_slice)
}

#[inline]
fn trim_ascii_bytes(mut slice: &[u8]) -> &[u8] {
    while let Some((&first, rest)) = slice.split_first() {
        if first.is_ascii_whitespace() {
            slice = rest;
        } else {
            break;
        }
    }
    while let Some((&last, rest)) = slice.split_last() {
        if last.is_ascii_whitespace() {
            slice = rest;
        } else {
            break;
        }
    }
    slice
}

fn is_skipped_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "target" | ".git" | "node_modules"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_all_four_hazard_categories() {
        let src = r#"
            fn optimize_system(data: &[u64], idx: usize, keys: &[&str]) -> u32 {
                let first = data[0];
                let item = data[idx];
                let parsed = "42".parse::<u64>().unwrap();
                let narrowed = parsed as u32;

                for k in keys {
                    let _ = fs::read_to_string(k);
                    let _ = RegQueryValueExW(k);
                }

                (first + item) as usize as u32 + narrowed
            }
        "#;

        let scanner = HeuristicBugScanner::new();
        let report = scanner.scan_source(Path::new("optimizer.rs"), src);

        let count_rule = |rule: BugRule| report.findings.iter().filter(|f| f.rule == rule).count();

        assert_eq!(count_rule(BugRule::OutOfBoundsIndexing), 2);
        assert_eq!(count_rule(BugRule::UncheckedUnwrapOrExpect), 1);
        assert_eq!(count_rule(BugRule::IntegerTruncationCast), 3);
        assert_eq!(count_rule(BugRule::UnbufferedBlockingLoop), 2);
    }

    #[test]
    fn ignores_test_code_comments_strings_and_checked_indexing() {
        let src = r##"
            // data[0].unwrap() as u32
            /*
               nested /* RegOpenKeyExW */ .expect("comment")
            */
            fn safe_lookup(data: &[u64], idx: usize) -> Option<u64> {
                let msg = "fake .unwrap() and arr[0] as u32";
                let raw = r#"raw .expect("nope") as usize"#;
                let _arr: [u8; 4] = [0; 4];
                let _full = &data[..];
                let _ = (msg, raw);

                if idx < data.len() {
                    return Some(data[idx]);
                }
                None
            }

            #[cfg(test)]
            mod tests {
                #[test]
                fn allowed_in_tests() {
                    let v = [1, 2, 3];
                    let _ = v[0];
                    let _ = Some(1u64).unwrap() as u32;
                }
            }
        "##;

        let scanner = HeuristicBugScanner::new();
        let mut finding_count = 0usize;
        let stats = scanner.scan_source_with(Path::new("safe.rs"), src, |_| {
            finding_count += 1;
        });

        assert_eq!(stats.files_scanned, 1);
        assert_eq!(finding_count, 0);
    }

    #[test]
    fn ignores_buffered_file_loop_and_impl_for_header() {
        let src = r#"
            impl Display for Optimizer {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    for path in &self.paths {
                        let _reader = BufReader::new(File::open(path)?);
                    }
                    Ok(())
                }
            }
        "#;

        let scanner = HeuristicBugScanner::new();
        let report = scanner.scan_source(Path::new("display.rs"), src);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn test_scan_entire_winblast0_src_directory_for_critical_hazards() {
        let scanner = HeuristicBugScanner::new();
        let report = scanner
            .scan_directory(Path::new("src"))
            .expect("Failed to scan src directory");

        println!(
            "Heuristic Bug Scanner Complete: Scanned {} files, {} lines. Total findings: {}",
            report.stats.files_scanned, report.stats.lines_scanned, report.stats.findings_count
        );

        let unbuffered_files: Vec<_> = report
            .findings
            .iter()
            .filter(|f| {
                f.rule == BugRule::UnbufferedBlockingLoop
                    && (f.snippet.contains("fs::") || f.snippet.contains("File::"))
            })
            .collect();

        for f in &unbuffered_files {
            eprintln!(
                "UNBUFFERED FILE LOOP: {}:{}:{} - {}",
                f.file.display(),
                f.line,
                f.column,
                f.snippet
            );
        }

        assert!(
            unbuffered_files.is_empty(),
            "Found {} unbuffered file loops in production src/",
            unbuffered_files.len()
        );
    }
}
