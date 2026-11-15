# WinBlast Optimizer v1.2.0 Release Notes

**Release Date:** November 15, 2026  
**Architecture:** Pure Rust 2021 Edition • Win32 FFI Core • GPU-Accelerated Rune UI  
**Target OS:** Windows 10 & Windows 11 (x64)

---

## 🌟 Highlights & New Features in v1.2.0

### 1. Zero-Allocation View-Level Internationalization (8 Languages)
- Implemented `src/core/view_i18n.rs` with 88 zero-allocation string catalog keys providing compile-time `&'static str` translations across all 8 supported languages:
  - English (`EN`), Turkish (`TR`), German (`DE`), French (`FR`), Spanish (`ES`), Russian (`RU`), Chinese (`ZH`), Japanese (`JA`).
- Complete UI translation coverage wired dynamically to `AppState::tr_v(...)` across all 11 extended subviews:
  - **Support & About**: Title, subtitle, repository labels, MIT license disclaimer, release highlights, and UI theme studio.
  - **Fault Diagnostics**: Title, alert badges, system stability status, rescan/clear buttons, pending reboot CBS/Windows Update locks.
  - **Performance Benchmark**: Title, start benchmark buttons, composite score metrics, compute throughput labels.
  - **Windows Component Repair**: Title, repair task cards, execution statuses.
  - **Game Profiles**: Title, title cards, apply/revert buttons, IFEO priority indicators.
  - **Thermal & Fans**: Title, OEM sensor requirements notice, junction temperatures.
  - **Storage Drives**: Title, physical drive models, one-click TRIM button, health statuses.
  - **Esports Game Suitability**: Title, hardware suitability evaluation, readiness badges.
  - **In-Game Overlay & Latency Mode**: Title, Game DVR / Game Mode toggles and descriptions.
  - **Gaming Telemetry**: Title, active game process detection, frametime pacing metrics.
  - **Network Diagnostics**: Title, flush DNS button, ping probe latency headers.

### 2. Elimination of Placebo Telemetry & Authentic Windows Interrogation
- **Physical Drive Interrogation (`disks.rs`)**: Replaced placeholder disk names with authentic Win32 `IOCTL_STORAGE_QUERY_PROPERTY` queries returning actual physical drive models, bus types (NVMe/SATA/USB), seek penalty descriptors, and volume metrics via `GetVolumeInformationW`.
- **ACPI ThermalZone Telemetry (`thermal.rs`)**: Replaced mock temperatures with direct ACPI registry queries, introducing an honest OEM sensor disclaimer when ring-0 drivers are unavailable.
- **Windows IFEO Registry Mutations (`game_profiles.rs`)**: Replaced in-memory flags with real Windows Image File Execution Options (IFEO) registry keys (`CpuPriorityClass = 3` High, `IoPriority = 2` High IO).
- **Windows Component Repair (`windows_repair.rs`)**: Wired one-click repair actions to authentic system commands: `sfc.exe /scannow`, `dism.exe /Online /Cleanup-Image /RestoreHealth`, `ipconfig.exe /flushdns`, `netsh.exe winsock reset`, and safe Explorer restart.
- **Online Driver Scanning (`driver_update.rs`)**: Implemented online driver version checking with direct browser deep-linking to official manufacturer portals (NVIDIA, AMD, Intel).

### 3. UI Thread Fluidity & Diagnostic Accuracy
- **Asynchronous Compute Benchmark**: Offloaded multi-threaded performance testing to background worker threads with Rune waker notification, completely eliminating UI thread freezes.
- **Fault Diagnostics Badge Accuracy**: Removed false-alarm `.max(1)` clamp, ensuring healthy systems display green `✔ 0 Faults` instead of a permanent red alert.
- **Interactive Elevation Badge**: One-click privilege elevation via `ShellExecuteW("runas", ...)` from the top bar.

---

## 🛡️ Digital Signature & Verification
- **Binary:** `winblast0.exe` (v1.2.0)
- **Signature:** Microsoft Authenticode signed with SHA-256 digest
- **Timestamp:** RFC 3161 compliant timestamping by DigiCert TimeStamping Responder
- **Verification:**
  ```powershell
  signtool verify /pa /v target\release\winblast0.exe
  Get-FileHash target\release\winblast0.exe -Algorithm SHA256
  ```

---

## 🧪 Test Coverage & Health
- **127 / 127 Unit & Integration Tests Passing (100% Pass Rate)**
- **0 Clippy Warnings (`cargo clippy --all-targets -- -D warnings`)**
- **0 Compiler Errors (`cargo check --all-targets`)**
