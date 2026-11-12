//! Unified error handling for WinBlast0.
//!
//! All fallible operations throughout the crate return [`Result<T>`] which uses
//! [`WinBlastError`] as the error type.

/// Unified error type for every subsystem in WinBlast0.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum WinBlastError {
    /// A Windows registry or filesystem I/O operation failed.
    #[error("I/O or Registry error: {0}")]
    Registry(#[from] std::io::Error),

    /// An SQLite / state-ledger operation failed.
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// A TOML rule file could not be parsed.
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    /// JSON serialization/deserialization failed.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A WMI query or COM operation failed.
    #[error("WMI error: {0}")]
    Wmi(String),

    /// The process is not running with Administrator privileges.
    #[error("Elevation required: {0}")]
    ElevationRequired(String),

    /// The current Windows build or edition is incompatible with a tweak.
    #[error("OS incompatible: {0}")]
    OsIncompatible(String),

    /// A tweak could not be applied.
    #[error("Tweak failed: {0}")]
    TweakFailed(String),

    /// A backup or restore-point operation failed.
    #[error("Backup error: {0}")]
    Backup(String),

    /// A PowerShell / DISM invocation failed.
    #[error("PowerShell error: {0}")]
    PowerShell(String),

    /// A hardware, power-policy, or Win32 inventory operation failed.
    #[error("Hardware/Power error: {0}")]
    Hardware(String),

    /// A Windows service protection rule or SCM call failed.
    #[error("Service error: {0}")]
    Service(String),

    /// An AppX/WinGet management error occurred.
    #[error("AppX/Package error: {0}")]
    AppX(String),

    /// A Scheduled Task operation failed.
    #[error("Scheduled Task error: {0}")]
    ScheduledTask(String),

    /// A ledger entry cannot be automatically rolled back.
    #[error("Rollback unavailable: {0}")]
    Rollback(String),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, WinBlastError>;
