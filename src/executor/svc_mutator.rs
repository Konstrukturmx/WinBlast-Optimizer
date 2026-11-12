//! Service start-type and state controller (WOW64 64-bit view aware + SCM sync + DelayedAutostart).

use winreg::enums::*;
use winreg::RegKey;

use crate::core::error::{Result, WinBlastError};
use crate::scanner::registry_scanner::open_subkey_flags_64;
use crate::scanner::task_scanner::decode_oem_bytes;

fn sc_start_mode(start_type: u32) -> Option<&'static str> {
    match start_type {
        0 => Some("boot"),
        1 => Some("system"),
        2 => Some("auto"),
        3 => Some("demand"),
        4 => Some("disabled"),
        _ => None,
    }
}

fn validate_service_name(service_name: &str) -> Result<&str> {
    let clean = service_name.trim();
    if clean.is_empty() || clean.contains(['\\', '/', '\0']) || clean == ".." {
        return Err(WinBlastError::Service(format!(
            "Invalid Windows service name '{}'",
            service_name
        )));
    }
    Ok(clean)
}

fn format_sc_output(output: &std::process::Output) -> String {
    let stdout = decode_oem_bytes(&output.stdout);
    let stderr = decode_oem_bytes(&output.stderr);
    format!("{} {}", stdout.trim(), stderr.trim())
        .trim()
        .to_string()
}

fn output_has_sc_code(output: &std::process::Output, code: i32, combined: &str) -> bool {
    if output.status.code() == Some(code) {
        return true;
    }
    let code_str = code.to_string();
    combined
        .split(|c: char| !c.is_ascii_digit())
        .any(|tok| tok == code_str)
}

/// Set a Windows service's start type via Service Control Manager (`sc config`) + registry,
/// preserving `DelayedAutostart` across disable/restore cycles and surfacing real SCM errors.
///
/// Values: 0 = Boot, 1 = System, 2 = Automatic, 3 = Manual, 4 = Disabled.
pub fn set_service_start_type(service_name: &str, start_type: u32) -> Result<()> {
    let clean_name = validate_service_name(service_name)?;
    if start_type == 4 && crate::tools::startup::is_protected_core_service(clean_name) {
        return Err(WinBlastError::Service(format!(
            "Refused to disable protected OS critical service '{}'",
            clean_name
        )));
    }
    let base_mode = sc_start_mode(start_type).ok_or_else(|| {
        WinBlastError::Service(format!("Unsupported service start type {}", start_type))
    })?;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = format!("SYSTEM\\CurrentControlSet\\Services\\{}", clean_name);
    let key = match open_subkey_flags_64(&hklm, &path, KEY_READ | KEY_SET_VALUE) {
        Ok(k) => k,
        Err(e) if (start_type == 3 || start_type == 4) && e.kind() == std::io::ErrorKind::NotFound => {
            // Optional/uninstalled service is already harmless
            return Ok(());
        }
        Err(e) => return Err(WinBlastError::Registry(e)),
    };

    let prev_delayed: u32 = key.get_value::<u32, _>("DelayedAutostart").unwrap_or(0);
    let svc_type: u32 = key.get_value::<u32, _>("Type").unwrap_or(0);
    let is_user_service_template = (svc_type & 0x40) != 0 && (svc_type & 0x80) == 0;

    let mode = if start_type == 2 && prev_delayed == 1 {
        "delayed-auto"
    } else {
        base_mode
    };

    // Synchronize Windows Service Control Manager (SCM) + registry Start/DelayedAutostart
    let output = crate::core::process::SilentCommand::new("sc")
        .args(["config", clean_name, "start=", mode])
        .output()
        .map_err(|e| WinBlastError::Service(format!("Failed to invoke sc config {}: {}", clean_name, e)))?;

    if !output.status.success() {
        let detail = format_sc_output(&output);
        return Err(WinBlastError::Service(format!(
            "sc config {} start= {} failed: {}",
            clean_name, mode, detail
        )));
    }

    if start_type != 2 && prev_delayed == 1 {
        // Keep DelayedAutostart marker intact if sc config cleared it while disabling/setting manual
        let _ = key.set_value("DelayedAutostart", &1u32);
    }

    if !is_user_service_template {
        if start_type == 4 {
            stop_service(clean_name)?;
        } else if start_type == 2 {
            start_service(clean_name)?;
        }
    }

    Ok(())
}

/// Stop a running service via `sc stop`, tolerating Win32 1062 (`ERROR_SERVICE_NOT_ACTIVE`).
pub fn stop_service(service_name: &str) -> Result<()> {
    let clean_name = validate_service_name(service_name)?;
    if crate::tools::startup::is_protected_core_service(clean_name) {
        return Err(WinBlastError::Service(format!(
            "Refused to stop protected OS critical service '{}'",
            clean_name
        )));
    }
    let output = crate::core::process::SilentCommand::new("sc")
        .args(["stop", clean_name])
        .output()
        .map_err(|e| WinBlastError::Service(format!("Failed to invoke sc stop {}: {}", clean_name, e)))?;

    if !output.status.success() {
        let detail = format_sc_output(&output);
        // Tolerate 1062 (service has not been started)
        if !output_has_sc_code(&output, 1062, &detail) {
            return Err(WinBlastError::Service(format!(
                "sc stop {} failed: {}",
                clean_name, detail
            )));
        }
    }
    Ok(())
}

/// Start a service via `sc start`, tolerating Win32 1056 (`ERROR_SERVICE_ALREADY_RUNNING`).
pub fn start_service(service_name: &str) -> Result<()> {
    let clean_name = validate_service_name(service_name)?;
    let output = crate::core::process::SilentCommand::new("sc")
        .args(["start", clean_name])
        .output()
        .map_err(|e| WinBlastError::Service(format!("Failed to invoke sc start {}: {}", clean_name, e)))?;

    if !output.status.success() {
        let detail = format_sc_output(&output);
        // Tolerate 1056 (an instance of the service is already running)
        if !output_has_sc_code(&output, 1056, &detail) {
            return Err(WinBlastError::Service(format!(
                "sc start {} failed: {}",
                clean_name, detail
            )));
        }
    }
    Ok(())
}
