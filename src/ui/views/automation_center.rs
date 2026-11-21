//! Automation Center View in Rune.
//!
//! Provides 1-click system auto-tuning, background health assessment,
//! scheduled maintenance task management, and memory/cache trimming.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;
use crate::scanner::TweakState;
use crate::tools::automation::{create_auto_tune_plan, AutoTuneProfile};
use crate::ui::widgets::button::{action_button, action_button_guarded, pill_badge};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;

    let active_profile = s.selected_auto_tune_profile;
    let plan = create_auto_tune_plan(&s.tweaks, active_profile);

    let mut outer = col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .pr(16.0)
        .gap(12.0)
        // Header
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text(s.tr(TextKey::AutomationTitle))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text(s.tr(TextKey::AutomationSub))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .flex_wrap()
                        .gaps(8.0, 8.0)
                        .child(
                            action_button_guarded(
                                if s.is_running_automation {
                                    "Running..."
                                } else {
                                    s.tr(TextKey::AutomationOneClick)
                                },
                                p.accent,
                                Color::WHITE,
                                !s.is_running_automation,
                                p,
                                |s| {
                                    let prof = s.selected_auto_tune_profile;
                                    s.execute_auto_tune_profile(prof);
                                },
                            )
                            .motion(style.motion())
                            .radius(style.btn_radius()),
                        )
                        .child(
                            action_button_guarded(
                                "Trim Memory & Cache",
                                p.btn_secondary,
                                p.text_primary,
                                !s.is_running_automation,
                                p,
                                |s| s.execute_memory_trim(),
                            )
                            .motion(style.motion())
                            .radius(style.btn_radius()),
                        ),
                ),
        );

    // If an AutoTuneResult exists, render the results summary card
    if let Some(ref res) = s.auto_tune_result {
        outer = outer.child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.success.alpha(0.4))
                .gap(6.0)
                .child(
                    row()
                        .w_full()
                        .justify_between()
                        .items_center()
                        .child(
                            row()
                                .gap(6.0)
                                .items_center()
                                .child(text("✅").font(rune::tok::type_::LABEL))
                                .child(
                                    text("Auto-Tune Execution Results")
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(pill_badge(format!("{} ms", res.duration_ms), p.accent, p)),
                )
                .child(
                    text(&res.message)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary)
                        .wrap(),
                )
                .child(
                    row()
                        .gap(12.0)
                        .flex_wrap()
                        .child(pill_badge(
                            format!("Applied: {}", res.tweaks_applied),
                            p.success,
                            p,
                        ))
                        .child(pill_badge(
                            format!("Failed/Skipped: {}", res.tweaks_failed),
                            if res.tweaks_failed > 0 {
                                p.warning
                            } else {
                                p.text_muted
                            },
                            p,
                        )),
                ),
        );
    }

    // If a MemoryTrimResult exists, render memory trim summary card
    if let Some(ref trim) = s.last_memory_trim_result {
        outer = outer.child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .justify_between()
                .items_center()
                .child(
                    row()
                        .gap(6.0)
                        .items_center()
                        .child(text("🧹").font(rune::tok::type_::LABEL))
                        .child(
                            text("Memory & Working Set Cache Trimmed")
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary),
                        ),
                )
                .child(
                    text(format!(
                        "Freed {:.1} MB • {} Processes Trimmed",
                        trim.freed_mb_estimated, trim.processes_trimmed
                    ))
                    .font(rune::tok::type_::BODY)
                    .color(p.accent),
                ),
        );
    }

    // Grid: Profile Selection & Health Overview
    outer = outer.child(
        row()
            .w_full()
            .min_w(0.0)
            .flex_wrap()
            .gaps(12.0, 12.0)
            .child(
                // Auto-Tune Profile Card
                col()
                    .grow(1.0)
                    .min_w(280.0)
                    .padding(style.card_padding())
                    .radius(style.card_radius())
                    .fill(p.card_fill_styled(style))
                    .border(style.border_width(), p.border_styled(style))
                    .gap(8.0)
                    .child(
                        row()
                            .gap(6.0)
                            .center()
                            .child(text("⚡").font(rune::tok::type_::LABEL))
                            .child(
                                text(s.tr_v(ViewTextKey::AutoCenterProfilesTitle))
                                    .font(rune::tok::type_::TITLE)
                                    .color(p.text_primary),
                            ),
                    )
                    .child(
                        text(s.tr_v(ViewTextKey::AutoCenterProfilesSubtitle))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted),
                    )
                    .child(
                        row()
                            .gap(6.0)
                            .flex_wrap()
                            .child(
                                action_button(
                                    "🎮 Ultra Gaming",
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::GamingLowLatency
                                    {
                                        p.accent
                                    } else {
                                        p.bg_card
                                    },
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::GamingLowLatency
                                    {
                                        Color::WHITE
                                    } else {
                                        p.text_secondary
                                    },
                                    p,
                                    |s| {
                                        s.selected_auto_tune_profile =
                                            AutoTuneProfile::GamingLowLatency
                                    },
                                )
                                .radius(style.badge_radius()),
                            )
                            .child(
                                action_button(
                                    "🎵 Pro Audio / DAW",
                                    if s.selected_auto_tune_profile == AutoTuneProfile::ProAudioDpc
                                    {
                                        p.accent
                                    } else {
                                        p.bg_card
                                    },
                                    if s.selected_auto_tune_profile == AutoTuneProfile::ProAudioDpc
                                    {
                                        Color::WHITE
                                    } else {
                                        p.text_secondary
                                    },
                                    p,
                                    |s| s.selected_auto_tune_profile = AutoTuneProfile::ProAudioDpc,
                                )
                                .radius(style.badge_radius()),
                            )
                            .child(
                                action_button(
                                    "🛡 Privacy & Workstation",
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::WorkstationStability
                                    {
                                        p.accent
                                    } else {
                                        p.bg_card
                                    },
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::WorkstationStability
                                    {
                                        Color::WHITE
                                    } else {
                                        p.text_secondary
                                    },
                                    p,
                                    |s| {
                                        s.selected_auto_tune_profile =
                                            AutoTuneProfile::WorkstationStability
                                    },
                                )
                                .radius(style.badge_radius()),
                            )
                            .child(
                                action_button(
                                    "🔋 Laptop Battery",
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::LaptopBatterySaving
                                    {
                                        p.accent
                                    } else {
                                        p.bg_card
                                    },
                                    if s.selected_auto_tune_profile
                                        == AutoTuneProfile::LaptopBatterySaving
                                    {
                                        Color::WHITE
                                    } else {
                                        p.text_secondary
                                    },
                                    p,
                                    |s| {
                                        s.selected_auto_tune_profile =
                                            AutoTuneProfile::LaptopBatterySaving
                                    },
                                )
                                .radius(style.badge_radius()),
                            ),
                    ),
            )
            .child(
                // Health Overview Card
                col()
                    .grow(1.0)
                    .min_w(280.0)
                    .padding(style.card_padding())
                    .radius(style.card_radius())
                    .fill(p.card_fill_styled(style))
                    .border(style.border_width(), p.border_styled(style))
                    .gap(8.0)
                    .child(
                        row()
                            .gap(6.0)
                            .center()
                            .child(text("🩺").font(rune::tok::type_::LABEL))
                            .child(
                                text(s.tr_v(ViewTextKey::AutoCenterHealthAssessment))
                                    .font(rune::tok::type_::TITLE)
                                    .color(p.text_primary),
                            ),
                    )
                    .child(if let Some(ref h) = s.system_health_assessment {
                        col()
                            .gap(4.0)
                            .child(
                                text(format!(
                                    "Overall Grade: {:?} (Score: {}/100)",
                                    h.overall_grade, h.score
                                ))
                                .font(rune::tok::type_::LABEL)
                                .color(p.accent),
                            )
                            .child(
                                text(format!(
                                    "Telemetry: {} • Latency: {} • Services: {}",
                                    h.telemetry_leak_grade,
                                    h.dpc_latency_grade,
                                    h.bloat_services_grade
                                ))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_secondary),
                            )
                    } else {
                        col()
                            .gap(4.0)
                            .child(
                                text(s.tr_v(ViewTextKey::AutoCenterHealthPending))
                                    .font(rune::tok::type_::LABEL)
                                    .color(p.text_muted),
                            )
                            .child(
                                text(s.tr_v(ViewTextKey::AutoCenterHealthRunPrompt))
                                    .font(rune::tok::type_::BODY)
                                    .color(p.text_muted),
                            )
                    }),
            ),
    );

    // Plan Preview Card: Displays the tweaks recommended for the active profile
    let mut plan_items_row = row().w_full().min_w(0.0).flex_wrap().gaps(6.0, 6.0);
    for id in plan.recommended_tweak_ids.iter().take(24) {
        if let Some(t) = s.tweaks.iter().find(|t| &t.id == id) {
            let is_opt = s
                .get_tweak_scan(id)
                .map(|r| r.current_state == TweakState::AlreadyOptimized)
                .unwrap_or(false);
            plan_items_row = plan_items_row.child(pill_badge(
                format!("{} {}", if is_opt { "✓" } else { "•" }, t.title),
                if is_opt { p.success } else { p.accent },
                p,
            ));
        }
    }
    if plan.recommended_tweak_ids.len() > 24 {
        plan_items_row = plan_items_row.child(pill_badge(
            format!(
                "+{} more optimizations",
                plan.recommended_tweak_ids.len() - 24
            ),
            p.btn_secondary,
            p,
        ));
    }

    outer = outer.child(
        col()
            .w_full()
            .min_w(0.0)
            .padding(style.card_padding())
            .radius(style.card_radius())
            .fill(p.card_fill_styled(style))
            .border(style.border_width(), p.border_styled(style))
            .gap(8.0)
            .child(
                row()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        row()
                            .gap(6.0)
                            .items_center()
                            .child(text("📋").font(rune::tok::type_::LABEL))
                            .child(
                                text(format!(
                                    "Profile Plan Preview ({} Optimizations)",
                                    plan.recommended_tweak_ids.len()
                                ))
                                .font(rune::tok::type_::SECTION)
                                .color(p.text_primary),
                            ),
                    )
                    .child(pill_badge(
                        match active_profile {
                            AutoTuneProfile::GamingLowLatency => "Ultra Gaming",
                            AutoTuneProfile::ProAudioDpc => "Pro Audio / DAW",
                            AutoTuneProfile::WorkstationStability => "Privacy & Workstation",
                            AutoTuneProfile::LaptopBatterySaving => "Laptop Battery",
                        },
                        p.accent,
                        p,
                    )),
            )
            .child(plan_items_row),
    );

    // Rule 1: Always attach .child(element) BEFORE .winblast_scroll(p)
    outer
        .scroll(ScrollSpec {
            stick_bottom: false,
        })
        .winblast_scroll(p)
}
