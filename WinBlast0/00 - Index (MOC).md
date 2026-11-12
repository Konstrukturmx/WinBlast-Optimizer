---
title: WinBlast0 Knowledge Base & System Index
tags: [winblast0, moc, index, rust, optimization]
created: 2026-11-08
updated: 2026-11-10
version: v0.8.0
---

# ⚡ WinBlast0 Knowledge Base (MOC)

Welcome to the **WinBlast0 Technical Knowledge Base**. This vault indexes the complete architecture, declarative tweak catalogs, scanning detection engines, real-time hardware telemetry, dual-layer transactional rollback ledger, and engineering history of the WinBlast0 pure-Rust system optimizer.

> [!NOTE]
> **Token Optimization**: When asking questions or making modifications in future AI sessions, refer to specific note titles (e.g. `[[Registry Lag & Kernel Scheduling]]` or `[[State Ledger & Rollback Engine]]`) to selectively ingest only relevant context and minimize token burn.

---

## 🗺️ Map of Content (Navigation)

### 🏗️ 1. System Architecture
- [[SystemBackend & Sandbox Test Environment]] — Object-safe `SystemBackend` trait, isolated `SandboxEnv` (1-store registry & recursive trees), `PanickingBackend` / `SlowScanBackend` / `FailAtMutationBackend`, and real Win32 / NTFS junction sandboxes ([tests/real_windows_env.rs](file:///c:/Hive/Projects/WinBlast0/tests/real_windows_env.rs)).
- [[Comprehensive i18n & Localization Architecture]] — Exhaustive zero-allocation string catalog (`320` keys × `8` languages), native Turkish suite + `I18n::fold_search_str`.
- [[Responsive UI, Animations & Changelog Ledger]] — Zero-echo `LayoutTier`, 10 views + 6 sub-tabs, virtualized rows (`El::virtual_rows`), `switch_indicator`/`switch_toggle_with`, WCAG contrast fixes, spring progress bar, and newest-first changelog drawer.
- [[GPU Low-Level Tuning & Vendor Targeting]] — Dedicated GPU pipeline tab, silicon vendor targeting (NVIDIA, AMD, Intel), conditional locking, and PCIe MSI routing.
- [[Pure-Rust Rune UI Engine]] — GPU-rendered framework, reactive state machine, 60 FPS motion & hover animations.
- [[State Ledger & Rollback Engine]] — SQLite WAL transactional ledger, bidirectional mutation engine (`apply_tweak` / `revert_tweak`), baseline reset API.
- [[Multi-Engine Scanning Subsystem]] — State-of-the-art detection for Registry (64/32-bit), Services, Scheduled Tasks (multilingual), AppX, and instant targeted single-tweak evaluation (`scan_single_tweak`).
- [[Real-Time Task Manager Monitoring]] — 1-second continuous telemetry sampling, waveform sparklines, NT kernel timer resolution.
- [[Hardware & GPU Low-Level Tuning]] — PCIe MSI interrupt routing, HAGS scheduling, MPO stutter fixes, and hybrid CPU core topology.
- [[CPU Core Parking & Dynamic Scheduling]] — Kernel power scheme core thresholds, live topology visualizer, 1-click unpark engine.
- [[System & Network Latency Benchmarking]] — DPC/ISR micro-jitter probe, Tier-1 DNS anycast ping tester, Before/After delta comparison.
- [[Internationalization & Multi-Theme System]] — 8-language zero-allocation i18n (Turkish priority), Dark/Light modes, 4 WCAG AA color schemes.

### 📜 2. Declarative Tweak Catalogs (100+ Tweaks)
- [[Open-Source Tweaks & System Hardening]] — Reverse-engineered & curated tweaks from Sophia Script, AtlasOS, Chris Titus WinUtil, ReviOS, and Optimizer.
- [[Catalog Overview & Risk Model]] — Multi-axis risk valuation (`Safe`, `Low`, `Moderate`, `High`), scoring, and zero-fail embedded fallbacks.
- [[Registry Lag & Kernel Scheduling]] — Win32PrioritySeparation, NetworkThrottlingIndex, SystemResponsiveness, NTFS timestamps, NTFS 8.3, TCP latency, 1:1 mouse input, paging executive, and Fast Startup.
- [[OS Settings & Windows 11 AI]] — Windows Copilot, Recall AI screen analysis, Bing Start menu web search, GameDVR overlay capture, Consumer Cloud Content, taskbar Widgets flyout, and UI animations.
- [[Privacy & Telemetry Shield]] — Activity history, CDP service, advertising IDs, Spotlight lockscreen ads, WerSvc, RemoteRegistry, WinSAT task, and location tracking.
- [[Bloatware & AppX Deprovisioning]] — 4-store registry indexing, sub-millisecond deprovisioning, package removal without PowerShell overhead.
- [[Hardware Tuning & MSI Mode]] — Message Signaled Interrupts (MSI) configuration, interrupt priorities, and compositor switches.

### ⚙️ 3. Core & Windows Internals
- [[Windows OS Detection & Edition Aliases]] — Native NT build query (`RtlGetVersion`) and flexible edition mapping (`Professional` / `Pro` / `EnterpriseS` / `Core`).
- [[Privilege & Elevation Token Verification]] — Win32 `OpenProcessToken` elevation verification, UAC `HighestAvailable` manifest handling.
- [[NT Kernel Timer Resolution (ntdll)]] — Direct `NtQueryTimerResolution` FFI kernel query, high-resolution timer dynamics.
- [[SQLite WAL Transaction Ledger]] — Concurrency configuration, 5-second busy timeout, `baselines` table for before/after performance comparison.

### 🧪 4. Testing, Auditing & Verification
- [[Test Suite & Validation Matrix]] ... Automated unit tests, catalog deserialization checks, live Windows 11 Build 26300 scan verification.
- [[Debugging & Edge Cases Resolved]] — Findings and fixes from the dedicated `winblast-debugger-fixer` subagent (slice panics, DB locks, hosts corruption).

### 📈 5. Project History & Roadmap
- [[Changelog v0.7.0 to v0.8.0]] — Version 0.8.0: Dedicated EGM adversarial debugging & logic-fixing release: 40+ bugs resolved across catalog ground truth (`168` tweaks), `sc config` arg splitting + `DelayedAutostart`, `read_dword_from_key` strict `REG_*` dispatch, `SetupDiGetDeviceInstanceIdW` GPU PnP routing, `GetLogicalProcessorInformationEx` `EfficiencyClass` P/E topology, `app.rs` RAII worker-panic guards & epoch fencing, `autounattend.xml` Setup-safe `reg delete`, and `NotApplicable`/`[Revert]` `DiffModal` UI logic — verified across **77 tests**.
- [[Changelog v0.6.0 to v0.7.0]] — Version 0.7.0: Deduplicated Winhance + optimizerDuck tweaks & features (168 tweaks across 10 catalogs), `CleanAndApps` & `StartupAndTasks` views, platform Hardware/Power/`autounattend.xml` generator, 313-key i18n, full rollback ledger parity, verified across 55 tests (`winblast-tester` + `winblast-debugger-fixer`).
- [[Changelog v0.5.0 to v0.6.0]] — Version 0.6.0: `SystemBackend` sandbox E2E testing across all 99 tweaks, full responsiveness (420x400 to 3840x2160), 185-key i18n, Light Mode contrast fixes, AppX/SCM/OEM fixes, verified by `winblast-tester` + `winblast-debugger-fixer`.
- [[Changelog v0.4.0 to v0.5.0]] — Version 0.5.0: 100% i18n coverage, Turkish suite, responsive UI, spring animations, live progress bar, bottom changelog drawer, and open-source tweak additions.
- [[Changelog v0.3.0 to v0.4.0]] — Version 0.4.0: Dedicated GPU tab, Hardware silicon vendor targeting (NVIDIA/AMD/Intel), conditional locking, MSI migration, and Layout redesign.
- [[Changelog v0.2.0 to v0.3.0]] — Version 0.3.0: Core parking, Latency Suite, i18n (Turkish), Themes, and Subprocess Stealth.
- [[Changelog v0.1.0 to v0.2.0]] — Detailed evolution from initial release to the v0.2.0 Next-Generation system.
- [[Roadmap & Future Vision]] — Future enhancements: driver telemetry stripping, network adapter TCP/IP offload tuning, automated baseline benchmarking.

---

## 📊 Quick System Snapshot (v0.8.0)

| Metric | Value |
| :--- | :--- |
| **Total Tweaks** | 168 validated declarative optimizations across 10 TOML catalogs (0 conflicts, 0 duplicates) |
| **Silicon & Setup** | `SetupDiGetDeviceInstanceIdW` multi-GPU isolation + Windows `autounattend.xml` (`amd64` / `arm64`) |
| **System Tools** | Disk Cleaner (COM Recycle Bin delta, NTFS junction safe), AppX `_`/ResourceId matcher, DISM/WinGet, Win32 SCM & Startup |
| **Embedding** | 100% embedded into standalone `dist\winblast0.exe` `.rdata` section (`28,108,288` bytes) |
| **GUI Framework** | Pure-Rust Rune Engine (`10` responsive views, `El::virtual_rows`, `[Revert]` DiffModal, `NotApplicable` locking) |
| **Execution Engine** | Dual Mode: Instant & Normal with RAII panic-recovery workers, epoch scan fencing, and atomic revert/rollback |
| **Languages (i18n)** | 8 Languages (`320` typed keys × `8` locales + Turkish-safe search folding) |
| **Theming** | Light & Dark Modes + 4 WCAG AA color schemes |
| **Subprocess Window** | 100% Invisible (`SilentCommand` with `CREATE_NO_WINDOW`) |
| **CPU Core Parking** | `GetLogicalProcessorInformationEx` P/E topology + sampled worker parking + `powrprof` (`Rocket = 2`) |
| **Latency Suite** | Microsecond QPC DPC Probe + Tier-1 DNS Ping Jitter Tester |
| **Telemetry Rate** | Continuous 1.0 second non-blocking sampling (Task Manager style) |
| **Rollback Safety** | Dual-Layer: Windows System Restore Points (VSS) + SQLite WAL Transactional Ledger |
| **Verified Tests** | **77/77 passed** (`52` lib + `3` live Build `26300`/Win32/NTFS + `18` sandbox/fault E2E + `4` UI E2E) & 0 clippy warnings |
