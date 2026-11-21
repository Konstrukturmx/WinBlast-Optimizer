//! Real-time Task Manager style system monitoring subsystem.
//!
//! Samples true NT scheduler CPU utilization (`NtQuerySystemInformation`), live kernel core
//! parking telemetry (`\\Processor Information(*)\\Parking Status` via PDH), RAM usage (`sysinfo`
//! memory-only), process/thread counts, and NT kernel timer resolution without blocking the UI.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use windows::core::PCWSTR;
use windows::Win32::System::Performance::{
    PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterArrayW,
    PdhOpenQueryW, PDH_CSTATUS_NEW_DATA, PDH_CSTATUS_VALID_DATA, PDH_FMT_COUNTERVALUE_ITEM_W,
    PDH_FMT_DOUBLE, PDH_HCOUNTER, PDH_HQUERY, PDH_MORE_DATA,
};
use windows::Win32::System::Threading::{
    GetActiveProcessorCount, GetActiveProcessorGroupCount, ALL_PROCESSOR_GROUPS,
};

#[link(name = "ntdll")]
extern "system" {
    fn NtQueryTimerResolution(
        minimum_resolution: *mut u32,
        maximum_resolution: *mut u32,
        current_resolution: *mut u32,
    ) -> i32;

    fn NtQuerySystemInformation(
        system_information_class: u32,
        system_information: *mut std::ffi::c_void,
        system_information_length: u32,
        return_length: *mut u32,
    ) -> i32;
}

const SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_CLASS: u32 = 8;
const STATUS_INFO_LENGTH_MISMATCH: i32 = 0xC0000004u32 as i32;
const CPU_SAMPLE_MIN_INTERVAL: Duration = Duration::from_millis(100);

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct SystemProcessorPerformanceInformation {
    idle_time: i64,
    kernel_time: i64,
    user_time: i64,
    dpc_time: i64,
    interrupt_time: i64,
    interrupt_count: u32,
}

#[repr(C)]
struct PerformanceInformation {
    cb: u32,
    commit_total: usize,
    commit_limit: usize,
    commit_peak: usize,
    physical_total: usize,
    physical_available: usize,
    system_cache: usize,
    kernel_total: usize,
    kernel_paged: usize,
    kernel_nonpaged: usize,
    page_size: usize,
    handle_count: u32,
    process_count: u32,
    thread_count: u32,
}

#[link(name = "psapi")]
extern "system" {
    fn GetPerformanceInfo(p_performance_information: *mut PerformanceInformation, cb: u32) -> i32;
}

/// Query active logical processor count across all processor groups.
fn active_logical_core_count() -> usize {
    let n = unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS) } as usize;
    if n > 0 {
        n
    } else {
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(1)
            .max(1)
    }
}

/// Compute per-group cumulative logical processor base offsets (`group -> start_index`).
fn query_processor_group_bases() -> (Vec<usize>, usize) {
    let group_count = unsafe { GetActiveProcessorGroupCount() } as usize;
    let mut bases = Vec::with_capacity(group_count.max(1));
    let mut running = 0usize;
    for g in 0..group_count {
        bases.push(running);
        running += unsafe { GetActiveProcessorCount(g as u16) } as usize;
    }
    (bases, running.max(active_logical_core_count()))
}

/// Query per-logical-processor NT kernel performance counters (`SystemProcessorPerformanceInformation`) into an existing buffer.
fn query_nt_processor_performance_into(
    buf: &mut Vec<SystemProcessorPerformanceInformation>,
) -> bool {
    let elem_sz = std::mem::size_of::<SystemProcessorPerformanceInformation>();
    if buf.is_empty() {
        let capacity = active_logical_core_count().max(1);
        buf.resize(capacity, SystemProcessorPerformanceInformation::default());
    }

    for _ in 0..2 {
        let mut ret_len = 0u32;
        let byte_len = (buf.len() * elem_sz) as u32;
        let status = unsafe {
            NtQuerySystemInformation(
                SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION_CLASS,
                buf.as_mut_ptr().cast(),
                byte_len,
                &mut ret_len,
            )
        };
        if status >= 0 {
            let count = ((ret_len as usize) / elem_sz).min(buf.len());
            if count == 0 {
                return false;
            }
            buf.truncate(count);
            return true;
        }
        if status == STATUS_INFO_LENGTH_MISMATCH || (ret_len as usize) > buf.len() * elem_sz {
            let capacity = ((ret_len as usize).div_ceil(elem_sz)).max(buf.len() * 2);
            buf.resize(capacity, SystemProcessorPerformanceInformation::default());
            continue;
        }
        return false;
    }
    false
}

