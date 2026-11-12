//! Pure-Rust Setup-Safe `autounattend.xml` Generator (inspired by Winhance).
//!
//! Explicitly filters down to `unattend_safe == true` catalog tweaks, routes `HKCU`
//! mutations cleanly into `%SystemDrive%\Users\Default\NTUSER.DAT` (`reg load HKU\DefaultUser`)
//! plus `FirstLogonCommands`, and escapes all XML characters safely.

use serde::{Deserialize, Serialize};

use crate::core::risk::{display_reg_value_name, normalize_reg_value_name, TweakAction, TweakDefinition};

/// Target Windows processor architecture for `autounattend.xml` components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TargetArch {
    #[default]
    Amd64,
    Arm64,
}

impl TargetArch {
    pub const fn as_str(self) -> &'static str {
        match self {
            TargetArch::Amd64 => "amd64",
            TargetArch::Arm64 => "arm64",
        }
    }

    pub const fn detect_host() -> Self {
        if cfg!(target_arch = "aarch64") {
            TargetArch::Arm64
        } else {
            TargetArch::Amd64
        }
    }
}

/// Configuration options for generating a Windows 10/11 `autounattend.xml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnattendOptions {
    pub bypass_tpm_ram_requirements: bool,
    pub bypass_msa_oobe: bool,
    pub skip_oobe_privacy_screens: bool,
    pub auto_timezone_utc: bool,
    pub default_local_user: String,
    pub computer_name: String,
    pub include_selected_tweaks: bool,
    #[serde(default)]
    pub architecture: TargetArch,
}

impl Default for UnattendOptions {
    fn default() -> Self {
        Self {
            bypass_tpm_ram_requirements: true,
            bypass_msa_oobe: true,
            skip_oobe_privacy_screens: true,
            auto_timezone_utc: false,
            default_local_user: "User".to_string(),
            computer_name: "WINBLAST-PC".to_string(),
            include_selected_tweaks: true,
            architecture: TargetArch::detect_host(),
        }
    }
}

fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn sanitize_computer_name(s: &str) -> String {
    let clean: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .take(15)
        .collect();
    let trimmed = clean.trim_matches('-').to_string();
    if trimmed.is_empty() || trimmed.chars().all(|c| c.is_ascii_digit()) {
        "WINBLAST-PC".to_string()
    } else {
        trimmed
    }
}

fn sanitize_local_user(s: &str, computer_name: &str) -> String {
    let clean: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(20)
        .collect();
    let candidate = if clean.is_empty() {
        "User".to_string()
    } else {
        clean
    };
    const RESERVED_LOCAL_USERS: &[&str] = &[
        "Administrator",
        "Guest",
        "DefaultAccount",
        "WDAGUtilityAccount",
    ];
    let candidate = if RESERVED_LOCAL_USERS
        .iter()
        .any(|r| candidate.eq_ignore_ascii_case(r))
    {
        "User".to_string()
    } else {
        candidate
    };
    if candidate.eq_ignore_ascii_case(computer_name) {
        if computer_name.eq_ignore_ascii_case("User") {
            "LocalUser".to_string()
        } else {
            "User".to_string()
        }
    } else {
        candidate
    }
}

/// Escape an argument placed inside double quotes for Win32 `CommandLineToArgvW` (`reg.exe`):
/// doubles any `\` run preceding `"` and doubles any trailing `\` run before the closing `"`.
fn escape_win32_quoted_arg(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut backslashes = 0usize;
    for ch in s.chars() {
        if ch == '\\' {
            backslashes += 1;
            out.push('\\');
        } else if ch == '"' {
            for _ in 0..backslashes {
                out.push('\\');
            }
            backslashes = 0;
            out.push_str("\\\"");
        } else {
            backslashes = 0;
            out.push(ch);
        }
    }
    for _ in 0..backslashes {
        out.push('\\');
    }
    out
}

fn map_hive(hive: &str, use_default_user_mount: bool) -> Option<String> {
    if hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
        Some("HKLM".to_string())
    } else if hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER") {
        Some(if use_default_user_mount {
            "HKU\\DefaultUser".to_string()
        } else {
            "HKCU".to_string()
        })
    } else {
        None
    }
}

