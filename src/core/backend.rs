//! System execution and sensing backend abstraction (`SystemBackend`, `RealBackend`, `SandboxEnv`).
//!
//! Decouples hardware, registry, SCM service, Task Scheduler, AppX, hosts, and telemetry calls
//! from live OS globals so WinBlast0 can execute both against live Windows systems and against
//! an isolated, parallel-safe in-memory + temp-dir sandbox environment for automated E2E testing.

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock};

use chrono::Utc;
use uuid::Uuid;

use crate::backup::restore_point::{self, RestorePoint};
use crate::core::elevation;
use crate::core::error::{Result, WinBlastError};
use crate::core::os_detect::WindowsInfo;
use crate::core::reg_helpers::{
    validate_registry_mutation_path, RegRawSnapshot, RegStringValue, RegTreeSnapshot,
};
use crate::core::risk::{
    normalize_reg_value_name, FeatureInstallState, OptionalFeatureKind, TweakAction,
    TweakDefinition,
};
use crate::hardware::cpu::CpuInfo;
use crate::hardware::gpu::{self, DetectedGpu, GpuVendor};
use crate::hardware::latency::LatencySnapshot;
use crate::hardware::monitor::{SystemMetrics, SystemMonitor};
use crate::hardware::net_latency::{self, DpcJitterResult, LatencyBenchmarkSnapshot, TargetPingResult};
use crate::hardware::parking::{self, CoreParkingState, CoreStatus};
use crate::hardware::power::{self, HibernationStatus, PowerSchemeInfo};
use crate::hardware::sys_inventory::{
    self, DriveMediaKind, PlatformSecurityInfo, RamDimmInfo, StorageVolumeInfo,
};
use crate::scanner::appx_scanner::AppxCache;
use crate::scanner::task_scanner::{normalize_task_path, TaskCache};
use crate::tools::cleaner::{CleanExecutionReport, CleanTargetKind, CleanTargetResult, CleanTargetScan};
use crate::tools::explorer_ctrl;
use crate::tools::features_apps::{self, AppxSafetyTier, InstalledAppxPackage, WindowsOptionalFeature};
use crate::tools::startup::{self, LiveScheduledTaskInfo, LiveServiceInfo, StartupEntry, StartupSource};

/// Stateful sampler trait for 1-second Task Manager telemetry updates.
pub trait MetricsSampler: Send {
    fn sample(&mut self) -> SystemMetrics;
}

impl MetricsSampler for SystemMonitor {
    fn sample(&mut self) -> SystemMetrics {
        SystemMonitor::sample(self)
    }
}

/// Abstract OS sensing and mutation backend.
pub trait SystemBackend: Send + Sync + 'static {
    fn is_elevated(&self) -> bool;
    fn detect_os(&self) -> Result<WindowsInfo>;
    fn default_ledger_path(&self) -> PathBuf;
    fn default_profile_path(&self) -> PathBuf;

    // Registry
    fn reg_get_dword(&self, hive: &str, path: &str, name: &str) -> Option<u32>;
    fn reg_set_dword(&self, hive: &str, path: &str, name: &str, value: u32) -> Result<()>;
    fn reg_delete_value(&self, hive: &str, path: &str, name: &str) -> Result<()>;

    // Vendor-aware & extended lossless Registry primitives
    fn reg_get_dword_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<u32> {
        let _ = gpu_vendor;
        self.reg_get_dword(hive, path, name)
    }
    fn reg_set_dword_for_vendor(&self, hive: &str, path: &str, name: &str, value: u32, gpu_vendor: Option<&str>) -> Result<()> {
        let _ = gpu_vendor;
        self.reg_set_dword(hive, path, name, value)
    }
    fn reg_delete_value_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Result<()> {
        let _ = gpu_vendor;
        self.reg_delete_value(hive, path, name)
    }
    fn reg_get_string(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<RegStringValue>;
    fn reg_set_string(&self, hive: &str, path: &str, name: &str, value: &str, expand_sz: bool, gpu_vendor: Option<&str>) -> Result<()>;
    fn reg_get_binary(&self, hive: &str, path: &str, name: &str) -> Option<Vec<u8>>;
    fn reg_set_binary(&self, hive: &str, path: &str, name: &str, bytes: &[u8]) -> Result<()>;
    fn reg_get_raw(&self, hive: &str, path: &str, name: &str) -> Option<RegRawSnapshot>;
    fn reg_set_raw(&self, hive: &str, path: &str, name: &str, raw: &RegRawSnapshot) -> Result<()>;
    fn reg_key_exists(&self, hive: &str, path: &str) -> bool;
    fn reg_snapshot_tree(&self, hive: &str, path: &str) -> Result<Option<RegTreeSnapshot>>;
    fn reg_delete_tree(&self, hive: &str, path: &str) -> Result<()>;
    fn reg_restore_tree(&self, hive: &str, path: &str, snapshot: &RegTreeSnapshot) -> Result<()>;

    // Hardware telemetry & dynamic formula support
    fn total_physical_ram_kb(&self) -> u64;
    fn logical_core_count(&self) -> u32;
    fn detect_platform_security(&self) -> PlatformSecurityInfo;

    // Power Management (native powrprof.dll)
    fn get_active_power_scheme(&self) -> Result<String>;
    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo>;
    fn set_active_power_scheme(&self, scheme_guid: &str) -> Result<()>;
    fn create_and_activate_performance_plan(&self) -> Result<String>;
    fn read_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str) -> Result<(u32, u32)>;
    fn write_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str, ac: Option<u32>, dc: Option<u32>, unhide: bool) -> Result<()>;
    fn query_hibernation_status(&self) -> HibernationStatus;
    fn set_hibernation_enabled(&self, enable: bool) -> Result<HibernationStatus>;
    fn set_usb_root_hub_power_saving(&self, disable_power_saving: bool) -> Result<usize>;
    fn set_usb_power_saving_enabled(&self, enable: bool) -> Result<usize> {
        self.set_usb_root_hub_power_saving(!enable)
    }

    // Services
    fn get_service_start_type(&self, service_name: &str) -> Option<u32>;
    fn set_service_start_type(&self, service_name: &str, start_type: u32) -> Result<()>;

    // Scheduled Tasks
    fn load_task_cache(&self) -> TaskCache;
    fn get_task_enabled(&self, task_path: &str) -> Option<bool>;
    fn set_scheduled_task_enabled(&self, task_path: &str, enable: bool) -> Result<()>;

    // AppX Packages
    fn load_appx_cache(&self) -> AppxCache;
    fn remove_appx_package(&self, package_family: &str) -> Result<()>;

    // Hosts File
    fn read_hosts_file(&self) -> Option<String>;
    fn write_hosts_file(&self, content: &str) -> Result<()>;
    fn block_telemetry_hosts(&self, hosts: &[&str]) -> Result<usize>;

    // VSS System Restore Points
    fn list_restore_points(&self) -> Result<Vec<RestorePoint>>;
    fn create_restore_point(&self, description: &str) -> Result<()>;
    fn ensure_preflight_restore_point(&self, description: &str) -> Result<bool> {
        let points = self.list_restore_points()?;
        if !points.is_empty() {
            let now_utc = chrono::Utc::now();
            let max_age_chrono = chrono::Duration::hours(24);
            for point in &points {
                if let Some(created_at) = restore_point::parse_restore_point_time(&point.creation_time) {
                    let diff = now_utc.signed_duration_since(created_at);
                    if diff >= -chrono::Duration::minutes(5) && diff <= max_age_chrono {
                        return Ok(false);
                    }
                }
            }
        }
        self.create_restore_point(description)?;
        Ok(true)
    }

    // Hardware & Telemetry
    fn detect_cpu(&self) -> CpuInfo;
    fn detect_gpus(&self) -> Vec<DetectedGpu>;
    fn set_gpu_msi_mode(&self, pnp_instance_id: &str, enable: bool, priority: u32) -> std::io::Result<()>;
    fn is_hags_enabled(&self) -> bool;
    fn set_hags(&self, enable: bool) -> std::io::Result<()>;
    fn is_mpo_disabled(&self) -> bool;
    fn set_mpo_disabled(&self, disable: bool) -> std::io::Result<()>;
    fn detect_core_parking(&self) -> CoreParkingState;
    fn detect_core_parking_sampled(&self) -> CoreParkingState {
        self.detect_core_parking()
    }
    fn unpark_all_cores(&self) -> Result<()>;
    fn restore_stock_parking(&self) -> Result<()>;
    fn apply_parking_snapshot(&self, snap: &parking::ParkingSnapshot) -> Result<()> {
        self.write_power_setting_index(
            None,
            parking::SUB_PROCESSOR,
            parking::CPMINCORES,
            Some(snap.min_ac),
            Some(snap.min_dc),
            false,
        )?;
        self.write_power_setting_index(
            None,
            parking::SUB_PROCESSOR,
            parking::CPMAXCORES,
            Some(snap.max_ac),
            Some(snap.max_dc),
            false,
        )?;
        for (guid, val_opt) in [
            (parking::CPMINCORES1, snap.min1),
            (parking::CPMAXCORES1, snap.max1),
        ] {
            if let Some((ac, dc)) = val_opt {
                if let Err(err) = self.write_power_setting_index(
                    None,
                    parking::SUB_PROCESSOR,
                    guid,
                    Some(ac),
                    Some(dc),
                    false,
                ) {
                    if !is_unsupported_power_guid_error(&err) {
                        tracing::warn!(
                            "Unexpected error restoring Class 1 core parking setting {}: {}",
                            guid,
                            err
                        );
                    }
                }
            }
        }
        Ok(())
    }
    fn capture_latency(&self) -> LatencySnapshot;
    fn run_latency_benchmark(&self, stage: &str) -> LatencyBenchmarkSnapshot;
    fn create_metrics_sampler(&self, max_history: usize) -> Box<dyn MetricsSampler>;

    // Disk Cleanup, Dynamic AppX, Optional Features, Edge/OneDrive & WinGet
    fn scan_clean_targets(&self) -> Vec<CleanTargetScan>;
    fn execute_clean_targets(&self, kinds: &[CleanTargetKind]) -> Result<CleanExecutionReport>;
    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage>;
    fn remove_appx_batch(&self, families: &[String], remove_provisioned: bool) -> Result<usize>;
    fn list_optional_features(&self) -> Result<Vec<WindowsOptionalFeature>>;
    fn set_optional_feature(&self, name: &str, kind: OptionalFeatureKind, enable: bool) -> Result<FeatureInstallState>;
    fn check_onedrive_kfm_active(&self) -> Option<String>;
    fn uninstall_onedrive_safe(&self) -> Result<String>;
    fn remove_edge_preserve_webview2(&self) -> Result<String>;
    fn winget_install_package(&self, package_id: &str) -> Result<String>;

    // Startup Manager, Live Services, Live Scheduled Tasks, Safe Explorer & Reboot
    fn list_startup_entries(&self) -> Result<Vec<StartupEntry>>;
    fn set_startup_entry_enabled(&self, entry: &StartupEntry, enable: bool) -> Result<()>;
    fn set_startup_entry_enabled_by_id(&self, id: &str, enable: bool) -> Result<()>;
    fn list_live_services(&self) -> Result<Vec<LiveServiceInfo>>;
    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo>;
    fn run_scheduled_task(&self, task_path: &str) -> Result<()>;
    fn restart_explorer_safely(&self) -> Result<String>;
    fn broadcast_shell_settings_changed(&self);
    fn request_system_reboot(&self) -> Result<()>;
}

/// Live Windows OS backend delegating to Win32, Registry, SCM, schtasks, PowerShell, and sysinfo.
#[derive(Debug, Default)]
pub struct RealBackend;

/// Resolve persistent writable WinBlast0 data directory (`%ProgramData%\WinBlast0` -> `%LocalAppData%\WinBlast0` fallback).
fn resolve_persistent_app_dir() -> PathBuf {
    let candidates = [
        std::env::var("ProgramData")
            .ok()
            .map(|p| PathBuf::from(p).join("WinBlast0")),
        std::env::var("LOCALAPPDATA")
            .ok()
            .map(|p| PathBuf::from(p).join("WinBlast0")),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf())),
    ];
    for cand in candidates.into_iter().flatten() {
        if std::fs::create_dir_all(&cand).is_ok() {
            // Verify directory write permission with an atomic probe
            let probe = cand.join(".write_probe");
            if std::fs::write(&probe, b"").is_ok() {
                let _ = std::fs::remove_file(&probe);
                return cand;
            }
        }
    }
    std::env::temp_dir().join("WinBlast0")
}

