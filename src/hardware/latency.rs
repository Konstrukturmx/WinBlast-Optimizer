//! Timer resolution and DPC/ISR latency snapshots.
//!
//! Captures the current system timer resolution, memory utilization, and thread metrics
//! for before/after optimization comparisons.

use chrono::{DateTime, Utc};
use sysinfo::System;

/// A snapshot of system latency and execution metrics.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LatencySnapshot {
    /// Current system timer resolution in nanoseconds (100ns units from NT).
    pub timer_resolution_ns: u64,
    /// Total system RAM in Megabytes.
    pub total_ram_mb: u64,
    /// Used system RAM in Megabytes.
    pub used_ram_mb: u64,
    /// Number of active processes running.
    pub process_count: usize,
    /// Estimated active thread count.
    pub thread_count: usize,
    /// When this snapshot was taken.
    pub timestamp: DateTime<Utc>,
}

impl LatencySnapshot {
    /// Capture system metrics and timer resolution.
    pub fn capture() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_ram_mb = sys.total_memory() / (1024 * 1024);
        let used_ram_mb = sys.used_memory() / (1024 * 1024);
        let process_count = sys.processes().len();
        let thread_count = super::monitor::query_system_thread_count().unwrap_or(process_count * 12);

        // Exact system timer resolution query from NT kernel
        let timer_resolution_ns = super::monitor::query_nt_timer_resolution();

        Self {
            timer_resolution_ns,
            total_ram_mb,
            used_ram_mb,
            process_count,
            thread_count,
            timestamp: Utc::now(),
        }
    }

    /// Formatted timer resolution string in milliseconds.
    pub fn timer_resolution_display(&self) -> String {
        let ms = self.timer_resolution_ns as f64 / 1_000_000.0;
        format!("{:.3} ms", ms)
    }

    /// Formatted RAM usage string.
    pub fn ram_display(&self) -> String {
        let pct = if self.total_ram_mb > 0 {
            (self.used_ram_mb as f64 / self.total_ram_mb as f64) * 100.0
        } else {
            0.0
        };
        format!("{:.1} GB / {:.1} GB ({:.1}%)",
            self.used_ram_mb as f64 / 1024.0,
            self.total_ram_mb as f64 / 1024.0,
            pct
        )
    }
}
