//! SQLite-backed transactional state ledger.
//!
//! Records every mutation performed by the executor so that individual tweaks
//! can be rolled back without a full System Restore.

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

/// A single recorded change in the state ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// Auto-incremented row ID.
    pub id: i64,
    /// The tweak ID that caused this change.
    pub tweak_id: String,
    /// Type of action (e.g., "RegistryDword", "ServiceState").
    pub action_type: String,
    /// Serialised original value before the change.
    pub original_value: String,
    /// Serialised new value after the change.
    pub new_value: String,
    /// UTC timestamp of when the change was applied.
    pub timestamp: DateTime<Utc>,
    /// Whether this entry has been rolled back.
    pub rolled_back: bool,
}

impl LedgerEntry {
    /// Return `true` when this ledger entry represents an action that can be undone automatically.
    #[inline]
    pub fn is_reversible(&self) -> bool {
        is_reversible_action(&self.action_type)
    }
}

/// Zero-allocation, borrowed view of a ledger record directly over SQLite row memory buffers.
#[derive(Debug, Clone, Copy)]
pub struct LedgerEntryView<'a> {
    /// Auto-incremented row ID.
    pub id: i64,
    /// The tweak ID that caused this change.
    pub tweak_id: &'a str,
    /// Type of action (e.g., "RegistryDword", "ServiceState").
    pub action_type: &'a str,
    /// Serialised original value before the change.
    pub original_value: &'a str,
    /// Serialised new value after the change.
    pub new_value: &'a str,
    /// RFC 3339 timestamp string borrowed directly from the row buffer.
    pub timestamp_str: &'a str,
    /// Whether this entry has been rolled back.
    pub rolled_back: bool,
}

impl<'a> LedgerEntryView<'a> {
    /// Return `true` when this ledger entry represents an action that can be undone automatically.
    #[inline]
    pub fn is_reversible(&self) -> bool {
        is_reversible_action(self.action_type)
    }

    /// Parse the borrowed RFC 3339 timestamp string into `DateTime<Utc>`.
    #[inline]
    pub fn parse_timestamp(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(self.timestamp_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    }

    /// Materialize an owned `LedgerEntry` from this borrowed view.
    pub fn to_owned(&self) -> LedgerEntry {
        LedgerEntry {
            id: self.id,
            tweak_id: self.tweak_id.to_string(),
            action_type: self.action_type.to_string(),
            original_value: self.original_value.to_string(),
            new_value: self.new_value.to_string(),
            timestamp: self.parse_timestamp(),
            rolled_back: self.rolled_back,
        }
    }
}

/// Check if an action type represents a reversible change.
#[inline]
pub fn is_reversible_action(action_type: &str) -> bool {
    let raw = action_type.strip_prefix("Revert:").unwrap_or(action_type);
    let kind = raw.split(':').next().unwrap_or("");
    !matches!(
        kind,
        "CleanTargets" | "DiskCleanup" | "AppXBatch" | "App" | "AppXPackage" | "Startup"
    )
}

/// A recorded system performance and utilization baseline for before/after comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemBaseline {
    pub id: i64,
    pub captured_at: DateTime<Utc>,
    pub total_ram_mb: u64,
    pub used_ram_mb: u64,
    pub process_count: usize,
    pub thread_count: usize,
    pub timer_resolution_ns: u64,
    pub optimized_count: usize,
    pub label: String,
}

/// SQLite-backed ledger that records and can undo system mutations.
pub struct StateLedger {
    conn: Connection,
}

impl StateLedger {
    /// Open (or create) the SQLite ledger database at the given path.
    pub fn new(db_path: &std::path::Path) -> crate::core::error::Result<Self> {
        if let Some(parent) = db_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let conn = Connection::open(db_path)?;
        Self::init_conn(conn, true)
    }

