---
title: NT Kernel Timer Resolution (ntdll)
tags: [core, kernel, timer, latency, ntdll, dpc]
created: 2026-11-08
---

# ⏱️ NT Kernel Timer Resolution (ntdll)

The Windows NT kernel timer resolution dictates the minimum time slice after which the thread scheduler can preempt a thread, evaluate wait queues, or fire high-precision multimedia timers.

---

## 🏛️ Kernel Mechanics

### Default vs High-Resolution:
- **Default Windows Timer**: `15.625 ms` (15,625,000 ns). At 60 FPS (16.66ms per frame) or 144 FPS (6.94ms per frame), a 15.625ms timer cannot wake rendering threads or process input events reliably between frames, creating micro-stutters and dropped frames.
- **High-Resolution Timer**: `0.500 ms` to `1.000 ms` (500,000 ns to 1,000,000 ns). Wakes threads up to 31x faster, smoothing out input polling and GPU frame pacing.

---

## 🔬 Implementation in `src/hardware/monitor.rs`

WinBlast0 links directly to `ntdll.lib` via MSVC:
```rust
#[link(name = "ntdll")]
extern "system" {
    fn NtQueryTimerResolution(
        minimum_resolution: *mut u32,
        maximum_resolution: *mut u32,
        current_resolution: *mut u32,
    ) -> i32;
}

pub fn query_nt_timer_resolution() -> u64 {
    let mut min_res = 0u32;
    let mut max_res = 0u32;
    let mut cur_res = 0u32;
    unsafe {
        if NtQueryTimerResolution(&mut min_res, &mut max_res, &mut cur_res) == 0 && cur_res > 0 {
            // cur_res is reported in 100-nanosecond units (e.g. 5000 = 0.5ms = 500_000 ns)
            return (cur_res as u64) * 100;
        }
    }
    15_625_000 // Fallback: 15.625 ms
}
```

This resolution is queried continuously during the 1-second telemetry loop and visualized on the dashboard.

---

## 🔗 Related Notes
- [[Real-Time Task Manager Monitoring]]
- [[Registry Lag & Kernel Scheduling]]
- [[00 - Index (MOC)]]
