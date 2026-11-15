//! Headless Rune UI responsive layout, localization, theme, DPI scale, modal/drawer,
//! duplicate-key, and interactive navigation verification suite.

use std::collections::HashSet;
use std::sync::Arc;

use rune::{el, Env, Runtime};

use winblast0::app::{ActiveView, AppState, CleanAppsSubTab, StartupTasksSubTab};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::i18n::{Language, TextKey};
use winblast0::core::risk::load_embedded_rules;
use winblast0::hardware::gpu::{DetectedGpu, GpuVendor};
use winblast0::scanner::telemetry_scanner::full_telemetry_audit_with;
use winblast0::tools::autounattend::TargetArch;
use winblast0::tools::startup::{LiveServiceInfo, StartupEntry, StartupSource};
use winblast0::ui;
use winblast0::ui::theme::{ThemeMode, ThemePalette};

const VIEWPORTS: &[(f32, f32)] = &[
    (420.0, 400.0),
    (640.0, 480.0),
    (800.0, 600.0),
    (1024.0, 768.0),
    (1280.0, 720.0),
    (1366.0, 768.0),
    (1920.0, 1080.0),
    (2560.0, 1440.0),
    (3440.0, 1440.0),
    (3840.0, 2160.0),
];

const ALL_VIEWS: &[ActiveView] = &[
    ActiveView::EasyDashboard,
    ActiveView::AdvancedMatrix,
    ActiveView::GpuTuning,
    ActiveView::HardwareView,
    ActiveView::PrivacyShield,
    ActiveView::BackupCenter,
    ActiveView::CoreParking,
    ActiveView::LatencyTester,
    ActiveView::CleanAndApps,
    ActiveView::StartupAndTasks,
];

const VERBOSE_LOCALES: &[Language] = &[
    Language::Turkish,
    Language::German,
    Language::Russian,
];

const THEMES: &[ThemeMode] = &[ThemeMode::Dark, ThemeMode::Light];
const SCALES: &[f32] = &[1.0, 1.25, 1.5];

/// Build a richly populated `AppState` backed entirely by an isolated `SandboxEnv`.
fn build_populated_sandbox_state() -> AppState {
    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    sbx.seed_all_tweaks(&tweaks);

    {
        let mut st = sbx.inner.write().unwrap();
        // Multi-GPU list (NVIDIA + AMD) while leaving Intel tweaks as incompatible cards
        st.gpus = vec![
            DetectedGpu {
                name: "NVIDIA GeForce RTX 4090 (Sandbox)".to_string(),
                pnp_instance_id:
                    "PCI\\VEN_10DE&DEV_2684&SUBSYS_000010DE&REV_A1\\4&1234567&0&0008".to_string(),
                msi_enabled: true,
                interrupt_priority: 2,
                vendor: GpuVendor::Nvidia,
                class_subkey: "0001".to_string(),
                dedicated_vram_mb: 24576,
            },
            DetectedGpu {
                name: "AMD Radeon RX 7900 XTX (Sandbox)".to_string(),
                pnp_instance_id:
                    "PCI\\VEN_1002&DEV_744C&SUBSYS_00001002&REV_C8\\4&7654321&0&0008".to_string(),
                msi_enabled: false,
                interrupt_priority: 1,
                vendor: GpuVendor::Amd,
                class_subkey: "0000".to_string(),
                dedicated_vram_mb: 24576,
            },
        ];
        st.services.insert("diagtrack".to_string(), 2);
        st.services.insert("wersvc".to_string(), 3);
        st.appx_packages
            .insert("Microsoft.SecHealthUI_8wekyb3d8bbwe".to_string());
        st.appx_packages
            .insert("Microsoft.BingWeather_8wekyb3d8bbwe".to_string());
        st.appx_packages
            .insert("Clipchamp.Clipchamp_yxz26nhyzhsrt".to_string());
        st.tasks
            .insert(r"\Microsoft\Windows\Autochk\Proxy".to_string(), true);
        st.tasks
            .insert(r"\CustomApps\TelemetryAgent".to_string(), true);
        st.startup_entries.push(StartupEntry {
            id: "HKCU|Run|DiscordUpdater".to_string(),
            name: "DiscordUpdater".to_string(),
            command: "\"C:\\Users\\Test\\AppData\\Local\\Discord\\Update.exe\"".to_string(),
            source: StartupSource::RegistryHkcuRun,
            enabled: true,
            location: "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
            hive: "HKCU".to_string(),
            reg_path: "Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
            approved_subkey: Some("Run".to_string()),
            is_wow64: false,
            policy_locked: false,
            is_microsoft: false,
        });
        st.live_services.push(LiveServiceInfo {
            service_name: "SteamClientService".to_string(),
            display_name: "Steam Client Service".to_string(),
            start_type: 3,
            is_running: false,
            binary_path: "C:\\Program Files (x86)\\Common Files\\Steam\\SteamService.exe".to_string(),
            is_protected_core: false,
            is_microsoft: false,
        });
    }

    let bench_before = winblast0::core::backend::SystemBackend::run_latency_benchmark(&sbx, "BEFORE");
    let bench_after = winblast0::core::backend::SystemBackend::run_latency_benchmark(&sbx, "AFTER");
    let restore_points = winblast0::core::backend::SystemBackend::list_restore_points(&sbx).unwrap_or_default();
    let audit = full_telemetry_audit_with(&sbx);

    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.telemetry_audit = Some(audit);
    state.latency_before = Some(bench_before);
    state.latency_after = Some(bench_after.clone());
    state.latest_latency_bench = Some(bench_after);
    state.restore_points = restore_points;

    // Populate ledger entries, scan results, Clean & Apps, Startup & Tasks, Platform Inventory & sparklines
    state.start_scan();
    state.refresh_platform_inventory();
    state.scan_clean_targets_async();
    state.scan_appx_and_features_async();
    state.scan_startup_and_tasks_async();
    state.toggle_hags(false);
    state.toggle_mpo(true);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        state.drain_worker_events();
        if !state.is_scanning
            && !state.scan_results.is_empty()
            && !state.clean_targets.is_empty()
            && !state.installed_appx.is_empty()
            && !state.startup_entries.is_empty()
            && !state.power_schemes.is_empty()
            && state.ledger_history.len() >= 2
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    state.cpu_sparkline = vec![14.0, 22.5, 19.2, 31.4, 27.0, 18.5, 24.0];
    state.ram_sparkline = vec![34.0, 34.2, 34.5, 34.1, 34.3, 34.2, 34.4];
    state.select_all();
    state
}

/// Settle layout reads and springs deterministically for the current viewport/state.
fn settle_frames(rt: &mut Runtime<AppState>) {
    rt.settle_all();
    rt.frame(0.016);
    rt.settle_all();
    rt.frame(0.016);
    rt.settle_all();
    rt.frame(0.016);
}

/// Click an element by key or key-path in window points.
fn click_key(rt: &mut Runtime<AppState>, key: &str) {
    let (mut x, mut y) = rt
        .find(key)
        .unwrap_or_else(|| panic!("Could not find element with key {:?} at viewport {:?}", key, rt.viewport));
    if y >= rt.viewport.1 || y < 0.0 {
        let scroll_delta = y - rt.viewport.1 * 0.5;
        rt.wheel(x.clamp(10.0, rt.viewport.0 - 10.0), rt.viewport.1 * 0.5, -scroll_delta);
        settle_frames(rt);
        if let Some((nx, ny)) = rt.find(key) {
            x = nx;
            y = ny;
        }
    }
    rt.pointer_move(x, y);
    rt.pointer_down(x, y);
    rt.frame(0.016);
    rt.pointer_up(x, y);
    settle_frames(rt);
}

