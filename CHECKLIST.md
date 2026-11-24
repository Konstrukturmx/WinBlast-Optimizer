# WinBlast0 — Release Checklist

## v1.7.0 (rebuild, 2026-11-24, local branch `1.7.0-rebuild` from `1.8.0`, not pushed)
Ships the 1.8.0 visual layer and the Slint migration under version 1.7.0, plus:
- [x] Loading screen: real Close button (was click-blocked text), dismissal resets for new operations; redesigned animation
- [x] Every page reachable: 26 views routed + in sidebar + in Ctrl+K (test `every_view_is_routed_listed_and_searchable`)
- [x] Themes: 4 OKLCH schemes × 5 styles × dark/light; WCAG AA unit test over all 40 combinations; removed schemes migrate
- [x] Hover feedback on buttons and list rows; GPU-only 3D depth layer (DXGI probe, off on WARP/Basic Render/Hyper-V)
- [x] Responsive: layout collision sweep (26 views × 5 widths, DE/RU at 800 px, top bar 8 languages × 10 widths) — 0 collisions
- [x] Heuristic sweep: UTF-8 slicing (3 real panics fixed), discarded Results in mutating code (2 now logged), raw Command spawns (elevation via ShellExecute)
- [x] Glyphs: 17 symbols that render blank replaced; `tests/glyph_coverage.rs` guards it
- [x] Health caption shows real counts instead of a fixed "Optimized"
- [x] Static CRT: exe no longer needs the VC++ Redistributable (imports verified)
- [x] Host: 287 passed, 0 failed, 8 ignored; clippy `--all-targets -D warnings` clean
- [x] VM (Hyper-V "Windows 10 MSIX packaging environment", Win10 Ent 19044, no VC++ runtime, Basic Render adapter): `--features vm-tests` suite 293 passed, 0 failed; signed release launched on the desktop, Close-during-scan verified with synthetic mouse clicks (`docs/vm-test-1.7.0/`); VM rolled back to checkpoint `clean`
- [x] Release built and signed: SHA-256 A7BE2879026A9D82B664FC0B19EA5501594C1F35678AD1FB6BCD8E662247AA80, signer CN=Konstrukturmx (070A70E0…404D, self-signed, untrusted root as expected), DigiCert RFC 3161 timestamp, FileVersion 1.7.0.0
- [ ] Push / tag / publish (owner)

## Visual layer (built 2026-11-23 as the unreleased 1.8.0; superseded by the 1.7.0 rebuild above)
- [x] Embedded fonts: Inter (UI) + JetBrains Mono (mono), subset, OFL licences in `ui/fonts/`
- [x] Motion: view transitions, staggered card entrance, gliding sidebar pill, hover lift
- [x] Visual FX: aurora backdrop (static at idle), glow accents, live radial gauges, shimmer loaders
- [x] UX: Ctrl+K command palette, toasts, risk-badge tooltips, Reduced motion switch (persisted)
- [x] i18n: 31 new keys × 8 languages (palette, motion, tooltips, toasts + 17 previously English-only labels); What's New updated to 1.8.0
- [x] Heuristic sweep: strict clippy lints (unwrap/expect/panic/indexing/slicing/truncation) on production code — no unguarded panics found; glyphs missing from the fonts replaced
- [x] Host tests: 273 passed, 0 failed, 7 ignored; clippy `-D warnings` clean
- [x] Offline render check (`tests/ui_render_preview.rs`): dashboard, toasts, palette, shimmer, About, light theme, compact width
- [ ] VM suite with `--features vm-tests` (needs the test VM login; no backend code changed in 1.8.0)
- [ ] Push / tag / publish (owner)

# v1.7.0 — Migration Checklist (v1.7.0 complete, local only)

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
