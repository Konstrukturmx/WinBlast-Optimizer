//! Risk calculation matrix, tweak definitions, and the declarative TOML rule loader.
//!
//! Every optimization in WinBlast0 is modelled as a [`TweakDefinition`] scored by
//! a multi-axis [`RiskValuation`].  Tweaks are stored in TOML catalog files under
//! `assets/rules/` and loaded at startup via [`load_rules_from_dir`].

use serde::{Deserialize, Serialize};

/// Severity level indicating how likely a tweak is to destabilize the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Zero side effects, instant revert (Easy Mode eligible).
    Safe = 1,
    /// Minor UX change (e.g., disables animations or tips).
    Low = 2,
    /// May affect specific workflows (e.g., disables Xbox Game Bar).
    Moderate = 3,
    /// Alters core OS scheduling/security (Advanced Mode only).
    High = 4,
    /// Deep stripping; requires reboot and may break Windows Update.
    Extreme = 5,
}

/// How the tweak can be undone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reversibility {
    /// Can be reverted immediately with a registry write or service change.
    Instant,
    /// Revert takes effect after a reboot.
    RequiresReboot,
    /// Revert takes effect after signing out and back in.
    RequiresLogoff,
    /// The removed AppX/MSIX package must be re-installed from the Store.
    RequiresPackageReinstall,
}

/// Multi-axis risk assessment attached to every tweak.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskValuation {
    /// Stability impact severity.
    pub stability_risk: RiskLevel,
    /// Performance improvement score (1–5).
    pub performance_gain: u8,
    /// Privacy improvement score (1–5).
    pub privacy_gain: u8,
    /// How the tweak can be reverted.
    pub reversibility: Reversibility,
    /// List of Windows features that will stop working.
    pub broken_features: Vec<String>,
}

/// Minimum OS requirements for a tweak to be applicable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsRequirement {
    /// Minimum NT build number (e.g., 19041 for Win10 2004).
    pub min_build: u32,
    /// Optional upper-bound build number.
    pub max_build: Option<u32>,
    /// Applicable editions (empty = all editions).
    pub editions: Vec<String>,
}

fn default_semicolon() -> String {
    ";".to_string()
}

fn default_equals() -> String {
    "=".to_string()
}

fn default_true() -> bool {
    true
}

/// Formula for dynamic registry DWORD computation based on live hardware telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicDwordFormula {
    TotalRamInKb,
    TotalRamInKbPlusMargin,
    LogicalCoreCount,
}

/// Power plan setting AC/DC target scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PowerTargetScope {
    AcOnly,
    DcOnly,
    #[default]
    AcAndDc,
}

/// DISM feature classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptionalFeatureKind {
    OptionalFeature,
    Capability,
}

/// DISM feature installation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureInstallState {
    Enabled,
    Disabled,
    EnablePending,
    DisablePending,
    Removed,
}

/// Normalize unnamed `(Default)` registry value aliases (`""`, `"@"`, and `"(Default)"` -> `""`).
pub fn normalize_reg_value_name(name: &str) -> &str {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed == "@" || trimmed.eq_ignore_ascii_case("(default)") {
        ""
    } else {
        trimmed
    }
}

/// Human-readable display name for unnamed `(Default)` registry values.
pub fn display_reg_value_name(name: &str) -> &str {
    if normalize_reg_value_name(name).is_empty() {
        "(Default)"
    } else {
        name.trim()
    }
}

