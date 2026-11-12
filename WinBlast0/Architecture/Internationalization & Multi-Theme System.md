---
title: Internationalization & Multi-Theme System
tags: [architecture, ui, i18n, localization, themes, wcag]
created: 2026-11-08
---

# 🌐 Internationalization & Dynamic Multi-Theming Architecture

WinBlast0 v0.3.0 implements complete multi-language localization and a WCAG 2.1 AA-audited dynamic theming engine.

---

## 🏛️ Zero-Allocation Internationalization (`src/core/i18n.rs`)

### 1. Static Table Indexing
To prevent heap allocation storms during Rune 60 FPS reactive render passes, all translated strings reside in `.rdata` and are resolved via direct $O(1)$ enum indexing:
```rust
I18n::get(s.language, TextKey::AppTitle) -> &'static str
```

### 2. Supported Languages (8 Total):
1. **English** (`en`)
2. **Turkish** (`tr`) — Full native translation coverage
3. **German** (`de`)
4. **French** (`fr`)
5. **Spanish** (`es`)
6. **Russian** (`ru`)
7. **Chinese** (`zh`)
8. **Japanese** (`ja`)

Languages can be cycled instantly from the top header bar without requiring application restarts.

---

## 🎨 Dynamic Theming & WCAG AA Compliance (`src/ui/theme.rs`)

### 1. Light vs. Dark Baseline
- **Dark Mode**: Deep `#080B14` canvas, high-contrast cards, luminescent neon accents.
- **Light Mode**: Clean Slate 50 (`#F8FAFC`), pure white cards (`#FFFFFF`), deepened high-chroma accents ensuring minimum 4.5:1 (WCAG AA) and 7:1 (WCAG AAA) contrast ratios.

### 2. Four Color Schemes:
1. **Cyber Indigo**: High-tech electric blue / royal navy.
2. **Midnight Stealth**: Monochromatic slate / tactical grey.
3. **Emerald Mint**: Vibrant emerald green / forest spruce.
4. **Crimson Sunset**: High-energy rose red / deep burgundy.

---

## 🔗 Related Notes
- [[Pure-Rust Rune UI Engine]]
- [[Changelog v0.2.0 to v0.3.0]]
- [[00 - Index (MOC)]]
