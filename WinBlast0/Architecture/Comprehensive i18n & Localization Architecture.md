# Comprehensive i18n & Localization Architecture (v0.7.0)

## 1. Executive Summary
WinBlast0 **v0.7.0** maintains **100% UI string localization** via `313` compile-time `TextKey` entries × `8` languages (`TRANSLATIONS: [[&str; TextKey::COUNT]; 8]`), plus locale-aware Turkish/ASCII search folding (`I18n::fold_search_str`).

---

## 2. Supported Languages & Native Scripts

| Language Code | Language | Native Label | Script / Encoding | Coverage Target |
|:---:|:---:|:---:|:---:|:---:|
| `en` | English | English | Latin | 100% |
| `tr` | Turkish | Türkçe | Latin (Extended) | 100% (Priority) |
| `de` | German | Deutsch | Latin (Extended) | 100% |
| `fr` | French | Français | Latin (Extended) | 100% |
| `es` | Spanish | Español | Latin (Extended) | 100% |
| `ru` | Russian | Русский | Cyrillic | 100% |
| `zh` | Chinese | 简体中文 | Simplified Hanzi | 100% |
| `ja` | Japanese | 日本語 | Kanji / Kana | 100% |

---

## 3. Architecture & Zero-Allocation Design
Localization in WinBlast0 is designed around compile-time static lookup tables without runtime heap allocations:

```mermaid
flowchart LR
    UI[Rune View] -->|s.tr(TextKey::Key)| I18N[i18n Resolver]
    I18N -->|Language Index| Table[TRANSLATIONS: [Language as usize][TextKey as usize]]
    Table -->|&'static str| View[Rune Text Element]
```

- **Lookup Complexity**: $\mathcal{O}(1)$ array index lookup.
- **Memory Footprint**: 0 heap allocations per translation query.
- **Safety**: Exhaustive enum match ensures no missing translations at compile time.

---

## 4. Comprehensive String Catalog Sections

1. **Top Header & Navigation**:
   - Branding, elevation badges, easy/advanced mode switcher, instant/normal engine toggle.
   - Sidebar group headers: `Optimization Modes`, `Hardware Engines`, `System & Safety`.
   - All 8 navigation tab labels.
2. **Easy Mode Dashboard**:
   - Health score, preset cards (Gaming, Pro Audio, Laptop Battery).
   - Before/After impact metrics (RAM freed, processes reduced, NT timer resolution, active optimizations).
   - Real-time Task Manager telemetry waveform titles.
3. **Dedicated GPU Section**:
   - Adapter specs, PCIe MSI interrupts, priorities (`Low`, `Normal`, `High`).
   - Display engine controls (HAGS, MPO stutter fix).
   - Vendor silicon cards (`NVIDIA`, `AMD`, `Intel`), condition-failed alerts, action links (`Hide`, `What is this?`).
4. **Bottom Changelog & Progress Bar**:
   - Progress bar labels (`Scanning`, `Applying`, `Optimized`).
   - Changelog drawer header, action buttons (`Clear Log`, `Export`), mutation event summaries.
5. **System Hardware & Diagnostics**:
   - CPU topology, NT kernel resolution, thread count, system information.
6. **Privacy Shield & Telemetry Audit**:
   - Audit scanner, telemetry hosts, services, scheduled tasks.
7. **Backup Center**:
   - VSS restore points, SQLite transactional state ledger, export/import profiles.
8. **Modals, Badges & Statuses**:
   - Risk levels (`Safe`, `Low Risk`, `Caution`, `High Risk`, `Extreme`).
   - Tweak states (`Active ✔`, `Stock Default`, `Partial ◐`, `Not Applicable`).
   - Localized Diff Modal inspector labels (`Default`, `Target`, `Start`, `Action`, `State`).
9. **Clean & Apps + Startup, Tasks & Services + Extended Hardware/Unattend (`v0.7.0`)**:
   - Localized `CleanTargetKind` display names, AppX safety tiers (`Safe`, `AI / Copilot`, `Caution`, `ProtectedCore`), WinGet installer, Startup source badges, service startup modes (`Auto`, `Manual`, `Disabled`, `Boot/Sys`), Policy Locked badge, Latency metric labels (`Min`, `Avg`, `Max`, `Jitter`, `Loss`, `Clear Results`), empty states, busy indicator, and `autounattend.xml` preset/architecture controls.
   - `I18n::fold_search_str` handles Turkish dotted/dotless `'İ' | 'I' | 'ı' | 'i' => 'i'` identically in Advanced Matrix, AppX, WinGet, and Startup/Tasks/Services search filters.
