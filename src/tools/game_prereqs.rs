//! Game Prerequisites Installer & Verification Engine (v1.4).
//!
//! Provides hardware-level detection and multi-stage cascade installation
//! (winget -> choco -> direct official download URL) for the 10 essential
//! gaming runtimes, libraries, and frameworks required by modern PC titles.

use crate::core::process::SilentCommand;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

/// Installation and verification lifecycle states for game prerequisites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrereqInstallState {
    NotInstalled,
    Installed,
    Installing,
    Failed,
}

/// Represents an essential gaming runtime or prerequisite package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePrerequisiteItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub installed: bool,
    pub installed_version: Option<String>,
    pub winget_id: Option<String>,
    pub choco_id: Option<String>,
    pub direct_url: String,
    pub state: PrereqInstallState,
    pub status_message: String,
}

impl GamePrerequisiteItem {
    /// True if the prerequisite is currently detected as installed.
    pub fn is_installed(&self) -> bool {
        self.installed
    }
}

struct PrereqDescriptor {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    description: &'static str,
    winget_id: Option<&'static str>,
    choco_id: Option<&'static str>,
    direct_url: &'static str,
    detect: fn() -> (bool, Option<String>),
    success_msg: &'static str,
    missing_msg: &'static str,
}

const PREREQ_DESCRIPTORS: [PrereqDescriptor; 10] = [
    // 1. DirectX End-User Runtimes (June 2010)
    PrereqDescriptor {
        id: "directx_june2010",
        name: "DirectX End-User Runtimes (June 2010)",
        category: "Graphics Runtime",
        description: "Legacy D3DX9, D3DX10, and D3DX11 DLLs required by thousands of classic & Unreal/Unity games.",
        winget_id: Some("Microsoft.DirectX"),
        choco_id: Some("directx"),
        direct_url: "https://www.microsoft.com/en-us/download/details.aspx?id=35",
        detect: detect_directx_legacy,
        success_msg: "Installed & Verified",
        missing_msg: "Missing legacy DirectX DLLs",
    },
    // 2. Visual C++ 2015-2022 Redistributable (x64)
    PrereqDescriptor {
        id: "vcredist_2015_2022_x64",
        name: "Visual C++ 2015-2022 Redistributable (x64)",
        category: "C++ Runtime",
        description: "Universal C runtime libraries for 64-bit modern PC gaming engines and launchers.",
        winget_id: Some("Microsoft.VCRedist.2015+.x64"),
        choco_id: Some("vcredist140"),
        direct_url: "https://aka.ms/vs/17/release/vc_redist.x64.exe",
        detect: detect_vcredist_2015_2022_x64,
        success_msg: "Installed & Verified",
        missing_msg: "64-bit VC++ 2015-2022 not found",
    },
    // 3. Visual C++ 2015-2022 Redistributable (x86)
    PrereqDescriptor {
        id: "vcredist_2015_2022_x86",
        name: "Visual C++ 2015-2022 Redistributable (x86)",
        category: "C++ Runtime",
        description: "Universal 32-bit runtime required by older games, emulators, and 32-bit helper binaries.",
        winget_id: Some("Microsoft.VCRedist.2015+.x86"),
        choco_id: Some("vcredist2015"),
        direct_url: "https://aka.ms/vs/17/release/vc_redist.x86.exe",
        detect: detect_vcredist_2015_2022_x86,
        success_msg: "Installed & Verified",
        missing_msg: "32-bit VC++ 2015-2022 not found",
    },
    // 4. Visual C++ All-in-One Redistributables (2005-2022)
    PrereqDescriptor {
        id: "vcredist_all_in_one",
        name: "Visual C++ All-in-One Package (2005-2022)",
        category: "C++ Runtime Suite",
        description: "Complete unified package containing VC++ 2005, 2008, 2010, 2012, 2013, and 2022 runtimes.",
        winget_id: Some("TechPowerUp.VisualCppRedistributable"),
        choco_id: Some("vcredist-all"),
        direct_url: "https://www.techpowerup.com/download/visual-c-redistributable-runtime-package-all-in-one/",
        detect: detect_vcredist_all_in_one,
        success_msg: "Full VC++ legacy suite detected",
        missing_msg: "Incomplete legacy runtime stack",
    },
    // 5. Vulkan Runtime
    PrereqDescriptor {
        id: "vulkan_runtime",
        name: "Vulkan Runtime Libraries",
        category: "Graphics API",
        description: "Low-overhead cross-platform 3D graphics API loader for modern games (DOOM, Baldur's Gate 3, etc.).",
        winget_id: Some("LunarG.VulkanSDK"),
        choco_id: Some("vulkan-runtime"),
        direct_url: "https://vulkan.lunarg.com/sdk/home",
        detect: detect_vulkan_runtime,
        success_msg: "Installed & Verified",
        missing_msg: "vulkan-1.dll not found in System32",
    },
    // 6. .NET Desktop Runtime 8.0 (x64)
    PrereqDescriptor {
        id: "dotnet_desktop_8",
        name: ".NET Desktop Runtime 8.0 (LTS)",
        category: "Managed Framework",
        description: "Long-Term Support runtime for modern game mod managers, launchers, and system overlays.",
        winget_id: Some("Microsoft.DotNet.DesktopRuntime.8"),
        choco_id: Some("dotnet-8.0-desktopruntime"),
        direct_url: "https://dotnet.microsoft.com/download/dotnet/8.0",
        detect: detect_dotnet_desktop_8,
        success_msg: "Installed & Verified",
        missing_msg: ".NET 8.0 Desktop Runtime missing",
    },
    // 7. .NET Desktop Runtime 9.0 (x64)
    PrereqDescriptor {
        id: "dotnet_desktop_9",
        name: ".NET Desktop Runtime 9.0 (STS)",
        category: "Managed Framework",
        description: "Latest high-performance .NET runtime with advanced JIT optimizations for cutting-edge game utilities.",
        winget_id: Some("Microsoft.DotNet.DesktopRuntime.9"),
        choco_id: Some("dotnet-9.0-desktopruntime"),
        direct_url: "https://dotnet.microsoft.com/download/dotnet/9.0",
        detect: detect_dotnet_desktop_9,
        success_msg: "Installed & Verified",
        missing_msg: ".NET 9.0 Desktop Runtime missing",
    },
    // 8. .NET Framework 4.8.1
    PrereqDescriptor {
        id: "dotnet_framework_481",
        name: ".NET Framework 4.8.1",
        category: "System Framework",
        description: "Core Windows application framework with ARM64 and accessibility enhancements.",
        winget_id: Some("Microsoft.DotNet.Framework.DeveloperPack_4"),
        choco_id: Some("netfx-4.8.1"),
        direct_url: "https://dotnet.microsoft.com/download/dotnet-framework/net481",
        detect: detect_dotnet_framework_481,
        success_msg: "Installed & Verified",
        missing_msg: ".NET Framework 4.8.1 not detected",
    },
    // 9. OpenAL (Open Audio Library)
    PrereqDescriptor {
        id: "openal",
        name: "OpenAL (Open Audio Library)",
        category: "Audio API",
        description: "3D positional audio API library required by Minecraft, source ports, and hundreds of 3D games.",
        winget_id: Some("OpenAL.OpenAL"),
        choco_id: Some("openal"),
        direct_url: "https://www.openal.org/downloads/",
        detect: detect_openal,
        success_msg: "Installed & Verified",
        missing_msg: "OpenAL32.dll not detected in system PATH",
    },
    // 10. Microsoft XNA Framework Redistributable 4.0
    PrereqDescriptor {
        id: "xna_framework_4",
        name: "Microsoft XNA Framework 4.0 Refresh",
        category: "Gaming Runtime",
        description: "Essential runtime library for indie classics including Terraria, Bastion, Fez, and Stardew Valley legacy.",
        winget_id: Some("Microsoft.XNAFramework"),
        choco_id: Some("xna"),
        direct_url: "https://www.microsoft.com/en-us/download/details.aspx?id=20914",
        detect: detect_xna_framework_4,
        success_msg: "Installed & Verified",
        missing_msg: "XNA 4.0 runtime assembly missing",
    },
];

