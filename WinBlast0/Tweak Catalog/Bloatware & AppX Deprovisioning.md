---
title: Bloatware & AppX Deprovisioning
tags: [tweaks, bloatware, appx, uwp, deprovisioning]
created: 2026-11-08
---

# 📦 Bloatware & AppX Deprovisioning

Pre-installed Universal Windows Platform (UWP) / AppX packages consume gigabytes of disk space and run background push notification services, live tile updaters, and scheduled maintenance tasks.

WinBlast0 targets modern Windows 10/11 bloatware without the fragile, multi-second latency of traditional PowerShell `Get-AppxPackage` commands.

---

## 📋 Targeted Bloatware Families (`bloatware_appx.toml`)

| Package Family | Common Display Name | Why Remove |
| :--- | :--- | :--- |
| `Microsoft.BingWeather_8wekyb3d8bbwe` | MSN Weather | Background location tracking, news ad integration |
| `Microsoft.BingNews_8wekyb3d8bbwe` | Microsoft News | Background news fetching, sponsored content |
| `Microsoft.GetHelp_8wekyb3d8bbwe` | Get Help | Telemetry diagnostics assistant |
| `Microsoft.Getstarted_8wekyb3d8bbwe` | Tips / Welcome | Tips notifications and feature promos |
| `Microsoft.MicrosoftSolitaireCollection_8wekyb3d8bbwe` | Solitaire Collection | Full-screen video ads and telemetry |
| `Microsoft.People_8wekyb3d8bbwe` | Microsoft People | Deprecated contact sync |
| `Microsoft.YourPhone_8wekyb3d8bbwe` | Phone Link | Constant bluetooth/WiFi background polling |
| `Microsoft.ZuneMusic_8wekyb3d8bbwe` | Media Player (Groove) | Replaced by modern media tools |
| `Microsoft.ZuneVideo_8wekyb3d8bbwe` | Movies & TV | Push promos and store licensing overhead |
| `Microsoft.WindowsFeedbackHub_8wekyb3d8bbwe` | Feedback Hub | Feedback telemetry and crash reporting |

---

## ⚡ Native 4-Store Registry Indexing

Traditional tools invoke PowerShell:
```powershell
Get-AppxPackage -AllUsers *name* | Remove-AppxPackage -AllUsers
```
Spawning PowerShell runs .NET runtime initialization, taking **4–10 seconds** per package.

WinBlast0 scans 4 native registry hives in **< 15 milliseconds**:
1. `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\Applications`
2. `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Appx\AppxAllUserStore\InboxApplications`
3. `HKCU\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages`
4. `HKLM\SOFTWARE\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages`

### Execution Mechanics:
When removing a package, WinBlast0 invokes a targeted, high-performance PowerShell runner script (`src/executor/ps_runner.rs`) with `-NoProfile -NonInteractive -ExecutionPolicy Bypass` that strips both user installation and system deprovisioning in a single execution.

---

## 🔗 Related Notes
- [[Catalog Overview & Risk Model]]
- [[Multi-Engine Scanning Subsystem]]
- [[00 - Index (MOC)]]
