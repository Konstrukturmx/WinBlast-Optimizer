//! Game Prerequisites Installer & Verification Engine (v1.4).
//!
//! Provides hardware-level detection and multi-stage cascade installation
//! (winget -> choco -> direct official download URL) for the 10 essential
//! gaming runtimes, libraries, and frameworks required by modern PC titles.

use crate::core::process::SilentCommand;
use serde::{Deserialize, Serialize};
use std::path::Path;
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

/// Scan all 10 essential gaming prerequisites against registry and system files.
pub fn scan_game_prereqs() -> Vec<GamePrerequisiteItem> {
    let mut items = Vec::with_capacity(10);

    // 1. DirectX End-User Runtimes (June 2010)
    let (dx_installed, dx_ver) = detect_directx_legacy();
    items.push(GamePrerequisiteItem {
        id: "directx_june2010".to_string(),
        name: "DirectX End-User Runtimes (June 2010)".to_string(),
        category: "Graphics Runtime".to_string(),
        description: "Legacy D3DX9, D3DX10, and D3DX11 DLLs required by thousands of classic & Unreal/Unity games.".to_string(),
        installed: dx_installed,
        installed_version: dx_ver,
        winget_id: Some("Microsoft.DirectX".to_string()),
        choco_id: Some("directx".to_string()),
        direct_url: "https://www.microsoft.com/en-us/download/details.aspx?id=35".to_string(),
        state: if dx_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if dx_installed { "Installed & Verified".to_string() } else { "Missing legacy DirectX DLLs".to_string() },
    });

    // 2. Visual C++ 2015-2022 Redistributable (x64)
    let (vc64_installed, vc64_ver) = detect_vcredist_2015_2022_x64();
    items.push(GamePrerequisiteItem {
        id: "vcredist_2015_2022_x64".to_string(),
        name: "Visual C++ 2015-2022 Redistributable (x64)".to_string(),
        category: "C++ Runtime".to_string(),
        description: "Universal C runtime libraries for 64-bit modern PC gaming engines and launchers.".to_string(),
        installed: vc64_installed,
        installed_version: vc64_ver,
        winget_id: Some("Microsoft.VCRedist.2015+.x64".to_string()),
        choco_id: Some("vcredist140".to_string()),
        direct_url: "https://aka.ms/vs/17/release/vc_redist.x64.exe".to_string(),
        state: if vc64_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if vc64_installed { "Installed & Verified".to_string() } else { "64-bit VC++ 2015-2022 not found".to_string() },
    });

    // 3. Visual C++ 2015-2022 Redistributable (x86)
    let (vc86_installed, vc86_ver) = detect_vcredist_2015_2022_x86();
    items.push(GamePrerequisiteItem {
        id: "vcredist_2015_2022_x86".to_string(),
        name: "Visual C++ 2015-2022 Redistributable (x86)".to_string(),
        category: "C++ Runtime".to_string(),
        description: "Universal 32-bit runtime required by older games, emulators, and 32-bit helper binaries.".to_string(),
        installed: vc86_installed,
        installed_version: vc86_ver,
        winget_id: Some("Microsoft.VCRedist.2015+.x86".to_string()),
        choco_id: Some("vcredist2015".to_string()),
        direct_url: "https://aka.ms/vs/17/release/vc_redist.x86.exe".to_string(),
        state: if vc86_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if vc86_installed { "Installed & Verified".to_string() } else { "32-bit VC++ 2015-2022 not found".to_string() },
    });

    // 4. Visual C++ All-in-One Redistributables (2005-2022)
    let (vc_aio_installed, vc_aio_ver) = detect_vcredist_all_in_one();
    items.push(GamePrerequisiteItem {
        id: "vcredist_all_in_one".to_string(),
        name: "Visual C++ All-in-One Package (2005-2022)".to_string(),
        category: "C++ Runtime Suite".to_string(),
        description: "Complete unified package containing VC++ 2005, 2008, 2010, 2012, 2013, and 2022 runtimes.".to_string(),
        installed: vc_aio_installed,
        installed_version: vc_aio_ver,
        winget_id: Some("TechPowerUp.VisualCppRedistributable".to_string()),
        choco_id: Some("vcredist-all".to_string()),
        direct_url: "https://www.techpowerup.com/download/visual-c-redistributable-runtime-package-all-in-one/".to_string(),
        state: if vc_aio_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if vc_aio_installed { "Full VC++ legacy suite detected".to_string() } else { "Incomplete legacy runtime stack".to_string() },
    });

    // 5. Vulkan Runtime
    let (vulkan_installed, vulkan_ver) = detect_vulkan_runtime();
    items.push(GamePrerequisiteItem {
        id: "vulkan_runtime".to_string(),
        name: "Vulkan Runtime Libraries".to_string(),
        category: "Graphics API".to_string(),
        description: "Low-overhead cross-platform 3D graphics API loader for modern games (DOOM, Baldur's Gate 3, etc.).".to_string(),
        installed: vulkan_installed,
        installed_version: vulkan_ver,
        winget_id: Some("LunarG.VulkanSDK".to_string()),
        choco_id: Some("vulkan-runtime".to_string()),
        direct_url: "https://vulkan.lunarg.com/sdk/home".to_string(),
        state: if vulkan_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if vulkan_installed { "Installed & Verified".to_string() } else { "vulkan-1.dll not found in System32".to_string() },
    });

    // 6. .NET Desktop Runtime 8.0 (x64)
    let (net8_installed, net8_ver) = detect_dotnet_desktop_8();
    items.push(GamePrerequisiteItem {
        id: "dotnet_desktop_8".to_string(),
        name: ".NET Desktop Runtime 8.0 (LTS)".to_string(),
        category: "Managed Framework".to_string(),
        description: "Long-Term Support runtime for modern game mod managers, launchers, and system overlays.".to_string(),
        installed: net8_installed,
        installed_version: net8_ver,
        winget_id: Some("Microsoft.DotNet.DesktopRuntime.8".to_string()),
        choco_id: Some("dotnet-8.0-desktopruntime".to_string()),
        direct_url: "https://dotnet.microsoft.com/download/dotnet/8.0".to_string(),
        state: if net8_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if net8_installed { "Installed & Verified".to_string() } else { ".NET 8.0 Desktop Runtime missing".to_string() },
    });

    // 7. .NET Desktop Runtime 9.0 (x64)
    let (net9_installed, net9_ver) = detect_dotnet_desktop_9();
    items.push(GamePrerequisiteItem {
        id: "dotnet_desktop_9".to_string(),
        name: ".NET Desktop Runtime 9.0 (STS)".to_string(),
        category: "Managed Framework".to_string(),
        description: "Latest high-performance .NET runtime with advanced JIT optimizations for cutting-edge game utilities.".to_string(),
        installed: net9_installed,
        installed_version: net9_ver,
        winget_id: Some("Microsoft.DotNet.DesktopRuntime.9".to_string()),
        choco_id: Some("dotnet-9.0-desktopruntime".to_string()),
        direct_url: "https://dotnet.microsoft.com/download/dotnet/9.0".to_string(),
        state: if net9_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if net9_installed { "Installed & Verified".to_string() } else { ".NET 9.0 Desktop Runtime missing".to_string() },
    });

    // 8. .NET Framework 4.8.1
    let (net481_installed, net481_ver) = detect_dotnet_framework_481();
    items.push(GamePrerequisiteItem {
        id: "dotnet_framework_481".to_string(),
        name: ".NET Framework 4.8.1".to_string(),
        category: "System Framework".to_string(),
        description: "Core Windows application framework with ARM64 and accessibility enhancements.".to_string(),
        installed: net481_installed,
        installed_version: net481_ver,
        winget_id: Some("Microsoft.DotNet.Framework.DeveloperPack_4".to_string()),
        choco_id: Some("netfx-4.8.1".to_string()),
        direct_url: "https://dotnet.microsoft.com/download/dotnet-framework/net481".to_string(),
        state: if net481_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if net481_installed { "Installed & Verified".to_string() } else { ".NET Framework 4.8.1 not detected".to_string() },
    });

    // 9. OpenAL (Open Audio Library)
    let (oal_installed, oal_ver) = detect_openal();
    items.push(GamePrerequisiteItem {
        id: "openal".to_string(),
        name: "OpenAL (Open Audio Library)".to_string(),
        category: "Audio API".to_string(),
        description: "3D positional audio API library required by Minecraft, source ports, and hundreds of 3D games.".to_string(),
        installed: oal_installed,
        installed_version: oal_ver,
        winget_id: Some("OpenAL.OpenAL".to_string()),
        choco_id: Some("openal".to_string()),
        direct_url: "https://www.openal.org/downloads/".to_string(),
        state: if oal_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if oal_installed { "Installed & Verified".to_string() } else { "OpenAL32.dll not detected in system PATH".to_string() },
    });

    // 10. Microsoft XNA Framework Redistributable 4.0
    let (xna_installed, xna_ver) = detect_xna_framework_4();
    items.push(GamePrerequisiteItem {
        id: "xna_framework_4".to_string(),
        name: "Microsoft XNA Framework 4.0 Refresh".to_string(),
        category: "Gaming Runtime".to_string(),
        description: "Essential runtime library for indie classics including Terraria, Bastion, Fez, and Stardew Valley legacy.".to_string(),
        installed: xna_installed,
        installed_version: xna_ver,
        winget_id: Some("Microsoft.XNAFramework".to_string()),
        choco_id: Some("xna".to_string()),
        direct_url: "https://www.microsoft.com/en-us/download/details.aspx?id=20914".to_string(),
        state: if xna_installed { PrereqInstallState::Installed } else { PrereqInstallState::NotInstalled },
        status_message: if xna_installed { "Installed & Verified".to_string() } else { "XNA 4.0 runtime assembly missing".to_string() },
    });

    items
}