/// A single atomic mutation that a tweak performs on the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TweakAction {
    /// Set (or restore) a DWORD registry value.
    RegistryDword {
        /// Registry hive name (e.g., "HKLM", "HKCU").
        hive: String,
        /// Subkey path.
        path: String,
        /// Value name.
        name: String,
        /// Value to write when optimizing.
        optimized_value: u32,
        /// Original Windows default value.
        default_value: u32,
    },
    /// Set, clear, or remove a string (`REG_SZ`/`REG_EXPAND_SZ`) registry value.
    RegistryString {
        hive: String,
        path: String,
        /// Value name (`""` or `"(Default)"` targets the key's unnamed default value).
        name: String,
        optimized_value: String,
        /// Original Windows default string (`"__ABSENT__"` deletes the value on revert).
        default_value: String,
        #[serde(default)]
        expand_sz: bool,
        #[serde(default)]
        absent_is_default: bool,
        #[serde(default)]
        case_insensitive: bool,
    },
    /// Delete a specific registry value (with lossless typed prior backup in StateLedger).
    RegistryDelete {
        hive: String,
        path: String,
        name: String,
    },
    /// Delete a guarded registry subkey tree (with lossless tree snapshot backup in StateLedger).
    RegistryDeleteTree { hive: String, path: String },
    /// Atomically update a single bit-mask inside a `REG_BINARY` value (e.g. `UserPreferencesMask`).
    RegistryBinaryBit {
        hive: String,
        path: String,
        name: String,
        byte_index: usize,
        min_length: usize,
        bit_mask: u8,
        optimized_bits: u8,
        default_bits: u8,
        #[serde(default)]
        create_if_missing: bool,
        #[serde(default)]
        default_full_hex: Option<String>,
    },
    /// Merge a single `Key=Value` pair inside a delimited string (e.g. `DirectXUserGlobalSettings`).
    CompositeStringKey {
        hive: String,
        path: String,
        name: String,
        entry_key: String,
        optimized_entry_value: String,
        default_entry_value: String,
        #[serde(default = "default_semicolon")]
        pair_separator: String,
        #[serde(default = "default_equals")]
        kv_separator: String,
        #[serde(default = "default_true")]
        trailing_separator: bool,
    },
    /// Set a DWORD registry value derived dynamically from live hardware telemetry.
    RegistryDynamicDword {
        hive: String,
        path: String,
        name: String,
        formula: DynamicDwordFormula,
        default_value: u32,
        #[serde(default)]
        tolerance_pct: u8,
    },
    /// Update an active power scheme setting via `powrprof.dll` (AC/DC index + immediate activation).
    PowerCfgSetting {
        subgroup_guid: String,
        setting_guid: String,
        #[serde(default)]
        scheme_guid: Option<String>,
        optimized_ac: u32,
        optimized_dc: u32,
        default_ac: u32,
        default_dc: u32,
        #[serde(default)]
        scope: PowerTargetScope,
        #[serde(default)]
        unhide_attribute: bool,
    },
    /// Change a Windows service's start type.
    ServiceState {
        /// Internal service name (e.g., "DiagTrack").
        service_name: String,
        /// Desired start type (4 = Disabled, 3 = Manual, 2 = Auto).
        target_start_type: u32,
        /// Original start type.
        default_start_type: u32,
    },
    /// Enable or disable a scheduled task.
    ScheduledTask {
        /// Full task path (e.g., `\Microsoft\Windows\...`).
        task_path: String,
        /// `true` to enable, `false` to disable.
        enable: bool,
    },
    /// Remove an AppX/MSIX package family.
    AppXPackage {
        /// Package family name (e.g., "Microsoft.BingWeather_8wekyb3d8bbwe").
        package_family: String,
    },
    /// Toggle a Windows Optional Feature or Capability via DISM.
    WindowsFeature {
        feature_name: String,
        kind: OptionalFeatureKind,
        enable: bool,
    },
}

/// Complete definition of a single optimization tweak.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakDefinition {
    /// Unique identifier (e.g., "win32_priority_separation_short_fixed").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Grouping category (e.g., "CPU & Scheduling", "Privacy").
    pub category: String,
    /// Detailed description of what this tweak does.
    pub description: String,
    /// Whether this tweak is included in Easy Mode presets.
    pub easy_mode_included: bool,
    /// Minimum OS requirements.
    pub os_req: OsRequirement,
    /// Risk and benefit assessment.
    pub valuation: RiskValuation,
    /// Ordered list of system mutations.
    pub actions: Vec<TweakAction>,
    /// Optional GPU silicon vendor requirement ("NVIDIA", "AMD", "Intel").
    #[serde(default)]
    pub gpu_vendor: Option<String>,
    /// Optional sub-category or functional tag (e.g., "Power", "Performance").
    #[serde(default)]
    pub sub_category: Option<String>,
    /// Whether this tweak is safe to emit into offline Windows Setup `autounattend.xml`.
    #[serde(default)]
    pub unattend_safe: bool,
    /// Optional provenance tag (e.g., `"Winhance"`, `"optimizerDuck"`).
    #[serde(default)]
    pub source_attribution: Option<String>,
    /// Fallback execution strategies if primary actions fail.
    #[serde(default)]
    pub fallbacks: Vec<Vec<TweakAction>>,
    /// Prerequisites that must be applied before this tweak.
    #[serde(default)]
    pub prerequisites: Vec<String>,
    /// Mutual exclusion or conflicting tweak IDs.
    #[serde(default)]
    pub conflicts_with: Vec<String>,
}