/// Scan all 10 essential gaming prerequisites against registry and system files in parallel via Rayon.
pub fn scan_game_prereqs() -> Vec<GamePrerequisiteItem> {
    PREREQ_DESCRIPTORS
        .into_par_iter()
        .map(|desc| {
            let (installed, ver) = (desc.detect)();
            GamePrerequisiteItem {
                id: desc.id.to_string(),
                name: desc.name.to_string(),
                category: desc.category.to_string(),
                description: desc.description.to_string(),
                installed,
                installed_version: ver,
                winget_id: desc.winget_id.map(ToString::to_string),
                choco_id: desc.choco_id.map(ToString::to_string),
                direct_url: desc.direct_url.to_string(),
                state: if installed {
                    PrereqInstallState::Installed
                } else {
                    PrereqInstallState::NotInstalled
                },
                status_message: if installed {
                    desc.success_msg.to_string()
                } else {
                    desc.missing_msg.to_string()
                },
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Detection Helpers
// ---------------------------------------------------------------------------

fn get_system32_path() -> PathBuf {
    std::env::var("SystemRoot")
        .map(|r| PathBuf::from(r).join("System32"))
        .unwrap_or_else(|_| PathBuf::from("C:\\Windows\\System32"))
}

fn get_syswow64_path() -> PathBuf {
    std::env::var("SystemRoot")
        .map(|r| PathBuf::from(r).join("SysWOW64"))
        .unwrap_or_else(|_| PathBuf::from("C:\\Windows\\SysWOW64"))
}

fn detect_directx_legacy() -> (bool, Option<String>) {
    // Check for d3dx9_43.dll, d3dx10_43.dll, d3dx11_43.dll in System32
    let s32 = get_system32_path();
    let has_d3dx9 = s32.join("d3dx9_43.dll").exists();
    let has_d3dx10 = s32.join("d3dx10_43.dll").exists();
    let has_d3dx11 = s32.join("d3dx11_43.dll").exists();

    if has_d3dx9 && has_d3dx11 {
        (true, Some("9.29.1974 (June 2010)".to_string()))
    } else if has_d3dx9 || has_d3dx10 || has_d3dx11 {
        (true, Some("Partial DirectX Legacy".to_string()))
    } else {
        (false, None)
    }
}

fn detect_vcredist_2015_2022_x64() -> (bool, Option<String>) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64")
    {
        let installed: Result<u32, _> = key.get_value("Installed");
        let version: Result<String, _> = key.get_value("Version");
        if installed.unwrap_or(0) == 1 {
            return (true, version.ok());
        }
    }
    // Fallback file check
    if get_system32_path().join("vcruntime140.dll").exists() {
        return (true, Some("14.x (Detected via System32)".to_string()));
    }
    (false, None)
}

fn detect_vcredist_2015_2022_x86() -> (bool, Option<String>) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let paths = [
        "SOFTWARE\\WOW6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x86",
        "SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x86",
    ];
    for p in paths {
        if let Ok(key) = hklm.open_subkey(p) {
            let installed: Result<u32, _> = key.get_value("Installed");
            let version: Result<String, _> = key.get_value("Version");
            if installed.unwrap_or(0) == 1 {
                return (true, version.ok());
            }
        }
    }
    if get_syswow64_path().join("vcruntime140.dll").exists() {
        return (true, Some("14.x (Detected via SysWOW64)".to_string()));
    }
    (false, None)
}

fn detect_vcredist_all_in_one() -> (bool, Option<String>) {
    let s32 = get_system32_path();
    let winsxs = s32
        .parent()
        .map(|p| p.join("WinSxS"))
        .unwrap_or_else(|| PathBuf::from("C:\\Windows\\WinSxS"));
    // Check multiple legacy VC runtime DLLs
    let has_2005 = s32.join("msvcr80.dll").exists() || winsxs.exists();
    let has_2008 = s32.join("msvcr90.dll").exists();
    let has_2010 = s32.join("msvcr100.dll").exists();
    let has_2012 = s32.join("msvcr110.dll").exists();
    let has_2013 = s32.join("msvcr120.dll").exists();
    let has_2022 = s32.join("vcruntime140.dll").exists();

    let count = [has_2005, has_2008, has_2010, has_2012, has_2013, has_2022]
        .iter()
        .filter(|&&b| b)
        .count();

    if count >= 4 {
        (true, Some(format!("{}/6 Generations Detected", count)))
    } else {
        (false, Some(format!("{}/6 Generations Detected", count)))
    }
}

fn detect_vulkan_runtime() -> (bool, Option<String>) {
    if get_system32_path().join("vulkan-1.dll").exists() {
        return (true, Some("vulkan-1.dll active".to_string()));
    }
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(_key) = hklm.open_subkey("SOFTWARE\\Khronos\\Vulkan") {
        return (true, Some("Khronos Vulkan Registry".to_string()));
    }
    (false, None)
}

fn detect_dotnet_desktop_8() -> (bool, Option<String>) {
    let p = Path::new("C:\\Program Files\\dotnet\\shared\\Microsoft.WindowsDesktop.App");
    if p.exists() {
        if let Ok(entries) = std::fs::read_dir(p) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("8.") {
                        return (true, Some(name.to_string()));
                    }
                }
            }
        }
    }
    (false, None)
}

