//! CPU Core Parking Detection & Tuning Engine.
//!
//! Provides inspection of Windows power scheme core parking thresholds
//! (`CPMINCORES`, `CPMAXCORES`, and Class 1 P-core `CPMINCORES1`/`CPMAXCORES1`),
//! live logical processor status visualization via NT/PDH telemetry,
//! and atomic unparking/snapshot rollback via `powrprof.dll`.

use serde::{Deserialize, Serialize};

use crate::core::backend::is_unsupported_power_guid_error;
use crate::core::error::Result;
use crate::hardware::monitor;
use crate::hardware::power;

pub const SUB_PROCESSOR: &str = "54533251-82be-4824-96c1-47b60b740d00";
pub const CPMINCORES: &str = "0cc5b647-c1df-4637-891a-dec35c318583";
pub const CPMAXCORES: &str = "ea062031-0e34-4ff1-9b6d-eb1059334028";
pub const CPMINCORES1: &str = "0cc5b647-c1df-4637-891a-dec35c318584";
pub const CPMAXCORES1: &str = "ea062031-0e34-4ff1-9b6d-eb1059334029";

/// Status of an individual logical CPU core.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStatus {
    pub core_index: usize,
    pub is_parked: bool,
    pub usage_pct: f32,
    pub core_type_label: String, // e.g. "P-Core", "E-Core", or "Core"
}

/// Global CPU Core Parking overview.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CoreParkingState {
    pub total_cores: usize,
    pub unparked_count: usize,
    pub parked_count: usize,
    pub min_cores_ac_pct: u32,
    pub min_cores_dc_pct: u32,
    pub max_cores_ac_pct: u32,
    pub is_fully_unparked: bool,
    pub is_hybrid_architecture: bool,
    pub cores: Vec<CoreStatus>,
    #[serde(default)]
    pub on_ac_power: bool,
    #[serde(default)]
    pub max_cores_dc_pct: u32,
    #[serde(default)]
    pub pcore_min_ac_pct: Option<u32>,
    #[serde(default)]
    pub pcore_min_dc_pct: Option<u32>,
    #[serde(default)]
    pub pcore_max_ac_pct: Option<u32>,
    #[serde(default)]
    pub pcore_max_dc_pct: Option<u32>,
}

/// Exact serializable snapshot of all active power scheme core parking thresholds
/// for lossless `StateLedger` rollback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParkingSnapshot {
    pub min_ac: u32,
    pub min_dc: u32,
    pub max_ac: u32,
    pub max_dc: u32,
    pub min1: Option<(u32, u32)>,
    pub max1: Option<(u32, u32)>,
}

impl ParkingSnapshot {
    /// Serialize this snapshot into JSON for ledger storage.
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(
                "{{\"min_ac\":{},\"min_dc\":{},\"max_ac\":{},\"max_dc\":{},\"min1\":null,\"max1\":null}}",
                self.min_ac, self.min_dc, self.max_ac, self.max_dc
            )
        })
    }

    /// Deserialize a previously stored JSON snapshot (returning `None` for legacy strings).
    pub fn deserialize(raw: &str) -> Option<Self> {
        serde_json::from_str::<Self>(raw.trim()).ok()
    }
}

/// Evaluate whether the active power rail (`AC` when `on_ac`, `DC` otherwise) keeps all
/// cores unparked (`min >= 100 && max >= 100`), including Class 1 P-core bounds on hybrid CPUs.
#[allow(clippy::too_many_arguments)]
pub(crate) fn compute_is_fully_unparked(
    on_ac: bool,
    is_hybrid: bool,
    min_ac: u32,
    min_dc: u32,
    max_ac: u32,
    max_dc: u32,
    min1: Option<(u32, u32)>,
    max1: Option<(u32, u32)>,
) -> bool {
    let (active_min, active_max) = if on_ac {
        (min_ac, max_ac)
    } else {
        (min_dc, max_dc)
    };
    if active_min < 100 || active_max < 100 {
        return false;
    }
    if is_hybrid {
        if let Some((ac1, dc1)) = min1 {
            if (if on_ac { ac1 } else { dc1 }) < 100 {
                return false;
            }
        }
        if let Some((ac1, dc1)) = max1 {
            if (if on_ac { ac1 } else { dc1 }) < 100 {
                return false;
            }
        }
    }
    true
}

impl CoreParkingState {
    /// Detect the current core parking configuration and live kernel parking statuses
    /// without sleeping (safe for UI startup and unit tests).
    pub fn detect() -> Self {
        Self::detect_internal(false)
    }

    /// Detect core parking state on a background worker, performing a single ~100ms
    /// NT/PDH sample so per-core CPU utilization and parking states are measured live.
    pub fn detect_sampled() -> Self {
        Self::detect_internal(true)
    }

