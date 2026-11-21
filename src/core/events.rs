//! Unidirectional Core-to-UI Event Contract for WinBlast0.
//!
//! All asynchronous mutations, background scans, hardware telemetry polls, and
//! worker completions flow back into the UI via strongly-typed `CoreEvent` messages.

use std::time::Duration;
use crate::tools::automation::AutoTuneResult;
use crate::hardware::disks::DiskSubsystemReport;
use crate::hardware::thermal::SystemThermalSnapshot;
use crate::hardware::bios::BiosSystemInfo;

/// High-frequency telemetry tick payload
#[derive(Debug, Clone, Default)]
pub struct TelemetryTickData {
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub ram_used_gb: f32,
    pub ram_total_gb: f32,
    pub timer_res_ms: f64,
    pub dpc_latency_us: f64,
}

/// Notification toast severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastSeverity {
    Info,
    Success,
    Warning,
    Error,
}

/// User-facing toast notification
#[derive(Debug, Clone)]
pub struct ToastNotification {
    pub title: String,
    pub message: String,
    pub severity: ToastSeverity,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub enum CoreEvent {
    /// High-frequency telemetry update
    Telemetry(TelemetryTickData),

    /// Health score updated
    HealthScoreUpdated { score: u32, details: String },

    /// Background scan completed
    ScanCompleted { total_tweaks: usize, optimized: usize },

    /// AutoTune profile completed
    AutoTuneCompleted(Result<AutoTuneResult, String>),

    /// Cleaner scan/clean completed
    CleanerScanCompleted { reclaimable_bytes: u64 },
    CleanerCleanCompleted { reclaimed_bytes: u64 },

    /// Hardware async probe completed
    DiskReportLoaded(Result<DiskSubsystemReport, String>),
    ThermalSnapshotLoaded(Result<SystemThermalSnapshot, String>),
    BiosInfoLoaded(Result<BiosSystemInfo, String>),
    DriverScanCompleted(Result<usize, String>),

    /// Batch toggling finished
    StartupBatchCompleted { affected: usize, enabled: bool },
    TasksBatchCompleted { affected: usize, enabled: bool },

    /// User-facing toast
    Toast(ToastNotification),

    /// Application log line
    LogAppended { line: String },
}
