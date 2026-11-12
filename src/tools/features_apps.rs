//! Dynamic AppX Bloatware Classification & Removal, DISM Windows Features/Capabilities
//! Management (with serialized CBS safety), Safe OneDrive KFM Check, Edge Removal, and
//! Curated WinGet App Installer.

use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

use crate::core::error::{Result, WinBlastError};
use crate::core::process::SilentCommand;
use crate::core::risk::{FeatureInstallState, OptionalFeatureKind};
use crate::scanner::appx_scanner::AppxCache;

/// Serialize DISM Optional Features / Capabilities operations so Windows CBS TrustedInstaller
/// never collides with `0x800f0823` concurrent servicing errors.
static DISM_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// Package safety tier derived from `optimizerDuck` + `Winhance` + Core OS hard locks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AppxSafetyTier {
    SafeBloat,
    AiBloat,
    Caution,
    ProtectedCore,
}

impl AppxSafetyTier {
    pub const fn as_label(self) -> &'static str {
        match self {
            AppxSafetyTier::SafeBloat => "Safe",
            AppxSafetyTier::AiBloat => "AI / Copilot",
            AppxSafetyTier::Caution => "Caution",
            AppxSafetyTier::ProtectedCore => "Protected",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledAppxPackage {
    pub family_name: String,
    pub short_name: String,
    pub publisher: String,
    pub safety_tier: AppxSafetyTier,
    pub is_provisioned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsOptionalFeature {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub kind: OptionalFeatureKind,
    pub state: FeatureInstallState,
    pub is_curated_bloat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinGetAppEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub description: &'static str,
}

pub const CURATED_WINGET_APPS: &[WinGetAppEntry] = &[
    WinGetAppEntry { id: "7zip.7zip", name: "7-Zip", category: "Utilities", description: "Fast open-source file archiver" },
    WinGetAppEntry { id: "Microsoft.PowerToys", name: "PowerToys", category: "Utilities", description: "Microsoft official power-user utility suite" },
    WinGetAppEntry { id: "voidtools.Everything", name: "Everything Search", category: "Utilities", description: "Instant NTFS filename search engine" },
    WinGetAppEntry { id: "Notepad++.Notepad++", name: "Notepad++", category: "Development", description: "Lightweight text and source code editor" },
    WinGetAppEntry { id: "VideoLAN.VLC", name: "VLC Media Player", category: "Media", description: "Open-source multimedia player" },
    WinGetAppEntry { id: "ShareX.ShareX", name: "ShareX", category: "Utilities", description: "Lightweight screen capture and annotation tool" },
    WinGetAppEntry { id: "OBSProject.OBSStudio", name: "OBS Studio", category: "Media", description: "Screen recording and live streaming studio" },
    WinGetAppEntry { id: "Mozilla.Firefox", name: "Mozilla Firefox", category: "Browsers", description: "Privacy-focused open-source web browser" },
    WinGetAppEntry { id: "Brave.Brave", name: "Brave Browser", category: "Browsers", description: "Chromium browser with built-in ad/tracker shield" },
    WinGetAppEntry { id: "Microsoft.VisualStudioCode", name: "Visual Studio Code", category: "Development", description: "Extensible code editor" },
    WinGetAppEntry { id: "Git.Git", name: "Git", category: "Development", description: "Distributed version control system" },
    WinGetAppEntry { id: "CPUID.HWMonitor", name: "HWMonitor", category: "Diagnostics", description: "Hardware voltage, fan & temperature sensor monitor" },
];

const PROTECTED_CORE_PREFIXES: &[&str] = &[
    "microsoft.sechealthui",
    "microsoft.windowsstore",
    "microsoft.desktopappinstaller",
    "microsoft.storepurchaseapp",
    "microsoft.services.store.engagement",
    "microsoft.vclibs",
    "microsoft.ui.xaml",
    "microsoft.net.native",
    "microsoft.windowsappruntime",
    "microsoft.windows.shellexperiencehost",
    "microsoft.windows.startmenuexperiencehost",
    "microsoft.windows.search",
    "microsoft.windows.cloudexperiencehost",
    "microsoft.windows.apprep.chxapp",
    "microsoft.aad.brokerplugin",
    "microsoft.accountscontrol",
    "microsoft.creddialoghost",
    "microsoft.bioenrollment",
    "microsoft.lockapp",
    "microsoft.win32webviewhost",
    "microsoft.xboxidentityprovider",
    "microsoftwindows.client.core",
    "microsoftwindows.client.cbs",
    "microsoft.heifimageextension",
    "microsoft.webpimageextension",
    "microsoft.vp9videoextensions",
    "microsoft.av1videoextension",
    "microsoft.hevcvideoextension",
];

const AI_BLOAT_PREFIXES: &[&str] = &[
    "microsoft.copilot",
    "microsoft.windows.ai",
    "microsoftwindows.client.aix",
    "microsoftwindows.client.copilot",
    "microsoft.writingassistant",
    "microsoft.edge.gameassist",
    "microsoft.office.actionsserver",
    "microsoftwindows.voiess",
    "microsoftwindows.speion",
    "microsoftwindows.livtop",
    "microsoftwindows.filons",
];

const SAFE_BLOAT_PREFIXES: &[&str] = &[
    "microsoft.bingweather",
    "microsoft.bingnews",
    "microsoft.news",
    "microsoftstart",
    "microsoft.bingfinance",
    "microsoft.bingsports",
    "microsoft.bingfoodanddrink",
    "microsoft.binghealthandfitness",
    "microsoft.bingtravel",
    "microsoft.bingtranslator",
    "microsoft.getstarted",
    "microsoft.windowstips",
    "microsoft.windowsfeedbackhub",
    "microsoft.messaging",
    "microsoft.oneconnect",
    "microsoft.people",
    "microsoft.skypeapp",
    "microsoftteams",
    "msteams",
    "microsoft.onenote",
    "microsoft.todos",
    "microsoft.microsoftofficehub",
    "microsoft.office.onenote",
    "microsoft.office.sway",
    "microsoft.microsoftjournal",
    "microsoft.microsoftpowerbiforwindows",
    "microsoft.wallet",
    "microsoft.mswallet",
    "microsoft.microsoft3dviewer",
    "microsoft.mixedreality.portal",
    "microsoft.print3d",
    "microsoft.3dbuilder",
    "microsoft.paint3d",
    "microsoft.zunevideo",
    "microsoft.windowssoundrecorder",
    "microsoft.stickynotes",
    "microsoft.microsoftstickynotes",
    "microsoft.microsoftsolitairecollection",
    "microsoft.windowsalarms",
    "microsoft.549981c3f5f10",
    "microsoft.windows.devhome",
    "microsoft.windowsmaps",
    "microsoft.xboxapp",
    "microsoft.minecraftuwp",
    "microsoft.networkspeedtest",
    "clipchamp.clipchamp",
    "spotifyab.spotifymusic",
    "disney.",
    "amazon.com.amazon",
    "4df9e0f8.netflix",
    "bytedancepte.ltd.tiktok",
    "king.com.",
    "amazonvideo.primevideo",
    "facebook.",
    "7ee7776c.linkedinforwindows",
    "flipboard",
    "iheartradio",
    "pandoramediainc",
    "tuneinradio",
    "duolingo",
    "picsart",
    "winzipuniversal",
    "adobesystemsincorporated.adobephotoshopexpress",
    "onecalendar",
    "nytcrowssword",
    "marchofempires",
    "asphalt8airborne",
    "farmville2countryescape",
    "hiddencity",
    "caesarsslotsfreecasino",
    "cookingfever",
    "disneymagickingdoms",
    "dolbylaboratories.dolbyaccess",
    "hulu",
    "slingtv",
];

fn is_crockford_pub_id(s: &str) -> bool {
    s.len() == 13
        && s.bytes().all(|b| {
            matches!(
                b.to_ascii_lowercase(),
                b'0'..=b'9' | b'a'..=b'h' | b'j'..=b'k' | b'm'..=b'n' | b'p'..=b't' | b'v'..=b'z'
            )
        })
}

fn is_appx_version(s: &str) -> bool {
    let mut count = 0usize;
    for seg in s.split('.') {
        count += 1;
        if seg.is_empty() || !seg.bytes().all(|b| b.is_ascii_digit()) {
            return false;
        }
    }
    count == 4
}

fn is_appx_arch(s: &str) -> bool {
    matches!(
        s.to_ascii_lowercase().as_str(),
        "x86" | "x64" | "arm" | "arm64" | "neutral" | "arm64x"
    )
}

/// Normalize an AppX PackageFullName (`Name_Ver_Arch_ResId_PubId`) from the right into
/// `PackageFamilyName` (`Name_PubId`), preserving underscores inside `Name` and handling
/// non-empty `ResourceId` tokens cleanly.
pub fn normalize_appx_family_name(raw: &str) -> String {
    let trimmed = raw.trim();
    let parts: Vec<&str> = trimmed.rsplitn(5, '_').collect();
    if parts.len() == 5 {
        let (pub_id, _res_id, arch, ver, name) =
            (parts[0], parts[1], parts[2], parts[3], parts[4]);
        if is_crockford_pub_id(pub_id)
            && is_appx_arch(arch)
            && is_appx_version(ver)
            && !name.is_empty()
        {
            return format!("{}_{}", name, pub_id);
        }
    }
    trimmed.to_string()
}

fn extract_appx_short_name(fam: &str) -> String {
    if let Some((name, pub_id)) = fam.rsplit_once('_') {
        if is_crockford_pub_id(pub_id) {
            return name.to_string();
        }
    }
    fam.split('_').next().unwrap_or(fam).to_string()
}

fn is_microsoft_appx_publisher(fam: &str, short_name: &str) -> bool {
    let pub_id = fam.rsplit_once('_').map(|(_, p)| p).unwrap_or("");
    if pub_id.eq_ignore_ascii_case("8wekyb3d8bbwe")
        || pub_id.eq_ignore_ascii_case("cw5n1h2txyewy")
        || pub_id.eq_ignore_ascii_case("kzf8qxf38zg5c")
    {
        return true;
    }
    let short_lower = short_name.to_ascii_lowercase();
    short_lower.starts_with("microsoft.")
        || short_lower.starts_with("microsoftwindows.")
        || short_lower == "microsoftstart"
        || short_lower == "microsoftteams"
        || short_lower == "msteams"
}

fn query_provisioned_appx_families() -> HashSet<String> {
    let mut provisioned = HashSet::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for sub in [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\Applications",
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\InboxApplications",
    ] {
        if let Ok(key) = hklm.open_subkey_with_flags(sub, KEY_READ | KEY_WOW64_64KEY) {
            for name in key.enum_keys().filter_map(|r| r.ok()) {
                let norm = normalize_appx_family_name(&name).to_ascii_lowercase();
                if !norm.is_empty() {
                    provisioned.insert(norm);
                }
            }
        }
    }
    if let Ok(deprov) = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Appx\\AppxAllUserStore\\Deprovisioned",
        KEY_READ | KEY_WOW64_64KEY,
    ) {
        for name in deprov.enum_keys().filter_map(|r| r.ok()) {
            let norm = normalize_appx_family_name(&name).to_ascii_lowercase();
            provisioned.remove(&norm);
        }
    }
    provisioned
}

/// Classify an AppX package family name into Safe / AI / Caution / ProtectedCore.
pub fn classify_appx_package(family_name: &str) -> AppxSafetyTier {
    let norm = normalize_appx_family_name(family_name);
    let short = extract_appx_short_name(&norm).to_ascii_lowercase();
    if PROTECTED_CORE_PREFIXES.iter().any(|p| short.starts_with(p)) {
        return AppxSafetyTier::ProtectedCore;
    }
    if AI_BLOAT_PREFIXES.iter().any(|p| short.starts_with(p)) {
        return AppxSafetyTier::AiBloat;
    }
    if SAFE_BLOAT_PREFIXES.iter().any(|p| short.starts_with(p)) {
        return AppxSafetyTier::SafeBloat;
    }
    AppxSafetyTier::Caution
}

/// Convert loaded `AppxCache` (fast direct registry AppX scan) into rich deduplicated classified packages
/// using an explicit provisioned package family set (`None` queries live `AppxAllUserStore`).
pub fn list_installed_appx_from_cache_with_provisioned(
    cache: &AppxCache,
    provisioned_override: Option<&HashSet<String>>,
) -> Vec<InstalledAppxPackage> {
    let owned_prov;
    let provisioned = match provisioned_override {
        Some(p) => p,
        None => {
            owned_prov = query_provisioned_appx_families();
            &owned_prov
        }
    };
    let mut seen: HashSet<String> = HashSet::new();
    let mut pkgs: Vec<InstalledAppxPackage> = cache
        .iter()
        .filter_map(|raw| {
            let fam = normalize_appx_family_name(raw);
            let fam_lower = fam.to_ascii_lowercase();
            if fam_lower.is_empty() || !seen.insert(fam_lower.clone()) {
                return None;
            }
            let short_name = extract_appx_short_name(&fam);
            let tier = classify_appx_package(&fam);
            let publisher = if is_microsoft_appx_publisher(&fam, &short_name) {
                "Microsoft".to_string()
            } else {
                "Third-Party / OEM".to_string()
            };
            let is_provisioned = provisioned.contains(&fam_lower);
            Some(InstalledAppxPackage {
                family_name: fam,
                short_name,
                publisher,
                safety_tier: tier,
                is_provisioned,
            })
        })
        .collect();
    pkgs.sort_by(|a, b| a.safety_tier.cmp(&b.safety_tier).then(a.short_name.cmp(&b.short_name)));
    pkgs
}

/// Convert loaded `AppxCache` (fast direct registry AppX scan) into rich deduplicated classified packages.
pub fn list_installed_appx_from_cache(cache: &AppxCache) -> Vec<InstalledAppxPackage> {
    list_installed_appx_from_cache_with_provisioned(cache, None)
}

/// Validate a single AppX package family identifier for injection safety and ProtectedCore refusal.
pub fn validate_appx_family(family: &str) -> Result<&str> {
    let clean = family.trim();
    if clean.is_empty()
        || !clean
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+'))
    {
        return Err(WinBlastError::AppX(format!(
            "Invalid AppX package family identifier: '{}'",
            family
        )));
    }
    if classify_appx_package(clean) == AppxSafetyTier::ProtectedCore {
        return Err(WinBlastError::AppX(format!(
            "Refused to remove protected Windows core package '{}'",
            clean
        )));
    }
    Ok(clean)
}

/// Batch-remove multiple non-protected AppX packages cleanly in ONE PowerShell pipeline (avoiding N+1 spawns!),
/// validating identifier characters against PowerShell injection and optionally deprovisioning for new users.
pub fn remove_appx_batch(families: &[String], remove_provisioned: bool) -> Result<usize> {
    if families.is_empty() {
        return Ok(0);
    }

    let mut valid_fams: Vec<String> = Vec::with_capacity(families.len());
    for f in families {
        let clean = validate_appx_family(f)?;
        valid_fams.push(clean.to_string());
    }

    let joined = valid_fams
        .iter()
        .map(|f| format!("'{}'", f.replace('\'', "")))
        .collect::<Vec<_>>()
        .join(",");

    let prov_block = if remove_provisioned {
        "Get-AppxProvisionedPackage -Online | Where-Object { \
           $pn = $_.PackageName; \
           $candFam = $null; \
           if ($pn -match '^(.+)_\\d+\\.\\d+\\.\\d+\\.\\d+_[^_]+_[^_]*_([0-9a-hjkmnpqrstvwxyzA-HJKMNPQRSTVWXYZ]{13})$') { \
             $candFam = \"$($Matches[1])_$($Matches[2])\"; \
           } elseif ($_.DisplayName) { \
             $pub = ($pn -split '_')[-1]; \
             $candFam = \"$($_.DisplayName)_$pub\"; \
           } \
           $candFam -and $targetSet.Contains($candFam) \
         } | ForEach-Object { \
           $pn = $_.PackageName; \
           $candFam = if ($pn -match '^(.+)_\\d+\\.\\d+\\.\\d+\\.\\d+_[^_]+_[^_]*_([0-9a-hjkmnpqrstvwxyzA-HJKMNPQRSTVWXYZ]{13})$') { \"$($Matches[1])_$($Matches[2])\" } else { \"$($_.DisplayName)_$(($pn -split '_')[-1])\" }; \
           try { Remove-AppxProvisionedPackage -Online -PackageName $pn -AllUsers -ErrorAction Stop | Out-Null; if ($removedFams.Add($candFam)) { $removed++ } } catch { $errors += \"$($pn): $($_.Exception.Message)\" } \
         };"
    } else {
        ""
    };

    let ps_script = format!(
        "$targets = @({joined}); \
         $targetSet = New-Object 'System.Collections.Generic.HashSet[string]' ([System.StringComparer]::OrdinalIgnoreCase); \
         foreach ($t in $targets) {{ [void]$targetSet.Add($t) }}; \
         $removedFams = New-Object 'System.Collections.Generic.HashSet[string]' ([System.StringComparer]::OrdinalIgnoreCase); \
         $removed = 0; $errors = @(); \
         foreach ($pf in $targets) {{ \
           $pkgs = Get-AppxPackage -AllUsers | Where-Object {{ $_.PackageFamilyName -eq $pf }}; \
           foreach ($p in $pkgs) {{ \
             try {{ Remove-AppxPackage -Package $p.PackageFullName -AllUsers -ErrorAction Stop; $removed++; [void]$removedFams.Add($pf) }} catch {{ $errors += \"$($pf): $($_.Exception.Message)\" }} \
           }} \
         }}; \
         {prov_block} \
         if ($errors.Count -gt 0) {{ Write-Error ($errors -join '; '); exit 1 }} \
         Write-Output \"REMOVED:$removed\"",
    );

    let out = SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_script])
        .output()
        .map_err(|e| WinBlastError::AppX(format!("Failed to launch PowerShell: {}", e)))?;

