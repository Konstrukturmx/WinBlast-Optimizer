//! Hardware overview binder: live waveforms, CPU topology + per-core load, NT latency,
//! power plans / hibernation / USB power, platform security, DIMMs, storage volumes.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{ChipItem, HardwareStore, KvRow, MainWindow, MeterCell, TitledRow};
use super::dashboard::cpu_title;

pub fn kv(k: &str, v: impl Into<SharedString>) -> KvRow {
    KvRow { k: k.into(), v: v.into() }
}

/// Localized name for the well-known Windows power plan GUIDs (v1.6.1 table).
pub fn power_plan_name<'a>(s: &'a AppState, guid: &str, friendly: &'a str) -> &'a str {
    match guid.to_ascii_lowercase().as_str() {
        "381b4222-f694-41f0-9685-ff5bb260df2e" => s.tr_v(ViewTextKey::PowerPlanBalanced),
        "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c" => s.tr_v(ViewTextKey::PowerPlanHighPerf),
        "a1841308-3541-4fab-bc81-f71556f20b4a" => s.tr_v(ViewTextKey::PowerPlanPowerSaver),
        "e9a42b02-d5df-448d-aa00-03f14749eb61" => s.tr_v(ViewTextKey::PowerPlanUltimate),
        _ if friendly.contains("WinBlast") => s.tr_v(ViewTextKey::PowerPlanWinBlastUltimate),
        _ => friendly,
    }
}

pub struct HardwareView {
    cpu_pts: Rc<VecModel<f32>>,
    ram_pts: Rc<VecModel<f32>>,
    cpu_rows: Rc<VecModel<KvRow>>,
    cores: Rc<VecModel<MeterCell>>,
    lat_rows: Rc<VecModel<KvRow>>,
    lat_snap: Rc<VecModel<KvRow>>,
    sec_rows: Rc<VecModel<KvRow>>,
    dimms: Rc<VecModel<TitledRow>>,
    volumes: Rc<VecModel<TitledRow>>,
    schemes: Rc<VecModel<ChipItem>>,
}