    fn detect_internal(sample_usage: bool) -> Self {
        let on_ac = power::is_on_ac_power();
        let (min_ac, min_dc) =
            power::read_power_setting_index(None, SUB_PROCESSOR, CPMINCORES).unwrap_or((100, 10));
        let (max_ac, max_dc) =
            power::read_power_setting_index(None, SUB_PROCESSOR, CPMAXCORES).unwrap_or((100, 100));
        let min1 = power::read_power_setting_index(None, SUB_PROCESSOR, CPMINCORES1).ok();
        let max1 = power::read_power_setting_index(None, SUB_PROCESSOR, CPMAXCORES1).ok();

        let (core_usages, core_parked) = monitor::sample_live_cpu_and_parking(sample_usage);
        let total_cores = core_usages.len().max(core_parked.len()).max(1);

        // Use true hardware EfficiencyClass from Win32 GetLogicalProcessorInformationEx
        let topo = crate::hardware::cpu::query_win32_core_topology(total_cores);
        let is_hybrid = topo.as_ref().is_some_and(|t| t.has_hybrid);

        let is_fully_unparked =
            compute_is_fully_unparked(on_ac, is_hybrid, min_ac, min_dc, max_ac, max_dc, min1, max1);

        let mut cores = Vec::with_capacity(total_cores);
        let mut unparked_count = 0usize;
        let mut parked_count = 0usize;

        for i in 0..total_cores {
            let raw_usage = core_usages.get(i).copied().unwrap_or(0.0);
            let usage = if raw_usage.is_finite() {
                raw_usage.clamp(0.0, 100.0)
            } else {
                0.0
            };
            let is_parked = core_parked.get(i).copied().unwrap_or(false);

            if is_parked {
                parked_count += 1;
            } else {
                unparked_count += 1;
            }

            let core_type_label = topo
                .as_ref()
                .and_then(|t| t.logical_core_types.get(i).copied())
                .unwrap_or("Core")
                .to_string();

            cores.push(CoreStatus {
                core_index: i,
                is_parked,
                usage_pct: usage,
                core_type_label,
            });
        }

        Self {
            total_cores,
            unparked_count,
            parked_count,
            min_cores_ac_pct: min_ac,
            min_cores_dc_pct: min_dc,
            max_cores_ac_pct: max_ac,
            is_fully_unparked,
            is_hybrid_architecture: is_hybrid,
            cores,
            on_ac_power: on_ac,
            max_cores_dc_pct: max_dc,
            pcore_min_ac_pct: min1.map(|(ac, _)| ac),
            pcore_min_dc_pct: min1.map(|(_, dc)| dc),
            pcore_max_ac_pct: max1.map(|(ac, _)| ac),
            pcore_max_dc_pct: max1.map(|(_, dc)| dc),
        }
    }
}

/// Capture all active core parking thresholds (`CPMINCORES`, `CPMAXCORES`, and optional Class 1).
pub fn capture_parking_snapshot() -> ParkingSnapshot {
    let (min_ac, min_dc) =
        power::read_power_setting_index(None, SUB_PROCESSOR, CPMINCORES).unwrap_or((100, 10));
    let (max_ac, max_dc) =
        power::read_power_setting_index(None, SUB_PROCESSOR, CPMAXCORES).unwrap_or((100, 100));
    let min1 = power::read_power_setting_index(None, SUB_PROCESSOR, CPMINCORES1).ok();
    let max1 = power::read_power_setting_index(None, SUB_PROCESSOR, CPMAXCORES1).ok();
    ParkingSnapshot {
        min_ac,
        min_dc,
        max_ac,
        max_dc,
        min1,
        max1,
    }
}

fn write_parking_thresholds(snap: &ParkingSnapshot, unhide: bool) -> Result<()> {
    let prev_min = power::read_power_setting_index(None, SUB_PROCESSOR, CPMINCORES).ok();
    power::write_power_setting_index(
        None,
        SUB_PROCESSOR,
        CPMINCORES,
        Some(snap.min_ac),
        Some(snap.min_dc),
        unhide,
    )?;

    if let Err(err) = power::write_power_setting_index(
        None,
        SUB_PROCESSOR,
        CPMAXCORES,
        Some(snap.max_ac),
        Some(snap.max_dc),
        unhide,
    ) {
        if let Some((prev_ac, prev_dc)) = prev_min {
            let _ = power::write_power_setting_index(
                None,
                SUB_PROCESSOR,
                CPMINCORES,
                Some(prev_ac),
                Some(prev_dc),
                false,
            );
        }
        return Err(err);
    }

    for (guid, val_opt) in [(CPMINCORES1, snap.min1), (CPMAXCORES1, snap.max1)] {
        if let Some((ac, dc)) = val_opt {
            if let Err(err) = power::write_power_setting_index(
                None,
                SUB_PROCESSOR,
                guid,
                Some(ac),
                Some(dc),
                unhide,
            ) {
                if !is_unsupported_power_guid_error(&err) {
                    tracing::warn!(
                        "Unexpected error configuring Class 1 core parking setting {}: {}",
                        guid,
                        err
                    );
                }
            }
        }
    }

    Ok(())
}

