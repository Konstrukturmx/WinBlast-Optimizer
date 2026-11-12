---
title: Changelog v0.6.0 to v0.7.0
tags: [changelog, release, v0.7.0, winhance, optimizerduck, localization, responsive, sandbox]
created: 2026-11-10
version: v0.7.0
---

# Changelog: v0.6.0 ➔ v0.7.0

## 🌟 Highlights
WinBlast0 **v0.7.0** integrates deduplicated optimizations and system utilities ported from **`memstechtips/Winhance`** and **`itsfatduck/optimizerDuck`**, expands the UI from 8 to **10 responsive views**, brings full **313-key × 8-language localization**, and hardens every subsystem through a 3-wave QA + debugging pass by `winblast-tester` and `winblast-debugger-fixer`:

- **Expanded 168-Tweak Embedded Catalog Across 10 TOML Files**: Added 3 new embedded catalogs (`ai_privacy_ext.toml`, `perf_power_ext.toml`, `ux_customization.toml`) and extended `hardware_tuning.toml` & `bloatware_appx.toml` with zero duplicate IDs or conflicting targets.
- **New Compound Engine & Scanner Action Primitives**: Added `RegistryDelete`, `RegistryDeleteTree`, `RegistryBinaryBit`, `CompositeStringKey`, `RegistryDynamicDword`, `PowerCfgSetting`, and `WindowsFeature` (`OptionalFeature` / DISM `Capability`), plus `Reversibility::RequiresLogoff`.
- **Clean & Apps (`CleanAndApps`) View**: Native Rust disk cleanup across 8 targets (`UserTemp`, `WindowsTemp`, `WindowsUpdateCache`, `DeliveryOptimizationCache`, `ThumbnailCache`, `WerAndCrashDumps`, `Prefetch`, `RecycleBin`), Safe/AI/Caution/ProtectedCore AppX removal with batch PowerShell pipeline, OneDrive safe removal gated on KFM detection, Edge removal preserving WebView2, DISM Windows Features/Capabilities toggling, and searchable WinGet installer (`CURATED_WINGET_APPS`).
- **Startup, Tasks & Live Services (`StartupAndTasks`) View**: Virtualized management (`El::virtual_rows`) across Registry `Run`, Startup folders, Scheduled Tasks (`HideMicrosoftTasks`), and Win32 Services (`OpenSCManagerW` + `EnumServicesStatusExW`), strictly protecting `CRITICAL_CORE_SERVICES` and GPO-locked startup entries (`policy_locked`).
- **Platform Hardware, Power & `autounattend.xml` Generator**: SMBIOS RAM DIMMs, storage drive health/space volumes, Secure Boot / VBS / HVCI status, native `powrprof` active scheme / WinBlast0 Ultimate power plan / hibernation / USB Hub power controls, non-elevated `CreateProcessWithTokenW` Explorer restart, and Windows 10/11 setup-safe `autounattend.xml` generator (`Amd64`/`Arm64`, OOBE/TPM/MSA bypasses, presets).

---

## 🛠️ Bugs Resolved Across Waves 1–3 (`winblast-tester` & `winblast-debugger-fixer`)
- **Catalog Loading (`src/core/risk.rs`)**: Added missing `Reversibility::RequiresLogoff` variant so `perf_power_ext.toml` (20 tweaks) and `ux_customization.toml` (13 tweaks) load reliably (bringing loaded count from 135 to all **168 tweaks**). Fixed MMCSS `Pro Audio\Priority` (`6`), `KeyboardDataQueueSize` (`256`), `power_settings_pcie_usb_nvme_epb` risk/unattend flags, `disable_generative_ai_app_access` (`min_build = 26100`), and DISM capability version suffixes (`~~~~`).
- **Ledger Rollback & Concurrency (`src/app.rs`, `src/executor/engine.rs`, `src/backup/state_ledger.rs`)**: Fixed `GpuMsi` (`bool|priority`), `WindowsFeature` (`kind|escaped_name`), `ScheduledTask` (real `prev_enabled`), `HostsBlock` (`unblock_telemetry_hosts_with`), `PowerCfg` (`ActiveScheme`, `UltimateScheme`, `Hibernation`, `UsbPowerSaving`), `StartupApproved` (`set_startup_entry_enabled_by_id`), sibling-preserving `RegistryBinaryBit` rollback, and non-reversible `LedgerEntry::is_reversible` handling for `CleanTargets`/`AppXBatch`/`App`. Split Clean & Apps in-flight flags so Disk Targets and AppX/Features scan concurrently on navigation, moved GPU refresh off the UI thread, and filtered unknown IDs on `import_profile`.
- **DISM, AppX, WinGet, SCM, Explorer & Cleaner (`src/tools/*.rs`, `src/scanner/mod.rs`, `src/core/backend.rs`)**: Fixed `RegistryDynamicDword` `u32` overflow on >=48 GB RAM hosts via `u64` math, DISM `/English` + exit code `3010` + `~~~~` prefix matching (and `SandboxEnv` parity), `validate_winget_package_id` allowing `+` (`Notepad++.Notepad++`), Win32 SCM enumeration replacing localized `sc query`, `explorer.exe` token duplication prior to single-PID exit/wait/relaunch, `cleaner.rs` reparse-point ordering & error reporting, and `autounattend.xml` escaping + `BypassNRO` routing.
- **UI, Switch Routing, Contrast & Responsiveness (`src/ui/**`, `src/core/i18n.rs`)**: Replaced pollutive `switch_toggle` clicks inside Clean & Apps and Startup Apps with `switch_indicator` and `switch_toggle_with`, fixed Changelog Drawer showing 12 oldest instead of 12 newest entries, prevented `nav_sidebar_scroll` collapse via `main_body.min_h(120.0)` and compact drawer nav placement, fixed Compact `AppX`/`Startup`/`Services` scroll containers & `virtual_rows` bounds (`140.0` services / `116.0` startup/tasks / `104.0` AppX), expanded `TextKey` to **313 keys × 8 languages**, added `I18n::fold_search_str`, and fixed Dark/Light WCAG AA contrast.

---

## 🔗 Related Notes
- [[00 - Index (MOC)]]
- [[Catalog Overview & Risk Model]]
- [[Responsive UI, Animations & Changelog Ledger]]
- [[Comprehensive i18n & Localization Architecture]]
- [[Debugging & Edge Cases Resolved]]
- [[Test Suite & Validation Matrix]]
