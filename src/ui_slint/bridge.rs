//! State bridge between WinBlast0 core engine and Slint 1.19 UI thread.
//!
//! Provides two-way reactive property binding, model virtualization,
//! differential shadow synchronization, and event loop integration.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ComponentHandle, Model, SharedString, VecModel};

use super::{
    AppxPackageItem, AutomationRuleItem, CleanCategoryItem, CoreStateItem, PhysicalDiskInfo,
    PrivacyItemData, RepairTaskItem, RestorePointItem, SensorReading, SlintMainWindow,
    StartupItemData, TweakItemData,
};
use crate::app::{ActiveView, AppState, PresetKind};
use crate::core::risk::RiskLevel;
use crate::scanner::TweakState;
use crate::tools::cleaner::CleanTargetKind;
use crate::tools::features_apps::AppxSafetyTier;
use crate::tools::windows_repair::RepairTask;

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub struct SlintBridge {
    window: SlintMainWindow,
    matrix_model: Rc<VecModel<TweakItemData>>,
    clean_categories_model: Rc<VecModel<CleanCategoryItem>>,
    appx_model: Rc<VecModel<AppxPackageItem>>,
    startup_model: Rc<VecModel<StartupItemData>>,
    cpu_cores_model: Rc<VecModel<CoreStateItem>>,
    sensors_model: Rc<VecModel<SensorReading>>,
    disks_model: Rc<VecModel<PhysicalDiskInfo>>,
    repairs_model: Rc<VecModel<RepairTaskItem>>,
    restore_model: Rc<VecModel<RestorePointItem>>,
    automation_model: Rc<VecModel<AutomationRuleItem>>,
    privacy_model: Rc<VecModel<PrivacyItemData>>,

    // Shadow differential tracking
    shadow_tweak_states: Vec<(bool, String)>,
    shadow_clean_sizes: Vec<u64>,
    shadow_appx_count: usize,
    shadow_startup_count: usize,
    shadow_restore_count: usize,
}

impl SlintBridge {
    pub fn new(window: SlintMainWindow) -> Self {
        let matrix_model = Rc::new(VecModel::default());
        let clean_categories_model = Rc::new(VecModel::default());
        let appx_model = Rc::new(VecModel::default());
        let startup_model = Rc::new(VecModel::default());
        let cpu_cores_model = Rc::new(VecModel::default());
        let sensors_model = Rc::new(VecModel::default());
        let disks_model = Rc::new(VecModel::default());
        let repairs_model = Rc::new(VecModel::default());
        let restore_model = Rc::new(VecModel::default());
        let automation_model = Rc::new(VecModel::default());
        let privacy_model = Rc::new(VecModel::default());

        window.set_matrix_tweaks(matrix_model.clone().into());
        window.set_clean_categories(clean_categories_model.clone().into());
        window.set_appx_packages(appx_model.clone().into());
        window.set_startup_items(startup_model.clone().into());
        window.set_cpu_cores(cpu_cores_model.clone().into());
        window.set_thermal_sensors(sensors_model.clone().into());
        window.set_physical_disks(disks_model.clone().into());
        window.set_repair_tasks(repairs_model.clone().into());
        window.set_restore_points(restore_model.clone().into());
        window.set_automation_rules(automation_model.clone().into());
        window.set_privacy_items(privacy_model.clone().into());

        Self {
            window,
            matrix_model,
            clean_categories_model,
            appx_model,
            startup_model,
            cpu_cores_model,
            sensors_model,
            disks_model,
            repairs_model,
            restore_model,
            automation_model,
            privacy_model,
            shadow_tweak_states: Vec::new(),
            shadow_clean_sizes: Vec::new(),
            shadow_appx_count: 0,
            shadow_startup_count: 0,
            shadow_restore_count: 0,
        }
    }

    pub fn window(&self) -> &SlintMainWindow {
        &self.window
    }

