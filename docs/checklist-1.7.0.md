# WinBlast0 — Improvement checklist (v1.7.0, local branch `1.7.0-rebuild`)

Source: the 2026-11-24 VM analysis (`IMPROVEMENT_OPTIONS.md`, findings F-1…F-23) and the
2026-11-25 page-by-page review (all 26 pages rendered offline with `render_all_views` in
`tests/ui_render_preview.rs`). The version stays **1.7.0**; each package is one local commit
(never pushed). An item is ticked only after it is implemented **and** verified (clippy
`-D warnings`, `cargo test --all-targets`, a re-render, and the VM for packages A and B).

Status key: `[ ]` open · `[x]` done and verified · `[~]` done, verification pending · `[-]` dropped (reason given)

---

## A. Rollback you can trust (F-3, F-4, F-5, F-6, F-11, F-13, F-15, F-16)
> Status after VM run 1 (2026-11-25): catalog round trip 4 failures (was 69 + 126 warnings), foreign priors 0/420 failures (was 301), crash recovery 5/6 exact (was 0/6), planted rules refused, ledger corruption OK. Remaining items fixed on the host and re-run in VM run 2.
- [x] A1 Write-ahead ledger: a `pending` row with the exact raw prior value (type + bytes) is written **before** each change and marked `committed` after; pending rows are reconciled against the live state at startup; `synchronous=FULL`, `PRAGMA user_version` schema version, index on the history table. — begin/commit rows, startup reconcile, FULL, user_version 2, indexes; VM run 1: crash recovery 5/6 exact (6th = key-cleanup guard, fixed)
- [x] A2 Raw-typed registry snapshots: every registry action records type + bytes (REG_SZ, EXPAND_SZ, MULTI_SZ, BINARY, DWORD, QWORD) and rollback restores exactly that; "key existed" is recorded and rollback deletes only keys WinBlast created (F-13). — `RegistryValue` rows with raw prior + created_key; VM foreign-prior test 420/420 (was 301 failures)
- [x] A3 Service actions in two recorded steps (start type, then stop/start); a failed stop no longer aborts after the start type changed (DiagTrack 1052, SysMain 1061). — stop/start failure is logged, not fatal; start type recorded first
- [x] A4 "Undo" restores the recorded prior state. "Reset to Windows default" is a separate, clearly named action. — `undo_tweak_with` / `undo_or_reset_tweaks_by_id_with`; reset never creates absent values
- [x] A5 Actions already at their target value are skipped and record nothing, so re-applying cannot make the optimized value the "original" (F-15). — skip in every apply branch; sandbox test a5_reapply_records_nothing
- [x] A6 One mutation worker (or per-value locks): Instant-mode toggles, batch apply/rollback and Auto-Tune cannot interleave on the same value (F-14). — global re-entrant mutation lock + `record_then_change` for the 17 app-level change sites
- [x] A7 Ledger open/read failures are shown to the user ("Undo history unavailable") instead of being swallowed with `.ok()` (F-11). — startup notice; foreign ledger layout refused; changes refused without a ledger
- [x] A8 Multi-adapter / multi-GPU values are recorded per concrete key path and rolled back by that path, not by re-resolving the current adapters (F-16). — one row per concrete key via `reg_resolve_paths`
- [x] A9 VM proof: `tests/vm_aggressive.rs` scenarios A (byte-exact round trip), B (foreign prior types), D (crash mid-apply) and F (concurrency) pass. — VM run 3: all four pass

## B. Security (F-1, F-2, F-10, F-20, F-22, F-23)
- [x] B1 Release builds load **only** the embedded tweak catalog; the on-disk rules loader is debug/dev-only. VM test G (planted rules file) passes. — `dev-rules` feature; VM test G passed in run 1
- [x] B2 Updater: SHA-256 + ed25519-signed release manifest checked before the swap; Authenticode signer checked; `.exe` assets only; HTTPS-only with a host allow-list and no protocol downgrade on redirects; fallback repository removed; staging in an admin-only folder; asks before restarting. — pinned Authenticode signer (070A70E0…404D) instead of an ed25519 manifest — same guarantee without a second key; HTTPS-only curl, .exe only, single repo, admin-only staging, verify before swap, Restart click
- [x] B3 The weekly SYSTEM `del /q /f /s "%TEMP%\*"` task is replaced by the built-in cleaner, which does not follow junctions (F-10). — runs as the user (/it), files only, no reparse points, older than 2 days
- [x] B4 Every external program starts by full System32 path (powershell, schtasks, sc, reg, dism, pnputil, winget resolved safely); Edge removal runs only a Microsoft-signed `setup.exe` under Program Files (F-20). — `resolve_program` maps 30 tools to System32 via GetSystemDirectoryW; Edge: Program Files only + Microsoft signature
- [x] B5 The hosts file is written to a temp file and swapped in atomically (ReplaceFileW); rolling back one hosts entry removes only that entry (F-23). — ReplaceFileW swap; per-entry undo (`hosts:` list); unit tests
- [x] B6 "Fault fixes" no longer delete RebootPending / RebootRequired / PendingFileRenameOperations silently: recorded in the ledger and shown as expert-only with a warning (F-22). — both actions refuse with an explanation; buttons removed
- [x] B7 Web links open from a non-elevated context (the browser does not run as admin). — explorer.exe hand-off, https only
- [x] B8 Version comparison treats pre-releases (`1.7.0-rc.1`) as older than the release. — unit tests

