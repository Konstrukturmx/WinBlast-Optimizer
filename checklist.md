# WinBlast0 — Improvement checklist (v1.7.0, local branch `1.7.0-rebuild`)

Source: the 2026-11-24 VM analysis (`docs/IMPROVEMENT_OPTIONS.md`, findings F-1…F-23) and the
2026-11-25 page-by-page review (all 26 pages rendered offline with `render_all_views` in
`tests/ui_render_preview.rs`). The version stays **1.7.0**; each package is one local commit
(never pushed). An item is ticked only after it is implemented **and** verified (clippy
`-D warnings`, `cargo test --all-targets`, a re-render, and the VM for packages A and B).

Status key: `[ ]` open · `[x]` done and verified · `[~]` done, verification pending · `[-]` dropped (reason given)

---

## A. Rollback you can trust (F-3, F-4, F-5, F-6, F-11, F-13, F-15, F-16)
- [ ] A1 Write-ahead ledger: a `pending` row with the exact raw prior value (type + bytes) is written **before** each change and marked `committed` after; pending rows are reconciled against the live state at startup; `synchronous=FULL`, `PRAGMA user_version` schema version, index on the history table.
- [ ] A2 Raw-typed registry snapshots: every registry action records type + bytes (REG_SZ, EXPAND_SZ, MULTI_SZ, BINARY, DWORD, QWORD) and rollback restores exactly that; "key existed" is recorded and rollback deletes only keys WinBlast created (F-13).
- [ ] A3 Service actions in two recorded steps (start type, then stop/start); a failed stop no longer aborts after the start type changed (DiagTrack 1052, SysMain 1061).
- [ ] A4 "Undo" restores the recorded prior state. "Reset to Windows default" is a separate, clearly named action.
- [ ] A5 Actions already at their target value are skipped and record nothing, so re-applying cannot make the optimized value the "original" (F-15).
- [ ] A6 One mutation worker (or per-value locks): Instant-mode toggles, batch apply/rollback and Auto-Tune cannot interleave on the same value (F-14).
- [ ] A7 Ledger open/read failures are shown to the user ("Undo history unavailable") instead of being swallowed with `.ok()` (F-11).
- [ ] A8 Multi-adapter / multi-GPU values are recorded per concrete key path and rolled back by that path, not by re-resolving the current adapters (F-16).
- [ ] A9 VM proof: `tests/vm_aggressive.rs` scenarios A (byte-exact round trip), B (foreign prior types), D (crash mid-apply) and F (concurrency) pass.

## B. Security (F-1, F-2, F-10, F-20, F-22, F-23)
- [ ] B1 Release builds load **only** the embedded tweak catalog; the on-disk rules loader is debug/dev-only. VM test G (planted rules file) passes.
- [ ] B2 Updater: SHA-256 + ed25519-signed release manifest checked before the swap; Authenticode signer checked; `.exe` assets only; HTTPS-only with a host allow-list and no protocol downgrade on redirects; fallback repository removed; staging in an admin-only folder; asks before restarting.
- [ ] B3 The weekly SYSTEM `del /q /f /s "%TEMP%\*"` task is replaced by the built-in cleaner, which does not follow junctions (F-10).
- [ ] B4 Every external program starts by full System32 path (powershell, schtasks, sc, reg, dism, pnputil, winget resolved safely); Edge removal runs only a Microsoft-signed `setup.exe` under Program Files (F-20).
- [ ] B5 The hosts file is written to a temp file and swapped in atomically (ReplaceFileW); rolling back one hosts entry removes only that entry (F-23).
- [ ] B6 "Fault fixes" no longer delete RebootPending / RebootRequired / PendingFileRenameOperations silently: recorded in the ledger and shown as expert-only with a warning (F-22).
- [ ] B7 Web links open from a non-elevated context (the browser does not run as admin).
- [ ] B8 Version comparison treats pre-releases (`1.7.0-rc.1`) as older than the release.