    let stdout = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
    let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
    if !out.status.success() {
        let msg = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        return Err(WinBlastError::AppX(format!("AppX removal error: {}", msg)));
    }
    let removed = stdout
        .lines()
        .find_map(|l| l.trim().strip_prefix("REMOVED:"))
        .and_then(|n| n.trim().parse::<usize>().ok())
        .unwrap_or(0);
    if removed == 0 {
        return Err(WinBlastError::AppX(
            "No matching installed or provisioned AppX packages were removed.".to_string(),
        ));
    }
    Ok(removed)
}

/// Curated list of Winhance's Windows Optional Features & Capabilities supported cleanly via DISM.
pub fn curated_windows_features_meta() -> Vec<(&'static str, &'static str, &'static str, OptionalFeatureKind, bool)> {
    vec![
        ("Recall", "Windows Recall AI (Optional Feature)", "Screenshot memory indexer on Copilot+ PCs (24H2+).", OptionalFeatureKind::OptionalFeature, true),
        ("Containers-DisposableClientVM", "Windows Sandbox", "Throwaway isolated desktop VM for testing untrusted apps.", OptionalFeatureKind::OptionalFeature, false),
        ("Microsoft-Windows-Subsystem-Linux", "Windows Subsystem for Linux (WSL)", "Run native Linux environments inside Windows.", OptionalFeatureKind::OptionalFeature, false),
        ("Microsoft-Hyper-V-All", "Hyper-V Virtualization", "Type-1 hypervisor platform and VM management tools.", OptionalFeatureKind::OptionalFeature, false),
        ("NetFx3", ".NET Framework 3.5", "Legacy .NET 2.0/3.0/3.5 runtime compatibility.", OptionalFeatureKind::OptionalFeature, false),
        ("Browser.InternetExplorer~~~~0.0.11.0", "Internet Explorer (Legacy Capability)", "Legacy Trident web browser shell.", OptionalFeatureKind::Capability, true),
        ("App.StepsRecorder~~~~0.0.1.0", "Steps Recorder", "Legacy problem-step screenshot recorder.", OptionalFeatureKind::Capability, true),
        ("Microsoft.Windows.WordPad~~~~0.0.1.0", "WordPad (Legacy)", "Legacy RTF/DOC editor capability.", OptionalFeatureKind::Capability, true),
        ("Microsoft.Windows.PowerShell.ISE~~~~0.0.1.0", "PowerShell ISE", "Legacy PowerShell script IDE.", OptionalFeatureKind::Capability, true),
        ("OpenSSH.Client~~~~0.0.1.0", "OpenSSH Client", "Built-in SSH CLI client.", OptionalFeatureKind::Capability, false),
    ]
}