/// Apply an exact [`ParkingSnapshot`] back to the active Windows power scheme.
pub fn apply_parking_snapshot(snap: &ParkingSnapshot) -> Result<()> {
    write_parking_thresholds(snap, false)?;
    tracing::info!(
        "Applied core parking snapshot (min={}/{}, max={}/{})",
        snap.min_ac,
        snap.min_dc,
        snap.max_ac,
        snap.max_dc
    );
    Ok(())
}

/// Unpark all CPU cores: sets `CPMINCORES`/`CPMAXCORES` (and Class 1 `CPMINCORES1`/`CPMAXCORES1`
/// if supported) to 100% across AC and DC profiles natively via `powrprof.dll`, unhides Core Parking
/// in Power Options GUI, and rolls back `CPMINCORES` if `CPMAXCORES` fails.
pub fn unpark_all_cores() -> Result<()> {
    let snap = ParkingSnapshot {
        min_ac: 100,
        min_dc: 100,
        max_ac: 100,
        max_dc: 100,
        min1: Some((100, 100)),
        max1: Some((100, 100)),
    };
    write_parking_thresholds(&snap, true)?;
    tracing::info!("All CPU logical cores successfully unparked (100% min/max AC & DC)");
    Ok(())
}

/// Restore stock Windows core parking defaults (`CPMINCORES` 10% AC / 5% DC,
/// `CPMAXCORES` 100%/100%, and Class 1 defaults when supported).
pub fn restore_stock_parking() -> Result<()> {
    let snap = ParkingSnapshot {
        min_ac: 10,
        min_dc: 5,
        max_ac: 100,
        max_dc: 100,
        min1: Some((10, 5)),
        max1: Some((100, 100)),
    };
    write_parking_thresholds(&snap, false)?;
    tracing::info!("Stock Windows CPU core parking thresholds restored");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_parking_detect() {
        let t0 = std::time::Instant::now();
        let state = CoreParkingState::detect();
        assert!(
            t0.elapsed() < std::time::Duration::from_secs(15),
            "detect took too long: {:?}",
            t0.elapsed()
        );
        assert!(state.total_cores >= 1);
        assert_eq!(state.cores.len(), state.total_cores);
        assert_eq!(state.unparked_count + state.parked_count, state.total_cores);
        for c in &state.cores {
            assert!(
                matches!(c.core_type_label.as_str(), "Core" | "P-Core" | "E-Core"),
                "Unexpected core_type_label: {:?}",
                c.core_type_label
            );
        }
    }

    #[test]
    fn test_parking_snapshot_roundtrip_and_active_rail_logic() {
        let snap = ParkingSnapshot {
            min_ac: 0,
            min_dc: 100,
            max_ac: 100,
            max_dc: 100,
            min1: Some((50, 10)),
            max1: Some((100, 100)),
        };
        let encoded = snap.serialize();
        let decoded = ParkingSnapshot::deserialize(&encoded).expect("Snapshot must deserialize");
        assert_eq!(decoded, snap);
        assert!(ParkingSnapshot::deserialize("unparked").is_none());
        assert!(ParkingSnapshot::deserialize("parked").is_none());

        // On AC when min_ac=0 and min_dc=100 -> must NOT be considered unparked!
        assert!(!compute_is_fully_unparked(
            true, false, 0, 100, 100, 100, None, None
        ));
        // On DC when min_ac=100 and min_dc=5 -> must NOT be considered unparked!
        assert!(!compute_is_fully_unparked(
            false, false, 100, 5, 100, 100, None, None
        ));
        // When max_ac < 100 -> must NOT be considered unparked!
        assert!(!compute_is_fully_unparked(
            true, false, 100, 100, 50, 100, None, None
        ));
        // Hybrid CPU with Class 1 min1 < 100 -> must NOT be considered unparked!
        assert!(!compute_is_fully_unparked(
            true,
            true,
            100,
            100,
            100,
            100,
            Some((50, 100)),
            Some((100, 100))
        ));
        // Hybrid CPU with all active-rail min/max >= 100 -> unparked!
        assert!(compute_is_fully_unparked(
            true,
            true,
            100,
            5,
            100,
            100,
            Some((100, 5)),
            Some((100, 100))
        ));
    }
}