/// Click a visible interactive button/label whose element text matches `exact` (or contains `substr`).
fn click_text(rt: &mut Runtime<AppState>, target: &str, exact: bool) {
    let els = rt.elements();
    let el_hit = els
        .iter()
        .find(|e| {
            !e.exiting
                && e.rect[2] > 0.0
                && e.rect[3] > 0.0
                && e.text.as_deref().is_some_and(|t| {
                    if exact {
                        t == target
                    } else {
                        t.contains(target)
                    }
                })
        })
        .unwrap_or_else(|| {
            panic!(
                "Could not find element with text {:?} (exact={}) at viewport {:?}",
                target, exact, rt.viewport
            )
        });
    let cx = el_hit.rect[0] + el_hit.rect[2] * 0.5;
    let cy = el_hit.rect[1] + el_hit.rect[3] * 0.5;
    assert!(
        cx >= 0.0 && cx <= rt.viewport.0 && cy >= 0.0 && cy <= rt.viewport.1,
        "Target {:?} at ({:.1}, {:.1}) is outside viewport {:?}",
        target,
        cx,
        cy,
        rt.viewport
    );
    rt.pointer_move(cx, cy);
    rt.pointer_down(cx, cy);
    rt.frame(0.016);
    rt.pointer_up(cx, cy);
    settle_frames(rt);
}

