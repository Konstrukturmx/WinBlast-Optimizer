# WinBlast0 v1.7.0 — Migration Checklist (updated 2026-11-23)

Branch: `1.7.0` (local only, never pushed). Reference: tag `v1.6.1` in `../WinBlast0-v161`.

## ✅ Done (committed)
### Foundation
- [x] Rune removed completely (deps, feature flags, vendor patches, `src/ui/`, stale doc comments, footer text)
- [x] Theme + i18n decoupled from Rune
- [x] Shell: top bar, sidebar + search, progress strip, changelog + undo, footer, dropdowns, HUD, diff modal
- [x] Live 8-language switching, 6 schemes × 5 styles × dark/light, zoom
- [x] Settings persisted (`%LOCALAPPDATA%\WinBlast0\settings.json`)
- [x] Native window frame + shortcuts (Esc, Ctrl+Tab, Ctrl+zoom, F5)
### Views (26 / 26)
- [x] Dashboard · Advanced Matrix (230 tweaks) · Clean & Apps · Startup/Tasks/Services
- [x] Backup Center · Privacy Shield · GPU Tuning · Hardware
- [x] CPU Parking · Thermal · Disk Health · Driver & BIOS · Latency
- [x] Goal Planner · Automation Center
- [x] Fault Diagnostics · Windows Repair · Performance Tests
- [x] Gaming hub (live monitor, FPS reference, profiles, overlay, suitability) + the 4 standalone gaming views
- [x] Game Prerequisites · Network Diagnostics
- [x] Support / About (1.7.0 notes, repo links, bug report, style studio; updater lives on the Dashboard)
### Cleanup & audits
- [x] Orphaned files removed (`ui/shell/titlebar.slint`, `ui/components/tweak_row.slint`, `ui/components/card.slint`)
- [x] i18n sweep: remaining English-only strings localized (Automation, Drivers, Gaming, Fault, Thermal, HUD, sidebar, search)
- [x] Parity audit: `PARITY_MATRIX.md` re-audited — 143 WIRED, 3 REPLACED by safer paths, 0 missing; all 150 store callbacks have handlers
- [x] Heuristic bug sweep: no `unwrap/expect` in production code, risky indexing guarded, no UI-thread blocking from callbacks; fabricated telemetry removed (disk health/temp, memory trim, RAM/core fallbacks, suitability default)
- [x] Host-touching tests gated to the VM: `cargo test --features vm-tests` (real_windows_env.rs, HKCU scratch tests, real AppX/winget guard checks)
### Bug fixes (33 — see `docs/v1.7.0-bugfixes.md`)
### Tests (host, sandboxed)
- [x] `cargo test --all-targets`: 269 passed, 0 failed, 6 ignored (VM-only)
- [x] `cargo clippy --all-targets -- -D warnings` clean (also with `--features vm-tests`)

## ⏳ Remaining
- [ ] Build Hyper-V Win10 VM (ISO at `D:\Hive\VMs\iso`) and run all suites there incl. `--features vm-tests` + screenshots
- [ ] `graphify update .`; CHANGELOG / README / release notes
- [ ] 1.7.0 release build
- [ ] New code-signing cert, remove 11 old duplicates, fix `sign_binary.ps1`, sign + DigiCert timestamp + verify
- [ ] Final local commit on `1.7.0` (no push)
