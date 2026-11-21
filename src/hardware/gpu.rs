//! GPU detection, PCIe MSI mode, HAGS, MPO, and interrupt priority management.
//!
//! Enumerates display adapters from the registry and exposes helpers to
//! enable Message Signaled Interrupts, set device priority, toggle HAGS, and MPO.

use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

/// Hardware GPU silicon vendor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

impl GpuVendor {
    pub const fn as_str(self) -> &'static str {
        match self {
            GpuVendor::Nvidia => "NVIDIA",
            GpuVendor::Amd => "AMD",
            GpuVendor::Intel => "Intel",
            GpuVendor::Unknown => "Unknown",
        }
    }
}

/// Detect if a standalone alphanumeric token matches `target` (avoids false positives like `"STATION"` matching `"ATI"`).
fn has_ascii_token(haystack_upper: &str, token: &str) -> bool {
    haystack_upper
        .split(|c: char| !c.is_ascii_alphanumeric())
        .any(|part| part == token)
}

/// Detect the GPU silicon vendor from PnP Device ID and adapter name.
pub fn detect_vendor(pnp_id: &str, name: &str) -> GpuVendor {
    let pnp_upper = pnp_id.to_uppercase();
    let name_upper = name.to_uppercase();

    // 1. Check PCI Hardware Vendor IDs (VEN_10DE = NVIDIA, VEN_1002/1022 = AMD, VEN_8086 = Intel)
    if pnp_upper.contains("VEN_10DE") {
        return GpuVendor::Nvidia;
    }
    if pnp_upper.contains("VEN_1002") || pnp_upper.contains("VEN_1022") {
        return GpuVendor::Amd;
    }
    if pnp_upper.contains("VEN_8086") {
        return GpuVendor::Intel;
    }

    // 2. Fallback to descriptive driver strings
    if name_upper.contains("NVIDIA")
        || name_upper.contains("GEFORCE")
        || name_upper.contains("RTX")
        || name_upper.contains("GTX")
        || name_upper.contains("QUADRO")
    {
        return GpuVendor::Nvidia;
    }
    if has_ascii_token(&name_upper, "AMD")
        || name_upper.contains("AMDGPU")
        || name_upper.contains("ADVANCED MICRO")
        || name_upper.contains("RADEON")
        || has_ascii_token(&name_upper, "ATI")
        || has_ascii_token(&pnp_upper, "1002")
        || has_ascii_token(&name_upper, "1002")
    {
        return GpuVendor::Amd;
    }
    if name_upper.contains("INTEL")
        || name_upper.contains("ARC")
        || name_upper.contains("IRIS")
        || name_upper.contains("UHD GRAPHICS")
    {
        return GpuVendor::Intel;
    }

    GpuVendor::Unknown
}

/// A detected GPU/display adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedGpu {
    /// Human-readable adapter name (e.g., "NVIDIA GeForce RTX 4090").
    pub name: String,
    /// PnP instance ID (e.g., `PCI\VEN_10DE&DEV_2684&SUBSYS_...`).
    pub pnp_instance_id: String,
    /// Whether MSI mode is currently enabled.
    pub msi_enabled: bool,
    /// Current interrupt priority (1 = Low, 2 = Normal, 3 = High).
    pub interrupt_priority: u32,
    /// Detected GPU silicon vendor.
    pub vendor: GpuVendor,
    /// Registry Display Class subkey index under `{4d36e968-e325-11ce-bfc1-08002be10318}` (e.g. `"0000"`).
    #[serde(default)]
    pub class_subkey: String,
    /// Dedicated GPU VRAM in MiB (accurate >4 GiB via DXGI `DedicatedVideoMemory`).
    #[serde(default)]
    pub dedicated_vram_mb: u64,
}

