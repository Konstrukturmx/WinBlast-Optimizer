---
title: Responsive UI, Animations & Changelog Ledger
tags: [architecture, ui, responsive, rune, animations, changelog]
created: 2026-11-09
updated: 2026-11-10
version: v0.7.0
---

# Responsive UI, Animations & Changelog Ledger (v0.7.0)

## 1. Multi-Resolution Architecture (Zero-Echo Tiering + Pure Wrapping Flex)

WinBlast0 **v0.7.0** adapts seamlessly across all **10 views** and **6 sub-tabs** from compact portrait windows (`420x400`, `min_size`) through standard desktop displays (`800x600`, `1024x768`, `1280x720`, `1366x768`, `1920x1080`) up to 4K & Ultrawide monitors (`2560x1440`, `3440x1440`, `3840x2160`) across `1.0x`, `1.25x`, and `1.5x` DPI scales.

### Root Breakpoint Classifier (`ui/mod.rs`)
- Places `.key("winblast_root")` on the unconditional outer `col().w_full().h_full()`.
- Evaluates a single zero-sized function item `cx.layout_with("winblast_root", tier_from_rect)` (0 heap allocations, 0 layout echoes):
  - `Compact` (`w < 680pt`): Hides desktop header noise chips/subtitle, activates collapsible full-height navigation drawer (`☰` toggle `.key("nav_menu_toggle")`) with primary tabs pinned at top of drawer and Mode/Engine/Scheme pills tucked cleanly beneath nav items, stacks `kv_row` vertically.
  - `Medium` (`680 <= w < 1020pt`): Collapses persistent sidebar into `"☰"` modal drawer so content enjoys full width.
  - `Wide` (`w >= 1020pt`): Persistent `224px` scrollable sidebar (`main_body.min_h(120.0)` prevents `nav_sidebar_scroll` collapsing during resize) + centered `max_w(1680.0)` reading container on 4K Ultrawide.

### Crucial Rune/Taffy Flex Rules Enforced
1. **Default `flex_shrink: 0.0` Guard**: Nested `row().flex_wrap()` button groups, segmented `mode_tab` bars, and `controls_group` ALWAYS carry `.min_w(0.0).shrink()` so Taffy shrinks wrapped multi-line flex groups instead of locking to unwrapped `flex_basis: auto`.
2. **Bounded Scroll Containers & Virtualized Lists (`v0.7.0`)**: Standard scrollable views use outer `.grow(1.0).min_w(0.0).min_h(0.0).scroll(...)`. `CleanAndApps` and `StartupAndTasks` use `El::virtual_rows` directly under `.scroll(...)` with responsive row heights (`104–140pt` Compact, `72–76pt` Desktop) and `.truncate()` secondary lines, switching to a unified outer scroll wrapper on `Compact` (`420x400`).
3. **Window-Level Diff Preview Modal**: Rendered on `winblast_root` inside `.pin(0,0,0,0).modal(true, ...)` with definite `.h_full().max_h(420.0)` bounds so `diff_modal` scroll viewports never collapse to `0 pt` height and always render a Close button.
4. **Unbroken Identifiers Wrapping**: AppX family IDs, registry keys, and PCIe PnP IDs use `.shrink().wrap().wrap_anywhere()` (or `.truncate()` inside fixed-stride virtual rows).

---

## 2. Shared WCAG-Compliant Widgets (`src/ui/widgets/button.rs` & `theme.rs`)
- `action_button`: Keeps semantic filled buttons saturated (`bg.alpha(0.84)`) on hover in Light Mode, uses accessible warning-tinted styling (`p.warning.alpha(0.16)` + `p.warning`), accessible Dark `CrimsonSunset` (`rgb(0.82, 0.14, 0.27)`), and darkened Light `text_muted` (`rgb(0.31, 0.37, 0.46)`).
- `switch_toggle`, `switch_toggle_with`, `switch_indicator`: `switch_toggle` handles catalog tweaks (`handle_switch_click`), `switch_toggle_with` binds custom item closures (e.g. startup entries), and `switch_indicator` provides pure visual switches inside interactive cards so row clicks never pollute `selected_tweaks`.
- `kv_row`, `section_card`, `pill_badge`, & `sparkline_with_palette`: Responsive stacking, universal wrapping, and flexible waveform bars.

---

## 3. Progress Bar & Bottom Changelog Drawer
- Live fractional spring fill (`.w_frac(...)` + `rune::tok::motion::SNAPPY`), excluding `NotApplicable` tweaks from the total denominator.
- Bounded drawer (`110pt` on Compact, `150pt` on Medium/Wide) iterating `.iter().take(12)` on newest-first (`ORDER BY id DESC`) history, displaying localized `AppliedBadge`, `RolledBackBadge`, `NonReversibleBadge`, and 1-click Undo rollback controls.
