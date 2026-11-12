---
title: Debugging & Edge Cases Resolved
tags: [debugging, subagent, audit, bugfixes, edgecases]
created: 2026-11-08
---

# 🛠️ Debugging & Edge Cases Resolved

Before completing v0.2.0, a dedicated **Rust Systems Debugger & Fixer Agent** (`winblast-debugger-fixer`) was spawned to perform an exhaustive audit of the codebase, hunt down edge cases, and ensure rock-solid stability.

---

## 📋 Comprehensive Bug Fix Log

### 1. Slice Index Panic in Rollback Engine (`src/executor/engine.rs`)
- **Bug**: In `rollback_entry`, registry key targets were parsed using `first_slash = target.find('\\')` and `last_slash = target.rfind('\\')`. For top-level keys with a single slash (e.g. `HKLM\ValueName`), `first_slash == last_slash`. Slicing `&target[first_slash + 1..last_slash]` caused Rust to panic with `slice index starts after end`.
- **Fix**: Added explicit guard:
  ```rust
  let (hive, path, name) = if first_slash == last_slash {
      (&target[..first_slash], "", &target[first_slash + 1..])
  } else {
      (&target[..first_slash], &target[first_slash + 1..last_slash], &target[last_slash + 1..])
  };
  ```

### 2. SQLite Database Lock Collisions (`src/backup/state_ledger.rs`)
- **Bug**: Concurrent access between background scanning threads, 1-second monitoring workers, and UI actions triggered `sqlite error: database is locked (busy)`.
- **Fix**: Configured a **5-second busy timeout** (`conn.busy_timeout(Duration::from_secs(5))`) and enabled **Write-Ahead Logging (WAL)**:
  `PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;`.

### 3. Integer Overflow Panic on Impact Gauge (`src/ui/views/easy_dashboard.rs`)
- **Bug**: Calculating delta text used unary negation: `-impact.ram_saved_mb`. In debug mode, negating `i64::MIN` panics with arithmetic overflow.
- **Fix**: Replaced unary negation with `.saturating_abs()`.

### 4. Hosts File Line Corrupting Concatenation (`src/executor/ps_runner.rs`)
- **Bug**: If `C:\Windows\System32\drivers\etc\hosts` did not terminate with a newline character, appended block entries concatenated directly onto the last DNS record (e.g. `127.0.0.1 localhost0.0.0.0 domain`), corrupting resolution.
- **Fix**: Checked `!existing.is_empty() && !existing.ends_with('\n')`, prepending `\r\n` before appending entries formatted with standard Windows CRLF line endings.

### 5. Multi-Host Tokenizer Omission (`src/scanner/telemetry_scanner.rs`)
- **Bug**: `get_blocked_hosts()` only examined the second token (`parts[1]`). Standard hosts files allowing multiple domains on a single line (e.g. `0.0.0.0 host1 host2 host3`) only detected the first host.
- **Fix**: Updated parser to iterate across all domain tokens (`parts[1..]`).

### 6. Absent Telemetry Services Falsely Reported as Disabled
- **Bug**: `evaluate_service` treats non-existent services as "safe" when the target start type is `4` (Disabled). As a result, third-party services that were never installed (e.g. Razer Synapse, Adobe GC Invoker) were falsely reported as "Disabled" and counted as optimizations.
- **Fix**: In `full_telemetry_audit`, directly queries `get_service_start_type(svc)`: absent services (`None`) are omitted, only real existing services are cataloged.

### 7. Empty GPU PnP ID Registry Mutation Hazard (`src/hardware/gpu.rs`)
- **Bug**: If a display adapter lacked a PnP hardware ID, `set_gpu_msi_mode` attempted to create an invalid registry key (`SYSTEM\CurrentControlSet\Enum\\...`).
- **Fix**: Added validation returning `std::io::ErrorKind::InvalidInput` if `pnp_instance_id.trim().is_empty()`.

### 8. AppX Empty Token Matching Guard (`src/scanner/appx_scanner.rs`)
- **Bug**: If an empty package name was passed to `AppxCache::evaluate_package`, prefix matching could match unrelated entries.
- **Fix**: Added guard clause returning `(true, "No package specified")`.

### 9. Rolling Sparkline Buffer Bounding (`src/app.rs`)
- **Bug**: In `WorkerEvent::MetricsSnapshot`, length was checked with `if len() >= 35`, which could permit buffer drift if it ever exceeded 35 items.
- **Fix**: Replaced with `while len() >= 35`, strictly guaranteeing bounded buffer sizes.

### 10. Rollback Failure on Reverted Tweaks (`src/executor/engine.rs`)
- **Bug**: When `revert_tweak` logged an action to SQLite, it formatted the action key as `Revert:RegistryDword:...`. When `rollback_entry` subsequently parsed the action type with `.splitn(2, ':')`, `parts[0]` evaluated to `"Revert"` instead of the action kind, triggering `Unknown action type in ledger: Revert` and failing the rollback.
- **Fix**: Added `.strip_prefix("Revert:")` prior to splitting action types, seamlessly enabling single-click rollback for both applied and reverted tweak records.

