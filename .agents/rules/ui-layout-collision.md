---
name: ui-layout-collision
description: Enforces responsive layout boundaries, prevents UI element collision, and mandates correct Rune scrollbar wrapper ordering.
always_on: true
---

# UI Layout Collision & Responsive Design Rules

This rule defines mandatory architecture and styling invariants for all UI components, views, and containers in WinBlast0 to prevent element collision, runaway layout growth, card overlaps, and viewport clobbering.

## 1. Rune Scroll Wrapper Attachment Ordering (CRITICAL)

The `WinBlastScrollable::winblast_scroll(p)` trait extension wraps the calling Rune element into a compound scroll area containing the inner scrollable viewport and the themed scrollbar rail.

- **Mandatory Order:** Always attach view children to the container **BEFORE** calling `.winblast_scroll(p)`.
  ```rust
  // CORRECT: Child is attached to the scrollable viewport
  col()
      .key("driver_updates_scroll")
      .w_full()
      .min_w(0.0)
      .h(list_height)
      .scroll(ScrollSpec { stick_bottom: false })
      .child(driver_list)
      .winblast_scroll(p)

  // FORBIDDEN: Child attached after winblast_scroll() is rendered as an
  // unconstrained sibling of the viewport, completely bypassing scrolling
  // and colliding with subsequent sibling cards below it!
  col()
      .scroll(ScrollSpec { stick_bottom: false })
      .winblast_scroll(p)
      .child(driver_list) // BUG: Overlaps Motherboard & BIOS cards!
  ```

## 2. Bounded Nested Containers

When embedding a scrollable list inside a card or composite view:
- The scroll container must define an explicit bounded height: `.h(list_height)` or `.grow(1.0).min_h(0.0)`.
- Always set `.min_w(0.0)` and `.w_full()` on flex column and row children to allow flex items to shrink when window width narrows.
- Always apply `.clip_soft(0.0)` or `.overflow_hidden()` when content could exceed the card's boundary.

## 3. Responsive Multi-Column Layout & Card Collision Prevention

When arranging cards in a row or grid:
- Use `.flex_wrap()` on the container row.
- Each child card column must define `.grow(1.0)` and a reasonable `.min_w(...)` (e.g. 260.0 to 320.0).
- Use `.gaps(x, y)` for consistent inter-card gutters without manual margin offsets.
- On narrow viewports (`LayoutTier::Compact`), cards must gracefully reflow into a single column without horizontal blowout or overlapping text.

## 4. Header Bar & Navigation Fixed Geometry

- Header and status bars must remain strictly pinned to their allocated height (e.g., `.min_h(44.0)`, `.max_h(44.0)`).
- Never allow dropdown buttons, metrics sparklines, or badges in the header to wrap or push the content area downward.
- Popover menus (Language, Theme, Presets) must always be anchored via `.anchored_fit(...)` and use `.modal(true, ...)` for backdrop and outside-click dismissal.

## 5. Automated Verification Checklist

Before approving any UI view modifications:
1. Grep for `\.winblast_scroll\(.*?\)\.child` across all `.rs` files. Any match is a layout collision bug.
2. Verify that high element count lists (e.g., 50+ drivers, 100+ services, 20+ disks) stay strictly within their scrollable containers and do not push neighboring cards out of alignment.
3. Test layout under Compact (1024x700), Normal (1400x900), and Wide (1920x1080) screen resolutions.