fn parse_dism_english_state(raw: &str, is_cap: bool) -> FeatureInstallState {
    let s = raw.trim().to_ascii_lowercase();
    if is_cap {
        if s == "installed" {
            FeatureInstallState::Enabled
        } else if s.contains("install pending") {
            FeatureInstallState::EnablePending
        } else if s.contains("uninstall pending") || s.contains("remove pending") {
            FeatureInstallState::DisablePending
        } else {
            FeatureInstallState::Disabled
        }
    } else if s == "enabled" {
        FeatureInstallState::Enabled
    } else if s.contains("enable pending") {
        FeatureInstallState::EnablePending
    } else if s.contains("disable pending") {
        FeatureInstallState::DisablePending
    } else {
        FeatureInstallState::Disabled
    }
}

fn dism_error_from_output(op: &str, out: &std::process::Output) -> WinBlastError {
    let code = out.status.code().unwrap_or(-1);
    let stdout = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
    let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
    let detail = stderr
        .lines()
        .chain(stdout.lines())
        .map(str::trim)
        .rfind(|l| !l.is_empty())
        .unwrap_or("");
    if code == 740 {
        return WinBlastError::ElevationRequired(format!(
            "DISM {} requires Administrator privileges (exit 740): {}",
            op, detail
        ));
    }
    WinBlastError::TweakFailed(format!("DISM {} exited with code {}: {}", op, code, detail))
}