/// Determine primary GPU vendor from detected display adapters.
#[allow(dead_code)]
pub fn primary_vendor(gpus: &[DetectedGpu]) -> GpuVendor {
    // Prefer discrete GPU vendors (NVIDIA or AMD) over integrated (Intel) or Unknown
    for g in gpus {
        if g.vendor == GpuVendor::Nvidia || g.vendor == GpuVendor::Amd {
            return g.vendor;
        }
    }
    for g in gpus {
        if g.vendor == GpuVendor::Intel {
            return g.vendor;
        }
    }
    gpus.first().map(|g| g.vendor).unwrap_or(GpuVendor::Unknown)
}

/// Display adapter setup class GUID `{4d36e968-e325-11ce-bfc1-08002be10318}`.
pub const DISPLAY_CLASS_PREFIX: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";

/// Parse a target vendor filter string (`"NVIDIA"`, `"AMD"`, `"INTEL"`, etc.).
pub fn parse_target_vendor(req: Option<&str>) -> Option<GpuVendor> {
    let clean = req?.trim().to_ascii_uppercase();
    match clean.as_str() {
        "NVIDIA" => Some(GpuVendor::Nvidia),
        "AMD" => Some(GpuVendor::Amd),
        "INTEL" => Some(GpuVendor::Intel),
        _ => None,
    }
}

/// Extract `(vendor_id, device_id)` hex pair from a PCI PnP ID like `PCI\VEN_10DE&DEV_2684...`.
fn parse_ven_dev(pnp_or_hw: &str) -> Option<(u32, u32)> {
    let up = pnp_or_hw.to_ascii_uppercase();
    let ven_idx = up.find("VEN_")?;
    let ven = u32::from_str_radix(up.get(ven_idx + 4..ven_idx + 8)?, 16).ok()?;
    let dev_idx = up.find("DEV_")?;
    let dev = u32::from_str_radix(up.get(dev_idx + 4..dev_idx + 8)?, 16).ok()?;
    Some((ven, dev))
}

/// RAII wrapper for `SetupDiGetClassDevsW` handle.
struct DevInfoGuard(windows::Win32::Devices::DeviceAndDriverInstallation::HDEVINFO);

impl Drop for DevInfoGuard {
    fn drop(&mut self) {
        unsafe {
            let _ =
                windows::Win32::Devices::DeviceAndDriverInstallation::SetupDiDestroyDeviceInfoList(
                    self.0,
                );
        }
    }
}

