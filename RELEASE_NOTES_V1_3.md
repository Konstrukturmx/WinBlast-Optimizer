# WinBlast Optimizer v1.3.0 Release Notes

**Release Date:** November 17, 2026  
**Architecture:** Pure Rust 2021 Edition • Win32 FFI Core • GPU-Accelerated Rune UI  
**Target OS:** Windows 10 & Windows 11 (x64)

---

## 🌟 Highlights & New Features in v1.3.0

### 1. Rayon Multi-Threaded Parallel Evaluation (`src/scanner/mod.rs`)
- Implemented `scan_all_tweaks_parallel` utilizing Rayon work-stealing parallel iterators and chunked batching.
- Reduced complete 208-tweak scan execution latency from sequential execution down to sub-180 ms on modern multi-core processors.
- Guaranteed 100% state parity with sequential evaluation across all scan detail kinds and categories.

### 2. Unified Strongly-Typed `WinBlastError` Hierarchy (`src/core/error.rs`)
- Replaced scattered stringly-typed errors across registry, Windows service, COM, and NT kernel operations with a structured `WinBlastError` enum.
- Variants include `Registry`, `Service`, `Com`, `NtStatus`, `Io`, and `InvalidInput` with full `thiserror` derivations.
- Preserves raw Win32 error codes, `HRESULT`, and `NTSTATUS` codes with root-cause source chaining for diagnostic precision.

### 3. Automated Pre-Flight VSS System Restore Points (`src/backup/restore_point.rs`)
- Integrated `ensure_preflight_restore_point` into batch tweak application.
- Automatically audits the creation timestamp of existing restore points using WMI/VSS; triggers a native snapshot only if no snapshot was taken within the last 24 hours to prevent redundant snapshot exhaustion.

### 4. Floating Dropdown Popovers with Native Anchoring (`src/ui/mod.rs`)
- Restructured top bar dropdowns using Rune's native `anchored_fit` mechanism directly attached to trigger button keys (`topbar_lang_btn`, `topbar_theme_mode_btn`, `theme_style_selector`, `topbar_color_scheme_btn`).
- Completely resolved the previous bug where dropdowns opened at fixed top-left coordinates (`x = 50px`) over the navigation sidebar.
- Added native outside-click and Escape key dismissal via `.modal(true, ...)` on active dropdown popovers.

### 5. Decoupled Theme Selector Cycling
- Clicking the `Theme Style` button now strictly toggles the dropdown popover menu instead of immediately cycling the active theme.
- Theme style changes only occur upon selecting an option inside the dropdown menu (`Glassy`, `Minimalistic`, `Classic`, `Compact`, `Materialistic`).

### 6. Overlap-Free Responsive Driver & BIOS Layout (`src/ui/views/driver_bios.rs`)
- Enforced a definite bounded container height `.h(list_height)` and soft clipping (`clip_soft(0.0)`) on the driver updates scroll container.
- Completely eliminated the layout bug where driver update cards visually bled past their parent card and overlapped Motherboard and BIOS firmware cards.

### 7. Rigid Single-Row 44px Top Header Bar
- Removed `.flex_wrap()` from the top bar and controls group, enforcing a locked 44px height.
- Streamlined telemetry items into a unified compact pill on wide screens (`CPU: 22% • RAM: 63%`), preventing the header from blowing out its vertical height on constrained resolutions.

---

## 🧪 Verification Matrix
- **Test Suite:** 163 / 163 automated unit and integration tests passing (100% pass rate).
- **Linter & Code Quality:** Zero warnings (`cargo clippy --all-targets -- -D warnings`).
- **Knowledge Graph:** 3,402 nodes, 7,159 edges, 165 communities synced via `/graphify`.
- **Binary:** `winblast0.exe` (v1.3.0) with embedded `winblast.ico` and PE version metadata `1.3.0.0`.
- **File Size:** `31,593,504` bytes (30.1 MB)
- **SHA-256 Checksum:** `806fd072dfa29214edc0cd5e30fc65b162bb16b15bd88b36a061207dcd0d51f6`
- **Signature:** Microsoft Authenticode signed by `Konstrukturmx` with DigiCert RFC 3161 SHA-256 timestamping.
