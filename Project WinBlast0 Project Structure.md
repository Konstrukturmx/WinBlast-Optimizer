## **Project WinBlast0**

## 

## **Architectural Enhancements**

* **Dual-Layer Rollback Engine**: Pair Windows System Restore Points (via WMI/VSS) with an internal SQLite state ledger that snapshots exact pre-change registry values, service startup types, and scheduled task states for instant single-tweak undo.  
* **Declarative TOML Rule Engine**: Decouple optimization logic from the Rust binary by defining tweaks in structured TOML files, allowing community auditing and rule updates without recompiling.  
* **Multi-Axis Risk Valuation**: Score every tweak across three metrics—Stability Risk, Performance Impact, and Feature Breakage (such as Xbox Game Bar, Windows Store, or network printing)—so users know exact trade-offs before applying changes.  
* **Easy vs. Advanced Mode Gating**: Restrict Easy Mode to safe, one-click profiles (Gaming, Pro Audio/Workstation, Laptop Battery) that filter out high-risk tweaks, while Advanced Mode exposes granular toggles, raw values, and custom scripts.  
* **Low-Level Hardware & Interrupt Tuning**: Go beyond power plans by automating PCIe Message Signaled Interrupts (MSI Mode), GPU interrupt priority affinity, Multi-Plane Overlay (MPO) stutter fixes, and hybrid CPU (P-core/E-core) parking rules.  
* **Cross-Application Telemetry Auditor**: Scan installed software alongside Windows OS components to neutralize third-party background telemetry and updaters from NVIDIA, Adobe, Razer, Office, and Chromium browsers via services, tasks, and firewall rules.  
* **Pre/Post Telemetry & Latency Benchmark**: Capture baseline RAM usage, active thread/handle counts, and DPC/ISR interrupt latency before applying tweaks to display a concrete before-and-after comparison.

## **Recommended Rust Tech Stack**

A system debloater should avoid WebView2-based frameworks (like Tauri) because users frequently strip Microsoft Edge and WebView2 from optimized Windows installs. A pure-Rust GPU-accelerated GUI guarantees the binary runs standalone on any Windows 10/11 build.

* **iced** or **egui**: Pure-Rust native GUI frameworks with custom styling support for modern dark-mode dashboards and toggle matrices without browser engine overhead.  
* **windows** and **windows-sys**: Official Microsoft Rust bindings for Win32 APIs, Service Control Manager, Security/UAC token inspection, and SetupAPI device enumeration.  
* **winreg**: Safe, ergonomic read/write/delete operations for HKLM, HKCU, and HKU registry hives.  
* **wmi**: Asynchronous WMI queries to trigger System Restore points, inspect BIOS settings, and query AppX/MSIX bloatware packages.  
* **raw-cpuid** and **sysinfo**: Accurate detection of CPU vendor (Intel/AMD), hybrid core topology, cache hierarchy, and real-time memory/process metrics.  
* **nvml-wrapper** and **dxgi**: Direct inspection of NVIDIA/AMD/Intel GPUs, VRAM state, driver versions, and PCIe bus hardware IDs.  
* **rusqlite**: Embedded local database to store the transactional undo history and scan results.  
* **serde** and **toml**: Parsing and validating declarative optimization rules and user export profiles.  
* **tokio**: Multi-threaded async runtime so parallel registry, WMI, and filesystem scans never block the UI thread.  
* **embed-manifest**: Build-time crate to embed a Windows application manifest requiring Administrator privileges (UAC) on launch.

## **Project Directory Structure**

Plaintext

