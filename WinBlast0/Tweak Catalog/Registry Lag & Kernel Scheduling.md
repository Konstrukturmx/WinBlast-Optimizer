---
title: Registry Lag & Kernel Scheduling Tweaks
tags: [tweaks, latency, scheduler, gaming, registry, ram]
created: 2026-11-08
---

# 🏎️ Registry Lag & Kernel Scheduling Tweaks

This catalog (`registry_lag.toml`) targets Windows NT kernel thread scheduling, filesystem write latency, network packet throttling, and process termination delays to deliver the lowest possible input lag and highest frametime consistency.

---

## 📋 Complete Tweak Matrix

### 1. `win32_priority_separation` — Foreground Thread Quantum Boost
- **What it does**: Sets the Windows thread scheduler quantum to short, variable length with high foreground boost (`0x26` = 38).
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl\Win32PrioritySeparation`
- **Values**: Optimized = `38` (`0x26`), Default = `2`
- **Gain**: Perf +4, Stability: Low risk.

### 2. `menu_show_delay` — Remove Artificial Menu Animation Delay
- **What it does**: Eliminates the default 400ms delay Windows waits before opening dropdown and context menus on hover.
- **Target**: `HKCU\Control Panel\Desktop\MenuShowDelay`
- **Values**: Optimized = `0` ms, Default = `400` ms
- **Gain**: Perf +1, Stability: Safe.

### 3. `network_throttling_disable` — Disable Network Packet Throttling
- **What it does**: Windows multimedia scheduler throttles non-multimedia network packets when games/audio run. Setting `NetworkThrottlingIndex` to `0xFFFFFFFF` completely disables this throttling mechanism, preventing packet delay spikes in online games.
- **Target**: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\NetworkThrottlingIndex`
- **Values**: Optimized = `4294967295` (`0xFFFFFFFF`), Default = `10`
- **Gain**: Perf +4, Stability: Safe.

### 4. `system_responsiveness_gaming` — Maximize Foreground System Responsiveness
- **What it does**: By default, Windows reserves 20% of CPU cycles for background processes. Setting `SystemResponsiveness` to `0` dedicates 100% of processing power to foreground gaming and real-time audio tasks.
- **Target**: `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\SystemResponsiveness`
- **Values**: Optimized = `0` %, Default = `20` %
- **Gain**: Perf +4, Stability: Safe.

### 5. `ntfs_disable_last_access` — Disable NTFS Last Access Timestamps
- **What it does**: Prevents NTFS from updating disk metadata timestamps on every file read operation. Eliminates continuous small write operations to NVMe/SSDs.
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\NtfsDisableLastAccessUpdate`
- **Values**: Optimized = `1` (Disabled), Default = `0`
- **Gain**: Perf +3, Stability: Safe.

### 6. `disable_paging_executive` — Keep Windows Kernel & Drivers in RAM
- **What it does**: Forces the Windows NT executive kernel, HAL, and device drivers to stay permanently in physical RAM instead of being paged out to disk.
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\DisablePagingExecutive`
- **Values**: Optimized = `1` (Keep in RAM), Default = `0`
- **Gain**: Perf +4, Stability: Safe.

### 7. `large_system_cache` — Enable Large System File Cache
- **What it does**: Configures the Windows cache manager to maintain a larger file system cache in physical memory, speeding up repeated disk I/O and texture asset streaming.
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\LargeSystemCache`
- **Values**: Optimized = `1`, Default = `0`
- **Gain**: Perf +3, Stability: Low risk.

### 8. `disable_fast_startup` — Disable Hybrid Sleep Driver Leaks
- **What it does**: Disables Windows Fast Startup (`hiberboot`). Fast Startup hibernates the Windows kernel session, accumulating memory leaks and driver corruption over weeks of uptime. Disabling it ensures a true clean boot cycle on every shutdown.
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Power\HiberbootEnabled`
- **Values**: Optimized = `0` (Disabled), Default = `1`
- **Gain**: Perf +2, Stability: Safe.

### 9. `auto_end_tasks` — Auto End Hung Tasks on Shutdown
- **What it does**: Automatically terminates hung applications on shutdown/reboot without showing dialogs, and reduces `WaitToKillAppTimeout` (20000 -> 2000 ms) and `HungAppTimeout` (5000 -> 1000 ms).
- **Target**: `HKCU\Control Panel\Desktop` (`AutoEndTasks`, `WaitToKillAppTimeout`, `HungAppTimeout`)
- **Gain**: Perf +2, Stability: Low risk.

### 10. `disable_wer` — Disable Windows Error Reporting
- **What it does**: Prevents the Windows Error Reporting service from generating large crash dumps and sending telemetry packages to Microsoft when programs crash.
- **Target**: `HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\Disabled`
- **Values**: Optimized = `1`, Default = `0`
- **Gain**: Perf +2, Priv +2, Stability: Safe.

---

## 🔗 Related Notes
- [[Hardware & GPU Low-Level Tuning]]
- [[Catalog Overview & Risk Model]]
- [[00 - Index (MOC)]]
