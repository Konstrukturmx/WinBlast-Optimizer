# WinBlast0 v1.7.0 — Migration Checklist (updated 2026-11-22)

Branch: `1.7.0` (local only, never pushed). Reference: tag `v1.6.1` in `../WinBlast0-v161`.

## ✅ Done (committed)
### Foundation
- [x] Rune removed completely (deps, feature flags, vendor patches, `src/ui/`)
- [x] Theme + i18n decoupled from Rune
- [x] Shell: top bar, sidebar + search, progress strip, changelog + undo, footer, dropdowns, HUD, diff modal
- [x] Live 8-language switching, 6 schemes × 5 styles × dark/light, zoom
- [x] Settings persisted (`%LOCALAPPDATA%\WinBlast0\settings.json`)
- [x] Native window frame + shortcuts (Esc, Ctrl+Tab, Ctrl+zoom, F5)
### Views (19 / 26)
- [x] Dashboard · Advanced Matrix (230 tweaks) · Clean & Apps · Startup/Tasks/Services
- [x] Backup Center · Privacy Shield · GPU Tuning · Hardware
- [x] CPU Parking · Thermal · Disk Health · Driver & BIOS · Latency
- [x] Goal Planner · Automation Center
- [x] Fault Diagnostics · Windows Repair · Performance Tests
### Bug fixes (22 so far — see `docs/v1.7.0-bugfixes.md`)
- [x] Startup view stuck busy after batch toggles
- [x] UI freezes → background workers (drivers, SFC/DISM, TRIM, memory trim, health, maintenance)
- [x] Fake data removed (hardware names, sensors, benchmark scores, DirectStorage claim, update size)
- [x] Search vs Select-All mismatch; risk filter; Export Selected
- [x] Fault scan ran every frame; Restart now needs confirmation
- [x] All mutating actions go through the backend (sandbox never touches host)
### Tests
- [x] 14 shell tests + 37 view tests passing (sandboxed); clippy clean at last check

## 🔄 In progress (WIP files exist, not yet registered/compiled)
- [ ] Gaming hub (live monitor, FPS reference, profiles, overlay, suitability) — `src/ui_slint/views/gaming.rs`, `ui/stores/gaming.slint` written; Slint views + registration + tests pending
- [ ] Game Prerequisites view
- [ ] Network Diagnostics view
- [ ] Their fixes (backend/AppState already done): Game Bar toggle did nothing; DNS actions always claimed success; fake TCP/MTU/frame values

## ⏳ Not started
- [ ] Support/About view (updater, changelog) — last view
- [ ] Remove leftover old files (`ui/shell/titlebar.slint`, `ui/components/tweak_row.slint`, …)
- [ ] i18n sweep: remaining English-only strings (Automation, Drivers, Gaming)
- [ ] Final parity audit: tick every v1.6.1 action in `PARITY_MATRIX.md`
- [ ] Heuristic bug sweep (unwrap/expect/indexing, UI-thread blocking, missing timeouts)
- [ ] Gate host-mutating tests (`real_windows_env.rs`, parts of `tweak_sandbox_e2e.rs`) to VM only
- [ ] Build Hyper-V Win10 VM (ISO at `D:\Hive\VMs\iso`) and run all suites there + screenshots
- [ ] `cargo check` / `clippy -D warnings` all targets; 100% tests
- [ ] `graphify update .`; CHANGELOG / README / release notes
- [ ] 1.7.0 release build
- [ ] New code-signing cert, remove 11 old duplicates, fix `sign_binary.ps1`, sign + DigiCert timestamp + verify
- [ ] Final local commit on `1.7.0` (no push)

## Resume point
Next: create `ui/views/gaming_stats.slint` (+ fps/profiles/overlay/suitability sub-views), `game_prereqs.slint`, `network_diag.slint` using `GamingStore` / `PrereqStore` / `NetworkStore`; register `pub mod gaming;` in `src/ui_slint/views/mod.rs` (sync_gaming for GamingStats/FpsBenchmark/GameProfiles/GameOverlay/GameSuitability, sync_prereqs, sync_network); add exports via main.slint; add tests; commit.