impl SystemBackend for RealBackend {
    fn is_elevated(&self) -> bool {
        elevation::is_elevated()
    }

    fn detect_os(&self) -> Result<WindowsInfo> {
        WindowsInfo::detect()
    }

    fn default_ledger_path(&self) -> PathBuf {
        resolve_persistent_app_dir().join("winblast0_ledger.db")
    }

    fn default_profile_path(&self) -> PathBuf {
        resolve_persistent_app_dir().join("winblast0_profile.wb0")
    }

    fn reg_get_dword(&self, hive: &str, path: &str, name: &str) -> Option<u32> {
        crate::scanner::registry_scanner::get_dword(hive, path, name)
    }

    fn reg_set_dword(&self, hive: &str, path: &str, name: &str, value: u32) -> Result<()> {
        crate::executor::reg_mutator::set_dword(hive, path, name, value)
    }

    fn reg_delete_value(&self, hive: &str, path: &str, name: &str) -> Result<()> {
        crate::executor::reg_mutator::delete_value(hive, path, name)
    }

    fn reg_get_dword_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<u32> {
        crate::scanner::registry_scanner::get_dword_for_vendor(hive, path, name, gpu_vendor)
    }

    fn reg_set_dword_for_vendor(&self, hive: &str, path: &str, name: &str, value: u32, gpu_vendor: Option<&str>) -> Result<()> {
        crate::executor::reg_mutator::set_dword_for_vendor(hive, path, name, value, gpu_vendor)
    }

    fn reg_delete_value_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Result<()> {
        crate::executor::reg_mutator::delete_value_for_vendor(hive, path, name, gpu_vendor)
    }

    fn reg_get_string(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<RegStringValue> {
        crate::scanner::registry_scanner::get_string_for_vendor(hive, path, name, gpu_vendor)
    }

    fn reg_set_string(&self, hive: &str, path: &str, name: &str, value: &str, expand_sz: bool, gpu_vendor: Option<&str>) -> Result<()> {
        crate::executor::reg_mutator::set_string_for_vendor(hive, path, name, value, expand_sz, gpu_vendor)
    }

    fn reg_get_binary(&self, hive: &str, path: &str, name: &str) -> Option<Vec<u8>> {
        crate::scanner::registry_scanner::get_binary(hive, path, name)
    }

    fn reg_set_binary(&self, hive: &str, path: &str, name: &str, bytes: &[u8]) -> Result<()> {
        crate::executor::reg_mutator::set_binary(hive, path, name, bytes)
    }

    fn reg_get_raw(&self, hive: &str, path: &str, name: &str) -> Option<RegRawSnapshot> {
        crate::scanner::registry_scanner::get_raw(hive, path, name)
    }

    fn reg_set_raw(&self, hive: &str, path: &str, name: &str, raw: &RegRawSnapshot) -> Result<()> {
        crate::executor::reg_mutator::set_raw(hive, path, name, raw)
    }

    fn reg_key_exists(&self, hive: &str, path: &str) -> bool {
        crate::scanner::registry_scanner::key_exists(hive, path)
    }

    fn reg_snapshot_tree(&self, hive: &str, path: &str) -> Result<Option<RegTreeSnapshot>> {
        crate::scanner::registry_scanner::snapshot_tree(hive, path)
    }

    fn reg_delete_tree(&self, hive: &str, path: &str) -> Result<()> {
        crate::executor::reg_mutator::delete_tree(hive, path)
    }

    fn reg_restore_tree(&self, hive: &str, path: &str, snapshot: &RegTreeSnapshot) -> Result<()> {
        crate::executor::reg_mutator::restore_tree(hive, path, snapshot)
    }

    fn total_physical_ram_kb(&self) -> u64 {
        sys_inventory::total_physical_ram_kb()
    }

    fn logical_core_count(&self) -> u32 {
        std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(8)
    }

    fn detect_platform_security(&self) -> PlatformSecurityInfo {
        sys_inventory::detect_platform_security_info()
    }

    fn get_active_power_scheme(&self) -> Result<String> {
        power::get_active_power_scheme()
    }

    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo> {
        power::list_power_schemes()
    }

    fn set_active_power_scheme(&self, scheme_guid: &str) -> Result<()> {
        power::set_active_power_scheme(scheme_guid)
    }

    fn create_and_activate_performance_plan(&self) -> Result<String> {
        power::create_and_activate_performance_plan()
    }

    fn read_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str) -> Result<(u32, u32)> {
        power::read_power_setting_index(scheme_guid, subgroup_guid, setting_guid)
    }

    fn write_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str, ac: Option<u32>, dc: Option<u32>, unhide: bool) -> Result<()> {
        power::write_power_setting_index(scheme_guid, subgroup_guid, setting_guid, ac, dc, unhide)
    }

    fn query_hibernation_status(&self) -> HibernationStatus {
        power::query_hibernation_and_standby()
    }

    fn set_hibernation_enabled(&self, enable: bool) -> Result<HibernationStatus> {
        power::set_hibernation_enabled(enable)
    }

    fn set_usb_root_hub_power_saving(&self, disable_power_saving: bool) -> Result<usize> {
        power::set_usb_root_hub_power_saving(disable_power_saving)
    }

    fn get_service_start_type(&self, service_name: &str) -> Option<u32> {
        crate::scanner::service_scanner::get_service_start_type(service_name)
    }

    fn set_service_start_type(&self, service_name: &str, start_type: u32) -> Result<()> {
        crate::executor::svc_mutator::set_service_start_type(service_name, start_type)
    }

    fn load_task_cache(&self) -> TaskCache {
        TaskCache::load()
    }

    fn get_task_enabled(&self, task_path: &str) -> Option<bool> {
        crate::scanner::task_scanner::get_task_enabled(task_path)
    }

    fn set_scheduled_task_enabled(&self, task_path: &str, enable: bool) -> Result<()> {
        crate::executor::ps_runner::set_scheduled_task_enabled(task_path, enable)
    }

    fn load_appx_cache(&self) -> AppxCache {
        AppxCache::load()
    }

    fn remove_appx_package(&self, package_family: &str) -> Result<()> {
        crate::executor::ps_runner::remove_appx_package(package_family)
    }

    fn read_hosts_file(&self) -> Option<String> {
        std::fs::read_to_string(r"C:\Windows\System32\drivers\etc\hosts").ok()
    }

    fn write_hosts_file(&self, content: &str) -> Result<()> {
        crate::executor::ps_runner::write_hosts_file_raw(content)
    }

    fn block_telemetry_hosts(&self, hosts: &[&str]) -> Result<usize> {
        crate::executor::ps_runner::block_telemetry_hosts(hosts)
    }

    fn list_restore_points(&self) -> Result<Vec<RestorePoint>> {
        restore_point::list_restore_points()
    }

    fn create_restore_point(&self, description: &str) -> Result<()> {
        restore_point::create_restore_point(description)
    }

    fn detect_cpu(&self) -> CpuInfo {
        CpuInfo::detect()
    }

    fn detect_gpus(&self) -> Vec<DetectedGpu> {
        gpu::detect_gpus()
    }

    fn set_gpu_msi_mode(&self, pnp_instance_id: &str, enable: bool, priority: u32) -> std::io::Result<()> {
        gpu::set_gpu_msi_mode(pnp_instance_id, enable, priority)
    }

    fn is_hags_enabled(&self) -> bool {
        gpu::is_hags_enabled()
    }

    fn set_hags(&self, enable: bool) -> std::io::Result<()> {
        gpu::set_hags(enable)
    }

    fn is_mpo_disabled(&self) -> bool {
        gpu::is_mpo_disabled()
    }

    fn set_mpo_disabled(&self, disable: bool) -> std::io::Result<()> {
        gpu::set_mpo_disabled(disable)
    }

    fn detect_core_parking(&self) -> CoreParkingState {
        CoreParkingState::detect()
    }

    fn detect_core_parking_sampled(&self) -> CoreParkingState {
        CoreParkingState::detect_sampled()
    }

    fn unpark_all_cores(&self) -> Result<()> {
        parking::unpark_all_cores()
    }

    fn restore_stock_parking(&self) -> Result<()> {
        parking::restore_stock_parking()
    }

    fn apply_parking_snapshot(&self, snap: &parking::ParkingSnapshot) -> Result<()> {
        parking::apply_parking_snapshot(snap)
    }

    fn capture_latency(&self) -> LatencySnapshot {
        LatencySnapshot::capture()
    }

    fn run_latency_benchmark(&self, stage: &str) -> LatencyBenchmarkSnapshot {
        net_latency::run_full_latency_benchmark(stage)
    }

    fn create_metrics_sampler(&self, max_history: usize) -> Box<dyn MetricsSampler> {
        Box::new(SystemMonitor::new(max_history))
    }

    fn scan_clean_targets(&self) -> Vec<CleanTargetScan> {
        crate::tools::cleaner::scan_all_clean_targets()
    }

    fn execute_clean_targets(&self, kinds: &[CleanTargetKind]) -> Result<CleanExecutionReport> {
        crate::tools::cleaner::execute_clean_targets(kinds)
    }

    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage> {
        let cache = AppxCache::load();
        features_apps::list_installed_appx_from_cache(&cache)
    }

    fn remove_appx_batch(&self, families: &[String], remove_provisioned: bool) -> Result<usize> {
        features_apps::remove_appx_batch(families, remove_provisioned)
    }

    fn list_optional_features(&self) -> Result<Vec<WindowsOptionalFeature>> {
        if !self.is_elevated() {
            return Err(WinBlastError::ElevationRequired(
                "Optional feature scan requires Administrator privileges".to_string(),
            ));
        }
        features_apps::list_optional_features()
    }

    fn set_optional_feature(&self, name: &str, kind: OptionalFeatureKind, enable: bool) -> Result<FeatureInstallState> {
        features_apps::set_optional_feature(name, kind, enable)
    }

    fn check_onedrive_kfm_active(&self) -> Option<String> {
        features_apps::check_onedrive_kfm_active()
    }

    fn uninstall_onedrive_safe(&self) -> Result<String> {
        features_apps::uninstall_onedrive_safe()
    }

    fn remove_edge_preserve_webview2(&self) -> Result<String> {
        features_apps::remove_edge_preserve_webview2()
    }

    fn winget_install_package(&self, package_id: &str) -> Result<String> {
        features_apps::winget_install_package(package_id)
    }

    fn list_startup_entries(&self) -> Result<Vec<StartupEntry>> {
        startup::list_startup_entries()
    }

    fn set_startup_entry_enabled(&self, entry: &StartupEntry, enable: bool) -> Result<()> {
        startup::set_startup_entry_enabled(entry, enable)
    }

    fn set_startup_entry_enabled_by_id(&self, id: &str, enable: bool) -> Result<()> {
        startup::set_startup_entry_enabled_by_id(id, enable)
    }

    fn list_live_services(&self) -> Result<Vec<LiveServiceInfo>> {
        startup::list_live_services()
    }

    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo> {
        let cache = TaskCache::load();
        startup::list_live_tasks_from_cache(&cache)
    }

    fn run_scheduled_task(&self, task_path: &str) -> Result<()> {
        startup::run_scheduled_task(task_path)
    }

    fn restart_explorer_safely(&self) -> Result<String> {
        explorer_ctrl::restart_explorer_safely()
    }

    fn broadcast_shell_settings_changed(&self) {
        explorer_ctrl::broadcast_shell_settings_changed();
    }

    fn request_system_reboot(&self) -> Result<()> {
        explorer_ctrl::request_system_reboot()
    }
}

/// Check whether a PowerCfg error indicates an absent/unsupported power setting GUID on this hardware.
pub(crate) fn is_unsupported_power_guid_error(err: &WinBlastError) -> bool {
    let lower = err.to_string().to_ascii_lowercase();
    lower.contains("0x00000002")
        || lower.contains("0x00000490")
        || lower.contains("absent")
        || lower.contains("not found")
}