fn drain_state_workers(rt: &mut Runtime<AppState>, pred: impl Fn(&AppState) -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        rt.update(|s, _| {
            s.drain_worker_events();
        });
        if pred(&rt.state) {
            settle_frames(rt);
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    panic!(
        "Timed out waiting for AppState worker events (status_message={:?})",
        rt.state.status_message
    );
}

/// Assert zero NaN/negative sizes, zero horizontal overflows, and zero duplicate/repaired keys.
fn assert_frame_clean(
    rt: &Runtime<AppState>,
    view_name: &str,
    context: &str,
    failures: &mut Vec<String>,
    unique_issues: &mut std::collections::BTreeMap<(String, String), String>,
) {
    let vp_w = rt.viewport.0;
    let els = rt.elements();
    assert!(!els.is_empty(), "{}: expected non-empty element tree", context);

    let mut sibling_keys: std::collections::HashMap<Option<usize>, HashSet<String>> =
        std::collections::HashMap::new();

    for e in &els {
        if e.exiting {
            continue;
        }

        // Duplicate-key check: Rune's key_diagnostics::repair always renames duplicate sibling keys to `<key>#dupN`
        if let Some(ref k) = e.key {
            if k.contains("#dup") {
                let msg = format!(
                    "[{}] Duplicate/repaired key {:?} at path {:?}",
                    context, k, e.path
                );
                unique_issues
                    .entry((view_name.to_string(), format!("dup_key:{}", e.path)))
                    .or_insert_with(|| msg.clone());
                failures.push(msg);
            }
            let seen = sibling_keys.entry(e.parent).or_default();
            if !seen.insert(k.clone()) {
                let msg = format!(
                    "[{}] Duplicate sibling key {:?} under parent {:?} at path {:?}",
                    context, k, e.parent, e.path
                );
                unique_issues
                    .entry((view_name.to_string(), format!("dup_sibling:{}", e.path)))
                    .or_insert_with(|| msg.clone());
                failures.push(msg);
            }
        }
        if e.path.contains("#dup") {
            let msg = format!(
                "[{}] Path contains repaired duplicate key segment: {:?}",
                context, e.path
            );
            unique_issues
                .entry((view_name.to_string(), format!("dup_path:{}", e.path)))
                .or_insert_with(|| msg.clone());
            failures.push(msg);
        }

        // Finite non-NaN geometry check on every visible element
        if !e.rect.iter().all(|v| v.is_finite()) {
            let msg = format!(
                "[{}] Non-finite rect {:?} on {:?} (path={:?})",
                context, e.rect, e.key, e.path
            );
            unique_issues
                .entry((view_name.to_string(), format!("nan:{}", e.path)))
                .or_insert_with(|| msg.clone());
            failures.push(msg);
            continue;
        }
        if e.rect[2] < -0.01 || e.rect[3] < -0.01 {
            let msg = format!(
                "[{}] Negative dimensions {:?} on {:?} (path={:?})",
                context, e.rect, e.key, e.path
            );
            unique_issues
                .entry((view_name.to_string(), format!("neg:{}", e.path)))
                .or_insert_with(|| msg.clone());
            failures.push(msg);
            continue;
        }

        if e.rect[2] > 0.0 && e.rect[3] > 0.0 {
            let left = e.rect[0];
            let right = e.rect[0] + e.rect[2];
            if left < -1.0 || right > vp_w + 1.5 {
                let msg = format!(
                    "[{}] Horizontal overflow: rect={:?} (left={:.2}, right={:.2}, vp_w={:.1}) role={} key={:?} text={:?} path={:?}",
                    context, e.rect, left, right, vp_w, e.role, e.key, e.text, e.path
                );
                unique_issues
                    .entry((view_name.to_string(), e.path.clone()))
                    .or_insert_with(|| msg.clone());
                failures.push(msg);
            }
        }
    }

    // Virtual row bounds containment check: rendered rows and descendants inside *_virtual_rows
    // must not exceed their configured virtual row height bounds.
    let is_compact = vp_w < 680.0;
    for (vr_idx, vr) in els.iter().enumerate() {
        if vr.exiting {
            continue;
        }
        let Some(ref vr_key) = vr.key else {
            continue;
        };
        let max_row_h = match vr_key.as_str() {
            "services_virtual_rows" => if is_compact { 140.0 } else { 76.0 },
            "startup_virtual_rows" | "tasks_virtual_rows" => if is_compact { 116.0 } else { 72.0 },
            "appx_virtual_rows" => if is_compact { 104.0 } else { 72.0 },
            _ => continue,
        };

        let row_indices: Vec<usize> = els
            .iter()
            .enumerate()
            .filter(|(_, e)| !e.exiting && e.parent == Some(vr_idx))
            .map(|(idx, _)| idx)
            .collect();

        for r_idx in &row_indices {
            let row_el = &els[*r_idx];
            if row_el.rect[2] <= 0.0 || row_el.rect[3] <= 0.0 {
                continue;
            }
            if row_el.rect[3] > max_row_h + 1.5 {
                let msg = format!(
                    "[{}] Virtual row {:?} height {:.2} exceeds row_h {:.1} (rect={:?})",
                    context, row_el.key, row_el.rect[3], max_row_h, row_el.rect
                );
                unique_issues
                    .entry((view_name.to_string(), format!("vr_row_h:{}", row_el.path)))
                    .or_insert_with(|| msg.clone());
                failures.push(msg);
            }
        }

        for desc in &els {
            if desc.exiting || desc.rect[2] <= 0.0 || desc.rect[3] <= 0.0 {
                continue;
            }
            // Walk up parents to find whether this element belongs to a row directly under `vr_idx`
            let mut cur = desc.parent;
            let mut direct_row = None;
            while let Some(p) = cur {
                if els[p].parent == Some(vr_idx) {
                    direct_row = Some(p);
                    break;
                }
                cur = els[p].parent;
            }
            if let Some(r_idx) = direct_row {
                let r = els[r_idx].rect;
                if r[3] > 0.0 {
                    let row_top = r[1];
                    let row_bot = r[1] + r[3];
                    let d_top = desc.rect[1];
                    let d_bot = desc.rect[1] + desc.rect[3];
                    if d_top < row_top - 1.5 || d_bot > row_bot + 1.5 {
                        let msg = format!(
                            "[{}] Virtual row child exceeds row height bounds: child={:?} row={:?} vr={} key={:?} text={:?} path={:?}",
                            context, desc.rect, r, vr_key, desc.key, desc.text, desc.path
                        );
                        unique_issues
                            .entry((view_name.to_string(), format!("vr_clip:{}", desc.path)))
                            .or_insert_with(|| msg.clone());
                        failures.push(msg);
                    }
                }
            }
        }
    }
}

fn assert_clean_appx_compact_vertical_geometry(
    rt: &Runtime<AppState>,
    context: &str,
    failures: &mut Vec<String>,
    unique_issues: &mut std::collections::BTreeMap<(String, String), String>,
) {
    let vp_h = rt.viewport.1;
    let els = rt.elements();
    if let Some(sc) = els.iter().find(|e| !e.exiting && e.key.as_deref() == Some("appx_scroll")) {
        if !sc.rect.iter().all(|v| v.is_finite()) || sc.rect[3] < 200.0 {
            let msg = format!(
                "[{}] Compact AppX 'appx_scroll' collapsed or non-finite: rect={:?} vp_h={}",
                context, sc.rect, vp_h
            );
            unique_issues
                .entry(("CompactAppxVertical".to_string(), "appx_scroll".to_string()))
                .or_insert_with(|| msg.clone());
            failures.push(msg);
        }
    } else {
        let msg = format!("[{}] Missing 'appx_scroll' in CleanAndApps/AppxFeatures compact layout", context);
        unique_issues
            .entry(("CompactAppxVertical".to_string(), "appx_scroll".to_string()))
            .or_insert_with(|| msg.clone());
        failures.push(msg);
    }

    let has_appx_vr = els
        .iter()
        .any(|e| !e.exiting && e.key.as_deref() == Some("appx_virtual_rows") && e.rect[3] > 0.0);
    if !has_appx_vr {
        let msg = format!("[{}] Expected non-zero height 'appx_virtual_rows' in Compact AppxFeatures", context);
        unique_issues
            .entry(("CompactAppxVertical".to_string(), "appx_virtual_rows".to_string()))
            .or_insert_with(|| msg.clone());
        failures.push(msg);
    }

    let has_feature_row = els.iter().any(|e| {
        !e.exiting
            && e.key.as_deref().is_some_and(|k| k.starts_with("feat_"))
            && e.rect[2] > 0.0
            && e.rect[3] > 0.0
    });
    if !has_feature_row {
        let msg = format!("[{}] Expected rendered 'feat_*' rows in Compact AppxFeatures", context);
        unique_issues
            .entry(("CompactAppxVertical".to_string(), "feat_rows".to_string()))
            .or_insert_with(|| msg.clone());
        failures.push(msg);
    }
}

#[test]
fn test_ui_responsive_matrix_all_viewports_views_themes_locales_scales() {
    let state = build_populated_sandbox_state();
    let mut rt = Runtime::new(state, ui::view, (1280.0, 720.0), Env::new(true, 1.0));
    let mut failures = Vec::new();
    let mut unique_issues = std::collections::BTreeMap::new();

    // 1. Full Viewport x ActiveView x ThemeMode sweep (with English & scales 1.0/1.5)
    for &(vp_w, vp_h) in VIEWPORTS {
        for &theme in THEMES {
            let scale = if vp_w <= 640.0 { 1.5 } else { 1.0 };
            rt.set_viewport((vp_w, vp_h), scale);
            rt.set_system_dark(theme == ThemeMode::Dark);
            rt.update(|s, _| {
                s.theme_mode = theme;
                s.theme_palette = ThemePalette::new(theme, s.color_scheme);
                s.set_language(Language::English);
                s.show_changelog = false;
                s.show_diff_preview = false;
                s.sidebar_open = false;
            });

            for &view_tab in ALL_VIEWS {
                rt.update(|s, _| s.navigate(view_tab));
                match view_tab {
                    ActiveView::CleanAndApps => {
                        for sub in [
                            CleanAppsSubTab::DiskCleaner,
                            CleanAppsSubTab::AppxFeatures,
                            CleanAppsSubTab::WinGetInstaller,
                        ] {
                            rt.update(|s, _| s.clean_sub_tab = sub);
                            settle_frames(&mut rt);
                            let ctx = format!(
                                "vp={}x{} scale={} theme={:?} lang=English view={:?}/{:?}",
                                vp_w, vp_h, scale, theme, view_tab, sub
                            );
                            let vname = format!("{:?}/{:?}", view_tab, sub);
                            assert_frame_clean(&rt, &vname, &ctx, &mut failures, &mut unique_issues);
                            if vp_w == 420.0 && vp_h == 400.0 && sub == CleanAppsSubTab::AppxFeatures {
                                assert_clean_appx_compact_vertical_geometry(
                                    &rt,
                                    &ctx,
                                    &mut failures,
                                    &mut unique_issues,
                                );
                            }
                        }
                    }
                    ActiveView::StartupAndTasks => {
                        for sub in [
                            StartupTasksSubTab::StartupApps,
                            StartupTasksSubTab::ScheduledTasks,
                            StartupTasksSubTab::Services,
                        ] {
                            rt.update(|s, _| s.startup_sub_tab = sub);
                            settle_frames(&mut rt);
                            let ctx = format!(
                                "vp={}x{} scale={} theme={:?} lang=English view={:?}/{:?}",
                                vp_w, vp_h, scale, theme, view_tab, sub
                            );
                            let vname = format!("{:?}/{:?}", view_tab, sub);
                            assert_frame_clean(&rt, &vname, &ctx, &mut failures, &mut unique_issues);
                        }
                    }
                    _ => {
                        settle_frames(&mut rt);
                        let ctx = format!(
                            "vp={}x{} scale={} theme={:?} lang=English view={:?}",
                            vp_w, vp_h, scale, theme, view_tab
                        );
                        assert_frame_clean(&rt, &format!("{:?}", view_tab), &ctx, &mut failures, &mut unique_issues);
                    }
                }
            }
        }
    }

    // 2. Verbose Locales (Turkish, German, Russian) & DPI Scales (1.0, 1.25, 1.5) across representative viewports & ALL 8 views + sub-tabs
    let locale_viewports = [(420.0, 400.0), (800.0, 600.0), (1366.0, 768.0), (3840.0, 2160.0)];
    for &lang in VERBOSE_LOCALES {
        for &scale in SCALES {
            for &(vp_w, vp_h) in &locale_viewports {
                rt.set_viewport((vp_w, vp_h), scale);
                let theme = if scale == 1.25 { ThemeMode::Light } else { ThemeMode::Dark };
                rt.set_system_dark(theme == ThemeMode::Dark);
                rt.update(|s, _| {
                    s.theme_mode = theme;
                    s.theme_palette = ThemePalette::new(theme, s.color_scheme);
                    s.set_language(lang);
                    s.show_changelog = false;
                    s.show_diff_preview = false;
                    s.sidebar_open = false;
                });

                for &view_tab in ALL_VIEWS {
                    rt.update(|s, _| s.navigate(view_tab));
                    match view_tab {
                        ActiveView::CleanAndApps => {
                            for sub in [
                                CleanAppsSubTab::DiskCleaner,
                                CleanAppsSubTab::AppxFeatures,
                                CleanAppsSubTab::WinGetInstaller,
                            ] {
                                rt.update(|s, _| s.clean_sub_tab = sub);
                                settle_frames(&mut rt);
                                let ctx = format!(
                                    "vp={}x{} scale={} theme={:?} lang={:?} view={:?}/{:?}",
                                    vp_w, vp_h, scale, theme, lang, view_tab, sub
                                );
                                let vname = format!("{:?}/{:?}", view_tab, sub);
                                assert_frame_clean(&rt, &vname, &ctx, &mut failures, &mut unique_issues);
                                if vp_w == 420.0 && vp_h == 400.0 && sub == CleanAppsSubTab::AppxFeatures {
                                    assert_clean_appx_compact_vertical_geometry(
                                        &rt,
                                        &ctx,
                                        &mut failures,
                                        &mut unique_issues,
                                    );
                                }
                            }
                        }
                        ActiveView::StartupAndTasks => {
                            for sub in [
                                StartupTasksSubTab::StartupApps,
                                StartupTasksSubTab::ScheduledTasks,
                                StartupTasksSubTab::Services,
                            ] {
                                rt.update(|s, _| s.startup_sub_tab = sub);
                                settle_frames(&mut rt);
                                let ctx = format!(
                                    "vp={}x{} scale={} theme={:?} lang={:?} view={:?}/{:?}",
                                    vp_w, vp_h, scale, theme, lang, view_tab, sub
                                );
                                let vname = format!("{:?}/{:?}", view_tab, sub);
                                assert_frame_clean(&rt, &vname, &ctx, &mut failures, &mut unique_issues);
                            }
                        }
                        _ => {
                            settle_frames(&mut rt);
                            let ctx = format!(
                                "vp={}x{} scale={} theme={:?} lang={:?} view={:?}",
                                vp_w, vp_h, scale, theme, lang, view_tab
                            );
                            assert_frame_clean(&rt, &format!("{:?}", view_tab), &ctx, &mut failures, &mut unique_issues);
                        }
                    }
                }
            }
        }
    }

    // 3. Drawers & Modals open (`show_changelog = true`, `show_diff_preview = true`, `sidebar_open = true`)
    // across Compact, Medium, Wide, and 4K viewports with Turkish/German/Russian + Light & Dark
    let modal_configs = [
        (420.0, 400.0, 1.0, ThemeMode::Dark, Language::Turkish),
        (640.0, 480.0, 1.25, ThemeMode::Light, Language::German),
        (800.0, 600.0, 1.5, ThemeMode::Dark, Language::Russian),
        (1920.0, 1080.0, 1.0, ThemeMode::Light, Language::Turkish),
        (3840.0, 2160.0, 1.5, ThemeMode::Dark, Language::German),
    ];
    for &(vp_w, vp_h, scale, theme, lang) in &modal_configs {
        rt.set_viewport((vp_w, vp_h), scale);
        rt.set_system_dark(theme == ThemeMode::Dark);

        // Changelog drawer + Sidebar drawer (compact/medium)
        rt.update(|s, _| {
            s.theme_mode = theme;
            s.theme_palette = ThemePalette::new(theme, s.color_scheme);
            s.set_language(lang);
            s.navigate(ActiveView::BackupCenter);
            s.show_changelog = true;
            s.show_diff_preview = false;
            s.sidebar_open = vp_w < 1020.0;
        });
        settle_frames(&mut rt);
        assert_frame_clean(
            &rt,
            "DrawerChangelog+Sidebar",
            &format!("drawer_changelog+sidebar vp={}x{} scale={} {:?} {:?}", vp_w, vp_h, scale, theme, lang),
            &mut failures,
            &mut unique_issues,
        );

        // Diff preview modal + Changelog drawer
        rt.update(|s, _| {
            s.sidebar_open = false;
            s.show_diff_preview = true;
            s.show_changelog = true;
        });
        settle_frames(&mut rt);
        assert_frame_clean(
            &rt,
            "ModalDiff+Changelog",
            &format!("modal_diff_preview+changelog vp={}x{} scale={} {:?} {:?}", vp_w, vp_h, scale, theme, lang),
            &mut failures,
            &mut unique_issues,
        );
    }

    assert!(
        failures.is_empty(),
        "Found {} total UI layout/key/overflow failures ({} unique (view, path) entries):\n{}",
        failures.len(),
        unique_issues.len(),
        unique_issues.values().cloned().collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn test_ui_live_viewport_sweep_and_compact_nav_menu_interaction() {
    let state = build_populated_sandbox_state();
    let mut rt = Runtime::new(state, ui::view, (1150.0, 780.0), Env::new(true, 1.0));
    let mut failures = Vec::new();
    let mut unique_issues = std::collections::BTreeMap::new();

    // Live viewport resize sweep: 1150x780 -> 800x600 -> 420x400 -> 3840x2160 -> 1920x1080
    let sweep = [
        (1150.0, 780.0),
        (800.0, 600.0),
        (420.0, 400.0),
        (3840.0, 2160.0),
        (1920.0, 1080.0),
    ];
    for &(w, h) in &sweep {
        rt.set_viewport((w, h), 1.0);
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "LiveSweep", &format!("live_sweep {}x{}", w, h), &mut failures, &mut unique_issues);
    }

    // Resize down to Compact 420x400 and test interactive navigation drawer toggle -> GpuTuning
    rt.set_viewport((420.0, 400.0), 1.0);
    settle_frames(&mut rt);
    assert_eq!(rt.state.active_view, ActiveView::EasyDashboard);
    assert!(!rt.state.sidebar_open);

    // Click compact menu toggle button
    click_key(&mut rt, "nav_menu_toggle");
    assert!(
        rt.state.sidebar_open,
        "Clicking 'nav_menu_toggle' at 420x400 must open sidebar drawer"
    );
    assert_frame_clean(&rt, "DrawerOpen", "compact_420x400_drawer_open", &mut failures, &mut unique_issues);

    // Click GPU Tuning nav button inside the opened drawer
    click_key(&mut rt, "nav_gpu");
    assert_eq!(
        rt.state.active_view,
        ActiveView::GpuTuning,
        "Clicking 'nav_gpu' inside compact drawer must navigate to ActiveView::GpuTuning"
    );
    assert!(
        !rt.state.sidebar_open,
        "Navigating via 'nav_gpu' must auto-close the compact drawer"
    );
    assert_frame_clean(&rt, "AfterNavGpu", "compact_420x400_after_nav_gpu", &mut failures, &mut unique_issues);

    // Click Clean & Apps (tab 9) and Startup & Tasks (tab 10) inside 420x400 compact drawer
    click_key(&mut rt, "nav_menu_toggle");
    click_key(&mut rt, "nav_clean_apps");
    assert_eq!(rt.state.active_view, ActiveView::CleanAndApps);
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::DiskCleaner);
    assert_frame_clean(&rt, "CompactCleanApps", "compact_420x400_clean_apps", &mut failures, &mut unique_issues);

    click_key(&mut rt, "sub_clean_appx");
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::AppxFeatures);
    assert_frame_clean(&rt, "CompactCleanAppx", "compact_420x400_clean_appx", &mut failures, &mut unique_issues);
    assert_clean_appx_compact_vertical_geometry(
        &rt,
        "compact_420x400_clean_appx",
        &mut failures,
        &mut unique_issues,
    );
    click_key(&mut rt, "sub_clean_winget");
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::WinGetInstaller);
    assert_frame_clean(&rt, "CompactCleanWinGet", "compact_420x400_clean_winget", &mut failures, &mut unique_issues);
    click_key(&mut rt, "sub_clean_disk");
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::DiskCleaner);

    click_key(&mut rt, "nav_menu_toggle");
    click_key(&mut rt, "nav_startup_tasks");
    assert_eq!(rt.state.active_view, ActiveView::StartupAndTasks);
    assert_eq!(rt.state.startup_sub_tab, StartupTasksSubTab::StartupApps);
    assert_frame_clean(&rt, "CompactStartupApps", "compact_420x400_startup_apps", &mut failures, &mut unique_issues);

    click_key(&mut rt, "sub_startup_tasks");
    assert_eq!(rt.state.startup_sub_tab, StartupTasksSubTab::ScheduledTasks);
    assert_frame_clean(&rt, "CompactStartupTasks", "compact_420x400_startup_tasks", &mut failures, &mut unique_issues);
    click_key(&mut rt, "sub_startup_services");
    assert_eq!(rt.state.startup_sub_tab, StartupTasksSubTab::Services);
    assert_frame_clean(&rt, "CompactStartupServices", "compact_420x400_startup_services", &mut failures, &mut unique_issues);
    click_key(&mut rt, "sub_startup_apps");
    assert_eq!(rt.state.startup_sub_tab, StartupTasksSubTab::StartupApps);

    // Interactive switch_indicator / switch_toggle_with checks + Turkish I/ı search + filter/preset resets
    rt.set_viewport((1280.0, 960.0), 1.0);
    settle_frames(&mut rt);
    let selected_tweaks_baseline = rt.state.selected_tweaks.clone();
    assert!(
        !selected_tweaks_baseline.is_empty(),
        "Baseline state should have selected tweaks so any switch pollution would be distinguishable"
    );

    // A) Clean & Apps -> Disk Cleaner switch click (`clean_sw_*`)
    rt.update(|s, _| {
        s.navigate(ActiveView::CleanAndApps);
        s.clean_sub_tab = CleanAppsSubTab::DiskCleaner;
    });
    settle_frames(&mut rt);
    let first_clean_id = rt.state.clean_targets[0].kind.id_key().to_string();
    let first_clean_sw = format!("clean_sw_{}", first_clean_id);
    let was_clean_selected = rt.state.clean_targets[0].selected;
    click_key(&mut rt, &first_clean_sw);
    assert_eq!(
        rt.state.clean_targets[0].selected,
        !was_clean_selected,
        "Clicking '{}' must toggle clean target selection",
        first_clean_sw
    );
    assert_eq!(
        rt.state.selected_tweaks, selected_tweaks_baseline,
        "Clicking clean switch must NEVER pollute AppState.selected_tweaks"
    );

    // B) Clean & Apps -> AppX switch click (`appx_sw_*` for SafeBloat + ProtectedCore) & Turkish I/ı search
    click_key(&mut rt, "sub_clean_appx");
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::AppxFeatures);

    // Filter to SafeBloat BingWeather via Turkish dotless 'ı' ("bıng" -> "bing")
    assert!(rt.find("appx_search_input").is_some(), "Missing appx_search_input");
    let safe_appx = rt
        .state
        .installed_appx
        .iter()
        .find(|p| p.family_name.eq_ignore_ascii_case("Microsoft.BingWeather_8wekyb3d8bbwe"))
        .expect("Missing BingWeather AppX entry")
        .family_name
        .clone();
    let core_appx = rt
        .state
        .installed_appx
        .iter()
        .find(|p| p.family_name.eq_ignore_ascii_case("Microsoft.SecHealthUI_8wekyb3d8bbwe"))
        .expect("Missing SecHealthUI AppX entry")
        .family_name
        .clone();

    rt.update(|s, _| s.appx_filter_edit.text = "B\u{0131}NG".to_string());
    settle_frames(&mut rt);
    let els_appx_tr = rt.elements();
    assert!(
        els_appx_tr.iter().any(|e| {
            !e.exiting
                && e.key
                    .as_deref()
                    .is_some_and(|k| k.eq_ignore_ascii_case(&format!("appx_{}", safe_appx)))
        }),
        "Turkish I/ı filter 'BıNG' should reveal {}",
        safe_appx
    );
    assert!(
        !els_appx_tr.iter().any(|e| {
            !e.exiting
                && e.key
                    .as_deref()
                    .is_some_and(|k| k.to_ascii_lowercase().starts_with("appx_clipchamp"))
        }),
        "Turkish I/ı filter 'BıNG' should filter out Clipchamp"
    );

    // Click SafeBloat switch `appx_sw_*` directly
    let safe_appx_was_sel = rt.state.selected_appx.contains(&safe_appx);
    click_key(&mut rt, &format!("appx_sw_{}", safe_appx));
    assert_eq!(
        rt.state.selected_appx.contains(&safe_appx),
        !safe_appx_was_sel,
        "Clicking appx_sw_* on SafeBloat package must toggle selected_appx"
    );
    assert_eq!(
        rt.state.selected_tweaks, selected_tweaks_baseline,
        "Clicking AppX switch must NEVER pollute AppState.selected_tweaks"
    );

    // Filter to ProtectedCore `Microsoft.SecHealthUI_8wekyb3d8bbwe` and click `appx_sw_*` -> must stay locked!
    rt.update(|s, _| s.appx_filter_edit.text = "SecHealthUI".to_string());
    settle_frames(&mut rt);
    click_key(&mut rt, &format!("appx_sw_{}", core_appx));
    assert!(
        !rt.state.selected_appx.contains(&core_appx),
        "ProtectedCore AppX package switch must remain locked when clicked"
    );
    assert_eq!(
        rt.state.selected_tweaks, selected_tweaks_baseline,
        "Clicking ProtectedCore AppX switch must NEVER pollute AppState.selected_tweaks"
    );
    rt.update(|s, _| s.appx_filter_edit.text.clear());
    settle_frames(&mut rt);

    // C) Clean & Apps -> WinGet switch click (`wg_sw_*`) & Turkish İ search (`winget_search_input`)
    click_key(&mut rt, "sub_clean_winget");
    assert_eq!(rt.state.clean_sub_tab, CleanAppsSubTab::WinGetInstaller);
    assert!(rt.find("winget_search_input").is_some(), "Missing winget_search_input");
    let was_7z_sel = rt.state.selected_winget.contains("7zip.7zip");
    click_key(&mut rt, "wg_sw_7zip.7zip");
    assert_eq!(
        rt.state.selected_winget.contains("7zip.7zip"),
        !was_7z_sel,
        "Clicking 'wg_sw_7zip.7zip' must toggle selected_winget"
    );
    assert_eq!(
        rt.state.selected_tweaks, selected_tweaks_baseline,
        "Clicking WinGet switch must NEVER pollute AppState.selected_tweaks"
    );

    rt.update(|s, _| s.winget_filter_edit.text = "G\u{0130}T".to_string());
    settle_frames(&mut rt);
    let els_wg_tr = rt.elements();
    assert!(
        els_wg_tr.iter().any(|e| !e.exiting && e.key.as_deref() == Some("wg_Git.Git")),
        "Turkish İ filter 'GİT' should render wg_Git.Git"
    );
    assert!(
        !els_wg_tr.iter().any(|e| !e.exiting && e.key.as_deref() == Some("wg_7zip.7zip")),
        "Turkish İ filter 'GİT' should hide wg_7zip.7zip"
    );
    rt.update(|s, _| s.winget_filter_edit.text.clear());

    // D) Startup & Tasks -> `st_sw_*` toggle, `hide_microsoft_tasks` toggle, Turkish İ search in `startup_search_input`
    rt.update(|s, _| {
        s.navigate(ActiveView::StartupAndTasks);
        s.startup_sub_tab = StartupTasksSubTab::StartupApps;
    });
    settle_frames(&mut rt);
    assert!(rt.find("startup_search_input").is_some(), "Missing startup_search_input");
    let discord_id = "HKCU|Run|DiscordUpdater";
    let discord_before = rt
        .state
        .startup_entries
        .iter()
        .find(|e| e.id == discord_id)
        .expect("Missing DiscordUpdater startup entry")
        .enabled;
    click_key(&mut rt, &format!("st_sw_{}", discord_id));
    drain_state_workers(&mut rt, |s| {
        !s.startup_tasks_busy
            && s.startup_entries
                .iter()
                .any(|e| e.id == discord_id && e.enabled == !discord_before)
    });
    let discord_after = rt
        .state
        .startup_entries
        .iter()
        .find(|e| e.id == discord_id)
        .unwrap()
        .enabled;
    assert_eq!(
        discord_after, !discord_before,
        "Clicking 'st_sw_*' must toggle non-locked startup entry via worker"
    );
    assert_eq!(
        rt.state.selected_tweaks, selected_tweaks_baseline,
        "Clicking startup switch must NEVER pollute AppState.selected_tweaks"
    );

    // Toggle HideMicrosoftTasks button -> false reveals OneDrive
    assert!(rt.state.hide_microsoft_tasks);
    let hide_ms_label = rt.state.tr(TextKey::HideMicrosoftTasks).to_string();
    click_text(&mut rt, &hide_ms_label, false);
    assert!(
        !rt.state.hide_microsoft_tasks,
        "Clicking HideMicrosoftTasks button must toggle rt.state.hide_microsoft_tasks to false"
    );
    assert!(
        rt.elements()
            .iter()
            .any(|e| !e.exiting && e.key.as_deref() == Some("st_HKCU|Run|OneDrive")),
        "Disabling hide_microsoft_tasks must show Microsoft OneDrive startup item"
    );

    // Turkish İ search filtering in startup_search_input
    rt.update(|s, _| s.startup_filter_edit.text = "ONEDR\u{0130}VE".to_string());
    settle_frames(&mut rt);
    let els_st_tr = rt.elements();
    assert!(
        els_st_tr
            .iter()
            .any(|e| !e.exiting && e.key.as_deref() == Some("st_HKCU|Run|OneDrive")),
        "Turkish İ query 'ONEDRİVE' must keep OneDrive startup entry visible"
    );
    assert!(
        !els_st_tr
            .iter()
            .any(|e| !e.exiting && e.key.as_deref() == Some("st_HKCU|Run|DiscordUpdater")),
        "Turkish İ query 'ONEDRİVE' must filter out DiscordUpdater"
    );
    rt.update(|s, _| s.startup_filter_edit.text.clear());

    // E) BackupCenter Unattend preset & arch buttons
    rt.set_viewport((1280.0, 1500.0), 1.0);
    rt.update(|s, _| s.navigate(ActiveView::BackupCenter));
    settle_frames(&mut rt);

    let preset_min = rt.state.tr(TextKey::UnattendPresetMinimal).to_string();
    click_text(&mut rt, &preset_min, true);
    assert!(!rt.state.unattend_opts.skip_oobe_privacy_screens);
    assert!(!rt.state.unattend_opts.auto_timezone_utc);

    let preset_gam = rt.state.tr(TextKey::UnattendPresetGaming).to_string();
    click_text(&mut rt, &preset_gam, true);
    assert!(rt.state.unattend_opts.skip_oobe_privacy_screens);
    assert!(rt.state.unattend_opts.include_selected_tweaks);
    assert!(rt.state.unattend_opts.auto_timezone_utc);

    let arch_before = rt.state.unattend_opts.architecture;
    let arch_prefix = rt.state.tr(TextKey::UnattendArchToggle).to_string();
    click_text(&mut rt, &arch_prefix, false);
    let expected_arch = match arch_before {
        TargetArch::Amd64 => TargetArch::Arm64,
        TargetArch::Arm64 => TargetArch::Amd64,
    };
    assert_eq!(
        rt.state.unattend_opts.architecture, expected_arch,
        "Clicking UnattendArchToggle must switch architecture"
    );

    // F) LatencyTester clear button & AdvancedMatrix clear filters button
    rt.set_viewport((1280.0, 960.0), 1.0);
    rt.update(|s, _| s.navigate(ActiveView::LatencyTester));
    settle_frames(&mut rt);
    assert!(rt.state.latency_before.is_some());
    let clear_lat = rt.state.tr(TextKey::LatencyClearBtn).to_string();
    click_text(&mut rt, &clear_lat, true);
    assert!(rt.state.latency_before.is_none());
    assert!(rt.state.latency_after.is_none());
    assert!(rt.state.latest_latency_bench.is_none());

    rt.update(|s, _| {
        s.navigate(ActiveView::AdvancedMatrix);
        s.search_edit.text = "telemetry".to_string();
        s.selected_category = Some("Privacy".to_string());
    });
    settle_frames(&mut rt);
    let clear_adv = rt.state.tr(TextKey::ClearFiltersBtn).to_string();
    click_text(&mut rt, &clear_adv, true);
    assert!(rt.state.search_edit.text.is_empty());
    assert!(rt.state.search_query.is_empty());
    assert!(rt.state.selected_category.is_none());

    assert!(
        failures.is_empty(),
        "Found {} failures ({} unique) during live sweep & compact nav interaction:\n{}",
        failures.len(),
        unique_issues.len(),
        unique_issues.values().cloned().collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn test_ui_no_key_warnings_across_all_views_via_headless_tree() {
    // Rune's `Runtime::key_warnings` is `pub(crate)`, while `rune::App` with `--tree` populates
    // `rt.key_warnings = Some(Vec::new())` and emits `"warnings": [...]` in the JSON tree dump.
    // We also validate `#dup` absence across every `Runtime::new` render above; here we also verify
    // zero warnings via Rune's official `--tree` dump if a headless GPU adapter is present.
    let mut tree_path = std::env::temp_dir();
    tree_path.push(format!("winblast0_tree_{}.json", uuid::Uuid::new_v4()));

    let spec = rune::AppSpec::new(
        "WinBlast0 Key Audit",
        (1280.0, 720.0),
        build_populated_sandbox_state,
        ui::view,
    );

    let res = rune::App::new(spec)
        .args([
            "--tree".to_string(),
            tree_path.to_string_lossy().to_string(),
            "--size".to_string(),
            "1280x720".to_string(),
        ])
        .try_run();

    match res {
        Ok(()) => {
            let json_str = std::fs::read_to_string(&tree_path)
                .expect("Headless --tree succeeded but JSON output was missing");
            let val: serde_json::Value =
                serde_json::from_str(&json_str).expect("Invalid JSON from --tree");
            let warnings = val["warnings"]
                .as_array()
                .expect("Missing 'warnings' array in --tree JSON");
            assert!(
                warnings.is_empty(),
                "Rune reported key_warnings in --tree output: {:?}",
                warnings
            );
            let _ = std::fs::remove_file(&tree_path);
        }
        Err(e) => {
            let _ = std::fs::remove_file(&tree_path);
            panic!("Headless --tree execution failed: {}", e);
        }
    }
}

#[test]
fn test_ui_v08_not_applicable_diff_modal_privacy_cores_and_protected_appx() {
    use winblast0::core::risk::TweakAction;
    use winblast0::hardware::parking::{CoreParkingState, CoreStatus};
    use winblast0::scanner::TweakState;

    let tweaks = load_embedded_rules();
    let sbx = SandboxEnv::new();
    {
        let mut st = sbx.inner.write().unwrap();
        st.os_info.build_number = 26300;
        st.os_info.is_win11 = true;
        st.os_info.edition = "Professional".to_string();
        st.os_info.display_version = "24H2".to_string();
        st.appx_packages
            .insert("Microsoft.SecHealthUI_8wekyb3d8bbwe".to_string());
        st.appx_packages
            .insert("Microsoft.WindowsStore_8wekyb3d8bbwe".to_string());
        st.appx_packages
            .insert("Microsoft.BingWeather_8wekyb3d8bbwe".to_string());
    }
    sbx.seed_all_tweaks(&tweaks);

    // Pre-apply one pure RegistryDword tweak so we have an AlreadyOptimized tweak to StageRevert
    let revert_tweak = tweaks
        .iter()
        .find(|t| {
            t.easy_mode_included
                && t.os_req.max_build.is_none()
                && t.gpu_vendor.is_none()
                && t.actions.len() == 1
                && matches!(t.actions[0], TweakAction::RegistryDword { .. })
        })
        .expect("Expected single-RegistryDword tweak")
        .clone();
    let apply_tweak = tweaks
        .iter()
        .find(|t| {
            t.id != revert_tweak.id
                && t.easy_mode_included
                && t.os_req.max_build.is_none()
                && t.gpu_vendor.is_none()
                && t.actions.len() == 1
                && matches!(t.actions[0], TweakAction::RegistryDword { .. })
        })
        .expect("Expected second single-RegistryDword tweak")
        .clone();

    let ledger = winblast0::backup::state_ledger::StateLedger::in_memory().unwrap();
    winblast0::executor::engine::apply_tweak_with(&revert_tweak, &ledger, &sbx)
        .expect("Pre-apply revert_tweak failed");

    let audit = full_telemetry_audit_with(&sbx);
    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.telemetry_audit = Some(audit);
    state.start_scan();
    state.scan_appx_and_features_async();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        state.drain_worker_events();
        if !state.is_scanning && !state.scan_results.is_empty() && !state.installed_appx.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(
        state.get_tweak_scan("disable_mpo").unwrap().current_state,
        TweakState::NotApplicable,
        "disable_mpo must be NotApplicable on build 26300"
    );
    assert_eq!(
        state.get_tweak_scan(&revert_tweak.id).unwrap().current_state,
        TweakState::AlreadyOptimized
    );
    assert_eq!(
        state.get_tweak_scan(&apply_tweak.id).unwrap().current_state,
        TweakState::NotOptimized
    );

    let mut rt = Runtime::new(state, ui::view, (1280.0, 960.0), Env::new(true, 1.0));
    let mut failures = Vec::new();
    let mut unique_issues = std::collections::BTreeMap::new();

    // 1. NotApplicable tweaks show StatusNotApplicable without clickable switch_toggle
    rt.update(|s, _| {
        s.navigate(ActiveView::AdvancedMatrix);
        s.search_edit.text = "disable_mpo".to_string();
    });
    settle_frames(&mut rt);
    assert_frame_clean(&rt, "NotApplicableFilter", "v08_not_applicable", &mut failures, &mut unique_issues);

    let not_app_label = rt.state.tr(TextKey::StatusNotApplicable).to_string();
    let els_na = rt.elements();
    assert!(
        els_na
            .iter()
            .any(|e| !e.exiting && e.text.as_deref() == Some(not_app_label.as_str())),
        "Expected StatusNotApplicable badge for disable_mpo in AdvancedMatrix"
    );
    let has_switch_knob_shell = els_na.iter().any(|e| {
        !e.exiting && (e.rect[2] - 36.0).abs() < 0.5 && (e.rect[3] - 20.0).abs() < 0.5
    });
    assert!(
        !has_switch_knob_shell,
        "NotApplicable tweak must NOT render a 36x20 clickable switch_toggle"
    );
    click_text(&mut rt, &not_app_label, true);
    assert!(!rt.state.selected_tweaks.contains("disable_mpo"));
    assert!(!rt.state.staged_transitions.contains_key("disable_mpo"));

    // Same check on EasyDashboard (disable_mpo is easy_mode_included = true)
    rt.update(|s, _| {
        s.search_edit.text.clear();
        s.navigate(ActiveView::EasyDashboard);
    });
    settle_frames(&mut rt);
    let els_easy = rt.elements();
    let expected_easy_switches = rt
        .state
        .scan_results
        .iter()
        .filter(|r| r.easy_mode_included && r.current_state != TweakState::NotApplicable)
        .count();
    let actual_easy_switches = els_easy
        .iter()
        .filter(|e| !e.exiting && (e.rect[2] - 36.0).abs() < 0.5 && (e.rect[3] - 20.0).abs() < 0.5)
        .count();
    assert_eq!(
        actual_easy_switches, expected_easy_switches,
        "EasyDashboard must render switch_toggle ONLY for applicable tweaks (not NotApplicable)"
    );
    assert!(
        els_easy
            .iter()
            .any(|e| !e.exiting && e.text.as_deref() == Some(not_app_label.as_str())),
        "EasyDashboard must render StatusNotApplicable badge for NotApplicable easy tweaks"
    );

    // 2. StageRevert renders [Revert] badge & swapped labels inside DiffModal (open_diff_preview vs inspect_tweak)
    rt.update(|s, _| {
        s.navigate(ActiveView::AdvancedMatrix);
        s.deselect_all();
        s.handle_switch_click(revert_tweak.id.clone());
        s.handle_switch_click(apply_tweak.id.clone());
    });
    settle_frames(&mut rt);

    // Inspect a tweak -> no [Revert] prefix
    rt.update(|s, _| s.inspect_tweak(&apply_tweak.id));
    settle_frames(&mut rt);
    let revert_prefix = format!("[{}]", rt.state.tr(TextKey::RevertLabel));
    assert!(rt.state.show_diff_preview && rt.state.inspected_tweak.is_some());
    assert!(
        !rt.elements()
            .iter()
            .any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.starts_with(&revert_prefix))),
        "inspect_tweak modal must not show [Revert] prefix"
    );

    // Dismiss inspect_tweak via CloseBtn, then open_diff_preview via PreviewDiff button
    let close_lbl = rt.state.tr(TextKey::CloseBtn).to_string();
    click_text(&mut rt, &close_lbl, true);
    assert!(!rt.state.show_diff_preview && rt.state.inspected_tweak.is_none());

    let preview_lbl = rt.state.tr(TextKey::PreviewDiff).to_string();
    click_text(&mut rt, &preview_lbl, true);
    assert!(rt.state.show_diff_preview && rt.state.inspected_tweak.is_none());
    assert_frame_clean(&rt, "DiffModalRevert", "v08_diff_modal_revert", &mut failures, &mut unique_issues);

    let els_diff = rt.elements();
    assert!(
        els_diff
            .iter()
            .any(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.starts_with(&revert_prefix))),
        "open_diff_preview with StageRevert must render '{}' badge header",
        revert_prefix
    );

    // Verify swapped Target: -> Stock: order inside the [Revert] card
    let tgt_lbl = rt.state.tr(TextKey::DiffTargetLabel).to_string();
    let def_lbl = rt.state.tr(TextKey::DiffDefaultLabel).to_string();
    let revert_hdr_idx = els_diff
        .iter()
        .position(|e| !e.exiting && e.text.as_deref().is_some_and(|t| t.starts_with(&revert_prefix)))
        .expect("Missing [Revert] header element");
    let card_idx = els_diff[revert_hdr_idx]
        .parent
        .expect("[Revert] header must have card parent");
    let card_texts: Vec<&str> = els_diff
        .iter()
        .filter(|e| {
            !e.exiting
                && (e.parent == Some(card_idx)
                    || e.parent.and_then(|p| els_diff[p].parent) == Some(card_idx))
        })
        .filter_map(|e| e.text.as_deref())
        .collect();
    let tgt_pos = card_texts
        .iter()
        .position(|t| t.starts_with(&tgt_lbl))
        .expect("Missing Target: label inside [Revert] card");
    let def_pos = card_texts
        .iter()
        .position(|t| t.starts_with(&def_lbl))
        .expect("Missing Stock: label inside [Revert] card");
    assert!(
        tgt_pos < def_pos,
        "Inside [Revert] card, Target: must appear before Stock: (got {:?})",
        card_texts
    );
    click_text(&mut rt, &close_lbl, true);

    // 3. PrivacyShield still renders RunAuditBtn when telemetry_audit is Some(...)
    rt.update(|s, _| s.navigate(ActiveView::PrivacyShield));
    for &(w, h) in &[(420.0, 400.0), (1280.0, 720.0)] {
        rt.set_viewport((w, h), 1.0);
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "PrivacyAuditBtn", &format!("privacy_{}x{}", w, h), &mut failures, &mut unique_issues);
        let run_audit_lbl = rt.state.tr(TextKey::RunAuditBtn).to_string();
        assert!(
            rt.elements().iter().any(|e| {
                !e.exiting
                    && e.rect[2] > 0.0
                    && e.rect[3] > 0.0
                    && e.text.as_deref() == Some(run_audit_lbl.as_str())
            }),
            "PrivacyShield must render RunAuditBtn even when telemetry_audit is Some(...) at {}x{}",
            w,
            h
        );
    }
    let run_audit_lbl = rt.state.tr(TextKey::RunAuditBtn).to_string();
    click_text(&mut rt, &run_audit_lbl, true);
    drain_state_workers(&mut rt, |s| s.telemetry_audit.is_some());
    assert!(rt.elements().iter().any(|e| !e.exiting && e.text.as_deref() == Some(run_audit_lbl.as_str())));

    // 4. CoreParking renders single-numbered cores (no "Core #0 #0") and HardwareView renders >8 cores + live latency
    rt.update(|s, _| {
        let mut cores = Vec::with_capacity(16);
        for i in 0..16 {
            cores.push(CoreStatus {
                core_index: i,
                is_parked: i >= 12,
                usage_pct: if i >= 12 { 0.0 } else { 18.5 + i as f32 },
                core_type_label: if i < 8 { "P-Core" } else { "E-Core" }.to_string(),
            });
        }
        s.core_parking = CoreParkingState {
            total_cores: 16,
            unparked_count: 12,
            parked_count: 4,
            min_cores_ac_pct: 50,
            min_cores_dc_pct: 10,
            max_cores_ac_pct: 100,
            is_fully_unparked: false,
            is_hybrid_architecture: true,
            cores,
            ..Default::default()
        };
        if let Some(ref mut m) = s.realtime_metrics {
            m.core_usages = (0..16).map(|i| 10.0 + i as f32).collect();
        }
        s.latency = None;
        s.refresh_latency();
        s.navigate(ActiveView::CoreParking);
    });
    drain_state_workers(&mut rt, |s| s.latency.is_some());
    for &(w, h) in &[(420.0, 400.0), (1280.0, 960.0)] {
        rt.set_viewport((w, h), 1.0);
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "CoreParking16", &format!("parking_{}x{}", w, h), &mut failures, &mut unique_issues);
        let els_cp = rt.elements();
        for expected in ["P-Core #0", "P-Core #7", "E-Core #8", "E-Core #15"] {
            assert!(
                els_cp.iter().any(|e| !e.exiting && e.text.as_deref() == Some(expected)),
                "Missing core card '{}' at {}x{}",
                expected,
                w,
                h
            );
        }
        for e in &els_cp {
            if let Some(ref t) = e.text {
                assert!(
                    !t.contains("#0 #0") && !t.contains("#1 #1"),
                    "Doubled core label detected: {:?}",
                    t
                );
                if t.starts_with("P-Core") || t.starts_with("E-Core") || t.starts_with("Core #") {
                    assert_eq!(t.matches('#').count(), 1, "Malformed core label: {:?}", t);
                }
            }
        }
    }

    rt.update(|s, _| s.navigate(ActiveView::HardwareView));
    for &(w, h) in &[(420.0, 400.0), (1280.0, 960.0)] {
        rt.set_viewport((w, h), 1.0);
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "Hardware16Cores", &format!("hardware_{}x{}", w, h), &mut failures, &mut unique_issues);
        let els_hw = rt.elements();
        for expected in ["C0:", "C8:", "C15:"] {
            assert!(
                els_hw.iter().any(|e| {
                    !e.exiting && e.text.as_deref().is_some_and(|t| t.starts_with(expected))
                }),
                "HardwareView missing >8 core chip '{}' at {}x{}",
                expected,
                w,
                h
            );
        }
        let lat_title = rt.state.tr(TextKey::LatencySnapshotTitle).to_string();
        assert!(
            els_hw.iter().any(|e| {
                !e.exiting && e.text.as_deref().is_some_and(|t| t.starts_with(&lat_title))
            }),
            "HardwareView missing live latency snapshot after refresh_latency() at {}x{}",
            w,
            h
        );
    }

    // 5. ProtectedCore AppX rows render lock indicator ("🔒") cleanly across 420x400..3840x2160
    // Test both unfiltered virtualized list AND filtered ProtectedCore row so the row is guaranteed
    // materialized inside `appx_virtual_rows` at every viewport height (including 420x400).
    rt.update(|s, _| {
        s.navigate(ActiveView::CleanAndApps);
        s.clean_sub_tab = CleanAppsSubTab::AppxFeatures;
        s.appx_filter_edit.text.clear();
    });
    for &(w, h) in &[(420.0, 400.0), (640.0, 480.0), (800.0, 600.0), (1366.0, 768.0), (1920.0, 1080.0), (3840.0, 2160.0)] {
        rt.update(|s, _| s.appx_filter_edit.text.clear());
        rt.set_viewport((w, h), 1.0);
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "ProtectedCoreAppxUnfiltered", &format!("appx_unfiltered_{}x{}", w, h), &mut failures, &mut unique_issues);

        rt.update(|s, _| s.appx_filter_edit.text = "SecHealthUI".to_string());
        settle_frames(&mut rt);
        assert_frame_clean(&rt, "ProtectedCoreAppx", &format!("appx_lock_{}x{}", w, h), &mut failures, &mut unique_issues);
        assert!(
            rt.elements()
                .iter()
                .any(|e| !e.exiting && e.rect[2] > 0.0 && e.rect[3] > 0.0 && e.text.as_deref() == Some("🔒")),
            "ProtectedCore AppX row must render lock indicator '🔒' at {}x{}",
            w,
            h
        );
    }
    rt.set_viewport((420.0, 400.0), 1.0);
    settle_frames(&mut rt);
    let core_appx_fam = rt
        .state
        .installed_appx
        .iter()
        .find(|p| p.family_name.eq_ignore_ascii_case("Microsoft.SecHealthUI_8wekyb3d8bbwe"))
        .expect("Missing SecHealthUI")
        .family_name
        .clone();
    click_key(&mut rt, &format!("appx_sw_{}", core_appx_fam));
    assert!(!rt.state.selected_appx.contains(&core_appx_fam));

    assert!(
        failures.is_empty(),
        "Found {} UI regression failures ({} unique):\n{}",
        failures.len(),
        unique_issues.len(),
        unique_issues.values().cloned().collect::<Vec<_>>().join("\n")
    );
}

