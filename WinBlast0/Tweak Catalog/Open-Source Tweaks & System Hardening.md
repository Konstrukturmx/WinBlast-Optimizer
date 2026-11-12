# Open-Source Tweaks & System Hardening (v0.5.0)

## 1. Overview
WinBlast0 v0.5.0 incorporates battle-tested optimizations reverse-engineered and curated from leading open-source Windows tuning tools, including **Sophia Script**, **AtlasOS**, **Chris Titus WinUtil**, **ReviOS**, and **Optimizer**.

---

## 2. Newly Integrated Tweaks Catalog

### 1. Gaming & Input Latency
- **Disable Sticky Keys & Accessibility Hotkeys (`disable_sticky_keys`)**:
  - *Origin*: Sophia Script / AtlasOS.
  - *Purpose*: Eliminates the 5x Shift popup dialog and keyboard toggle beeps that interrupt competitive gaming.
  - *Registry*:
    - `HKCU\Control Panel\Accessibility\StickyKeys` -> `Flags = 506`
    - `HKCU\Control Panel\Accessibility\Keyboard Response` -> `Flags = 122`
    - `HKCU\Control Panel\Accessibility\ToggleKeys` -> `Flags = 58`

- **Disable Game Bar Presence Writer (`disable_game_bar_presence_writer`)**:
  - *Origin*: AtlasOS / ReviOS.
  - *Purpose*: Disables `GameBarPresenceWriter.exe` which attaches to game executables and causes frame time micro-stuttering.
  - *Registry*:
    - `HKLM\SOFTWARE\Microsoft\WindowsRuntime\ActivatableClassId\Windows.Gaming.Internal.PresenceWriter` -> `ActivationType = 0`

- **Disable High Precision Event Timer Synthetic Interrupts (`disable_hpet_synthetic_timer`)**:
  - *Origin*: AtlasOS.
  - *Purpose*: Prevents the NT kernel from relying on synthetic HPET timers, enforcing hardware Invariant TSC clocks for ultra-low DPC latency.
  - *Registry*:
    - `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\kernel` -> `DisableDynamicTick = 1`

### 2. Network & Bandwidth Optimization
- **Disable Delivery Optimization P2P Sharing (`disable_delivery_optimization_p2p`)**:
  - *Origin*: Chris Titus WinUtil / Optimizer.
  - *Purpose*: Prevents Windows Update from utilizing consumer upload bandwidth as a peer-to-peer distribution node for external machines.
  - *Registry*:
    - `HKLM\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization` -> `DODownloadMode = 0`

- **Optimize Network TCP Congestion & Window Auto-Tuning (`optimize_tcp_congestion_tuning`)**:
  - *Origin*: ReviOS.
  - *Purpose*: Enables TCP timestamp options and disables WSD multicast overhead to improve packet throughput and ping stability.
  - *Registry*:
    - `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters` -> `EnableWsd = 0`
    - `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters` -> `Tcp1323Opts = 1`

### 3. Stability & Diagnostics
- **Disable Windows Error Reporting Service (`disable_windows_error_reporting`)**:
  - *Origin*: Sophia Script / Optimizer.
  - *Purpose*: Prevents `werfault.exe` and `WerSvc` from freezing crashed game processes to generate massive memory dumps and upload them to telemetry servers.
  - *Registry*:
    - `HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting` -> `Disabled = 1`
    - `HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting` -> `Disabled = 1`

- **Disable Windows Search Bing Cloud Suggestions (`disable_bing_search_start_menu`)**:
  - *Origin*: Chris Titus WinUtil.
  - *Purpose*: Stops Windows Search from querying Bing servers with local keystrokes, eliminating search latency and removing unwanted web advertisements from the Start Menu.
  - *Registry*:
    - `HKCU\Software\Policies\Microsoft\Windows\Explorer` -> `DisableSearchBoxSuggestions = 1`
    - `HKCU\Software\Microsoft\Windows\CurrentVersion\Search` -> `BingSearchEnabled = 0`
    - `HKCU\Software\Microsoft\Windows\CurrentVersion\Search` -> `CortanaConsent = 0`

- **Lock Kernel Paging Executive in RAM (`disable_paging_executive`)**:
  - *Origin*: AtlasOS.
  - *Purpose*: Keeps the NT kernel executive and device drivers in physical memory rather than paging them out to disk, ensuring instantaneous interrupt response.
  - *Registry*:
    - `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management` -> `DisablePagingExecutive = 1`