fn query_dism_features_locked() -> Result<HashMap<String, FeatureInstallState>> {
    let out = SilentCommand::new("dism")
        .args(["/English", "/Online", "/Get-Features", "/Format:List"])
        .output()
        .map_err(|e| WinBlastError::TweakFailed(format!("DISM spawn failed: {}", e)))?;

    if !out.status.success() {
        return Err(dism_error_from_output("/Get-Features", &out));
    }

    let mut feature_states = HashMap::new();
    let txt = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
    let mut cur_name = String::new();
    for line in txt.lines() {
        if let Some((lhs, rhs)) = line.split_once(':') {
            let key = lhs.trim();
            let val = rhs.trim();
            if key.eq_ignore_ascii_case("Feature Name") {
                cur_name = val.to_ascii_lowercase();
            } else if key.eq_ignore_ascii_case("State") && !cur_name.is_empty() {
                feature_states.insert(
                    std::mem::take(&mut cur_name),
                    parse_dism_english_state(val, false),
                );
            }
        }
    }
    Ok(feature_states)
}

fn query_dism_capabilities_locked() -> Result<Vec<(String, FeatureInstallState)>> {
    let out = SilentCommand::new("dism")
        .args(["/English", "/Online", "/Get-Capabilities", "/Format:List"])
        .output()
        .map_err(|e| WinBlastError::TweakFailed(format!("DISM spawn failed: {}", e)))?;

    if !out.status.success() {
        return Err(dism_error_from_output("/Get-Capabilities", &out));
    }

    let mut caps = Vec::new();
    let txt = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
    let mut cur_cap = String::new();
    for line in txt.lines() {
        if let Some((lhs, rhs)) = line.split_once(':') {
            let key = lhs.trim();
            let val = rhs.trim();
            if key.eq_ignore_ascii_case("Capability Identity") {
                cur_cap = val.to_string();
            } else if key.eq_ignore_ascii_case("State") && !cur_cap.is_empty() {
                caps.push((
                    std::mem::take(&mut cur_cap),
                    parse_dism_english_state(val, true),
                ));
            }
        }
    }
    Ok(caps)
}