```
winblast0/
├── Cargo.toml
├── build.rs                          # Embeds UAC Administrator manifest & Windows icon
├── assets/
│   ├── icons/                        # App and hardware vendor icons
│   └── rules/                        # Declarative TOML optimization catalogs
│       ├── os_settings.toml          # Visual effects, power plans, NTFS, network
│       ├── registry_lag.toml         # MenuShowDelay, Win32PrioritySeparation, MMCSS
│       ├── bloatware_appx.toml       # UWP/AppX package whitelist & removal rules
│       ├── telemetry_os.toml         # DiagTrack, dmwappushservice, CompatTelRunner
│       ├── telemetry_apps.toml       # NVIDIA, Adobe, Edge, Steam, Chrome rules
│       ├── privacy.toml              # Advertising ID, Cortana, Recall, cloud search
│       └── hardware_tuning.toml      # MSI Mode, HAGS, MPO, Core Parking, C-States
└── src/
    ├── main.rs                       # Entry point, UAC check, async runtime init
    ├── app.rs                        # Top-level GUI state machine (Easy vs Advanced)
    ├── core/
    │   ├── mod.rs
    │   ├── error.rs                  # Unified error handling (thiserror)
    │   ├── os_detect.rs              # Windows 10/11 build, edition, & 24H2+ feature checks
    │   ├── elevation.rs              # Token privilege verification (SeRestorePrivilege, etc.)
    │   └── risk.rs                   # Risk calculation matrix & dependency checker
    ├── backup/
    │   ├── mod.rs
    │   ├── restore_point.rs          # WMI SystemRestore checkpoint creation & trigger
    │   ├── state_ledger.rs           # SQLite transactional backup of mutated keys/services
    │   └── profile_io.rs             # Export/import user tweak configurations (.wb0)
    ├── hardware/
    │   ├── mod.rs
    │   ├── cpu.rs                    # Vendor, P/E cores, power plan GUIDs, thread quantum
    │   ├── gpu.rs                    # PCIe device path, MSI mode status, HAGS, shader cache
    │   └── latency.rs                # Timer resolution & DPC latency snapshot
    ├── scanner/
    │   ├── mod.rs                    # Parallel scan orchestrator
    │   ├── registry_scanner.rs       # Evaluates current registry state against rules
    │   ├── service_scanner.rs        # Win32 Service Control Manager state auditor
    │   ├── task_scanner.rs           # Windows Task Scheduler XML/COM inspector
    │   ├── appx_scanner.rs           # Installed UWP bloatware detector
    │   └── telemetry_scanner.rs      # Hosts file, firewall rules, & 3rd-party agent detector
    ├── executor/
    │   ├── mod.rs
    │   ├── engine.rs                 # Applies tweaks atomically after writing to state_ledger
    │   ├── reg_mutator.rs            # Registry mutation & TrustedInstaller ownership handler
    │   ├── svc_mutator.rs            # Service start-type & state controller
    │   └── ps_runner.rs              # Constrained PowerShell/DISM runner for AppX removal
    └── ui/
        ├── mod.rs
        ├── theme.rs                  # Color tokens (Risk badges: Green/Amber/Red)
        ├── views/
        │   ├── easy_dashboard.rs     # 1-Click presets, health score, instant restore button
        │   ├── advanced_matrix.rs    # Filterable tree of all tweaks with live risk breakdown
        │   ├── hardware_view.rs      # CPU/GPU topology cards & hardware-specific switches
        │   ├── privacy_shield.rs     # OS & 3rd-party telemetry dashboard
        │   └── backup_center.rs      # Restore points & granular transaction history log
        └── widgets/
            ├── risk_badge.rs         # Visual indicator of Stability Risk & Performance Gain
            └── diff_modal.rs         # Shows exact Registry/Service changes before execution
```

## **Core Architecture & Code Blueprint**

### **1\. Domain Model & Risk Valuation Engine**

This core module defines how WinBlast0 models tweaks, evaluates OS compatibility, calculates risk, and guarantees every action can be reversed.

Rust