## C. Honest data — AGENTS.md rule 2 (F-7, F-8, F-18, F-19 + new)
- [ ] C1 Thermals: no derived readings (CPU max core = ACPI + 4 °C, GPU hotspot = core + offset, RPM = % × 2400, ambient); missing values show "—" with "needs vendor sensor". Throttling detection reads bit 0.
- [ ] C2 Performance tests measure RAM latency and disk read for real; no fixed values, no clamped score; the verdict sentence appears only after a run.
- [ ] C3 Driver page: no fixed per-category sizes; driver install, Winsock reset and DNS apply report the real exit status.
- [ ] C4 DNS apply picks the adapter by route metric (`Get-NetIPInterface`) and uses `Set-DnsClientServerAddress -InterfaceIndex`; the result shown is the real one (F-8).
- [ ] C5 Windows Repair: exit codes checked, `sfc` UTF-16 output decoded, the task list is visible before running.
- [ ] C6 Language-independent parsing: drivers via CIM / `pnputil /format xml`, thermal counters via PDH indexes, task scanner without English word lists (F-19).
- [ ] C7 Restore-point age check uses UTC consistently (F-18).
- [ ] C8 No claims before data: Dashboard health (no red "0 %"), Secure Boot warning, Fault Diagnostics "System Stable / No pending reboot / No BSOD", CPU Parking badge, Network "Default", empty Disk Health banner — all show "Not scanned yet" until a scan returns.
- [ ] C9 The FPS "lab" reference figures (stock vs optimized FPS) are removed; FPS content moves into the Gaming page and shows only what this PC measured.
- [ ] C10 Game-profile descriptions describe what the IFEO settings actually do (CPU / I/O priority, FSO flag); no "locks GPU clocks" / "Vanguard scheduling" claims; the empty state no longer promises an "add" feature that does not exist.
- [ ] C11 Latency: "DPC latency" relabelled to what is measured (user-mode timer jitter).

## D. Robustness and responsiveness (F-9, F-12, F-14, F-21)
- [ ] D1 One `run_with_timeout` helper for every external process, killing the process on timeout with a clear error (F-9).
- [ ] D2 A crashed monitor thread or worker is reported in the UI; update threads use `spawn_guarded_worker`.
- [ ] D3 Ledger reads run off the UI thread, over one connection, with paged history (F-21).
- [ ] D4 Disk / thermal probes run only while their page is open, with a cache (F-21).
- [ ] D5 Access denied during a scan reads as "Unknown — needs admin", never as "not set" (F-12).
- [ ] D6 Single-instance guard.
- [ ] D7 Startup detection (OS, CPU, GPU, SQLite, core parking) moves off the UI thread so the window appears first.

## E. Navigation: 26 pages → 16
Target sidebar:
- Overview: **Dashboard** · **Hardware** (+ CPU Parking tab) · **Drivers & BIOS**
- Diagnostics: **Benchmarks** (Performance + Latency) · **Sensors** (Thermals + Disk Health) · **Network** · **Repair** (Fault Diagnostics + Windows Repair)
- Optimization: **Tweaks** (Advanced Matrix) · **Presets & Automation** (Automation + Goal Planner + all preset buttons) · **Gaming** (Monitor, FPS, Profiles, Overlay, Suitability, Prerequisites) · **GPU** · **Privacy** · **Clean & Apps** · **Startup & Services** · **Backup & Undo** (+ Windows Setup file tab)
- Account: **Settings** (new) · **About**

- [ ] E1 Gaming page with tabs replaces In-Game Monitor, FPS Benchmark, Game Profiles, Game Overlay, Game Suitability and Game Prerequisites.
- [ ] E2 One preset system: Dashboard quick presets, Matrix presets, Automation profiles and Goal Planner goals become one "Presets & Automation" page; other pages link to it.
- [ ] E3 Benchmarks, Sensors and Repair pages merge their pairs; CPU Parking becomes a Hardware tab.
- [ ] E4 Settings page: language, theme scheme/style/mode, zoom, Reduced motion, depth effects, apply mode (Instant/Normal) — moved out of About and the top bar overflow.
- [ ] E5 The sidebar no longer shows a doubled icon (the 🤖 is removed from the "Automation Center" label in all 8 languages; a test checks labels contain no leading emoji).
- [ ] E6 The autounattend generator becomes the "Windows Setup file" tab of Backup & Undo.
- [ ] E7 Old view ids still resolve (saved settings, Ctrl+K, deep links) to the new page and tab; `every_view_is_routed_listed_and_searchable` and the layout-collision test cover the new set.

