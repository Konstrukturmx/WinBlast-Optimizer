//! Latency benchmark binder: DPC jitter, timer resolution, Anycast DNS ping/jitter, before/after.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::hardware::net_latency::LatencyBenchmarkSnapshot;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{LatencyMetric, LatencyStore, MainWindow, PingRow};

fn ping_text(ms: f64) -> String {
    if !ms.is_finite() || ms < 0.0 {
        "Offline (100% Loss)".into()
    } else {
        format!("{ms:.1} ms")
    }
}

fn jitter_text(ms: f64) -> String {
    if !ms.is_finite() || ms < 0.0 {
        "—".into()
    } else {
        format!("{ms:.2} ms")
    }
}

fn better(after: f64, before: f64) -> &'static str {
    if after < 0.0 {
        "danger"
    } else if after <= before {
        "success"
    } else {
        "warning"
    }
}

pub fn comparison(s: &AppState, b: &LatencyBenchmarkSnapshot, a: &LatencyBenchmarkSnapshot) -> Vec<LatencyMetric> {
    let was = s.tr(TextKey::WasLabel);
    vec![
        LatencyMetric {
            label: s.tr(TextKey::DpcJitterPeak).into(),
            now: format!("{} µs", a.dpc.peak_jitter_us).into(),
            was: format!("({was} {} µs)", b.dpc.peak_jitter_us).into(),
            tone: if a.dpc.peak_jitter_us <= b.dpc.peak_jitter_us { "success" } else { "warning" }.into(),
        },
        LatencyMetric {
            label: s.tr(TextKey::TimerResolution).into(),
            now: format!("{:.3} ms", a.dpc.timer_resolution_ms).into(),
            was: format!("({was} {:.3} ms)", b.dpc.timer_resolution_ms).into(),
            tone: if a.dpc.timer_resolution_ms <= b.dpc.timer_resolution_ms { "success" } else { "warning" }.into(),
        },
        LatencyMetric {
            label: s.tr(TextKey::AvgPingDns).into(),
            now: ping_text(a.overall_avg_ping_ms).into(),
            was: format!("({was} {})", ping_text(b.overall_avg_ping_ms)).into(),
            tone: better(a.overall_avg_ping_ms, b.overall_avg_ping_ms).into(),
        },
        LatencyMetric {
            label: s.tr(TextKey::AvgPingJitter).into(),
            now: jitter_text(a.overall_avg_jitter_ms).into(),
            was: format!("({was} {})", jitter_text(b.overall_avg_jitter_ms)).into(),
            tone: better(a.overall_avg_jitter_ms, b.overall_avg_jitter_ms).into(),
        },
    ]
}

pub struct LatencyView {
    comparison: Rc<VecModel<LatencyMetric>>,
    kpis: Rc<VecModel<LatencyMetric>>,
    targets: Rc<VecModel<PingRow>>,
}

impl LatencyView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<LatencyStore>();
        let (comparison, a) = new_model();
        let (kpis, b) = new_model();
        let (targets, c) = new_model();
        st.set_comparison(a);
        st.set_kpis(b);
        st.set_targets(c);
        Self { comparison, kpis, targets }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<LatencyStore>();
        let c = ctx.clone();
        st.on_run(move || {
            c.act(|s| {
                if !s.is_benchmarking_latency {
                    s.start_latency_benchmark();
                }
            });
        });
        let c = ctx.clone();
        st.on_save_baseline(move || {
            c.act(|s| {
                if !s.is_benchmarking_latency {
                    s.set_latency_baseline_before();
                }
            });
        });
        let c = ctx.clone();
        st.on_clear(move || {
            c.act(|s| s.reset_latency_benchmark());
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<LatencyStore>();
        st.set_running(s.is_benchmarking_latency);
        match (&s.latency_before, &s.latency_after) {
            (Some(b), Some(a)) => {
                st.set_has_comparison(true);
                sync_vec(&self.comparison, comparison(s, b, a));
            }
            _ => {
                st.set_has_comparison(false);
                sync_vec(&self.comparison, Vec::new());
            }
        }
        match &s.latest_latency_bench {
            Some(bench) => {
                st.set_has_result(true);
                let k = |label: TextKey, now: String| LatencyMetric { label: s.tr(label).into(), now: now.into(), was: SharedString::new(), tone: "accent".into() };
                sync_vec(
                    &self.kpis,
                    vec![
                        k(TextKey::DpcJitterPeak, format!("{} µs", bench.dpc.peak_jitter_us)),
                        k(TextKey::TimerResolution, format!("{:.3} ms", bench.dpc.timer_resolution_ms)),
                        k(TextKey::AvgPingDns, ping_text(bench.overall_avg_ping_ms)),
                        k(TextKey::AvgPingJitter, jitter_text(bench.overall_avg_jitter_ms)),
                    ],
                );
                sync_vec(
                    &self.targets,
                    bench
                        .targets
                        .iter()
                        .map(|t| PingRow {
                            name: t.name.as_str().into(),
                            loss: format!("{}: {:.0}%", s.tr(TextKey::LossLabel), t.packet_loss_pct).into(),
                            loss_ok: t.packet_loss_pct <= 0.0,
                            min: format!("{:.1} ms", t.rtt_min_ms).into(),
                            avg: format!("{:.1} ms", t.rtt_avg_ms).into(),
                            max: format!("{:.1} ms", t.rtt_max_ms).into(),
                            jitter: format!("{:.2} ms", t.jitter_ms).into(),
                        })
                        .collect(),
                );
            }
            None => {
                st.set_has_result(false);
                sync_vec(&self.kpis, Vec::new());
                sync_vec(&self.targets, Vec::new());
            }
        }
    }
}
