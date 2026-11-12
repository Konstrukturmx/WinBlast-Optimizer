---
title: Changelog v0.5.0 to v0.6.0
tags: [changelog, release, v0.6.0, testing, responsive, sandbox]
created: 2026-11-10
version: v0.6.0
---

# Changelog: v0.5.0 ➔ v0.6.0

## 🌟 Highlights
WinBlast0 **v0.6.0** marks a major leap in architectural verification, multi-resolution ergonomics, and Windows subsystem reliability:
- **Complete Isolated Test Sandbox (`SystemBackend` + `SandboxEnv`)**: Verified all **99 tweaks** end-to-end (Seed -> Scan -> Apply -> Scan -> Rollback -> Revert) with zero host OS interference.
- **Responsive Across All Resolutions (`420x400` to `3840x2160`)**: Pure flex-wrapping grids, collapsible hamburger nav drawer (`< 1020pt`), zero-echo root breakpoint tiering, and verified via automated headless Rune geometry sweeps across 10 resolutions, Dark/Light themes, 3 verbose locales, and multiple DPI scales.
- **Dedicated Dual-Agent QA Loop**: Verified cleanly by both a newly created **`winblast-tester`** agent and **`winblast-debugger-fixer`** (*Rust Systems Debugger & Fixer*).

---

## 🛠️ Deep System & Execution Bug Fixes
- **AppX Uninstaller & Detector**: Fixed `Get-AppxPackage -Name` bug in `ps_runner.rs` & `appx_scanner.rs` so `PackageFamilyName` removes and verifies packages accurately across `-AllUsers` + provisioned stores.
- **SCM Synchronization (`svc_mutator.rs`)**: Paired registry `Start` writes with `sc config <service> start= ...` so SCM stays synchronized across disable/revert cycles.
- **Instant Mode Error Unlocking (`app.rs`)**: Added `WorkerEvent::SingleTweakFailed` so transient registry/IO errors release `in_flight_tweaks` cleanly instead of locking UI toggles.
- **Absent Pre-State Rollback (`__ABSENT__`)**: Rolling back tweaks whose keys did not exist before mutation now cleanly deletes the created key/value rather than leaving synthetic defaults behind.
- **Multilingual Task Scheduler OEM Decoding**: Added Win32 `CP_OEMCP` decoding + Turkish/CJK/European localized `Disabled` matching in `task_scanner.rs`, eliminating ambiguous prefix cross-matching.
- **Optimus GPU Subkey Routing & `REG_SZ` Preservation**: Dynamic display class resolution in `gpu.rs` + automatic `REG_SZ` handling for numeric string OS settings (`MenuShowDelay`, `MinAnimate`, `AutoEndTasks`, `MouseSpeed`).
- **Profile Import & Select All**: Fixed `import_profile()` synchronizing `staged_transitions`, excluded `NotApplicable` tweaks from `select_all()`, and logged HAGS/MPO/MSI/Parking/Privacy mutations into SQLite `StateLedger`.
- **TOML Catalog Reconciliation**: Fixed `enable_game_mode` (`default = 0`), removed conflicting `NtfsDisableLastAccessUpdate`, deduplicated overlapping telemetry/WER/Cortana rules, replaced `UserPreferencesMask` (`REG_BINARY`) with `REG_DWORD` Explorer shadow settings, and updated PowerSchemes `AcSettingIndex`.

---

## 🎨 UI, Responsiveness & Localization Fixes
- **100% Exhaustive i18n (`185` TextKeys × `8` Languages)**: Localized `hardware_view`, `privacy_shield`, `backup_center`, `cpu_parking`, `latency_bench`, `diff_modal`, and changelog badges cleanly.
- **Light Mode WCAG Contrast Fixes**: Fixed white-on-white button hover wash and invisible OFF toggle knobs in Light Mode (`widgets/button.rs` + `theme.rs`).
- **Live Advanced Matrix Filters**: Activated `text_input` search bar (`SearchPlaceholder`) + category filter chips (`CategoryAll`).
- **Zero Overflow & Zero Collapsed Scrolls**: Fixed nested `row().flex_wrap()` `flex_shrink: 0.0` overflow across `top_bar` and view toolbars, `.h_full().max_h(420.0)` bounded Diff Modal overlay, and `.wrap_anywhere()` on long AppX/registry status details.