    /// Create an ephemeral in-memory SQLite ledger for isolated automated tests.
    #[allow(dead_code)]
    pub fn in_memory() -> crate::core::error::Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init_conn(conn, false)
    }

    fn init_conn(conn: Connection, enable_wal: bool) -> crate::core::error::Result<Self> {
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        if enable_wal {
            conn.execute_batch(
                "PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;",
            )?;
        }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS ledger (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                tweak_id      TEXT    NOT NULL,
                action_type   TEXT    NOT NULL,
                original_value TEXT   NOT NULL,
                new_value     TEXT    NOT NULL,
                timestamp     TEXT    NOT NULL,
                rolled_back   INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS baselines (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                captured_at         TEXT    NOT NULL,
                total_ram_mb        INTEGER NOT NULL,
                used_ram_mb         INTEGER NOT NULL,
                process_count       INTEGER NOT NULL,
                thread_count        INTEGER NOT NULL,
                timer_resolution_ns INTEGER NOT NULL,
                optimized_count     INTEGER NOT NULL,
                label               TEXT    NOT NULL
            );",
        )?;
        Ok(Self { conn })
    }

    /// Record a single mutation in the ledger.
    pub fn record_change(
        &self,
        tweak_id: &str,
        action_type: &str,
        original_value: &str,
        new_value: &str,
    ) -> crate::core::error::Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO ledger (tweak_id, action_type, original_value, new_value, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tweak_id, action_type, original_value, new_value, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Return the total count of entries recorded in the state ledger without allocating heap strings.
    pub fn count_history(&self) -> crate::core::error::Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(1) FROM ledger",
            [],
            |r| r.get(0),
        )?;
        Ok(count.max(0) as usize)
    }

    /// Zero-copy iterator over state ledger entries, invoking `callback` with borrowed row views.
    ///
    /// Stops iterating when `callback` returns `false`.
    pub fn for_each_history<F>(&self, mut callback: F) -> crate::core::error::Result<()>
    where
        F: FnMut(LedgerEntryView<'_>) -> bool,
    {
        let mut stmt = self.conn.prepare(
            "SELECT id, tweak_id, action_type, original_value, new_value, timestamp, rolled_back
             FROM ledger ORDER BY id DESC",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let view = LedgerEntryView {
                id: row.get(0)?,
                tweak_id: row.get_ref(1)?.as_str()?,
                action_type: row.get_ref(2)?.as_str()?,
                original_value: row.get_ref(3)?.as_str()?,
                new_value: row.get_ref(4)?.as_str()?,
                timestamp_str: row.get_ref(5)?.as_str()?,
                rolled_back: row.get::<_, i32>(6)? != 0,
            };
            if !callback(view) {
                break;
            }
        }
        Ok(())
    }

    /// Zero-copy iterator over state ledger entries filtered by `tweak_id`, invoking `callback` with borrowed row views.
    ///
    /// Stops iterating when `callback` returns `false`.
    pub fn for_each_history_by_tweak<F>(&self, tweak_id: &str, mut callback: F) -> crate::core::error::Result<()>
    where
        F: FnMut(LedgerEntryView<'_>) -> bool,
    {
        let mut stmt = self.conn.prepare(
            "SELECT id, tweak_id, action_type, original_value, new_value, timestamp, rolled_back
             FROM ledger WHERE tweak_id = ?1 ORDER BY id DESC",
        )?;
        let mut rows = stmt.query(params![tweak_id])?;
        while let Some(row) = rows.next()? {
            let view = LedgerEntryView {
                id: row.get(0)?,
                tweak_id: row.get_ref(1)?.as_str()?,
                action_type: row.get_ref(2)?.as_str()?,
                original_value: row.get_ref(3)?.as_str()?,
                new_value: row.get_ref(4)?.as_str()?,
                timestamp_str: row.get_ref(5)?.as_str()?,
                rolled_back: row.get::<_, i32>(6)? != 0,
            };
            if !callback(view) {
                break;
            }
        }
        Ok(())
    }

    /// Fast lookup for the latest active (unrolled) mutation recorded for a specific tweak ID.
    pub fn find_latest_unrolled_by_tweak(&self, tweak_id: &str) -> crate::core::error::Result<Option<LedgerEntry>> {
        let mut latest = None;
        self.for_each_history_by_tweak(tweak_id, |view| {
            if !view.rolled_back {
                latest = Some(view.to_owned());
                false
            } else {
                true
            }
        })?;
        Ok(latest)
    }

    /// Retrieve the full change history, newest first.
    pub fn get_history(&self) -> crate::core::error::Result<Vec<LedgerEntry>> {
        let count = self.count_history().unwrap_or(0);
        let mut entries = Vec::with_capacity(count);
        self.for_each_history(|view| {
            entries.push(view.to_owned());
            true
        })?;
        Ok(entries)
    }

    /// Mark a ledger entry as rolled back.
    ///
    /// The caller is responsible for actually reverting the system state;
    /// this method only updates the ledger record.
    pub fn rollback_entry(&self, entry_id: i64) -> crate::core::error::Result<()> {
        self.conn.execute(
            "UPDATE ledger SET rolled_back = 1 WHERE id = ?1",
            params![entry_id],
        )?;
        Ok(())
    }

    /// Clear all historical records from the transaction ledger.
    pub fn clear_history(&self) -> crate::core::error::Result<()> {
        self.conn.execute("DELETE FROM ledger", [])?;
        Ok(())
    }

    /// Record a system baseline for before/after comparison.
    #[allow(clippy::too_many_arguments)]
    pub fn save_baseline(
        &self,
        total_ram_mb: u64,
        used_ram_mb: u64,
        process_count: usize,
        thread_count: usize,
        timer_resolution_ns: u64,
        optimized_count: usize,
        label: &str,
    ) -> crate::core::error::Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO baselines (
                captured_at, total_ram_mb, used_ram_mb, process_count, thread_count,
                timer_resolution_ns, optimized_count, label
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                now,
                total_ram_mb as i64,
                used_ram_mb as i64,
                process_count as i64,
                thread_count as i64,
                timer_resolution_ns as i64,
                optimized_count as i64,
                label,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Clear all previous baselines and capture a fresh new initial baseline.
    #[allow(clippy::too_many_arguments)]
    pub fn reset_baseline(
        &self,
        total_ram_mb: u64,
        used_ram_mb: u64,
        process_count: usize,
        thread_count: usize,
        timer_resolution_ns: u64,
        optimized_count: usize,
        label: &str,
    ) -> crate::core::error::Result<i64> {
        self.conn.execute("DELETE FROM baselines", [])?;
        self.save_baseline(
            total_ram_mb,
            used_ram_mb,
            process_count,
            thread_count,
            timer_resolution_ns,
            optimized_count,
            label,
        )
    }

    /// Retrieve the initial (first recorded) baseline.
    pub fn get_initial_baseline(&self) -> crate::core::error::Result<Option<SystemBaseline>> {
        self.query_single_baseline("SELECT id, captured_at, total_ram_mb, used_ram_mb, process_count, thread_count, timer_resolution_ns, optimized_count, label FROM baselines ORDER BY id ASC LIMIT 1")
    }

    /// Retrieve the latest recorded baseline.
    pub fn get_latest_baseline(&self) -> crate::core::error::Result<Option<SystemBaseline>> {
        self.query_single_baseline("SELECT id, captured_at, total_ram_mb, used_ram_mb, process_count, thread_count, timer_resolution_ns, optimized_count, label FROM baselines ORDER BY id DESC LIMIT 1")
    }

    fn query_single_baseline(&self, sql: &str) -> crate::core::error::Result<Option<SystemBaseline>> {
        let mut stmt = self.conn.prepare(sql)?;
        let mut rows = stmt.query_map([], |row| {
            let ts_str: String = row.get(1)?;
            let captured_at = DateTime::parse_from_rfc3339(&ts_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            Ok(SystemBaseline {
                id: row.get(0)?,
                captured_at,
                total_ram_mb: row.get::<_, i64>(2)?.max(0) as u64,
                used_ram_mb: row.get::<_, i64>(3)?.max(0) as u64,
                process_count: row.get::<_, i64>(4)?.max(0) as usize,
                thread_count: row.get::<_, i64>(5)?.max(0) as usize,
                timer_resolution_ns: row.get::<_, i64>(6)?.max(0) as u64,
                optimized_count: row.get::<_, i64>(7)?.max(0) as usize,
                label: row.get(8)?,
            })
        })?;

        if let Some(first) = rows.next() {
            Ok(Some(first?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ledger_crud() {
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("test_ledger_{}.db", uuid::Uuid::new_v4()));

        let ledger = StateLedger::new(&db_path).expect("Failed to create ledger");
        let entry_id = ledger
            .record_change("tweak_1", "RegistryDword", "2", "38")
            .expect("Failed to record change");

        let history = ledger.get_history().expect("Failed to get history");
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, entry_id);
        assert_eq!(history[0].tweak_id, "tweak_1");
        assert_eq!(history[0].original_value, "2");
        assert_eq!(history[0].new_value, "38");
        assert!(!history[0].rolled_back);

        ledger.rollback_entry(entry_id).expect("Failed to rollback");
        let updated_history = ledger.get_history().expect("Failed to get updated history");
        assert!(updated_history[0].rolled_back);

        // Test baseline CRUD
        let baseline_id = ledger
            .save_baseline(32768, 8192, 180, 2400, 15_625_000, 0, "Initial Pre-Optimization")
            .expect("Failed to save baseline");
        assert!(baseline_id > 0);

        let initial = ledger.get_initial_baseline().expect("Failed to get initial baseline");
        assert!(initial.is_some());
        let b = initial.unwrap();
        assert_eq!(b.used_ram_mb, 8192);
        assert_eq!(b.process_count, 180);
        assert_eq!(b.label, "Initial Pre-Optimization");

        // Test baseline reset
        let reset_id = ledger
            .reset_baseline(32768, 7168, 160, 2100, 500_000, 15, "Post-Optimization Baseline")
            .expect("Failed to reset baseline");
        assert!(reset_id > 0);

        let new_initial = ledger.get_initial_baseline().expect("Failed to get new baseline").unwrap();
        assert_eq!(new_initial.used_ram_mb, 7168);
        assert_eq!(new_initial.process_count, 160);
        assert_eq!(new_initial.timer_resolution_ns, 500_000);
        assert_eq!(new_initial.optimized_count, 15);
        assert_eq!(new_initial.label, "Post-Optimization Baseline");

        // Test is_reversible coverage
        let e_appx = ledger.record_change("appx", "AppXPackage:Microsoft.BingWeather_8wekyb3d8bbwe", "installed", "removed").unwrap();
        let e_startup = ledger.record_change("st", "Startup:OneDrive", "enabled", "disabled").unwrap();
        let e_approved = ledger.record_change("sta", "StartupApproved:HKCU|Run|OneDrive", "true", "false").unwrap();
        let hist = ledger.get_history().unwrap();
        assert!(!hist.iter().find(|e| e.id == e_appx).unwrap().is_reversible());
        assert!(!hist.iter().find(|e| e.id == e_startup).unwrap().is_reversible());
        assert!(hist.iter().find(|e| e.id == e_approved).unwrap().is_reversible());

        // Test zero-copy methods
        assert_eq!(ledger.count_history().unwrap(), 4);
        let mut count = 0;
        let mut found_approved = false;
        ledger.for_each_history(|view| {
            count += 1;
            if view.tweak_id == "sta" {
                found_approved = true;
                assert!(view.is_reversible());
                assert_eq!(view.original_value, "true");
                assert_eq!(view.new_value, "false");
            }
            true
        }).unwrap();
        assert_eq!(count, 4);
        assert!(found_approved);

        let latest_sta = ledger.find_latest_unrolled_by_tweak("sta").unwrap();
        assert!(latest_sta.is_some());
        assert_eq!(latest_sta.unwrap().id, e_approved);

        let _ = std::fs::remove_file(db_path);
    }
}