/// Container for deserializing TOML rule files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweakCatalog {
    /// All tweak definitions in this catalog file.
    pub tweaks: Vec<TweakDefinition>,
}

/// Helper to check if a detected Windows edition matches a rule requirement.
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

    // Education
    if rule == "education" && detected.contains("education") {
        return true;
    }

    false
}

impl TweakDefinition {
    /// Check if this tweak is compatible with the given OS build and edition.
    pub fn is_compatible(&self, build: u32, edition: &str) -> bool {
        if build < self.os_req.min_build {
            return false;
        }
        if let Some(max) = self.os_req.max_build {
            if build > max {
                return false;
            }
        }
        self.os_req.editions.is_empty()
            || self
                .os_req
                .editions
                .iter()
                .any(|e| matches_edition(e, edition))
    }

    /// Check if this tweak is compatible with the detected GPU vendors.
    pub fn is_gpu_vendor_compatible(
        &self,
        detected_vendors: &[crate::hardware::gpu::GpuVendor],
    ) -> bool {
        match &self.gpu_vendor {
            None => true,
            Some(req) => {
                let req_clean = req.trim().to_uppercase();
                if req_clean.is_empty() || req_clean == "ANY" || req_clean == "ALL" {
                    return true;
                }
                detected_vendors.iter().any(|v| match v {
                    crate::hardware::gpu::GpuVendor::Nvidia => req_clean == "NVIDIA",
                    crate::hardware::gpu::GpuVendor::Amd => req_clean == "AMD",
                    crate::hardware::gpu::GpuVendor::Intel => req_clean == "INTEL",
                    crate::hardware::gpu::GpuVendor::Unknown => false,
                })
            }
        }
    }
}

/// Load all tweak definitions from TOML files in the given directory.
///
/// Each `.toml` file is expected to deserialize into a [`TweakCatalog`].
/// Files that are not `.toml` are silently skipped.
pub fn load_rules_from_dir(
    dir: &std::path::Path,
) -> crate::core::error::Result<Vec<TweakDefinition>> {
    let mut all_tweaks = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                if let Ok(mut reader) = std::fs::File::open(&path).map(std::io::BufReader::new) {
                    let mut content = String::new();
                    use std::io::Read;
                    if reader.read_to_string(&mut content).is_ok() {
                        match toml::from_str::<TweakCatalog>(&content) {
                            Ok(catalog) => all_tweaks.extend(catalog.tweaks),
                            Err(e) => {
                                tracing::error!("Failed to parse rule file {:?}: {}", path, e)
                            }
                        }
                    } else {
                        tracing::error!("Failed to read rule file {:?}", path);
                    }
                } else {
                    tracing::error!("Failed to open rule file {:?}", path);
                }
            }
        }
    }
    Ok(all_tweaks)
}

const EMBEDDED_CATALOG_APPX: &str = include_str!("../../assets/rules/bloatware_appx.toml");
const EMBEDDED_CATALOG_HARDWARE: &str = include_str!("../../assets/rules/hardware_tuning.toml");
const EMBEDDED_CATALOG_OS_SETTINGS: &str = include_str!("../../assets/rules/os_settings.toml");
const EMBEDDED_CATALOG_PRIVACY: &str = include_str!("../../assets/rules/privacy.toml");
const EMBEDDED_CATALOG_REGISTRY_LAG: &str = include_str!("../../assets/rules/registry_lag.toml");
const EMBEDDED_CATALOG_TELEMETRY_APPS: &str =
    include_str!("../../assets/rules/telemetry_apps.toml");
const EMBEDDED_CATALOG_TELEMETRY_OS: &str = include_str!("../../assets/rules/telemetry_os.toml");
const EMBEDDED_CATALOG_AI_PRIVACY_EXT: &str =
    include_str!("../../assets/rules/ai_privacy_ext.toml");
const EMBEDDED_CATALOG_PERF_POWER_EXT: &str =
    include_str!("../../assets/rules/perf_power_ext.toml");
const EMBEDDED_CATALOG_UX_CUSTOMIZATION: &str =
    include_str!("../../assets/rules/ux_customization.toml");
const EMBEDDED_CATALOG_SYSTEM_DEEP_OPT: &str =
    include_str!("../../assets/rules/system_deep_opt.toml");
const EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA: &str =
    include_str!("../../assets/rules/gaming_latency_ultra.toml");