/// Cached `SystemBackend` wrapper used by `run_full_scan_with` to load `TaskCache` / `AppxCache` at most once.
pub(crate) struct CachedScanBackend<'a> {
    inner: &'a dyn SystemBackend,
    task_cache: OnceLock<TaskCache>,
    appx_cache: OnceLock<AppxCache>,
    optional_features: OnceLock<std::result::Result<Vec<WindowsOptionalFeature>, String>>,
    gpu_vendors: OnceLock<Vec<GpuVendor>>,
}

impl<'a> CachedScanBackend<'a> {
    pub(crate) fn new(inner: &'a dyn SystemBackend) -> Self {
        Self {
            inner,
            task_cache: OnceLock::new(),
            appx_cache: OnceLock::new(),
            optional_features: OnceLock::new(),
            gpu_vendors: OnceLock::new(),
        }
    }

    pub(crate) fn tasks(&self) -> &TaskCache {
        self.task_cache.get_or_init(|| self.inner.load_task_cache())
    }

    pub(crate) fn appx(&self) -> &AppxCache {
        self.appx_cache.get_or_init(|| self.inner.load_appx_cache())
    }

    pub(crate) fn optional_features(&self) -> std::result::Result<&[WindowsOptionalFeature], &str> {
        match self.optional_features.get_or_init(|| {
            self.inner
                .list_optional_features()
                .map_err(|e| e.to_string())
        }) {
            Ok(v) => Ok(v.as_slice()),
            Err(e) => Err(e.as_str()),
        }
    }

    pub(crate) fn gpu_vendors(&self) -> &[GpuVendor] {
        self.gpu_vendors.get_or_init(|| {
            self.inner
                .detect_gpus()
                .into_iter()
                .map(|g| g.vendor)
                .collect()
        })
    }

    pub(crate) fn inner(&self) -> &'a dyn SystemBackend {
        self.inner
    }
}

// =============================================================================
// Isolated Parallel-Safe Sandbox Test Environment (`SandboxEnv`)
// =============================================================================

/// Operations recorded inside `SandboxEnv` for deterministic E2E assertions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandboxOp {
    RegSet { hive: String, path: String, name: String, value: u32 },
    RegSetString { hive: String, path: String, name: String, value: String, expand_sz: bool },
    RegSetBinary { hive: String, path: String, name: String, len: usize },
    RegDelete { hive: String, path: String, name: String },
    RegDeleteTree { hive: String, path: String },
    PowerCfgSet { scheme: String, subgroup: String, setting: String, ac: Option<u32>, dc: Option<u32> },
    ServiceSet { name: String, start_type: u32 },
    TaskSet { path: String, enable: bool },
    AppxRemove { family: String },
    FeatureSet { name: String, kind: OptionalFeatureKind, enable: bool },
    CleanExecuted { targets: usize, freed_bytes: u64 },
    StartupToggled { id: String, enable: bool },
    ExplorerRestarted,
    RebootRequested,
    BlockHosts { added: usize },
    GpuMsi { pnp_id: String, enable: bool, priority: u32 },
    SetHags(bool),
    SetMpo(bool),
    UnparkCores,
    RestoreParking,
    CreateRestorePoint(String),
}

/// Auto-cleaning ephemeral directory backing sandbox SQLite ledgers and `.wb0` profiles.
#[derive(Debug)]
pub struct SandboxTempDir {
    pub path: PathBuf,
}

impl SandboxTempDir {
    pub fn new() -> Self {
        let path = std::env::temp_dir().join(format!("winblast0_sbx_{}", Uuid::new_v4()));
        let _ = std::fs::create_dir_all(&path);
        Self { path }
    }
}

