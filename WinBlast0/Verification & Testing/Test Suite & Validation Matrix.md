---
title: Test Suite & Validation Matrix
tags: [testing, verification, cargo, clippy, matrix, e2e, rune, win32]
created: 2026-11-08
updated: 2026-11-10
---

# 🧪 Test Suite & Validation Matrix (v0.8.0)

WinBlast0 **v0.8.0** verifies every subsystem via **52** unit tests, **3** real Windows 11 Build `26300` & Win32 Registry/NTFS junction isolation tests, **18** sandbox & fault-injection E2E tests across all **168 tweaks**, and **4** headless Rune UI multi-resolution geometry & interaction suites.

---

## 🏃 Running the Complete Verification Suite
```powershell
cargo clippy --all-targets -- -D warnings
cargo test --no-fail-fast
cargo build --release
```

### Summary of Observed Results (v0.8.0)
| Test Binary / Check | Count | Status | Scope Covered |
| :--- | :---: | :---: | :--- |
| **`src/lib.rs` (Unit Tests)** | 52 | **PASS** | Risk evaluation, `os_detect` Win11 Build 26300 host check, `is_known_numeric_reg_sz` (29 keys), 8-language `320`-key i18n & Turkish `fold_search_str`, strict `REG_*` dispatch, SQLite WAL & memory CRUD, NT timers, DPC probe, `EfficiencyClass` P/E topology, `detect_vendor` token guard, `PowerCfg` GUIDs, AppX `split_appx_name_and_publisher`, `task_scanner` `CP_OEMCP` & COM details, hosts BOM/IPv6/footer parser, `StartupApproved` 12-byte encoding, cleaner reparse safety, `autounattend.xml` escaping |
| **`tests/real_windows_env.rs`** | 3 | **PASS** | Read-only live Windows 11 Build `26300` (`26H2`) host audit (`RealBackend`, `SetupDiGetDeviceInstanceIdW` RTX 5070 PCI PnP ID, virtual display isolation, Win32 SCM services, 168-tweak scan), isolated Win32 64-bit `HKCU` multi-type registry & recursive tree snapshot/rollback, isolated NTFS directory junction & `FILE_ATTRIBUTE_READONLY` temp cleanup |
| **`tests/tweak_sandbox_e2e.rs`** | 18 | **PASS** | All **168 tweaks** full sandbox lifecycle, multi-axis conflict audit, `FailAtMutationBackend` atomic rollback, `PanickingBackend` RAII worker-panic recovery, `SlowScanBackend` epoch & `mutated_during_scan` fencing, `Normal <-> Instant` / `Easy <-> Advanced` / `DiffModal` separation, AppX underscore/ResourceId matching, `autounattend.xml` Setup-safe `reg delete` & `REG_SZ` permutations |
| **`tests/ui_responsive_e2e.rs`** | 4 | **PASS** | Headless `rune::Runtime` sweep across **10 viewports** (`420x400` .. `3840x2160`) × **10 views** + **4 sub-tabs** × Dark/Light × EN/TR/DE/RU × `1.0x`/`1.25x`/`1.5x` DPI × open Modals/Drawers + `virtual_rows` height/clip bounds; live `420x400` drawer & switch clicks; `NotApplicable` locked toggles, `[Revert]` `DiffModal`, `RunAuditBtn` persistence, `CoreParking`/`HardwareView` >8-core & `latency` readout, `ProtectedCore` `🔒` badge; `--tree` zero key warnings |
| **`cargo clippy --all-targets -- -D warnings`** | — | **0 Warnings** | Clean under strict `-D warnings` across lib, bin, and integration tests |
| **`cargo build --release`** | — | **PASS** | Standalone executable `dist\winblast0.exe` (`28,108,288` bytes) compiled cleanly with embedded `.rdata` catalogs |

---

## 🔗 Related Notes
- [[SystemBackend & Sandbox Test Environment]]
- [[Responsive UI, Animations & Changelog Ledger]]
- [[Debugging & Edge Cases Resolved]]
- [[Changelog v0.7.0 to v0.8.0]]
- [[Changelog v0.6.0 to v0.7.0]]
- [[00 - Index (MOC)]]
