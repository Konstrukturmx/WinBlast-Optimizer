---
title: CPU Core Parking & Dynamic Scheduling
tags: [architecture, cpu, core-parking, powercfg, latency]
created: 2026-11-08
---

# ⚙️ CPU Core Parking & Dynamic Scheduling Architecture

In modern Windows operating systems, the kernel power management subsystem dynamically parks (places into low-power $C$-states and deschedules) logical CPU processor cores during periods of low or asymmetrical CPU load. While this saves power on battery-operated laptops, it introduces significant **micro-stutter, frame drops, and interrupt execution latency** in gaming, DAW audio production, and latency-sensitive workloads due to core wake-up latency ($C$-state transitions).

---

## 🏛️ Power Subgroup & Threshold Architecture

Windows manages core parking through the Processor Power Management (`SUB_PROCESSOR`) subgroup:
- **Processor Subgroup GUID**: `54533251-82be-4824-96c1-47b60b740d00`
- **Core Parking Min Cores (`CPMINCORES`)**: `0cc5b647-c1d6-4526-824a-2138478c1a96`
- **Core Parking Max Cores (`CPMAXCORES`)**: `ea062031-0e34-4ff1-9b6d-eb10593acda8`
- **Core Parking Concurrency (`CPMINPARKEDCORES`)**: `44723270-4b23-4881-80cd-6f38077b8b47`

### The In-Memory Active Scheme Pitfall
Directly modifying registry keys under `HKLM\SYSTEM\CurrentControlSet\Control\Power\PowerSettings\...` changes template metadata or unhides settings in classic Control Panel (`powercfg.cpl`), but **does not update the currently active power scheme running in Windows kernel memory**.

WinBlast0 executes a two-phase commit:
1. **Unhide in Power Options GUI**: Writes `Attributes = 0` (DWORD) to the setting registry keys so the options become visible in Windows Control Panel.
2. **Stealth Powercfg Commit**:
   ```powershell
   powercfg -setacvalueindex SCHEME_CURRENT 54533251-82be-4824-96c1-47b60b740d00 0cc5b647-c1d6-4526-824a-2138478c1a96 100
   powercfg -setdcvalueindex SCHEME_CURRENT 54533251-82be-4824-96c1-47b60b740d00 0cc5b647-c1d6-4526-824a-2138478c1a96 100
   powercfg -setactive SCHEME_CURRENT
   ```
   All executions use `SilentCommand` with `CREATE_NO_WINDOW`, preventing any terminal popups.

---

## 📊 Live Topology & Intel Hybrid (P/E-Core) Awareness

WinBlast0 dynamically maps all logical cores ($0..N-1$) and identifies:
- **Standard Symmetric Multi-Processing (SMP)**: AMD Ryzen and Intel 11th Gen and prior.
- **Hybrid Performance / Efficiency (P/E) Cores**: Intel 12th/13th/14th Gen and Core Ultra. When asymmetric core frequencies are detected, cores are tagged as `P-Core` or `E-Core` to assist users in understanding scheduling behavior.

---

## 🔗 Related Notes
- [[Hardware & GPU Low-Level Tuning]]
- [[System & Network Latency Benchmarking]]
- [[00 - Index (MOC)]]
