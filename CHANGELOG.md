# Changelog

All notable changes to **WinBlast Optimizer** are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v1.5.0] - 2026-11-16

### 🌟 Landmark Highlights
- **In-App GitHub Releases Self-Updater (`src/core/updater.rs`)**:
  - Direct HTTPS API integration checking GitHub releases with semver comparison.
  - Zero-dependency in-place binary upgrade: renames current executable to `.old`, moves new release into place, and automatically relaunches.
- **Administrator Elevation Guard & 1-Click Relaunch (`src/core/elevation.rs`)**:
  - Pre-flight `OpenProcessToken` and `TokenElevation` interrogation on startup.
  - Contextual warning card on Dashboard with one-click UAC elevation via `ShellExecuteW("runas")`.
- **Game Prerequisites & Runtimes Cascade Installer (`src/tools/game_prereqs.rs`)**:
  - Full audit and installation of essential runtimes (DirectX, Visual C++ 2005-2022 All-in-One, Vulkan, .NET Desktop Runtime 8.0/9.0, OpenAL, XNA).
  - 3-tier fallback engine: WinGet silent install -> Chocolatey CLI -> Direct official vendor website launch.
- **Unified Gaming & Performance Hub (`src/ui/views/gaming_stats.rs`)**:
  - Consolidated scattered gaming tools into a unified 5-tab workspace: Real-Time Monitor, FPS Benchmark, Game Profiles (IFEO registry tuning), In-Game HUD, and Esports Game Suitability.
- **Multi-Axis Sorting Engine Across All Data Views**:
  - Dynamic 1-click sorting for Advanced Matrix (Name, Category, Risk, Status), Clean & Apps (Size, Name), Startup & Tasks (Source, Status), Drivers (Class, Vendor, Updates), and Game Prerequisites (Missing First).
- **Comprehensive UI Bug Fixes & Scrollbar Gutter Overhaul**:
  - Added dedicated right-side padding (`.pr(14.0)` to `.pr(16.0)`) across all scrollable viewports, completely eliminating scrollbar thumb overlap with action buttons.
  - Redesigned WinGet application cards into a two-tier layout, eliminating text wrapping and badge clipping.
- **Expanded Declarative Tweak Catalogs (208 Total Tweaks)**:
  - Added `assets/rules/gaming_latency_ultra.toml` and `assets/rules/system_deep_opt.toml`, expanding catalog to 208 vetted optimizations.
  - Expanded zero-allocation 8-language localization to 510 `TextKey` variants with 100% full coverage.
- **Exhaustive Automated QA (154 Tests)**:
  - 154 automated tests passing with 0 warnings and 0 compiler errors.

---

## [v1.2.0] - 2026-11-15

### 🌟 Highlights
- **Zero-Allocation View Localization (8 Languages)**: Implemented `src/core/view_i18n.rs` with 88 zero-allocation string catalog keys covering all extended subviews in English, Turkish, German, French, Spanish, Russian, Chinese, and Japanese.
- **Authentic Win32 Hardware Interrogation**: Replaced placeholder telemetry with direct `IOCTL_STORAGE_QUERY_PROPERTY` physical disk queries, ACPI ThermalZone telemetry, and real Windows IFEO registry mutations.
- **Windows Component Repair Suite**: Integrated one-click repair actions for `sfc /scannow`, `dism /Online /Cleanup-Image /RestoreHealth`, `ipconfig /flushdns`, and Winsock resets.
- **UI Thread Fluidity**: Background worker benchmark execution and accurate fault diagnostics status clamp removal.

---

## [v1.1.0] - 2026-11-15

### 🌟 Highlights
- **Online Driver Scanning & Hardware SMBIOS Diagnostics**: Direct SMBIOS motherboard, UEFI, and GPU driver inspection with deep-links to manufacturer update portals.
- **Sidebar & Menu Localization**: Complete 8-language localization of sidebar navigation, theme switchers, and quick action cards.
- **Dashboard Redesign**: Integrated real-time CPU/RAM activity waveforms and system baseline delta meters.

---

## [v1.0.0] - 2026-11-15