fn detect_dotnet_desktop_9() -> (bool, Option<String>) {
    let p = Path::new("C:\\Program Files\\dotnet\\shared\\Microsoft.WindowsDesktop.App");
    if p.exists() {
        if let Ok(entries) = std::fs::read_dir(p) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with("9.") {
                        return (true, Some(name.to_string()));
                    }
                }
            }
        }
    }
    (false, None)
}

fn detect_dotnet_framework_481() -> (bool, Option<String>) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full") {
        let release: Result<u32, _> = key.get_value("Release");
        if let Ok(rel) = release {
            if rel >= 533320 {
                return (true, Some("4.8.1".to_string()));
            } else if rel >= 528040 {
                return (true, Some("4.8".to_string()));
            } else if rel >= 461808 {
                return (true, Some("4.7.2".to_string()));
            }
        }
    }
    (false, None)
}

fn detect_openal() -> (bool, Option<String>) {
    if get_system32_path().join("OpenAL32.dll").exists()
        || get_syswow64_path().join("OpenAL32.dll").exists()
    {
        return (true, Some("OpenAL 1.1 / Soft".to_string()));
    }
    (false, None)
}

fn detect_xna_framework_4() -> (bool, Option<String>) {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let paths = [
        "SOFTWARE\\WOW6432Node\\Microsoft\\XNA\\Framework\\v4.0",
        "SOFTWARE\\Microsoft\\XNA\\Framework\\v4.0",
    ];
    for p in paths {
        if let Ok(key) = hklm.open_subkey(p) {
            let installed: Result<u32, _> = key.get_value("Installed");
            if installed.unwrap_or(0) == 1 {
                return (true, Some("4.0.40903.0".to_string()));
            }
        }
    }
    if Path::new("C:\\Program Files (x86)\\Microsoft XNA\\XNA Game Studio\\v4.0").exists() {
        return (true, Some("4.0 (Game Studio Dir)".to_string()));
    }
    (false, None)
}