pub const EMBEDDED_CATALOGS: &[&str] = &[
    EMBEDDED_CATALOG_APPX,
    EMBEDDED_CATALOG_HARDWARE,
    EMBEDDED_CATALOG_OS_SETTINGS,
    EMBEDDED_CATALOG_PRIVACY,
    EMBEDDED_CATALOG_REGISTRY_LAG,
    EMBEDDED_CATALOG_TELEMETRY_APPS,
    EMBEDDED_CATALOG_TELEMETRY_OS,
    EMBEDDED_CATALOG_AI_PRIVACY_EXT,
    EMBEDDED_CATALOG_PERF_POWER_EXT,
    EMBEDDED_CATALOG_UX_CUSTOMIZATION,
    EMBEDDED_CATALOG_SYSTEM_DEEP_OPT,
    EMBEDDED_CATALOG_GAMING_LATENCY_ULTRA,
];

/// Load all embedded rule catalogs compiled directly into the binary.
pub fn load_embedded_rules() -> Vec<TweakDefinition> {
    let mut all_tweaks = Vec::new();
    for toml_str in EMBEDDED_CATALOGS {
        match toml::from_str::<TweakCatalog>(toml_str) {
            Ok(catalog) => all_tweaks.extend(catalog.tweaks),
            Err(e) => tracing::error!("Failed to parse embedded catalog: {}", e),
        }
    }
    all_tweaks
}