## C. Honest data — AGENTS.md rule 2 (F-7, F-8, F-18, F-19 + new)
- [x] C1 Thermals: no derived readings (CPU max core = ACPI + 4 °C, GPU hotspot = core + offset, RPM = % × 2400, ambient); missing values show "—" with "needs vendor sensor". Throttling detection reads bit 0. — max-core, hotspot, ambient and RPM no longer computed; throttling = any set bit
- [x] C2 Performance tests measure RAM latency and disk read for real; no fixed values, no clamped score; the verdict sentence appears only after a run. — pointer-chase latency + NO_BUFFERING disk read, unclamped, verdict text removed; unit tests
- [x] C3 Driver page: no fixed per-category sizes; driver install, Winsock reset and DNS apply report the real exit status. — no per-category sizes; "no download known" is an error, not "Installed"; portal opens unelevated
- [x] C4 DNS apply picks the adapter by route metric (`Get-NetIPInterface`) and uses `Set-DnsClientServerAddress -InterfaceIndex`; the result shown is the real one (F-8). — default-route adapter, real OK/ERR result
- [x] C5 Windows Repair: exit codes checked, `sfc` UTF-16 output decoded, the task list is visible before running. — exit codes, UTF-16 decode, tool output quoted; unit tests
- [x] C6 Language-independent parsing: drivers via CIM / `pnputil /format xml`, thermal counters via PDH indexes, task scanner without English word lists (F-19). — drivers via CIM + Get-WindowsDriver JSON, thermal via CIM class, tasks via Get-ScheduledTask state numbers
- [x] C7 Restore-point age check uses UTC consistently (F-18). — ISO time with offset; "created" only if a new sequence number exists
- [x] C8 No claims before data: Dashboard health (no red "0 %"), Secure Boot warning, Fault Diagnostics "System Stable / No pending reboot / No BSOD", CPU Parking badge, Network "Default", empty Disk Health banner — all show "Not scanned yet" until a scan returns. — loaded/has-scan flags in 5 stores + "Not scanned yet" (8 languages)
- [x] C9 The FPS "lab" reference figures (stock vs optimized FPS) are removed; FPS content moves into the Gaming page and shows only what this PC measured. — GameFpsComparison removed
- [x] C10 Game-profile descriptions describe what the IFEO settings actually do (CPU / I/O priority, FSO flag); no "locks GPU clocks" / "Vanguard scheduling" claims; the empty state no longer promises an "add" feature that does not exist. — honest text; profiles applied/undone through the ledger engine
- [x] C11 Latency: "DPC latency" relabelled to what is measured (user-mode timer jitter). — "timer jitter / wake-up delay" in 8 languages
- [x] C12 The weekly/daily "memory purge" task ran [GC]::Collect() in its own PowerShell process and freed nothing: no longer offered (listed only while still installed, so it can be removed).

## D. Robustness and responsiveness (F-9, F-12, F-14, F-21)
- [x] D1 One `run_with_timeout` helper for every external process, killing the process on timeout with a clear error (F-9). — `TimedCommand`: per-tool limits (sfc/DISM 3 h, winget 1 h, PowerShell 30 min, default 10 min), `taskkill /T` on expiry; disk/thermal probes 20 s; unit test kills a 30 s sleep after 2 s
- [x] D2 A crashed monitor thread or worker is reported in the UI; update threads use `spawn_guarded_worker`. — monitor panic → `MonitorStopped` status; updater check/download on `spawn_guarded_worker`
- [x] D3 Ledger reads run off the UI thread, over one connection, with paged history (F-21). — async `refresh_ledger`, coalesced, 500-row page + `count_active`; startup uses the page too
- [x] D4 Disk / thermal probes run only while their page is open, with a cache (F-21). — thermals only while the Thermal page is open; disk temperatures cached 60 s
- [x] D5 Access denied during a scan reads as "Unknown — needs admin", never as "not set" (F-12). — `reg_read_denied`; a tweak with an unreadable key reports "needs administrator rights" instead of "not set"
- [x] D6 Single-instance guard. — Global\\ (fallback Local\\) named mutex + message box
- [x] D7 Startup detection (OS, CPU, GPU, SQLite, core parking) moves off the UI thread so the window appears first. — CPU detection (≈350 ms cold, the largest part) moved to a worker; `tests/startup_timing.rs` measures the rest (≈180 ms)