impl Default for SandboxTempDir {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SandboxTempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// Mutable state container inside [`SandboxEnv`].
#[derive(Debug, Clone)]
pub struct SandboxState {
    pub elevated: bool,
    pub os_info: WindowsInfo,
    pub registry: HashMap<(String, String, String), u32>,
    pub reg_strings: HashMap<(String, String, String), RegStringValue>,
    pub reg_binaries: HashMap<(String, String, String), Vec<u8>>,
    pub reg_raw: HashMap<(String, String, String), RegRawSnapshot>,
    pub reg_trees: HashMap<(String, String), RegTreeSnapshot>,
    pub active_power_scheme: String,
    pub power_schemes: Vec<PowerSchemeInfo>,
    pub power_settings: HashMap<(String, String, String), (u32, u32)>,
    pub total_ram_kb: u64,
    pub logical_cores: u32,
    pub hibernation: HibernationStatus,
    pub platform_security: PlatformSecurityInfo,
    pub clean_targets: Vec<CleanTargetScan>,
    pub optional_features: Vec<WindowsOptionalFeature>,
    pub startup_entries: Vec<StartupEntry>,
    pub live_services: Vec<LiveServiceInfo>,
    pub onedrive_kfm_path: Option<String>,
    pub services: HashMap<String, u32>,
    pub tasks: HashMap<String, bool>,
    pub appx_packages: HashSet<String>,
    pub hosts_content: String,
    pub cpu_info: CpuInfo,
    pub gpus: Vec<DetectedGpu>,
    pub hags_enabled: bool,
    pub mpo_disabled: bool,
    pub core_parking: CoreParkingState,
    pub restore_points: Vec<RestorePoint>,
    pub metrics_samples: VecDeque<SystemMetrics>,
    pub latency_snapshot: LatencySnapshot,
    pub latency_benchmark_before: LatencyBenchmarkSnapshot,
    pub latency_benchmark_after: LatencyBenchmarkSnapshot,
    /// Optional forced error returned on the next mutation (tests UI deadlock/rollback recovery).
    pub inject_next_error: Option<String>,
    /// Audit trail of every mutation executed in the sandbox.
    pub ops: Vec<SandboxOp>,
}

impl Default for SandboxState {
    fn default() -> Self {
        let mut cores = Vec::new();
        for i in 0..8 {
            cores.push(CoreStatus {
                core_index: i,
                is_parked: i >= 4,
                usage_pct: if i < 4 { 24.5 + i as f32 * 5.0 } else { 0.0 },
                core_type_label: if i < 4 { "P-Core".to_string() } else { "E-Core".to_string() },
            });
        }

        let clean_targets: Vec<CleanTargetScan> = CleanTargetKind::ALL
            .iter()
            .enumerate()
            .map(|(idx, &kind)| CleanTargetScan {
                kind,
                paths: vec![format!("C:\\Sandbox\\{}", kind.id_key())],
                reclaimable_bytes: ((idx as u64) + 1) * 12_500_000,
                file_count: ((idx as u64) + 1) * 35,
                selected: kind.default_selected(),
                requires_elevation: matches!(
                    kind,
                    CleanTargetKind::WindowsTemp
                        | CleanTargetKind::WindowsUpdateCache
                        | CleanTargetKind::DeliveryOptimizationCache
                        | CleanTargetKind::WerAndCrashDumps
                        | CleanTargetKind::Prefetch
                ),
                last_error: None,
            })
            .collect();

        let optional_features: Vec<WindowsOptionalFeature> =
            features_apps::curated_windows_features_meta()
                .into_iter()
                .map(|(name, display, desc, kind, bloat)| WindowsOptionalFeature {
                    name: name.to_string(),
                    display_name: display.to_string(),
                    description: desc.to_string(),
                    kind,
                    state: if bloat {
                        FeatureInstallState::Enabled
                    } else {
                        FeatureInstallState::Disabled
                    },
                    is_curated_bloat: bloat,
                })
                .collect();

        Self {
            elevated: true,
            os_info: WindowsInfo {
                build_number: 26100,
                display_version: "24H2".to_string(),
                edition: "Pro".to_string(),
                is_win11: true,
            },
            registry: HashMap::new(),
            reg_strings: HashMap::new(),
            reg_binaries: HashMap::new(),
            reg_raw: HashMap::new(),
            reg_trees: HashMap::new(),
            active_power_scheme: "381b4222-f694-41f0-9685-ff5bb260df2e".to_string(),
            power_schemes: vec![
                PowerSchemeInfo {
                    guid: "381b4222-f694-41f0-9685-ff5bb260df2e".to_string(),
                    friendly_name: "Balanced".to_string(),
                    is_active: true,
                },
                PowerSchemeInfo {
                    guid: "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c".to_string(),
                    friendly_name: "High Performance".to_string(),
                    is_active: false,
                },
            ],
            power_settings: HashMap::new(),
            total_ram_kb: 33_554_432,
            logical_cores: 8,
            hibernation: HibernationStatus {
                hibernation_enabled: true,
                hiberfile_present: true,
                fast_startup_enabled: true,
                modern_standby_s0ix: false,
            },
            platform_security: PlatformSecurityInfo {
                firmware_mode: "UEFI".to_string(),
                secure_boot_enabled: Some(true),
                vbs_enabled: true,
                hvci_enabled: true,
                motherboard: "Sandbox Reference Board Z790".to_string(),
                bios_version: "1.40".to_string(),
                total_physical_ram_kb: 33_554_432,
                ram_summary: "32 GB DDR5 @ 6000 MT/s (2 DIMMs)".to_string(),
                ram_dimms: vec![
                    RamDimmInfo {
                        locator: "DIMM_A1".to_string(),
                        manufacturer: "Micron".to_string(),
                        ddr_type: "DDR5".to_string(),
                        speed_mts: 6000,
                        capacity_mb: 16384,
                    },
                    RamDimmInfo {
                        locator: "DIMM_B1".to_string(),
                        manufacturer: "Micron".to_string(),
                        ddr_type: "DDR5".to_string(),
                        speed_mts: 6000,
                        capacity_mb: 16384,
                    },
                ],
                volumes: vec![StorageVolumeInfo {
                    drive_letter: "C:".to_string(),
                    label: "Windows".to_string(),
                    file_system: "NTFS".to_string(),
                    total_gb: 930.5,
                    free_gb: 512.0,
                    used_pct: 45.0,
                    media_kind: DriveMediaKind::Nvme,
                    is_system_drive: true,
                }],
                uptime_hours: 4.5,
            },
            clean_targets,
            optional_features,
            startup_entries: vec![StartupEntry {
                id: "HKCU|Run|OneDrive".to_string(),
                name: "OneDrive".to_string(),
                command: "\"C:\\Program Files\\Microsoft OneDrive\\OneDrive.exe\" /background".to_string(),
                source: StartupSource::RegistryHkcuRun,
                enabled: true,
                location: "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
                hive: "HKCU".to_string(),
                reg_path: "Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
                approved_subkey: Some("Run".to_string()),
                is_wow64: false,
                policy_locked: false,
                is_microsoft: true,
            }],
            live_services: {
                let ls = vec![
                    LiveServiceInfo {
                        service_name: "DiagTrack".to_string(),
                        display_name: "Connected User Experiences and Telemetry".to_string(),
                        start_type: 2,
                        is_running: true,
                        binary_path: "C:\\Windows\\System32\\svchost.exe -k utcsvc -p".to_string(),
                        is_protected_core: false,
                        is_microsoft: true,
                    },
                    LiveServiceInfo {
                        service_name: "RpcSs".to_string(),
                        display_name: "Remote Procedure Call (RPC)".to_string(),
                        start_type: 2,
                        is_running: true,
                        binary_path: "C:\\Windows\\System32\\svchost.exe -k rpcss -p".to_string(),
                        is_protected_core: true,
                        is_microsoft: true,
                    },
                ];
                ls
            },
            onedrive_kfm_path: None,
            services: [
                ("diagtrack".to_string(), 2u32),
                ("rpcss".to_string(), 2u32),
            ]
            .into_iter()
            .collect(),
            tasks: HashMap::new(),
            appx_packages: HashSet::new(),
            hosts_content: "127.0.0.1 localhost\r\n".to_string(),
            cpu_info: CpuInfo {
                vendor: "GenuineIntel".to_string(),
                model_name: "Intel(R) Core(TM) i9-14900K (Sandbox)".to_string(),
                physical_cores: 8,
                logical_cores: 8,
                has_hybrid_topology: true,
                p_core_count: 4,
                e_core_count: 4,
            },
            gpus: vec![
                DetectedGpu {
                    name: "NVIDIA GeForce RTX 4090 (Sandbox)".to_string(),
                    pnp_instance_id: "PCI\\VEN_10DE&DEV_2684&SUBSYS_000010DE&REV_A1\\4&1234567&0&0008".to_string(),
                    msi_enabled: true,
                    interrupt_priority: 2,
                    vendor: GpuVendor::Nvidia,
                    class_subkey: "0000".to_string(),
                    dedicated_vram_mb: 24576,
                },
                DetectedGpu {
                    name: "AMD Radeon RX 7900 XTX (Sandbox)".to_string(),
                    pnp_instance_id: "PCI\\VEN_1002&DEV_744C&SUBSYS_00001002&REV_C8\\4&7654321&0&0008".to_string(),
                    msi_enabled: false,
                    interrupt_priority: 1,
                    vendor: GpuVendor::Amd,
                    class_subkey: "0001".to_string(),
                    dedicated_vram_mb: 24576,
                },
                DetectedGpu {
                    name: "Intel(R) UHD Graphics 770 (Sandbox)".to_string(),
                    pnp_instance_id: "PCI\\VEN_8086&DEV_A780&SUBSYS_00008086&REV_04\\3&11583659&0&10".to_string(),
                    msi_enabled: false,
                    interrupt_priority: 1,
                    vendor: GpuVendor::Intel,
                    class_subkey: "0002".to_string(),
                    dedicated_vram_mb: 1024,
                },
            ],
            hags_enabled: true,
            mpo_disabled: false,
            core_parking: CoreParkingState {
                total_cores: 8,
                unparked_count: 4,
                parked_count: 4,
                min_cores_ac_pct: 50,
                min_cores_dc_pct: 10,
                max_cores_ac_pct: 100,
                is_fully_unparked: false,
                is_hybrid_architecture: true,
                cores,
                on_ac_power: true,
                max_cores_dc_pct: 100,
                pcore_min_ac_pct: Some(50),
                pcore_min_dc_pct: Some(10),
                pcore_max_ac_pct: Some(100),
                pcore_max_dc_pct: Some(100),
            },
            restore_points: vec![RestorePoint {
                sequence_number: 1,
                description: "WinBlast0 Sandbox Checkpoint".to_string(),
                creation_time: "2026-11-10 08:00:00".to_string(),
            }],
            metrics_samples: VecDeque::new(),
            latency_snapshot: LatencySnapshot {
                timer_resolution_ns: 15_625_000,
                total_ram_mb: 32768,
                used_ram_mb: 11200,
                process_count: 142,
                thread_count: 2150,
                timestamp: Utc::now(),
            },
            latency_benchmark_before: LatencyBenchmarkSnapshot {
                stage: "BEFORE".to_string(),
                timestamp: "2026-11-10T08:00:00Z".to_string(),
                dpc: DpcJitterResult {
                    samples_count: 500,
                    avg_gap_us: 1012.4,
                    peak_jitter_us: 420,
                    timer_resolution_ms: 15.625,
                },
                targets: vec![TargetPingResult {
                    name: "Cloudflare (1.1.1.1)".to_string(),
                    rtt_min_ms: 11.2,
                    rtt_avg_ms: 14.5,
                    rtt_max_ms: 21.0,
                    jitter_ms: 1.8,
                    packet_loss_pct: 0.0,
                }],
                overall_avg_ping_ms: 14.5,
                overall_avg_jitter_ms: 1.8,
            },
            latency_benchmark_after: LatencyBenchmarkSnapshot {
                stage: "AFTER".to_string(),
                timestamp: "2026-11-10T08:05:00Z".to_string(),
                dpc: DpcJitterResult {
                    samples_count: 500,
                    avg_gap_us: 502.1,
                    peak_jitter_us: 110,
                    timer_resolution_ms: 0.500,
                },
                targets: vec![TargetPingResult {
                    name: "Cloudflare (1.1.1.1)".to_string(),
                    rtt_min_ms: 10.1,
                    rtt_avg_ms: 11.9,
                    rtt_max_ms: 15.0,
                    jitter_ms: 0.7,
                    packet_loss_pct: 0.0,
                }],
                overall_avg_ping_ms: 11.9,
                overall_avg_jitter_ms: 0.7,
            },
            inject_next_error: None,
            ops: Vec::new(),
        }
    }
}

/// Isolated, thread-safe sandbox environment implementing [`SystemBackend`].
#[derive(Debug, Clone)]
pub struct SandboxEnv {
    pub inner: Arc<RwLock<SandboxState>>,
    pub temp_dir: Arc<SandboxTempDir>,
}

impl Default for SandboxEnv {
    fn default() -> Self {
        Self::new()
    }
}

fn optional_feature_matches(candidate: &str, query: &str) -> bool {
    if candidate.eq_ignore_ascii_case(query) {
        return true;
    }
    let c_prefix = candidate.split_once("~~~~").map(|(p, _)| p).unwrap_or(candidate);
    let q_prefix = query.split_once("~~~~").map(|(p, _)| p).unwrap_or(query);
    !c_prefix.is_empty() && c_prefix.eq_ignore_ascii_case(q_prefix)
}

impl SandboxEnv {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SandboxState::default())),
            temp_dir: Arc::new(SandboxTempDir::new()),
        }
    }

    pub fn normalize_hive(hive: &str) -> String {
        match hive.trim().to_ascii_uppercase().as_str() {
            "HKLM" | "HKEY_LOCAL_MACHINE" => "HKLM".to_string(),
            "HKCU" | "HKEY_CURRENT_USER" => "HKCU".to_string(),
            "HKU" | "HKEY_USERS" => "HKU".to_string(),
            "HKCR" | "HKEY_CLASSES_ROOT" => "HKCR".to_string(),
            other => other.to_string(),
        }
    }

    pub fn normalize_reg_path(path: &str) -> String {
        path.trim().trim_matches(['\\', '/']).to_lowercase()
    }

    pub fn reg_key(hive: &str, path: &str, name: &str) -> (String, String, String) {
        (
            Self::normalize_hive(hive),
            Self::normalize_reg_path(path),
            normalize_reg_value_name(name).to_lowercase(),
        )
    }

    fn clear_reg_value_in_state(st: &mut SandboxState, key: &(String, String, String)) {
        st.registry.remove(key);
        st.reg_strings.remove(key);
        st.reg_binaries.remove(key);
        st.reg_raw.remove(key);
    }

    fn ensure_tree_path_in_state(st: &mut SandboxState, hive: &str, path: &str) {
        let norm_hive = Self::normalize_hive(hive);
        let norm_path = Self::normalize_reg_path(path);
        if norm_path.is_empty() {
            return;
        }
        let mut cur = String::new();
        for seg in norm_path.split('\\').filter(|s| !s.is_empty()) {
            if !cur.is_empty() {
                cur.push('\\');
            }
            cur.push_str(seg);
            st.reg_trees
                .entry((norm_hive.clone(), cur.clone()))
                .or_insert_with(|| RegTreeSnapshot {
                    subkey: String::new(),
                    values: Vec::new(),
                    children: Vec::new(),
                });
        }
    }

    fn write_dword_in_state(
        st: &mut SandboxState,
        hive: &str,
        path: &str,
        name: &str,
        value: u32,
    ) {
        let norm_name = normalize_reg_value_name(name);
        let key = Self::reg_key(hive, path, name);
        let (existing_numeric_sz, existing_expand_sz, existing_neg) =
            match st.reg_strings.get(&key) {
                Some(s) => {
                    let t = s.value.trim();
                    let is_u32 = t.parse::<u32>().is_ok();
                    let neg_i32 = !is_u32 && t.parse::<i32>().is_ok_and(|n| n < 0);
                    (is_u32 || t.parse::<i32>().is_ok(), s.expand_sz, neg_i32)
                }
                None => (false, false, false),
            };
        let known_sz = crate::core::reg_helpers::is_known_numeric_reg_sz(path, norm_name);
        Self::clear_reg_value_in_state(st, &key);
        Self::ensure_tree_path_in_state(st, hive, path);

        if existing_numeric_sz || known_sz {
            let str_val = if (existing_neg || known_sz) && (value as i32) < 0 {
                (value as i32).to_string()
            } else {
                value.to_string()
            };
            st.reg_strings.insert(
                key,
                RegStringValue {
                    value: str_val,
                    expand_sz: existing_expand_sz,
                },
            );
        } else {
            st.registry.insert(key, value);
        }
    }

    fn write_raw_in_state(
        st: &mut SandboxState,
        hive: &str,
        path: &str,
        name: &str,
        raw: &RegRawSnapshot,
    ) -> Result<()> {
        let bytes = crate::core::reg_helpers::decode_hex(&raw.hex_bytes)?;
        let key = Self::reg_key(hive, path, name);
        Self::clear_reg_value_in_state(st, &key);
        Self::ensure_tree_path_in_state(st, hive, path);
        match raw.reg_type {
            4 if bytes.len() == 4 => {
                let dw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                st.registry.insert(key, dw);
            }
            3 => {
                st.reg_binaries.insert(key, bytes);
            }
            1 | 2 if bytes.len() % 2 == 0 => {
                let mut units: Vec<u16> = bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|&c| u16::from_le_bytes(c))
                    .collect();
                while units.last() == Some(&0) {
                    units.pop();
                }
                st.reg_strings.insert(
                    key,
                    RegStringValue {
                        value: String::from_utf16_lossy(&units),
                        expand_sz: raw.reg_type == 2,
                    },
                );
            }
            _ => {
                st.reg_raw.insert(key, raw.clone());
            }
        }
        Ok(())
    }

    fn restore_tree_into_state(
        st: &mut SandboxState,
        hive: &str,
        path: &str,
        snapshot: &RegTreeSnapshot,
    ) -> Result<()> {
        let norm_hive = Self::normalize_hive(hive);
        let norm_path = Self::normalize_reg_path(path);
        Self::ensure_tree_path_in_state(st, &norm_hive, &norm_path);
        st.reg_trees
            .insert((norm_hive.clone(), norm_path.clone()), snapshot.clone());
        for (vname, raw) in &snapshot.values {
            Self::write_raw_in_state(st, &norm_hive, &norm_path, vname, raw)?;
        }
        for child in &snapshot.children {
            let child_path = format!("{}\\{}", norm_path, child.subkey);
            Self::restore_tree_into_state(st, &norm_hive, &child_path, child)?;
        }
        Ok(())
    }

    fn remove_appx_family_in_state(
        st: &mut SandboxState,
        clean_lower: &str,
        raw_family: &str,
    ) -> usize {
        let clean_norm =
            features_apps::normalize_appx_family_name(clean_lower).to_ascii_lowercase();
        let (name_part, pub_part) =
            crate::scanner::appx_scanner::split_appx_name_and_publisher(&clean_norm);
        let before = st.appx_packages.len();
        st.appx_packages.retain(|pkg| {
            let pkg_lower = pkg.to_ascii_lowercase();
            !crate::scanner::appx_scanner::appx_package_matches_family(
                &pkg_lower,
                &clean_norm,
                name_part,
                pub_part,
            )
        });
        let removed = before.saturating_sub(st.appx_packages.len());
        st.ops.push(SandboxOp::AppxRemove {
            family: raw_family.to_string(),
        });
        removed
    }

    fn resolve_net_interface_paths_in_state(
        st: &SandboxState,
        hive: &str,
        path: &str,
    ) -> (bool, Vec<String>) {
        const IFACES_PREFIX_LOWER: &str =
            "system\\currentcontrolset\\services\\tcpip\\parameters\\interfaces\\";
        const FALLBACK_GUID: &str = "{00000000-0000-0000-0000-000000000000}";

        let is_hklm = hive.trim().eq_ignore_ascii_case("HKLM")
            || hive.trim().eq_ignore_ascii_case("HKEY_LOCAL_MACHINE");
        let lower = path.trim().to_ascii_lowercase();
        if !is_hklm || !lower.contains("<net_interface_guid>") {
            return (false, vec![path.to_string()]);
        }

        let mut guids: Vec<String> = Vec::new();
        for (h, p, _) in st
            .registry
            .keys()
            .chain(st.reg_strings.keys())
            .chain(st.reg_binaries.keys())
            .chain(st.reg_raw.keys())
        {
            if h == "HKLM" && p.starts_with(IFACES_PREFIX_LOWER) {
                let rest = &p[IFACES_PREFIX_LOWER.len()..];
                let seg = rest.split('\\').next().unwrap_or("");
                if !seg.is_empty()
                    && seg != "<net_interface_guid>"
                    && !guids.iter().any(|g| g.eq_ignore_ascii_case(seg))
                {
                    guids.push(seg.to_string());
                }
            }
        }
        guids.sort();
        if guids.is_empty() {
            guids.push(FALLBACK_GUID.to_string());
        }
        let resolved = guids
            .into_iter()
            .map(|g| {
                crate::scanner::registry_scanner::replace_placeholder_ci(
                    path,
                    "<NET_INTERFACE_GUID>",
                    &g,
                )
            })
            .collect();
        (true, resolved)
    }

    fn resolve_target_paths_in_state(
        st: &SandboxState,
        hive: &str,
        path: &str,
        gpu_vendor: Option<&str>,
    ) -> Result<Vec<String>> {
        let (is_net, net_paths) = Self::resolve_net_interface_paths_in_state(st, hive, path);
        if is_net {
            return Ok(net_paths);
        }

        const DISPLAY_CLASS_LOWER: &str =
            "system\\currentcontrolset\\control\\class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
        if !hive.eq_ignore_ascii_case("HKLM") && !hive.eq_ignore_ascii_case("HKEY_LOCAL_MACHINE") {
            return Ok(vec![path.to_string()]);
        }
        let lower = path.to_ascii_lowercase();
        if !lower.starts_with(DISPLAY_CLASS_LOWER) {
            return Ok(vec![path.to_string()]);
        }
        let rest = path[DISPLAY_CLASS_LOWER.len()..].trim_start_matches('\\');
        let (sub_seg, suffix) = match rest.find('\\') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, ""),
        };
        let wanted = gpu::parse_target_vendor(gpu_vendor);
        if wanted.is_none() && !sub_seg.is_empty() && !sub_seg.eq_ignore_ascii_case("0000") {
            return Ok(vec![path.to_string()]);
        }
        let mut out = Vec::new();
        for g in &st.gpus {
            if g.class_subkey.is_empty() {
                continue;
            }
            let is_match = match wanted {
                Some(w) => g.vendor == w,
                None => g.vendor == GpuVendor::Nvidia || g.vendor == GpuVendor::Amd,
            };
            if is_match {
                out.push(format!(
                    "{}\\{}{}",
                    gpu::DISPLAY_CLASS_PREFIX, g.class_subkey, suffix
                ));
            }
        }
        if out.is_empty() {
            if let Some(w) = wanted {
                return Err(WinBlastError::Hardware(format!(
                    "No {} GPU adapter detected in sandbox",
                    w.as_str()
                )));
            }
            Ok(vec![path.to_string()])
        } else {
            Ok(out)
        }
    }

    /// Seed initial "stock unoptimized" Windows values matching all given tweak actions.
    pub fn seed_tweak_defaults(&self, actions: &[TweakAction]) {
        self.seed_actions_with_vendor(actions, None);
    }

    fn seed_actions_with_vendor(&self, actions: &[TweakAction], gpu_vendor: Option<&str>) {
        let mut st = self.inner.write().unwrap();
        let active_scheme = st.active_power_scheme.clone();
        for action in actions {
            match action {
                TweakAction::RegistryDword {
                    hive,
                    path,
                    name,
                    default_value,
                    ..
                } => {
                    let paths = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor)
                        .unwrap_or_else(|_| vec![path.to_string()]);
                    for rp in paths {
                        Self::write_dword_in_state(&mut st, hive, &rp, name, *default_value);
                    }
                }
                TweakAction::RegistryString {
                    hive,
                    path,
                    name,
                    default_value,
                    expand_sz,
                    absent_is_default,
                    ..
                } => {
                    let paths = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor)
                        .unwrap_or_else(|_| vec![path.to_string()]);
                    for rp in paths {
                        let k = Self::reg_key(hive, &rp, name);
                        Self::clear_reg_value_in_state(&mut st, &k);
                        if !(*absent_is_default || default_value == "__ABSENT__") {
                            Self::ensure_tree_path_in_state(&mut st, hive, &rp);
                            st.reg_strings.insert(
                                k,
                                RegStringValue {
                                    value: default_value.clone(),
                                    expand_sz: *expand_sz,
                                },
                            );
                        }
                    }
                }
                TweakAction::RegistryDelete { hive, path, name } => {
                    let k = Self::reg_key(hive, path, name);
                    Self::clear_reg_value_in_state(&mut st, &k);
                    Self::ensure_tree_path_in_state(&mut st, hive, path);
                    st.reg_raw.insert(
                        k,
                        RegRawSnapshot {
                            reg_type: 4,
                            hex_bytes: "01000000".to_string(),
                        },
                    );
                }
                TweakAction::RegistryDeleteTree { hive, path } => {
                    let k = (Self::normalize_hive(hive), Self::normalize_reg_path(path));
                    Self::ensure_tree_path_in_state(&mut st, hive, path);
                    st.reg_trees.insert(
                        k,
                        RegTreeSnapshot {
                            subkey: String::new(),
                            values: vec![(
                                "".to_string(),
                                RegRawSnapshot {
                                    reg_type: 1,
                                    hex_bytes: "74006500730074000000".to_string(),
                                },
                            )],
                            children: Vec::new(),
                        },
                    );
                }
                TweakAction::RegistryBinaryBit {
                    hive,
                    path,
                    name,
                    byte_index,
                    min_length,
                    bit_mask,
                    default_bits,
                    default_full_hex,
                    ..
                } => {
                    let k = Self::reg_key(hive, path, name);
                    let mut buf = st
                        .reg_binaries
                        .get(&k)
                        .cloned()
                        .or_else(|| {
                            default_full_hex
                                .as_deref()
                                .and_then(|h| crate::core::reg_helpers::decode_hex(h).ok())
                        })
                        .unwrap_or_else(|| {
                            vec![0x9E, 0x3E, 0x07, 0x80, 0x12, 0x00, 0x00, 0x00]
                        });
                    let req_len = (*min_length).max(byte_index + 1);
                    if buf.len() < req_len {
                        buf.resize(req_len, 0);
                    }
                    buf[*byte_index] = (buf[*byte_index] & !bit_mask) | (default_bits & bit_mask);
                    Self::clear_reg_value_in_state(&mut st, &k);
                    Self::ensure_tree_path_in_state(&mut st, hive, path);
                    st.reg_binaries.insert(k, buf);
                }
                TweakAction::CompositeStringKey {
                    hive,
                    path,
                    name,
                    entry_key,
                    default_entry_value,
                    pair_separator,
                    kv_separator,
                    trailing_separator,
                    ..
                } => {
                    let k = Self::reg_key(hive, path, name);
                    if default_entry_value != "__ABSENT__" {
                        let existing = st.reg_strings.get(&k).map(|s| s.value.clone());
                        let (merged, _) = crate::core::reg_helpers::merge_composite_pair(
                            existing.as_deref(),
                            entry_key,
                            default_entry_value,
                            pair_separator,
                            kv_separator,
                            *trailing_separator,
                        );
                        if !merged.is_empty() {
                            Self::clear_reg_value_in_state(&mut st, &k);
                            Self::ensure_tree_path_in_state(&mut st, hive, path);
                            st.reg_strings.insert(
                                k,
                                RegStringValue {
                                    value: merged,
                                    expand_sz: false,
                                },
                            );
                        }
                    }
                }
                TweakAction::RegistryDynamicDword {
                    hive,
                    path,
                    name,
                    default_value,
                    ..
                } => {
                    Self::write_dword_in_state(&mut st, hive, path, name, *default_value);
                }
                TweakAction::PowerCfgSetting {
                    subgroup_guid,
                    setting_guid,
                    scheme_guid,
                    default_ac,
                    default_dc,
                    ..
                } => {
                    let scheme = scheme_guid
                        .as_deref()
                        .map(power::normalize_guid_str)
                        .unwrap_or_else(|| active_scheme.clone());
                    st.power_settings.insert(
                        (
                            scheme,
                            power::normalize_guid_str(subgroup_guid),
                            power::normalize_guid_str(setting_guid),
                        ),
                        (*default_ac, *default_dc),
                    );
                }
                TweakAction::ServiceState {
                    service_name,
                    default_start_type,
                    ..
                } => {
                    let svc_key = service_name.trim().to_lowercase();
                    st.services.insert(svc_key, *default_start_type);
                    if let Some(ls) = st
                        .live_services
                        .iter_mut()
                        .find(|s| s.service_name.eq_ignore_ascii_case(service_name.trim()))
                    {
                        ls.start_type = *default_start_type;
                        if *default_start_type == 4 {
                            ls.is_running = false;
                        }
                    }
                }
                TweakAction::ScheduledTask { task_path, enable } => {
                    st.tasks.insert(normalize_task_path(task_path), !*enable);
                }
                TweakAction::AppXPackage { package_family } => {
                    let clean = features_apps::normalize_appx_family_name(package_family.trim())
                        .to_ascii_lowercase();
                    let (name, pub_opt) =
                        crate::scanner::appx_scanner::split_appx_name_and_publisher(&clean);
                    if let Some(pub_id) = pub_opt {
                        st.appx_packages
                            .insert(format!("{}_2.1.0.0_x64__{}", name, pub_id));
                    } else {
                        st.appx_packages.insert(clean);
                    }
                }
                TweakAction::WindowsFeature {
                    feature_name,
                    kind,
                    enable,
                } => {
                    let target_state = if *enable {
                        FeatureInstallState::Disabled
                    } else {
                        FeatureInstallState::Enabled
                    };
                    if let Some(f) = st
                        .optional_features
                        .iter_mut()
                        .find(|f| optional_feature_matches(&f.name, feature_name))
                    {
                        f.state = target_state;
                    } else {
                        st.optional_features.push(WindowsOptionalFeature {
                            name: feature_name.clone(),
                            display_name: feature_name.clone(),
                            description: "Sandbox feature".to_string(),
                            kind: *kind,
                            state: target_state,
                            is_curated_bloat: false,
                        });
                    }
                }
            }
        }
    }

    /// Seed defaults specifically resolving multi-GPU paths via optional `gpu_vendor`.
    pub fn seed_tweak_defaults_for_tweak(&self, tweak: &TweakDefinition) {
        self.seed_actions_with_vendor(&tweak.actions, tweak.gpu_vendor.as_deref());
    }

    /// Seed defaults for every tweak in a slice (ideal for populated UI preview/testing).
    pub fn seed_all_tweaks(&self, tweaks: &[TweakDefinition]) {
        for tweak in tweaks {
            self.seed_tweak_defaults_for_tweak(tweak);
        }
    }

    /// Inspect recorded operations in the sandbox.
    pub fn recorded_ops(&self) -> Vec<SandboxOp> {
        self.inner.read().unwrap().ops.clone()
    }

    fn take_injected_error(&self) -> Result<()> {
        let mut st = self.inner.write().unwrap();
        if let Some(msg) = st.inject_next_error.take() {
            return Err(WinBlastError::TweakFailed(msg));
        }
        Ok(())
    }

    fn sync_parking_in_state(st: &mut SandboxState, snap: &parking::ParkingSnapshot) {
        let scheme = st.active_power_scheme.clone();
        let sub = parking::SUB_PROCESSOR.to_string();
        st.power_settings.insert(
            (scheme.clone(), sub.clone(), parking::CPMINCORES.to_string()),
            (snap.min_ac, snap.min_dc),
        );
        st.power_settings.insert(
            (scheme.clone(), sub.clone(), parking::CPMAXCORES.to_string()),
            (snap.max_ac, snap.max_dc),
        );
        if st.core_parking.is_hybrid_architecture {
            if let Some(v) = snap.min1 {
                st.power_settings.insert(
                    (scheme.clone(), sub.clone(), parking::CPMINCORES1.to_string()),
                    v,
                );
            }
            if let Some(v) = snap.max1 {
                st.power_settings.insert(
                    (scheme, sub, parking::CPMAXCORES1.to_string()),
                    v,
                );
            }
        }

        st.core_parking.on_ac_power = true;
        st.core_parking.min_cores_ac_pct = snap.min_ac;
        st.core_parking.min_cores_dc_pct = snap.min_dc;
        st.core_parking.max_cores_ac_pct = snap.max_ac;
        st.core_parking.max_cores_dc_pct = snap.max_dc;
        st.core_parking.pcore_min_ac_pct = snap.min1.map(|(ac, _)| ac);
        st.core_parking.pcore_min_dc_pct = snap.min1.map(|(_, dc)| dc);
        st.core_parking.pcore_max_ac_pct = snap.max1.map(|(ac, _)| ac);
        st.core_parking.pcore_max_dc_pct = snap.max1.map(|(_, dc)| dc);

        let is_unparked = parking::compute_is_fully_unparked(
            true,
            st.core_parking.is_hybrid_architecture,
            snap.min_ac,
            snap.min_dc,
            snap.max_ac,
            snap.max_dc,
            snap.min1,
            snap.max1,
        );
        st.core_parking.is_fully_unparked = is_unparked;

        let total = st.core_parking.total_cores;
        let active = if is_unparked {
            total
        } else {
            let eff_pct = snap.min_ac.min(snap.max_ac);
            ((total as f64 * (eff_pct as f64 / 100.0)).ceil() as usize).clamp(1, total)
        };
        st.core_parking.unparked_count = active;
        st.core_parking.parked_count = total.saturating_sub(active);
        for (i, c) in st.core_parking.cores.iter_mut().enumerate() {
            c.is_parked = i >= active;
        }
    }
}

