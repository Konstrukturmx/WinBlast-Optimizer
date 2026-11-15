//! System and network latency benchmark engine for WinBlast0.
//!
//! Measures DPC/hardware interrupt execution jitter, NT kernel timer resolution,
//! and network packet round-trip time & jitter against Tier-1 global anycast DNS nodes
//! (Cloudflare 1.1.1.1, Google 8.8.8.8, Quad9 9.9.9.9).

use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use super::monitor::query_nt_timer_resolution;

/// Latency target endpoint.
#[derive(Debug, Clone)]
pub struct PingTarget {
    pub name: &'static str,
    pub addr: SocketAddr,
}

pub const PING_TARGETS: [PingTarget; 3] = [
    PingTarget {
        name: "Cloudflare (1.1.1.1)",
        addr: SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1)), 53),
    },
    PingTarget {
        name: "Google (8.8.8.8)",
        addr: SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(8, 8, 8, 8)), 53),
    },
    PingTarget {
        name: "Quad9 (9.9.9.9)",
        addr: SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(9, 9, 9, 9)), 53),
    },
];

/// Result of probing a single network endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPingResult {
    pub name: String,
    pub rtt_min_ms: f64,
    pub rtt_avg_ms: f64,
    pub rtt_max_ms: f64,
    pub jitter_ms: f64,
    pub packet_loss_pct: f64,
}

/// DPC / Hardware interrupt micro-jitter probe result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DpcJitterResult {
    pub samples_count: usize,
    pub avg_gap_us: f64,
    pub peak_jitter_us: u64,
    pub timer_resolution_ms: f64,
}

/// Complete benchmark snapshot for Before / After comparative analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyBenchmarkSnapshot {
    pub stage: String, // "BEFORE" or "AFTER"
    pub timestamp: String,
    pub dpc: DpcJitterResult,
    pub targets: Vec<TargetPingResult>,
    pub overall_avg_ping_ms: f64,
    pub overall_avg_jitter_ms: f64,
}

/// Measure DPC and kernel scheduling jitter via high-resolution QPC probe.
pub fn measure_dpc_jitter(samples: usize) -> DpcJitterResult {
    let timer_res_ns = query_nt_timer_resolution();
    let timer_resolution_ms = timer_res_ns as f64 / 1_000_000.0;

    if samples == 0 {
        return DpcJitterResult {
            samples_count: 0,
            avg_gap_us: 0.0,
            peak_jitter_us: 0,
            timer_resolution_ms,
        };
    }

    let mut peak_gap_us = 0u64;
    let mut total_gap_us = 0.0f64;

    // High-frequency sampling loop detecting OS execution stalls
    let mut prev = Instant::now();
    for _ in 0..samples {
        // Micro-spin to expose potential kernel DPC / hardware ISR interrupts
        for _ in 0..50 {
            std::hint::spin_loop();
        }
        let now = Instant::now();
        let elapsed_us = now.duration_since(prev).as_micros() as u64;
        prev = now;

        if elapsed_us > peak_gap_us {
            peak_gap_us = elapsed_us;
        }
        total_gap_us += elapsed_us as f64;
    }

    let avg_gap_us = total_gap_us / samples as f64;

    DpcJitterResult {
        samples_count: samples,
        avg_gap_us,
        peak_jitter_us: peak_gap_us,
        timer_resolution_ms,
    }
}