#[allow(dead_code)]
fn _keep_el_import_used() {
    let _ = el();
}

fn init_gpu_state() -> AppState {
    let mut s = build_populated_sandbox_state();
    s.navigate(ActiveView::GpuTuning);
    s
}

fn init_dashboard_state() -> AppState {
    let mut s = build_populated_sandbox_state();
    s.navigate(ActiveView::EasyDashboard);
    s
}

#[test]
fn snap_dashboard_view() {
    let spec = rune::AppSpec::new(
        "WinBlast0 Dashboard",
        (1150.0, 780.0),
        init_dashboard_state,
        ui::view,
    );
    let out_path = std::path::Path::new("assets/screenshots/dashboard.png");
    let res = rune::App::new(spec)
        .args([
            "--snap".to_string(),
            out_path.to_string_lossy().to_string(),
            "--size".to_string(),
            "1150x780".to_string(),
            // Capture the shipped default (see `src/main.rs`), so the themed scroll bars show.
            "--scroll-bars".to_string(),
            "always".to_string(),
        ])
        .try_run();
    println!("Snap result: {:?}", res);
    assert!(out_path.exists(), "Snapshot file should be created");
}

fn init_cpu_parking_state() -> AppState {
    let mut s = build_populated_sandbox_state();
    s.navigate(ActiveView::CoreParking);
    let mut cores = Vec::new();
    for i in 0..16 {
        cores.push(winblast0::hardware::parking::CoreStatus {
            core_index: i,
            is_parked: i >= 12,
            usage_pct: if i >= 12 { 0.0 } else { 18.0 + ((i * 5) % 40) as f32 },
            core_type_label: if i < 8 { "P-Core".to_string() } else { "E-Core".to_string() },
        });
    }
    s.core_parking = winblast0::hardware::parking::CoreParkingState {
        total_cores: 16,
        unparked_count: 12,
        parked_count: 4,
        min_cores_ac_pct: 100,
        min_cores_dc_pct: 50,
        max_cores_ac_pct: 100,
        max_cores_dc_pct: 100,
        is_fully_unparked: false,
        is_hybrid_architecture: true,
        cores,
        on_ac_power: true,
        pcore_min_ac_pct: Some(100),
        pcore_min_dc_pct: Some(50),
        pcore_max_ac_pct: Some(100),
        pcore_max_dc_pct: Some(100),
    };
    s
}