/// Query per-logical-processor NT kernel performance counters (`SystemProcessorPerformanceInformation`).
fn query_nt_processor_performance() -> Option<Vec<SystemProcessorPerformanceInformation>> {
    let mut buf = Vec::new();
    if query_nt_processor_performance_into(&mut buf) {
        Some(buf)
    } else {
        None
    }
}

/// Compute true OS scheduler CPU utilization deltas from consecutive NT performance snapshots into a reusable buffer.
fn compute_nt_cpu_deltas_into(
    prev: &[SystemProcessorPerformanceInformation],
    cur: &[SystemProcessorPerformanceInformation],
    cores: &mut Vec<f32>,
) -> Option<f32> {
    if prev.is_empty() || prev.len() != cur.len() {
        return None;
    }
    let mut sum_busy = 0u128;
    let mut sum_total = 0u128;
    cores.clear();
    cores.reserve(cur.len());

    for (p, c) in prev.iter().zip(cur.iter()) {
        let d_idle = (c.idle_time.max(0) as u64).saturating_sub(p.idle_time.max(0) as u64);
        let d_kernel = (c.kernel_time.max(0) as u64).saturating_sub(p.kernel_time.max(0) as u64);
        let d_user = (c.user_time.max(0) as u64).saturating_sub(p.user_time.max(0) as u64);
        let total = d_kernel.saturating_add(d_user);
        let busy = d_kernel
            .saturating_sub(d_idle)
            .saturating_add(d_user)
            .min(total);
        let pct = if total > 0 {
            ((100.0 * busy as f64) / total as f64).clamp(0.0, 100.0) as f32
        } else {
            0.0
        };
        cores.push(pct);
        sum_busy += busy as u128;
        sum_total += total as u128;
    }

    if sum_total == 0 {
        return None;
    }
    Some(((100.0 * sum_busy as f64) / sum_total as f64).clamp(0.0, 100.0) as f32)
}

/// Compute true OS scheduler CPU utilization deltas from consecutive NT performance snapshots.
///
/// Windows NT `KernelTime` includes `IdleTime`, so `busy = (d_kernel - d_idle) + d_user`
/// remains accurate even when `IDLEDISABLE = 1` suppresses hardware C-states.
#[cfg(test)]
fn compute_nt_cpu_deltas(
    prev: &[SystemProcessorPerformanceInformation],
    cur: &[SystemProcessorPerformanceInformation],
) -> Option<(f32, Vec<f32>)> {
    let mut cores = Vec::with_capacity(cur.len());
    let global = compute_nt_cpu_deltas_into(prev, cur, &mut cores)?;
    Some((global, cores))
}

/// True NT scheduler CPU sampler using `NtQuerySystemInformation(8)` with zero heap allocations on hot path.
struct NtCpuSampler {
    prev: Vec<SystemProcessorPerformanceInformation>,
    cur: Vec<SystemProcessorPerformanceInformation>,
    last_sample_at: Option<Instant>,
    last_global: f32,
    last_cores: Vec<f32>,
}

impl NtCpuSampler {
    fn new() -> Self {
        let prev = query_nt_processor_performance().unwrap_or_default();
        let n = prev.len().max(active_logical_core_count());
        Self {
            prev,
            cur: Vec::with_capacity(n),
            last_sample_at: Some(Instant::now()),
            last_global: 0.0,
            last_cores: vec![0.0; n],
        }
    }

    fn core_count(&self) -> usize {
        self.prev
            .len()
            .max(self.last_cores.len())
            .max(active_logical_core_count())
    }

    fn sample(&mut self) -> Option<(f32, Vec<f32>)> {
        if !query_nt_processor_performance_into(&mut self.cur) {
            return None;
        }
        if self.prev.len() != self.cur.len() || self.prev.is_empty() {
            std::mem::swap(&mut self.prev, &mut self.cur);
            self.last_sample_at = Some(Instant::now());
            self.last_cores.resize(self.prev.len().max(1), 0.0);
            return None;
        }
        let global = compute_nt_cpu_deltas_into(&self.prev, &self.cur, &mut self.last_cores)?;
        std::mem::swap(&mut self.prev, &mut self.cur);
        self.last_sample_at = Some(Instant::now());
        self.last_global = global;
        Some((global, self.last_cores.clone()))
    }
}