## F. Shared UI rules and per-page fixes
- [ ] F1 One "Not scanned yet" empty-state component, used by every page instead of blank cards or default values.
- [ ] F2 At most one primary button per page; actions that change the system are secondary and confirm first (Hibernation, USB power, Ultimate Plan, Clear Minidumps, Run TRIM, Disable MPO, Restart Explorer, Run All repairs).
- [ ] F3 Wording pass: no engineering jargon (A*, Dijkstra, Pareto, Raven); correct search placeholders (Prerequisites, Startup); one positive label per switch (Game Overlay triple negative); "Clean Selected (n)" never shows "()"; no empty labels ("Ledger DB:", "Arch:", update card, Matrix and Startup blank chips); clear Privacy empty state.
- [ ] F4 Startup header: no stretched gaps, normal-height search box.
- [ ] F5 The empty horizontal scrollbar strip above the status bar is gone; the status bar shows real state (version, admin, pending changes) instead of marketing text.
- [ ] F6 Hardware page: read-only cards and actions are separated, actions are clearly labelled with their current state.
- [ ] F7 Dashboard quick-action cards sit in an even grid.
- [ ] F8 GPU page: no disabled "— N/A" primary button, no stray "×", the empty Vendor section is hidden or explained.
- [ ] F9 Unlabelled switches (HAGS on Gaming, DNS benchmark on Network) get labels.
- [ ] F10 "Report Fault" uses a neutral/secondary style, not the danger colour.
- [ ] F11 Matrix: one action toolbar (no duplicate "Apply Selected"), honest revert wording (see A4).

## G. Tests
- [ ] G1 `tests/vm_aggressive.rs` committed and part of the pre-release VM run.
- [ ] G2 The sandbox lifecycle test compares exact values (`SandboxState: PartialEq`), not scan state.
- [ ] G3 `fuzz_mutation_properties` honours `PROPTEST_CASES`, counts the tweaks it exercised, and seeds absent / string / 0 / `u32::MAX` priors.
- [ ] G4 "Skipping…" fails under `vm-tests`; tests that cannot fail are fixed or deleted; the test-only `pnputil /scan-devices` helper leaves shipped code.
- [ ] G5 `render_all_views` renders the new page set; a check fails on empty labels in the rendered tree.
- [ ] G6 New VM scenarios: tampered/zip/redirect updater assets from a local mirror; standard user; adapter switch between apply and rollback; UTC+10 / UTC−8; two instances; 100k-row ledger with a UI tick-time check.
- [-] G7 German/Russian Windows image and a Windows 11 VM — not available on this host; C6 is verified by unit tests on captured output instead.

## Release (still v1.7.0, local only)
- [ ] R1 CHANGELOG, README and About "What's new" updated in 8 languages.
- [ ] R2 `cargo clippy --all-targets -- -D warnings` clean (also with `--features vm-tests`); `cargo test --all-targets` 100 % pass.
- [ ] R3 VM suite (`--features vm-tests` + `WINBLAST_VM_AGGRESSIVE=1`) in "Windows 10 MSIX packaging environment" from checkpoint `clean-test`, then reverted.
- [ ] R4 `cargo build --release`, signed by thumbprint with the DigiCert timestamp; SHA-256 recorded here.
- [ ] R5 `graphify update .`; one local commit per package on `1.7.0-rebuild`.
- [ ] R6 Push / tag / publish (owner only).

## Done earlier in this cycle
- [x] Disk TBW: real NVMe SMART query, SATA vendor-unit table with N/A / Requires admin, 60 s refresh, hex-dump fixtures (commit 36ea513)
- [x] Fonts: Turkish/Latin/Cyrillic coverage and weight-axis tests; explicit PerMonitorV2 (commit 36ea513)
- [-] renderer-skia benchmark: no prebuilt Skia for the static-CRT build (404); building it needs LLVM — not pursued