struct SandboxMetricsSampler {
    inner: Arc<RwLock<SandboxState>>,
    tick: f32,
}

impl MetricsSampler for SandboxMetricsSampler {
    fn sample(&mut self) -> SystemMetrics {
        let mut st = self.inner.write().unwrap();
        if let Some(m) = st.metrics_samples.pop_front() {
            return m;
        }
        self.tick += 1.0;
        let cpu = 18.0 + (self.tick * 0.7).sin().abs() * 25.0;
        SystemMetrics {
            cpu_usage: cpu,
            core_usages: st.core_parking.cores.iter().map(|c| c.usage_pct).collect(),
            core_parked: st.core_parking.cores.iter().map(|c| c.is_parked).collect(),
            total_ram_mb: 32768,
            used_ram_mb: 11200,
            available_ram_mb: 21568,
            ram_percentage: 34.2,
            process_count: 142,
            thread_count: 2150,
            timer_resolution_ns: 15_625_000,
            timestamp: Utc::now(),
        }
    }
}

impl SystemBackend for SandboxEnv {
    fn is_elevated(&self) -> bool {
        self.inner.read().unwrap().elevated
    }

    fn detect_os(&self) -> Result<WindowsInfo> {
        Ok(self.inner.read().unwrap().os_info.clone())
    }

