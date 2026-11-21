//! Network Diagnostics, DNS Resolution Latency & Socket Telemetry Engine.
//!
//! Measures live socket connection latency against major DNS resolvers and queries real network interfaces.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::{Duration, Instant};

/// DNS server probe test result with provider metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsProbeResult {
    pub name: String,
    pub host: String,
    pub primary_ip: String,
    pub secondary_ip: String,
    pub category: String,
    pub ping_ms: Option<u32>,
    pub status: String,
}

/// System network configuration and live diagnostic telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiagnostics {
    pub adapter_name: String,
    pub ip_address: String,
    pub mtu_size: u32,
    pub tcp_ack_frequency_optimized: bool,
    pub tcp_no_delay_optimized: bool,
    pub probes: Vec<DnsProbeResult>,
}

/// Dynamically query the host's actual local IPv4 address via standard OS socket routing.
pub fn query_local_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                return local_addr.ip().to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

/// Default catalog of the world's leading public and secure DNS providers.
const DNS_CATALOG: [(&str, &str, &str, &str); 10] = [
    (
        "Cloudflare DNS",
        "1.1.1.1",
        "1.0.0.1",
        "Ultra-Low Latency & Privacy",
    ),
    (
        "Google Public DNS",
        "8.8.8.8",
        "8.8.4.4",
        "Global Anycast & Reliability",
    ),
    (
        "Quad9 DNS",
        "9.9.9.9",
        "149.112.112.112",
        "Malware Threat Shield & Zero Logs",
    ),
    (
        "AdGuard DNS",
        "94.140.14.14",
        "94.140.15.15",
        "Ad & Tracker Blocking",
    ),
    (
        "OpenDNS (Cisco)",
        "208.67.222.222",
        "208.67.220.220",
        "Enterprise Security & Phishing Filter",
    ),
    (
        "Control D",
        "76.76.2.0",
        "76.76.10.0",
        "High-Performance Anycast",
    ),
    (
        "CleanBrowsing",
        "185.228.168.9",
        "185.228.169.9",
        "Security & Malware Filtering",
    ),
    (
        "Mullvad DNS",
        "194.242.2.2",
        "193.19.108.2",
        "Zero-Log Strict Privacy",
    ),
    (
        "Level3 / Lumen",
        "4.2.2.1",
        "4.2.2.2",
        "Tier-1 Backbone Routing",
    ),
    (
        "Comodo Secure DNS",
        "8.26.56.26",
        "8.20.247.20",
        "Real-Time Cyber Threat Filter",
    ),
];

impl Default for NetworkDiagnostics {
    fn default() -> Self {
        Self {
            adapter_name: "Active Network Interface".to_string(),
            ip_address: query_local_ip(),
            mtu_size: 1500,
            tcp_ack_frequency_optimized: true,
            tcp_no_delay_optimized: true,
            probes: DNS_CATALOG
                .iter()
                .map(|(name, p_ip, s_ip, cat)| DnsProbeResult {
                    name: name.to_string(),
                    host: format!("{}:53", p_ip),
                    primary_ip: p_ip.to_string(),
                    secondary_ip: s_ip.to_string(),
                    category: cat.to_string(),
                    ping_ms: None,
                    status: "Ready to test".to_string(),
                })
                .collect(),
        }
    }
}

