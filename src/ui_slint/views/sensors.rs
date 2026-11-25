//! CPU Core Parking, Thermal & Fan monitor and Disk Health binders.
//! No fabricated values: missing sensors render as "—" with the honest OEM banner.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;
use crate::hardware::tbw::HostWrites;
use crate::hardware::thermal::ThermalHealth;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{CoreCard, DiskStore, DriveCard, MainWindow, ParkingStore, ThermalRow, ThermalStore};

const DASH: &str = "—";

pub fn temp_tone(t: f32, warn: f32, hot: f32) -> &'static str {
    if t > hot {
        "danger"
    } else if t > warn {
        "warning"
    } else {
        "success"
    }
}

pub struct SensorsView {
    cores: Rc<VecModel<CoreCard>>,
    thermals: Rc<VecModel<ThermalRow>>,
    drives: Rc<VecModel<DriveCard>>,
}

impl SensorsView {
    pub fn new(window: &MainWindow) -> Self {
        let (cores, a) = new_model();
        let (thermals, b) = new_model();
        let (drives, c) = new_model();
        window.global::<ParkingStore>().set_cores(a);
        window.global::<ThermalStore>().set_components(b);
        window.global::<DiskStore>().set_drives(c);
        Self { cores, thermals, drives }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let p = window.global::<ParkingStore>();
        let c = ctx.clone();
        p.on_unpark_all(move || {
            c.act(|s| {
                if !s.core_parking_inflight && !s.is_rolling_back {
                    s.unpark_all_cores();
                }
            });
        });
        let c = ctx.clone();
        p.on_restore_stock(move || {
            c.act(|s| {
                if !s.core_parking_inflight && !s.is_rolling_back {
                    s.restore_stock_parking();
                }
            });
        });
        let c = ctx.clone();
        p.on_refresh(move || {
            c.act(|s| s.refresh_core_parking());
        });
        let c = ctx.clone();
        window.global::<ThermalStore>().on_refresh(move || {
            c.act(|s| s.scan_thermal_snapshot_async());
        });
        let d = window.global::<DiskStore>();
        let c = ctx.clone();
        d.on_refresh(move || {
            c.act(|s| s.scan_disk_health_async());
        });
        let c = ctx.clone();
        d.on_trim_all(move || {
            c.act(|s| s.run_trim_all_async());
        });
    }

    pub fn sync_parking(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<ParkingStore>();
        let cp = &s.core_parking;
        st.set_fully_unparked(cp.is_fully_unparked);
        st.set_total(cp.total_cores as i32);
        st.set_unparked(cp.unparked_count as i32);
        st.set_parked(cp.parked_count as i32);
        st.set_min_ac_dc(format!("{}% / {}%", cp.min_cores_ac_pct, cp.min_cores_dc_pct).into());
        st.set_inflight(s.core_parking_inflight);
        st.set_rolling_back(s.is_rolling_back);
        sync_vec(
            &self.cores,
            cp.cores
                .iter()
                .map(|c| {
                    let label = match c.core_type_label.as_str() {
                        "P-Core" => s.tr(TextKey::PCoreLabel),
                        "E-Core" => s.tr(TextKey::ECoreLabel),
                        _ => s.tr(TextKey::CoreLabel),
                    };
                    let u = if c.usage_pct.is_finite() { c.usage_pct.clamp(0.0, 100.0) } else { 0.0 };
                    CoreCard {
                        title: format!("{label} #{}", c.core_index).into(),
                        parked: c.is_parked,
                        usage: (u / 100.0).clamp(0.02, 1.0),
                        usage_text: format!("{u:.1}%").into(),
                    }
                })
                .collect(),
        );
    }

