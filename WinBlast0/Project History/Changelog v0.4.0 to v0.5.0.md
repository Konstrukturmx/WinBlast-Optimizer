# Changelog: Version 0.4.0 to Version 0.5.0

## Release Summary
**WinBlast0 v0.5.0** is a major user experience, responsiveness, and catalog expansion release. It addresses complete internationalization coverage across all 8 languages, fixes UI alignment and responsive grid behaviors (specifically in the GPU section), introduces fluid spring and hover micro-animations, adds a live progress bar with a bottom changelog ledger drawer, and integrates proven optimizations from leading open-source tuning suites.

---

## Key Changes by Subsystem

### 1. Internationalization (i18n)
- **100% UI Localization Coverage**: Expanded `TextKey` from 54 to 120+ typed keys, covering all sidebar groups, headers, dashboard metrics, units, GPU adapter details, condition alerts, changelog actions, and risk badges.
- **Native Turkish (`Türkçe`) Translation Suite**: Comprehensive native Turkish translations for all UI strings, labels, and dialogs.
- **Full Coverage for 8 Languages**: English, Turkish, German, French, Spanish, Russian, Chinese (Simplified), and Japanese.

### 2. UI Layout & Responsiveness
- **GPU Section Responsive Grid**: Removed rigid fixed width (`370px`) in favor of dynamic `.grow(1.0).min_w(320.0)` cards that adapt cleanly to any viewport width.
- **Visual Symmetry**: Unified condition-failed inset boxes and description heights so incompatible and compatible cards maintain consistent alignment.
- **Hardware PnP ID Wrapping**: Added text wrapping to long PCIe PnP hardware identifiers.

### 3. Motion & Micro-Interactions
- Integrated `rune::tok::motion::SNAPPY` spring physics across all interactive cards, switches, and navigation tabs.
- Added card lift, glowing borders, and smooth color transitions on hover.
- Added animated switch knob translation with spring damping.

### 4. Live Progress Bar & Bottom Changelog Drawer
- **Animated Progress Bar**: Tracks scanning progress, batch mutation commits, and overall system optimization health.
- **Bottom Changelog Drawer**: Real-time expandable ledger drawer displaying the latest system mutations with 1-click single-tweak rollback triggers directly from the feed.

### 5. Catalog Expansion (Open-Source Tweaks)
- Integrated high-value tweaks from **Sophia Script**, **AtlasOS**, **Chris Titus WinUtil**, **ReviOS**, and **Optimizer**:
  - Disable Sticky Keys & accessibility hotkeys
  - Disable Game Bar Presence Writer
  - Disable High Precision Event Timer (HPET) synthetic interrupts
  - Disable Delivery Optimization P2P sharing
  - Optimize TCP/IP window auto-tuning and congestion parameters
  - Disable Windows Error Reporting (`WerSvc`)
  - Disable Bing Start Menu web search suggestions
  - Lock Kernel Paging Executive in physical RAM
- Eliminated duplicate IDs (`disable_fast_startup`, `disable_windows_recall`, `disable_activity_history`).