/// Probe curated Optional Features & Capabilities cleanly via DISM (`/English`, using single lock).
/// Surfaces DISM failures (e.g. exit 740 / non-elevated) as explicit `Err` rather than masking as `Disabled`.
pub fn list_optional_features() -> Result<Vec<WindowsOptionalFeature>> {
    let _guard = DISM_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let feature_states = query_dism_features_locked()?;
    let caps = query_dism_capabilities_locked()?;
    let cap_states: HashMap<String, FeatureInstallState> = caps
        .into_iter()
        .map(|(id, st)| (id.to_ascii_lowercase(), st))
        .collect();

    let mut out_list = Vec::new();
    for (name, display, desc, kind, bloat) in curated_windows_features_meta() {
        let lower = name.to_ascii_lowercase();
        let state = match kind {
            OptionalFeatureKind::OptionalFeature => feature_states
                .get(&lower)
                .copied()
                .unwrap_or(FeatureInstallState::Disabled),
            OptionalFeatureKind::Capability => {
                let prefix = lower.split("~~~~").next().unwrap_or(&lower);
                cap_states
                    .get(&lower)
                    .copied()
                    .or_else(|| {
                        cap_states
                            .iter()
                            .find(|(k, _)| {
                                k.split("~~~~").next().unwrap_or(k.as_str()) == prefix
                            })
                            .map(|(_, st)| *st)
                    })
                    .unwrap_or(FeatureInstallState::Disabled)
            }
        };
        out_list.push(WindowsOptionalFeature {
            name: name.to_string(),
            display_name: display.to_string(),
            description: desc.to_string(),
            kind,
            state,
            is_curated_bloat: bloat,
        });
    }
    Ok(out_list)
}