fn format_reg_add_cmd(
    hive: &str,
    path: &str,
    name: &str,
    reg_type: &str,
    val: &str,
    use_default_user_mount: bool,
) -> Option<String> {
    let clean_path = path.trim().trim_matches('\\');
    if clean_path.is_empty() {
        return None;
    }
    let mapped_hive = map_hive(hive, use_default_user_mount)?;

    let norm_name = normalize_reg_value_name(name);
    let val_flag = if norm_name.is_empty() {
        "/ve".to_string()
    } else {
        format!("/v \"{}\"", escape_win32_quoted_arg(norm_name))
    };

    let full_key = format!("{}\\{}", mapped_hive, clean_path);
    Some(format!(
        "reg.exe add \"{}\" {} /t {} /d \"{}\" /f",
        escape_win32_quoted_arg(&full_key),
        val_flag,
        reg_type,
        escape_win32_quoted_arg(val)
    ))
}

fn format_reg_delete_cmd(
    hive: &str,
    path: &str,
    name: &str,
    use_default_user_mount: bool,
) -> Option<String> {
    let clean_path = path.trim().trim_matches('\\');
    if clean_path.is_empty() {
        return None;
    }
    let mapped_hive = map_hive(hive, use_default_user_mount)?;
    let norm_name = normalize_reg_value_name(name);
    let val_flag = if norm_name.is_empty() {
        "/ve".to_string()
    } else {
        format!("/v \"{}\"", escape_win32_quoted_arg(norm_name))
    };
    let full_key = format!("{}\\{}", mapped_hive, clean_path);
    Some(format!(
        "cmd /c reg.exe delete \"{}\" {} /f >nul 2>&1 || exit /b 0",
        escape_win32_quoted_arg(&full_key),
        val_flag
    ))
}

