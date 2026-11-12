---
title: Changelog — v0.2.0 to v0.3.0
tags: [changelog, releases, v030, history]
created: 2026-11-08
---

# 📜 Changelog — WinBlast0 v0.3.0 Release

---

## 🚀 Version 0.3.0 (Major Feature Release) — 2026-11-08

### ✨ New Features & Capabilities:
- **Comprehensive Internationalization (i18n)**:
  - Added support for 8 major languages: **English**, **Turkish (Türkçe)**, **German (Deutsch)**, **French (Français)**, **Spanish (Español)**, **Russian (Русский)**, **Chinese (简体中文)**, and **Japanese (日本語)**.
  - Zero-allocation static lookup architecture (`I18n::get`) preventing UI frame drops.
  - Reactive one-click language switcher in the header.
- **Dynamic Theming System (Dark & Light Modes + 4 Color Schemes)**:
  - Added full Light Mode and Dark Mode support.
  - Added 4 custom color schemes: **Cyber Indigo**, **Midnight Stealth**, **Emerald Mint**, and **Crimson Sunset**.
  - All color pairings audited for WCAG 2.1 AA contrast compliance (>4.5:1 text readability).
- **Subprocess Window Stealth (`SilentCommand`)**:
  - Replaced all raw subprocess calls across the entire codebase with `SilentCommand`.
  - Enforced `CREATE_NO_WINDOW` (`0x08000000`) on all Windows child processes (PowerShell, schtasks, sc, powercfg), eliminating command prompt / terminal flashing completely.
- **System & Network Latency Benchmark Suite**:
  - Added dedicated Latency Benchmark tab.
  - High-precision QPC micro-jitter probe measuring DPC/ISR interrupt latency.
  - Multi-target anycast DNS network probe (Cloudflare `1.1.1.1`, Google `8.8.8.8`, Quad9 `9.9.9.9`) measuring min/avg/max RTT and RFC 3550 jitter.
  - Before vs. After comparison card with delta calculation.
- **Dedicated CPU Core Parking Management Tab**:
  - Real-time logical core topology visualizer (Core #0..#N) showing 🟢 Active vs 🟡 Parked states.
  - 1-Click "Unpark All Cores" (sets CPMINCORES / CPMAXCORES to 100%, unhides Power Options CPL controls, commits active scheme).
  - 1-Click "Restore Stock Parking" to return to default battery/AC thresholds.
  - Intel Hybrid Architecture detection (P-Core vs E-Core labeling).

---

## 🔗 Related Notes
- [[CPU Core Parking & Dynamic Scheduling]]
- [[System & Network Latency Benchmarking]]
- [[Internationalization & Multi-Theme System]]
- [[00 - Index (MOC)]]