/// State-of-the-art catalog loader with multi-path resolution and embedded fallback.
///
/// Ensures the application NEVER runs with 0 tweaks, regardless of working directory
/// or invocation method.
pub fn load_rules_with_fallback(preferred_dir: Option<&std::path::Path>) -> Vec<TweakDefinition> {
    // 1. Try preferred directory if given
    if let Some(dir) = preferred_dir {
        if let Ok(tweaks) = load_rules_from_dir(dir) {
            if !tweaks.is_empty() {
                tracing::info!(
                    "Loaded {} tweaks from preferred directory: {:?}",
                    tweaks.len(),
                    dir
                );
                return tweaks;
            }
        }
    }

    // 2. Candidate relative paths
    let candidates = [
        std::path::PathBuf::from("assets/rules"),
        std::path::PathBuf::from("../assets/rules"),
        std::path::PathBuf::from("../../assets/rules"),
    ];

    for candidate in &candidates {
        if candidate.is_dir() {
            if let Ok(tweaks) = load_rules_from_dir(candidate) {
                if !tweaks.is_empty() {
                    tracing::info!(
                        "Loaded {} tweaks from candidate: {:?}",
                        tweaks.len(),
                        candidate
                    );
                    return tweaks;
                }
            }
        }
    }

    // 3. Candidate relative to current executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_candidates = [
                exe_dir.join("assets").join("rules"),
                exe_dir.join("..").join("assets").join("rules"),
                exe_dir.join("..").join("..").join("assets").join("rules"),
            ];
            for candidate in &exe_candidates {
                if candidate.is_dir() {
                    if let Ok(tweaks) = load_rules_from_dir(candidate) {
                        if !tweaks.is_empty() {
                            tracing::info!(
                                "Loaded {} tweaks from exe-relative candidate: {:?}",
                                tweaks.len(),
                                candidate
                            );
                            return tweaks;
                        }
                    }
                }
            }
        }
    }

    // 4. Guaranteed zero-fail fallback: embedded rule catalogs
    let embedded = load_embedded_rules();
    tracing::info!(
        "Loaded {} tweaks from embedded rule catalogs",
        embedded.len()
    );
    embedded
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_all_rules() {
        let rules_dir = Path::new("assets/rules");
        let tweaks = load_rules_from_dir(rules_dir).expect("Failed to load rules");
        assert!(!tweaks.is_empty(), "Expected non-empty tweaks catalog");
        assert!(tweaks.len() >= 80, "Expected at least 80 tweaks loaded");
        println!("Successfully loaded {} tweaks", tweaks.len());
    }

    #[test]
    fn test_no_duplicate_tweak_ids() {
        use std::collections::HashSet;

        let tweaks = load_embedded_rules();
        let mut seen = HashSet::new();
        let mut duplicates = Vec::new();

        for t in &tweaks {
            if !seen.insert(t.id.clone()) {
                duplicates.push(t.id.clone());
            }
        }

        assert!(
            duplicates.is_empty(),
            "Found duplicate tweak IDs in catalogs: {:?}",
            duplicates
        );
        println!("Verified {} unique tweaks with 0 duplicates.", tweaks.len());
    }

    #[test]
    fn test_embedded_catalog_fallback() {
        let tweaks = load_embedded_rules();
        assert!(
            tweaks.len() >= 80,
            "All catalog tweaks must be embedded in binary (found {})",
            tweaks.len()
        );

        // Test fallback with non-existent path
        let non_existent = Path::new("C:/non/existent/path/for/winblast/testing");
        let fallback_tweaks = load_rules_with_fallback(Some(non_existent));
        assert!(
            fallback_tweaks.len() >= 80,
            "Fallback must return all tweaks (found {})",
            fallback_tweaks.len()
        );
    }

    #[test]
    fn test_risk_valuation() {
        let risk = RiskValuation {
            stability_risk: RiskLevel::Safe,
            performance_gain: 4,
            privacy_gain: 1,
            reversibility: Reversibility::Instant,
            broken_features: vec![],
        };
        assert_eq!(risk.stability_risk, RiskLevel::Safe);
        assert_eq!(risk.performance_gain, 4);
        assert_eq!(normalize_reg_value_name("@"), "");
        assert_eq!(normalize_reg_value_name("(Default)"), "");
        assert_eq!(display_reg_value_name("@"), "(Default)");
    }

    #[test]
    fn test_is_compatible_edition_aliases() {
        let tweak = TweakDefinition {
            id: "test".into(),
            title: "Test".into(),
            category: "Test".into(),
            description: "Test".into(),
            easy_mode_included: true,
            os_req: OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec!["Home".into(), "Pro".into(), "Enterprise".into()],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 1,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions: vec![],
            gpu_vendor: None,
            sub_category: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        };

        assert!(tweak.is_compatible(19041, "Professional"));
        assert!(tweak.is_compatible(22631, "Pro"));
        assert!(tweak.is_compatible(26100, "Core"));
        assert!(tweak.is_compatible(19045, "EnterpriseS"));
        assert!(tweak.is_compatible(22000, "Unknown"));
        assert!(!tweak.is_compatible(10000, "Professional"));
    }

    #[test]
    fn test_deserialize_tweak_with_fallbacks_and_graph_metadata() {
        let toml_str = r#"
        [[tweaks]]
        id = "tweak_with_fallbacks"
        title = "Test Fallbacks"
        category = "Test"
        description = "Test multi-strategy execution and graph constraints"
        easy_mode_included = false
        prerequisites = ["prior_tweak_a", "prior_tweak_b"]
        conflicts_with = ["conflicting_tweak_c"]
        fallbacks = [
            [
                { type = "RegistryDword", hive = "HKCU", path = "Software\\WinBlast", name = "Fallback1", optimized_value = 1, default_value = 0 }
            ],
            [
                { type = "RegistryString", hive = "HKLM", path = "Software\\WinBlast", name = "Fallback2", optimized_value = "Enabled", default_value = "Disabled" }
            ]
        ]

        [tweaks.os_req]
        min_build = 19041
        editions = ["Pro"]

        [tweaks.valuation]
        stability_risk = "Safe"
        performance_gain = 3
        privacy_gain = 0
        reversibility = "Instant"
        broken_features = []

        [[tweaks.actions]]
        type = "RegistryDword"
        hive = "HKLM"
        path = "Software\\WinBlast"
        name = "Primary"
        optimized_value = 1
        default_value = 0
        "#;

        let catalog: TweakCatalog =
            toml::from_str(toml_str).expect("Failed to deserialize TOML with fallbacks");
        assert_eq!(catalog.tweaks.len(), 1);
        let t = &catalog.tweaks[0];
        assert_eq!(t.prerequisites, vec!["prior_tweak_a", "prior_tweak_b"]);
        assert_eq!(t.conflicts_with, vec!["conflicting_tweak_c"]);
        assert_eq!(t.fallbacks.len(), 2);
        assert_eq!(t.fallbacks[0].len(), 1);
        assert_eq!(t.fallbacks[1].len(), 1);
    }

    #[test]
    fn test_gpu_vendor_compatibility() {
        use crate::hardware::gpu::{detect_vendor, set_gpu_msi_mode, GpuVendor};

        // 1. PCI Hardware Vendor ID detection
        assert_eq!(
            detect_vendor(
                "PCI\\VEN_10DE&DEV_2684&SUBSYS_16E810DE",
                "3D Video Controller"
            ),
            GpuVendor::Nvidia
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_1002&DEV_73DF&SUBSYS_0C011002", "Display"),
            GpuVendor::Amd
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_1022&DEV_1638", "Display Controller"),
            GpuVendor::Amd
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_8086&DEV_56A0&SUBSYS_00008086", "Video Controller"),
            GpuVendor::Intel
        );

        // 2. Driver name fallbacks
        assert_eq!(
            detect_vendor("", "NVIDIA GeForce RTX 4090"),
            GpuVendor::Nvidia
        );
        assert_eq!(detect_vendor("", "AMD Radeon RX 7900 XTX"), GpuVendor::Amd);
        assert_eq!(
            detect_vendor("", "Intel(R) Arc(TM) A770 Graphics"),
            GpuVendor::Intel
        );
        assert_eq!(
            detect_vendor("", "Intel(R) UHD Graphics 770"),
            GpuVendor::Intel
        );

        // 3. Virtual display adapters & unexpected/empty PnP IDs (must not panic)
        assert_eq!(
            detect_vendor(
                "ROOT\\BASICDISPLAY\\0000",
                "Microsoft Basic Display Adapter"
            ),
            GpuVendor::Unknown
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_15AD&DEV_0405", "VMware SVGA 3D"),
            GpuVendor::Unknown
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_80EE&DEV_BEEF", "VirtualBox Graphics Adapter"),
            GpuVendor::Unknown
        );
        assert_eq!(detect_vendor("", ""), GpuVendor::Unknown);
        assert_eq!(
            detect_vendor("UNKNOWN_ID", "Unknown GPU"),
            GpuVendor::Unknown
        );

        // 4. MSI mode error handling on empty or whitespace PnP ID
        assert!(set_gpu_msi_mode("", true, 2).is_err());
        assert!(set_gpu_msi_mode("   ", false, 3).is_err());

        // 5. GpuVendor string representation
        assert_eq!(GpuVendor::Nvidia.as_str(), "NVIDIA");
        assert_eq!(GpuVendor::Amd.as_str(), "AMD");
        assert_eq!(GpuVendor::Intel.as_str(), "Intel");
        assert_eq!(GpuVendor::Unknown.as_str(), "Unknown");

        // 6. Vendor compatibility matrix
        let mut tweak = TweakDefinition {
            id: "nvidia_test".into(),
            title: "NVIDIA Test".into(),
            category: "GPU Tuning".into(),
            description: "Test".into(),
            easy_mode_included: false,
            os_req: OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec![],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 3,
                privacy_gain: 0,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions: vec![],
            gpu_vendor: Some("NVIDIA".into()),
            sub_category: None,
            unattend_safe: false,
            source_attribution: None,
            fallbacks: Vec::new(),
            prerequisites: Vec::new(),
            conflicts_with: Vec::new(),
        };

        // System with NVIDIA GPU
        let nvidia_sys = [GpuVendor::Nvidia];
        assert!(tweak.is_gpu_vendor_compatible(&nvidia_sys));

        // System with AMD GPU
        let amd_sys = [GpuVendor::Amd];
        assert!(!tweak.is_gpu_vendor_compatible(&amd_sys));

        // System with Intel GPU
        let intel_sys = [GpuVendor::Intel];
        assert!(!tweak.is_gpu_vendor_compatible(&intel_sys));

        // Multi-GPU system (Discrete NVIDIA + Integrated Intel)
        let dual_sys = [GpuVendor::Intel, GpuVendor::Nvidia];
        assert!(tweak.is_gpu_vendor_compatible(&dual_sys));

        // Tweak requiring AMD
        tweak.gpu_vendor = Some("AMD".into());
        assert!(!tweak.is_gpu_vendor_compatible(&nvidia_sys));
        assert!(tweak.is_gpu_vendor_compatible(&amd_sys));
        assert!(!tweak.is_gpu_vendor_compatible(&dual_sys));

        // Tweak requiring Intel
        tweak.gpu_vendor = Some("Intel".into());
        assert!(!tweak.is_gpu_vendor_compatible(&nvidia_sys));
        assert!(tweak.is_gpu_vendor_compatible(&intel_sys));
        assert!(tweak.is_gpu_vendor_compatible(&dual_sys));

        // Neutral tweak
        tweak.gpu_vendor = None;
        assert!(tweak.is_gpu_vendor_compatible(&nvidia_sys));
        assert!(tweak.is_gpu_vendor_compatible(&amd_sys));
        assert!(tweak.is_gpu_vendor_compatible(&dual_sys));
        assert!(tweak.is_gpu_vendor_compatible(&[]));
    }
}
