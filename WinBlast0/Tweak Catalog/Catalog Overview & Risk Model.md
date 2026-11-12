---
title: Declarative Tweak Catalog & Risk Valuation Model
tags: [catalog, tweaks, risk, toml, embedding]
created: 2026-11-08
updated: 2026-11-10
version: v0.7.0
---

# 📜 Declarative Tweak Catalog & Risk Valuation Model

All system modifications in WinBlast0 are defined **declaratively** in TOML catalogs. There is zero hardcoded registry hacking scattered through the codebase. Every tweak is a typed data structure specifying its applicability, risk rating, performance gain, privacy gain, broken features, `unattend_safe` setup eligibility, and rollback targets.

---

## 🏛️ Tweak Definition Schema (`src/core/risk.rs`)

```rust
pub struct TweakDefinition {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub easy_mode_included: bool,
    pub os_req: OsRequirement,
    pub valuation: RiskValuation,
    pub actions: Vec<TweakAction>,
    pub gpu_vendor: Option<String>,
    pub sub_category: Option<String>,
    pub unattend_safe: bool,
}
```

### Supported `TweakAction` Variants:
- `RegistryDword`, `RegistryString`, `RegistryDelete`, `RegistryDeleteTree`
- `RegistryBinaryBit` (surgical byte/bitmask preservation), `CompositeStringKey` (e.g. DirectX `UserPreferences` token merge), `RegistryDynamicDword` (`TotalRamInKb` formula using `u64` math)
- `PowerCfgSetting` (`AcOnly` / `DcOnly` / `AcAndDc` via native `powrprof`)
- `ServiceState`, `ScheduledTask`, `AppXPackage`, `WindowsFeature` (`OptionalFeature` / DISM `Capability`)

### Reversibility & Risk Assessment Spectrum:
- **Risk**: `Safe`, `Low`, `Moderate`, `High` / `Extreme` (High & Extreme are excluded from Easy Mode, presets, and `Select All`).
- **Reversibility**: `Instant`, `RequiresReboot`, `RequiresLogoff`, or `RequiresPackageReinstall`.

---

## 📦 Zero-Fail Embedded Catalogs (`include_str!`)

All **10 TOML catalog files** (`168` total tweaks) are compiled directly into the standalone binary's `.rdata` section via `EMBEDDED_CATALOGS` in [`src/core/risk.rs`](file:///c:/Hive/Projects/WinBlast0/src/core/risk.rs):

| Catalog File | Domain | Tweak Count | Focus |
| :--- | :--- | :---: | :--- |
| `ai_privacy_ext.toml` | AI, Privacy & Telemetry | 21 | Windows Recall/Copilot/Paint/Edge/Office AI, WMI AutoLoggers, ConsentStore, telemetry tasks/services |
| `bloatware_appx.toml` | [[Bloatware & AppX Deprovisioning]] | 26 | Consumer UWP/AppX packages & Windows Capabilities (Steps Recorder, IE11, Recall) |
| `hardware_tuning.toml` | [[Hardware Tuning & MSI Mode]] | 16 | PCIe MSI mode, HAGS, MPO, NVIDIA/AMD/Intel vendor tuning, PowerCfg bindings |
| `os_settings.toml` | [[OS Settings & Windows 11 AI]] | 21 | Core Win10/Win11 Copilot, Bing Search, GameDVR, Cloud Content, explorer & update UX |
| `perf_power_ext.toml` | Performance & Power Ext | 20 | `LongPathsEnabled`, MMCSS, DirectX/VRR overlays, `PowerCfgSetting`, `SvcHostSplitThresholdInKB`, VBS/HVCI opt-ins |
| `privacy.toml` | [[Privacy & Telemetry Shield]] | 12 | Activity history, advertising ID, location sensors, spotlight ads |
| `registry_lag.toml` | [[Registry Lag & Kernel Scheduling]] | 18 | `Win32PrioritySeparation`, MMCSS gaming/Pro Audio, queues, TCP/network tuning, Fast Startup |
| `telemetry_apps.toml` | [[Privacy & Telemetry Shield]] | 7 | Third-party background telemetry tasks/services |
| `telemetry_os.toml` | [[Privacy & Telemetry Shield]] | 14 | OS telemetry services (`DiagTrack`, `WerSvc`, `RemoteRegistry`), scheduled tasks |
| `ux_customization.toml` | UX Customization | 13 | Classic context menu, Explorer navigation panes, `UserPreferencesMask` bitfields, Taskbar layout, Dark Mode |
| **Total** | | **168** | **Zero duplicate IDs, zero conflicts across overlapping scopes** |

---

## 🔗 Related Notes
- [[Registry Lag & Kernel Scheduling]]
- [[OS Settings & Windows 11 AI]]
- [[Privacy & Telemetry Shield]]
- [[Bloatware & AppX Deprovisioning]]
- [[00 - Index (MOC)]]
