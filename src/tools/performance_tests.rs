//! CPU compute, RAM bandwidth / latency and storage read benchmark.
//!
//! Every number is measured on this PC during the run (AGENTS.md rule 2): RAM latency by
//! pointer chasing through a 64 MiB random cycle, disk read with `FILE_FLAG_NO_BUFFERING` so
//! the file cache cannot answer. A part that cannot be measured stays 0 and the UI shows "—".

use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Individual benchmark outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScore {
    pub cpu_single_score: u64,
    pub cpu_multi_score: u64,
    pub ram_read_gb_s: f32,
    /// Average dependent-load latency in ns; 0 = not measured.
    pub ram_latency_ns: f32,
    /// Sequential uncached read in MB/s; 0 = not measured.
    pub disk_seq_read_mb_s: f32,
    pub total_composite_score: u64,
}

/// "Not measured" score. v1.6.1 returned invented numbers here (1850 / 14200 / 58.4 GB/s /
/// 6850 MB/s / 9420), which the Performance page displayed before any benchmark had run.
impl Default for BenchmarkScore {
    fn default() -> Self {
        Self {
            cpu_single_score: 0,
            cpu_multi_score: 0,
            ram_read_gb_s: 0.0,
            ram_latency_ns: 0.0,
            disk_seq_read_mb_s: 0.0,
            total_composite_score: 0,
        }
    }
}

pub struct BenchmarkRunner;

/// Average latency of dependent loads through a random cycle that does not fit in cache.
pub fn measure_ram_latency_ns(bytes: usize, steps: usize) -> f32 {
    let n = (bytes / std::mem::size_of::<usize>()).max(2);
    // Sattolo's algorithm: one cycle through every slot, so the chase visits all of them.
    let mut next: Vec<usize> = (0..n).collect();
    let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut rand = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for i in (1..n).rev() {
        let j = (rand() % i as u64) as usize;
        next.swap(i, j);
    }
    let mut p = 0usize;
    // Warm-up so page faults are not timed.
    for _ in 0..n.min(1 << 20) {
        p = next[p];
    }
    let start = Instant::now();
    for _ in 0..steps {
        p = next[p];
    }
    let ns = start.elapsed().as_nanos() as f64;
    std::hint::black_box(p);
    (ns / steps.max(1) as f64) as f32
}

/// Sequential read speed of a temporary file on the system drive, bypassing the file cache.
pub fn measure_disk_read_mb_s(file_mb: usize) -> Option<f32> {
    use std::io::{Read, Write};
    use std::os::windows::fs::OpenOptionsExt;
    const CHUNK: usize = 1 << 20;
    const FILE_FLAG_NO_BUFFERING: u32 = 0x2000_0000;
    const FILE_FLAG_SEQUENTIAL_SCAN: u32 = 0x0800_0000;
    let path = std::env::temp_dir().join(format!("winblast_bench_{}.bin", std::process::id()));
    let result = (|| -> std::io::Result<f32> {
        {
            let mut f = std::fs::File::create(&path)?;
            let block = vec![0x5Au8; CHUNK];
            for _ in 0..file_mb {
                f.write_all(&block)?;
            }
            f.sync_all()?;
        }
        // NO_BUFFERING needs a sector-aligned buffer.
        let layout = std::alloc::Layout::from_size_align(CHUNK, 4096)
            .map_err(|_| std::io::Error::other("layout"))?;
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(std::io::Error::other("allocation failed"));
        }
        let buf = unsafe { std::slice::from_raw_parts_mut(ptr, CHUNK) };
        let mut f = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_NO_BUFFERING | FILE_FLAG_SEQUENTIAL_SCAN)
            .open(&path)?;
        let start = Instant::now();
        let mut total = 0usize;
        loop {
            match f.read(buf) {
                Ok(0) => break,
                Ok(n) => total += n,
                Err(e) => {
                    unsafe { std::alloc::dealloc(ptr, layout) };
                    return Err(e);
                }
            }
        }
        let secs = start.elapsed().as_secs_f64();
        unsafe { std::alloc::dealloc(ptr, layout) };
        if total == 0 || secs <= 0.0 {
            return Err(std::io::Error::other("nothing read"));
        }
        Ok(((total as f64 / 1_000_000.0) / secs) as f32)
    })();
    let _ = std::fs::remove_file(&path);
    result.ok()
}

impl BenchmarkRunner {
    /// Run the CPU, RAM and disk measurements (a few seconds).
    pub fn run_quick() -> BenchmarkScore {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        // 1. CPU: the same fixed work on every thread; score = work per second (no clamping).
        let start = Instant::now();
        let ops = 5_000_000;
        let mut handles = Vec::new();
        for _ in 0..threads {
            handles.push(std::thread::spawn(move || {
                let mut acc = 1.0f64;
                for i in 1..ops {
                    acc = (acc * 1.000001 + (i as f64).sin()).sqrt();
                }
                std::hint::black_box(acc)
            }));
        }
        for h in handles {
            let _ = h.join();
        }
        let elapsed_ms = start.elapsed().as_millis().max(1) as u64;
        let multi_score = (threads as u64 * 100_000_000) / elapsed_ms;
        let single_score = multi_score / (threads as u64).max(1);

        // 2. RAM bandwidth: 4 × 32 MiB copies.
        let ram_start = Instant::now();
        let buf_size = 32 * 1024 * 1024;
        let mut src = vec![0xAAu8; buf_size];
        let mut dst = vec![0x00u8; buf_size];
        for _ in 0..4 {
            dst.copy_from_slice(&src);
            src[0] = src[0].wrapping_add(1);
        }
        std::hint::black_box(&dst);
        let ram_secs = ram_start.elapsed().as_secs_f32().max(1e-6);
        let ram_read_gb_s = ((buf_size as f32 * 4.0) / (1024.0 * 1024.0 * 1024.0)) / ram_secs;

        // 3. RAM latency and 4. disk read, both measured.
        let ram_latency_ns = measure_ram_latency_ns(64 * 1024 * 1024, 5_000_000);
        let disk_seq_read_mb_s = measure_disk_read_mb_s(256).unwrap_or(0.0);

        let composite = ((multi_score as f32 * 0.5)
            + (ram_read_gb_s * 50.0)
            + (single_score as f32 * 1.2)) as u64;

        BenchmarkScore {
            cpu_single_score: single_score,
            cpu_multi_score: multi_score,
            ram_read_gb_s: (ram_read_gb_s * 10.0).round() / 10.0,
            ram_latency_ns: (ram_latency_ns * 10.0).round() / 10.0,
            disk_seq_read_mb_s: disk_seq_read_mb_s.round(),
            total_composite_score: composite,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_latency_is_measured_not_fixed() {
        let small = measure_ram_latency_ns(16 * 1024, 200_000);
        let big = measure_ram_latency_ns(64 * 1024 * 1024, 2_000_000);
        assert!(small > 0.0 && big > 0.0);
        assert!(big > small, "a 64 MiB chase ({big} ns) must be slower than an L1-sized one ({small} ns)");
        assert_ne!(big, 58.5);
    }

    #[test]
    fn disk_read_is_measured() {
        let mb_s = measure_disk_read_mb_s(16).expect("uncached read of a temp file");
        assert!(mb_s > 1.0 && mb_s != 6850.0, "{mb_s}");
    }
}
