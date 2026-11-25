# WinBlast0 1.7.0: deep analysis and improvement options

Date: 2026-11-24 · Branch: `1.7.0-rebuild` (commit 4ccc5f6, plus the uncommitted test file `tests/vm_aggressive.rs`)

This document lists what is wrong or weak in the program and the options for fixing it. Each finding is tagged with how sure we are:

- **VM-confirmed**: reproduced on real Windows in the Hyper-V test VM.
- **Code-confirmed**: verified by reading the source; not reproduced.
- **Reviewed**: reported by one code review and checked only briefly.

The evidence (logs, fingerprints, screenshot) is in `docs/vm-test-1.7.0/aggressive/`.

---

## 1. How it was tested

| Run | Where | Result |
|---|---|---|
| Existing suites (12 binaries, `--features vm-tests`), 3 runs in a row | VM (Windows 10 Enterprise 19044) | 3 × 293 passed, 0 failed, 1 ignored; no flaky tests |
| The 3rd of those runs, under the Turkish (tr-TR) locale | VM | all passed |
| Scan fingerprint of 230 tweaks plus hardware probes in en-US, tr-TR, de-DE, ar-SA | VM | identical in every locale except RAM, which changes because of Hyper-V dynamic memory |
| Same fingerprint as a standard (non-admin) user | VM | 5 scan states differ (see F-12) |
| **New** `vm_real_catalog_apply_rollback_is_byte_exact` | VM, real HKLM, services, tasks and power settings | 176 tweaks run, **69 failures, 126 warnings** |
| **New** `vm_foreign_prior_types_survive_round_trip` | VM | 420 round trips, **301 failures** |
| **New** `vm_crash_mid_apply_recovers_from_ledger` (TerminateProcess at 150 ms to 5.2 s) | VM | **failed in all 6 rounds**: settings left changed |
| **New** `vm_concurrent_apply_rollback_stress` (16 threads × 25 iterations, one ledger file) | VM | **5 failures** |
| **New** `vm_ledger_corruption_is_survivable` (9 corrupt-file cases plus 2 concurrent writers) | VM | passed (no panics; see F-11) |
| **New** `vm_planted_rules_file_is_not_loaded` | VM | **failed**: security hole confirmed (F-1) |
| UI monkey/soak: signed release exe, 350 random clicks, 70 view switches, Esc presses, 3 minutes | VM | no crash; memory levelled off at 44 → 57 MB, handles about 380, threads 16 → 12 (no leak) |
| Code review in 4 areas (rollback, security, runtime/telemetry, test quality) | host, read-only | about 80 findings; the important ones are below |

The new tests run only with `--features vm-tests` **and** `WINBLAST_VM_AGGRESSIVE=1`, and they refuse to start unless the BIOS reports a virtual machine. They cannot damage a developer PC.

---

## 2. Findings, most severe first

### Critical / High

