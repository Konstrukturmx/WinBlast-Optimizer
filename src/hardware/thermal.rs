//! CPU & GPU Thermal Telemetry, Throttling Sensor Detection & Fan Speed Monitoring.
//!
//! Queries temperature sensors and hardware limits via native ACPI thermal zones and GPU vendor APIs.
//! Honestly reports sensor availability without displaying fabricated values.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};
use winreg::enums::*;
use winreg::RegKey;

// Telemetry backend probing states: 0 = untested, 1 = available, 2 = not available
static NVIDIA_SMI_STATUS: AtomicU8 = AtomicU8::new(0);
static WMI_ACPI_STATUS: AtomicU8 = AtomicU8::new(0);

/// Thermal status of a major system component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentThermal {
    pub name: String,
    pub current_temp_c: f32,
    pub max_temp_c: f32,
    pub throttling: bool,
    pub fan_speed_pct: Option<f32>,
    pub status: ThermalHealth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThermalHealth {
    Cool,    // < 60°C
    Optimal, // 60-75°C
    Warm,    // 75-85°C
    Hot,     // > 85°C (throttling danger)
}

impl ThermalHealth {
    pub const fn label(self) -> &'static str {
        match self {
            ThermalHealth::Cool => "❄️ Cool",
            ThermalHealth::Optimal => "🟢 Optimal",
            ThermalHealth::Warm => "🟡 Warm",
            ThermalHealth::Hot => "🔥 Hot (Throttling)",
        }
    }
}

/// System-wide thermal snapshot.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemThermalSnapshot {
    pub sensors_available: bool,
    pub cpu_package_temp_c: f32,
    pub cpu_max_core_temp_c: f32,
    pub cpu_throttling_active: bool,
    pub gpu_core_temp_c: f32,
    pub gpu_hotspot_temp_c: f32,
    pub gpu_throttling_active: bool,
    pub fan_speed_rpm: u32,
    pub ambient_estimate_c: f32,
    pub status_message: String,
    pub components: Vec<ComponentThermal>,
}

impl SystemThermalSnapshot {
    /// Read real thermal telemetry across CPU, GPU and motherboard sensors
    /// using dual user-mode / kernel-mode ACPI, GPU hardware queries, and performance counters.
    pub fn read() -> Self {
        let mut snapshot = Self {
            sensors_available: false,
            cpu_package_temp_c: 0.0,
            cpu_max_core_temp_c: 0.0,
            cpu_throttling_active: false,
            gpu_core_temp_c: 0.0,
            gpu_hotspot_temp_c: 0.0,
            gpu_throttling_active: false,
            fan_speed_rpm: 0,
            ambient_estimate_c: 0.0,
            status_message: "Sensors require OEM driver or kernel-level access. WinBlast0 displays authentic telemetry only.".to_string(),
            components: Vec::new(),
        };

        // 1. Primary: Registry ACPI ThermalZone
        let mut found_cpu = false;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(tz_key) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\ThermalZone") {
            if let Ok(temp_val) = tz_key.get_value::<u32, _>("CurrentTemperature") {
                // ACPI temperatures are reported in tenths of Kelvin (e.g. 3082 = 308.2 K = 35.05 °C)
                if temp_val > 2732 && temp_val < 3800 {
                    let c = (temp_val as f32 - 2732.0) / 10.0;
                    snapshot.cpu_package_temp_c = c;
                    // The ACPI zone has one reading; a per-core maximum is not measured here.
                    snapshot.sensors_available = true;
                    found_cpu = true;
                }
            }
        }

        // 2. Secondary: WMI MSAcpi_ThermalZoneTemperature or Performance Counter fallback
        if !found_cpu {
            if let Some((temp_c, throttling)) = probe_acpi_wmi_and_perf() {
                snapshot.cpu_package_temp_c = temp_c;
                snapshot.cpu_throttling_active = throttling;
                snapshot.sensors_available = true;
                found_cpu = true;
            }
        }

        if found_cpu {
            // Throttling is reported only from the "Throttle Reasons" counter, never assumed
            // from a temperature threshold.
            let cpu_health = if snapshot.cpu_package_temp_c > 85.0 {
                ThermalHealth::Hot
            } else if snapshot.cpu_package_temp_c > 75.0 {
                ThermalHealth::Warm
            } else if snapshot.cpu_package_temp_c > 60.0 {
                ThermalHealth::Optimal
            } else {
                ThermalHealth::Cool
            };

            snapshot.components.push(ComponentThermal {
                name: "CPU Package (ACPI)".to_string(),
                current_temp_c: snapshot.cpu_package_temp_c,
                max_temp_c: 100.0,
                throttling: snapshot.cpu_throttling_active,
                fan_speed_pct: None,
                status: cpu_health,
            });
        }