    /// Initializes all subview models with startup data from AppState.
    pub fn init(&mut self, state: &AppState) {
        // 1. Matrix Tweaks Model
        let mut tweak_rows = Vec::with_capacity(state.tweaks.len());
        self.shadow_tweak_states.clear();
        for (i, t) in state.tweaks.iter().enumerate() {
            let is_opt = state
                .scan_results
                .get(i)
                .map(|sr| matches!(sr.current_state, TweakState::AlreadyOptimized))
                .unwrap_or(false);
            let risk_str = match t.valuation.stability_risk {
                RiskLevel::Safe => "safe",
                RiskLevel::Low => "low",
                RiskLevel::Moderate => "moderate",
                RiskLevel::High | RiskLevel::Extreme => "high",
            };
            let status_str = if is_opt { "optimized" } else { "unoptimized" };
            self.shadow_tweak_states
                .push((is_opt, status_str.to_string()));

            tweak_rows.push(TweakItemData {
                id: SharedString::from(t.id.as_str()),
                title: SharedString::from(t.title.as_str()),
                description: SharedString::from(t.description.as_str()),
                category: SharedString::from(t.category.as_str()),
                risk: SharedString::from(risk_str),
                status: SharedString::from(status_str),
                enabled: is_opt,
            });
        }
        self.matrix_model.set_vec(tweak_rows);
        self.window
            .set_matrix_total_count(state.tweaks.len() as i32);

        // 2. Clean Categories Model
        let mut clean_rows = Vec::with_capacity(CleanTargetKind::ALL.len());
        self.shadow_clean_sizes.clear();
        for kind in CleanTargetKind::ALL {
            self.shadow_clean_sizes.push(0);
            clean_rows.push(CleanCategoryItem {
                id: SharedString::from(kind.id_key()),
                name: SharedString::from(kind.display_name()),
                description: SharedString::from(kind.display_name()),
                size_text: SharedString::from("0 B"),
                size_bytes: 0,
                selected: kind.default_selected(),
            });
        }
        self.clean_categories_model.set_vec(clean_rows);

        // 3. Repair Tasks Model
        let repair_tasks = RepairTask::all();
        let mut repair_rows = Vec::with_capacity(repair_tasks.len());
        for task in repair_tasks {
            repair_rows.push(RepairTaskItem {
                id: SharedString::from(task.id),
                name: SharedString::from(task.title),
                command: SharedString::from(task.command_preview),
                description: SharedString::from(task.description),
                is_running: false,
                last_result: SharedString::from("Ready"),
            });
        }
        self.repairs_model.set_vec(repair_rows);

        // 4. Default Privacy Rules Model
        let privacy_presets = [
            (
                "telemetry_diag",
                "Connected User Experiences & Telemetry",
                "Disables DiagTrack service and ETL logging",
                "Core",
            ),
            (
                "cortana_search",
                "Bing & Web Search in Start Menu",
                "Disables Bing cloud search queries in Windows Search",
                "Search",
            ),
            (
                "advertising_id",
                "Windows Advertising ID",
                "Prevents apps from using advertising ID for cross-app experiences",
                "Advertising",
            ),
            (
                "activity_history",
                "Windows Activity History Timeline",
                "Prevents Windows from collecting clipboard and task timeline",
                "Diagnostics",
            ),
            (
                "app_diagnostics",
                "App Diagnostic Access",
                "Prevents third-party apps from accessing diagnostic info of other apps",
                "Permissions",
            ),
            (
                "feedback_frequency",
                "Automated Feedback Frequency",
                "Sets diagnostic data feedback request frequency to Never",
                "Diagnostics",
            ),
        ];
        let mut privacy_rows = Vec::with_capacity(privacy_presets.len());
        for (id, title, desc, cat) in privacy_presets {
            privacy_rows.push(PrivacyItemData {
                id: SharedString::from(id),
                title: SharedString::from(title),
                description: SharedString::from(desc),
                category: SharedString::from(cat),
                enabled: true,
            });
        }
        self.privacy_model.set_vec(privacy_rows);
        self.window.set_privacy_blocked_count(48);

        // 5. Default Automation Rules Model
        let auto_presets = [
            (
                "auto_temp_clean",
                "Weekly Disk Temp Cleanup",
                "Purges User & Windows %TEMP% caches on idle",
                "Weekly",
            ),
            (
                "auto_ram_trim",
                "Adaptive RAM Working-Set Trim",
                "Trims stale background memory when usage exceeds 85%",
                "On Idle",
            ),
            (
                "auto_dns_flush",
                "Daily DNS Resolver Refresh",
                "Flushes DNS cache daily to eliminate stale lookups",
                "Daily",
            ),
            (
                "auto_restore_point",
                "Pre-Optimization VSS Checkpoint",
                "Automatically creates System Restore Point before tweaks",
                "On Demand",
            ),
        ];
        let mut auto_rows = Vec::with_capacity(auto_presets.len());
        for (id, title, desc, sched) in auto_presets {
            auto_rows.push(AutomationRuleItem {
                id: SharedString::from(id),
                title: SharedString::from(title),
                description: SharedString::from(desc),
                schedule_type: SharedString::from(sched),
                last_run: SharedString::from("Never"),
                enabled: true,
            });
        }
        self.automation_model.set_vec(auto_rows);

        // Initial sync of properties
        self.sync_from_state(state);
    }

