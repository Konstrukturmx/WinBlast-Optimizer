# WinBlast0 v1.7.0 — Migration Checklist (updated 2026-11-23 — v1.7.0 complete, local only)

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

### VM, release, signing
- [x] Hyper-V test VM `WinBlast-Test` (Win10 Enterprise Eval 22H2, build 19045) at `D:\Hive\VMs\WinBlast-Test`, checkpoint `clean-install`; driven via PowerShell Direct (pwsh 7 — Windows PowerShell 5.1 hangs)
- [x] All suites in the VM incl. `--features vm-tests`: 274 passed, 0 failed (after fixing 2 Win11-only assumptions in real_windows_env.rs)
- [x] Release build running in the VM; screenshots of all 26 views in `D:\Hive\VMs\WinBlast-Test\screenshots-1.7.0`; real ping probe verified
- [x] UI fixes found in the VM: top-bar overlap at ~1024 px, DNS count, emoji on Windows 10; Slint licence attribution (AboutSlint) added
- [x] CHANGELOG v1.7.0 entry rewritten (old one claimed a Rune dual-build and a custom title bar); README updated
- [x] `graphify update .` (4630 nodes, 9014 edges)
- [x] `cargo build --release`; signed with the single new cert `070A70E017B5BB8F7A345688C82D8A578DAC404D` (valid to 2031-11-23), DigiCert RFC 3161 timestamp; 16 old duplicate certs removed from CurrentUser\My
- [x] `scripts/sign_binary.ps1` rewritten: signs by thumbprint from the store, no cert generation, no password
- [x] Final local commit on `1.7.0` (not pushed)

## Open items for the owner
- Push / tag / publish the release (your call — nothing was pushed).
- The self-signed signature shows as "untrusted" on other PCs unless the cert is trusted there; a commercial cert would fix that.
- `scripts/Konstrukturmx_CodeSigning.pfx` (git-ignored) belongs to a deleted old cert and its password was in the old script — delete it if you don't need it.
- `FpsReferenceNotice` says figures come "from WinBlast lab test systems"; I could not verify that source.