/// Map a PDH `\Processor Information(<instance>)\Parking Status` instance name (`<group>,<lp>`)
/// to a flat zero-based logical processor index, ignoring `_Total` summary instances.
fn parse_pdh_processor_instance(name: &str, group_bases: &[usize]) -> Option<usize> {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed.to_ascii_lowercase().contains("_total") {
        return None;
    }
    let (g_str, lp_str) = trimmed.split_once(',')?;
    let group: usize = g_str.trim().parse().ok()?;
    let lp: usize = lp_str.trim().parse().ok()?;
    let base = group_bases.get(group).copied()?;
    Some(base + lp)
}

/// RAII PDH collector for live kernel `Parking Status` per logical processor.
struct PdhParkingCollector {
    query: PDH_HQUERY,
    counter: PDH_HCOUNTER,
    group_bases: Vec<usize>,
}

// SAFETY: `PDH_HQUERY` and `PDH_HCOUNTER` are owned exclusively by `PdhParkingCollector`
// and accessed only via `&mut self` (never shared concurrently without synchronization).
unsafe impl Send for PdhParkingCollector {}

impl Drop for PdhParkingCollector {
    fn drop(&mut self) {
        if !self.query.is_invalid() {
            unsafe {
                let _ = PdhCloseQuery(self.query);
            }
        }
    }
}

impl PdhParkingCollector {
    fn new() -> Option<Self> {
        let mut query = PDH_HQUERY::default();
        let rc = unsafe { PdhOpenQueryW(PCWSTR::null(), 0, &mut query) };
        if rc != 0 || query.is_invalid() {
            return None;
        }

        let (group_bases, _) = query_processor_group_bases();
        let mut collector = Self {
            query,
            counter: PDH_HCOUNTER::default(),
            group_bases,
        };

        let path_w: Vec<u16> = "\\Processor Information(*)\\Parking Status\0"
            .encode_utf16()
            .collect();
        let add_rc = unsafe {
            PdhAddEnglishCounterW(
                collector.query,
                PCWSTR(path_w.as_ptr()),
                0,
                &mut collector.counter,
            )
        };
        if add_rc != 0 || collector.counter.is_invalid() {
            return None;
        }

        // Initial query collection so wildcard instances are populated immediately
        let col_rc = unsafe { PdhCollectQueryData(collector.query) };
        if col_rc != 0 {
            return None;
        }

        Some(collector)
    }

    fn sample(&mut self, core_count: usize) -> Vec<bool> {
        let mut parked = vec![false; core_count];
        if core_count == 0 {
            return parked;
        }
        if unsafe { PdhCollectQueryData(self.query) } != 0 {
            return parked;
        }

        let mut buf_size = 0u32;
        let mut item_count = 0u32;
        let mut rc = unsafe {
            PdhGetFormattedCounterArrayW(
                self.counter,
                PDH_FMT_DOUBLE,
                &mut buf_size,
                &mut item_count,
                None,
            )
        };
        if rc != PDH_MORE_DATA || buf_size == 0 {
            return parked;
        }

        let item_sz = std::mem::size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>();
        for _ in 0..2 {
            let cap_items = (buf_size as usize).div_ceil(item_sz).max(1);
            let mut items = vec![PDH_FMT_COUNTERVALUE_ITEM_W::default(); cap_items];
            let mut actual_buf_size = (items.len() * item_sz) as u32;
            let mut actual_count = 0u32;
            rc = unsafe {
                PdhGetFormattedCounterArrayW(
                    self.counter,
                    PDH_FMT_DOUBLE,
                    &mut actual_buf_size,
                    &mut actual_count,
                    Some(items.as_mut_ptr()),
                )
            };
            if rc == 0 {
                let valid = (actual_count as usize).min(items.len());
                for item in &items[..valid] {
                    if item.szName.is_null() {
                        continue;
                    }
                    let Ok(name) = (unsafe { item.szName.to_string() }) else {
                        continue;
                    };
                    let Some(idx) = parse_pdh_processor_instance(&name, &self.group_bases) else {
                        continue;
                    };
                    if idx < parked.len()
                        && (item.FmtValue.CStatus == PDH_CSTATUS_VALID_DATA
                            || item.FmtValue.CStatus == PDH_CSTATUS_NEW_DATA)
                    {
                        let val = unsafe { item.FmtValue.Anonymous.doubleValue };
                        if val.is_finite() {
                            parked[idx] = val >= 0.5;
                        }
                    }
                }
                return parked;
            }
            if rc == PDH_MORE_DATA && actual_buf_size > 0 {
                buf_size = actual_buf_size;
                continue;
            }
            break;
        }

        parked
    }
}

