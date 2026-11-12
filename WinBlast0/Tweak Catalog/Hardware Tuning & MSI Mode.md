---
title: Hardware Tuning & MSI Mode Tweaks
tags: [tweaks, hardware, msi, irq, hags, mpo]
created: 2026-11-08
---

# ⚡ Hardware Tuning & MSI Mode Tweaks

Defined in `hardware_tuning.toml`, these tweaks interact with GPU drivers, display compositor modes, and PCIe bus interrupt handling to eliminate micro-stutters and input latency in modern games.

---

## 📋 Complete Tweak Matrix

### 1. `enable_gpu_msi_mode` — Message Signaled Interrupts (MSI)
- **What it does**: Switches modern PCIe graphics cards (NVIDIA RTX, AMD Radeon, Intel Arc) from legacy line-based IRQ sharing to dedicated PCIe Message Signaled Interrupts (MSI).
- **Registry Target**: `HKLM\SYSTEM\CurrentControlSet\Enum\PCI\{Device_ID}\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties\MSISupported`
- **Values**: Optimized = `1` (MSI Enabled), Default = `0`
- **Gain**: Perf +4, Stability: Low risk.
- **Benefit**: Eliminates shared IRQ polling delays, stabilizing 1% and 0.1% low frametimes in GPU-bound games.

### 2. `set_gpu_high_priority` — High Priority Interrupt Affinity
- **What it does**: Elevates the GPU's interrupt priority in the Windows kernel above non-essential peripherals (such as USB mouse, keyboards, webcams, or networking).
- **Registry Target**: `HKLM\SYSTEM\CurrentControlSet\Enum\PCI\{Device_ID}\Device Parameters\Interrupt Management\Affinity Policy\DevicePriority`
- **Values**: Optimized = `3` (`High`), Default = `2` (`Normal`)
- **Gain**: Perf +3, Stability: Low risk.

### 3. `enable_hags` — Hardware-Accelerated GPU Scheduling
- **What it does**: Offloads scheduling tasks from the CPU to the graphics card's dedicated GPU scheduling processor.
- **Registry Target**: `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\HwSchMode`
- **Values**: Optimized = `2` (Enabled), Default = `1` (Disabled)
- **Gain**: Perf +3, Stability: Low risk.

### 4. `disable_mpo` — Multi-Plane Overlay (MPO) Stutter Fix
- **What it does**: Disables DWM Multi-Plane Overlay (`OverlayTestMode = 5`). Permanently resolves desktop flickering, Discord screen share crashes, and VRR micro-stutters on Windows 11.
- **Registry Target**: `HKLM\SOFTWARE\Microsoft\Windows\Dwm\OverlayTestMode`
- **Values**: Optimized = `5` (MPO Disabled), Default = `0`
- **Gain**: Perf +4, Stability: Safe.

---

## 🎮 Vendor-Targeted Silicon Optimizations (v0.4.0)

### 5. NVIDIA GeForce / RTX Optimizations
- **`Disable NVIDIA Dynamic P-State` (`fa58802f-b98c-4e98-aa67-067c5a394296`)**: Sets `DisabledDynamicPstate = 1` in the primary display adapter class key, locking the GPU into fixed high performance state and eliminating clock fluctuation latencies.
- **`Disable NVIDIA Async P-States` (`9e7702be-9e03-4c96-a316-c4e435cb993e`)**: Sets `DisableAsyncPstates = 1`, enforcing deterministic synchronous clock switching for VR and frame-paced titles.
- **`NVIDIA Max Performance PowerMizer` (`5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c`)**: Sets `PowerMizerEnable = 1`, `PowerMizerLevel = 1`, `PowerMizerLevelAC = 1`.

### 6. AMD Radeon Silicon Optimizations
- **`Disable AMD ULPS` (`4433ce93-b632-4d67-ae71-0241c31099c0`)**: Sets `EnableUlps = 0` to disable Ultra Low Power State, eliminating sleep wake stutter on multi-core / crossfire configs.
- **`Disable AMD ASPM` (`8a4b4c4d-000e-42db-8bd7-293067f8fae7`)**: Sets `ASPMDisabled = 1`, preventing PCIe Active State Power Management bus link downshifting.
- **`Disable AMD Power Gating` (`a3836587-c461-4beb-9c4c-88493b36c138`)**: Sets `EnablePowerGating = 0` to keep shader engines running without dynamic gating pauses.
- **`Disable AMD Video Clock Gating` (`db379bb5-fc8c-4ad3-be32-11be09e76fba`)**: Sets `EnableVideoClockGating = 0` to eliminate DPC frame presentation spikes.

### 7. Intel Arc / UHD Optimizations
- **`Disable Intel Async Flips` (`df8739ff-e322-4958-b369-df584bb9b0c1`)**: Sets `AsyncFlips = 0` in `HKLM\SOFTWARE\Intel\GMM` for deterministic presentation.
- **`Disable Intel Adaptive V-Sync` (`ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054`)**: Sets `AdaptiveVsync = 0` in `HKLM\SOFTWARE\Intel\GMM`.

---

## 🔗 Related Notes
- [[GPU Low-Level Tuning & Vendor Targeting]]
- [[Hardware & GPU Low-Level Tuning]]
- [[Registry Lag & Kernel Scheduling]]
- [[00 - Index (MOC)]]