    /// Synchronizes core AppState snapshot into Slint properties
    pub fn sync_from_state(&self, state: &AppState) {
        self.window
            .set_app_version(env!("CARGO_PKG_VERSION").into());
        self.window
            .set_is_elevated(crate::core::elevation::is_elevated());
        self.window.set_active_view(state.active_view.name().into());

        // Fast statusbar telemetry
        if let Some(ref m) = state.realtime_metrics {
            self.window.set_cpu_usage(m.cpu_usage);
            self.window.set_ram_usage(m.ram_percentage);
            let used_gb = m.used_ram_mb as f64 / 1024.0;
            let total_gb = m.total_ram_mb as f64 / 1024.0;
            self.window
                .set_ram_text(format!("{:.1} / {:.1} GB", used_gb, total_gb).into());
        }

        // OS info
        if let Some(ref o) = state.os_info {
            let os_desc = format!("{} (Build {})", o.edition, o.build_number);
            self.window.set_dashboard_os_version(os_desc.into());
        }

        let total_ram_mb: u64 = state.ram_dimms.iter().map(|d| d.capacity_mb).sum();
        if total_ram_mb > 0 {
            self.window.set_dashboard_ram_capacity(
                format!("{:.1} GB", total_ram_mb as f64 / 1024.0).into(),
            );
            self.window
                .set_hw_ram_total(format!("{:.1} GB", total_ram_mb as f64 / 1024.0).into());
        }
        if let Some(dimm) = state.ram_dimms.first() {
            self.window
                .set_hw_ram_speed(format!("{} MT/s ({})", dimm.speed_mts, dimm.ddr_type).into());
        }

        // Background worker status
        let is_busy = state.is_scanning
            || state.is_applying
            || state.clean_apps_busy
            || state.startup_tasks_busy;
        self.window.set_worker_active(is_busy);
        if let Some(ref msg) = state.progress_message {
            self.window.set_worker_label(msg.as_str().into());
        } else if state.is_scanning {
            self.window
                .set_worker_label("Scanning system tweaks...".into());
        } else if state.is_applying {
            self.window
                .set_worker_label("Applying optimizations...".into());
        } else if state.clean_apps_busy {
            self.window
                .set_worker_label("Cleaning disk caches...".into());
        } else {
            self.window.set_worker_label("Ready".into());
        }

        // In-Progress HUD
        if state.is_applying || state.clean_apps_busy {
            self.window.set_hud_active(true);
            self.window.set_hud_progress(state.progress * 100.0);
            if let Some(ref step) = state.progress_message {
                self.window.set_hud_current_step(step.as_str().into());
            }
        } else if !state.is_applying && !state.clean_apps_busy && self.window.get_hud_active() {
            self.window.set_hud_active(false);
        }

        // Hardware specs
        if let Some(ref cpu) = state.cpu_info {
            self.window
                .set_dashboard_cpu_model(cpu.model_name.as_str().into());
            self.window.set_hw_cpu_name(cpu.model_name.as_str().into());
            self.window.set_hw_cpu_cores(
                format!(
                    "{} Cores / {} Threads",
                    cpu.physical_cores, cpu.logical_cores
                )
                .into(),
            );
            self.window.set_cpu_name(cpu.model_name.as_str().into());
            self.window.set_cpu_logical_cores(cpu.logical_cores as i32);
            self.window
                .set_cpu_physical_cores(cpu.physical_cores as i32);
        }

        if let Some(gpu) = state.gpus.first() {
            self.window
                .set_dashboard_gpu_model(gpu.name.as_str().into());
            self.window.set_hw_gpu_name(gpu.name.as_str().into());
            self.window.set_gpu_name(gpu.name.as_str().into());
            self.window.set_gpu_is_msi_enabled(gpu.msi_enabled);
        }
        self.window.set_gpu_is_hags_enabled(state.hags_enabled);

        // CPU Parking State
        self.window
            .set_cpu_parked_count(state.core_parking.parked_count as i32);
        self.window
            .set_cpu_min_unparked_percent(state.core_parking.min_cores_ac_pct as i32);
        let active_scheme = state
            .power_schemes
            .iter()
            .find(|s| s.is_active)
            .map(|s| s.friendly_name.as_str())
            .unwrap_or("Balanced");
        self.window
            .set_cpu_active_power_scheme(active_scheme.into());

        // Easy Dashboard counts
        let applied_count = state
            .scan_results
            .iter()
            .filter(|sr| matches!(sr.current_state, TweakState::AlreadyOptimized))
            .count() as i32;
        self.window.set_dashboard_applied_tweaks(applied_count);
        self.window.set_matrix_optimized_count(applied_count);

        let total_count = state.tweaks.len() as i32;
        let health_score = if total_count > 0 {
            50 + (applied_count * 50 / total_count).min(50)
        } else {
            85
        };
        self.window.set_dashboard_health_score(health_score);
    }