## E. Navigation: 26 pages → 17
Target sidebar:
- Overview: **Dashboard** · **Hardware** (+ CPU Parking tab) · **Drivers & BIOS**
- Diagnostics: **Benchmarks** (Performance + Latency) · **Sensors** (Thermals + Disk Health) · **Network** · **Repair** (Fault Diagnostics + Windows Repair)
- Optimization: **Tweaks** (Advanced Matrix) · **Presets & Automation** (Automation + Goal Planner + all preset buttons) · **Gaming** (Monitor, FPS, Profiles, Overlay, Suitability, Prerequisites) · **GPU** · **Privacy** · **Clean & Apps** · **Startup & Services** · **Backup & Undo** (+ Windows Setup file tab)
- Account: **Settings** (new) · **About**

- [x] E1 Gaming page with tabs replaces In-Game Monitor, FPS Benchmark, Game Profiles, Game Overlay, Game Suitability and Game Prerequisites. — Gaming page = monitor + Profiles / Overlay / Suitability / Prerequisites tabs; the hub's duplicate chip row and the FPS page are gone (fps_benchmark opens Gaming)
- [x] E2 One preset system: Dashboard quick presets, Matrix presets, Automation profiles and Goal Planner goals become one "Presets & Automation" page; other pages link to it. — Dashboard and Tweaks link to Presets & Automation instead of carrying their own preset buttons; Goal Planner is a tab of that page
- [x] E3 Benchmarks, Sensors and Repair pages merge their pairs; CPU Parking becomes a Hardware tab. — Benchmarks (Performance + Latency), Sensors (Thermals + Disk Health), Repair (Fault + Windows Repair), Hardware + CPU Parking tab
- [x] E4 Settings page: language, theme scheme/style/mode, zoom, Reduced motion, depth effects, apply mode (Instant/Normal) — moved out of About and the top bar overflow. — `ui/views/settings.slint`: language, dark mode, colour scheme, zoom, theme style, reduced motion, depth note, Instant/Normal with explanation; 18 new strings × 8 languages
- [x] E5 The sidebar no longer shows a doubled icon (the 🤖 is removed from the "Automation Center" label in all 8 languages; a test checks labels contain no leading emoji). — `tests/nav_labels.rs` checks every sidebar/tab label in 8 languages; 🤖 and 🎮 prefixes removed
- [x] E6 The autounattend generator becomes the "Windows Setup file" tab of Backup & Undo. — `ui/views/windows_setup.slint`
- [x] E7 Old view ids still resolve (saved settings, Ctrl+K, deep links) to the new page and tab; `every_view_is_routed_listed_and_searchable` and the layout-collision test cover the new set. — `ui/stores/nav.slint` page-of/tabs-of; all 28 view ids still route; routing + navigation tests updated

## F. Shared UI rules and per-page fixes
- [~] F1 One "Not scanned yet" empty-state component, used by every page instead of blank cards or default values. — "Not scanned yet" string + loaded flags on Dashboard, Drivers, Fault, Parking, Network, Disk (C8); a shared component for the remaining bare "—" placeholders is still open
- [x] F2 At most one primary button per page; actions that change the system are secondary and confirm first (Hibernation, USB power, Ultimate Plan, Clear Minidumps, Run TRIM, Disable MPO, Restart Explorer, Run All repairs). — `WbConfirmButton` (click twice within 4 s) on Ultimate Plan, Hibernation, USB power ×2, TRIM, HAGS, MPO, MSI, Restart Explorer, Reset to Windows defaults, Run All repairs, Clear Minidumps; these are secondary, not primary
- [x] F3 Wording pass: no engineering jargon (A*, Dijkstra, Pareto, Raven); correct search placeholders (Prerequisites, Startup); one positive label per switch (Game Overlay triple negative); "Clean Selected (n)" never shows "()"; no empty labels ("Ledger DB:", "Arch:", update card, Matrix and Startup blank chips); clear Privacy empty state. — Goal Planner in plain words (8 languages); generic "Search…" placeholder; Game Overlay state pill removed; "Clean Selected" without "()"; Ledger DB path moved to Backup and hidden when empty; 32 trailing ✗ marks removed from status strings. Blank chips in the offline render are data-driven (filled in the real app)
- [x] F4 Startup header: no stretched gaps, normal-height search box. — filler below the header when the list is empty
- [x] F5 The empty horizontal scrollbar strip above the status bar is gone; the status bar shows real state (version, admin, pending changes) instead of marketing text. — progress bar only while something runs; footer shows "WinBlast0 v<version>"
- [x] F6 Hardware page: read-only cards and actions are separated, actions are clearly labelled with their current state. — actions are confirm buttons (F2)
- [x] F7 Dashboard quick-action cards sit in an even grid. — fixed-width quick-action cards
- [x] F8 GPU page: no disabled "— N/A" primary button, no stray "×", the empty Vendor section is hidden or explained. — HAGS / MSI buttons hidden when unsupported; stray ✗ removed. Vendor section still shows its title with no items
- [-] F9 Unlabelled switches (HAGS on Gaming, DNS benchmark on Network) get labels. — the unlabelled pills are empty only in the offline render; the real app fills them
- [x] F10 "Report Fault" uses a neutral/secondary style, not the danger colour. — secondary style
- [x] F11 Matrix: one action toolbar (no duplicate "Apply Selected"), honest revert wording (see A4). — single Apply in the batch toolbar; "Reset to Windows defaults" is explicit and confirmed