/// Validate a Windows Optional Feature / Capability name against unsafe characters.
pub fn validate_feature_name(name: &str) -> Result<&str> {
    let clean = name.trim();
    if clean.is_empty()
        || !clean
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '~' | '_'))
    {
        return Err(WinBlastError::TweakFailed(format!(
            "Invalid feature/capability name: {}",
            name
        )));
    }
    Ok(clean)
}

/// Resolve a shorthand capability name (e.g. `"App.StepsRecorder"`) against live DISM
/// `Capability Identity` entries first, falling back to curated metadata.
fn resolve_capability_identity_locked(name: &str) -> String {
    if name.contains("~~~~") {
        return name.to_string();
    }
    let req_prefix = name.split("~~~~").next().unwrap_or(name);
    if let Ok(live_caps) = query_dism_capabilities_locked() {
        if let Some((identity, _)) = live_caps.into_iter().find(|(id, _)| {
            let cap_prefix = id
                .split("~~~~")
                .next()
                .and_then(|s| s.split("~~~").next())
                .unwrap_or(id.as_str());
            cap_prefix.eq_ignore_ascii_case(req_prefix) || id.eq_ignore_ascii_case(name)
        }) {
            return identity;
        }
    }
    for (cur_name, _, _, cur_kind, _) in curated_windows_features_meta() {
        if cur_kind == OptionalFeatureKind::Capability {
            let cur_prefix = cur_name.split("~~~~").next().unwrap_or(cur_name);
            if cur_prefix.eq_ignore_ascii_case(req_prefix) {
                return cur_name.to_string();
            }
        }
    }
    name.to_string()
}

/// Enable or disable a Windows Optional Feature or Capability via serialized DISM execution.
pub fn set_optional_feature(
    name: &str,
    kind: OptionalFeatureKind,
    enable: bool,
) -> Result<FeatureInstallState> {
    let clean = validate_feature_name(name)?;
    let _guard = DISM_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let resolved_name = if kind == OptionalFeatureKind::Capability {
        resolve_capability_identity_locked(clean)
    } else {
        clean.to_string()
    };

    let out = match (kind, enable) {
        (OptionalFeatureKind::OptionalFeature, true) => SilentCommand::new("dism")
            .args([
                "/English",
                "/Online",
                "/Enable-Feature",
                &format!("/FeatureName:{}", resolved_name),
                "/All",
                "/NoRestart",
            ])
            .output(),
        (OptionalFeatureKind::OptionalFeature, false) => SilentCommand::new("dism")
            .args([
                "/English",
                "/Online",
                "/Disable-Feature",
                &format!("/FeatureName:{}", resolved_name),
                "/NoRestart",
            ])
            .output(),
        (OptionalFeatureKind::Capability, true) => SilentCommand::new("dism")
            .args([
                "/English",
                "/Online",
                "/Add-Capability",
                &format!("/CapabilityName:{}", resolved_name),
                "/NoRestart",
            ])
            .output(),
        (OptionalFeatureKind::Capability, false) => SilentCommand::new("dism")
            .args([
                "/English",
                "/Online",
                "/Remove-Capability",
                &format!("/CapabilityName:{}", resolved_name),
                "/NoRestart",
            ])
            .output(),
    }
    .map_err(|e| WinBlastError::TweakFailed(format!("DISM spawn failed: {}", e)))?;

    let code = out.status.code().unwrap_or(-1);
    if out.status.success() {
        return Ok(if enable {
            FeatureInstallState::Enabled
        } else {
            FeatureInstallState::Disabled
        });
    }
    // ERROR_SUCCESS_REBOOT_REQUIRED = 3010
    if code == 3010 {
        return Ok(if enable {
            FeatureInstallState::EnablePending
        } else {
            FeatureInstallState::DisablePending
        });
    }

    Err(dism_error_from_output(&resolved_name, &out))
}

/// Check if OneDrive Known Folder Move (KFM) currently redirects Desktop/Documents/Pictures into OneDrive.
pub fn check_onedrive_kfm_active() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(k) = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\User Shell Folders",
        KEY_READ | KEY_WOW64_64KEY,
    ) {
        let folder_keys = [
            ("Desktop", "Desktop"),
            ("Personal", "Documents"),
            ("My Pictures", "Pictures"),
            ("{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", "Desktop (GUID)"),
            ("{754AC886-DF64-4CBA-86B5-F7FBF4FBCEF5}", "Desktop (Backup GUID)"),
            ("{F42EE2D3-909F-4907-8871-4C22FC0BF756}", "Documents (GUID)"),
            ("{FDD39AD0-238F-46AF-ADB4-6C85480369C7}", "Documents (Backup GUID)"),
            ("{0DDD015D-B06C-45D5-8C4C-F59713854639}", "Pictures (GUID)"),
            ("{33E28130-4E1E-4676-835A-98395C3BC3BB}", "Pictures (Backup GUID)"),
        ];
        for (val_name, label) in folder_keys {
            if let Ok(path) = k.get_value::<String, _>(val_name) {
                if path.to_ascii_lowercase().contains("onedrive") {
                    return Some(format!("{} -> {}", label, path));
                }
            }
        }
    }

    if let Ok(accounts) = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\OneDrive\\Accounts",
        KEY_READ | KEY_WOW64_64KEY,
    ) {
        for acct in accounts.enum_keys().filter_map(|r| r.ok()) {
            if let Ok(ak) = accounts.open_subkey_with_flags(&acct, KEY_READ | KEY_WOW64_64KEY) {
                let kfm_mask: u32 = ak.get_value("KfmFoldersProtectedNow").unwrap_or(0);
                if kfm_mask != 0 {
                    return Some(format!(
                        "OneDrive Account '{}' KfmFoldersProtectedNow=0x{:X}",
                        acct, kfm_mask
                    ));
                }
            }
        }
    }
    None
}

