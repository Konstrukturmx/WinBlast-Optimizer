//! Hosts file, firewall rules, and third-party telemetry agent detector.
//!
//! Scans installed software alongside Windows OS components and supports `SystemBackend` injection.

use crate::core::backend::SystemBackend;

/// Known telemetry hosts to check in `C:\Windows\System32\drivers\etc\hosts`.
pub const TELEMETRY_HOSTS: &[&str] = &[
    "vortex.data.microsoft.com",
    "vortex-win.data.microsoft.com",
    "telecommand.telemetry.microsoft.com",
    "oca.telemetry.microsoft.com",
    "sqm.telemetry.microsoft.com",
    "watson.telemetry.microsoft.com",
    "settings-win.data.microsoft.com",
    "telemetry.nvidia.com",
    "gfe.nvidia.com",
    "cc-api-data.adobe.io",
    "clients2.google.com",
];

/// Known third-party and OS telemetry services (aligned with catalog service names).
pub const KNOWN_TELEMETRY_SERVICES: &[(&str, &str)] = &[
    ("DiagTrack", "Connected User Experiences and Telemetry"),
    ("dmwappushservice", "WAP Push Service"),
    ("WerSvc", "Windows Error Reporting"),
    ("NvTelemetryContainer", "NVIDIA Telemetry Container"),
    ("AdobeARMservice", "Adobe Acrobat Update Service"),
    ("AGMService", "Adobe Genuine Software Integrity Service"),
    ("Razer Game Manager Service", "Razer Synapse Telemetry"),
    ("edgeupdate", "Microsoft Edge Update Service"),
    ("gupdate", "Google Update Service"),
];

/// Result of a full telemetry audit.
#[derive(Debug, Clone)]
pub struct TelemetryAuditResult {
    /// Hosts blocked in Windows hosts file (routed to 0.0.0.0 or 127.0.0.1).
    pub blocked_hosts: Vec<String>,
    /// Telemetry hosts currently unblocked.
    pub unblocked_hosts: Vec<String>,
    /// Active telemetry services found on the machine.
    pub active_services: Vec<String>,
    /// Disabled or blocked telemetry services.
    pub disabled_services: Vec<String>,
}

/// Pure hosts file parser skipping `#` comments and extracting domains mapped to loopback
/// (supports UTF-8 BOM, IPv4 `0.0.0.0`/`127.0.0.1`, IPv6 `::1`/`::`, and space/tab separators).
pub fn parse_blocked_hosts_from_content(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            // Strip UTF-8 BOM and inline comments
            let clean_line = line
                .trim_start_matches('\u{FEFF}')
                .split('#')
                .next()
                .unwrap_or("")
                .trim();
            if clean_line.is_empty() {
                return None;
            }
            let mut parts = clean_line.split_whitespace();
            let ip = parts.next()?;
            if matches!(ip, "0.0.0.0" | "127.0.0.1" | "::1" | "::") {
                Some(parts.map(|s| s.to_string()).collect::<Vec<_>>())
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

/// Read the live Windows hosts file and return domains blocked to loopback.
pub fn get_blocked_hosts() -> Vec<String> {
    let hosts_path = r"C:\Windows\System32\drivers\etc\hosts";
    match std::fs::read_to_string(hosts_path) {
        Ok(c) => parse_blocked_hosts_from_content(&c),
        Err(_) => Vec::new(),
    }
}

/// Perform a full telemetry audit using the given backend (supports `RealBackend` and `SandboxEnv`).
pub fn full_telemetry_audit_with(backend: &dyn SystemBackend) -> TelemetryAuditResult {
    let hosts_content = backend.read_hosts_file().unwrap_or_default();
    let blocked_in_hosts = parse_blocked_hosts_from_content(&hosts_content);
    let mut blocked_hosts = Vec::new();
    let mut unblocked_hosts = Vec::new();

    for host in TELEMETRY_HOSTS {
        if blocked_in_hosts
            .iter()
            .any(|b| b.eq_ignore_ascii_case(host))
        {
            blocked_hosts.push(host.to_string());
        } else {
            unblocked_hosts.push(host.to_string());
        }
    }

    let mut active_services = Vec::new();
    let mut disabled_services = Vec::new();

    for (svc_name, display_name) in KNOWN_TELEMETRY_SERVICES {
        match backend.get_service_start_type(svc_name) {
            Some(4) => disabled_services.push(format!("{} ({})", display_name, svc_name)),
            Some(_) => active_services.push(format!("{} ({})", display_name, svc_name)),
            None => {}
        }
    }

    TelemetryAuditResult {
        blocked_hosts,
        unblocked_hosts,
        active_services,
        disabled_services,
    }
}

/// Perform a full telemetry scan across live hosts file and services.
#[allow(dead_code)]
pub fn full_telemetry_audit() -> TelemetryAuditResult {
    full_telemetry_audit_with(&crate::core::backend::RealBackend)
}

/// Legacy helper returning list of detected active telemetry agents.
#[allow(dead_code)]
pub fn scan_telemetry_agents() -> Vec<String> {
    full_telemetry_audit().active_services
}