| ID | Finding | Evidence | Where |
|---|---|---|---|
| F-1 | **A planted rules file replaces the whole tweak catalog in the elevated app.** One `.toml` in `assets/rules` under the working directory, or in `<exe>\..\..\assets\rules`, which is `C:\assets\rules` for an exe in Program Files and creatable by any user, replaced all 230 built-in tweaks (1 tweak loaded). The elevated app then writes whatever registry values, services and tasks that file asks for. | **VM-confirmed** | `src/core/risk.rs:485-530`, `src/app.rs:815` |
| F-2 | **The updater runs whatever it downloads, without a signature or hash check.** It only checks that the file is over 1 MB. It falls back to a second GitHub account (`Devinance/WinBlast-Optimizer`) when a request fails, accepts a `.zip` asset as the exe, follows HTTP redirects, and stages the file in a user-writable %TEMP% folder before an elevated copy. | **Code-confirmed** | `src/core/updater.rs:37, 89, 178, 214-393` |
| F-3 | **Rollback deletes a value that existed with a different type.** If a target value already exists as REG_SZ text, REG_QWORD, REG_BINARY, REG_MULTI_SZ or REG_EXPAND_SZ, apply records it as "absent" and rollback **deletes** it, so the user's data is lost. 301 of 420 seeded round trips failed. Numeric REG_SZ values survive. | **VM-confirmed** | `src/scanner/registry_scanner.rs:298-346`, `src/executor/reg_mutator.rs:64` |
| F-4 | **A crash during apply leaves changes with no ledger entry.** Each change is made first and recorded afterwards. Spawned `sc`/`schtasks` processes outlive a killed app. After a hard kill, recovering from the ledger left `DisableWindowsConsumerFeatures=1` and the Edge update task disabled, with nothing able to undo them. | **VM-confirmed** | `src/executor/engine.rs:161-179` and every action branch |
| F-5 | **A service-stop failure leaves a service in the wrong state for good.** `sc config` succeeds and then `sc stop` fails (1052 for DiagTrack, 1061 for SysMain). The error aborts the step, so DiagTrack ended **Disabled (Start=4)** when it was Automatic (2) before, and the ledger can't fix it. Re-applying the SysMain tweak also fails. | **VM-confirmed** | `src/executor/svc_mutator.rs:93-126` |
| F-6 | **"Undo" (revert) writes the catalog default, not the user's previous value.** 124 of 176 tweaks ended in a different state from before. 39 created keys or policies that never existed, which can show "managed by your organization". 97 changed values that existed before. | **VM-confirmed** | `src/executor/engine.rs:758-1250` |
| F-7 | **Fake telemetry, which breaks AGENTS.md rule 2.** Reported temperatures and fan speeds are derived, not measured: CPU max core = ACPI + 4 °C, GPU hotspot = core + 8.5/10 °C, fan RPM = % × 2400. The benchmark returns fixed RAM latency 58.5 ns and disk read 6850 MB/s, and the score is clamped. "Lifetime writes" is writes since boot. Driver sizes are fixed per category. Driver install, Winsock reset and DNS apply always report success. | **Code-confirmed** (the VM has no sensors, so it showed "—") | `src/hardware/thermal.rs:89,100,148,299,322`, `src/tools/performance_tests.rs:86-87`, `src/hardware/disks.rs:346-384`, `src/hardware/driver_update.rs:731-745,1040-1101`, `src/tools/windows_repair.rs:80-163`, `src/tools/network_diag.rs:270-305` |
| F-8 | **"Apply DNS" never finds the adapter.** It filters on `InterfaceMetric`, which `Get-NetAdapter` objects don't have, then falls back to an adapter literally named "Ethernet", and prints SUCCESS whatever happened. | **Code-confirmed** | `src/tools/network_diag.rs:270-305` |
| F-9 | **No external command has a timeout.** One hung PowerShell, DISM, pnputil or curl call freezes its feature forever. In the monitor thread, it stops live telemetry for the rest of the session. | **Code-confirmed** | `src/core/process.rs:15`, `src/app.rs:1082` |
| F-10 | **A weekly SYSTEM task runs `del /q /f /s "%TEMP%\*"`.** For SYSTEM that is C:\Windows\Temp, which users can write to, so a junction there could make it delete files elsewhere. | **Code-confirmed**; the junction case was not tested | `src/tools/automation.rs:513` |

### Medium

| ID | Finding | Evidence | Where |
|---|---|---|---|
| F-11 | **Ledger robustness.** A 0-byte ledger opens silently as a new, empty history, so undo history is lost without a warning. A ledger with a foreign schema opens and accepts writes, but reading history fails. The app opens the ledger with `.ok()`, so any error means "no ledger" and nothing tells the user. `synchronous=NORMAL` can lose the last rows on power loss. There is no schema version and no index. (No panics in 9 corruption cases, and 2 concurrent writers lost 0 rows: good.) | **VM-confirmed** | `src/backup/state_ledger.rs:117-160`, `src/app.rs:832` |
| F-12 | **Scans are wrong when not elevated.** As a standard user, `disable_wmi_telemetry_autologgers` (HKLM) reads as NotOptimized instead of PartiallyOptimized, because an access-denied read is treated as "not set". A DISM-based tweak shows "Not applicable" when it really means "needs admin". | **VM-confirmed** | `src/scanner/registry_scanner.rs`, `src/core/backend.rs` |
| F-13 | **Rollback leaves empty registry keys behind.** 32 tweaks create keys (mostly under `Policies\…`) that remain, empty, after an exact rollback. Usually harmless, but rollback isn't truly exact. | **VM-confirmed** | `src/executor/reg_mutator.rs` |
| F-14 | **Concurrent toggles of tweaks that share a value lose an update.** In Instant mode each tweak runs on its own worker; 8 tweaks edit the same `UserPreferencesMask`. The stress test ended with `GameDVR_Enabled` changed. Auto-Tune isn't locked out during a batch apply or rollback. | **VM-confirmed** (once) / **Code-confirmed** | `src/app.rs:3186-3298, 5617` |
| F-15 | **Applying twice makes the optimized value the "original".** The soak test applied `cpu_parking_stock` twice and the changelog shows 2 entries. Undoing them out of order ends in the optimized state. | **VM-observed** | `src/executor/engine.rs:161` |
| F-16 | **Multi-adapter and multi-GPU paths.** Different values per adapter are recorded as "absent" and then deleted on every adapter. Rollback re-resolves the *current* active adapters, so a laptop that switches Ethernet to Wi-Fi keeps TcpAckFrequency for good. | **Reviewed** | `src/scanner/registry_scanner.rs:105-110, 365-376` |
| F-17 | **`disable_edge_update` scans as PartiallyOptimized (3/4) right after a successful apply.** The task path doesn't match the real task name, so the UI shows the tweak as not applied. | **VM-confirmed** | `assets/rules/*.toml` (Edge task path) |
| F-18 | **The restore-point age check mixes local time and UTC.** In UTC+1 or later, every apply asks for a new restore point. Windows' 24-hour throttle then makes "created" untrue. | **Code-confirmed** | `src/backup/restore_point.rs:56, 124-128` |
| F-19 | **The driver scan works only on English Windows.** It parses English `pnputil` field names. The thermal performance-counter path and the disabled-task word list are English-only as well. | **Code-confirmed** (needs a de/ru VM to reproduce) | `src/hardware/driver_update.rs:308-401`, `src/hardware/thermal.rs:251`, `src/scanner/task_scanner.rs:78-103` |
| F-20 | **Programs are started by bare name** (`powershell`, `schtasks`, `winget`), so a planted exe next to a portable copy runs elevated. Edge removal runs any `setup.exe` under %LOCALAPPDATA%. | **Code-confirmed** | `src/core/process.rs:16`, `src/tools/features_apps.rs:1059` |
| F-21 | **The UI thread does SQLite work** (`refresh_ledger` after about 17 event types, no row limit, 5 s busy timeout). The monitor thread starts `fsutil` and PowerShell every 2nd sample even when no sensor view is open. | **Code-confirmed** | `src/app.rs:4283, 1082`, `src/hardware/disks.rs:92` |
| F-22 | **"Fault fixes" delete `RebootPending`, `RebootRequired` and `PendingFileRenameOperations`** with no ledger entry. | **Code-confirmed** | `src/tools/diagnostics.rs:182-205` |
| F-23 | **The hosts file is truncated and rewritten in place**, so a crash or AV lock leaves it empty. Rolling back one hosts row removes the whole WinBlast section. | **Code-confirmed** | `src/executor/ps_runner.rs:204-215` |