    fn default_ledger_path(&self) -> PathBuf {
        self.temp_dir.path.join("winblast0_sandbox_ledger.db")
    }

    fn default_profile_path(&self) -> PathBuf {
        self.temp_dir.path.join("winblast0_sandbox_profile.wb0")
    }

    fn reg_get_dword(&self, hive: &str, path: &str, name: &str) -> Option<u32> {
        self.reg_get_dword_for_vendor(hive, path, name, None)
    }

    fn reg_set_dword(&self, hive: &str, path: &str, name: &str, value: u32) -> Result<()> {
        self.reg_set_dword_for_vendor(hive, path, name, value, None)
    }

    fn reg_delete_value(&self, hive: &str, path: &str, name: &str) -> Result<()> {
        self.reg_delete_value_for_vendor(hive, path, name, None)
    }

    fn reg_get_dword_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<u32> {
        let st = self.inner.read().unwrap();
        let resolved = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor).ok()?;
        if resolved.is_empty() {
            return None;
        }
        let mut consensus: Option<u32> = None;
        for rp in resolved {
            let key = Self::reg_key(hive, &rp, name);
            let val = if let Some(&dw) = st.registry.get(&key) {
                dw
            } else if let Some(s) = st.reg_strings.get(&key) {
                let t = s.value.trim();
                if let Ok(u) = t.parse::<u32>() {
                    u
                } else {
                    t.parse::<i32>().ok()? as u32
                }
            } else {
                let raw = st.reg_raw.get(&key)?;
                let bytes = crate::core::reg_helpers::decode_hex(&raw.hex_bytes).ok()?;
                match raw.reg_type {
                    4 if bytes.len() >= 4 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    5 if bytes.len() >= 4 => u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    11 if bytes.len() >= 8 => {
                        let q = u64::from_le_bytes([
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                        ]);
                        u32::try_from(q).ok()?
                    }
                    _ => return None,
                }
            };
            match consensus {
                None => consensus = Some(val),
                Some(prev) if prev == val => {}
                Some(_) => return None,
            }
        }
        consensus
    }

    fn reg_set_dword_for_vendor(&self, hive: &str, path: &str, name: &str, value: u32, gpu_vendor: Option<&str>) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, false)?;
        let norm_hive = Self::normalize_hive(hive);
        let mut st = self.inner.write().unwrap();
        let resolved = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor)?;
        for rp in resolved {
            Self::write_dword_in_state(&mut st, &norm_hive, &rp, name, value);
            st.ops.push(SandboxOp::RegSet {
                hive: norm_hive.clone(),
                path: rp,
                name: name.to_string(),
                value,
            });
        }
        Ok(())
    }

    fn reg_delete_value_for_vendor(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, false)?;
        let norm_hive = Self::normalize_hive(hive);
        let mut st = self.inner.write().unwrap();
        let resolved = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor)?;
        for rp in resolved {
            let key = Self::reg_key(&norm_hive, &rp, name);
            Self::clear_reg_value_in_state(&mut st, &key);
            st.ops.push(SandboxOp::RegDelete {
                hive: norm_hive.clone(),
                path: rp,
                name: name.to_string(),
            });
        }
        Ok(())
    }

    fn reg_get_string(&self, hive: &str, path: &str, name: &str, gpu_vendor: Option<&str>) -> Option<RegStringValue> {
        let st = self.inner.read().unwrap();
        let resolved = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor).ok()?;
        for rp in resolved {
            let key = Self::reg_key(hive, &rp, name);
            if let Some(v) = st.reg_strings.get(&key) {
                return Some(v.clone());
            }
        }
        None
    }

    fn reg_set_string(&self, hive: &str, path: &str, name: &str, value: &str, expand_sz: bool, gpu_vendor: Option<&str>) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, false)?;
        let norm_hive = Self::normalize_hive(hive);
        let mut st = self.inner.write().unwrap();
        let resolved = Self::resolve_target_paths_in_state(&st, hive, path, gpu_vendor)?;
        for rp in resolved {
            let key = Self::reg_key(&norm_hive, &rp, name);
            Self::clear_reg_value_in_state(&mut st, &key);
            Self::ensure_tree_path_in_state(&mut st, &norm_hive, &rp);
            st.reg_strings.insert(
                key,
                RegStringValue {
                    value: value.to_string(),
                    expand_sz,
                },
            );
            st.ops.push(SandboxOp::RegSetString {
                hive: norm_hive.clone(),
                path: rp,
                name: name.to_string(),
                value: value.to_string(),
                expand_sz,
            });
        }
        Ok(())
    }

    fn reg_get_binary(&self, hive: &str, path: &str, name: &str) -> Option<Vec<u8>> {
        let key = Self::reg_key(hive, path, name);
        self.inner.read().unwrap().reg_binaries.get(&key).cloned()
    }

    fn reg_set_binary(&self, hive: &str, path: &str, name: &str, bytes: &[u8]) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, false)?;
        let norm_hive = Self::normalize_hive(hive);
        let key = Self::reg_key(&norm_hive, path, name);
        let mut st = self.inner.write().unwrap();
        Self::clear_reg_value_in_state(&mut st, &key);
        Self::ensure_tree_path_in_state(&mut st, &norm_hive, path);
        st.reg_binaries.insert(key, bytes.to_vec());
        st.ops.push(SandboxOp::RegSetBinary {
            hive: norm_hive,
            path: path.to_string(),
            name: name.to_string(),
            len: bytes.len(),
        });
        Ok(())
    }

    fn reg_get_raw(&self, hive: &str, path: &str, name: &str) -> Option<RegRawSnapshot> {
        let key = Self::reg_key(hive, path, name);
        let st = self.inner.read().unwrap();
        if let Some(raw) = st.reg_raw.get(&key) {
            return Some(raw.clone());
        }
        if let Some(dw) = st.registry.get(&key) {
            return Some(RegRawSnapshot {
                reg_type: 4,
                hex_bytes: crate::core::reg_helpers::encode_hex(&dw.to_le_bytes()),
            });
        }
        if let Some(bin) = st.reg_binaries.get(&key) {
            return Some(RegRawSnapshot {
                reg_type: 3,
                hex_bytes: crate::core::reg_helpers::encode_hex(bin),
            });
        }
        if let Some(s) = st.reg_strings.get(&key) {
            let mut bytes = Vec::with_capacity((s.value.len() + 1) * 2);
            for unit in s.value.encode_utf16() {
                bytes.extend_from_slice(&unit.to_le_bytes());
            }
            bytes.extend_from_slice(&[0u8, 0u8]);
            return Some(RegRawSnapshot {
                reg_type: if s.expand_sz { 2 } else { 1 },
                hex_bytes: crate::core::reg_helpers::encode_hex(&bytes),
            });
        }
        None
    }

    fn reg_set_raw(&self, hive: &str, path: &str, name: &str, raw: &RegRawSnapshot) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, false)?;
        let mut st = self.inner.write().unwrap();
        Self::write_raw_in_state(&mut st, hive, path, name, raw)
    }

    fn reg_key_exists(&self, hive: &str, path: &str) -> bool {
        let norm_hive = Self::normalize_hive(hive);
        let norm_path = Self::normalize_reg_path(path);
        let prefix = format!("{}\\", norm_path);
        let st = self.inner.read().unwrap();
        if st.reg_trees.contains_key(&(norm_hive.clone(), norm_path.clone())) {
            return true;
        }
        st.registry
            .keys()
            .chain(st.reg_strings.keys())
            .chain(st.reg_binaries.keys())
            .chain(st.reg_raw.keys())
            .any(|(h, p, _)| h == &norm_hive && (p == &norm_path || p.starts_with(&prefix)))
    }

    fn reg_snapshot_tree(&self, hive: &str, path: &str) -> Result<Option<RegTreeSnapshot>> {
        let k = (Self::normalize_hive(hive), Self::normalize_reg_path(path));
        Ok(self.inner.read().unwrap().reg_trees.get(&k).cloned())
    }

    fn reg_delete_tree(&self, hive: &str, path: &str) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, true)?;
        let norm_hive = Self::normalize_hive(hive);
        let norm_path = Self::normalize_reg_path(path);
        let prefix = format!("{}\\", norm_path);
        let mut st = self.inner.write().unwrap();
        st.reg_trees
            .retain(|(h, p), _| !(h == &norm_hive && (p == &norm_path || p.starts_with(&prefix))));
        st.registry
            .retain(|(h, p, _), _| !(h == &norm_hive && (p == &norm_path || p.starts_with(&prefix))));
        st.reg_strings
            .retain(|(h, p, _), _| !(h == &norm_hive && (p == &norm_path || p.starts_with(&prefix))));
        st.reg_binaries
            .retain(|(h, p, _), _| !(h == &norm_hive && (p == &norm_path || p.starts_with(&prefix))));
        st.reg_raw
            .retain(|(h, p, _), _| !(h == &norm_hive && (p == &norm_path || p.starts_with(&prefix))));
        st.ops.push(SandboxOp::RegDeleteTree {
            hive: norm_hive,
            path: path.to_string(),
        });
        Ok(())
    }

    fn reg_restore_tree(&self, hive: &str, path: &str, snapshot: &RegTreeSnapshot) -> Result<()> {
        self.take_injected_error()?;
        validate_registry_mutation_path(hive, path, true)?;
        let mut st = self.inner.write().unwrap();
        Self::restore_tree_into_state(&mut st, hive, path, snapshot)
    }

    fn total_physical_ram_kb(&self) -> u64 {
        self.inner.read().unwrap().total_ram_kb
    }

    fn logical_core_count(&self) -> u32 {
        self.inner.read().unwrap().logical_cores
    }

    fn detect_platform_security(&self) -> PlatformSecurityInfo {
        self.inner.read().unwrap().platform_security.clone()
    }

    fn get_active_power_scheme(&self) -> Result<String> {
        Ok(self.inner.read().unwrap().active_power_scheme.clone())
    }

    fn list_power_schemes(&self) -> Vec<PowerSchemeInfo> {
        self.inner.read().unwrap().power_schemes.clone()
    }

    fn set_active_power_scheme(&self, scheme_guid: &str) -> Result<()> {
        self.take_injected_error()?;
        let _ = power::parse_guid(scheme_guid)?;
        let norm = power::normalize_guid_str(scheme_guid);
        let mut st = self.inner.write().unwrap();
        if !st
            .power_schemes
            .iter()
            .any(|p| p.guid.eq_ignore_ascii_case(&norm))
        {
            return Err(WinBlastError::Hardware(format!(
                "Power scheme {} does not exist in sandbox",
                norm
            )));
        }
        st.active_power_scheme = norm.clone();
        for p in &mut st.power_schemes {
            p.is_active = p.guid.eq_ignore_ascii_case(&norm);
        }
        Ok(())
    }

    fn create_and_activate_performance_plan(&self) -> Result<String> {
        self.take_injected_error()?;
        let guid = "e9a42b02-d5df-448d-aa00-03f14749eb61".to_string();
        let mut st = self.inner.write().unwrap();
        if !st.power_schemes.iter().any(|p| p.guid == guid) {
            st.power_schemes.push(PowerSchemeInfo {
                guid: guid.clone(),
                friendly_name: "WinBlast0 Ultimate Performance".to_string(),
                is_active: true,
            });
        }
        st.active_power_scheme = guid.clone();
        for p in &mut st.power_schemes {
            p.is_active = p.guid == guid;
        }
        Ok(guid)
    }

    fn read_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str) -> Result<(u32, u32)> {
        if let Some(sg) = scheme_guid {
            let _ = power::parse_guid(sg)?;
        }
        let _ = power::parse_guid(subgroup_guid)?;
        let _ = power::parse_guid(setting_guid)?;
        let st = self.inner.read().unwrap();
        let scheme = scheme_guid
            .map(power::normalize_guid_str)
            .unwrap_or_else(|| st.active_power_scheme.clone());
        let key = (
            scheme,
            power::normalize_guid_str(subgroup_guid),
            power::normalize_guid_str(setting_guid),
        );
        st.power_settings.get(&key).copied().ok_or_else(|| {
            WinBlastError::Hardware(format!("Sandbox power setting absent: {} / {}", subgroup_guid, setting_guid))
        })
    }

    fn write_power_setting_index(&self, scheme_guid: Option<&str>, subgroup_guid: &str, setting_guid: &str, ac: Option<u32>, dc: Option<u32>, _unhide: bool) -> Result<()> {
        self.take_injected_error()?;
        if let Some(sg) = scheme_guid {
            let _ = power::parse_guid(sg)?;
        }
        let _ = power::parse_guid(subgroup_guid)?;
        let _ = power::parse_guid(setting_guid)?;
        let mut st = self.inner.write().unwrap();
        let scheme = scheme_guid
            .map(power::normalize_guid_str)
            .unwrap_or_else(|| st.active_power_scheme.clone());
        let sub = power::normalize_guid_str(subgroup_guid);
        let set = power::normalize_guid_str(setting_guid);
        let key = (scheme.clone(), sub.clone(), set.clone());
        let (mut cur_ac, mut cur_dc) = st.power_settings.get(&key).copied().unwrap_or((100, 100));
        if let Some(a) = ac {
            cur_ac = a;
        }
        if let Some(d) = dc {
            cur_dc = d;
        }
        st.power_settings.insert(key, (cur_ac, cur_dc));
        if scheme == st.active_power_scheme
            && sub == parking::SUB_PROCESSOR
            && matches!(
                set.as_str(),
                parking::CPMINCORES
                    | parking::CPMAXCORES
                    | parking::CPMINCORES1
                    | parking::CPMAXCORES1
            )
        {
            let snap = parking::ParkingSnapshot {
                min_ac: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMINCORES.to_string(),
                    ))
                    .map(|&(a, _)| a)
                    .unwrap_or(st.core_parking.min_cores_ac_pct),
                min_dc: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMINCORES.to_string(),
                    ))
                    .map(|&(_, d)| d)
                    .unwrap_or(st.core_parking.min_cores_dc_pct),
                max_ac: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMAXCORES.to_string(),
                    ))
                    .map(|&(a, _)| a)
                    .unwrap_or(st.core_parking.max_cores_ac_pct),
                max_dc: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMAXCORES.to_string(),
                    ))
                    .map(|&(_, d)| d)
                    .unwrap_or(st.core_parking.max_cores_dc_pct),
                min1: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMINCORES1.to_string(),
                    ))
                    .copied(),
                max1: st
                    .power_settings
                    .get(&(
                        scheme.clone(),
                        sub.clone(),
                        parking::CPMAXCORES1.to_string(),
                    ))
                    .copied(),
            };
            Self::sync_parking_in_state(&mut st, &snap);
        }
        st.ops.push(SandboxOp::PowerCfgSet { scheme, subgroup: sub, setting: set, ac, dc });
        Ok(())
    }

    fn query_hibernation_status(&self) -> HibernationStatus {
        self.inner.read().unwrap().hibernation.clone()
    }

    fn set_hibernation_enabled(&self, enable: bool) -> Result<HibernationStatus> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        st.hibernation.hibernation_enabled = enable;
        st.hibernation.hiberfile_present = enable;
        if !enable {
            st.hibernation.fast_startup_enabled = false;
        }
        Ok(st.hibernation.clone())
    }

    fn set_usb_root_hub_power_saving(&self, disable_power_saving: bool) -> Result<usize> {
        self.take_injected_error()?;
        const USB_SUBGROUP: &str = "2a737441-1930-4402-8d77-b2bebba308a3";
        const USB_SELECTIVE_SUSPEND: &str = "48e6b7a6-50f5-4782-a5d4-53bb8f07e226";
        let mut st = self.inner.write().unwrap();
        let scheme = st.active_power_scheme.clone();
        let key = (
            scheme.clone(),
            USB_SUBGROUP.to_string(),
            USB_SELECTIVE_SUSPEND.to_string(),
        );
        let ac_val = if disable_power_saving { 0 } else { 1 };
        let (_, prev_dc) = st.power_settings.get(&key).copied().unwrap_or((1, 1));
        st.power_settings.insert(key, (ac_val, prev_dc));
        st.ops.push(SandboxOp::PowerCfgSet {
            scheme,
            subgroup: USB_SUBGROUP.to_string(),
            setting: USB_SELECTIVE_SUSPEND.to_string(),
            ac: Some(ac_val),
            dc: None,
        });
        Ok(4)
    }

    fn get_service_start_type(&self, service_name: &str) -> Option<u32> {
        self.inner
            .read()
            .unwrap()
            .services
            .get(&service_name.trim().to_lowercase())
            .copied()
    }

    fn set_service_start_type(&self, service_name: &str, start_type: u32) -> Result<()> {
        self.take_injected_error()?;
        let clean = service_name.trim();
        if clean.is_empty() || clean.contains(['\\', '/', '\0']) || clean == ".." {
            return Err(WinBlastError::invalid_input(format!(
                "Invalid Windows service name '{}'",
                service_name
            )));
        }
        if start_type > 4 {
            return Err(WinBlastError::invalid_input(format!(
                "Unsupported service start type {}",
                start_type
            )));
        }
        if start_type == 4 && startup::is_protected_core_service(clean) {
            return Err(WinBlastError::service(
                5,
                format!(
                    "Refused to disable protected OS critical service '{}'",
                    clean
                ),
            ));
        }
        let key = clean.to_lowercase();
        let mut st = self.inner.write().unwrap();
        if !st.services.contains_key(&key) {
            if start_type == 3 || start_type == 4 {
                return Ok(());
            }
            return Err(WinBlastError::registry(
                2,
                format!("Sandbox service absent: {}", clean),
            ));
        }
        st.services.insert(key, start_type);
        if let Some(ls) = st
            .live_services
            .iter_mut()
            .find(|s| s.service_name.eq_ignore_ascii_case(clean))
        {
            ls.start_type = start_type;
            if start_type == 4 {
                ls.is_running = false;
            } else if start_type == 2 && !ls.is_protected_core {
                ls.is_running = true;
            }
        }
        st.ops.push(SandboxOp::ServiceSet {
            name: clean.to_string(),
            start_type,
        });
        Ok(())
    }

    fn load_task_cache(&self) -> TaskCache {
        let tasks = self.inner.read().unwrap().tasks.clone();
        TaskCache::from_map(tasks, true)
    }

    fn get_task_enabled(&self, task_path: &str) -> Option<bool> {
        let norm = normalize_task_path(task_path);
        let st = self.inner.read().unwrap();
        if let Some(&en) = st.tasks.get(&norm) {
            return Some(en);
        }
        let space_variant = format!("{} ", norm);
        let mut hits: Vec<(&String, &bool)> = st
            .tasks
            .iter()
            .filter(|(k, _)| k.starts_with(&space_variant))
            .collect();
        hits.sort_by(|a, b| a.0.cmp(b.0));
        hits.first().map(|(_, &en)| en)
    }

    fn set_scheduled_task_enabled(&self, task_path: &str, enable: bool) -> Result<()> {
        self.take_injected_error()?;
        let norm = normalize_task_path(task_path);
        let mut st = self.inner.write().unwrap();
        let target_key = if st.tasks.contains_key(&norm) {
            Some(norm)
        } else {
            let space_variant = format!("{} ", norm);
            let mut hits: Vec<String> = st
                .tasks
                .keys()
                .filter(|k| k.starts_with(&space_variant))
                .cloned()
                .collect();
            hits.sort();
            hits.into_iter().next()
        };
        let Some(actual_key) = target_key else {
            if !enable {
                return Ok(());
            }
            return Err(WinBlastError::TweakFailed(format!(
                "Sandbox scheduled task absent: {}",
                task_path
            )));
        };
        st.tasks.insert(actual_key, enable);
        st.ops.push(SandboxOp::TaskSet {
            path: task_path.to_string(),
            enable,
        });
        Ok(())
    }

    fn load_appx_cache(&self) -> AppxCache {
        let pkgs = self.inner.read().unwrap().appx_packages.clone();
        AppxCache::from_packages(pkgs)
    }

    fn remove_appx_package(&self, package_family: &str) -> Result<()> {
        self.take_injected_error()?;
        let clean = package_family.trim().to_lowercase();
        if clean.is_empty() {
            return Ok(());
        }
        if features_apps::classify_appx_package(&clean) == AppxSafetyTier::ProtectedCore {
            return Err(WinBlastError::AppX(format!(
                "Refused to remove protected Windows core package '{}'",
                package_family.trim()
            )));
        }
        let mut st = self.inner.write().unwrap();
        Self::remove_appx_family_in_state(&mut st, &clean, package_family);
        Ok(())
    }

    fn read_hosts_file(&self) -> Option<String> {
        Some(self.inner.read().unwrap().hosts_content.clone())
    }

    fn write_hosts_file(&self, content: &str) -> Result<()> {
        self.take_injected_error()?;
        self.inner.write().unwrap().hosts_content = content.to_string();
        Ok(())
    }

    fn block_telemetry_hosts(&self, hosts: &[&str]) -> Result<usize> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let (updated, added) =
            crate::executor::ps_runner::block_telemetry_hosts_in_content(&st.hosts_content, hosts);
        st.hosts_content = updated;
        st.ops.push(SandboxOp::BlockHosts { added });
        Ok(added)
    }

    fn list_restore_points(&self) -> Result<Vec<RestorePoint>> {
        Ok(self.inner.read().unwrap().restore_points.clone())
    }

    fn create_restore_point(&self, description: &str) -> Result<()> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let seq = st.restore_points.len() as u32 + 1;
        st.restore_points.push(RestorePoint {
            sequence_number: seq,
            description: description.to_string(),
            creation_time: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
        st.ops
            .push(SandboxOp::CreateRestorePoint(description.to_string()));
        Ok(())
    }

    fn detect_cpu(&self) -> CpuInfo {
        self.inner.read().unwrap().cpu_info.clone()
    }

    fn detect_gpus(&self) -> Vec<DetectedGpu> {
        self.inner.read().unwrap().gpus.clone()
    }

    fn set_gpu_msi_mode(&self, pnp_instance_id: &str, enable: bool, priority: u32) -> std::io::Result<()> {
        if let Err(e) = self.take_injected_error() {
            return Err(std::io::Error::other(e.to_string()));
        }
        let mut st = self.inner.write().unwrap();
        for g in &mut st.gpus {
            if g.pnp_instance_id == pnp_instance_id {
                g.msi_enabled = enable;
                g.interrupt_priority = priority.clamp(1, 3);
            }
        }
        st.ops.push(SandboxOp::GpuMsi {
            pnp_id: pnp_instance_id.to_string(),
            enable,
            priority,
        });
        Ok(())
    }

    fn is_hags_enabled(&self) -> bool {
        self.inner.read().unwrap().hags_enabled
    }

    fn set_hags(&self, enable: bool) -> std::io::Result<()> {
        if let Err(e) = self.take_injected_error() {
            return Err(std::io::Error::other(e.to_string()));
        }
        let mut st = self.inner.write().unwrap();
        st.hags_enabled = enable;
        st.ops.push(SandboxOp::SetHags(enable));
        Ok(())
    }

    fn is_mpo_disabled(&self) -> bool {
        self.inner.read().unwrap().mpo_disabled
    }

    fn set_mpo_disabled(&self, disable: bool) -> std::io::Result<()> {
        if let Err(e) = self.take_injected_error() {
            return Err(std::io::Error::other(e.to_string()));
        }
        let mut st = self.inner.write().unwrap();
        st.mpo_disabled = disable;
        st.ops.push(SandboxOp::SetMpo(disable));
        Ok(())
    }

    fn detect_core_parking(&self) -> CoreParkingState {
        self.inner.read().unwrap().core_parking.clone()
    }

    fn unpark_all_cores(&self) -> Result<()> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let snap = parking::ParkingSnapshot {
            min_ac: 100,
            min_dc: 100,
            max_ac: 100,
            max_dc: 100,
            min1: Some((100, 100)),
            max1: Some((100, 100)),
        };
        Self::sync_parking_in_state(&mut st, &snap);
        st.ops.push(SandboxOp::UnparkCores);
        Ok(())
    }

    fn restore_stock_parking(&self) -> Result<()> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let snap = parking::ParkingSnapshot {
            min_ac: 10,
            min_dc: 5,
            max_ac: 100,
            max_dc: 100,
            min1: Some((10, 5)),
            max1: Some((100, 100)),
        };
        Self::sync_parking_in_state(&mut st, &snap);
        st.ops.push(SandboxOp::RestoreParking);
        Ok(())
    }

    fn apply_parking_snapshot(&self, snap: &parking::ParkingSnapshot) -> Result<()> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        Self::sync_parking_in_state(&mut st, snap);
        Ok(())
    }

    fn capture_latency(&self) -> LatencySnapshot {
        self.inner.read().unwrap().latency_snapshot.clone()
    }

    fn run_latency_benchmark(&self, stage: &str) -> LatencyBenchmarkSnapshot {
        let st = self.inner.read().unwrap();
        if stage.eq_ignore_ascii_case("AFTER") {
            st.latency_benchmark_after.clone()
        } else {
            st.latency_benchmark_before.clone()
        }
    }

    fn create_metrics_sampler(&self, _max_history: usize) -> Box<dyn MetricsSampler> {
        Box::new(SandboxMetricsSampler {
            inner: Arc::clone(&self.inner),
            tick: 0.0,
        })
    }

    fn scan_clean_targets(&self) -> Vec<CleanTargetScan> {
        self.inner.read().unwrap().clean_targets.clone()
    }

    fn execute_clean_targets(&self, kinds: &[CleanTargetKind]) -> Result<CleanExecutionReport> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let mut report = CleanExecutionReport::default();
        for &kind in kinds {
            if let Some(target) = st.clean_targets.iter_mut().find(|t| t.kind == kind) {
                let freed = target.reclaimable_bytes;
                let files = target.file_count;
                target.reclaimable_bytes = 0;
                target.file_count = 0;
                report.total_freed_bytes += freed;
                report.total_deleted_files += files;
                report.results.push(CleanTargetResult {
                    kind,
                    freed_bytes: freed,
                    deleted_files: files,
                    skipped_locked: 0,
                    error: None,
                });
            }
        }
        st.ops.push(SandboxOp::CleanExecuted {
            targets: report.results.len(),
            freed_bytes: report.total_freed_bytes,
        });
        Ok(report)
    }

    fn list_installed_appx_detailed(&self) -> Vec<InstalledAppxPackage> {
        let cache = self.load_appx_cache();
        let empty_prov = HashSet::new();
        features_apps::list_installed_appx_from_cache_with_provisioned(&cache, Some(&empty_prov))
    }

    fn remove_appx_batch(&self, families: &[String], _remove_provisioned: bool) -> Result<usize> {
        self.take_injected_error()?;
        if families.is_empty() {
            return Ok(0);
        }
        let mut validated = Vec::with_capacity(families.len());
        for f in families {
            let clean = features_apps::validate_appx_family(f)?;
            validated.push(clean.to_string());
        }
        let mut st = self.inner.write().unwrap();
        let mut removed = 0usize;
        for f in &validated {
            removed += Self::remove_appx_family_in_state(&mut st, &f.to_lowercase(), f);
        }
        if removed == 0 {
            return Err(WinBlastError::AppX(
                "No matching installed or provisioned AppX packages were removed.".to_string(),
            ));
        }
        Ok(removed)
    }

    fn list_optional_features(&self) -> Result<Vec<WindowsOptionalFeature>> {
        let st = self.inner.read().unwrap();
        if !st.elevated {
            return Err(WinBlastError::ElevationRequired(
                "Optional feature scan requires Administrator privileges".to_string(),
            ));
        }
        Ok(st.optional_features.clone())
    }

    fn set_optional_feature(&self, name: &str, kind: OptionalFeatureKind, enable: bool) -> Result<FeatureInstallState> {
        self.take_injected_error()?;
        let clean = features_apps::validate_feature_name(name)?;
        let mut st = self.inner.write().unwrap();
        let new_state = if enable {
            FeatureInstallState::Enabled
        } else {
            FeatureInstallState::Disabled
        };
        if let Some(f) = st
            .optional_features
            .iter_mut()
            .find(|f| optional_feature_matches(&f.name, clean))
        {
            f.state = new_state;
        } else {
            st.optional_features.push(WindowsOptionalFeature {
                name: clean.to_string(),
                display_name: clean.to_string(),
                description: "Sandbox feature".to_string(),
                kind,
                state: new_state,
                is_curated_bloat: false,
            });
        }
        st.ops.push(SandboxOp::FeatureSet {
            name: clean.to_string(),
            kind,
            enable,
        });
        Ok(new_state)
    }

    fn check_onedrive_kfm_active(&self) -> Option<String> {
        self.inner.read().unwrap().onedrive_kfm_path.clone()
    }

    fn uninstall_onedrive_safe(&self) -> Result<String> {
        self.take_injected_error()?;
        if let Some(kfm) = self.inner.read().unwrap().onedrive_kfm_path.clone() {
            return Err(WinBlastError::TweakFailed(format!(
                "Blocked OneDrive uninstall: Known Folder Move active ({})",
                kfm
            )));
        }
        Ok("OneDrive uninstalled safely in sandbox.".to_string())
    }

    fn remove_edge_preserve_webview2(&self) -> Result<String> {
        self.take_injected_error()?;
        Ok("Microsoft Edge uninstalled in sandbox (WebView2 preserved).".to_string())
    }

    fn winget_install_package(&self, package_id: &str) -> Result<String> {
        self.take_injected_error()?;
        let clean = features_apps::validate_winget_package_id(package_id)?;
        Ok(format!("Installed {} (Sandbox)", clean))
    }

    fn list_startup_entries(&self) -> Result<Vec<StartupEntry>> {
        Ok(self.inner.read().unwrap().startup_entries.clone())
    }

    fn set_startup_entry_enabled(&self, entry: &StartupEntry, enable: bool) -> Result<()> {
        self.take_injected_error()?;
        if entry.policy_locked {
            return Err(WinBlastError::TweakFailed(format!(
                "Startup entry '{}' is policy locked.",
                entry.name
            )));
        }
        let mut st = self.inner.write().unwrap();
        if let Some(s) = st.startup_entries.iter_mut().find(|e| e.id == entry.id) {
            if s.policy_locked {
                return Err(WinBlastError::TweakFailed("Policy locked".to_string()));
            }
            s.enabled = enable;
        }
        st.ops.push(SandboxOp::StartupToggled {
            id: entry.id.clone(),
            enable,
        });
        Ok(())
    }

    fn set_startup_entry_enabled_by_id(&self, id: &str, enable: bool) -> Result<()> {
        self.take_injected_error()?;
        let mut st = self.inner.write().unwrap();
        let entry = st
            .startup_entries
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or_else(|| WinBlastError::TweakFailed(format!("Startup entry '{}' not found", id)))?;
        if entry.policy_locked {
            return Err(WinBlastError::TweakFailed("Policy locked".to_string()));
        }
        entry.enabled = enable;
        st.ops.push(SandboxOp::StartupToggled {
            id: id.to_string(),
            enable,
        });
        Ok(())
    }

    fn list_live_services(&self) -> Result<Vec<LiveServiceInfo>> {
        Ok(self.inner.read().unwrap().live_services.clone())
    }

    fn list_live_tasks(&self) -> Vec<LiveScheduledTaskInfo> {
        let cache = self.load_task_cache();
        startup::list_live_tasks_from_cache(&cache)
    }

    fn run_scheduled_task(&self, task_path: &str) -> Result<()> {
        self.take_injected_error()?;
        let norm = normalize_task_path(task_path);
        let st = self.inner.read().unwrap();
        let space_variant = format!("{} ", norm);
        let exists = st.tasks.contains_key(&norm)
            || st.tasks.keys().any(|k| k.starts_with(&space_variant));
        if !exists {
            return Err(WinBlastError::ScheduledTask(format!(
                "Scheduled task '{}' does not exist in sandbox",
                task_path
            )));
        }
        Ok(())
    }

    fn restart_explorer_safely(&self) -> Result<String> {
        self.take_injected_error()?;
        self.inner.write().unwrap().ops.push(SandboxOp::ExplorerRestarted);
        Ok("Explorer restarted safely in sandbox.".to_string())
    }

    fn broadcast_shell_settings_changed(&self) {}

    fn request_system_reboot(&self) -> Result<()> {
        self.take_injected_error()?;
        self.inner.write().unwrap().ops.push(SandboxOp::RebootRequested);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_core_parking_labels_metrics_and_power_sync() {
        let sbx = SandboxEnv::new();
        let cp = sbx.detect_core_parking();
        assert_eq!(cp.cores[0].core_type_label, "P-Core");
        assert_eq!(cp.cores[4].core_type_label, "E-Core");
        assert!(cp.on_ac_power);
        assert_eq!(cp.max_cores_dc_pct, 100);

        let mut sampler = sbx.create_metrics_sampler(8);
        let m0 = sampler.sample();
        assert_eq!(m0.core_parked.len(), cp.cores.len());
        assert_eq!(
            m0.core_parked,
            cp.cores.iter().map(|c| c.is_parked).collect::<Vec<_>>()
        );

        sbx.unpark_all_cores().unwrap();
        let cp_unparked = sbx.detect_core_parking();
        assert!(cp_unparked.is_fully_unparked);
        assert_eq!(
            sbx.read_power_setting_index(None, parking::SUB_PROCESSOR, parking::CPMINCORES)
                .unwrap(),
            (100, 100)
        );
        assert_eq!(
            sbx.read_power_setting_index(None, parking::SUB_PROCESSOR, parking::CPMAXCORES)
                .unwrap(),
            (100, 100)
        );

        let m1 = sampler.sample();
        assert!(m1.core_parked.iter().all(|&p| !p));

        sbx.restore_stock_parking().unwrap();
        let cp_stock = sbx.detect_core_parking();
        assert!(!cp_stock.is_fully_unparked);
        assert_eq!(
            sbx.read_power_setting_index(None, parking::SUB_PROCESSOR, parking::CPMINCORES)
                .unwrap(),
            (10, 5)
        );
        assert_eq!(
            sbx.read_power_setting_index(None, parking::SUB_PROCESSOR, parking::CPMAXCORES)
                .unwrap(),
            (100, 100)
        );
    }
}