/// Query currently present (active, non-ghost) display adapter class subkeys (e.g. `"0000"`, `"0001"`)
/// mapped to their true Win32 PnP `DeviceInstanceId` via SetupAPI (`SetupDiGetDeviceInstanceIdW`).
fn present_display_class_subkeys() -> Vec<(String, String)> {
    use windows::core::GUID;
    use windows::Win32::Devices::DeviceAndDriverInstallation::{
        SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceInstanceIdW,
        SetupDiGetDeviceRegistryPropertyW, DIGCF_PRESENT, SPDRP_DRIVER, SP_DEVINFO_DATA,
    };

    const GUID_DEVCLASS_DISPLAY: GUID = GUID::from_u128(0x4d36e968_e325_11ce_bfc1_08002be10318);
    let mut result: Vec<(String, String)> = Vec::new();
    let Ok(hdev) =
        (unsafe { SetupDiGetClassDevsW(Some(&GUID_DEVCLASS_DISPLAY), None, None, DIGCF_PRESENT) })
    else {
        return result;
    };
    let _guard = DevInfoGuard(hdev);

    let mut idx = 0u32;
    loop {
        let mut dev_data = SP_DEVINFO_DATA {
            cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
            ..Default::default()
        };
        if unsafe { SetupDiEnumDeviceInfo(hdev, idx, &mut dev_data) }.is_err() {
            break;
        }
        idx += 1;

        let mut buf = [0u8; 512];
        let mut req_size = 0u32;
        if unsafe {
            SetupDiGetDeviceRegistryPropertyW(
                hdev,
                &dev_data,
                SPDRP_DRIVER,
                None,
                Some(&mut buf),
                Some(&mut req_size),
            )
        }
        .is_ok()
        {
            let u16_len = (req_size as usize / 2).min(buf.len() / 2);
            let words: Vec<u16> = buf[..u16_len * 2]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|&[lo, hi]| u16::from_le_bytes([lo, hi]))
                .take_while(|&w| w != 0)
                .collect();
            let driver_key = String::from_utf16_lossy(&words);
            // Driver key format: "{4d36e968-e325-11ce-bfc1-08002be10318}\0000"
            if let Some((_, sub)) = driver_key.rsplit_once('\\') {
                if !sub.is_empty() {
                    let mut id_buf = [0u16; 512];
                    let mut id_req = 0u32;
                    let instance_id = if unsafe {
                        SetupDiGetDeviceInstanceIdW(
                            hdev,
                            &dev_data,
                            Some(&mut id_buf),
                            Some(&mut id_req),
                        )
                    }
                    .is_ok()
                    {
                        let end = id_buf.iter().position(|&w| w == 0).unwrap_or(id_buf.len());
                        String::from_utf16_lossy(&id_buf[..end]).trim().to_string()
                    } else if id_req as usize > id_buf.len() {
                        let mut big_buf = vec![0u16; id_req as usize];
                        if unsafe {
                            SetupDiGetDeviceInstanceIdW(
                                hdev,
                                &dev_data,
                                Some(&mut big_buf),
                                Some(&mut id_req),
                            )
                        }
                        .is_ok()
                        {
                            let end = big_buf
                                .iter()
                                .position(|&w| w == 0)
                                .unwrap_or(big_buf.len());
                            String::from_utf16_lossy(&big_buf[..end]).trim().to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    if let Some(existing) =
                        result.iter_mut().find(|(s, _)| s.eq_ignore_ascii_case(sub))
                    {
                        if existing.1.is_empty() && !instance_id.is_empty() {
                            existing.1 = instance_id;
                        }
                    } else {
                        result.push((sub.to_string(), instance_id));
                    }
                }
            }
        }
    }
    result
}

/// Probe hardware DXGI adapters (`DedicatedVideoMemory` > 4 GiB safe). Returns `(VendorId, DeviceId, VramMiB)`.
fn detect_dxgi_vram_table() -> Vec<(u32, u32, u64)> {
    use windows::Win32::Graphics::Dxgi::{
        CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE,
    };

    let mut table = Vec::new();
    let Ok(factory) = (unsafe { CreateDXGIFactory1::<IDXGIFactory1>() }) else {
        return table;
    };
    let mut i = 0u32;
    while let Ok(adapter) = unsafe { factory.EnumAdapters1(i) } {
        i += 1;
        if let Ok(desc) = unsafe { adapter.GetDesc1() } {
            let is_sw = (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) != 0;
            let is_ms_basic = desc.VendorId == 0x1414 && desc.DeviceId == 0x008C;
            if !is_sw && !is_ms_basic {
                let vram_mb = (desc.DedicatedVideoMemory as u64) / (1024 * 1024);
                table.push((desc.VendorId, desc.DeviceId, vram_mb));
            }
        }
    }
    table
}

/// Resolve display adapter class path dynamically for multi-GPU / hybrid (Optimus) systems,
/// routing exclusively to adapters matching `target_vendor` (when specified) and preserving subkey suffixes.
pub fn resolve_gpu_class_path_for_vendor(
    hive: &str,
    path: &str,
    target_vendor: Option<&str>,
) -> Vec<String> {
    const DISPLAY_CLASS_LOWER: &str =
        "system\\currentcontrolset\\control\\class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
    if !hive.eq_ignore_ascii_case("HKLM") && !hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
        return vec![path.to_string()];
    }
    let lower = path.to_ascii_lowercase();
    if !lower.starts_with(DISPLAY_CLASS_LOWER) {
        return vec![path.to_string()];
    }

    // Preserve any nested suffix after `\0000` (e.g. `\0000\UMD` -> `\0001\UMD`)
    let rest = path[DISPLAY_CLASS_LOWER.len()..].trim_start_matches('\\');
    let (sub_seg, suffix) = match rest.find('\\') {
        Some(idx) => (&rest[..idx], &rest[idx..]),
        None => (rest, ""),
    };

    let wanted = parse_target_vendor(target_vendor);
    // If an explicit resolved subkey (e.g. "0001" from StateLedger) was passed without vendor override, keep it verbatim
    if wanted.is_none() && !sub_seg.is_empty() && !sub_seg.eq_ignore_ascii_case("0000") {
        return vec![path.to_string()];
    }
    let present = present_display_class_subkeys();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let mut matched = Vec::new();

    if let Ok(class_key) = hklm
        .open_subkey_with_flags(DISPLAY_CLASS_PREFIX, KEY_READ | KEY_WOW64_64KEY)
        .or_else(|_| hklm.open_subkey(DISPLAY_CLASS_PREFIX))
    {
        for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
            if subkey_name.eq_ignore_ascii_case("Properties") {
                continue;
            }
            let present_entry = present
                .iter()
                .find(|(p, _)| p.eq_ignore_ascii_case(&subkey_name));
            if !present.is_empty() && present_entry.is_none() {
                continue;
            }
            if let Ok(sub) = class_key
                .open_subkey_with_flags(&subkey_name, KEY_READ | KEY_WOW64_64KEY)
                .or_else(|_| class_key.open_subkey(&subkey_name))
            {
                let desc: String = sub.get_value("DriverDesc").unwrap_or_default();
                let setupapi_pnp = present_entry.map(|(_, id)| id.as_str()).unwrap_or("");
                let reg_pnp: String = sub.get_value("DeviceInstanceID").unwrap_or_default();
                let matching_hint: String = sub.get_value("MatchingDeviceId").unwrap_or_default();
                let pnp_for_vendor = if !setupapi_pnp.is_empty() {
                    setupapi_pnp
                } else if !reg_pnp.trim().is_empty() {
                    reg_pnp.trim()
                } else {
                    matching_hint.trim()
                };
                let vendor = detect_vendor(pnp_for_vendor, &desc);
                let is_match = match wanted {
                    Some(w) => vendor == w,
                    None => vendor == GpuVendor::Nvidia || vendor == GpuVendor::Amd,
                };
                if is_match {
                    matched.push(format!(
                        "{}\\{}{}",
                        DISPLAY_CLASS_PREFIX, subkey_name, suffix
                    ));
                }
            }
        }
    }

    if matched.is_empty() {
        vec![path.to_string()]
    } else {
        matched
    }
}

