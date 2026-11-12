---
title: Windows OS Detection & Edition Aliases
tags: [core, internals, win32, rtlgetversion, editions]
created: 2026-11-08
---

# 🪟 Windows OS Detection & Edition Aliases

Because Windows 10 and 11 feature different registry paths, Group Policy settings, and task names depending on the build number and edition, accurate OS detection is crucial.

Located in `src/core/os_detect.rs`:

---

## 🏛️ OS Query Mechanics (`RtlGetVersion`)

Standard Win32 API functions like `GetVersionExW` are lie-prone: Windows applications return `Windows 8 (Build 9200)` unless an application manifest explicitly declares Windows 10/11 compatibility.

WinBlast0 dynamically loads `RtlGetVersion` from `ntdll.dll`:
```rust
type RtlGetVersionFn = unsafe extern "system" fn(*mut OSVERSIONINFOEXW) -> i32;
```
`RtlGetVersion` is a native NT kernel query that **always returns the true Windows kernel build number** (e.g. `19045` for Win10 22H2, `22631` for Win11 23H2, and `26100` / `26300` for Win11 24H2 Canary).

Additionally, it queries:
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\EditionID`
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProductName`
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\DisplayVersion` (e.g. `"24H2"`, `"23H2"`)

---

## 🏷️ Flexible Edition Mapping

Different Windows versions report edition names inconsistently (e.g. `Professional`, `Pro`, `Core`, `Home`, `EnterpriseS`, `IoTEnterprise`).

In `src/core/risk.rs`:
```rust
fn matches_edition(rule_edition: &str, detected_edition: &str) -> bool {
    let rule = rule_edition.trim().to_lowercase();
    let detected = detected_edition.trim().to_lowercase();

    if rule.is_empty() || detected.is_empty() || detected == "unknown" {
        return true;
    }
    if rule == detected {
        return true;
    }
    // Pro / Professional
    if (rule == "pro" || rule == "professional")
        && (detected.contains("pro") || detected.contains("professional"))
    {
        return true;
    }
    // Home / Core
    if (rule == "home" || rule == "core")
        && (detected.contains("home") || detected.contains("core"))
    {
        return true;
    }
    // Enterprise / IoTEnterprise / EnterpriseS
    if rule == "enterprise" && detected.contains("enterprise") {
        return true;
    }
    false
}
```
This ensures that when a rule specifies `["Home", "Pro", "Enterprise"]`, it applies seamlessly across all retail, OEM, and LTSC editions.

---

## 🔗 Related Notes
- [[Privilege & Elevation Token Verification]]
- [[Multi-Engine Scanning Subsystem]]
- [[00 - Index (MOC)]]