### Low

- The self-signed signature shows "Publisher: Unknown" on UAC, as seen in the VM. Only a trusted (commercial) certificate fixes this.
- Version comparison treats `1.7.0-rc.1` as newer than `1.7.0` (`updater.rs:50-66`).
- Preset sharing (1542 lines) and `ui_sync` have no callers. Ship them or delete them. If preset sharing is wired up, `VerifyIfPresent` must not accept unsigned presets.
- Web pages open from the elevated process, so the browser runs as admin.
- Thermal throttling detection ignores bit 0 (`thermal.rs:266`). "DPC jitter" measures user-mode scheduling, not DPC latency.
- Startup does OS, CPU and GPU detection, SQLite work and core-parking probes on the UI thread before the window appears.

### Test-suite weaknesses (from the test review)

- The sandbox lifecycle test checks only *scan state*, not exact values, so it passes even when rollback writes the wrong value. That is why F-3, F-5 and F-6 were never caught.
- `fuzz_mutation_properties` hard-codes `with_cases(50)`, so `PROPTEST_CASES` is ignored. Its rollback property can pass without testing anything.
- Several tests can't fail. For example, `tweak_sandbox_e2e.rs:4445` sets flags and asserts them; some "stress" tests only check that nothing panicked.
- VM tests print "Skipping…" and pass when access is denied.
- `driver_update.rs:1139` is a test-only helper inside shipped code that runs `pnputil /scan-devices` on the host.

---

## 3. Improvement options

Each option is a package that can be done on its own. Effort: S = under half a day, M = 1–2 days, L = 3+ days.

### Option A: Make rollback trustworthy (recommended first)
Fixes F-3, F-4, F-5, F-6, F-11, F-13, F-15 and F-16.
1. **Write-ahead ledger.** Insert a `pending` row with the exact raw prior value (type + bytes, via `reg_get_raw`) **before** each change, then mark it `committed`. On startup, check pending rows against the live state. Use `synchronous=FULL` and add a `user_version` schema version and an index.
2. **Raw-typed snapshots for every registry action.** Record `RegRawSnapshot` instead of "parsed u32 or ABSENT", and restore exactly the same type and bytes. Record "key existed" so rollback can delete only keys it created.
3. **Service actions as two steps.** Treat the start type and the stop/start as separate, individually recorded steps. A failed stop must not abort a start-type change that already happened.
4. **"Undo" restores the recorded prior state,** not catalog defaults. Keep "Reset to Windows default" as a separate, clearly named action.
5. **Skip actions already at the target value,** so re-applying records nothing new.
6. **Per-value locking** (or one mutation worker) so Instant mode and Auto-Tune can't interleave on the same value.
7. Tell the user when the ledger can't be opened or read ("Undo history unavailable"), instead of `.ok()`.

Effort: L. Proof: `tests/vm_aggressive.rs` A, B, D and F must pass in the VM.

