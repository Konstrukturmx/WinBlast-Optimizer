---
title: OS Settings & Windows 11 AI Tweaks
tags: [tweaks, os, windows11, ai, copilot, recall, visual]
created: 2026-11-08
---

# 🤖 OS Settings & Windows 11 AI Tweaks

This catalog (`os_settings.toml`) eliminates modern Windows 11 bloatware features including artificial intelligence screen scrapers, Bing cloud queries in local search, taskbar Widgets, and compositing overhead from animations and transparency.

---

## 📋 Complete Tweak Matrix

### 1. `disable_bing_search_start` — Stop Keystrokes to Bing in Start Menu
- **What it does**: By default, typing in the Windows Start menu sends every single character to Microsoft Bing servers over HTTPS. This tweak stops web search integration, making local file and application search instant.
- **Targets**:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer\DisableSearchBoxSuggestions` = `1`
  - `HKCU\Software\Microsoft\Windows\CurrentVersion\Search\BingSearchEnabled` = `0`
- **Gain**: Perf +3, Priv +4, Stability: Safe.
- **Broken Feature**: Web links will not appear when typing in Start.

### 2. `disable_windows_copilot` — Disable Windows Copilot AI Integration
- **What it does**: Disables the Windows Copilot sidepanel, resident background services, and the `Win+C` shortcut across all user accounts via Group Policy.
- **Targets**:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot\TurnOffWindowsCopilot` = `1`
  - `HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot\TurnOffWindowsCopilot` = `1`
- **Gain**: Perf +3, Priv +4, Stability: Safe.
- **OS Requirement**: Windows 11 Build 22621+.

### 3. `disable_windows_recall` — Disable Windows Recall AI Screen Analysis
- **What it does**: Disables Windows Recall and snapshot analysis (`DisableAIDataAnalysis`), preventing the OS from silently recording continuous screen captures of user activity.
- **Target**: `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI\DisableAIDataAnalysis` = `1`
- **Gain**: Perf +3, Priv +5, Stability: Safe.
- **OS Requirement**: Windows 11 Build 26100+ (24H2).

### 4. `disable_widgets_news` — Disable Taskbar Widgets & MSN News Feed
- **What it does**: Completely disables the taskbar Widgets icon and stops background polling of trending MSN news stories, weather, and financial tickers.
- **Target**: `HKLM\SOFTWARE\Policies\Microsoft\Dsh\AllowNewsAndInterests` = `0`
- **Gain**: Perf +3, Priv +3, Stability: Safe.
- **Broken Feature**: Taskbar Widgets flyout disabled.

### 5. `disable_animations` — Disable Window Minimize/Maximize Animations
- **What it does**: Disables the animate-windows-when-minimizing-and-maximizing visual effect, eliminating perceived UI sluggishness.
- **Targets**:
  - `HKCU\Control Panel\Desktop\WindowMetrics\MinAnimate` = `0`
  - `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarAnimations` = `0`
- **Gain**: Perf +2, Stability: Safe.

### 6. `disable_transparency` — Disable Acrylic & Transparency Effects
- **What it does**: Turns off DWM acrylic transparency on the taskbar, Start menu, and Action Center, reducing GPU compositing overhead.
- **Target**: `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize\EnableTransparency` = `0`
- **Gain**: Perf +2, Stability: Safe.

### 7. `high_performance_power_plan` — Set High Performance Power Plan Overlay
- **What it does**: Sets the active power overlay to High Performance via `PerfBoostMode`, ensuring CPU clock speeds remain elevated without aggressive power throttling.
- **Target**: `HKLM\SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7\ValueMax` = `3`
- **Gain**: Perf +3, Stability: Low risk.

---

## 🔗 Related Notes
- [[Privacy & Telemetry Shield]]
- [[Registry Lag & Kernel Scheduling]]
- [[00 - Index (MOC)]]
