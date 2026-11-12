# Changelog

All notable changes to **WinBlast Optimizer** are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v0.9.0] - 2026-11-12

### 🌟 Highlights
- **Program-Wide Dynamic Localization**: Zero hardcoded UI strings across all 8 supported languages. Every tweak title, description, category, scan result detail, and status event is translated dynamically at runtime.
- **CPU Core Parking Range Unlock**: Resolved `Win32 error 0x0000000D (13)` (`ERROR_INVALID_DATA`) on Windows 11 (including 24H2) by dynamically removing clamped power profile range limits before mutation.
- **NT Kernel Utilization Sampler**: Replaced uninitialized cold-start metrics with kernel-level `NtQuerySystemInformation(SystemProcessorPerformanceInformation)` deltas, fixing false 100% CPU utilization reporting.
- **Hybrid P/E Core Topology & Rollback**: Integrated `GetLogicalProcessorInformationEx` topology classification with `ParkingSnapshot` atomic rollback in the state ledger.
- **GitHub Initial Release & VirusTotal Scan**: Official open-source release with pre-compiled 64-bit binary, MIT License, and clean VirusTotal report.

### Added
- **Full Tweak Localization Engines (`src/core/tweak_i18n/`)**:
  - Comprehensive dictionaries for all 168 tweaks in English, Turkish, German, French, Spanish, Russian, Chinese, and Japanese.
  - Localized dictionary lookup for WinGet curated applications, Windows Optional Features, and background telemetry services.
- **Structured Scanner Outcomes (`ScanDetailKind`)**:
  - Converted all 48 scanner result branches into strongly typed `ScanDetailKind` variants, allowing dynamic localized formatting (e.g., Turkish `Kaldırıldı / Yüklü Değil (İyileştirildi)`).
- **Structured Status Messaging (`StatusMessageSpec`)**:
  - Replaced raw string formatting in `AppState` with structured status specifications that update immediately upon switching languages.
- **Color Scheme & Changelog i18n**:
  - Localized bottom transaction changelog actions (`Apply`, `Revert`, `Delete`) and theme scheme button labels.
- **Power Range Unlocking (`src/hardware/power.rs`)**:
  - Added `unlock_power_setting_range_if_clamped()` to unclamp `CPMINCORES` / `CPMAXCORES` schema ranges before invoking `PowerWriteACValueIndex`.

### Fixed
- Fixed cold-start 100% CPU utilization spikes by tracking true kernel idle time differentials.
- Fixed core parking monitor loop panic safety using `catch_unwind`.
- Fixed foreign GPU vendor selection where incompatible vendor tweaks could be toggled.
- Fixed UI layout truncation across localized strings in compact and tablet viewports.

---

## [v0.8.0] - 2026-11-10

### 🌟 Highlights
- Dedicated Elephant-Goldfish Model (EGM) adversarial debugging and logic-hardening release.
- Resolved 40+ subtle edge cases across catalog ground truth (168 tweaks), Win32 FFI bindings, SCM argument parsing, and UI concurrency.
- Validated against live Windows 11 Build 26300 (`26H2`) and an isolated 64-bit registry sandbox across 77 automated tests.

### Fixed & Improved
- **Catalog Ground Truth & OS Detection**:
  - Added `KEY_WOW64_64KEY` to `CurrentVersion` queries and fallback for Windows 10 Build 19041 (`2004`).
  - Added support for 29 numeric `REG_SZ` keys handling negative decimal values.
  - Strict dispatch for `read_dword_from_key` by `REG_*` type eliminating UTF-16LE byte truncation.
  - Blocked shallow deletions of critical system subtrees (`SOFTWARE\Microsoft\Windows\CurrentVersion`).
  - Corrected Paint AI, Copilot hardware key, Recall 24H2 nested task paths, and MMCSS priority defaults.
- **Service Control Manager & Tasks**:
  - Fixed single-arg `"start= auto"` error in `sc config` by properly splitting arguments into `["config", svc, "start=", mode]`.
  - Preserved and restored `DelayedAutostart` (`start= delayed-auto`).
  - Handled COM `ITaskService` access-denied discrimination and decoded OEM code page (`CP_OEMCP`).
  - Fixed Win11 24H2 `" Exp"` suffix tasks and stripped UTF-8 BOM from `hosts` file operations.