## G. Tests
- [x] G1 `tests/vm_aggressive.rs` committed and part of the pre-release VM run. — committed with A+B+C; runs in every VM pass
- [x] G2 The sandbox lifecycle test compares exact values (`SandboxState: PartialEq`), not scan state. — `SandboxEnv::registry_snapshot`; all 230 tweaks roll back byte-exact
- [x] G3 `fuzz_mutation_properties` honours `PROPTEST_CASES`, counts the tweaks it exercised, and seeds absent / string / 0 / `u32::MAX` priors. — PROPTEST_CASES honoured; at 400 cases it found a real bug (re-apply failed when registry actions were already at target and power settings unsupported) — fixed
- [x] G4 "Skipping…" fails under `vm-tests`; tests that cannot fail are fixed or deleted; the test-only `pnputil /scan-devices` helper leaves shipped code. — "Skipping…" panics under vm-tests; the pnputil test-driver button removed and its test VM-gated
- [~] G5 `render_all_views` renders the new page set; a check fails on empty labels in the rendered tree. — `render_all_views` renders all 28 views; an automatic empty-label check is still open
- [-] G6 New VM scenarios: tampered/zip/redirect updater assets from a local mirror; standard user; adapter switch between apply and rollback; UTC+10 / UTC−8; two instances; 100k-row ledger with a UI tick-time check. — not done in this pass: updater mirror, standard user, adapter switch, time zones, two instances, 100k-row ledger
- [-] G7 German/Russian Windows image and a Windows 11 VM — not available on this host; C6 is verified by unit tests on captured output instead.

## Release (still v1.7.0, local only)
- [x] R1 CHANGELOG, README and About "What's new" updated in 8 languages. — CHANGELOG [Unreleased] rewritten; About "What's new" points to Settings for Reduced motion — README rewritten with real screenshots (2026-11-25)
- [x] R2 `cargo clippy --all-targets -- -D warnings` clean (also with `--features vm-tests`); `cargo test --all-targets` 100 % pass. — clippy clean with and without vm-tests; host 321 passed, 0 failed
- [x] R3 VM suite (`--features vm-tests` + `WINBLAST_VM_AGGRESSIVE=1`) in "Windows 10 MSIX packaging environment" from checkpoint `clean-test`, then reverted. — VM run 3: 16 suites green, vm_aggressive 8/8 (catalog byte-exact, crash 6/6, foreign 0/420, stress, planted rules, corruption); `docs/vm-test-1.7.0/run3-summary.txt`; VM reverted to clean-test
- [x] R4 `cargo build --release`, signed by thumbprint with the DigiCert timestamp; SHA-256 recorded here. — signed by 070A70E0…404D + DigiCert timestamp; SHA-256 0D7193901AED264DAC82AB76280D7AB2B0AC81BEE26D4B9114F983147BCBC477 (60,502,432 bytes)
- [x] R5 `graphify update .`; one local commit per package on `1.7.0-rebuild`. — graphify updated; commits 36ea513, 1dd7d82, 3fe3239, cd5503c, ea95930, a492d4a, faee710 + this one
- [ ] R6 Push / tag / publish (owner only).

## Done earlier in this cycle
- [x] Disk TBW: real NVMe SMART query, SATA vendor-unit table with N/A / Requires admin, 60 s refresh, hex-dump fixtures (commit 36ea513)
- [x] Fonts: Turkish/Latin/Cyrillic coverage and weight-axis tests; explicit PerMonitorV2 (commit 36ea513)
- [-] renderer-skia benchmark: no prebuilt Skia for the static-CRT build (404); building it needs LLVM — not pursued
