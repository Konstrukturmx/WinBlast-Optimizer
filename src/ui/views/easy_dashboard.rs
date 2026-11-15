//! Easy Mode Dashboard in Rune — one-click presets, health gauge, spring switches,
//! live Task Manager waveforms, and Before/After impact metrics responsive across all resolutions.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::{AppState, BeforeAfterImpact, PresetKind};
use crate::core::i18n::TextKey;
use crate::scanner::{ScanResult, TweakState};
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::{
    action_button, action_button_guarded, pill_badge, switch_toggle, tweak_state_meta,
};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::widgets::{risk_badge, sparkline};
use crate::ui::LayoutTier;

/// Compute a health score (0–100) from scan results for Easy Mode eligible tweaks.
fn health_score(results: &[ScanResult]) -> u32 {
    let eligible: Vec<&ScanResult> = results
        .iter()
        .filter(|r| r.easy_mode_included && r.current_state != TweakState::NotApplicable)
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

/// Real-time 1-second Task Manager style waveform sparklines wrapping seamlessly on compact screens.
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
                38.0,
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
                38.0,
                30,
                p,
            )
            .grow(1.0)
            .min_w(min_card_w),
        )
}

/// Renders the responsive Easy Mode dashboard inside a single outer scroll viewport.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let score = health_score(&s.scan_results);
    let color = health_color(score, p);

    let header_card = row()
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
                .min_w(200.0)
                .gap(4.0)
                .child(
                    text(s.tr(TextKey::EasyMode))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(s.tr(TextKey::SystemHealthSub))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .px(14.0)
                .py(8.0)
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
                    .font(rune::tok::type_::TITLE)
                    .color(color)
                    .shrink()
                    .wrap(),
                ),
        );

    let presets_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(8.0, 10.0)
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button(
                    s.tr(TextKey::GamingPreset),
                    p.accent,
                    Color::WHITE,
                    p,
                    |s| s.apply_preset(PresetKind::Gaming),
                ))
                .child(action_button(
                    s.tr(TextKey::ProAudioPreset),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.apply_preset(PresetKind::ProAudio),
                ))
                .child(action_button(
                    s.tr(TextKey::LaptopBatteryPreset),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.apply_preset(PresetKind::LaptopBattery),
                )),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .flex_wrap()
                .gaps(8.0, 8.0)
                .child(action_button_guarded(
                    if s.is_scanning {
                        s.tr(TextKey::Scanning)
                    } else {
                        s.tr(TextKey::ScanSystem)
                    },
                    p.btn_secondary,
                    p.text_primary,
                    !s.is_scanning,
                    p,
                    |s| s.start_scan(),
                ))
                .child(action_button_guarded(
                    if s.is_applying {
                        s.tr(TextKey::BusyWorking).to_string()
                    } else if s.selected_tweaks.is_empty() {
                        s.tr(TextKey::ApplySelected).to_string()
                    } else {
                        format!("{} ({})", s.tr(TextKey::ApplySelected), s.selected_tweaks.len())
                    },
                    p.success,
                    Color::WHITE,
                    !s.is_applying && !s.is_rolling_back,
                    p,
                    |s| s.apply_selected(),
                ))
                .child(action_button(
                    s.tr(TextKey::PreviewDiff),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| {
                        if s.show_diff_preview && s.inspected_tweak.is_none() {
                            s.show_diff_preview = false;
                        } else {
                            s.open_diff_preview();
                        }
                    },
                )),
        );

    let mut tweak_list = col().w_full().min_w(0.0).gap(6.0);
    let easy_tweaks: Vec<&ScanResult> = s
        .scan_results
        .iter()
        .filter(|r| r.easy_mode_included)
        .collect();

    if easy_tweaks.is_empty() {
        tweak_list = tweak_list.child(
            row()
                .w_full()
                .padding(p.card_padding)
                .radius(p.card_radius)
                .fill(p.bg_card)
                .border(p.border_width, p.border)
                .center()
                .child(
                    text(if s.is_scanning {
                        s.tr(TextKey::ScanningEvaluating)
                    } else {
                        s.tr(TextKey::NoEligibleTweaks)
                    })
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
                ),
        );
    } else {
        for sr in easy_tweaks {
            let is_active = s.is_switch_active(&sr.tweak_id);
            let in_flight = s.in_flight_tweaks.contains(&sr.tweak_id);
            let (state_text, state_color) = tweak_state_meta(&sr.current_state, s);
            let loc_title = s.tweak_title(&sr.tweak_id, &sr.title);
            let loc_desc = s.tweak_description(&sr.tweak_id, &sr.description);
            let loc_detail = s.scan_detail_text(sr);

            let switch = if sr.current_state == TweakState::NotApplicable {
                pill_badge(s.tr(TextKey::StatusNotApplicable), p.text_muted, p)
            } else {
                switch_toggle(is_active, in_flight, sr.tweak_id.clone(), p)
            };
            let badge = risk_badge::view_localized(&sr.risk, p, s);

            let item_card = row()
                .w_full()
                .min_w(0.0)
                .padding(p.card_padding.min(10.0))
                .radius(p.card_radius.min(6.0))
                .fill(if is_active { p.bg_card_hover } else { p.bg_card })
                .border(
                    p.border_width,
                    if is_active { p.accent.alpha(0.35) } else { p.border },
                )
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .flex_wrap()
                .items_center()
                .gaps(8.0, 12.0)
                .child(switch)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(180.0)
                        .gap(2.0)
                        .child(
                            text(loc_title)
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        )
                        .child(
                            text(loc_desc)
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        ),
                )
                .child(badge)
                .child(
                    col()
                        .shrink()
                        .min_w(110.0)
                        .gap(2.0)
                        .child(
                            text(state_text)
                                .font(rune::tok::type_::LABEL)
                                .color(state_color)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(&loc_detail)
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        ),
                );

            tweak_list = tweak_list.child(item_card);
        }
    }

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
        .child(presets_card)
        .child(tweak_list)
        .winblast_scroll(p)
}