/// Shared live CPU utilization & kernel parking status sampler reused by [`SystemMonitor`]
/// and [`crate::hardware::parking::CoreParkingState`].
pub fn sample_live_cpu_and_parking(sleep_if_cold: bool) -> (Vec<f32>, Vec<bool>) {
    let mut pdh = PdhParkingCollector::new();
    let core_usages = if sleep_if_cold {
        let mut cpu_sampler = NtCpuSampler::new();
        let core_count = cpu_sampler.core_count().max(1);
        std::thread::sleep(CPU_SAMPLE_MIN_INTERVAL);
        match cpu_sampler.sample() {
            Some((_, cores)) if !cores.is_empty() => cores,
            _ => vec![0.0; core_count],
        }
    } else {
        vec![0.0; active_logical_core_count()]
    };

    let core_parked = pdh
        .as_mut()
        .map(|p| p.sample(core_usages.len()))
        .unwrap_or_else(|| vec![false; core_usages.len()]);

    (core_usages, core_parked)
}

/// Query the exact NT kernel timer resolution in nanoseconds.
pub fn query_nt_timer_resolution() -> u64 {
    let mut min_res = 0u32;
    let mut max_res = 0u32;
    let mut cur_res = 0u32;
    unsafe {
        if NtQueryTimerResolution(&mut min_res, &mut max_res, &mut cur_res) == 0 && cur_res > 0 {
            // cur_res is reported in 100-nanosecond units (e.g. 10000 = 1ms = 1_000_000 ns)
            return (cur_res as u64) * 100;
        }
    }
    15_625_000 // Standard Windows fallback: 15.625 ms
}

/// Query live Windows NT kernel performance metrics (`GetPerformanceInfo`) in a single syscall.
fn query_system_perf_info() -> Option<PerformanceInformation> {
    let mut info = std::mem::MaybeUninit::<PerformanceInformation>::zeroed();
    let cb = std::mem::size_of::<PerformanceInformation>() as u32;
    unsafe {
        std::ptr::addr_of_mut!((*info.as_mut_ptr()).cb).write(cb);
        if GetPerformanceInfo(info.as_mut_ptr(), cb) != 0 {
            Some(info.assume_init())
        } else {
            None
        }
    }
}

/// Query accurate system-wide process and execution thread counts directly via Windows NT performance info.
pub fn query_system_process_and_thread_counts() -> Option<(usize, usize)> {
    query_system_perf_info().map(|perf| (perf.process_count as usize, perf.thread_count as usize))
}

/// Query accurate system-wide execution thread count directly via Windows NT performance info.
#[allow(dead_code)]
pub fn query_system_thread_count() -> Option<usize> {
    query_system_process_and_thread_counts().map(|(_, t)| t)
}

/// Real-time Task Manager style system telemetry snapshot.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemMetrics {
    /// Overall CPU usage percentage (0.0 – 100.0).
    pub cpu_usage: f32,
    /// Per-core CPU usage percentages.
    pub core_usages: Vec<f32>,
    /// Per-core live kernel Parking Status flags (`true` = parked).
    #[serde(default)]
    pub core_parked: Vec<bool>,
    /// Total system RAM in Megabytes.
    pub total_ram_mb: u64,
    /// Currently used system RAM in Megabytes.
    pub used_ram_mb: u64,
    /// Currently available system RAM in Megabytes.
    pub available_ram_mb: u64,
    /// Memory utilization percentage (0.0 – 100.0).
    pub ram_percentage: f32,
    /// Total count of active operating system processes.
    pub process_count: usize,
    /// Estimated total active threads across all processes.
    pub thread_count: usize,
    /// Current system timer resolution in nanoseconds.
    pub timer_resolution_ns: u64,
    /// Timestamp when this snapshot was captured.
    pub timestamp: DateTime<Utc>,
}

impl SystemMetrics {
    /// Format timer resolution in milliseconds (e.g. "0.500 ms" or "15.625 ms").
    pub fn timer_res_ms_display(&self) -> String {
        let ms = self.timer_resolution_ns as f64 / 1_000_000.0;
        format!("{:.3} ms", ms)
    }

    /// Formatted RAM string: "6.2 GB / 31.9 GB (19.4%)"
    pub fn ram_display(&self) -> String {
        format!(
            "{:.1} GB / {:.1} GB ({:.1}%)",
            self.used_ram_mb as f64 / 1024.0,
            self.total_ram_mb as f64 / 1024.0,
            self.ram_percentage
        )
    }
}

