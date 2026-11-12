---
title: Changelog — v0.1.0 to v0.2.0
tags: [changelog, history, releases, v020]
created: 2026-11-08
---

# 📜 Changelog — WinBlast0 Evolution

---

## 🚀 Version 0.2.0 (Next-Generation Release) — 2026-11-08

### ✨ New Features:
- **Expanded Declarative Tweak Catalog**: Expanded from 55 to **84 tweaks** across 7 catalogs.
  - Added Windows 11 AI bloatware controls: Windows Copilot (`TurnOffWindowsCopilot`) and Windows Recall (`DisableAIDataAnalysis`).
  - Added Start menu Bing search disabling (`DisableSearchBoxSuggestions`, `BingSearchEnabled`).
  - Added Taskbar Widgets & MSN news feed disabling (`AllowNewsAndInterests`).
  - Added network packet throttling fix (`NetworkThrottlingIndex = 0xFFFFFFFF`).
  - Added system responsiveness gaming priority (`SystemResponsiveness = 0`).
  - Added NTFS metadata last access timestamp disabling (`NtfsDisableLastAccessUpdate = 1`).
  - Added NTFS 8.3 short filename disabling (`NtfsDisable8dot3NameCreation = 1`).
  - Added NTFS Master File Table zone reservation (`NtfsMftZoneReservation = 2`).
  - Added TCP low latency delayed ACK disabling (`TcpAckFrequency = 1`).
  - Added TCP Nagle's algorithm bypass (`TCPNoDelay = 1`).
  - Added 1:1 raw mouse input curve disabling (`MouseSpeed = 0`).
  - Added Xbox GameDVR background overlay capture disabling (`GameDVR_Enabled = 0`).
  - Added GameDVR app capture buffer disabling (`AppCaptureEnabled = 0`).
  - Added GameDVR system-wide Group Policy lock (`AllowGameDVR = 0`).
  - Added Windows Consumer Cloud Features / sponsored app disabling (`DisableWindowsConsumerFeatures = 1`).
  - Added Windows Spotlight wallpaper background download disabling (`RotatingLockScreenEnabled = 0`).
  - Added Lock screen tips and suggested store app promotions disabling (`SubscribedContent-338387Enabled = 0`).
  - Added Windows Error Reporting service disabling (`WerSvc = 4`).
  - Added Remote Registry service network lockdown (`RemoteRegistry = 4`).
  - Added Disk Diagnostic Telemetry scheduled task disabling (`Microsoft-Windows-DiskDiagnosticDataCollector`).
  - Added WinSAT background stress benchmark scheduled task disabling (`WinSAT`).
  - Added kernel executive RAM residency (`DisablePagingExecutive = 1`).
  - Added large system file cache (`LargeSystemCache = 1`).
  - Added Fast Startup hybrid sleep leak prevention (`HiberbootEnabled = 0`).
  - Added Activity History & Timeline cloud upload disabling (`PublishUserActivities`, `UploadUserActivities`).
  - Added Connected Devices Platform disabling (`CDPUserSvc = 4`).
  - Added Delivery Optimization P2P upload bandwidth stealing fix (`DODownloadMode = 0`).
  - Added Diagnostic Execution Service disabling (`diagsvc = 4`).
- **Live System Optimization Detection on Launch**:
  - Automatically queries registry, services, tasks, and AppX on application warmup.
  - Tweak switches visually reflect true live Windows optimization state (active/ON if already applied on the OS, whether by WinBlast0 or third-party/OS).
- **Dual Engine Execution Modes (Instant vs Normal)**:
  - Top header toggle allows one-click switching between **Instant Mode** and **Normal Mode**.
  - *Instant Mode*: Switch clicks execute immediately in the background with real-time UI updates and sub-millisecond targeted scanning (`scan_single_tweak`).
  - *Normal Mode*: Switch clicks stage transitions (`StageApply`, `StageRevert`) for atomic batch commit upon clicking "Apply Selected".
- **Bidirectional Mutation & Revert Engine**:
  - Implemented `revert_tweak` and `revert_tweaks_by_id` in `src/executor/engine.rs`.
  - Enables restoring any tweak back to stock Windows defaults even if applied out-of-program.
- **Performance Baseline Reset API**:
  - Added "🔄 Reset Baseline" button in the Before/After impact card.
  - Backed by `StateLedger::reset_baseline` to clear previous baselines and establish a new baseline from current metrics.
- **Real-Time 1-Second Task Manager Monitoring**:
  - Continuous non-blocking background worker sampling CPU %, per-core CPU %, RAM (used/total), process count, thread count, and NT kernel timer resolution.
  - Dual live waveform sparklines (`sparkline.rs`) rendered with GPU primitives on Easy Dashboard and Hardware views.
  - Live hardware telemetry ticker in the top navigation bar.
- **UI & Motion Polish**:
  - Full GPU spring animations (`rune::tok::motion::SNAPPY`) on switches, buttons, and cards.
  - Active accent border glows and subtle card highlights on applied tweaks.
  - Interactive hover and tactile press reactions across all navigation buttons, tabs, and tweak rows.

### 🐛 Bug Fixes & Stability:
- Resolved single-slash registry rollback slice index panic.
- Configured SQLite 5-second busy timeout and enabled WAL journal mode.
- Fixed integer overflow hazards on impact delta formatting with `.saturating_abs()`.
- Fixed hosts file trailing newline check to prevent line concatenation bugs.
- Corrected hosts parser to capture multi-host domain lines.
- Prevented absent third-party telemetry services from being falsely counted as disabled.
- Added validation for empty GPU PnP instance IDs and AppX family names.
- Fixed sparkline buffer bounding logic (`while len() >= 35`).
- Cleaned all Clippy and compiler warnings to zero.

---

## ⚡ Version 0.1.0 (Initial Pure-Rust Rune Engine Port) — 2026-11-08

### ✨ Features:
- Ported entire WinBlast0 application to pure-Rust Rune framework (`AppSpec`, `AppState`, `El`).
- Zero WebView2, zero Chromium, and zero browser engine overhead.
- Dual-Layer rollback architecture: Windows System Restore Points (VSS) + SQLite Transactional Ledger.
- Initial 55 tweaks across 7 TOML catalogs.
- State-of-the-art multi-engine detection for Registry (`KEY_WOW64_64KEY`), Scheduled Tasks (multilingual CSV parsing), and AppX (4 registry stores).
- Embedded rule catalogs into `.rdata` with zero-fail multi-path fallback (`load_rules_with_fallback`).
- Native NT timer resolution query (`NtQueryTimerResolution` from `ntdll.dll`).
- PCIe GPU MSI Mode and interrupt priority tuning.

---

## 🔗 Related Notes
- [[Roadmap & Future Vision]]
- [[Test Suite & Validation Matrix]]
- [[00 - Index (MOC)]]
