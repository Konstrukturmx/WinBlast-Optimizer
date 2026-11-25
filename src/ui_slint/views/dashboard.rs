//! Easy Dashboard binder: health score, self-updater, live CPU/RAM waveforms, category
//! breakdown, before/after impact, hardware summary, presets and quick-launch cards.

use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::app::{AppState, UpdateStatus};
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;
use crate::scanner::{ScanResult, TweakState};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{CategoryStat, DashboardStore, MainWindow};

/// (already optimized, applicable) tweak counts behind the health score.
pub fn health_counts(results: &[ScanResult]) -> (usize, usize) {
    let eligible = results.iter().filter(|r| r.current_state != TweakState::NotApplicable);
    eligible.fold((0, 0), |(o, e), r| (o + usize::from(r.current_state == TweakState::AlreadyOptimized), e + 1))
}

/// 0-100 share of applicable tweaks already optimized (v1.6.1 formula).
pub fn health_score(results: &[ScanResult]) -> u32 {
    let (optimized, eligible) = health_counts(results);
    if eligible == 0 {
        return 0;
    }
    ((optimized as f64 / eligible as f64) * 100.0).round() as u32
}

/// Caption under the health ring. It used to be a fixed "Optimized", which read as a verdict
/// even at 7 %; now it states the real count.
pub fn health_caption(results: &[ScanResult], optimized_word: &str) -> String {
    match health_counts(results) {
        (_, 0) => String::new(),
        (o, e) => format!("{o}/{e} · {optimized_word}"),
    }
}

pub fn health_tone(score: u32) -> &'static str {
    match score {
        0..=39 => "danger",
        40..=69 => "warning",
        _ => "success",
    }
}

/// (gaming, services, privacy, hardware) as (optimized, total), v1.6.1 bucketing rules.
pub fn category_buckets(results: &[ScanResult]) -> [(usize, usize); 4] {
    let mut b = [(0usize, 0usize); 4];
    for r in results.iter().filter(|r| r.current_state != TweakState::NotApplicable) {
        let cat = r.category.to_lowercase();
        let i = if cat.contains("game") || cat.contains("gpu") || cat.contains("input") {
            0
        } else if cat.contains("service") || cat.contains("startup") || cat.contains("task") {
            1
        } else if cat.contains("privacy") || cat.contains("telemetry") || cat.contains("security") {
            2
        } else {
            3
        };
        b[i].1 += 1;
        if r.current_state == TweakState::AlreadyOptimized {
            b[i].0 += 1;
        }
    }
    b
}

/// CPU sparkline title adjusted to the configured telemetry interval (v1.6.1 behaviour).
pub fn cpu_title(s: &AppState) -> String {
    let base = s.tr(TextKey::CpuSparklineTitle);
    let secs = s.telemetry_interval_secs();
    if (secs - 1.0).abs() < 0.05 {
        return base.to_string();
    }
    let sec = if secs.fract() == 0.0 { format!("{secs:.0}") } else { format!("{secs:.1}") };
    base.replace("1sn", &format!("{sec}sn")).replace("1s", &format!("{sec}s")).replace("1秒", &format!("{sec}秒"))
}

pub struct DashboardView {
    cats: Rc<VecModel<CategoryStat>>,
    cpu: Rc<VecModel<f32>>,
    ram: Rc<VecModel<f32>>,
}

