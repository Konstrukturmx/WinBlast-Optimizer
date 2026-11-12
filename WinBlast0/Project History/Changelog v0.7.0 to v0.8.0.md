---
title: Changelog v0.7.0 to v0.8.0
tags: [changelog, release, v0.8.0, debugging, logic-fixes, ffi, concurrency, testing]
created: 2026-11-10
version: v0.8.0
---

# Changelog: v0.7.0 ➔ v0.8.0

## 🌟 Highlights
WinBlast0 **v0.8.0** is a dedicated deep-debugging, logic-hardening, and inductive/deductive verification release engineered using the **Elephant-Goldfish Model (EGM)**, live **Windows 11 Build 26300 (`26H2`)** read-only probes, isolated **Win32 64-bit Registry & NTFS Junction sandboxes**, and **`winblast-debugger-fixer`** + **`winblast-tester`**.

---

## 🛠️ Deep Bug & Logic Fixes Across All Subsystems

### 1. Catalog Truthfulness, OS Detection & Registry Semantics
- **OS Detection (`src/core/os_detect.rs`)**: Added `KEY_WOW64_64KEY` to `CurrentVersion` lookup and `ReleaseId` fallback for baseline Windows 10 Build `19041` (`2004`).
- **Registry Helpers & Type Safety (`src/core/reg_helpers.rs`, `src/executor/reg_mutator.rs`, `src/scanner/registry_scanner.rs`)**:
  - Normalized `"@"` alongside `"(Default)"` / `""` in `normalize_reg_value_name`.
  - Expanded shared `is_known_numeric_reg_sz` across 29 Build `26300` desktop/mouse/keyboard/accessibility `REG_SZ` keys (`BorderWidth`, `MouseThreshold1/2`, `MouseHoverTime`, `KeyboardSpeed`, `LowLevelHooksTimeout`, etc.), handling negative `i32` decimal strings and preventing `KEY_WOW64_64KEY` drop on absent keys.
  - Strictly dispatched `read_dword_from_key` by `REG_*` value type (`REG_DWORD`, `REG_DWORD_BIG_ENDIAN`, `REG_QWORD <= u32::MAX`, parsed numeric `REG_SZ`/`REG_EXPAND_SZ`) eliminating UTF-16LE byte truncation bugs, and enforced `REG_BINARY` check in `get_binary`.
  - Blocked shallow critical subtrees (`SOFTWARE\Microsoft\Windows\CurrentVersion`, `SYSTEM\CurrentControlSet\Services`, etc.) and whitespace/slash-only paths in `validate_registry_mutation_path`.
  - Added `#` percent-encoding (`%23`) in `escape_pipe`/`unescape_pipe`, deduplication in `merge_composite_pair`, and dynamic active NIC `<NET_INTERFACE_GUID>` resolution (`resolve_net_interface_paths`).
- **TOML Catalog Ground Truth (`assets/rules/*.toml`, `168` tweaks)**:
  - Fixed Paint AI policy path (`SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint`), Copilot hardware key (`HKCU\Software\Policies\Microsoft\Windows\CopilotKey`), Recall 24H2 nested tasks (`\Recall\PolicyConfiguration`, `\Recall\InitialConfiguration`), Wifi Sense writable policy (`WcmSvc`), `Tcp1323Opts = 3`, Razer Synapse `WOW6432Node`, MMCSS `Pro Audio\Priority` (`1 -> 6`), NTFS `NtfsDisableLastAccessUpdate` (`2147483650` default) & `NtfsDisable8dot3NameCreation` (`2` default), `ServicesPipeTimeout` (`30000 -> 15000`), NVMe idle defaults (`200` AC / `100` DC), `NoDriveTypeAutoRun` (`145` default), `UserDuckingPreference` (`1 -> 3`), PowerMizer (`PerfLevelSrc = 0x2222`), and Windows 10/11 build & Home edition compatibility guards (`disable_mpo` `max_build = 25999`, `enable_directx_windowed_game_optimizations` & Gallery & `TaskbarEndTask` `min_build = 22621`, `consumer_features_disable` Pro+).

### 2. Service Control Manager, Tasks, Scanner, Executor & Rollback
- **SCM Sync (`src/executor/svc_mutator.rs`, `src/scanner/service_scanner.rs`)**: Fixed broken single-arg `"start= auto"` passed to `sc config` into separate args (`["config", svc, "start=", mode]`), preserved & restored `DelayedAutostart` (`start= delayed-auto`), surfaced `sc` errors cleanly, and handled absent optional 3rd-party services gracefully for targets `3`/`4`.
- **Task Scheduler & Hosts (`src/scanner/task_scanner.rs`, `src/scanner/telemetry_scanner.rs`, `src/executor/ps_runner.rs`)**: Decoded `CP_OEMCP` before UTF-8 fallback, added COM `ITaskService` detail + `AccessDenied` discrimination, resolved Win11 24H2 `" Exp"` suffix tasks in both scan and toggle, added Spanish `"deshabilitado"`, stripped hosts `\u{FEFF}` BOM, supported `::1`/`::` IPv6 + tabs, and matched exact header/footer lines cleanly.
- **Scanner & Executor (`src/scanner/mod.rs`, `src/executor/engine.rs`, `src/backup/state_ledger.rs`, `src/backup/restore_point.rs`, `src/backup/profile_io.rs`)**:
  - Enforced `is_gpu_vendor_compatible` in `evaluate_tweak`, `apply_tweak_with`, `revert_tweak_with`, `select_all`, and `apply_preset` (preventing `\0000` cross-adapter fallback mutations).
  - Implemented reverse atomic rollback inside `revert_tweak_with`, marked consumed `RegistryDelete`/`RegistryDeleteTree` priors as `rolled_back = 1`, seeded `default_full_hex` on absent `RegistryBinaryBit` revert, tolerated already-deleted `__ABSENT__` on out-of-order `RegistryBinaryBit` rollback, preserved `gpu_vendor` + empty string distinctions on `CompositeStringKey`, marked `"AppXPackage"` & `"Startup"` irreversible in `LedgerEntry::is_reversible`, added `-ErrorAction Stop` to `Checkpoint-Computer`, and added `create_dir_all` in `profile_io`.