### 11. Privacy Shield Switch & Staged State Desync (`src/ui/views/privacy_shield.rs`)
- **Bug**: Privacy Shield switches were bound to `s.selected_tweaks.contains(id)` rather than `s.is_switch_active(id)`. On launch, privacy tweaks already disabled on Windows displayed as OFF instead of ON. Furthermore, toggling switches bypassed `handle_switch_click` staging logic and did not reflect `in_flight` state.
- **Fix**: Unified switches to use `s.is_switch_active`, `s.in_flight_tweaks`, and `s.handle_switch_click`, correctly honoring live Windows status and Instant/Normal execution modes.

### 12. Sparkline Float NaN Edge Case Panic (`src/ui/widgets/sparkline.rs`)
- **Bug**: Rust's `f32::clamp(min, max)` panics if the input value is `NaN`. If hardware telemetry transiently yielded `NaN` during sleep/wake or hypervisor transitions, the UI panicked.
- **Fix**: Sanitized inputs using `if val.is_finite() { val } else { 0.0 }` prior to clamping.

### 13. SQLite Directory Auto-Creation (`src/backup/state_ledger.rs`)
- **Bug**: If `ledger_path` was specified in a non-existent directory, `Connection::open` failed with `unable to open database file`.
- **Fix**: Added `std::fs::create_dir_all(parent)` before opening SQLite connection.

### 14. Real-Time Accurate NT Thread Count Telemetry (`src/hardware/monitor.rs`)
- **Bug**: `thread_count` was computed via `processes().values().map(|_| 1).sum()`, which produced the process count rather than real OS thread counts (~200 vs ~5,900).
- **Fix**: Integrated Win32 `GetPerformanceInfo` from `psapi.dll`, reading exact Windows system-wide execution thread counts directly with zero-overhead fallback.

### 15. Clippy Argument Lint (`src/backup/state_ledger.rs`)
- **Bug**: `reset_baseline` took 8 parameters without `#[allow(clippy::too_many_arguments)]`, emitting a compiler lint.
- **Fix**: Added `#[allow(clippy::too_many_arguments)]` matching `save_baseline`.

---

## 🔍 v0.6.0 ➔ v0.7.0 Fixes (`winblast-tester` + `winblast-debugger-fixer`)

### 16. 33 Tweaks Dropped by Missing `Reversibility::RequiresLogoff` (`src/core/risk.rs`)
- **Bug**: `perf_power_ext.toml` and `ux_customization.toml` used `reversibility = "RequiresLogoff"`, which failed TOML deserialization into `Reversibility` and caused file-level loader skipping (dropping 33 of 168 tweaks).
- **Fix**: Added `Reversibility::RequiresLogoff` and enforced strict raw-vs-loaded count verification (`168 == 168`) in `tests/tweak_sandbox_e2e.rs`.

### 17. Interactive Switch Routing & Changelog Ordering (`src/ui/widgets/button.rs`, `clean_apps.rs`, `startup_tasks.rs`, `ui/mod.rs`)
- **Bug**: `switch_toggle` unconditionally called `handle_switch_click` (polluting `selected_tweaks` on Clean/AppX/WinGet/Startup rows); Changelog Drawer called `.iter().rev().take(12)` on an already-`DESC` query (displaying oldest 12 instead of newest 12 entries).
- **Fix**: Added `switch_indicator` & `switch_toggle_with`, switched Changelog to `.iter().take(12)`, gated Undo via `LedgerEntry::is_reversible()`, and moved Compact quick pills below nav items with `main_body.min_h(120.0)` so drawer items stay clickable and `nav_sidebar_scroll` never collapses to `0 pt`.

### 18. Ledger Rollback Parity & Async Concurrency (`src/app.rs`, `src/executor/engine.rs`, `src/core/backend.rs`)
- **Bug**: Mismatched ledger formats (`GpuMsi`, `WindowsFeature`, `ScheduledTask`, `HostsBlock`, `PowerCfg`, `StartupApproved`, `UsbRootHubPowerSaving`) caused failed/no-op Undo; single `clean_apps_busy` bool blocked simultaneous Disk + AppX scans on navigation; missing `RegistryDynamicDword` `u64` math overflowed `u32` on >=48 GB RAM hosts.
- **Fix**: Unified transactional rollback formats, implemented real hosts unblock (`unblock_telemetry_hosts_with`), sibling-preserving `RegistryBinaryBit` rollback, `PowerCfg` scheme/hibernation/USB rollback, separate inflight flags, off-thread `detect_gpus()` + `RollbackRefreshData`, and `u64` RAM formula math.