    pub fn sync_thermal(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<ThermalStore>();
        st.set_scanning(s.is_thermal_scanning);
        let t = s.thermal_snapshot.clone().unwrap_or_default();
        let real = t.sensors_available;
        st.set_sensors_available(real);
        let fmt = |v: f32| if real && v.is_finite() && v > 0.0 { format!("{v:.1}°C") } else { DASH.to_string() };
        let throttle = |b: bool| if b { s.tr_v(ViewTextKey::ThermalThrottlingActive) } else { s.tr_v(ViewTextKey::ThermalThrottlingNone) };
        st.set_cpu_temp(fmt(t.cpu_package_temp_c).into());
        st.set_cpu_tone(if real { temp_tone(t.cpu_package_temp_c, 75.0, 85.0) } else { "muted" }.into());
        st.set_cpu_detail(if real {
            format!("{}: {} • {}", s.tr_v(ViewTextKey::ThermalMaxCore), fmt(t.cpu_max_core_temp_c), throttle(t.cpu_throttling_active)).into()
        } else {
            SharedString::new()
        });
        st.set_gpu_temp(fmt(t.gpu_core_temp_c).into());
        st.set_gpu_tone(if real && t.gpu_core_temp_c > 0.0 { temp_tone(t.gpu_core_temp_c, 72.0, 83.0) } else { "muted" }.into());
        st.set_gpu_detail(if real && t.gpu_core_temp_c > 0.0 {
            format!("Hotspot: {} • {}", fmt(t.gpu_hotspot_temp_c), throttle(t.gpu_throttling_active)).into()
        } else {
            SharedString::new()
        });
        st.set_fan_text(if t.fan_speed_rpm > 0 { format!("{} RPM", t.fan_speed_rpm).into() } else { DASH.into() });
        st.set_fan_detail(match t.components.iter().find_map(|c| c.fan_speed_pct) {
            Some(pct) if pct.is_finite() => format!("GPU Fan PWM: {pct:.0}%").into(),
            _ => SharedString::new(),
        });
        sync_vec(
            &self.thermals,
            t.components
                .iter()
                .map(|c| ThermalRow {
                    name: c.name.as_str().into(),
                    threshold: format!("{:.0}°C", c.max_temp_c).into(),
                    temp: format!("{:.1}°C", c.current_temp_c).into(),
                    status: c.status.label().into(),
                    tone: match c.status {
                        ThermalHealth::Cool | ThermalHealth::Optimal => "success",
                        ThermalHealth::Warm => "warning",
                        ThermalHealth::Hot => "danger",
                    }
                    .into(),
                })
                .collect(),
        );
    }

    pub fn sync_disks(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<DiskStore>();
        st.set_scanning(s.is_disk_scanning);
        st.set_loaded(s.disk_report.is_some());
        st.set_trim_inflight(s.disk_trim_inflight);
        let r = s.disk_report.clone().unwrap_or_default();
        st.set_trim_active(r.global_trim_active);
        let trim = if r.global_trim_active {
            s.tr_v(ViewTextKey::DiskGlobalTrimActive)
        } else {
            s.tr_v(ViewTextKey::DiskGlobalTrimDisabled)
        };
        // v1.7.0 fix: only claim DirectStorage when the probe actually detected it.
        st.set_trim_banner(if r.nvme_direct_storage_supported {
            format!("{} • {}", trim, s.tr_v(ViewTextKey::DiskDirectStorageSupported)).into()
        } else {
            trim.into()
        });
        sync_vec(
            &self.drives,
            r.drives
                .iter()
                .map(|d| DriveCard {
                    title: format!("Disk {}: {}", d.index, d.model).into(),
                    sub: format!("{} • {:.1} GB", d.interface_type, d.total_size_gb).into(),
                    health: d.health_percentage.map_or(-1, |h| h as i32),
                    health_text: match d.health_percentage {
                        Some(h) => format!("{}: {}%", s.tr_v(ViewTextKey::DiskHealthStatus), h).into(),
                        None => format!("{}: {}", s.tr_v(ViewTextKey::DiskHealthStatus), DASH).into(),
                    },
                    status: d.status.as_str().into(),
                    writes: {
                        let value = match d.host_writes {
                            HostWrites::Reported(tb) => format!("{tb:.2} TB"),
                            HostWrites::NotAvailable => s.tr_v(ViewTextKey::DiskWritesNotAvailable).to_string(),
                            HostWrites::RequiresAdmin => s.tr_v(ViewTextKey::DiskWritesRequiresAdmin).to_string(),
                        };
                        format!("{}: {value}", s.tr_v(ViewTextKey::DiskLifetimeWrites)).into()
                    },
                    hours: format!("{}: {} h", s.tr_v(ViewTextKey::DiskPowerOnHours), d.power_on_hours).into(),
                    temp: if d.temperature_c > 0.0 { format!("{:.0}°C", d.temperature_c).into() } else { DASH.into() },
                    tone: match d.health_percentage {
                        Some(h) if h >= 90 => "success",
                        Some(h) if h >= 75 => "warning",
                        Some(_) => "danger",
                        None => "muted",
                    }
                    .into(),
                })
                .collect(),
        );
    }
}