impl HardwareView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<HardwareStore>();
        macro_rules! m {
            ($set:ident) => {{
                let (m, rc) = new_model();
                st.$set(rc);
                m
            }};
        }
        Self {
            cpu_pts: m!(set_cpu_points),
            ram_pts: m!(set_ram_points),
            cpu_rows: m!(set_cpu_rows),
            cores: m!(set_cores),
            lat_rows: m!(set_latency_rows),
            lat_snap: m!(set_latency_snapshot_rows),
            sec_rows: m!(set_security_rows),
            dimms: m!(set_dimms),
            volumes: m!(set_volumes),
            schemes: m!(set_schemes),
        }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<HardwareStore>();
        let c = ctx.clone();
        st.on_refresh(move || {
            c.act(|s| s.refresh_platform_inventory());
        });
        let c = ctx.clone();
        st.on_refresh_latency(move || {
            c.act(|s| s.refresh_latency());
        });
        let c = ctx.clone();
        st.on_set_power_plan(move |guid| {
            c.act(|s| {
                if !s.power_scheme_inflight && !s.is_rolling_back && s.power_schemes.iter().any(|p| p.guid == guid.as_str()) {
                    s.set_active_power_plan(guid.to_string());
                }
            });
        });
        let c = ctx.clone();
        st.on_ultimate_plan(move || {
            c.act(|s| {
                if !s.power_scheme_inflight && !s.is_rolling_back {
                    s.activate_winblast_ultimate_power_plan();
                }
            });
        });
        let c = ctx.clone();
        st.on_toggle_hibernation(move |enable| {
            c.act(|s| {
                if !s.hibernation_inflight && !s.is_rolling_back {
                    s.toggle_hibernation(enable);
                }
            });
        });
        let c = ctx.clone();
        st.on_usb_power_saving(move |disable| {
            c.act(|s| {
                if !s.usb_power_inflight && !s.is_rolling_back {
                    s.toggle_usb_power_saving(disable);
                }
            });
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<HardwareStore>();
        let scanning = s.tr(TextKey::Scanning);

        // Waveforms
        match &s.realtime_metrics {
            Some(m) => {
                st.set_metrics_available(true);
                st.set_cpu_title(cpu_title(s).into());
                st.set_cpu_text(format!("{:.1}% ({} {})", m.cpu_usage, m.thread_count, s.tr(TextKey::ThreadsLabel)).into());
                st.set_ram_title(s.tr(TextKey::RamSparklineTitle).into());
                st.set_ram_text(
                    format!("{:.1} / {:.1} GB ({:.1}%)", m.used_ram_mb as f64 / 1024.0, m.total_ram_mb as f64 / 1024.0, m.ram_percentage).into(),
                );
                let tail = |v: &[f32]| v.iter().rev().take(30).rev().map(|x| if x.is_finite() { *x } else { 0.0 }).collect::<Vec<f32>>();
                sync_vec(&self.cpu_pts, tail(&s.cpu_sparkline));
                sync_vec(&self.ram_pts, tail(&s.ram_sparkline));
            }
            None => st.set_metrics_available(false),
        }

        // CPU
        let cpu_rows = match &s.cpu_info {
            Some(cpu) => {
                let topo = if cpu.has_hybrid_topology {
                    format!("{} ({}P / {}E)", s.tr(TextKey::HybridTopology), cpu.p_core_count, cpu.e_core_count)
                } else {
                    s.tr(TextKey::StandardTopology).to_string()
                };
                vec![
                    kv(s.tr(TextKey::VendorLabel), cpu.vendor.as_str()),
                    kv(s.tr(TextKey::ModelLabel), cpu.model_name.as_str()),
                    kv(s.tr(TextKey::PhysicalCoresLabel), cpu.physical_cores.to_string()),
                    kv(s.tr(TextKey::LogicalProcessorsLabel), cpu.logical_cores.to_string()),
                    kv(s.tr(TextKey::TopologyLabel), topo),
                ]
            }
            None => vec![kv(s.tr(TextKey::CpuCardTitle), scanning)],
        };
        sync_vec(&self.cpu_rows, cpu_rows);
        sync_vec(
            &self.cores,
            s.realtime_metrics
                .as_ref()
                .map(|m| {
                    m.core_usages
                        .iter()
                        .enumerate()
                        .map(|(i, u)| {
                            let u = if u.is_finite() { u.clamp(0.0, 100.0) } else { 0.0 };
                            MeterCell {
                                label: format!("C{i}: {u:.0}%").into(),
                                value: u / 100.0,
                                tone: if u > 80.0 { "danger" } else if u > 45.0 { "warning" } else { "accent" }.into(),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default(),
        );

        // Latency
        let mut lat = Vec::new();
        if let Some(m) = &s.realtime_metrics {
            lat.push(kv(s.tr(TextKey::TimerResolution), m.timer_res_ms_display()));
            lat.push(kv(s.tr(TextKey::RamUsage), m.ram_display()));
            lat.push(kv(s.tr(TextKey::BackgroundProcesses), m.process_count.to_string()));
            lat.push(kv(s.tr(TextKey::ThreadsLabel), m.thread_count.to_string()));
        }
        if let Some(b) = &s.latest_latency_bench {
            lat.push(kv(s.tr(TextKey::DpcJitterPeak), format!("{} µs ({:.3} ms)", b.dpc.peak_jitter_us, b.dpc.timer_resolution_ms)));
        }
        if s.realtime_metrics.is_none() && s.latency.is_none() {
            lat.push(kv(s.tr(TextKey::LatencyCardTitle), scanning));
        }
        sync_vec(&self.lat_rows, lat);
        match &s.latency {
            Some(snap) => {
                st.set_latency_snapshot_title(
                    format!("{} ({})", s.tr(TextKey::LatencySnapshotTitle), snap.timestamp.format("%H:%M:%S UTC")).into(),
                );
                sync_vec(
                    &self.lat_snap,
                    vec![
                        kv(s.tr(TextKey::TimerResolution), snap.timer_resolution_display()),
                        kv(s.tr(TextKey::RamUsage), snap.ram_display()),
                        kv(
                            s.tr(TextKey::BackgroundProcesses),
                            format!("{} · {} {}", snap.process_count, snap.thread_count, s.tr(TextKey::ThreadsLabel)),
                        ),
                    ],
                );
            }
            None => {
                st.set_latency_snapshot_title(SharedString::new());
                sync_vec(&self.lat_snap, Vec::new());
            }
        }

        // Platform security
        let sec_rows = match &s.platform_security {
            Some(sec) => {
                let on_off = |b: bool| if b { s.tr(TextKey::StateEnabled) } else { s.tr(TextKey::StateDisabled) };
                vec![
                    kv(s.tr(TextKey::FirmwareLabel), sec.firmware_mode.as_str()),
                    kv(
                        s.tr(TextKey::SecureBootLabel),
                        match sec.secure_boot_enabled {
                            Some(b) => on_off(b),
                            None => s.tr(TextKey::UnknownLabel),
                        },
                    ),
                    kv(s.tr(TextKey::VbsHvciLabel), format!("VBS: {} · HVCI: {}", on_off(sec.vbs_enabled), on_off(sec.hvci_enabled))),
                    kv(s.tr(TextKey::BoardBiosLabel), format!("{} ({})", sec.motherboard, sec.bios_version)),
                    kv(s.tr(TextKey::UptimeLabel), format!("{:.1} {}", sec.uptime_hours, s.tr(TextKey::HoursUnit))),
                ]
            }
            None => vec![kv(s.tr(TextKey::PlatformSecurityTitle), scanning)],
        };
        sync_vec(&self.sec_rows, sec_rows);

        // DIMMs & volumes
        st.set_dimm_fallback(if s.ram_dimms.is_empty() {
            s.platform_security.as_ref().map(|p| p.ram_summary.clone()).unwrap_or_else(|| "—".into()).into()
        } else {
            SharedString::new()
        });
        sync_vec(
            &self.dimms,
            s.ram_dimms
                .iter()
                .map(|d| TitledRow {
                    title: format!("{} · {} ({})", d.locator, d.manufacturer, d.ddr_type).into(),
                    sub: format!("{:.1} GB @ {} MT/s", d.capacity_mb as f64 / 1024.0, d.speed_mts).into(),
                    tone: SharedString::new(),
                })
                .collect(),
        );
        sync_vec(
            &self.volumes,
            s.storage_volumes
                .iter()
                .map(|v| TitledRow {
                    title: format!("{} {} ({} / {})", v.drive_letter, v.label, v.media_kind.as_label(), v.file_system).into(),
                    sub: format!(
                        "{:.0} / {:.0} GB {} ({:.0}% {})",
                        v.free_gb,
                        v.total_gb,
                        s.tr(TextKey::FreeLabel),
                        v.used_pct,
                        s.tr(TextKey::UsedLabel)
                    )
                    .into(),
                    tone: if v.used_pct >= 90.0 { "danger" } else if v.used_pct >= 75.0 { "warning" } else { "" }.into(),
                })
                .collect(),
        );

        // Power
        sync_vec(
            &self.schemes,
            s.power_schemes
                .iter()
                .map(|p| ChipItem {
                    id: p.guid.as_str().into(),
                    label: format!("{}{}", if p.is_active { "✓ " } else { "" }, power_plan_name(s, &p.guid, &p.friendly_name)).into(),
                    active: p.is_active,
                })
                .collect(),
        );
        let h = &s.hibernation_status;
        st.set_hibernation_enabled(h.hibernation_enabled);
        st.set_hibernation_note(
            format!(
                "{} ({}: {}, S0ix: {})",
                if h.hibernation_enabled { s.tr(TextKey::StateEnabled) } else { s.tr(TextKey::StateDisabled) },
                s.tr(TextKey::FastStartupLabel),
                if h.fast_startup_enabled { s.tr(TextKey::StatusOn) } else { s.tr(TextKey::StatusOff) },
                if h.modern_standby_s0ix { "S0ix" } else { "S3" }
            )
            .into(),
        );
        st.set_power_inflight(s.power_scheme_inflight);
        st.set_hibernation_inflight(s.hibernation_inflight);
        st.set_usb_inflight(s.usb_power_inflight);
        st.set_rolling_back(s.is_rolling_back);
    }
}