- **Executor & Rollback Ledger**:
  - Enforced `is_gpu_vendor_compatible` across all scan and apply operations to prevent cross-adapter mutations.
  - Implemented reverse atomic rollback order in `revert_tweak_with`.
  - Added RAII panic recovery guards (`spawn_guarded_worker`) and epoch fencing in `AppState`.
- **Tools & Hardware**:
  - Safe NTFS junction cleaning in Disk Cleaner with `FILE_ATTRIBUTE_READONLY` stripping.
  - Real PCI instance ID mapping via `SetupDiGetDeviceInstanceIdW`.
  - Replaced heuristic HT classification with `GetLogicalProcessorInformationEx(RelationProcessorCore)` `EfficiencyClass` topology.

---

## [v0.7.0] - 2026-11-09

### Added
- **Integrated Winhance & optimizerDuck Catalogs**:
  - Expanded declarative catalog from 99 to 168 vetted optimizations without duplication or registry overlap.
- **`Clean & Apps` Suite**:
  - Full-featured UWP app deprovisioner with publisher-scoped package removal.
  - Curated WinGet installer supporting Essential Browsers, Developer Runtimes, and Utilities.
  - Windows Optional Features manager (`DISM` capability integration).
- **`Startup & Tasks` Manager**:
  - Win32 SCM background service manager with critical service protection blocklists.
  - Startup tasks manager reading `Run`, `RunOnce`, and `StartupApproved` keys.
- **`autounattend.xml` Generator**:
  - Automated unattend XML generator for streamlined, debloated Windows installations (`amd64` and `arm64`).

---

## [v0.6.0] - 2026-11-08

### Added
- **`SystemBackend` Mocking & Sandbox Testing Framework**:
  - Object-safe backend trait with full in-memory registry, service, and task scheduling emulation.
  - Automated sandbox testing covering all declarative tweaks.
- **Responsive Layout Matrix**:
  - Adaptive UI support scaling from 420×400 compact laptop screens to 3840×2160 4K displays.
  - Virtualized row rendering (`El::virtual_rows`) for ultra-fast list scrolling.
- **Theme Contrast & Accessibility**:
  - High-contrast adjustments for Light Mode meeting WCAG AA requirements.

---

## [v0.5.0] - 2026-11-07

### Added
- **Comprehensive Turkish Localization Suite**:
  - Initial 100% Turkish translation and diacritic-insensitive search folding (`I18n::fold_search_str`).
- **Interactive Progress & Changelog Drawer**:
  - Bottom expandable changelog ledger showing transaction history in real-time.
  - Animated spring progress bar tracking staged vs. applied optimizations.

---

## [v0.4.0] - 2026-11-06

### Added
- **Dedicated GPU Tuning Pipeline**:
  - Hardware silicon vendor targeting (NVIDIA, AMD, Intel).
  - Conditional vendor locking to prevent applying foreign GPU tweaks.
  - PCIe Message Signaled Interrupt (MSI) configuration and vector interrupt prioritization.
- **Redesigned User Interface**:
  - Two distinct user paradigms: **Easy Dashboard** for one-click presets and **Advanced Matrix** for fine-grained control.

---

## [v0.3.0] - 2026-11-05

### Added
- **CPU Core Parking Engine**:
  - Kernel power scheme integration with real-time logical core awake/sleep visualization.
- **Network & Latency Benchmarking Suite**:
  - Sub-millisecond DPC/ISR jitter probe using QPC timer.
  - Multi-target Anycast DNS ping benchmark.
- **Subprocess Stealth**:
  - Zero-flicker execution engine with `CREATE_NO_WINDOW` and standard descriptor redirection.

---

## [v0.2.0] - 2026-11-03

### Added
- **Rune GPU GUI Framework**:
  - Replaced legacy web/shell frontend with a pure-Rust, hardware-accelerated immediate mode interface.
- **SQLite WAL State Ledger**:
  - Local transaction database recording pre-optimization states for guaranteed atomic rollbacks.
- **Windows System Restore Point (VSS) Integration**:
  - Pre-flight restore point creation before executing batch optimizations.

---

## [v0.1.0] - 2026-10-28

### Added
- Initial project prototype.
- Declarative TOML tweak catalog format.
- Basic 64-bit Windows Registry scanning and mutation engine.
- UWP bloatware identification.