/// Continuous monitor maintaining persistent NT CPU / PDH parking / RAM state and rolling waveform history.
pub struct SystemMonitor {
    sys: System,
    cpu_sampler: NtCpuSampler,
    pdh_parking: Option<PdhParkingCollector>,
    cpu_warmed: bool,
    pub cpu_history: VecDeque<f32>,
    pub ram_history: VecDeque<f32>,
    pub max_history: usize,
}

impl SystemMonitor {
    /// Create a new monitor with an initial baseline snapshot (memory-only `sysinfo`).
    pub fn new(max_history: usize) -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_memory();
        let pdh_parking = PdhParkingCollector::new();
        let cpu_sampler = NtCpuSampler::new();

        Self {
            sys,
            cpu_sampler,
            pdh_parking,
            cpu_warmed: false,
            cpu_history: VecDeque::with_capacity(max_history),
            ram_history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Sample current metrics and update rolling history buffers.
    pub fn sample(&mut self) -> SystemMetrics {
        let elapsed = self
            .cpu_sampler
            .last_sample_at
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO);

        if !self.cpu_warmed {
            if elapsed < CPU_SAMPLE_MIN_INTERVAL {
                if std::thread::current().name() != Some("main") {
                    std::thread::sleep(CPU_SAMPLE_MIN_INTERVAL - elapsed);
                    if self.cpu_sampler.sample().is_some() {
                        self.cpu_warmed = true;
                    }
                }
            } else if self.cpu_sampler.sample().is_some() {
                self.cpu_warmed = true;
            }
        } else if elapsed >= CPU_SAMPLE_MIN_INTERVAL {
            let _ = self.cpu_sampler.sample();
        }

        let (cpu_usage, core_usages) = if self.cpu_warmed {
            (
                if self.cpu_sampler.last_global.is_finite() {
                    self.cpu_sampler.last_global.clamp(0.0, 100.0)
                } else {
                    0.0
                },
                self.cpu_sampler
                    .last_cores
                    .iter()
                    .map(|&u| {
                        if u.is_finite() {
                            u.clamp(0.0, 100.0)
                        } else {
                            0.0
                        }
                    })
                    .collect(),
            )
        } else {
            (0.0, vec![0.0; self.cpu_sampler.core_count().max(1)])
        };

        let core_parked = self
            .pdh_parking
            .as_mut()
            .map(|p| p.sample(core_usages.len()))
            .unwrap_or_else(|| vec![false; core_usages.len()]);

        let perf_info = query_system_perf_info();
        let (total_ram_mb, used_ram_mb, available_ram_mb, process_count, thread_count) =
            if let Some(perf) = perf_info {
                let total_bytes =
                    (perf.physical_total as u64).saturating_mul(perf.page_size as u64);
                let avail_bytes =
                    (perf.physical_available as u64).saturating_mul(perf.page_size as u64);
                let used_bytes = total_bytes.saturating_sub(avail_bytes);
                (
                    total_bytes / (1024 * 1024),
                    used_bytes / (1024 * 1024),
                    avail_bytes / (1024 * 1024),
                    perf.process_count as usize,
                    perf.thread_count as usize,
                )
            } else {
                self.sys.refresh_memory();
                let total = self.sys.total_memory() / (1024 * 1024);
                let used = self.sys.used_memory() / (1024 * 1024);
                let avail = self.sys.available_memory() / (1024 * 1024);
                let (p, t) = query_system_process_and_thread_counts()
                    .unwrap_or_else(|| (0, core_usages.len().max(1) * 16));
                (total, used, avail, p, t)
            };

        let ram_percentage = if total_ram_mb > 0 {
            (used_ram_mb as f32 / total_ram_mb as f32) * 100.0
        } else {
            0.0
        };

        let timer_resolution_ns = query_nt_timer_resolution();

        // Push to rolling history
        if self.cpu_history.len() >= self.max_history {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(cpu_usage);

        if self.ram_history.len() >= self.max_history {
            self.ram_history.pop_front();
        }
        self.ram_history.push_back(ram_percentage);

        SystemMetrics {
            cpu_usage,
            core_usages,
            core_parked,
            total_ram_mb,
            used_ram_mb,
            available_ram_mb,
            ram_percentage,
            process_count,
            thread_count,
            timer_resolution_ns,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nt_timer_resolution() {
        let res_ns = query_nt_timer_resolution();
        assert!(res_ns > 0, "Timer resolution must be non-zero");
        // Standard Windows resolutions are between 0.5ms (500_000 ns) and 15.625ms (15_625_000 ns)
        assert!(
            res_ns <= 20_000_000,
            "Timer resolution unexpectedly high: {}",
            res_ns
        );
    }

    #[test]
    fn test_system_thread_count() {
        if let Some(threads) = query_system_thread_count() {
            assert!(
                threads > 50,
                "Windows system must have at least 50 threads, got {}",
                threads
            );
            println!(
                "System reported {} active threads via GetPerformanceInfo",
                threads
            );
        }
    }

    #[test]
    fn test_nt_cpu_delta_handles_idledisable_and_multigroup() {
        assert_eq!(
            std::mem::size_of::<SystemProcessorPerformanceInformation>(),
            48
        );

        // Even when IDLEDISABLE=1 stops hardware C-states, NT IdleTime == KernelTime when idle
        let prev = [
            SystemProcessorPerformanceInformation {
                idle_time: 1_000_000,
                kernel_time: 1_000_000,
                user_time: 500_000,
                ..Default::default()
            },
            SystemProcessorPerformanceInformation {
                idle_time: 1_000_000,
                kernel_time: 1_000_000,
                user_time: 500_000,
                ..Default::default()
            },
        ];
        let cur = [
            SystemProcessorPerformanceInformation {
                idle_time: 2_000_000,   // +1_000_000 idle
                kernel_time: 2_000_000, // +1_000_000 kernel (0 busy kernel)
                user_time: 500_000,     // +0 user
                ..Default::default()
            },
            SystemProcessorPerformanceInformation {
                idle_time: 1_500_000,   // +500_000 idle
                kernel_time: 1_800_000, // +800_000 kernel (300_000 busy kernel)
                user_time: 700_000,     // +200_000 user -> total 1_000_000, busy 500_000 (50%)
                ..Default::default()
            },
        ];
        let (global, cores) = compute_nt_cpu_deltas(&prev, &cur).unwrap();
        assert!(
            (cores[0] - 0.0).abs() < 0.01,
            "Idle core must be 0.0%, got {}",
            cores[0]
        );
        assert!(
            (cores[1] - 50.0).abs() < 0.01,
            "50% busy core got {}",
            cores[1]
        );
        assert!(
            (global - 25.0).abs() < 0.01,
            "Global avg must be 25.0%, got {}",
            global
        );

        // Zero-delta must return None instead of dividing by zero
        assert!(compute_nt_cpu_deltas(&prev, &prev).is_none());

        // Multi-group PDH instance parsing
        let bases = [0usize, 64usize];
        assert_eq!(parse_pdh_processor_instance("0,0", &bases), Some(0));
        assert_eq!(parse_pdh_processor_instance("1,3", &bases), Some(67));
        assert_eq!(parse_pdh_processor_instance("0,_Total", &bases), None);
        assert_eq!(parse_pdh_processor_instance("_total", &bases), None);
    }

    #[test]
    fn test_monitor_sample() {
        let mut monitor = SystemMonitor::new(10);
        let m = monitor.sample();
        assert!(m.cpu_usage.is_finite());
        assert!(m.cpu_usage < 99.9 || m.core_usages.iter().all(|&u| u >= 99.9));
        assert!(!m.core_usages.is_empty());
        assert_eq!(m.core_parked.len(), m.core_usages.len());
        assert!(m.total_ram_mb > 0);
        assert!(m.process_count > 0);
        assert!(m.thread_count >= m.process_count);
        assert!(!m.ram_display().is_empty());
        assert!(!m.timer_res_ms_display().is_empty());
    }

    #[test]
    fn test_monitor_cold_main_thread_never_sleeps_or_spikes() {
        let (elapsed, m) = std::thread::Builder::new()
            .name("main".to_string())
            .spawn(|| {
                let mut monitor = SystemMonitor::new(4);
                let t0 = Instant::now();
                let snap = monitor.sample();
                (t0.elapsed(), snap)
            })
            .unwrap()
            .join()
            .unwrap();

        assert!(
            elapsed < Duration::from_millis(80),
            "Cold sample on 'main' thread slept too long: {:?}",
            elapsed
        );
        assert_eq!(m.cpu_usage, 0.0);
        assert!(m.core_usages.iter().all(|&u| u == 0.0));
        assert_eq!(m.core_parked.len(), m.core_usages.len());
    }
}