/// Resolve display adapter class path dynamically for multi-GPU / hybrid (Optimus) systems.
///
/// Legacy wrapper returning the first matching discrete adapter path. Prefer `resolve_gpu_class_path_for_vendor`.
pub fn resolve_gpu_class_path(hive: &str, path: &str) -> String {
    resolve_gpu_class_path_for_vendor(hive, path, None)
        .into_iter()
        .next()
        .unwrap_or_else(|| path.to_string())
}

/// Enumerate active (present) GPUs via SetupAPI + display adapter registry + DXGI VRAM.
pub fn detect_gpus() -> Vec<DetectedGpu> {
    let mut gpus = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_path = DISPLAY_CLASS_PREFIX;
    let present = present_display_class_subkeys();
    let dxgi_vram = detect_dxgi_vram_table();

    let class_key = match hklm
        .open_subkey_with_flags(class_path, KEY_READ | KEY_WOW64_64KEY)
        .or_else(|_| hklm.open_subkey(class_path))
    {
        Ok(k) => k,
        Err(_) => return gpus,
    };

    for subkey_name in class_key.enum_keys().filter_map(|k| k.ok()) {
        if subkey_name.eq_ignore_ascii_case("Properties") {
            continue;
        }
        // Skip DDU/stale orphaned subkeys when SetupAPI enumerates live present display adapters
        let present_entry = present
            .iter()
            .find(|(p, _)| p.eq_ignore_ascii_case(&subkey_name));
        if !present.is_empty() && present_entry.is_none() {
            continue;
        }
        if let Ok(subkey) = class_key
            .open_subkey_with_flags(&subkey_name, KEY_READ | KEY_WOW64_64KEY)
            .or_else(|_| class_key.open_subkey(&subkey_name))
        {
            let name: String = subkey
                .get_value("DriverDesc")
                .unwrap_or_else(|_| "Unknown GPU".to_string());

            // Prefer authoritative SetupAPI DeviceInstanceId (mapped directly from SP_DEVINFO_DATA),
            // then registry DeviceInstanceID. Never fabricate instance IDs from MatchingDeviceId!
            let setupapi_pnp = present_entry.map(|(_, id)| id.as_str()).unwrap_or("");
            let reg_pnp: String = subkey.get_value("DeviceInstanceID").unwrap_or_default();
            let pnp_id: String = if !setupapi_pnp.is_empty() {
                setupapi_pnp.to_string()
            } else {
                reg_pnp.trim().to_string()
            };

            if name == "Unknown GPU" && pnp_id.is_empty() {
                continue;
            }

            // Check MSI mode via Enum\pnp_id or class fallback
            let mut msi_enabled = false;
            let mut interrupt_priority = 2u32;

            if !pnp_id.is_empty() {
                let msi_path = format!(
                    "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
                    pnp_id
                );
                if let Ok(k) = hklm
                    .open_subkey_with_flags(&msi_path, KEY_READ | KEY_WOW64_64KEY)
                    .or_else(|_| hklm.open_subkey(&msi_path))
                {
                    msi_enabled = k.get_value::<u32, _>("MSISupported").unwrap_or(0) == 1;
                }

                let aff_path = format!(
                    "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\Affinity Policy",
                    pnp_id
                );
                if let Ok(k) = hklm
                    .open_subkey_with_flags(&aff_path, KEY_READ | KEY_WOW64_64KEY)
                    .or_else(|_| hklm.open_subkey(&aff_path))
                {
                    interrupt_priority = k.get_value::<u32, _>("DevicePriority").unwrap_or(2);
                }
            }

            // Fallback to checking class subkey directly
            if !msi_enabled {
                let class_msi = format!(
                    "{}\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
                    class_path, subkey_name
                );
                if let Ok(k) = hklm
                    .open_subkey_with_flags(&class_msi, KEY_READ | KEY_WOW64_64KEY)
                    .or_else(|_| hklm.open_subkey(&class_msi))
                {
                    msi_enabled = k.get_value::<u32, _>("MSISupported").unwrap_or(0) == 1;
                }
            }

            let matching_id: String = subkey.get_value("MatchingDeviceId").unwrap_or_default();
            let vendor_hint = if pnp_id.is_empty() {
                matching_id.as_str()
            } else {
                pnp_id.as_str()
            };
            let vendor = detect_vendor(vendor_hint, &name);

            // Avoid duplicate entries if registry mirrors the same device instance ID
            if !pnp_id.is_empty()
                && gpus
                    .iter()
                    .any(|g: &DetectedGpu| g.pnp_instance_id == pnp_id)
            {
                continue;
            }

            let ven_dev = parse_ven_dev(&pnp_id).or_else(|| parse_ven_dev(&matching_id));
            let dedicated_vram_mb = ven_dev
                .and_then(|(ven, dev)| {
                    dxgi_vram
                        .iter()
                        .find(|(dv, dd, _)| *dv == ven && *dd == dev)
                        .map(|(_, _, mb)| *mb)
                })
                .unwrap_or(0);

            gpus.push(DetectedGpu {
                name,
                pnp_instance_id: pnp_id,
                msi_enabled,
                interrupt_priority,
                vendor,
                class_subkey: subkey_name,
                dedicated_vram_mb,
            });
        }
    }

    gpus
}