#[test]
fn snap_gpu_tuning_view() {
    let spec = rune::AppSpec::new(
        "WinBlast0 GPU Tuning",
        (1150.0, 780.0),
        init_gpu_state,
        ui::view,
    );
    let out_path = std::path::Path::new("assets/screenshots/gpu_tuning.png");
    let res = rune::App::new(spec)
        .args([
            "--snap".to_string(),
            out_path.to_string_lossy().to_string(),
            "--size".to_string(),
            "1150x780".to_string(),
            // Capture the shipped default (see `src/main.rs`), so the themed scroll bars show.
            "--scroll-bars".to_string(),
            "always".to_string(),
        ])
        .try_run();
    println!("Snap result: {:?}", res);
    assert!(out_path.exists(), "Snapshot file should be created");
}

fn init_matrix_state() -> AppState {
    let mut s = build_populated_sandbox_state();
    s.navigate(ActiveView::AdvancedMatrix);
    s
}

#[test]
fn snap_advanced_matrix_view() {
    let spec = rune::AppSpec::new(
        "WinBlast0 Advanced Matrix",
        (1150.0, 780.0),
        init_matrix_state,
        ui::view,
    );
    let out_path = std::path::Path::new("assets/screenshots/advanced_matrix.png");
    let res = rune::App::new(spec)
        .args([
            "--snap".to_string(),
            out_path.to_string_lossy().to_string(),
            "--size".to_string(),
            "1150x780".to_string(),
            // Capture the shipped default (see `src/main.rs`), so the themed scroll bars show.
            "--scroll-bars".to_string(),
            "always".to_string(),
        ])
        .try_run();
    println!("Snap result: {:?}", res);
    assert!(out_path.exists(), "Snapshot file should be created");
}

#[test]
fn snap_cpu_parking_view() {
    let spec = rune::AppSpec::new(
        "WinBlast0 CPU Parking",
        (1150.0, 780.0),
        init_cpu_parking_state,
        ui::view,
    );
    let out_path = std::path::Path::new("assets/screenshots/cpu_parking.png");
    let res = rune::App::new(spec)
        .args([
            "--snap".to_string(),
            out_path.to_string_lossy().to_string(),
            "--size".to_string(),
            "1150x780".to_string(),
            // Capture the shipped default (see `src/main.rs`), so the themed scroll bars show.
            "--scroll-bars".to_string(),
            "always".to_string(),
        ])
        .try_run();
    println!("Snap result: {:?}", res);
    assert!(out_path.exists(), "Snapshot file should be created");
}