```
// src/core/risk.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe = 1,        // Zero side effects, instant revert (Easy Mode eligible)
    Low = 2,         // Minor UX change (e.g., disables animations or tips)
    Moderate = 3,    // May affect specific workflows (e.g., disables Xbox Game Bar)
    High = 4,        // Alters core OS scheduling/security (Advanced Mode only)
    Extreme = 5,     // Deep stripping; requires reboot and may break Windows Update
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reversibility {
    Instant,
    RequiresReboot,
    RequiresPackageReinstall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskValuation {
    pub stability_risk: RiskLevel,
    pub performance_gain: u8,         // 1 to 5 scale
    pub privacy_gain: u8,             // 1 to 5 scale
    pub reversibility: Reversibility,
    pub broken_features: Vec<String>, // e.g., ["Clipboard History", "Windows Store"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRequirement {
    pub min_build: u32,               // e.g., 19041 for Win10 2004, 22000 for Win11
    pub max_build: Option<u32>,
    pub editions: Vec<String>,        // e.g., ["Pro", "Enterprise", "Home"]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TweakAction {
    RegistryDword {
        hive: String,
        path: String,
        name: String,
        optimized_value: u32,
        default_value: u32,
    },
    ServiceState {
        service_name: String,
        target_start_type: u32,       // 4 = Disabled, 3 = Manual, 2 = Auto
        default_start_type: u32,
    },
    ScheduledTask {
        task_path: String,
        enable: bool,
    },
    AppXPackage {
        package_family: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakDefinition {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub easy_mode_included: bool,
    pub os_req: OsRequirement,
    pub valuation: RiskValuation,
    pub actions: Vec<TweakAction>,
}
```

### **2\. Declarative Rule Format**

Storing tweaks in TOML files under the assets directory lets Advanced Mode display the exact registry paths and service names before the user clicks Apply.

Ini, TOML

```
# assets/rules/registry_lag.toml

[[tweaks]]
id = "win32_priority_separation_short_fixed"
title = "Optimize Foreground Thread Quantum (Gaming)"
category = "CPU & Scheduling"
description = "Configures Win32PrioritySeparation to 0x26 (Short, Fixed, High foreground boost) for lower input latency."
easy_mode_included = true

[tweaks.os_req]
min_build = 19041
editions = ["Home", "Pro", "Enterprise", "Education"]

[tweaks.valuation]
stability_risk = "Safe"
performance_gain = 4
privacy_gain = 1
reversibility = "Instant"
broken_features = []

[[tweaks.actions]]
type = "RegistryDword"
hive = "HKLM"
path = "SYSTEM\\CurrentControlSet\\Control\\PriorityControl"
name = "Win32PrioritySeparation"
optimized_value = 38 # 0x26 Hex
default_value = 2    # Windows Default
```

### **3\. Hardware GPU MSI Mode & Priority Optimizer Logic**

To optimize detected GPUs dynamically, WinBlast0 enumerates the PCI bus via SetupAPI or registry class keys, locates the active display adapter's hardware instance path, and enables Message Signaled Interrupts (MSI) so the GPU avoids sharing virtual IRQ lines.

Rust

```
// src/hardware/gpu.rs
use winreg::enums::*;
use winreg::RegKey;
use std::io;

pub struct DetectedGpu {
    pub name: String,
    pub pnp_instance_id: String, // e.g., PCI\VEN_10DE&DEV_2684&SUBSYS_...
    pub msi_enabled: bool,
    pub interrupt_priority: u32, // 1 = Low, 2 = Normal, 3 = High
}

pub fn set_gpu_msi_mode(pnp_instance_id: &str, enable: bool, priority: u32) -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let msi_path = format!(
        "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
        pnp_instance_id
    );
    let affinity_path = format!(
        "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\Affinity Policy",
        pnp_instance_id
    );

    let (msi_key, _) = hklm.create_subkey(&msi_path)?;
    msi_key.set_value("MSISupported", &(if enable { 1u32 } else { 0u32 }))?;

    let (aff_key, _) = hklm.create_subkey(&affinity_path)?;
    aff_key.set_value("DevicePriority", &priority)?;

    Ok(())
}
```

### **4\. Auto-Elevation Manifest Setup**

Because modifying HKLM keys, system services, and System Restore points requires Administrator privileges, add a build script at the root of the project so the compiled executable automatically prompts for UAC elevation.

Rust

```
// build.rs
use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifestExecutionLevel::RequireAdministrator;

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(
            new_manifest("WinBlast0.Optimizer")
                .requested_execution_level(RequireAdministrator)
        ).expect("Failed to embed UAC administrator manifest");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
```