### 19. DISM, WinGet, Win32 SCM, Explorer Token Restart, & `autounattend.xml` (`src/tools/*.rs`, `src/scanner/mod.rs`)
- **Bug**: DISM lacked `/English` and full `~~~~` capability suffix matching; `winget` validator rejected `+` (`Notepad++.Notepad++`); `sc query` failed on non-English OS; `explorer.exe` restart raced multi-session `taskkill`; `autounattend.xml` tied `BypassNRO` to TPM bypass.
- **Fix**: Enforced DISM `/English` + `3010` + `~~~~` prefix matching across real & `SandboxEnv` paths, exported `validate_winget_package_id`, replaced `sc query` with Win32 `OpenSCManagerW`/`EnumServicesStatusExW`, duplicated Explorer token before single-PID exit/wait, and fixed `autounattend.xml` escaping, `BypassNRO`, and `TargetArch`.

---

## 🔍 v0.7.0 ➔ v0.8.0 Fixes (EGM Adversarial Audits + `winblast-debugger-fixer` + `winblast-tester`)

### 20. `sc config` Single-Arg Syntax Failure, `REG_SZ` UTF-16LE Corruption & Catalog Ground Truth
- **Bugs**: `svc_mutator.rs` passed `"start= auto"` as a single quoted arg to `sc.exe` (rejected) and lost `DelayedAutostart = 1`; `read_dword_from_key` fell through to raw UTF-16LE bytes on non-integer `REG_SZ` & `> u32::MAX` `REG_QWORD`; `os_detect.rs` lacked `KEY_WOW64_64KEY` & `ReleaseId`; Paint/CopilotKey/Recall24H2/WifiSense/Tcp1323Opts/MMCSS/NTFS/ServicesPipeTimeout/NVMe/AutoRun/Ducking/PowerMizer rules held incorrect stock paths, ACLs, types, or defaults.
- **Fixes**: Split `"start="` and mode args cleanly, preserved `DelayedAutostart`, dispatched `read_dword_from_key` strictly by `REG_*` type, expanded `is_known_numeric_reg_sz` across 29 verified Build 26300 keys, and corrected all affected TOML definitions + added active `<NET_INTERFACE_GUID>` NIC targeting.

### 21. Atomic `revert_tweak_with`, `SandboxEnv` Fidelity, & Worker Panic/Epoch Fencing
- **Bugs**: Worker thread panics could leave `is_scanning`/`is_applying`/`clean_apps_busy`/`startup_tasks_busy`/`in_flight_tweaks` deadlocked; slow `ScanCompleted` could overwrite concurrent `SingleTweakUpdated`; `Normal -> Instant` toggle left `staged_transitions` executing inverted switches; `LedgerEntry::is_reversible` omitted `"AppXPackage"`; `autounattend.rs` bare `reg delete` aborted Setup on missing keys; `explorer_ctrl.rs` killed desktop Explorer before verifying `SeImpersonatePrivilege`.
- **Fixes**: Added panic-safe RAII `spawn_guarded_worker`, `mutated_during_scan` + `epoch` fencing, clean `Normal <-> Instant` & `Easy <-> Advanced` sync, atomic `revert_tweak_with` reverse rollback, Setup-safe `cmd /c reg delete ... || exit /b 0`, pre-kill Shell token & `CreateEnvironmentBlock`, and unified `SandboxEnv` 1-store semantics + `split_appx_name_and_publisher`.

### 22. Multi-GPU `SetupDiGetDeviceInstanceIdW`, `EfficiencyClass` P/E Cores, & UI State Logic
- **Bugs**: `gpu.rs` fell back to `MatchingDeviceId` causing dual-GPU collisions and orphaned `\Enum` keys plus `"ATI"` substring false matches; `parking.rs`/`cpu.rs` hit `"Core #0 #0"` doubled labels, cold `0.0%` false-parked cores, `DefaultPowerSchemeValues` short-circuit, and `p_cores = 0` on Core Ultra / HT-disabled CPUs; `power.rs` set `Single` (`3`) instead of `Rocket` (`2`); UI showed Undo on irreversible `AppXPackage`, swapped metric labels, hid `RunAuditBtn` post-audit, and left `NotApplicable` toggles active.
- **Fixes**: Enumerated true `SetupDiGetDeviceInstanceIdW`, queried `GetLogicalProcessorInformationEx` `EfficiencyClass`, split instant `CoreParkingState::detect()` vs background `detect_sampled()`, fixed `Rocket = 2`, separated `inspected_tweak` & `[Revert]` `DiffModal`, and enforced `NotApplicable` locking across all views.

---

## 🔗 Related Notes
- [[Test Suite & Validation Matrix]]
- [[State Ledger & Rollback Engine]]
- [[00 - Index (MOC)]]