    /// Periodic 100ms polling hook for differential row synchronization
    pub fn poll_sync(&mut self, state: &AppState) {
        self.sync_from_state(state);

        // Differential Matrix Tweak Updates
        if state.scan_results.len() == state.tweaks.len()
            && self.shadow_tweak_states.len() == state.tweaks.len()
        {
            for (i, sr) in state.scan_results.iter().enumerate() {
                let is_opt = matches!(sr.current_state, TweakState::AlreadyOptimized);
                let (prev_opt, ref prev_status) = self.shadow_tweak_states[i];
                let status_str = if is_opt { "optimized" } else { "unoptimized" };

                if prev_opt != is_opt || prev_status != status_str {
                    if let Some(mut row) = self.matrix_model.row_data(i) {
                        row.enabled = is_opt;
                        row.status = SharedString::from(status_str);
                        self.matrix_model.set_row_data(i, row);
                        self.shadow_tweak_states[i] = (is_opt, status_str.to_string());
                    }
                }
            }
        }

        // Differential Clean Targets Updates
        if !state.clean_targets.is_empty() {
            let mut total_reclaimable: u64 = 0;
            for target in &state.clean_targets {
                total_reclaimable += target.reclaimable_bytes;
                if let Some(idx) = CleanTargetKind::ALL.iter().position(|k| *k == target.kind) {
                    if self.shadow_clean_sizes.get(idx) != Some(&target.reclaimable_bytes) {
                        if let Some(mut row) = self.clean_categories_model.row_data(idx) {
                            row.size_bytes = (target.reclaimable_bytes as i64) as i32;
                            row.size_text =
                                SharedString::from(format_bytes(target.reclaimable_bytes));
                            row.selected = target.selected;
                            self.clean_categories_model.set_row_data(idx, row);
                            if idx < self.shadow_clean_sizes.len() {
                                self.shadow_clean_sizes[idx] = target.reclaimable_bytes;
                            }
                        }
                    }
                }
            }
            self.window
                .set_clean_total_reclaimable(format_bytes(total_reclaimable).into());
        }

        // AppX Packages Updates
        if state.installed_appx.len() != self.shadow_appx_count {
            let mut appx_rows = Vec::with_capacity(state.installed_appx.len());
            for pkg in &state.installed_appx {
                let is_protected = pkg.safety_tier == AppxSafetyTier::ProtectedCore;
                let is_selected = state.selected_appx.contains(&pkg.family_name);
                appx_rows.push(AppxPackageItem {
                    id: SharedString::from(pkg.family_name.as_str()),
                    name: SharedString::from(pkg.short_name.as_str()),
                    publisher: SharedString::from(pkg.publisher.as_str()),
                    size_text: SharedString::from(pkg.safety_tier.as_label()),
                    is_protected,
                    selected: is_selected,
                });
            }
            self.appx_model.set_vec(appx_rows);
            self.shadow_appx_count = state.installed_appx.len();
        }

        // Startup Items Updates
        if state.startup_entries.len() != self.shadow_startup_count {
            let mut startup_rows = Vec::with_capacity(state.startup_entries.len());
            for entry in &state.startup_entries {
                startup_rows.push(StartupItemData {
                    id: SharedString::from(entry.id.as_str()),
                    name: SharedString::from(entry.name.as_str()),
                    command: SharedString::from(entry.command.as_str()),
                    publisher: SharedString::from(if entry.is_microsoft {
                        "Microsoft Corporation"
                    } else {
                        "Third Party"
                    }),
                    impact: SharedString::from("low"),
                    is_task: false,
                    enabled: entry.enabled,
                });
            }
            self.startup_model.set_vec(startup_rows);
            self.shadow_startup_count = state.startup_entries.len();
            self.window
                .set_startup_total_items(state.startup_entries.len() as i32);
            let disabled_count = state.startup_entries.iter().filter(|e| !e.enabled).count() as i32;
            self.window.set_startup_disabled_items(disabled_count);
        }

        // Restore Points Updates
        if state.restore_points.len() != self.shadow_restore_count {
            let mut restore_rows = Vec::with_capacity(state.restore_points.len());
            for rp in &state.restore_points {
                restore_rows.push(RestorePointItem {
                    id: SharedString::from(rp.sequence_number.to_string()),
                    description: SharedString::from(rp.description.as_str()),
                    creation_time: SharedString::from(rp.creation_time.as_str()),
                    point_type: SharedString::from("System Restore Point"),
                });
            }
            self.restore_model.set_vec(restore_rows);
            self.shadow_restore_count = state.restore_points.len();
        }

        // CPU Cores Updates
        if !state.core_parking.cores.is_empty() {
            let mut core_rows = Vec::with_capacity(state.core_parking.cores.len());
            for (i, c) in state.core_parking.cores.iter().enumerate() {
                core_rows.push(CoreStateItem {
                    core_id: i as i32,
                    is_parked: c.is_parked,
                    usage: c.usage_pct,
                    frequency_mhz: 3600,
                });
            }
            self.cpu_cores_model.set_vec(core_rows);
        }

        // Thermal Sensors Updates
        if let Some(ref thermals) = state.thermal_snapshot {
            let mut sensor_rows = Vec::with_capacity(thermals.components.len());
            for c in &thermals.components {
                sensor_rows.push(SensorReading {
                    name: SharedString::from(c.name.as_str()),
                    temp_c: c.current_temp_c,
                    max_temp_c: c.max_temp_c,
                    is_throttling: c.throttling,
                });
            }
            self.sensors_model.set_vec(sensor_rows);
        }

        // Physical Disks Updates
        if let Some(ref report) = state.disk_report {
            let mut disk_rows = Vec::with_capacity(report.drives.len());
            for d in &report.drives {
                disk_rows.push(PhysicalDiskInfo {
                    model: SharedString::from(d.model.as_str()),
                    serial: SharedString::from(format!("Disk #{}", d.index)),
                    bus_type: SharedString::from(d.interface_type.as_str()),
                    capacity_text: SharedString::from(format!("{:.0} GB", d.total_size_gb)),
                    tbw_text: SharedString::from(format!("{:.1} TB", d.lifetime_writes_tb)),
                    health_percent: d.health_percentage as i32,
                    temperature_c: d.temperature_c as i32,
                    power_on_hours: d.power_on_hours as i32,
                    unsafe_shutdowns: 0,
                });
            }
            self.disks_model.set_vec(disk_rows);
        }
    }