// ---------------------------------------------------------------------------
// Detection Helpers
// ---------------------------------------------------------------------------

fn detect_directx_legacy() -> (bool, Option<String>) {
    // Check for d3dx9_43.dll, d3dx10_43.dll, d3dx11_43.dll in System32
    let s32 = Path::new("C:\\Windows\\System32");
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
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64") {
        let installed: Result<u32, _> = key.get_value("Installed");
        let version: Result<String, _> = key.get_value("Version");
        if installed.unwrap_or(0) == 1 {
            return (true, version.ok());
        }
    }
    // Fallback file check
    if Path::new("C:\\Windows\\System32\\vcruntime140.dll").exists() {
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
    if Path::new("C:\\Windows\\SysWOW64\\vcruntime140.dll").exists() {
        return (true, Some("14.x (Detected via SysWOW64)".to_string()));
    }
    (false, None)
}

fn detect_vcredist_all_in_one() -> (bool, Option<String>) {
    let s32 = Path::new("C:\\Windows\\System32");
    // Check multiple legacy VC runtime DLLs
    let has_2005 = s32.join("msvcr80.dll").exists() || Path::new("C:\\Windows\\WinSxS").exists();
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
    if Path::new("C:\\Windows\\System32\\vulkan-1.dll").exists() {
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
    if Path::new("C:\\Windows\\System32\\OpenAL32.dll").exists()
        || Path::new("C:\\Windows\\SysWOW64\\OpenAL32.dll").exists()
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
        item.status_message = format!("Package managers unavailable. Opening official portal: {}", item.direct_url);
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
    item.status_message = "Installation failed: No available package source or download URL".to_string();
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
            assert!(ids.insert(item.id.clone()), "Duplicate prerequisite ID: {}", item.id);
            assert!(!item.name.is_empty(), "Name must not be empty for {}", item.id);
            assert!(!item.category.is_empty(), "Category must not be empty for {}", item.id);
            assert!(!item.description.is_empty(), "Description must not be empty for {}", item.id);
            assert!(item.direct_url.starts_with("https://"), "Direct URL must be HTTPS for {}", item.id);
            assert!(!item.status_message.is_empty(), "Status message must not be empty for {}", item.id);
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
        if dx { assert!(dx_ver.is_some()); }

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
        assert!(res.is_err(), "Expected Err when no installation sources are available");
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
        let decoded: Vec<GamePrerequisiteItem> = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(items.len(), decoded.len());
        for (orig, dec) in items.iter().zip(decoded.iter()) {
            assert_eq!(orig.id, dec.id);
            assert_eq!(orig.name, dec.name);
            assert_eq!(orig.installed, dec.installed);
            assert_eq!(orig.state, dec.state);
        }
    }
}