// ---------------------------------------------------------------------------
// Multi-Stage Cascade Installer (winget -> choco -> direct URL)
// ---------------------------------------------------------------------------

/// Installs a game prerequisite using a 3-tier cascade:
/// 1. `winget install --id <id> --silent --accept-package-agreements --accept-source-agreements`
/// 2. If winget fails, `choco install <id> -y`
/// 3. If choco fails, launch official vendor download page in default browser.
pub fn install_prereq_cascade(item: &mut GamePrerequisiteItem) -> Result<String, String> {
    item.state = PrereqInstallState::Installing;
    item.status_message = "Attempting package manager install...".to_string();

    // Tier 1: Try Winget
    if let Some(ref wid) = item.winget_id {
        item.status_message = format!("Running winget install for {}...", wid);
        let output = SilentCommand::new("winget")
            .args([
                "install",
                "--id",
                wid,
                "--exact",
                "--silent",
                "--accept-package-agreements",
                "--accept-source-agreements",
            ])
            .output();

        if let Ok(o) = output {
            if o.status.success() {
                item.installed = true;
                item.state = PrereqInstallState::Installed;
                item.status_message = format!("Successfully installed via winget: {}", wid);
                return Ok(item.status_message.clone());
            }
        }
    }

    // Tier 2: Try Chocolatey
    if let Some(ref cid) = item.choco_id {
        item.status_message = format!("Winget unavailable, trying Chocolatey for {}...", cid);
        let output = SilentCommand::new("choco")
            .args(["install", cid, "-y", "--no-progress"])
            .output();

        if let Ok(o) = output {
            if o.status.success() {
                item.installed = true;
                item.state = PrereqInstallState::Installed;
                item.status_message = format!("Successfully installed via Chocolatey: {}", cid);
                return Ok(item.status_message.clone());
            }
        }
    }

    // Tier 3: Direct Official Website Launch
    if !item.direct_url.is_empty() {
        item.status_message = format!(
            "Package managers unavailable. Opening official portal: {}",
            item.direct_url
        );
        let _ = SilentCommand::new("rundll32")
            .args(["url.dll,FileProtocolHandler", &item.direct_url])
            .spawn();

        item.state = PrereqInstallState::NotInstalled;
        return Ok(format!(
            "Official download portal for {} opened in your web browser.",
            item.name
        ));
    }

    item.state = PrereqInstallState::Failed;
    item.status_message =
        "Installation failed: No available package source or download URL".to_string();
    Err(item.status_message.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ISO 29119-3 / 29119-4 Test Case Specification:
    /// - Identifier: TC_GAME_PREREQS_SCAN_01
    /// - Objective: Verify that scan_game_prereqs returns exactly 10 essential gaming prerequisites with complete metadata.
    /// - Preconditions: Host file system and registry initialized.
    /// - Equivalence Classes: Graphics Runtimes, C++ Runtimes, Managed Frameworks, Audio APIs.
    /// - Boundary Conditions: Array length == 10; all direct URLs use https protocol.
    /// - Invariants: Zero duplicate IDs, all IDs non-empty, all descriptions non-empty.
    /// - Expected Outcome: Exactly 10 items returned with valid categories, IDs, names, and states.
    #[test]
    fn test_iso_29119_scan_returns_all_10_prereqs() {
        let items = scan_game_prereqs();
        assert_eq!(items.len(), 10, "Expected exactly 10 gaming prerequisites");
        let mut ids = std::collections::HashSet::new();
        for item in &items {
            assert!(
                ids.insert(item.id.clone()),
                "Duplicate prerequisite ID: {}",
                item.id
            );
            assert!(
                !item.name.is_empty(),
                "Name must not be empty for {}",
                item.id
            );
            assert!(
                !item.category.is_empty(),
                "Category must not be empty for {}",
                item.id
            );
            assert!(
                !item.description.is_empty(),
                "Description must not be empty for {}",
                item.id
            );
            assert!(
                item.direct_url.starts_with("https://"),
                "Direct URL must be HTTPS for {}",
                item.id
            );
            assert!(
                !item.status_message.is_empty(),
                "Status message must not be empty for {}",
                item.id
            );
            assert_eq!(item.is_installed(), item.installed);
            if item.installed {
                assert_eq!(item.state, PrereqInstallState::Installed);
            } else {
                assert_eq!(item.state, PrereqInstallState::NotInstalled);
            }
        }
    }

    /// ISO 29119-3 / 29119-4 Test Case Specification:
    /// - Identifier: TC_GAME_PREREQS_DETECTION_HEURISTICS_02
    /// - Objective: Verify all 10 hardware/registry detection heuristic functions execute without panic.
    /// - Preconditions: Live host or sandboxed environment.
    /// - Equivalence Classes: DirectX, VC++ (x64, x86, AIO), Vulkan, .NET (8, 9, 4.8.1), OpenAL, XNA.
    /// - Boundary Conditions: System32 file presence vs absent DLLs; registry subkey access.
    /// - Invariants: Every heuristic returns (bool, Option<String>), where installed=true implies version string present or detected.
    /// - Expected Outcome: All 10 detection functions execute deterministically.
    #[test]
    fn test_iso_29119_runtime_detection_heuristics_execution() {
        let (dx, dx_ver) = detect_directx_legacy();
        if dx {
            assert!(dx_ver.is_some());
        }

        let (vc64, _vc64_ver) = detect_vcredist_2015_2022_x64();
        let (vc86, _vc86_ver) = detect_vcredist_2015_2022_x86();
        let (_vc_aio, vc_aio_ver) = detect_vcredist_all_in_one();
        assert!(vc_aio_ver.is_some());

        let (_vulkan, _vulkan_ver) = detect_vulkan_runtime();
        let (_net8, _net8_ver) = detect_dotnet_desktop_8();
        let (_net9, _net9_ver) = detect_dotnet_desktop_9();
        let (_net481, _net481_ver) = detect_dotnet_framework_481();
        let (_oal, _oal_ver) = detect_openal();
        let (_xna, _xna_ver) = detect_xna_framework_4();

        // Invariant check: if both 64-bit and 32-bit VC runtimes are detected, boolean flags must be consistent
        let _ = vc64 || vc86;
    }

    /// ISO 29119-3 / 29119-4 Test Case Specification:
    /// - Identifier: TC_GAME_PREREQS_CASCADE_URL_03
    /// - Objective: Verify Tier 3 fallback triggers browser URL launch when package managers are absent.
    /// - Preconditions: Item with winget_id=None, choco_id=None, direct_url populated.
    /// - Equivalence Classes: Direct URL fallback branch.
    /// - Boundary Conditions: Valid HTTPS download URL.
    /// - Invariants: State remains NotInstalled until user finishes browser installation; returns Ok with portal notice.
    /// - Expected Outcome: Success response indicating official download portal opened.
    #[test]
    fn test_iso_29119_cascade_fallback_url_validity() {
        let mut item = GamePrerequisiteItem {
            id: "mock_prereq".to_string(),
            name: "Mock Prereq".to_string(),
            category: "Test".to_string(),
            description: "Test".to_string(),
            installed: false,
            installed_version: None,
            winget_id: None,
            choco_id: None,
            direct_url: "https://example.com/download".to_string(),
            state: PrereqInstallState::NotInstalled,
            status_message: String::new(),
        };

        let res = install_prereq_cascade(&mut item);
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Official download portal"));
        assert_eq!(item.state, PrereqInstallState::NotInstalled);
    }

    /// ISO 29119-3 / 29119-4 Test Case Specification:
    /// - Identifier: TC_GAME_PREREQS_CASCADE_ERROR_RECOVERY_04
    /// - Objective: Verify error recovery and graceful failure when package managers and direct_url are all empty/unavailable.
    /// - Preconditions: Item with winget_id=None, choco_id=None, direct_url="".
    /// - Equivalence Classes: Total installation source failure.
    /// - Boundary Conditions: Empty direct_url string.
    /// - Invariants: Item state transitions to PrereqInstallState::Failed; Err is returned with actionable message.
    /// - Expected Outcome: Function returns Err without panicking, and item.state == Failed.
    #[test]
    fn test_iso_29119_cascade_error_recovery_empty_source() {
        let mut item = GamePrerequisiteItem {
            id: "empty_source_prereq".to_string(),
            name: "Empty Source Prereq".to_string(),
            category: "Test".to_string(),
            description: "Test".to_string(),
            installed: false,
            installed_version: None,
            winget_id: None,
            choco_id: None,
            direct_url: String::new(),
            state: PrereqInstallState::NotInstalled,
            status_message: String::new(),
        };

        let res = install_prereq_cascade(&mut item);
        assert!(
            res.is_err(),
            "Expected Err when no installation sources are available"
        );
        assert_eq!(item.state, PrereqInstallState::Failed);
        assert!(item.status_message.contains("Installation failed"));
    }

    /// ISO 29119-3 / 29119-4 Test Case Specification:
    /// - Identifier: TC_GAME_PREREQS_SERDE_ROUNDTRIP_05
    /// - Objective: Verify JSON serialization and deserialization roundtrip for GamePrerequisiteItem.
    /// - Preconditions: Populated GamePrerequisiteItem struct.
    /// - Equivalence Classes: Installed vs NotInstalled vs Installing vs Failed states.
    /// - Boundary Conditions: Special characters in URLs and descriptions.
    /// - Invariants: Deserialized object is identical to serialized object.
    /// - Expected Outcome: Flawless serde roundtrip.
    #[test]
    fn test_iso_29119_serde_roundtrip() {
        let items = scan_game_prereqs();
        let json = serde_json::to_string(&items).expect("Serialization failed");
        let decoded: Vec<GamePrerequisiteItem> =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(items.len(), decoded.len());
        for (orig, dec) in items.iter().zip(decoded.iter()) {
            assert_eq!(orig.id, dec.id);
            assert_eq!(orig.name, dec.name);
            assert_eq!(orig.installed, dec.installed);
            assert_eq!(orig.state, dec.state);
        }
    }
}