/// Generate a complete, validated Windows 10/11 `autounattend.xml` string from options
/// and the user's selected `unattend_safe` tweaks.
pub fn generate_autounattend_xml(
    opts: &UnattendOptions,
    selected_tweaks: &[&TweakDefinition],
) -> String {
    let mut specialize_cmds: Vec<(String, String)> = Vec::new();
    let mut first_logon_cmds: Vec<(String, String)> = Vec::new();
    let mut hkcu_default_cmds: Vec<String> = Vec::new();

    if opts.bypass_msa_oobe {
        // BypassNRO enables local account setup offline on Windows 11 OOBE
        specialize_cmds.push((
            "BypassNRO".to_string(),
            "reg.exe add \"HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\OOBE\" /v BypassNRO /t REG_DWORD /d 1 /f".to_string(),
        ));
    }

    if opts.include_selected_tweaks {
        for tweak in selected_tweaks {
            if !tweak.unattend_safe {
                continue;
            }
            for action in &tweak.actions {
                match action {
                    TweakAction::RegistryDword { hive, path, name, optimized_value, .. } => {
                        let norm_name = normalize_reg_value_name(name);
                        let is_sz = crate::core::reg_helpers::is_known_numeric_reg_sz(path, norm_name);
                        let reg_ty = if is_sz { "REG_SZ" } else { "REG_DWORD" };
                        let val_str = if is_sz && (*optimized_value as i32) < 0 {
                            (*optimized_value as i32).to_string()
                        } else {
                            optimized_value.to_string()
                        };
                        if hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
                            if let Some(cmd) = format_reg_add_cmd(hive, path, name, reg_ty, &val_str, false) {
                                specialize_cmds.push((format!("WB0_{}", tweak.id), cmd));
                            }
                        } else if hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER") {
                            if let Some(c) = format_reg_add_cmd(hive, path, name, reg_ty, &val_str, true) {
                                hkcu_default_cmds.push(c);
                            }
                            if let Some(c) = format_reg_add_cmd(hive, path, name, reg_ty, &val_str, false) {
                                first_logon_cmds.push((format!("WB0_{}_{}", tweak.id, display_reg_value_name(name)), c));
                            }
                        }
                    }
                    TweakAction::RegistryString { hive, path, name, optimized_value, expand_sz, .. } => {
                        if optimized_value == "__ABSENT__" {
                            if hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
                                if let Some(cmd) = format_reg_delete_cmd(hive, path, name, false) {
                                    specialize_cmds.push((format!("WB0_{}", tweak.id), cmd));
                                }
                            } else if hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER") {
                                if let Some(c) = format_reg_delete_cmd(hive, path, name, true) {
                                    hkcu_default_cmds.push(c);
                                }
                                if let Some(c) = format_reg_delete_cmd(hive, path, name, false) {
                                    first_logon_cmds.push((format!("WB0_{}_{}", tweak.id, display_reg_value_name(name)), c));
                                }
                            }
                            continue;
                        }
                        let ty = if *expand_sz { "REG_EXPAND_SZ" } else { "REG_SZ" };
                        if hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
                            if let Some(cmd) = format_reg_add_cmd(hive, path, name, ty, optimized_value, false) {
                                specialize_cmds.push((format!("WB0_{}", tweak.id), cmd));
                            }
                        } else if hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER") {
                            if let Some(c) = format_reg_add_cmd(hive, path, name, ty, optimized_value, true) {
                                hkcu_default_cmds.push(c);
                            }
                            if let Some(c) = format_reg_add_cmd(hive, path, name, ty, optimized_value, false) {
                                first_logon_cmds.push((format!("WB0_{}_{}", tweak.id, display_reg_value_name(name)), c));
                            }
                        }
                    }
                    TweakAction::RegistryDelete { hive, path, name } => {
                        if hive.eq_ignore_ascii_case("HKLM") || hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
                            if let Some(cmd) = format_reg_delete_cmd(hive, path, name, false) {
                                specialize_cmds.push((format!("WB0_{}", tweak.id), cmd));
                            }
                        } else if hive.eq_ignore_ascii_case("HKCU") || hive.eq_ignore_ascii_case("HKEY_CURRENT_USER") {
                            if let Some(c) = format_reg_delete_cmd(hive, path, name, true) {
                                hkcu_default_cmds.push(c);
                            }
                            if let Some(c) = format_reg_delete_cmd(hive, path, name, false) {
                                first_logon_cmds.push((format!("WB0_{}_{}", tweak.id, display_reg_value_name(name)), c));
                            }
                        }
                    }
                    TweakAction::ServiceState { service_name, target_start_type, .. } => {
                        let svc_clean = service_name.trim().trim_matches('\\');
                        if !svc_clean.is_empty() && !svc_clean.contains('\0') {
                            let svc_path = format!("SYSTEM\\CurrentControlSet\\Services\\{}", svc_clean);
                            if let Some(cmd) = format_reg_add_cmd("HKLM", &svc_path, "Start", "REG_DWORD", &target_start_type.to_string(), false) {
                                specialize_cmds.push((format!("WB0_{}_{}", tweak.id, svc_clean), cmd));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let arch = opts.architecture.as_str();
    let mut xml = String::with_capacity(4096);
    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    xml.push_str("<unattend xmlns=\"urn:schemas-microsoft-com:unattend\" xmlns:wcm=\"http://schemas.microsoft.com/WMIConfig/2002/State\">\n");

    // 1. windowsPE pass (hardware check bypasses)
    if opts.bypass_tpm_ram_requirements {
        xml.push_str("  <settings pass=\"windowsPE\">\n");
        xml.push_str(&format!(
            "    <component name=\"Microsoft-Windows-Setup\" processorArchitecture=\"{}\" publicKeyToken=\"31bf3856ad364e35\" language=\"neutral\" versionScope=\"nonSxS\">\n",
            arch
        ));
        xml.push_str("      <RunSynchronous>\n");
        let lab_checks = [
            ("BypassTPM", "reg.exe add \"HKLM\\SYSTEM\\Setup\\LabConfig\" /v BypassTPMCheck /t REG_DWORD /d 1 /f"),
            ("BypassSecureBoot", "reg.exe add \"HKLM\\SYSTEM\\Setup\\LabConfig\" /v BypassSecureBootCheck /t REG_DWORD /d 1 /f"),
            ("BypassRAM", "reg.exe add \"HKLM\\SYSTEM\\Setup\\LabConfig\" /v BypassRAMCheck /t REG_DWORD /d 1 /f"),
            ("BypassStorage", "reg.exe add \"HKLM\\SYSTEM\\Setup\\LabConfig\" /v BypassStorageCheck /t REG_DWORD /d 1 /f"),
        ];
        for (i, (desc, cmd)) in lab_checks.iter().enumerate() {
            xml.push_str(&format!(
                "        <RunSynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>{}</Description>\n          <Path>{}</Path>\n        </RunSynchronousCommand>\n",
                i + 1,
                escape_xml(desc),
                escape_xml(cmd)
            ));
        }
        xml.push_str("      </RunSynchronous>\n");
        xml.push_str("    </component>\n");
        xml.push_str("  </settings>\n");
    }

    // 2. specialize pass (ComputerName + HKLM rules + Default User NTUSER.DAT patching)
    let comp_name = sanitize_computer_name(&opts.computer_name);
    xml.push_str("  <settings pass=\"specialize\">\n");
    xml.push_str(&format!(
        "    <component name=\"Microsoft-Windows-Shell-Setup\" processorArchitecture=\"{}\" publicKeyToken=\"31bf3856ad364e35\" language=\"neutral\" versionScope=\"nonSxS\">\n",
        arch
    ));
    xml.push_str(&format!("      <ComputerName>{}</ComputerName>\n", escape_xml(&comp_name)));
    if opts.auto_timezone_utc {
        xml.push_str("      <TimeZone>UTC</TimeZone>\n");
    }
    xml.push_str("    </component>\n");

    if !specialize_cmds.is_empty() || !hkcu_default_cmds.is_empty() {
        xml.push_str(&format!(
            "    <component name=\"Microsoft-Windows-Deployment\" processorArchitecture=\"{}\" publicKeyToken=\"31bf3856ad364e35\" language=\"neutral\" versionScope=\"nonSxS\">\n",
            arch
        ));
        xml.push_str("      <RunSynchronous>\n");
        let mut order = 1usize;
        for (desc, cmd) in &specialize_cmds {
            xml.push_str(&format!(
                "        <RunSynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>{}</Description>\n          <Path>{}</Path>\n        </RunSynchronousCommand>\n",
                order,
                escape_xml(desc),
                escape_xml(cmd)
            ));
            order += 1;
        }
        if !hkcu_default_cmds.is_empty() {
            xml.push_str(&format!(
                "        <RunSynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>LoadDefaultUserHive</Description>\n          <Path>reg.exe load HKU\\DefaultUser %SystemDrive%\\Users\\Default\\NTUSER.DAT</Path>\n        </RunSynchronousCommand>\n",
                order
            ));
            order += 1;
            for (idx, cmd) in hkcu_default_cmds.iter().enumerate() {
                xml.push_str(&format!(
                    "        <RunSynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>DefaultUserRule_{}</Description>\n          <Path>{}</Path>\n        </RunSynchronousCommand>\n",
                    order,
                    idx + 1,
                    escape_xml(cmd)
                ));
                order += 1;
            }
            xml.push_str(&format!(
                "        <RunSynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>UnloadDefaultUserHive</Description>\n          <Path>reg.exe unload HKU\\DefaultUser</Path>\n        </RunSynchronousCommand>\n",
                order
            ));
        }
        xml.push_str("      </RunSynchronous>\n");
        xml.push_str("    </component>\n");
    }
    xml.push_str("  </settings>\n");

    // 3. oobeSystem pass
    xml.push_str("  <settings pass=\"oobeSystem\">\n");
    xml.push_str(&format!(
        "    <component name=\"Microsoft-Windows-Shell-Setup\" processorArchitecture=\"{}\" publicKeyToken=\"31bf3856ad364e35\" language=\"neutral\" versionScope=\"nonSxS\">\n",
        arch
    ));
    if opts.skip_oobe_privacy_screens {
        xml.push_str("      <OOBE>\n");
        xml.push_str("        <HideEULAPage>true</HideEULAPage>\n");
        if opts.bypass_msa_oobe {
            xml.push_str("        <HideLocalAccountScreen>true</HideLocalAccountScreen>\n");
        }
        xml.push_str("        <HideOEMRegistrationScreen>true</HideOEMRegistrationScreen>\n");
        xml.push_str("        <HideOnlineAccountScreens>true</HideOnlineAccountScreens>\n");
        xml.push_str("        <HideWirelessSetupInOOBE>true</HideWirelessSetupInOOBE>\n");
        xml.push_str("        <ProtectYourPC>3</ProtectYourPC>\n");
        xml.push_str("      </OOBE>\n");
    }
    if opts.bypass_msa_oobe {
        let user_clean = sanitize_local_user(&opts.default_local_user, &comp_name);
        xml.push_str("      <UserAccounts>\n        <LocalAccounts>\n          <LocalAccount wcm:action=\"add\">\n");
        xml.push_str(&format!("            <Name>{}</Name>\n", escape_xml(&user_clean)));
        xml.push_str("            <Group>Administrators</Group>\n            <Password><Value></Value><PlainText>true</PlainText></Password>\n");
        xml.push_str("          </LocalAccount>\n        </LocalAccounts>\n      </UserAccounts>\n");
    }
    if !first_logon_cmds.is_empty() {
        xml.push_str("      <FirstLogonCommands>\n");
        for (idx, (desc, cmd)) in first_logon_cmds.iter().enumerate() {
            xml.push_str(&format!(
                "        <SynchronousCommand wcm:action=\"add\">\n          <Order>{}</Order>\n          <Description>{}</Description>\n          <CommandLine>{}</CommandLine>\n        </SynchronousCommand>\n",
                idx + 1,
                escape_xml(desc),
                escape_xml(cmd)
            ));
        }
        xml.push_str("      </FirstLogonCommands>\n");
    }
    xml.push_str("    </component>\n");
    xml.push_str("  </settings>\n");
    xml.push_str("</unattend>\n");
    xml
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::risk::{OsRequirement, Reversibility, RiskLevel, RiskValuation};

    #[test]
    fn test_autounattend_generation_wellformed_and_escaped() {
        let safe_tweak = TweakDefinition {
            id: "test_safe".to_string(),
            title: "Safe Tweak & Test".to_string(),
            category: "Privacy".to_string(),
            description: "Desc".to_string(),
            easy_mode_included: true,
            os_req: OsRequirement {
                min_build: 19041,
                max_build: None,
                editions: vec![],
            },
            valuation: RiskValuation {
                stability_risk: RiskLevel::Safe,
                performance_gain: 1,
                privacy_gain: 5,
                reversibility: Reversibility::Instant,
                broken_features: vec![],
            },
            actions: vec![
                TweakAction::RegistryDword {
                    hive: "HKLM".to_string(),
                    path: "SOFTWARE\\Policies\\Test".to_string(),
                    name: "AllowA&B".to_string(),
                    optimized_value: 0,
                    default_value: 1,
                },
                TweakAction::RegistryString {
                    hive: "HKCU".to_string(),
                    path: "Software\\Classes\\CLSID\\{test}\\InprocServer32".to_string(),
                    name: "(Default)".to_string(),
                    optimized_value: "".to_string(),
                    default_value: "__ABSENT__".to_string(),
                    expand_sz: false,
                    absent_is_default: true,
                    case_insensitive: false,
                },
            ],
            gpu_vendor: None,
            sub_category: None,
            unattend_safe: true,
            source_attribution: Some("Winhance".to_string()),
        };

        let unsafe_tweak = TweakDefinition {
            id: "test_unsafe".to_string(),
            unattend_safe: false,
            ..safe_tweak.clone()
        };

        let opts = UnattendOptions::default();
        let xml = generate_autounattend_xml(&opts, &[&safe_tweak, &unsafe_tweak]);
        assert!(xml.contains("BypassTPMCheck"));
        assert!(xml.contains("AllowA&amp;B"));
        assert!(xml.contains("reg.exe load HKU\\DefaultUser %SystemDrive%\\Users\\Default\\NTUSER.DAT"));
        assert!(xml.contains("/ve /t REG_SZ"));
        assert!(!xml.contains("test_unsafe"));
    }
}