/// Probe a single network target with multiple bursts, calculating RTT and RFC 3550 jitter.
pub fn probe_network_target(target: &PingTarget, probes: usize) -> TargetPingResult {
    if probes == 0 {
        return TargetPingResult {
            name: target.name.to_string(),
            rtt_min_ms: 0.0,
            rtt_avg_ms: 0.0,
            rtt_max_ms: 0.0,
            jitter_ms: 0.0,
            packet_loss_pct: 0.0,
        };
    }

    let mut rtts = Vec::with_capacity(probes);
    let mut failed = 0usize;

    for _ in 0..probes {
        let start = Instant::now();
        match TcpStream::connect_timeout(&target.addr, Duration::from_millis(800)) {
            Ok(stream) => {
                let rtt = start.elapsed().as_secs_f64() * 1000.0;
                rtts.push(rtt);
                drop(stream);
            }
            Err(_) => {
                failed += 1;
            }
        }
        // Small 25ms spacing between probes
        std::thread::sleep(Duration::from_millis(25));
    }

    if rtts.is_empty() {
        return TargetPingResult {
            name: target.name.to_string(),
            rtt_min_ms: 0.0,
            rtt_avg_ms: 0.0,
            rtt_max_ms: 0.0,
            jitter_ms: 0.0,
            packet_loss_pct: 100.0,
        };
    }

    let raw_min = rtts.iter().cloned().fold(f64::INFINITY, f64::min);
    let raw_max = rtts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min = if raw_min.is_finite() { raw_min } else { 0.0 };
    let max = if raw_max.is_finite() { raw_max } else { 0.0 };
    let avg = rtts.iter().sum::<f64>() / rtts.len() as f64;

    // RFC 3550 Interarrival Jitter Calculation
    let mut jitter = 0.0f64;
    for i in 1..rtts.len() {
        let diff = (rtts[i] - rtts[i - 1]).abs();
        jitter += (diff - jitter) / 16.0;
    }

    let packet_loss_pct = (failed as f64 / probes as f64) * 100.0;

    TargetPingResult {
        name: target.name.to_string(),
        rtt_min_ms: min,
        rtt_avg_ms: avg,
        rtt_max_ms: max,
        jitter_ms: jitter,
        packet_loss_pct,
    }
}

/// Run a complete latency benchmark suite (DPC probe + multi-target network probe).
pub fn run_full_latency_benchmark(stage: &str) -> LatencyBenchmarkSnapshot {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 1. Measure DPC and kernel interrupt jitter with 5,000 samples
    let dpc = measure_dpc_jitter(5_000);

    // 2. Probe Tier-1 DNS anycast targets concurrently via scoped threads
    let targets = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(PING_TARGETS.len());
        for target in &PING_TARGETS {
            handles.push(s.spawn(move || probe_network_target(target, 4)));
        }
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_else(|_| TargetPingResult {
                name: "Probe Thread Panic".to_string(),
                rtt_min_ms: 0.0,
                rtt_avg_ms: 0.0,
                rtt_max_ms: 0.0,
                jitter_ms: 0.0,
                packet_loss_pct: 100.0,
            }))
            .collect::<Vec<_>>()
    });

    let mut total_ping = 0.0f64;
    let mut total_jitter = 0.0f64;
    let mut successful_targets = 0usize;

    for res in &targets {
        if res.packet_loss_pct < 100.0 {
            total_ping += res.rtt_avg_ms;
            total_jitter += res.jitter_ms;
            successful_targets += 1;
        }
    }

    let overall_avg_ping_ms = if successful_targets > 0 {
        total_ping / successful_targets as f64
    } else {
        -1.0
    };

    let overall_avg_jitter_ms = if successful_targets > 0 {
        total_jitter / successful_targets as f64
    } else {
        -1.0
    };

    LatencyBenchmarkSnapshot {
        stage: stage.to_string(),
        timestamp: now,
        dpc,
        targets,
        overall_avg_ping_ms,
        overall_avg_jitter_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpc_jitter_measurement() {
        let dpc = measure_dpc_jitter(100);
        assert!(dpc.samples_count == 100);
        assert!(dpc.timer_resolution_ms > 0.0);

        let dpc_zero = measure_dpc_jitter(0);
        assert_eq!(dpc_zero.samples_count, 0);
        assert_eq!(dpc_zero.peak_jitter_us, 0);
    }

    #[test]
    fn test_targets_structure() {
        assert_eq!(PING_TARGETS.len(), 3);
        assert_eq!(PING_TARGETS[0].name, "Cloudflare (1.1.1.1)");
        assert_eq!(PING_TARGETS[1].name, "Google (8.8.8.8)");
        assert_eq!(PING_TARGETS[2].name, "Quad9 (9.9.9.9)");
    }

    #[test]
    fn test_probe_network_zero_probes() {
        let res = probe_network_target(&PING_TARGETS[0], 0);
        assert_eq!(res.rtt_min_ms, 0.0);
        assert_eq!(res.packet_loss_pct, 0.0);
    }
}