fn find_onedrive_setup_exe() -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    let mut installed_roots = Vec::new();
    if let Ok(la) = std::env::var("LOCALAPPDATA") {
        if !la.trim().is_empty() {
            installed_roots.push(PathBuf::from(la).join("Microsoft").join("OneDrive"));
        }
    }
    if let Ok(pf) = std::env::var("ProgramFiles") {
        if !pf.trim().is_empty() {
            installed_roots.push(PathBuf::from(pf).join("Microsoft OneDrive"));
        }
    }
    if let Ok(pf86) = std::env::var("ProgramFiles(x86)") {
        if !pf86.trim().is_empty() {
            installed_roots.push(PathBuf::from(pf86).join("Microsoft OneDrive"));
        }
    }

    // Probe active installed OneDrive locations (<root>\OneDriveSetup.exe & <root>\<ver>\OneDriveSetup.exe)
    // BEFORE inbox %WINDIR%\SysWOW64 / System32 stubs.
    for root in installed_roots {
        let direct = root.join("OneDriveSetup.exe");
        if direct.is_file() {
            return Some(direct);
        }
        if let Ok(rd) = std::fs::read_dir(&root) {
            let mut sub_setups: Vec<PathBuf> = rd
                .flatten()
                .map(|ent| ent.path().join("OneDriveSetup.exe"))
                .filter(|p| p.is_file())
                .collect();
            sub_setups.sort();
            if let Some(latest) = sub_setups.pop() {
                return Some(latest);
            }
        }
    }

    let sys_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    [
        PathBuf::from(&sys_root).join("SysWOW64").join("OneDriveSetup.exe"),
        PathBuf::from(&sys_root).join("System32").join("OneDriveSetup.exe"),
    ]
    .into_iter()
    .find(|stub| stub.is_file())
}

/// Safely uninstall OneDrive only if Known Folder Move (KFM) redirection is NOT active.
pub fn uninstall_onedrive_safe() -> Result<String> {
    if let Some(active_kfm) = check_onedrive_kfm_active() {
        return Err(WinBlastError::TweakFailed(format!(
            "Blocked OneDrive uninstall: Known Folder Move redirects user files ({}). Unlink OneDrive folder backup first.",
            active_kfm
        )));
    }
    let setup = find_onedrive_setup_exe().ok_or_else(|| {
        WinBlastError::TweakFailed(
            "OneDriveSetup.exe not found (OneDrive already removed).".to_string(),
        )
    })?;

    let _ = SilentCommand::new("taskkill")
        .args(["/F", "/IM", "OneDrive.exe"])
        .output();
    let out = SilentCommand::new(&setup)
        .args(["/uninstall"])
        .output()
        .map_err(|e| WinBlastError::TweakFailed(format!("OneDriveSetup failed: {}", e)))?;

    if !out.status.success() {
        let stdout = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
        return Err(WinBlastError::TweakFailed(format!(
            "OneDriveSetup /uninstall failed (exit {:?}): {} {}",
            out.status.code(),
            stdout.trim(),
            stderr.trim()
        )));
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run_key) = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_READ | KEY_SET_VALUE | KEY_WOW64_64KEY,
    ) {
        let _ = run_key.delete_value("OneDrive");
        let _ = run_key.delete_value("OneDriveSetup");
    }

    Ok("OneDrive uninstalled cleanly without KFM data loss.".to_string())
}

