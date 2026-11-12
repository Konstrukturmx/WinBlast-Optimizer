---
title: Privacy & Telemetry Shield Tweaks
tags: [tweaks, privacy, telemetry, diagtrack, advertising, hosts]
created: 2026-11-08
---

# 🛡️ Privacy & Telemetry Shield Tweaks

Spanning `privacy.toml`, `telemetry_os.toml`, and `telemetry_apps.toml`, this subsystem dismantles Windows diagnostics, user telemetry tracking, cloud activity synchronization, third-party software updaters, and advertising identifiers.

---

## 📋 Complete Tweak Matrix

### 1. `disable_telemetry_diagtrack` — Connected User Experiences & Telemetry
- **What it does**: Disables the primary Windows telemetry pipeline (`DiagTrack` / Connected User Experiences and Telemetry service) and sets `AllowTelemetry` to `0` (Security/Enterprise level).
- **Targets**:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection\AllowTelemetry` = `0`
  - `ServiceState`: `DiagTrack` -> Start Type = `4` (Disabled)
- **Gain**: Perf +2, Priv +5, Stability: Safe.

### 2. `disable_dmwappushservice` — WAP Push Message Routing Service
- **What it does**: Disables `dmwappushservice`, which routes telemetry and diagnostic packets from applications to Microsoft endpoints.
- **Target**: `ServiceState`: `dmwappushservice` -> Start Type = `4` (Disabled)
- **Gain**: Perf +1, Priv +4, Stability: Safe.

### 3. `disable_delivery_optimization_p2p` — Disable P2P Upload Bandwidth Stealing
- **What it does**: By default, Windows Delivery Optimization uploads update packages to other PCs on the internet. Setting `DODownloadMode` to `0` restricts it to local LAN/HTTP only, preventing upload bandwidth saturation.
- **Target**: `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization\DODownloadMode` = `0`
- **Gain**: Perf +4, Priv +3, Stability: Safe.

### 4. `disable_diagnostic_execution_service` — Disable diagsvc
- **What it does**: Disables the Diagnostic Execution Service (`diagsvc`), which runs scheduled diagnostic background packages.
- **Target**: `ServiceState`: `diagsvc` -> Start Type = `4` (Disabled)
- **Gain**: Perf +2, Priv +4, Stability: Safe.

### 5. `disable_advertising_id` — Disable Advertising ID
- **What it does**: Disables the per-user unique GUID advertising ID used by Microsoft and Store apps for cross-application ad tracking.
- **Targets**:
  - `HKCU\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo\Enabled` = `0`
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo\DisabledByGroupPolicy` = `1`
- **Gain**: Priv +5, Stability: Safe.

### 6. `disable_activity_history` — Disable Activity History & Timeline Uploads
- **What it does**: Prevents Windows from logging every opened document, application, and web page, and blocks cloud timeline uploads to Microsoft accounts.
- **Targets**:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\PublishUserActivities` = `0`
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\System\UploadUserActivities` = `0`
- **Gain**: Perf +2, Priv +5, Stability: Safe.

### 7. `disable_cdp_service` — Connected Devices Platform User Service
- **What it does**: Disables `CDPUserSvc`, eliminating background bluetooth/LAN broadcasting and synchronization beacons for nearby devices.
- **Target**: `ServiceState`: `CDPUserSvc` -> Start Type = `4` (Disabled)
- **Gain**: Perf +2, Priv +4, Stability: Safe.

### 8. `disable_cortana` — Disable Cortana Voice Assistant
- **What it does**: Disables Cortana voice assistant via Group Policy, preventing it from listening, running background audio buffers, and transmitting voice data.
- **Target**: `HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Search\AllowCortana` = `0`
- **Gain**: Perf +2, Priv +5, Stability: Safe.

### 9. `disable_location_tracking` — Disable Geolocation Tracking
- **What it does**: Disables the Windows Geolocation Service (`lfsvc`) and location sensors globally via policy.
- **Targets**:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors\DisableLocation` = `1`
  - `ServiceState`: `lfsvc` -> Start Type = `4` (Disabled)
- **Gain**: Priv +5, Stability: Safe.

### 10. `disable_customer_experience_program` — CEIP Tasks
- **What it does**: Disables Customer Experience Improvement Program scheduled tasks (`Consolidator`, `UsbCeip`, `KernelCeipTask`).
- **Targets**:
  - `ScheduledTask`: `\Microsoft\Windows\Customer Experience Improvement Program\Consolidator` = `false`
  - `ScheduledTask`: `\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip` = `false`
- **Gain**: Perf +2, Priv +4, Stability: Safe.

---

## 🌐 Third-Party Telemetry & Hosts Shield

WinBlast0 features built-in DNS-level host blocking via `C:\Windows\System32\drivers\etc\hosts`:
- **Known Endpoints**: Blocks 20+ prominent telemetry endpoints (`v10.events.data.microsoft.com`, `telemetry.microsoft.com`, `settings-win.data.microsoft.com`, NVIDIA telemetry, Adobe crash daemons).
- **Formatting Safety**: In v0.2.0, the hosts modifier verifies that the existing file ends with a newline before appending, preventing corrupted host lines (`127.0.0.1 localhost0.0.0.0 domain`).

---

## 🔗 Related Notes
- [[OS Settings & Windows 11 AI]]
- [[Bloatware & AppX Deprovisioning]]
- [[00 - Index (MOC)]]
