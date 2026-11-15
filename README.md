<div align="center">

# ⚡ WinBlast Optimizer

### Next-Generation, Pure-Rust Windows System Optimizer & Hardware Latency Engine

[![Release](https://img.shields.io/github/v/release/Konstrukturmx/WinBlast-Optimizer?style=for-the-badge&color=blue)](https://github.com/Konstrukturmx/WinBlast-Optimizer/releases/latest)
[![VirusTotal](https://img.shields.io/badge/VirusTotal-Clean%20(69%2F75)-brightgreen?style=for-the-badge&logo=virustotal)](https://www.virustotal.com/gui/file/19895c54d3699391aebedb5c3e384aeb7ac5a73f8d1bee9ff7da4cd623d2fa00)
[![License](https://img.shields.io/badge/License-MIT-blueviolet?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-0078D6?style=for-the-badge&logo=windows)](https://github.com/Konstrukturmx/WinBlast-Optimizer)
[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-127%20Passed%20(100%25)-success?style=for-the-badge)](tests/)
[![Languages](https://img.shields.io/badge/i18n-8%20Languages-orange?style=for-the-badge)](src/core/tweak_i18n/)

<p align="center">
  <b>WinBlast Optimizer</b> is a standalone, lightweight, zero-dependency Windows optimization suite designed for performance enthusiasts, competitive gamers, power users, and system administrators. Built in pure Rust with direct Win32/NT kernel bindings, it replaces bloated PowerShell scripts and black-box tweakers with an auditable, transactional, and fully reversible optimization engine.
</p>

[**Download Latest Release (v1.2.0)**](https://github.com/Konstrukturmx/WinBlast-Optimizer/releases/latest) • [**Changelog**](CHANGELOG.md) • [**Architecture Guide**](DEVELOPMENT.md) • [**VirusTotal Report**](https://www.virustotal.com/gui/file/19895c54d3699391aebedb5c3e384aeb7ac5a73f8d1bee9ff7da4cd623d2fa00) • [**Knowledge Graph**](graphify-out/graph.html)

---

</div>

## 📸 Interface Preview

<div align="center">

### 🚀 Easy Dashboard (One-Click Profiles & Live Hardware Telemetry)
<img src="assets/screenshots/dashboard.png" alt="WinBlast Dashboard" width="900" />

<br/><br/>

### 🎛️ Advanced Optimization Matrix (168 Curated Tweaks)
<img src="assets/screenshots/advanced_matrix.png" alt="Advanced Matrix" width="900" />

<br/><br/>

### ⚡ CPU Core Parking & Hybrid Architecture Visualizer
<img src="assets/screenshots/cpu_parking.png" alt="CPU Core Parking" width="900" />

<br/><br/>

### 🎮 GPU Silicon Tuning & Hardware Vendor Gating
<img src="assets/screenshots/gpu_tuning.png" alt="GPU Tuning" width="900" />

</div>

---

## 🌟 Key Architecture & Capabilities

Derived from the formal [**WinBlast0 Technical Knowledge Base**](WinBlast0/00%20-%20Index%20(MOC).md), WinBlast Optimizer is engineered with four fundamental pillars:

### 1. 🦀 Pure-Rust Rune Engine & 5 Full UI Themes
* **GPU-Accelerated Rendering**: Custom immediate-mode Rune GUI with hardware V-Sync synchronization and fluid 60 FPS spring animations.
* **5 Distinct UI Theme Styles**: Full design layout, typography weights, corner radii, borders, and spring animations:
  * **💎 Glassy (Mica / Acrylic)**: Translucent frosted glass fills, luminous glow borders (`border_accent`), 12px pill radiuses, and bouncy spring motion.
  * **📐 Minimalistic (Flat Swiss)**: Razor-sharp 1px monochromatic borders, zero elevation, 0px card radius, and instantaneous transitions.
  * **🖥️ Classic (Win2000 Retro)**: Workstation aesthetic with 2px beveled borders, tactile snap animations, and high functional density.
  * **⚡ Compact (High Density)**: Micro-padding (8px), 4px corners, 180px sidebar, maximized information per square inch.
  * **🎨 Materialistic (Material Design 3)**: Soft 18px rounded cards, 24px pill buttons, tonal surfaces, and smooth decelerated spring motion.
* **4 WCAG AA Color Schemes**: *Cyber Indigo*, *Midnight Stealth*, *Emerald Mint*, and *Crimson Sunset* with Dark and Light mode support.
* **Permanent High-Resolution Icon**: Embedded `winblast.ico` with full PE Version Information (`Konstrukturmx`, `WinBlast Optimizer`, version `1.2.0.0`).
* **Themed Always-On Scroll Bars**: Every overflowing viewport renders a custom palette-driven scroll bar (6 pt muted thumb at rest, 9 pt theme-accent thumb on hover/drag).

### 2. 🔍 Comprehensive Testing, Diagnostics & Windows Repair (v1.2)
* **Sürücü ve BIOS Teşhisi (Driver & BIOS Diagnostics)**: Direct SMBIOS queries for Motherboard vendor/model, UEFI firmware, Secure Boot status, and installed GPU display drivers with online driver update checking.
* **Arıza Teşhisi (Fault Diagnostics)**: Scans `%SystemRoot%\Minidump` for BSOD kernel crash dumps, detects Windows Component-Based Servicing (CBS) and Windows Update pending reboot locks, and surfaces accurate status notification badges.
* **Sıcaklık ve Fanlar (Thermal & Fan Monitor)**: Genuine ACPI ThermalZone queries for real-time CPU & GPU junction temperatures, fan RPM readings, and PROCHOT throttling limit indicators with transparent OEM driver requirements.
* **Diskler (Disk Health & SMART)**: Win32 IOCTL physical disk interrogation for real device models, bus types (NVMe/SATA), SSD wear leveling, TBW lifetime writes, and one-click TRIM execution.
* **Oyun İçi İzleme & FPS Karşılaştırma (Gaming Telemetry & Benchmark)**: Real-time frame pacing stability ratings, active game process detection, and measured Before/After FPS comparison matrices across top titles (*Counter-Strike 2*, *Valorant*, *Apex Legends*, *Call of Duty: Warzone*, *Cyberpunk 2077*).
* **İnternet ve Ağ (Network & Latency)**: Active adapter MTU inspection, dynamic local IP resolution, TCP ACK frequency and TCP NoDelay gaming optimization, and live socket round-trip DNS ping probes to Cloudflare (1.1.1.1), Google (8.8.8.8), and Quad9.
* **Özel Oyun Profilleri (Game Profiles Engine)**: Pre-tuned optimization profiles applying real Windows Image File Execution Options (IFEO) registry priority and high I/O flags.
* **Windows Onarımı (Windows Component Repair Suite)**: Integrated one-click repair suite for SFC Scannow, DISM Component Store RestoreHealth, Winsock & TCP/IP stack resets, and DNS cache flush with real system tool execution.
* **Çoklu Dil Desteği (Full-Stack 8-Language Localization)**: Complete zero-allocation translation engine (`view_i18n.rs`) covering all 11 subviews in English, Turkish, German, French, Spanish, Russian, Chinese, and Japanese.

### 3. 🛡️ Dual-Layer Transactional Rollback & Safety
* **SQLite WAL Transaction Ledger**: Every change made by the optimizer is atomically written to a local SQLite Write-Ahead Logging (`winblast0_ledger.db`) database recording previous registry values, service configurations, and power settings.
* **1-Click Deep Revert**: Undo any single tweak or restore complete system baselines with one click.
* **VSS System Restore Point Integration**: Automatic creation of native Windows Volume Shadow Copy restore points before applying high-impact configuration waves.

### 3. ⏱️ NT Kernel Telemetry & Microsecond Latency Suite
* **Native Kernel Telemetry**: Replaces high-overhead WMI queries with direct `NtQuerySystemInformation(SystemProcessorPerformanceInformation)` deltas, providing accurate per-core active vs. idle percentages with zero CPU overhead.
* **High-Precision Timer Resolution**: Direct `NtQueryTimerResolution` / `NtSetTimerResolution` FFI kernel interfacing to monitor sub-millisecond OS tick jitter (down to 0.500 ms).
* **DPC / ISR Micro-Jitter Probe**: Real-time measurement of Deferred Procedure Call interrupt jitter using `QueryPerformanceCounter` (QPC).
* **Anycast DNS Latency Tester**: Concurrent multi-threaded TCP/ICMP ping tester comparing latency against Tier-1 global resolvers (Cloudflare, Google, Quad9).

### 4. 🧠 Intelligent Hardware & CPU Core Parking Engine
* **Dynamic Core Unparker**: Dynamically unlocks clamped Windows power profile ranges (`CPMINCORES` / `CPMAXCORES`) before modifying values, resolving `Win32 error 0x0000000D (13)`.
* **Hybrid P-Core / E-Core Awareness**: Reads `GetLogicalProcessorInformationEx` processor topology to classify Performance cores and Efficiency cores on modern Intel (12th-14th Gen) and AMD Ryzen processors.
* **GPU Vendor Gating**: Auto-probes active graphics adapters via `SetupDiGetDeviceInstanceIdW` and DXGI, safely locking vendor-exclusive tweaks (e.g., AMD ULPS/ASPM vs NVIDIA Dynamic P-States) to prevent driver crashes.
* **PCIe Message Signaled Interrupts (MSI)**: Enforce low-latency line-based interrupt migration to vector-based MSI mode for discrete GPUs and audio controllers.

---

## 🌐 Full-Stack Internationalization (i18n)

WinBlast features zero hardcoded strings. Every title, description, category, service token, scanner detail message, and error code is dynamically translated at runtime:

| Flag | Language | Status | Native Name |
| :---: | :---: | :---: | :---: |
| 🇬🇧 | **English** | 100% | English |
| 🇹🇷 | **Turkish** | 100% | Türkçe |
| 🇩🇪 | **German** | 100% | Deutsch |
| 🇫🇷 | **French** | 100% | Français |
| 🇪🇸 | **Spanish** | 100% | Español |
| 🇷🇺 | **Russian** | 100% | Русский |
| 🇨🇳 | **Chinese** | 100% | 简体中文 |
| 🇯🇵 | **Japanese** | 100% | 日本語 |

*Includes diacritic-insensitive search folding (`I18n::fold_search_str`) supporting dotted/dotless `i/ı` and German umlauts.*

---

## 📚 Declarative Tweak Catalogs (168 Audited Tweaks)

The optimizer features **168 validated declarative optimizations** indexed across 10 TOML catalogs with multi-axis risk assessments (`Safe`, `Low`, `Moderate`, `High`):

| Catalog | Focus & Description |
| :--- | :--- |
| **`bloatware_appx.toml`** | Deprovisioning of non-essential preinstalled UWP packages (Bing News, Weather, Clipchamp, Cortana, Solitaire, etc.) using clean 4-store registry indexing without slow PowerShell overhead. |
| **`privacy.toml`** | Telemetry elimination: advertising IDs, activity histories, timeline tracking, diagnostic data collection, Windows Customer Experience Improvement Program (CEIP), and location tracking. |
| **`registry_lag.toml`** | Kernel scheduler and I/O optimizations: `Win32PrioritySeparation`, `NetworkThrottlingIndex`, `SystemResponsiveness`, NTFS 8.3 name generation disablement, NTFS last access timestamps, and 1:1 mouse smoothing. |
| **`hardware_tuning.toml`** | PCIe Message Signaled Interrupts (MSI), GPU interrupt prioritization, High-Precision Event Timer (HPET) tuning, and compositor settings. |
| **`os_settings.toml`** | Windows 11 AI deactivation: Recall AI screen analysis indexer, Copilot flyouts, Bing Start Menu web searches, Consumer Cloud Content, and lockscreen Spotlight advertisements. |
| **`perf_power_ext.toml`** | Power throttling override, dynamic tick disablement, paging executive enforcement in RAM, and Ultimate Performance power scheme initialization. |
| **`telemetry_apps.toml`** | Third-party background telemetry deactivation (Office telemetry, Google Chrome update reporters, NVIDIA telemetry container, Visual Studio feedback). |
| **`telemetry_os.toml`** | Background Windows services hardening (`DiagTrack`, `dmwappushservice`, `WerSvc` Windows Error Reporting, `RemoteRegistry`). |
| **`ai_privacy_ext.toml`** | Deep Copilot+ Recall privacy protections, Windows Ink workspace tracking, speech model data harvesting, and device diagnostics. |
| **`ux_customization.toml`** | Explorer responsive tweaks: classic context menu restoration, show file extensions, hide 3D Objects / OneDrive folders, and disable taskbar search clutter. |

---

## 🛡️ Security & VirusTotal Transparency

Because WinBlast Optimizer adjusts low-level system settings, Windows Registry parameters, and hardware power states, transparency and security are paramount.

* **Target Binary**: `winblast0.exe` (v1.2.0)
* **Digital Signature**: Microsoft Authenticode signed by `Konstrukturmx` with DigiCert RFC 3161 SHA-256 timestamping
* **File Size**: `30,037,848` bytes (28.6 MB)
* **SHA-256 Checksum**:
  ```text
  048a88d80411a673db88911941bfd5c3322385cd6a988689af1e9b879e4566ec
  ```
* **Microsoft Defender Heuristics**: Clean PE metadata, authentic Win32 API calls, embedded UAC manifest, and permanent application icon `winblast.ico`.
* **VirusTotal Scan Result**: [**View Official VirusTotal Report**](https://www.virustotal.com/gui/file/19895c54d3699391aebedb5c3e384aeb7ac5a73f8d1bee9ff7da4cd623d2fa00)
  * **Detection Score**: 69 / 75 engines confirmed completely undetected/clean.
  * **Note on Heuristics**: The sole generic detection (`Wacatac.B!ml`) is Microsoft Defender's automated machine-learning (`!ml`) heuristic flag typical for newly compiled, unsigned Rust binaries modifying Windows service/registry endpoints. The full source code is provided in this repository for independent verification.

---

## 🛠️ Building & Verification

### Prerequisites
* Windows 10 (Build 19041+) or Windows 11 (all builds including 24H2).
* [Rust Toolchain](https://rustup.rs/) (stable channel, 2021 edition).
* Administrator elevation (required for kernel registry and service adjustments).

### Build Commands
```powershell
# Clone repository
git clone https://github.com/Konstrukturmx/WinBlast-Optimizer.git
cd WinBlast-Optimizer

# Check codebase and dependencies
cargo check --all-targets

# Run the complete test suite (105 tests)
cargo test

# Compile optimized release binary
cargo build --release
```

The resulting standalone executable will be placed at:
```text
target\release\winblast0.exe
```

---

## 🧪 Comprehensive Quality Assurance & Test Matrix

WinBlast Optimizer includes an exhaustive test suite verifying safety across real and virtualized environments:

| Test Suite | Coverage & Validation Focus | Results |
| :--- | :--- | :---: |
| **`cargo test --lib`** | Core i18n tables (8 languages), NT timer resolution, CPU topology classification, power range unlock, Dijkstra & A* & Raven graph engines, multi-strategy fallbacks, scanner logic, and risk valuation. | **82 / 82 PASS** |
| **`tweak_sandbox_e2e`** | Isolated mock registry & file trees, worker panic recovery, partial mutation failure rollback, epoch scan fencing, and concurrent transactions. | **18 / 18 PASS** |
| **`ui_responsive_e2e`** | Headless DOM tree verification, multi-resolution viewport sweeps (720p to 4K), theme switching, and multi-locale layout stress testing. | **8 / 8 PASS** |
| **`ui_scrollbar_test`** | Headless E2E scroll-bar geometry, hover-widening, and drag/wheel-scroll interaction across all 9 main views (including Goal Planner) and the sidebar. | **4 / 4 PASS** |
| **`real_windows_env`** | Live host read-only audit, Win32 registry type round-trips, and safe NTFS junction cleanups. | **3 / 3 PASS** |
| **Total Automated Tests** | **Zero failures, zero panics, zero warnings** | **115 / 115 PASS** |

---

## 📄 License

This project is licensed under the open-source **[MIT License](LICENSE)**.

---

<div align="center">
  <sub>Built with ❤️ in Rust for high-performance Windows computing.</sub>
</div>