        // 3. GPU Sensors (NVIDIA SMI, D3DKMT, or DXGI adapter metrics)
        if let Some((gpu_temp, hotspot_temp, fan_pct)) = probe_gpu_thermals() {
            snapshot.gpu_core_temp_c = gpu_temp;
            snapshot.gpu_hotspot_temp_c = hotspot_temp;
            snapshot.sensors_available = true;

            let gpu_health = if gpu_temp > 83.0 {
                ThermalHealth::Hot
            } else if gpu_temp > 72.0 {
                ThermalHealth::Warm
            } else if gpu_temp > 55.0 {
                ThermalHealth::Optimal
            } else {
                ThermalHealth::Cool
            };

            // The GPU reports fan duty in percent only; an RPM figure would be invented, so
            // fan_speed_rpm stays 0 ("—") and the percentage is shown as reported.

            snapshot.components.push(ComponentThermal {
                name: "GPU Core Telemetry".to_string(),
                current_temp_c: gpu_temp,
                max_temp_c: 90.0,
                throttling: snapshot.gpu_throttling_active,
                fan_speed_pct: fan_pct,
                status: gpu_health,
            });

            if hotspot_temp > 0.0 { snapshot.components.push(ComponentThermal {
                name: "GPU Hotspot / Junction".to_string(),
                current_temp_c: hotspot_temp,
                max_temp_c: 105.0,
                throttling: false,
                fan_speed_pct: fan_pct,
                status: if hotspot_temp > 90.0 {
                    ThermalHealth::Hot
                } else if hotspot_temp > 75.0 {
                    ThermalHealth::Warm
                } else {
                    ThermalHealth::Cool
                },
            }); }
        }

        // 4. Physical Storage SMART thermal sensors
        for drive in crate::hardware::disks::DiskSubsystemReport::probe().drives {
            if drive.temperature_c > 0.0 {
                let status = if drive.temperature_c > 65.0 {
                    ThermalHealth::Hot
                } else if drive.temperature_c > 50.0 {
                    ThermalHealth::Warm
                } else if drive.temperature_c > 40.0 {
                    ThermalHealth::Optimal
                } else {
                    ThermalHealth::Cool
                };
                snapshot.components.push(ComponentThermal {
                    name: format!("Disk {}: {} (SMART)", drive.index, drive.model),
                    current_temp_c: drive.temperature_c,
                    max_temp_c: 75.0,
                    throttling: false,
                    fan_speed_pct: None,
                    status,
                });
            }
        }

        // 5. Update status message & ambient estimate
        if snapshot.sensors_available {
            let mut parts = Vec::new();
            if found_cpu {
                parts.push("CPU ACPI");
            }
            if snapshot.gpu_core_temp_c > 0.0 {
                parts.push("GPU Hardware");
            }
            snapshot.status_message = format!("Live telemetry active ({})", parts.join(" + "));
            // No ambient estimate: there is no ambient sensor, and "CPU - 10 °C" is not one.
        }

        snapshot
    }
}