/// Enable or disable MSI mode and set interrupt priority for a GPU.
pub fn set_gpu_msi_mode(pnp_instance_id: &str, enable: bool, priority: u32) -> std::io::Result<()> {
    let pnp_clean = pnp_instance_id.trim();
    if pnp_clean.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Cannot configure MSI: missing or empty PnP device instance ID",
        ));
    }

    let priority = priority.clamp(1, 3);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let msi_path = format!(
        "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\MessageSignaledInterruptProperties",
        pnp_clean
    );
    let affinity_path = format!(
        "SYSTEM\\CurrentControlSet\\Enum\\{}\\Device Parameters\\Interrupt Management\\Affinity Policy",
        pnp_clean
    );

    let (msi_key, _) = hklm
        .create_subkey_with_flags(&msi_path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
        .or_else(|_| hklm.create_subkey(&msi_path))?;
    msi_key.set_value("MSISupported", &(if enable { 1u32 } else { 0u32 }))?;

    let (aff_key, _) = hklm
        .create_subkey_with_flags(&affinity_path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
        .or_else(|_| hklm.create_subkey(&affinity_path))?;
    aff_key.set_value("DevicePriority", &priority)?;

    tracing::info!(
        "GPU MSI mode {} for {} (priority={})",
        if enable { "enabled" } else { "disabled" },
        pnp_instance_id,
        priority
    );

    Ok(())
}

/// Check if Hardware-Accelerated GPU Scheduling (HAGS) is enabled.
pub fn is_hags_enabled() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey_with_flags(
        "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
        KEY_READ | KEY_WOW64_64KEY,
    )
    .or_else(|_| hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers"))
    .and_then(|k| k.get_value::<u32, _>("HwSchMode"))
    .map(|v| v == 2)
    .unwrap_or(false)
}

/// Enable or disable Hardware-Accelerated GPU Scheduling.
pub fn set_hags(enable: bool) -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm
        .create_subkey_with_flags(
            "SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers",
            KEY_READ | KEY_WRITE | KEY_WOW64_64KEY,
        )
        .or_else(|_| hklm.create_subkey("SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers"))?;
    key.set_value("HwSchMode", &(if enable { 2u32 } else { 1u32 }))?;
    tracing::info!(
        "HAGS set to {}",
        if enable {
            "enabled (2)"
        } else {
            "disabled (1)"
        }
    );
    Ok(())
}