/// Non-destructively remove Microsoft Edge (preserving WebView2 Runtime) & block Edge reinstall updates.
pub fn remove_edge_preserve_webview2() -> Result<String> {
    // Set EdgeUpdate DoNotUpdateToEdgeWithChromium in both 64-bit & 32-bit (WOW6432Node) views
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    for wow in [KEY_WOW64_64KEY, KEY_WOW64_32KEY] {
        if let Ok((k, _)) =
            hklm.create_subkey_with_flags("SOFTWARE\\Microsoft\\EdgeUpdate", KEY_READ | KEY_WRITE | wow)
        {
            let _ = k.set_value("DoNotUpdateToEdgeWithChromium", &1u32);
        }
    }

    let script = r#"
$baseDirs = @(${env:ProgramFiles(x86)}, $env:ProgramFiles, $env:LOCALAPPDATA) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
$roots = $baseDirs | ForEach-Object { [System.IO.Path]::Combine($_, 'Microsoft\Edge\Application') } | Select-Object -Unique
$foundAny = $false
$failed = @()
Get-Process msedge -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
foreach ($r in $roots) {
  if (!(Test-Path $r)) { continue }
  $isUser = $env:LOCALAPPDATA -and $r.StartsWith($env:LOCALAPPDATA, [System.StringComparison]::OrdinalIgnoreCase)
  $setups = Get-ChildItem -Path $r -Filter "setup.exe" -Recurse -ErrorAction SilentlyContinue
  if (-not $setups -or $setups.Count -eq 0) { continue }
  $foundAny = $true
  foreach ($s in $setups) {
    if (!(Test-Path $r) -or !(Test-Path $s.FullName)) { break }
    $argStr = if ($isUser) { "--uninstall --verbose-logging --force-uninstall" } else { "--uninstall --system-level --verbose-logging --force-uninstall" }
    $proc = Start-Process -FilePath $s.FullName -ArgumentList $argStr -Wait -PassThru
    if ($proc.ExitCode -ne 0 -and $proc.ExitCode -ne 19) {
      $failed += "Exit $($proc.ExitCode) ($($s.FullName))"
    } elseif (!(Test-Path $r) -or -not (Get-ChildItem -Path $r -Filter "msedge.exe" -Recurse -ErrorAction SilentlyContinue)) {
      break
    }
  }
}
if (-not $foundAny) {
  Write-Output "NOT_FOUND"
  exit 0
}
if ($failed.Count -gt 0) {
  Write-Error ($failed -join '; ')
  exit 1
}
"#;
    let out = SilentCommand::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| WinBlastError::AppX(format!("Failed Edge uninstall script: {}", e)))?;

    if !out.status.success() {
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
        let stdout = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
        let detail = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };
        return Err(WinBlastError::AppX(format!("Edge uninstall error: {}", detail)));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    if stdout.trim() == "NOT_FOUND" {
        return Ok("Microsoft Edge browser installation not found; EdgeUpdate re-install policy locked.".to_string());
    }
    Ok("Microsoft Edge uninstalled while preserving WebView2 runtime.".to_string())
}

/// Validate a WinGet package identifier before invoking `winget install --id`.
pub fn validate_winget_package_id(package_id: &str) -> Result<&str> {
    let clean = package_id.trim();
    if clean.is_empty()
        || !clean
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
    {
        return Err(WinBlastError::AppX(format!("Invalid WinGet ID: {}", package_id)));
    }
    Ok(clean)
}

fn strip_ansi_sequences(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek().copied() {
                Some('[') => {
                    chars.next();
                    for nc in chars.by_ref() {
                        if nc.is_ascii() && (0x40..=0x7E).contains(&(nc as u8)) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    while let Some(nc) = chars.next() {
                        if nc == '\x07' {
                            break;
                        }
                        if nc == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn is_progress_noise_char(c: char) -> bool {
    matches!(
        c,
        '█' | '▒' | '░' | '▓' | '▌' | '▐' | '▀' | '▄' | '■' | '□' | '-' | '\\' | '|' | '/'
    ) || c.is_whitespace()
}

pub(crate) fn extract_winget_error_detail(stderr: &str, stdout: &str) -> String {
    let clean_err = strip_ansi_sequences(stderr);
    let clean_out = strip_ansi_sequences(stdout);
    clean_err
        .split(['\r', '\n'])
        .chain(clean_out.split(['\r', '\n']))
        .map(|l| {
            l.trim_matches(|c: char| is_progress_noise_char(c))
                .trim()
        })
        .rfind(|l| !l.is_empty())
        .unwrap_or("")
        .to_string()
}

/// Install a curated Windows software package silently via WinGet CLI.
pub fn winget_install_package(package_id: &str) -> Result<String> {
    let clean = validate_winget_package_id(package_id)?;
    let out = SilentCommand::new("winget")
        .args([
            "install",
            "--id",
            clean,
            "-e",
            "--silent",
            "--disable-interactivity",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ])
        .output()
        .map_err(|e| WinBlastError::AppX(format!("WinGet CLI unavailable: {}", e)))?;

    let code_u32 = out.status.code().map(|c| c as u32);
    if out.status.success() {
        Ok(format!("Installed {}", clean))
    } else if code_u32 == Some(0x8A15_002B) {
        Ok(format!("{} is already up to date", clean))
    } else if code_u32 == Some(0x8A15_0061) {
        Ok(format!("{} is already installed", clean))
    } else {
        let stdout = crate::scanner::task_scanner::decode_oem_bytes(&out.stdout);
        let stderr = crate::scanner::task_scanner::decode_oem_bytes(&out.stderr);
        let detail = extract_winget_error_detail(&stderr, &stdout);
        Err(WinBlastError::AppX(format!(
            "WinGet {} exit {:?}: {}",
            clean,
            out.status.code(),
            detail
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_core_appx_never_classified_bloat() {
        assert_eq!(classify_appx_package("Microsoft.SecHealthUI_8wekyb3d8bbwe"), AppxSafetyTier::ProtectedCore);
        assert_eq!(classify_appx_package("Microsoft.WindowsStore_8wekyb3d8bbwe"), AppxSafetyTier::ProtectedCore);
        assert_eq!(classify_appx_package("Microsoft.VCLibs.140.00_8wekyb3d8bbwe"), AppxSafetyTier::ProtectedCore);
        assert_eq!(classify_appx_package("Microsoft.BingWeather_8wekyb3d8bbwe"), AppxSafetyTier::SafeBloat);
        assert_eq!(classify_appx_package("Microsoft.Copilot_8wekyb3d8bbwe"), AppxSafetyTier::AiBloat);
    }
}
