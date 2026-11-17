//! Unified error handling for WinBlast0.
//!
//! All fallible operations throughout the crate return [`Result<T>`] which uses
//! [`WinBlastError`] as the error type.

/// Unified error type for every subsystem in WinBlast0.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum WinBlastError {
    /// A Windows registry operation failed with Win32 / HRESULT error code and message.
    #[error("Win32 Registry Error ({0}): {1}")]
    Registry(i32, String),

    /// A Windows service protection rule or SCM call failed with error code.
    #[error("Service Control Manager Error ({0}): {1}")]
    Service(u32, String),

    /// A COM/WMI operation failed with HRESULT.
    #[error("COM/WMI Failure (HRESULT 0x{0:08X})")]
    Com(i32),

    /// An NT Kernel Status failure occurred.
    #[error("NT Kernel Status Error (0x{0:08X})")]
    NtStatus(u32),

    /// A filesystem or generic I/O operation failed.
    #[error("I/O Failure: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid argument or input provided.
    #[error("Invalid Input: {0}")]
    InvalidInput(String),

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

impl WinBlastError {
    /// Helper to convert a `std::io::Error` from registry APIs into `WinBlastError::Registry`.
    pub fn from_reg_err(err: std::io::Error) -> Self {
        let code = err.raw_os_error().unwrap_or(0);
        let msg = err.to_string();
        Self::Registry(code, msg)
    }

    /// Helper to construct a structured `Registry` error.
    pub fn registry<S: Into<String>>(code: i32, msg: S) -> Self {
        Self::Registry(code, msg.into())
    }

    /// Helper to construct a structured `Service` error.
    pub fn service<S: Into<String>>(code: u32, msg: S) -> Self {
        Self::Service(code, msg.into())
    }

    /// Helper to construct an `InvalidInput` error.
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, WinBlastError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let reg_err = WinBlastError::Registry(2, "File not found".into());
        assert_eq!(reg_err.to_string(), "Win32 Registry Error (2): File not found");

        let svc_err = WinBlastError::Service(1060, "Service does not exist".into());
        assert_eq!(svc_err.to_string(), "Service Control Manager Error (1060): Service does not exist");

        let com_err = WinBlastError::Com(-2147467259); // 0x80004005
        assert!(com_err.to_string().contains("HRESULT 0x80004005"));

        let nt_err = WinBlastError::NtStatus(0xC0000022);
        assert!(nt_err.to_string().contains("0xC0000022"));

        let input_err = WinBlastError::InvalidInput("Malformed path".into());
        assert_eq!(input_err.to_string(), "Invalid Input: Malformed path");

        let io_err: WinBlastError = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied").into();
        assert!(matches!(io_err, WinBlastError::Io(_)));
    }

    #[test]
    fn test_error_helpers() {
        let reg = WinBlastError::registry(5, "Access is denied");
        assert!(matches!(reg, WinBlastError::Registry(5, _)));

        let svc = WinBlastError::service(1056, "Service already running");
        assert!(matches!(svc, WinBlastError::Service(1056, _)));

        let inp = WinBlastError::invalid_input("bad argument");
        assert!(matches!(inp, WinBlastError::InvalidInput(_)));

        let io = std::io::Error::from_raw_os_error(2);
        let from_io = WinBlastError::from_reg_err(io);
        assert!(matches!(from_io, WinBlastError::Registry(2, _)));
    }
}