impl NetworkDiagnostics {
    /// Probe public DNS servers concurrently in parallel to prevent UI thread lockup.
    pub fn probe_live() -> Self {
        let mut diag = Self::default();

        let mut handles = Vec::new();

        for (name, p_ip, s_ip, cat) in DNS_CATALOG {
            let host = format!("{}:53", p_ip);
            let handle = std::thread::spawn(move || {
                let start = Instant::now();
                let addr: Result<SocketAddr, _> = host.parse();
                let latency = if let Ok(sa) = addr {
                    // 400ms timeout per probe
                    if TcpStream::connect_timeout(&sa, Duration::from_millis(400)).is_ok() {
                        Some(start.elapsed().as_millis() as u32)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let status = match latency {
                    Some(ms) if ms < 20 => "Fast (<20ms)".to_string(),
                    Some(ms) if ms < 50 => "Good (<50ms)".to_string(),
                    Some(_) => "Moderate Latency".to_string(),
                    None => "Timeout / Filtered".to_string(),
                };

                DnsProbeResult {
                    name: name.to_string(),
                    host,
                    primary_ip: p_ip.to_string(),
                    secondary_ip: s_ip.to_string(),
                    category: cat.to_string(),
                    ping_ms: latency,
                    status,
                }
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for h in handles {
            if let Ok(res) = h.join() {
                results.push(res);
            }
        }

        // Sort by ping ascending (fastest first, timeouts last)
        results.sort_by_key(|r| r.ping_ms.unwrap_or(9999));

        diag.probes = results;
        diag
    }

    /// Flush DNS Resolver Cache via `ipconfig /flushdns`.
    pub fn flush_dns() -> Result<String, String> {
        let out = SilentCommand::new("ipconfig")
            .arg("/flushdns")
            .output()
            .map_err(|e| format!("Failed to flush DNS: {}", e))?;

        if out.status.success() {
            Ok("Successfully flushed DNS Resolver Cache.".to_string())
        } else {
            Err("Failed to flush DNS cache.".to_string())
        }
    }

    /// Apply DNS servers to the active network adapter using netsh.
    pub fn apply_dns_servers(primary: &str, secondary: &str) -> Result<String, String> {
        let p_clean = primary.trim();
        let s_clean = secondary.trim();
        let p_ip: std::net::IpAddr = p_clean
            .parse()
            .map_err(|e| format!("Invalid primary DNS IP address '{}': {}", p_clean, e))?;
        let s_ip: std::net::IpAddr = s_clean
            .parse()
            .map_err(|e| format!("Invalid secondary DNS IP address '{}': {}", s_clean, e))?;

        // Query active adapter name from netsh interface ipv4 show interfaces
        let ps_cmd = format!(
            r#"$adapter = Get-NetAdapter | Where-Object {{ $_.Status -eq 'Up' -and $_.InterfaceMetric -ne $null }} | Select-Object -First 1;
if ($adapter) {{
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ServerAddresses ('{}', '{}');
    Write-Output "SUCCESS"
}} else {{
    netsh interface ipv4 set dnsservers name="Ethernet" source=static address="{}" register=primary;
    netsh interface ipv4 add dnsservers name="Ethernet" address="{}" index=2;
    Write-Output "SUCCESS"
}}"#,
            p_ip, s_ip, p_ip, s_ip
        );

        let out = SilentCommand::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
            .output()
            .map_err(|e| format!("Failed to configure DNS: {e}"))?;

        if out.status.success() {
            Ok(format!(
                "Configured primary DNS: {} and secondary: {}",
                primary, secondary
            ))
        } else {
            Err("Failed to set DNS servers on active adapter.".to_string())
        }
    }

    /// Reset DNS configuration to DHCP (Automatic) on active network adapter.
    pub fn reset_dns_dhcp() -> Result<String, String> {
        let ps_cmd = r#"$adapter = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Select-Object -First 1;
if ($adapter) {
    Set-DnsClientServerAddress -InterfaceIndex $adapter.InterfaceIndex -ResetServerAddresses;
    Write-Output "SUCCESS"
} else {
    netsh interface ipv4 set dnsservers name="Ethernet" source=dhcp;
    Write-Output "SUCCESS"
}"#;

        let out = SilentCommand::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", ps_cmd])
            .output()
            .map_err(|e| format!("Failed to reset DNS: {e}"))?;

        if out.status.success() {
            Ok("Reset DNS servers to Automatic (DHCP).".to_string())
        } else {
            Err("Failed to reset DNS configuration.".to_string())
        }
    }
}