- **AppX Matcher Deduplication (`src/scanner/appx_scanner.rs`, `src/core/backend.rs`)**: Extracted shared `split_appx_name_and_publisher` & `appx_package_matches_family` matching trailing 13-char `PublisherId` via `rsplit_once('_')` cleanly for AppX names containing underscores.

### 3. Concurrency, Backend Fidelity & Windows Tools
- **`AppState` Concurrency & Mode Sync (`src/app.rs`)**:
  - Routed background workers through panic-safe RAII `spawn_guarded_worker`, fenced slow `ScanCompleted` against `mutated_during_scan` + `epoch`, serialized hardware/power toggles with in-worker pre-state reads, moved `reset_baseline` off UI thread, and preserved `CleanTargetsScanned` checkbox states.
  - Cleared `staged_transitions` cleanly on `Normal -> Instant`, kept `Instant` switches bound to live scan state, synced `AppState.mode` in `navigate()`, unified `search_edit` + `search_query`, and separated `inspected_tweak` from `pending_items` (`open_diff_preview`).
- **Tools (`src/tools/*.rs`)**:
  - `cleaner.rs`: Added COM initialization/cleanup for `SHQueryRecycleBinW` / `SHEmptyRecycleBinW`, computed `before - after` delta, cleared `FILE_ATTRIBUTE_READONLY`, flagged `WerAndCrashDumps` elevation required.
  - `features_apps.rs`: Fixed right-to-left AppX family ID normalization (`_`/non-empty `ResourceId`), real `$removed` count & publisher-scoped deprovisioning, DISM error propagation (no fake unelevated `Disabled`), `OneDriveSetup.exe` installed-path ordering + Run cleanup, Edge multi-dir/per-user cleanup, and WinGet `--disable-interactivity` / `0x8A15002B` & `0x8A150061` handling.
  - `startup.rs`: Stopped writing nonexistent `StartupApproved\RunOnce`, added `WOW6432Node\...\RunOnce`, fixed Unicode case matching, handled `ERROR_MORE_DATA` + `SERVICE_USER_SERVICE` active service status, retained protected services cleanly, resolved `@dll,-id` MUI names, and tightened `is_microsoft` heuristics.
  - `explorer_ctrl.rs`: Verified elevation + duplicated Shell token & environment block (`CreateEnvironmentBlock`) **before** terminating single `shell_pid` cleanly.
  - `autounattend.rs`: Wrapped `reg delete` with `cmd /c reg delete ... >nul 2>&1 || exit /b 0` so absent keys never abort Windows Setup `specialize`, consulted `is_known_numeric_reg_sz`, switched to `%SystemDrive%\Users\Default\NTUSER.DAT`, gated `<HideLocalAccountScreen>` on `bypass_msa_oobe`, and blocked reserved usernames (`Administrator`, `Guest`, etc.).

### 4. Hardware FFI & UI Fixes
- **Hardware (`src/hardware/*.rs`)**: Added `SetupDiGetDeviceInstanceIdW` real PCI instance ID mapping & removed orphaned `MatchingDeviceId` fallback + token-bounded `"ATI"` vendor matching in `gpu.rs`; replaced HT/frequency heuristics with Win32 `GetLogicalProcessorInformationEx(RelationProcessorCore)` `EfficiencyClass` P/E topology in `cpu.rs` & `parking.rs`; fixed doubled `"Core #0 #0"` labels, `DefaultPowerSchemeValues` fallback, non-blocking `CoreParkingState::detect()` + background `detect_sampled()`, and `SystemMonitor` warm-up; fixed `Perf increase policy = 2 (Rocket)` & `SHLoadIndirectString` in `power.rs`; verified IOCTL `bytes_ret` & runtime `NtQuerySystemInformation(103)` + `IsSecureKernelRunning` VBS/HVCI detection in `sys_inventory.rs`.
- **UI & i18n (`src/core/i18n.rs`, `src/ui/**`)**: Expanded `TextKey::COUNT = 320` × 8 languages (updating footer to `v0.8.0`), added `DiffModal` single-inspect vs `[Revert]` direction view, disabled switches on `NotApplicable` & unsupported HAGS, added `RunAuditBtn` persistence & `latency` readout in `HardwareView`, fixed wrong `ActiveRam`/`AfterTest` labels, surfaced Cleaner errors, separated `ProtectedCore` lock look from in-flight spinners, and added loading vs empty placeholders.

---

## 🔗 Related Notes
- [[00 - Index (MOC)]]
- [[Changelog v0.6.0 to v0.7.0]]
- [[Test Suite & Validation Matrix]]
- [[Debugging & Edge Cases Resolved]]
