//! Easy Mode Dashboard in Rune — Pure telemetry control center, real-time hardware waveforms,
//! Before/After impact metrics, system architecture summary, and quick launcher navigation cards.

use rune::color::Color;
use rune::{col, el, row, text, El, ScrollSpec};

use crate::app::{ActiveView, AppState, BeforeAfterImpact};
use crate::core::i18n::TextKey;
use crate::scanner::{ScanResult, TweakState};
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::widgets::sparkline;
use crate::ui::LayoutTier;

/// Compute an overall health score (0–100) from scan results across all applicable tweaks.
fn health_score(results: &[ScanResult]) -> u32 {
    let eligible: Vec<&ScanResult> = results
        .iter()
        .filter(|r| r.current_state != TweakState::NotApplicable)
        .collect();

    if eligible.is_empty() {
        return 0;
    }
    let optimized = eligible
        .iter()
        .filter(|r| r.current_state == TweakState::AlreadyOptimized)
        .count();
    ((optimized as f64 / eligible.len() as f64) * 100.0).round() as u32
}

fn health_color(score: u32, p: &ThemePalette) -> Color {
    match score {
        0..=39 => p.danger,
        40..=69 => p.warning,
        _ => p.success,
    }
}

/// Circular progress ring badge visualizing health score with high-contrast animated indicator.
fn score_ring_badge(score: u32, color: Color, p: &ThemePalette, s: &AppState) -> El {
    row()
        .min_w(0.0)
        .shrink()
        .px(10.0)
        .py(5.0)
        .radius(p.card_radius.min(8.0))
        .fill(color.alpha(0.14))
        .border(p.border_width, color.alpha(0.50))
        .items_center()
        .gap(8.0)
        .child(
            el()
                .size(34.0, 34.0)
                .radius(17.0)
                .fill(color.alpha(0.20))
                .border(2.5, color)
                .motion(rune_motion::Spring::BOUNCY)
                .center()
                .child(
                    text(format!("{}%", score))
                        .font(rune::tok::type_::LABEL)
                        .color(color)
                        .shrink(),
                ),
        )
        .child(
            col()
                .min_w(0.0)
                .gap(1.0)
                .child(
                    text(s.tr(TextKey::SystemHealth))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(s.tr(TextKey::OptimizedLabel))
                        .font(rune::tok::type_::LABEL)
                        .color(color)
                        .shrink()
                        .wrap(),
                ),
        )
}