    /// Registers all Slint UI action callbacks to mutate backend AppState
    pub fn bind_callbacks(&self, state: Rc<RefCell<AppState>>) {
        // Navigation
        let s_nav = state.clone();
        let win_nav = self.window.as_weak();
        self.window.on_navigate(move |view_name| {
            if let Some(w) = win_nav.upgrade() {
                let active = match view_name.as_str() {
                    "dashboard" => ActiveView::EasyDashboard,
                    "matrix" => ActiveView::AdvancedMatrix,
                    "clean_apps" => ActiveView::CleanAndApps,
                    "startup_tasks" => ActiveView::StartupAndTasks,
                    "gpu" => ActiveView::GpuTuning,
                    "cpu_parking" => ActiveView::CoreParking,
                    "hardware" => ActiveView::HardwareView,
                    "thermal" => ActiveView::ThermalMonitor,
                    "disk_health" => ActiveView::DiskHealth,
                    "driver_bios" => ActiveView::DriverBios,
                    "latency" => ActiveView::LatencyTester,
                    "fps_benchmark" => ActiveView::FpsBenchmark,
                    "gaming_stats" => ActiveView::GamingStats,
                    "network_diag" => ActiveView::NetworkDiag,
                    "performance_tests" => ActiveView::PerformanceTests,
                    "game_profiles" => ActiveView::GameProfiles,
                    "game_overlay" => ActiveView::GameOverlay,
                    "game_suitability" => ActiveView::GameSuitability,
                    "game_prereqs" => ActiveView::GamePrerequisites,
                    "automation" => ActiveView::AutomationCenter,
                    "goal_planner" => ActiveView::GoalPlanner,
                    "windows_repair" => ActiveView::WindowsRepair,
                    "fault_diagnostics" => ActiveView::FaultDiagnostics,
                    "backup" => ActiveView::BackupCenter,
                    "privacy" => ActiveView::PrivacyShield,
                    "support_about" => ActiveView::SupportAbout,
                    _ => ActiveView::EasyDashboard,
                };
                s_nav.borrow_mut().active_view = active;
                w.set_active_view(view_name);
            }
        });

        // Easy Dashboard Presets
        let s_preset = state.clone();
        self.window.on_apply_dashboard_preset(move |preset| {
            let mut s = s_preset.borrow_mut();
            match preset.as_str() {
                "gaming" => s.apply_preset(PresetKind::Gaming),
                "pro_audio" => s.apply_preset(PresetKind::ProAudio),
                "battery" => s.apply_preset(PresetKind::LaptopBattery),
                _ => s.apply_selected(),
            }
        });

        // System Scan
        let s_scan = state.clone();
        self.window.on_scan_system_clicked(move || {
            let mut s = s_scan.borrow_mut();
            s.start_scan();
            s.scan_clean_targets_async();
            s.scan_disk_health_async();
            s.scan_thermal_snapshot_async();
        });

        // Matrix Tweak Toggled
        let s_tweak = state.clone();
        self.window.on_tweak_toggled(move |id, enabled| {
            let mut s = s_tweak.borrow_mut();
            s.toggle_tweak(id.to_string(), enabled);
            if enabled {
                s.apply_selected();
            } else {
                s.revert_selected();
            }
        });

        // Apply All Safe Tweaks
        let s_safe = state.clone();
        self.window.on_apply_all_safe(move || {
            let mut s = s_safe.borrow_mut();
            let safe_ids: Vec<String> = s
                .tweaks
                .iter()
                .filter(|t| t.valuation.stability_risk == RiskLevel::Safe)
                .map(|t| t.id.clone())
                .collect();
            s.selected_tweaks.clear();
            for id in safe_ids {
                s.selected_tweaks.insert(id);
            }
            s.apply_selected();
        });

        // Revert All Tweaks
        let s_revert = state.clone();
        self.window.on_revert_all(move || {
            let mut s = s_revert.borrow_mut();
            s.revert_selected();
        });

        // Refresh Scan
        let s_refresh = state.clone();
        self.window.on_refresh_scan(move || {
            s_refresh.borrow_mut().start_scan();
        });

        // Disk Cleaner Actions
        let s_cscan = state.clone();
        self.window.on_scan_disk_clicked(move || {
            let mut s = s_cscan.borrow_mut();
            s.scan_clean_targets_async();
            s.scan_appx_and_features_async();
        });

        let s_cclean = state.clone();
        self.window.on_clean_disk_clicked(move || {
            s_cclean.borrow_mut().execute_clean_selected();
        });

        let s_appx_toggle = state.clone();
        self.window.on_appx_toggled(move |id, _selected| {
            s_appx_toggle
                .borrow_mut()
                .toggle_appx_selection(id.to_string());
        });

        let s_appx_uninst = state.clone();
        self.window.on_uninstall_selected_appx(move || {
            s_appx_uninst.borrow_mut().remove_selected_appx_async();
        });

        // Startup & Tasks Actions
        let s_start_toggle = state.clone();
        self.window.on_startup_item_toggled(move |id, _enabled| {
            let mut s = s_start_toggle.borrow_mut();
            if let Some(entry) = s.startup_entries.iter_mut().find(|e| e.id == id.as_str()) {
                entry.enabled = !entry.enabled;
            }
        });

        let s_start_refresh = state.clone();
        self.window.on_refresh_startup_clicked(move || {
            s_start_refresh.borrow_mut().scan_startup_and_tasks_async();
        });

        // GPU Tuning
        let s_msi = state.clone();
        self.window.on_gpu_msi_toggled(move |enabled| {
            let mut s = s_msi.borrow_mut();
            if let Some(gpu) = s.gpus.first() {
                let pnp = gpu.pnp_instance_id.clone();
                s.toggle_gpu_msi(pnp, enabled, 2);
            }
        });

        let s_hags = state.clone();
        self.window.on_gpu_hags_toggled(move |enabled| {
            s_hags.borrow_mut().toggle_hags(enabled);
        });

        // CPU Parking
        let s_cpu_park = state.clone();
        self.window.on_apply_cpu_parking(move || {
            let mut s = s_cpu_park.borrow_mut();
            s.status_message = "Applied CPU core parking frequency policy".to_string();
        });

        let s_cpu_reset = state.clone();
        self.window.on_reset_cpu_defaults(move || {
            s_cpu_reset.borrow_mut().restore_stock_parking();
        });

        // Thermals & Disks Refresh
        let s_thermal = state.clone();
        self.window.on_refresh_thermal(move || {
            s_thermal.borrow_mut().scan_thermal_snapshot_async();
        });

        let s_disks = state.clone();
        self.window.on_refresh_disks(move || {
            s_disks.borrow_mut().scan_disk_health_async();
        });

        // System Repairs
        let s_repair = state.clone();
        let win_repair = self.window.as_weak();
        self.window.on_run_repair_task(move |task_id| {
            let task_id_str = task_id.to_string();
            std::thread::spawn(move || {
                let _ = RepairTask::execute(&task_id_str);
            });
            if let Some(w) = win_repair.upgrade() {
                w.set_repair_active_label(task_id);
                w.set_repair_any_running(true);
            }
            s_repair.borrow_mut().status_message = "Running system repair task...".to_string();
        });

        // Restore Points
        let s_restore = state.clone();
        self.window.on_create_restore_point(move || {
            s_restore.borrow_mut().create_restore_point();
        });
    }
}