### Option B: Close the security holes (recommended first, together with A)
Fixes F-1, F-2, F-10, F-20, F-22 and F-23.
1. Release builds load **only the embedded catalog**. Keep the disk loader behind a debug/dev flag. Test G turns green.
2. Updater:
   - publish a signed manifest (ed25519 key built into the exe) with SHA-256 per asset, and check it before the swap;
   - check the Authenticode signer;
   - only `.exe` assets;
   - HTTPS only (`--proto =https --proto-redir =https`, host allow-list);
   - remove the fallback repo;
   - stage in an admin-only folder;
   - ask before restarting.
3. Replace the SYSTEM `del /s` task with the built-in cleaner, which skips junctions.
4. Start programs by full System32 path. Allow only Microsoft-signed `setup.exe` from Program Files.
5. Write the hosts file to a temp file and swap it in with `ReplaceFileW`. Remove the "fault fixes" that delete servicing state, or make them recorded and expert-only.

Effort: M–L.

### Option C: Honest telemetry and results (AGENTS.md rule 2)
Fixes F-7, F-8, F-18 and F-19.
- Remove every derived reading: hotspot, max core, RPM, ambient, "lifetime writes" and driver sizes. Show "—" with "needs vendor sensor".
- Make the benchmark actually measure RAM latency and disk read, and drop the clamp.
- DNS:
  - choose the adapter by `Get-NetIPInterface` route metric;
  - use `Set-DnsClientServerAddress -InterfaceIndex`;
  - report the real result.
- Repair tools check exit codes and read UTF-16 `sfc` output.
- Make parsing language-independent: use `pnputil /format xml` or CIM instead of English text, and PDH counter indexes instead of English names.

Effort: M.

### Option D: Robustness and responsiveness
Fixes F-9, F-12, F-14 and F-21.
- One `run_with_timeout` helper for every external process, with kill-on-timeout and a clear error.
- Report a crashed monitor thread or worker to the UI; use `spawn_guarded_worker` for the update threads.
- Move ledger reads off the UI thread, keep one connection, and page the history.
- Run the disk/thermal probes only while a sensor view is open, and cache them.
- Treat "access denied" during a scan as "Unknown – needs admin" rather than "not set".
- A single-instance guard.

Effort: M.

### Option E: Test-suite hardening
- Keep `tests/vm_aggressive.rs` (added in this analysis) and run it in the VM before every release, the same way the VM suite is run now.
- Make the sandbox lifecycle test compare exact values (`SandboxState: PartialEq`) instead of scan state.
- Make `PROPTEST_CASES` work, count the tweaks actually exercised, and add absent, string, 0 and `u32::MAX` priors.
- Make "Skipping…" a failure under `vm-tests`. Delete tests that can't fail; move `driver_update.rs:1139` out of shipped code.
- New VM scenarios:
  - Updater: a local mirror that serves tampered, `.zip`-first and HTTP-redirect assets.
  - Run as a standard user.
  - Network adapters: Ethernet → Wi-Fi switch between apply and rollback.
  - Time zones UTC+10 and UTC−8.
  - A German or Russian language image for pnputil and the task scanner.
  - Two app instances at once.
  - A 100k-row ledger with a UI tick-time check.
  - A Windows 11 VM: 27 tweaks need build 22000+ and have never run on real Windows.

Effort: M.

### Option F: Product polish (lower priority)
- A trusted code-signing certificate (removes "Publisher: Unknown" and SmartScreen warnings).
- Fix the Edge update task path (F-17).
- Remove or wire up preset sharing and `ui_sync`.
- Open web pages from the non-elevated shell.
- Move startup detection off the UI thread for a faster first window.

Effort: S–M each.

---

## 4. Recommendation

1. **A + B together** come first. Rollback that loses or changes user data, and an elevated app that runs planted rules or unverified updates, are the two things that can really hurt users. The VM tests already show exactly what "fixed" means.
2. Then **C**, which is a stated project rule (no fabricated telemetry), and **D**.
3. **E** continuously. Its first part, `tests/vm_aggressive.rs`, is already written and has found F-1, F-3, F-4, F-5, F-6, F-12, F-13, F-14 and F-17.

## 5. Running the aggressive VM tests

```powershell
# inside the disposable VM only, elevated, then restore the checkpoint
$env:WINBLAST_VM_AGGRESSIVE = '1'
cargo test --features vm-tests --test vm_aggressive -- --include-ignored --test-threads=1 --nocapture
# locale / user fingerprint:
$env:WINBLAST_FINGERPRINT_OUT = 'C:\fp.json'; cargo test --features vm-tests --test vm_aggressive vm_scan_fingerprint -- --include-ignored
```

VM: Hyper-V "Windows 10 MSIX packaging environment", checkpoint **`clean-test`**. That is `clean` plus the `test` admin account, which the old `clean` checkpoint did not contain.