/// Check if Multi-Plane Overlay (MPO) is disabled.
pub fn is_mpo_disabled() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\Dwm",
        KEY_READ | KEY_WOW64_64KEY,
    )
    .or_else(|_| hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\Dwm"))
    .and_then(|k| k.get_value::<u32, _>("OverlayTestMode"))
    .map(|v| v == 5)
    .unwrap_or(false)
}

/// Disable or enable Multi-Plane Overlay (MPO).
pub fn set_mpo_disabled(disable: bool) -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm
        .create_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\Dwm",
            KEY_READ | KEY_WRITE | KEY_WOW64_64KEY,
        )
        .or_else(|_| hklm.create_subkey("SOFTWARE\\Microsoft\\Windows\\Dwm"))?;
    if disable {
        key.set_value("OverlayTestMode", &5u32)?;
    } else {
        let _ = key.delete_value("OverlayTestMode");
    }
    tracing::info!(
        "MPO {}",
        if disable {
            "disabled (5)"
        } else {
            "restored (default)"
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_vendor_no_false_positive_ati() {
        assert_eq!(
            detect_vendor("", "Workstation Display Adapter"),
            GpuVendor::Unknown
        );
        assert_eq!(
            detect_vendor("", "Virtualization Graphics Controller"),
            GpuVendor::Unknown
        );
        assert_eq!(
            detect_vendor("", "NVIDIA Corporation Display"),
            GpuVendor::Nvidia
        );
        assert_eq!(detect_vendor("", "ATI Radeon HD 5870"), GpuVendor::Amd);
        assert_eq!(detect_vendor("", "ATI-Graphics Controller"), GpuVendor::Amd);
        assert_eq!(detect_vendor("", "ATI"), GpuVendor::Amd);
        assert_eq!(
            detect_vendor("", "Advanced Micro Devices, Inc."),
            GpuVendor::Amd
        );
        assert_eq!(
            detect_vendor("PCI\\VEN_1002&DEV_744C", "Display"),
            GpuVendor::Amd
        );
        assert_eq!(detect_vendor("1002", ""), GpuVendor::Amd);
    }
}