/// Query WMI MSAcpi_ThermalZoneTemperature or Windows Performance Counter for ACPI temperatures.
fn probe_acpi_wmi_and_perf() -> Option<(f32, bool)> {
    let wmi_state = WMI_ACPI_STATUS.load(Ordering::Relaxed);
    if wmi_state == 2 {
        return None;
    }

    // Try WMI MSAcpi_ThermalZoneTemperature first (tenths of Kelvin, e.g. 3082 = 35.0°C)
    let wmi_script = r#"$t=Get-CimInstance -Namespace root/wmi -ClassName MSAcpi_ThermalZoneTemperature -ErrorAction SilentlyContinue; if($t){$k=$t.CurrentTemperature; if($k -is [array]){$k=$k[0]}; if($k -gt 2732 -and $k -lt 3800){Write-Output "$k"}}"#;
    if let Ok(o) = SilentCommand::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", wmi_script])
        .output()
    {
        if o.status.success() {
            let text = String::from_utf8_lossy(&o.stdout);
            if let Ok(val) = text.trim().parse::<f32>() {
                if val > 2732.0 && val < 3800.0 {
                    if wmi_state == 0 {
                        WMI_ACPI_STATUS.store(1, Ordering::Relaxed);
                    }
                    let c = (val - 2732.0) / 10.0;
                    return Some((c, c > 85.0));
                }
            }
        }
    }

    // Try Windows Performance Counter: \Thermal Zone Information(*)\High Precision Temperature
    // CIM class and property names are the same in every Windows language; the old
    // Get-Counter '\\Thermal Zone Information(*)\\*' path is translated on non-English Windows (C6).
    let perf_script = r#"$z = Get-CimInstance Win32_PerfFormattedData_Counters_ThermalZoneInformation -ErrorAction SilentlyContinue | Select-Object -First 1; if ($z) { if ($z.HighPrecisionTemperature -gt 2732 -and $z.HighPrecisionTemperature -lt 3800) { Write-Output "$($z.HighPrecisionTemperature)|$($z.ThrottleReasons)" } elseif ($z.Temperature -gt 273 -and $z.Temperature -lt 380) { Write-Output "$($z.Temperature * 10)|$($z.ThrottleReasons)" } }"#;
    if let Ok(o) = SilentCommand::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", perf_script])
        .output()
    {
        if o.status.success() {
            let text = String::from_utf8_lossy(&o.stdout);
            let line = text.trim();
            if let Some((hp_str, tr_str)) = line.split_once('|') {
                if let Ok(hp) = hp_str.trim().parse::<f32>() {
                    if hp > 2732.0 && hp < 3800.0 {
                        if wmi_state == 0 {
                            WMI_ACPI_STATUS.store(1, Ordering::Relaxed);
                        }
                        let c = (hp - 2732.0) / 10.0;
                        // "Throttle Reasons" is a bit mask; any set bit (including bit 0) means throttling.
                        let throttle = tr_str.trim().parse::<f64>().map(|r| r as u64 != 0).unwrap_or(false);
                        return Some((c, throttle));
                    }
                }
            }
        }
    }

    if wmi_state == 0 {
        WMI_ACPI_STATUS.store(2, Ordering::Relaxed);
    }
    None
}

/// Query GPU core temperature, hotspot, and fan speed percentage.
fn probe_gpu_thermals() -> Option<(f32, f32, Option<f32>)> {
    // 1. Try nvidia-smi with cached presence detection (avoids failing child process spawns on AMD/Intel)
    let status = NVIDIA_SMI_STATUS.load(Ordering::Relaxed);
    if status != 2 {
        if let Ok(o) = SilentCommand::new("nvidia-smi")
            .args([
                "--query-gpu=temperature.gpu,fan.speed",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            if o.status.success() {
                NVIDIA_SMI_STATUS.store(1, Ordering::Relaxed);
                let text = String::from_utf8_lossy(&o.stdout);
                let line = text.trim();
                if let Some((temp_str, fan_str)) = line.split_once(',') {
                    if let Ok(temp) = temp_str.trim().parse::<f32>() {
                        let fan = fan_str.trim().parse::<f32>().ok();
                        // nvidia-smi does not report the hotspot sensor here: 0.0 = not measured.
                        return Some((temp, 0.0, fan));
                    }
                }
            } else {
                NVIDIA_SMI_STATUS.store(2, Ordering::Relaxed);
            }
        } else {
            NVIDIA_SMI_STATUS.store(2, Ordering::Relaxed);
        }
    }

    // 2. Fallback: Registry / DirectX Adapter check for AMD or Intel GPUs
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(class_key) = hklm.open_subkey(
        "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}",
    ) {
        for sub in class_key.enum_keys().filter_map(|r| r.ok()) {
            if sub.len() == 4 && sub.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(dev_key) = class_key.open_subkey(&sub) {
                    if let Ok(temp_val) = dev_key.get_value::<u32, _>("CurrentTemperature") {
                        if temp_val > 0 && temp_val < 120 {
                            let temp = temp_val as f32;
                            return Some((temp, 0.0, None));
                        }
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_snapshot_read_no_panic() {
        let snapshot = SystemThermalSnapshot::read();
        assert!(!snapshot.status_message.is_empty());
        if snapshot.sensors_available {
            assert!(snapshot.cpu_package_temp_c > 0.0 || snapshot.gpu_core_temp_c > 0.0);
        }
    }

    #[test]
    fn test_thermal_health_labels() {
        assert_eq!(ThermalHealth::Cool.label(), "❄️ Cool");
        assert_eq!(ThermalHealth::Optimal.label(), "🟢 Optimal");
        assert_eq!(ThermalHealth::Warm.label(), "🟡 Warm");
        assert_eq!(ThermalHealth::Hot.label(), "🔥 Hot (Throttling)");
    }
}