### 🌟 Landmark Highlights
- **Graph-Theoretic Optimization Engine**:
  - **Dijkstra's Shortest-Path & Topological Scheduler** (`src/graph/dijkstra.rs`): Sequences tweak applications in topological prerequisite order while minimizing service disruption penalties and grouping reboot requirements at the tail of execution.
  - **A* Heuristic Goal Planner** (`src/graph/astar.rs`): Solves multi-objective optimization profiles (Ultra Gaming Rig, Stealth Privacy Fortress, Balanced Daily Driver) to satisfy target performance and privacy gains with minimal risk friction.
  - **Raven Multi-Criteria Decision & Pareto Engine** (`src/graph/raven.rs`): Evaluates Pareto dominance across Performance, Privacy, Stability, and Reversibility, automatically detecting and resolving mutual exclusions between conflicting tweaks.
- **Multi-Strategy Fallback & Resilient Mutation Engine**:
  - Tweaks now support cascaded `fallbacks: Vec<Vec<TweakAction>>` across Registry, Services, and Windows Policies. If a primary mutation encounters permissions or policy locks, the executor sequentially attempts fallback strategies with atomic intermediate rollback.
- **Interactive Rune 1.0 Goal Planner View** (`src/ui/views/goal_planner.rs`):
  - Dedicated interactive UI view allowing users to select target optimization goals, explore A* heuristic paths in real time, inspect Raven Pareto dominance scores, and view the step-by-step topological execution timeline.
- **`rune-motion` Physics & Spring Animation Integration**:
  - Integrated `rune-motion` spring animation physics for fluid interactive switches, cards, and view transitions.
- **Exhaustive Automated QA**:
  - Test suite expanded to 115 passing tests across 5 test suites (`cargo test --lib`, `tweak_sandbox_e2e`, `ui_responsive_e2e`, `ui_scrollbar_test`, and `real_windows_env`) with 100% pass rate and zero warnings.

---

## [v0.9.1] - 2026-11-13

### 🌟 Highlights
- **Themed Scroll Bars Across the Entire UI**: Every overflowing viewport — all main views, the sidebar navigation, the changelog drawer, the diff-preview modal, and the Clean & Apps / Startup & Tasks sub-lists — now renders a custom palette-driven Rune-Kit scroll bar with a muted 6 pt thumb at rest that widens to a 9 pt theme-accent thumb on hover and while dragging.
- **Always Visible by Default**: Windows 11 hides scroll bars until the user scrolls; WinBlast now explicitly forces `ScrollBars::Always` at launch so scroll affordances are always discoverable, while an explicit `--scroll-bars MODE` command-line override is still honoured.
- **Regression-Tested**: Four new headless E2E tests (geometry, presence across all viewports, hover-widening, and drag + wheel-scroll interaction) bring the automated suite to 105 passing tests.

### Added
- **`src/ui/widgets/scrollbar.rs`**: `WinBlastScrollable` extension trait plus `winblast_scrollbar_style()`, building the themed `ScrollBarStyle` from the active light/dark `ThemePalette` (`text_muted` thumb at rest, `accent` thumb on hover/drag).
- **Scroll-bar integration** in `easy_dashboard`, `advanced_matrix`, `gpu_tuning`, `cpu_parking`, `hardware_view`, `privacy_shield`, `backup_center`, `latency_bench`, `clean_apps`, `startup_tasks`, the sidebar navigation, the changelog drawer, and the diff modal.
- **`tests/ui_scrollbar_test.rs`**: headless E2E coverage that asserts a `scroll-bar` + `thumb` are rendered for every main view, validates thumb geometry and hover-widening, and verifies drag and wheel scrolling move the viewport.

### Changed
- **Version Footer**: `FooterStatus` now derives its version from `CARGO_PKG_VERSION` at compile time instead of eight hardcoded `v0.9.0` literals, so the footer can never drift from the released build.
- **README Screenshots**: re-rendered with the shipped scroll-bar default so previews match the released UI; QA matrix counts refreshed to 105 tests.
- **`main.rs` Launch Args**: Rune now receives its flags without the executable path (matching `App::args`), and the forced `--scroll-bars always` default is only appended when the user has not supplied their own mode.

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
