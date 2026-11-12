# Changelog: v0.3.0 → v0.4.0

## Release Summary
WinBlast0 v0.4.0 is a major milestone release delivering a dedicated **GPU Optimization Subsystem**, **Hardware Silicon Vendor Targeting**, **PCIe MSI Interrupt Migration**, and a comprehensive **UX & App Layout Redesign** tailored for both novice users and enthusiast power users.

---

## 1. Key New Features

### 🎮 Dedicated GPU Optimization Tab
- Added a dedicated `ActiveView::GpuTuning` view in Rune UI.
- Migrated PCIe Message Signaled Interrupts (MSI Mode) and device priority selectors from the general hardware view into the GPU tab.
- Integrated OS display engine switches:
  - Hardware-Accelerated GPU Scheduling (HAGS) toggle.
  - Multi-Plane Overlay (MPO) stutter mitigation toggle.
- Added live Detected Graphics Hardware banner displaying adapter index, full device name, PnP instance ID, and active vendor chip.

### 🛡️ Hardware Silicon Vendor Targeting & Conditional Locking
- Added `GpuVendor` enum (`Nvidia`, `Amd`, `Intel`, `Unknown`) with hardware PCI Vendor ID detection (`VEN_10DE`, `VEN_1002`/`VEN_1022`, `VEN_8086`) and driver string fallbacks.
- Added `#[serde(default)] pub gpu_vendor: Option<String>` to `TweakDefinition`.
- Added vendor silicon tweaks with authentic UUID identifiers:
  - **NVIDIA**: `Disable NVIDIA Dynamic P-State`, `Disable NVIDIA Async P-States`, `NVIDIA Max Performance PowerMizer`.
  - **AMD**: `Disable AMD ULPS`, `Disable AMD ASPM`, `Disable AMD Power Gating`, `Disable AMD Video Clock Gating`.
  - **Intel**: `Disable Intel Async Flips`, `Disable Intel Adaptive V-Sync`.
- Implemented **Conditional Incompatibility Cards**:
  - Incompatible cards display warning badge `⚠️ Not supported on this system`, tweak UUID, bold title, and amber alert box: `Condition failed: {Vendor} GPU required. This item only works on systems with an {Vendor} graphics card.`
  - Incompatible cards are visually dimmed with interactions safely disabled.
  - Compatible cards display vendor and silicon badges, active toggle switches, risk ratings, and diff inspection `< / >`.

### 🧭 App Layout & UX Redesign ("From Anyone to Power Users")
- Re-architected sidebar navigation into structured, logical sections:
  - **OPTIMIZATION MODES**:
    - `⚡ Easy Dashboard` (Curated 1-click profiles, system health score, safe-only tweaks).
    - `🎛️ Advanced Matrix` (Full granular tree, category pills, registry paths, dry-run diffs).
  - **HARDWARE ENGINES**:
    - `🎮 GPU Optimization` (Consolidated vendor silicon, MSI mode, HAGS, MPO).
    - `⚙️ CPU Core Parking` (Live 1s per-core park status, 1-click unpark/restore).
    - `⚡ Latency Benchmark` (QPC micro-jitter probe, multithreaded DNS latency test).
    - `🖥️ CPU & Diagnostics` (1-second Task Manager waveforms, NT timer resolution, process count).
  - **SYSTEM & SAFETY**:
    - `🛡️ Privacy Shield` (Hosts blocker, telemetry audit, diagtrack status).
    - `💾 Backup Center` (SQLite WAL transaction ledger, restore point management).

### 🌐 Internationalization & String Expansion
- Added 5 new localized keys to `TextKey`:
  - `GpuTuning`
  - `NotSupportedOnSystem`
  - `ConditionFailed`
  - `WhatIsThis`
  - `Hide`
- Exhaustively populated across all 8 supported languages: English, Turkish, German, French, Spanish, Russian, Chinese, Japanese.

---

## 2. Verification & Metrics
- Unit test suite expanded to **20/20 tests passing**.
- `cargo clippy` passes with **0 warnings**.
- Tested on physical NVIDIA RTX 5070 hardware:
  - NVIDIA silicon tweaks correctly evaluate to active and operational.
  - AMD and Intel silicon tweaks reliably lock with condition failed alert banners.