/// Card visualizing Before vs After optimization resource consumption impact cleanly across all widths.
fn before_after_card(impact: &BeforeAfterImpact, p: &ThemePalette, s: &AppState) -> El {
    let ram_diff_color = if impact.ram_saved_mb >= 0 { p.success } else { p.warning };
    let proc_diff_color = if impact.processes_saved >= 0 { p.success } else { p.warning };
    let timer_color = if impact.current_timer_ns < impact.baseline_timer_ns {
        p.success
    } else {
        p.text_secondary
    };

    let timer_baseline_ms = impact.baseline_timer_ns as f64 / 1_000_000.0;
    let timer_current_ms = impact.current_timer_ns as f64 / 1_000_000.0;

    col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .hover(p.bg_card_hover)
        .motion(p.motion)
        .gap(12.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gaps(8.0, 12.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(180.0)
                        .gap(2.0)
                        .child(
                            text(s.tr(TextKey::BeforeAfterImpact))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(s.tr(TextKey::BeforeAfterSub))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(action_button_guarded(
                    if s.baseline_reset_inflight {
                        s.tr(TextKey::BusyWorking)
                    } else {
                        s.tr(TextKey::ResetBaseline)
                    },
                    p.btn_secondary,
                    p.text_primary,
                    !s.baseline_reset_inflight,
                    p,
                    |s| s.reset_baseline(),
                )),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(10.0, 10.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(165.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(3.0)
                        .child(
                            text(s.tr(TextKey::RamUsage))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            row()
                                .flex_wrap()
                                .gap(6.0)
                                .items_center()
                                .child(
                                    text(format!("{:.1} GB", impact.current_ram_mb as f64 / 1024.0))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(format!(
                                        "({} {:.1} GB)",
                                        s.tr(TextKey::WasLabel),
                                        impact.baseline_ram_mb as f64 / 1024.0
                                    ))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            text(if impact.ram_saved_mb >= 0 {
                                format!("▼ {} MB {}", impact.ram_saved_mb, s.tr(TextKey::FreedRam))
                            } else {
                                format!(
                                    "▲ {} MB {}",
                                    impact.ram_saved_mb.saturating_abs(),
                                    s.tr(TextKey::ActiveRam)
                                )
                            })
                            .font(rune::tok::type_::LABEL)
                            .color(ram_diff_color)
                            .shrink()
                            .wrap(),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(165.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(3.0)
                        .child(
                            text(s.tr(TextKey::BackgroundProcesses))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            row()
                                .flex_wrap()
                                .gap(6.0)
                                .items_center()
                                .child(
                                    text(format!("{}", impact.current_processes))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(format!(
                                        "({} {})",
                                        s.tr(TextKey::WasLabel),
                                        impact.baseline_processes
                                    ))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            text(if impact.processes_saved >= 0 {
                                format!("▼ {} {}", impact.processes_saved, s.tr(TextKey::FewerProcesses))
                            } else {
                                format!(
                                    "▲ {} {}",
                                    impact.processes_saved.saturating_abs(),
                                    s.tr(TextKey::BackgroundProcesses)
                                )
                            })
                            .font(rune::tok::type_::LABEL)
                            .color(proc_diff_color)
                            .shrink()
                            .wrap(),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(165.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(3.0)
                        .child(
                            text(s.tr(TextKey::TimerResolution))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(format!("{:.3} ms", timer_current_ms))
                                .font(rune::tok::type_::TITLE)
                                .color(timer_color),
                        )
                        .child(
                            text(format!("{}: {:.3} ms", s.tr(TextKey::BeforeTest), timer_baseline_ms))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(165.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(3.0)
                        .child(
                            text(s.tr(TextKey::ActiveOptimizations))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            row()
                                .flex_wrap()
                                .gap(6.0)
                                .items_center()
                                .child(
                                    text(format!("{}", impact.current_optimized_count))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.success),
                                )
                                .child(
                                    text(format!(
                                        "({} {})",
                                        s.tr(TextKey::WasLabel),
                                        impact.baseline_optimized_count
                                    ))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            text(if impact.tweaks_gained > 0 {
                                format!("▲ +{} {}", impact.tweaks_gained, s.tr(TextKey::TweaksGained))
                            } else {
                                s.tr(TextKey::BaselineProfileActive).to_string()
                            })
                            .font(rune::tok::type_::LABEL)
                            .color(if impact.tweaks_gained > 0 {
                                p.success
                            } else {
                                p.text_muted
                            })
                            .shrink()
                            .wrap(),
                        ),
                ),
        )
}

/// Real-time 1-second Task Manager style waveform sparklines.
fn live_telemetry_card(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let Some(ref m) = s.realtime_metrics else {
        return col();
    };
    let min_card_w = if tier == LayoutTier::Compact { 240.0 } else { 300.0 };

    row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .gaps(12.0, 12.0)
        .child(
            sparkline::sparkline_with_palette(
                s.tr(TextKey::CpuSparklineTitle),
                &format!("{:.1}% ({} {})", m.cpu_usage, m.thread_count, s.tr(TextKey::ThreadsLabel)),
                &s.cpu_sparkline,
                p.sparkline_cpu,
                42.0,
                30,
                p,
            )
            .grow(1.0)
            .min_w(min_card_w),
        )
        .child(
            sparkline::sparkline_with_palette(
                s.tr(TextKey::RamSparklineTitle),
                &format!(
                    "{:.1} / {:.1} GB ({:.1}%)",
                    m.used_ram_mb as f64 / 1024.0,
                    m.total_ram_mb as f64 / 1024.0,
                    m.ram_percentage
                ),
                &s.ram_sparkline,
                p.sparkline_ram,
                42.0,
                30,
                p,
            )
            .grow(1.0)
            .min_w(min_card_w),
        )
}

/// System Architecture and Hardware Specs Overview Card.
fn hardware_overview_card(s: &AppState, p: &ThemePalette) -> El {
    let cpu_name = s.cpu_info.as_ref().map(|c| c.model_name.as_str()).unwrap_or("Intel / AMD Processor");
    let cpu_topology = s.cpu_info.as_ref().map(|c| {
        format!("{} Physical / {} Logical Threads", c.physical_cores, c.logical_cores)
    }).unwrap_or_else(|| "Multi-Core Topology".to_string());

    let primary_gpu = s.gpus.first();
    let gpu_name = primary_gpu.map(|g| g.name.as_str()).unwrap_or("Discrete / Integrated GPU");
    let gpu_driver = primary_gpu.map(|g| {
        if g.dedicated_vram_mb > 0 {
            format!("{:.1} GB Dedicated VRAM", g.dedicated_vram_mb as f64 / 1024.0)
        } else {
            "WHQL Display Driver".to_string()
        }
    }).unwrap_or_else(|| "WHQL Driver".to_string());

    let ram_text = s.platform_security.as_ref().map(|ps| ps.ram_summary.as_str()).unwrap_or("DDR4/DDR5 Memory");
    let motherboard = s.platform_security.as_ref().map(|ps| ps.motherboard.as_str()).unwrap_or("Standard PC Motherboard");
    let uptime = s.platform_security.as_ref().map(|ps| format!("{:.1} Hours Uptime", ps.uptime_hours)).unwrap_or_else(|| "Active Session".to_string());

    col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(10.0)
        .child(
            row()
                .gap(6.0)
                .center()
                .child(text("💻").font(rune::tok::type_::LABEL))
                .child(
                    text(s.tr(TextKey::DashboardHwSummaryTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(10.0, 10.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(220.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(2.0)
                        .child(text(s.tr(TextKey::DashboardHwCpu)).font(rune::tok::type_::LABEL).color(p.accent))
                        .child(text(cpu_name).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(cpu_topology).font(rune::tok::type_::BODY).color(p.text_muted)),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(220.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(2.0)
                        .child(text(s.tr(TextKey::DashboardHwGpu)).font(rune::tok::type_::LABEL).color(p.accent))
                        .child(text(gpu_name).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(gpu_driver).font(rune::tok::type_::BODY).color(p.text_muted)),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(220.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(2.0)
                        .child(text(s.tr(TextKey::DashboardHwRam)).font(rune::tok::type_::LABEL).color(p.accent))
                        .child(text(ram_text).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(motherboard).font(rune::tok::type_::BODY).color(p.text_muted)),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(220.0)
                        .padding(10.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(2.0)
                        .child(text(s.tr(TextKey::DashboardHwPlatform)).font(rune::tok::type_::LABEL).color(p.accent))
                        .child(text(uptime).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(format!("{} {}", s.scan_results.len(), s.tr(TextKey::DashboardHwOptimizationRules))).font(rune::tok::type_::BODY).color(p.text_muted)),
                ),
        )
}

/// Quick navigation launcher cards and 1-click optimization presets row.
fn quick_actions_card(s: &AppState, p: &ThemePalette) -> El {
    col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .hover(p.bg_card_hover)
        .motion(p.motion)
        .gap(12.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gaps(8.0, 10.0)
                .child(
                    row()
                        .gap(6.0)
                        .center()
                        .child(text("⚡").font(rune::tok::type_::LABEL))
                        .child(
                            text(s.tr(TextKey::QuickActionsTitle))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        ),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(6.0, 6.0)
                        .child(action_button(
                            s.tr(TextKey::GamingPreset),
                            p.accent,
                            Color::WHITE,
                            p,
                            |s| s.apply_preset(crate::app::PresetKind::Gaming),
                        ))
                        .child(action_button(
                            s.tr(TextKey::ProAudioPreset),
                            p.btn_secondary,
                            p.text_primary,
                            p,
                            |s| s.apply_preset(crate::app::PresetKind::ProAudio),
                        ))
                        .child(action_button(
                            s.tr(TextKey::LaptopBatteryPreset),
                            p.btn_secondary,
                            p.text_primary,
                            p,
                            |s| s.apply_preset(crate::app::PresetKind::LaptopBattery),
                        )),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(10.0, 10.0)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text(s.tr(TextKey::DashboardQaFpsTitle)).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(s.tr(TextKey::DashboardQaFpsDesc)).font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button(s.tr(TextKey::DashboardQaFpsBtn), p.accent, Color::WHITE, p, |s| {
                            s.navigate(ActiveView::AdvancedMatrix);
                        })),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text(s.tr(TextKey::DashboardQaCleanTitle)).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(s.tr(TextKey::DashboardQaCleanDesc)).font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button(s.tr(TextKey::DashboardQaCleanBtn), p.btn_secondary, p.text_primary, p, |s| {
                            s.navigate(ActiveView::CleanAndApps);
                        })),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text(s.tr(TextKey::DashboardQaCpuTitle)).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(s.tr(TextKey::DashboardQaCpuDesc)).font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button(s.tr(TextKey::DashboardQaCpuBtn), p.btn_secondary, p.text_primary, p, |s| {
                            s.navigate(ActiveView::CoreParking);
                        })),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text(s.tr(TextKey::DashboardQaDriverTitle)).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(s.tr(TextKey::DashboardQaDriverDesc)).font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button(s.tr(TextKey::DashboardQaDriverBtn), p.btn_secondary, p.text_primary, p, |s| {
                            s.navigate(ActiveView::DriverBios);
                        })),
                )
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text(s.tr(TextKey::DashboardQaFaultTitle)).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(s.tr(TextKey::DashboardQaFaultDesc)).font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button(s.tr(TextKey::DashboardQaFaultBtn), p.btn_secondary, p.text_primary, p, |s| {
                            s.navigate(ActiveView::FaultDiagnostics);
                        })),
                ),
        )
}

/// Administrator Privileges Warning Banner with 1-click relaunch button.
fn admin_warning_banner(s: &AppState, p: &ThemePalette) -> El {
    if s.is_admin_elevated() {
        return el();
    }
    col()
        .w_full()
        .min_w(0.0)
        .padding(12.0)
        .radius(p.card_radius)
        .fill(p.warning.alpha(0.12))
        .border(1.5, p.warning)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gaps(8.0, 10.0)
                .child(
                    row()
                        .grow(1.0)
                        .min_w(200.0)
                        .gap(10.0)
                        .items_center()
                        .child(text("⚠️").font(rune::tok::type_::TITLE))
                        .child(
                            col()
                                .grow(1.0)
                                .gap(2.0)
                                .child(
                                    text(s.tr(TextKey::AdminRequiredTitle))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.warning)
                                        .shrink()
                                        .wrap(),
                                )
                                .child(
                                    text(s.tr(TextKey::AdminRequiredDesc))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_primary)
                                        .shrink()
                                        .wrap(),
                                ),
                        ),
                )
                .child(
                    action_button(
                        s.tr(TextKey::RelaunchAdminBtn),
                        p.warning,
                        Color::BLACK,
                        p,
                        |s| s.relaunch_as_admin(),
                    ),
                ),
        )
}

/// GitHub Releases Self-Updater card.
fn updater_status_card(s: &AppState, p: &ThemePalette) -> El {
    use crate::app::UpdateStatus;
    match &s.update_status {
        UpdateStatus::Idle => {
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .gaps(8.0, 8.0)
                .child(
                    row()
                        .gap(6.0)
                        .items_center()
                        .child(text("📦").font(rune::tok::type_::LABEL))
                        .child(
                            text(concat!("WinBlast0 v", env!("CARGO_PKG_VERSION"), " • Pure-Rust Rune Engine"))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        ),
                )
                .child(
                    action_button(
                        s.tr(TextKey::UpdateCheckBtn),
                        p.btn_secondary,
                        p.text_primary,
                        p,
                        |s| s.check_for_updates(),
                    ),
                )
        }
        UpdateStatus::Checking => {
            row()
                .w_full()
                .min_w(0.0)
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.accent.alpha(0.10))
                .border(1.0, p.accent)
                .child(
                    row()
                        .gap(6.0)
                        .items_center()
                        .child(text("⟳").font(rune::tok::type_::LABEL).color(p.accent))
                        .child(
                            text(s.tr(TextKey::UpdateChecking))
                                .font(rune::tok::type_::LABEL)
                                .color(p.accent),
                        ),
                )
        }
        UpdateStatus::Available(ref info) => {
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.success.alpha(0.12))
                .border(1.5, p.success)
                .gaps(8.0, 8.0)
                .child(
                    row()
                        .gap(8.0)
                        .items_center()
                        .child(text("🚀").font(rune::tok::type_::TITLE))
                        .child(
                            col()
                                .gap(2.0)
                                .child(
                                    text(format!("{}: {}", s.tr(TextKey::UpdateAvailableBadge), info.tag_name))
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.success),
                                )
                                .child(
                                    text(format!("{} • {:.1} MB", info.name, info.download_size_mb.unwrap_or(12.5)))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        ),
                )
                .child(
                    action_button(
                        s.tr(TextKey::UpdateDownloadInstallBtn),
                        p.success,
                        Color::BLACK,
                        p,
                        |s| s.download_and_install_update(),
                    ),
                )
        }
        UpdateStatus::Downloading { progress_pct } => {
            row()
                .w_full()
                .min_w(0.0)
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.accent.alpha(0.12))
                .border(1.0, p.accent)
                .child(
                    row()
                        .gap(6.0)
                        .items_center()
                        .child(text("📥").font(rune::tok::type_::LABEL))
                        .child(
                            text(format!("{} ({:.0}%)", s.tr(TextKey::UpdateDownloading), progress_pct))
                                .font(rune::tok::type_::LABEL)
                                .color(p.accent),
                        ),
                )
        }
        UpdateStatus::ReadyToRestart { .. } => {
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.accent.alpha(0.15))
                .border(1.5, p.accent)
                .gaps(8.0, 8.0)
                .child(
                    row()
                        .gap(8.0)
                        .items_center()
                        .child(text("✔").font(rune::tok::type_::TITLE).color(p.accent))
                        .child(
                            text("Update Ready to Install — Ready to restart.")
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        ),
                )
                .child(
                    action_button(
                        s.tr(TextKey::UpdateReadyRestartBtn),
                        p.accent,
                        Color::WHITE,
                        p,
                        |s| s.apply_and_restart(),
                    ),
                )
        }
        UpdateStatus::UpToDate => {
            row()
                .w_full()
                .min_w(0.0)
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .child(
                    row()
                        .gap(6.0)
                        .items_center()
                        .child(text("✔").font(rune::tok::type_::LABEL).color(p.success))
                        .child(
                            text(s.tr(TextKey::UpdateUpToDate))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        ),
                )
                .child(
                    action_button(
                        s.tr(TextKey::UpdateCheckBtn),
                        p.btn_secondary,
                        p.text_primary,
                        p,
                        |s| s.check_for_updates(),
                    ),
                )
        }
        UpdateStatus::Failed(ref err) => {
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .padding(10.0)
                .radius(6.0)
                .fill(p.danger.alpha(0.12))
                .border(1.0, p.danger)
                .gaps(8.0, 8.0)
                .child(
                    text(format!("{}: {}", s.tr(TextKey::UpdateFailedMsg), err))
                        .font(rune::tok::type_::BODY)
                        .color(p.danger)
                        .wrap(),
                )
                .child(
                    action_button(
                        s.tr(TextKey::UpdateCheckBtn),
                        p.btn_secondary,
                        p.text_primary,
                        p,
                        |s| s.check_for_updates(),
                    ),
                )
        }
    }
}

/// System optimization categories breakdown with visual mini progress indicators.
fn optimization_categories_summary(s: &AppState, p: &ThemePalette) -> El {
    let mut gaming_opt = 0;
    let mut gaming_total = 0;
    let mut svc_opt = 0;
    let mut svc_total = 0;
    let mut priv_opt = 0;
    let mut priv_total = 0;
    let mut hw_opt = 0;
    let mut hw_total = 0;

    for r in &s.scan_results {
        if r.current_state == TweakState::NotApplicable {
            continue;
        }
        let is_opt = r.current_state == TweakState::AlreadyOptimized;
        let cat = r.category.to_lowercase();
        if cat.contains("game") || cat.contains("gpu") || cat.contains("input") {
            gaming_total += 1;
            if is_opt { gaming_opt += 1; }
        } else if cat.contains("service") || cat.contains("startup") || cat.contains("task") {
            svc_total += 1;
            if is_opt { svc_opt += 1; }
        } else if cat.contains("privacy") || cat.contains("telemetry") || cat.contains("security") {
            priv_total += 1;
            if is_opt { priv_opt += 1; }
        } else {
            hw_total += 1;
            if is_opt { hw_opt += 1; }
        }
    }

    let cat_col = |label: &str, opt: usize, tot: usize, bar_color: Color| {
        let pct = if tot > 0 { (opt as f32 / tot as f32).clamp(0.0, 1.0) } else { 0.0 };
        col()
            .grow(1.0)
            .min_w(160.0)
            .padding(10.0)
            .radius(6.0)
            .fill(p.bg_primary)
            .border(1.0, p.border)
            .gap(4.0)
            .child(
                row()
                    .justify_between()
                    .child(text(label).font(rune::tok::type_::LABEL).color(p.text_primary))
                    .child(text(format!("{}/{}", opt, tot)).font(rune::tok::type_::LABEL).color(p.text_muted)),
            )
            .child(
                el()
                    .w_full()
                    .h(5.0)
                    .radius(2.5)
                    .fill(p.bg_card)
                    .child(
                        el()
                            .w(150.0 * pct)
                            .h(5.0)
                            .radius(2.5)
                            .fill(bar_color),
                    ),
            )
    };

    col()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .gap(10.0)
        .child(
            row()
                .gap(6.0)
                .items_center()
                .child(text("📊").font(rune::tok::type_::LABEL))
                .child(
                    text("System Optimization Categories Breakdown")
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(10.0, 10.0)
                .child(cat_col("🎮 Gaming & GPU", gaming_opt, gaming_total, p.accent))
                .child(cat_col("⚙ Services & Tasks", svc_opt, svc_total, p.success))
                .child(cat_col("🛡 Privacy & Telemetry", priv_opt, priv_total, p.warning))
                .child(cat_col("🖥 Hardware & Latency", hw_opt, hw_total, p.text_secondary)),
        )
}

/// Renders the pure Dashboard view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let score = health_score(&s.scan_results);
    let color = health_color(score, p);

    let header_card = if tier == LayoutTier::Compact {
        col()
            .w_full()
            .min_w(0.0)
            .padding(p.card_padding)
            .radius(p.card_radius)
            .fill(p.bg_card)
            .border(p.border_width, p.border)
            .hover(p.bg_card_hover)
            .motion(p.motion)
            .gap(10.0)
            .child(
                col()
                    .w_full()
                    .min_w(0.0)
                    .gap(4.0)
                    .child(
                        text(s.tr(TextKey::NavSystemPanel))
                            .font(rune::tok::type_::TITLE)
                            .color(p.text_primary)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(s.tr(TextKey::DashboardSubtitle))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    ),
            )
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .gap(8.0)
                    .flex_wrap()
                    .items_center()
                    .child(score_ring_badge(score, color, p, s))
                    .child(action_button_guarded(
                        if s.is_scanning {
                            s.tr(TextKey::Scanning)
                        } else {
                            s.tr(TextKey::ScanSystem)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.is_scanning,
                        p,
                        |s| s.start_scan(),
                    )),
            )
    } else {
        row()
            .w_full()
            .min_w(0.0)
            .padding(p.card_padding)
            .radius(p.card_radius)
            .fill(p.bg_card)
            .border(p.border_width, p.border)
            .hover(p.bg_card_hover)
            .motion(p.motion)
            .flex_wrap()
            .justify_between()
            .items_center()
            .gaps(10.0, 16.0)
            .child(
                col()
                    .grow(1.0)
                    .min_w(180.0)
                    .gap(4.0)
                    .child(
                        text(s.tr(TextKey::NavSystemPanel))
                            .font(rune::tok::type_::TITLE)
                            .color(p.text_primary)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(s.tr(TextKey::DashboardSubtitle))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    ),
            )
            .child(
                row()
                    .min_w(0.0)
                    .gap(8.0)
                    .flex_wrap()
                    .center()
                    .child(score_ring_badge(score, color, p, s))
                    .child(action_button_guarded(
                        if s.is_scanning {
                            s.tr(TextKey::Scanning)
                        } else {
                            s.tr(TextKey::ScanSystem)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.is_scanning,
                        p,
                        |s| s.start_scan(),
                    )),
            )
    };

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    let mut main_col = col()
        .key("easy_dashboard_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .pr(14.0)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(admin_warning_banner(s, p))
        .child(updater_status_card(s, p))
        .child(header_card)
        .child(live_telemetry_card(s, p, tier))
        .child(optimization_categories_summary(s, p));

    if let Some(impact) = s.calculate_before_after_impact() {
        main_col = main_col.child(before_after_card(&impact, p, s));
    }

    main_col
        .child(hardware_overview_card(s, p))
        .child(quick_actions_card(s, p))
        .winblast_scroll(p)
}