impl DashboardView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<DashboardStore>();
        let (cats, c) = new_model();
        let (cpu, cp) = new_model();
        let (ram, rp) = new_model();
        st.set_categories(c);
        st.set_cpu_points(cp);
        st.set_ram_points(rp);
        let _: Option<ModelRc<f32>> = None;
        Self { cats, cpu, ram }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<DashboardStore>();
        let c = ctx.clone();
        st.on_relaunch_admin(move || {
            c.act(|s| s.relaunch_as_admin());
        });
        let c = ctx.clone();
        st.on_check_updates(move || {
            c.act(|s| s.check_for_updates());
        });
        let c = ctx.clone();
        st.on_download_update(move || {
            c.act(|s| s.download_and_install_update());
        });
        let c = ctx.clone();
        st.on_apply_restart(move || {
            c.act(|s| s.apply_and_restart());
        });
        let c = ctx.clone();
        st.on_reset_baseline(move || {
            c.act(|s| {
                if !s.baseline_reset_inflight {
                    s.reset_baseline();
                }
            });
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<DashboardStore>();
        let detecting = s.tr(TextKey::ValueDetecting);
        st.set_admin_required(!s.is_admin_elevated());

        // Updater
        let (state, title, detail, progress): (&str, String, String, f32) = match &s.update_status {
            UpdateStatus::Idle => ("idle", format!("WinBlast0 v{} • Slint 1.19", env!("CARGO_PKG_VERSION")), String::new(), 0.0),
            UpdateStatus::Checking => ("checking", s.tr(TextKey::UpdateChecking).into(), String::new(), 0.0),
            UpdateStatus::Available(info) => (
                "available",
                format!("{}: {}", s.tr(TextKey::UpdateAvailableBadge), info.tag_name),
                match info.download_size_mb {
                    Some(mb) if mb.is_finite() && mb > 0.0 => format!("{} • {:.1} MB", info.name, mb),
                    _ => info.name.clone(),
                },
                0.0,
            ),
            UpdateStatus::Downloading { progress_pct } => (
                "downloading",
                format!("{} ({:.0}%)", s.tr(TextKey::UpdateDownloading), progress_pct),
                String::new(),
                (progress_pct / 100.0).clamp(0.0, 1.0),
            ),
            UpdateStatus::ReadyToRestart { .. } => ("ready", s.tr_v(ViewTextKey::DashboardUpdateReady).into(), String::new(), 1.0),
            UpdateStatus::UpToDate => ("uptodate", s.tr(TextKey::UpdateUpToDate).into(), String::new(), 0.0),
            UpdateStatus::Failed(err) => ("failed", format!("{}: {}", s.tr(TextKey::UpdateFailedMsg), err), String::new(), 0.0),
        };
        st.set_update_state(state.into());
        st.set_update_title(title.into());
        st.set_update_detail(detail.into());
        st.set_update_progress(progress);

        // Health
        let score = health_score(&s.scan_results);
        st.set_health_score(score as i32);
        st.set_health_tone(health_tone(score).into());
        let has_scan = !s.scan_results.is_empty();
        st.set_has_scan(has_scan);
        st.set_health_caption(if has_scan {
            health_caption(&s.scan_results, s.tr(TextKey::OptimizedLabel)).into()
        } else {
            s.tr_v(crate::core::view_i18n::ViewTextKey::NotScannedYet).into()
        });
        st.set_is_scanning(s.is_scanning);

        // Telemetry
        match &s.realtime_metrics {
            Some(m) => {
                st.set_metrics_available(true);
                st.set_cpu_title(cpu_title(s).into());
                st.set_cpu_text(format!("{:.1}% ({} {})", m.cpu_usage, m.thread_count, s.tr(TextKey::ThreadsLabel)).into());
                st.set_ram_title(s.tr(TextKey::RamSparklineTitle).into());
                st.set_ram_text(
                    format!(
                        "{:.1} / {:.1} GB ({:.1}%)",
                        m.used_ram_mb as f64 / 1024.0,
                        m.total_ram_mb as f64 / 1024.0,
                        m.ram_percentage
                    )
                    .into(),
                );
                let tail = |v: &[f32]| v.iter().rev().take(30).rev().map(|x| if x.is_finite() { *x } else { 0.0 }).collect::<Vec<f32>>();
                sync_vec(&self.cpu, tail(&s.cpu_sparkline));
                sync_vec(&self.ram, tail(&s.ram_sparkline));
            }
            None => st.set_metrics_available(false),
        }

        // Category breakdown
        let b = category_buckets(&s.scan_results);
        let labels = [
            (s.tr_v(ViewTextKey::DashboardCatGaming), "accent"),
            (s.tr_v(ViewTextKey::DashboardCatServices), "success"),
            (s.tr_v(ViewTextKey::DashboardCatPrivacy), "warning"),
            (s.tr_v(ViewTextKey::DashboardCatHardware), "muted"),
        ];
        sync_vec(
            &self.cats,
            labels
                .iter()
                .zip(b)
                .map(|((l, tone), (o, t))| CategoryStat { label: (*l).into(), optimized: o as i32, total: t as i32, tone: (*tone).into() })
                .collect(),
        );

        // Before / after impact
        st.set_baseline_reset_inflight(s.baseline_reset_inflight);
        match s.calculate_before_after_impact() {
            Some(i) => {
                st.set_impact_available(true);
                st.set_ram_now(format!("{:.1} GB", i.current_ram_mb as f64 / 1024.0).into());
                st.set_ram_was(format!("({} {:.1} GB)", s.tr(TextKey::WasLabel), i.baseline_ram_mb as f64 / 1024.0).into());
                st.set_ram_good(i.ram_saved_mb >= 0);
                st.set_ram_delta(
                    if i.ram_saved_mb >= 0 {
                        format!("▼ {} MB {}", i.ram_saved_mb, s.tr(TextKey::FreedRam))
                    } else {
                        format!("▲ {} MB {}", i.ram_saved_mb.saturating_abs(), s.tr(TextKey::ActiveRam))
                    }
                    .into(),
                );
                st.set_proc_now(i.current_processes.to_string().into());
                st.set_proc_was(format!("({} {})", s.tr(TextKey::WasLabel), i.baseline_processes).into());
                st.set_proc_good(i.processes_saved >= 0);
                st.set_proc_delta(
                    if i.processes_saved >= 0 {
                        format!("▼ {} {}", i.processes_saved, s.tr(TextKey::FewerProcesses))
                    } else {
                        format!("▲ {} {}", i.processes_saved.saturating_abs(), s.tr(TextKey::BackgroundProcesses))
                    }
                    .into(),
                );
                st.set_timer_now(format!("{:.3} ms", i.current_timer_ns as f64 / 1_000_000.0).into());
                st.set_timer_was(format!("{}: {:.3} ms", s.tr(TextKey::BeforeTest), i.baseline_timer_ns as f64 / 1_000_000.0).into());
                st.set_timer_good(i.current_timer_ns < i.baseline_timer_ns);
                st.set_opt_now(i.current_optimized_count.to_string().into());
                st.set_opt_was(format!("({} {})", s.tr(TextKey::WasLabel), i.baseline_optimized_count).into());
                st.set_opt_gained(i.tweaks_gained > 0);
                st.set_opt_delta(
                    if i.tweaks_gained > 0 {
                        format!("▲ +{} {}", i.tweaks_gained, s.tr(TextKey::TweaksGained))
                    } else {
                        s.tr(TextKey::BaselineProfileActive).to_string()
                    }
                    .into(),
                );
            }
            None => st.set_impact_available(false),
        }

        // Hardware summary — real values only.
        let cpu = s.cpu_info.as_ref();
        st.set_hw_cpu(cpu.map_or(detecting.into(), |c| c.model_name.as_str().into()));
        st.set_hw_cpu_topology(cpu.map_or(SharedString::new(), |c| {
            format!("{} Physical / {} Logical Threads", c.physical_cores, c.logical_cores).into()
        }));
        let gpu = s.gpus.first();
        st.set_hw_gpu(gpu.map_or(detecting.into(), |g| g.name.as_str().into()));
        st.set_hw_gpu_detail(gpu.map_or(SharedString::new(), |g| {
            if g.dedicated_vram_mb > 0 {
                format!("{:.1} GB Dedicated VRAM", g.dedicated_vram_mb as f64 / 1024.0).into()
            } else {
                SharedString::new()
            }
        }));
        let ps = s.platform_security.as_ref();
        st.set_hw_ram(ps.map_or(detecting.into(), |p| p.ram_summary.as_str().into()));
        st.set_hw_board(ps.map_or(SharedString::new(), |p| p.motherboard.as_str().into()));
        st.set_hw_uptime(ps.map_or(detecting.into(), |p| format!("{:.1} Hours Uptime", p.uptime_hours).into()));
        st.set_hw_rules(format!("{} {}", s.scan_results.len(), s.tr(TextKey::DashboardHwOptimizationRules)).into());
    }
}
