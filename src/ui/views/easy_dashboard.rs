//! Easy Mode Dashboard in Rune — Pure telemetry control center, real-time hardware waveforms,
//! Before/After impact metrics, system architecture summary, and quick launcher navigation cards.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

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
                    text("Sistem Özellikleri & Donanım Özeti")
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
                        .child(text("İşlemci (CPU)").font(rune::tok::type_::LABEL).color(p.accent))
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
                        .child(text("Ekran Kartı (GPU)").font(rune::tok::type_::LABEL).color(p.accent))
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
                        .child(text("Bellek (RAM)").font(rune::tok::type_::LABEL).color(p.accent))
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
                        .child(text("Platform & Oturum").font(rune::tok::type_::LABEL).color(p.accent))
                        .child(text(uptime).font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text(format!("{} Optimizasyon Kuralı", s.scan_results.len())).font(rune::tok::type_::BODY).color(p.text_muted)),
                ),
        )
}

/// Quick navigation launcher cards to direct sections.
fn quick_actions_card(_s: &AppState, p: &ThemePalette) -> El {
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
                .child(text("⚡").font(rune::tok::type_::LABEL))
                .child(
                    text("Hızlı Modül ve Optimizasyon Başlatıcı")
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
                        .min_w(200.0)
                        .padding(12.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .gap(6.0)
                        .child(text("⚡ FPS Optimizasyonu").font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text("Tüm sistem ve oyun tweak matrisi, 1-tık profiller ve canlı anahtarlar.").font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button("Matrise Git", p.accent, Color::WHITE, p, |s| {
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
                        .child(text("🗑️ Temizlik & Başlangıç").font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text("Disk temizleyici, AppX bloatware silici ve başlangıç programları.").font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button("Temizliğe Git", p.btn_secondary, p.text_primary, p, |s| {
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
                        .child(text("🔲 İşlemci Analizi").font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text("Çekirdek parkını kaldır, mikrosaniyeler seviyesinde gecikmeyi azalt.").font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button("İşlemciye Git", p.btn_secondary, p.text_primary, p, |s| {
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
                        .child(text("💾 Sürücü ve BIOS").font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text("Donanım SMBIOS bilgisi ve çevrimiçi sürücü güncelleme motoru.").font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button("Sürücülere Git", p.btn_secondary, p.text_primary, p, |s| {
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
                        .child(text("🛡️ Arıza Teşhisi").font(rune::tok::type_::TITLE).color(p.text_primary))
                        .child(text("BSOD minidump tarayıcı ve bekleyen CBS yeniden başlatma analizi.").font(rune::tok::type_::BODY).color(p.text_muted))
                        .child(action_button("Arıza Teşhisi Aç", p.btn_secondary, p.text_primary, p, |s| {
                            s.navigate(ActiveView::FaultDiagnostics);
                        })),
                ),
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
                        text("Canlı telemetri, çekirdek dalga formları ve sistem kaynak verimliliği göstergesi.")
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
                    .child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .px(10.0)
                            .py(6.0)
                            .radius(p.badge_radius)
                            .fill(color.alpha(0.18))
                            .border(p.border_width, color.alpha(0.4))
                            .center()
                            .child(
                                text(format!(
                                    "{}: {}% {}",
                                    s.tr(TextKey::SystemHealth),
                                    score,
                                    s.tr(TextKey::OptimizedLabel)
                                ))
                                .font(rune::tok::type_::LABEL)
                                .color(color)
                                .shrink()
                                .wrap(),
                            ),
                    )
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
                        text("Canlı telemetri, çekirdek dalga formları ve sistem kaynak verimliliği göstergesi.")
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
                    .child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .px(12.0)
                            .py(6.0)
                            .radius(p.badge_radius)
                            .fill(color.alpha(0.18))
                            .border(p.border_width, color.alpha(0.4))
                            .center()
                            .child(
                                text(format!(
                                    "{}: {}% {}",
                                    s.tr(TextKey::SystemHealth),
                                    score,
                                    s.tr(TextKey::OptimizedLabel)
                                ))
                                .font(rune::tok::type_::LABEL)
                                .color(color)
                                .shrink()
                                .wrap(),
                            ),
                    )
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
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header_card)
        .child(live_telemetry_card(s, p, tier));

    if let Some(impact) = s.calculate_before_after_impact() {
        main_col = main_col.child(before_after_card(&impact, p, s));
    }

    main_col
        .child(hardware_overview_card(s, p))
        .child(quick_actions_card(s, p))
        .winblast_scroll(p)
}
