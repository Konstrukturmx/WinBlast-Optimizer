# WinBlast Optimizer v1.5.0 Release Notes

**Release Date:** November 16, 2026  
**Architecture:** Pure Rust 2021 Edition • Win32 FFI Core • GPU-Accelerated Rune UI  
**Target OS:** Windows 10 & Windows 11 (x64)

---

## 🌟 Highlights & New Features in v1.5.0

### 1. In-App GitHub Releases Self-Updater (`src/core/updater.rs`)
- **Direct GitHub Releases API Integration**: Lightweight, native HTTPS interrogator querying the official repository (`/repos/Konstrukturmx/WinBlast-Optimizer/releases/latest`) for new tagged versions.
- **Semver-Aware Comparison**: Automatically parses release tags, compares against the running binary version (`v1.5.0`), and calculates download payloads with live asset size telemetry.
- **Atomic In-Place Replacement**: Replaces the executing `winblast0.exe` without requiring external installer scripts or third-party updaters. Renames running process binary to `.old` and moves new release executable into place before triggering instant restart.
- **Dedicated Dashboard Status Card**: Live update checker card on Easy Dashboard with 1-click "Check for Updates" and "Download & Install" triggers.

### 2. Administrator Elevation Guard & 1-Click Relaunch (`src/core/elevation.rs`)
- **Pre-Flight Access Token Inspection**: Direct Win32 `OpenProcessToken` and `GetTokenInformation(TokenElevation)` interrogation upon application boot.
- **Context-Aware Dashboard Warning**: Unobtrusive warning banner dynamically rendered on the Easy Dashboard when running under standard user rights, explaining that registry tweaks, driver updates, and Windows services require elevation.
- **1-Click UAC Elevation**: Dedicated "🛡️ Relaunch as Admin" button triggering `ShellExecuteW(..., "runas", ...)` to elevate with user-friendly Windows UAC consent.

### 3. Game Prerequisites & Runtimes Cascade Installer (`src/tools/game_prereqs.rs`)
- **Comprehensive Runtime Auditing**: Detects, verifies, and installs missing game engines and framework runtimes:
  - DirectX End-User Runtimes (June 2010 Redistributable)
  - Visual C++ 2015–2022 Redistributable (x86 & x64)
  - Visual C++ All-in-One Package (2005–2022 Legacy & Modern Suite)
  - Vulkan Runtime Libraries (`vulkan-1.dll` loader)
  - Microsoft .NET Desktop Runtime 8.0 (LTS) & 9.0 (STS)
  - Microsoft .NET Framework 4.8.1
  - OpenAL (Open Audio Library 3D audio API)
  - Microsoft XNA Framework 4.0 Refresh
- **Resilient 3-Tier Cascade Fallback Engine**:
  1. *Tier 1 (WinGet)*: High-speed silent package manager execution (`winget install --id ... --silent`).
  2. *Tier 2 (Chocolatey)*: CLI package manager fallback (`choco install ... -y`).
  3. *Tier 3 (Vendor Portal)*: Direct fallback opening official vendor download websites.
- **1-Click "Install All Missing"**: Automated sequential queue executing unattended background installations with real-time UI state synchronization.

### 4. Unified Gaming & Performance Hub (`src/ui/views/gaming_stats.rs`)
- **Unified Workspace Consolidation**: Consolidated previously fragmented gaming views into a single, high-density 5-tab Gaming Hub:
  - **📊 Live Monitor**: Real-time frametime pacing, jitter calculation, and background game thread suppression.
  - **📈 FPS Benchmark**: Pre-configured benchmark tests across top competitive esports titles (*Counter-Strike 2*, *Valorant*, *Apex Legends*, *Warzone*, *Cyberpunk 2077*).
  - **🎮 Game Profiles**: Authentic Windows Image File Execution Options (IFEO) registry tuning (`CpuPriorityClass` and `IoPriority`).
  - **🖥️ In-Game HUD**: Direct toggles for Game DVR, Windows Game Mode, and hardware GPU scheduling.
  - **🎯 Game Suitability**: Hardware capability and bottleneck analysis for competitive 144Hz/240Hz play.

### 5. Multi-Axis Sorting Engine Across All Data Views
- **1-Click Multi-Axis Sorting**: Added interactive sorting controls and stateful sort modes across:
  - **Advanced Matrix**: Name (Asc/Desc), Category, Risk Level (`Safe` to `High`), Applied Status.
  - **Disk & AppX Cleanup**: Reclaimable Disk Space, Package Name.
  - **Startup & Tasks**: Startup Source (`Run`, `RunOnce`, `StartupApproved`), Status, Vendor.
  - **Driver Lifecycle**: Device Class, Vendor, Update Availability, WHQL digital certification.
  - **Game Prerequisites**: Missing First, Alphabetical.
  - **Privacy Shield**: Protection Category, Risk Severity.

### 6. Responsive Scrollbar Overhaul & Layout Collision Elimination
- **Dedicated Scrollbar Gutters (`.pr(14.0)` to `.pr(16.0)`)**: Resolved critical UI bug where scrollbar thumbs collided with or hovered directly over right-aligned action buttons and toggle switches.
- **Two-Tier WinGet Cards in Clean & Apps**: Redesigned WinGet application cards into a structured two-row hierarchy (top row: switch, name, category badge; bottom row: full description), completely eliminating text truncation and badge collision.
- **Sub-Navigation Refinements**: Cleaned up bottom and lateral navigation buttons with clean Unicode chevrons (`◀ Prev` / `Next ▶`).

### 7. Expanded 208 Tweak Declarative Catalog & 510-Key i18n
- **40 New Declarative Optimizations**: Added `assets/rules/gaming_latency_ultra.toml` and `assets/rules/system_deep_opt.toml`, expanding verified tweaks from 168 to 208 without registry overlap or service breakage.
- **Zero-Allocation 8-Language Localization (510 Keys)**: Expanded `TextKey` catalog from 420 to 510 keys, providing full translations across English (`EN`), Turkish (`TR`), German (`DE`), French (`FR`), Spanish (`ES`), Russian (`RU`), Chinese (`ZH`), and Japanese (`JA`).

---

## 🛡️ Digital Signature & Verification
- **Binary:** `winblast0.exe` (v1.5.0)
- **Signature:** Microsoft Authenticode signed with SHA-256 digest
- **File Size:** `31,533,608` bytes (30.0 MB)
- **SHA-256 Checksum:** `b26c4b9d1757d001544ec12f868b6ee04fa14868aca14f0527ec6ba654eb819d`
- **Timestamp:** RFC 3161 compliant timestamping by DigiCert TimeStamping Responder
- **Verification:**
  ```powershell
  signtool verify /pa /v target\release\winblast0.exe
  Get-FileHash target\release\winblast0.exe -Algorithm SHA256
  ```

---

## 🧪 Comprehensive Quality Assurance & Verification
- **154 / 154 Automated Tests Passing (100% Pass Rate)**:
  - `cargo test --lib`: 109 unit tests passing
  - `tweak_sandbox_e2e`: 24 mock registry and rollback tests passing
  - `ui_responsive_e2e`: 13 responsive tests with full 2,160-frame viewport sweep
  - `ui_scrollbar_test`: 5 scrollbar interaction and collision tests passing
  - `real_windows_env`: 3 live host Win32 integration tests passing
- **0 Clippy Warnings (`cargo clippy --all-targets -- -D warnings`)**
- **0 Compiler Errors (`cargo check --all-targets`)**
