//! Automation Center View in Rune.
//!
//! Provides 1-click system auto-tuning, background health assessment,
//! scheduled maintenance task management, and memory/cache trimming.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::tools::automation::AutoTuneProfile;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;

    col()
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
                                if s.is_running_automation { "Running..." } else { s.tr(TextKey::AutomationOneClick) },
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
        )
        // Profile Selection & Health Overview Grid
        .child(
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
                                    text("Auto-Tune Profiles")
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            text("Select an optimization profile tailored for your primary system workload:")
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
                                        if s.selected_auto_tune_profile == AutoTuneProfile::GamingLowLatency { p.accent } else { p.bg_card },
                                        if s.selected_auto_tune_profile == AutoTuneProfile::GamingLowLatency { Color::WHITE } else { p.text_secondary },
                                        p,
                                        |s| s.selected_auto_tune_profile = AutoTuneProfile::GamingLowLatency,
                                    )
                                    .radius(style.badge_radius()),
                                )
                                .child(
                                    action_button(
                                        "🎵 Pro Audio / DAW",
                                        if s.selected_auto_tune_profile == AutoTuneProfile::ProAudioDpc { p.accent } else { p.bg_card },
                                        if s.selected_auto_tune_profile == AutoTuneProfile::ProAudioDpc { Color::WHITE } else { p.text_secondary },
                                        p,
                                        |s| s.selected_auto_tune_profile = AutoTuneProfile::ProAudioDpc,
                                    )
                                    .radius(style.badge_radius()),
                                )
                                .child(
                                    action_button(
                                        "🛡 Privacy & Workstation",
                                        if s.selected_auto_tune_profile == AutoTuneProfile::WorkstationStability { p.accent } else { p.bg_card },
                                        if s.selected_auto_tune_profile == AutoTuneProfile::WorkstationStability { Color::WHITE } else { p.text_secondary },
                                        p,
                                        |s| s.selected_auto_tune_profile = AutoTuneProfile::WorkstationStability,
                                    )
                                    .radius(style.badge_radius()),
                                )
                                .child(
                                    action_button(
                                        "🔋 Laptop Battery",
                                        if s.selected_auto_tune_profile == AutoTuneProfile::LaptopBatterySaving { p.accent } else { p.bg_card },
                                        if s.selected_auto_tune_profile == AutoTuneProfile::LaptopBatterySaving { Color::WHITE } else { p.text_secondary },
                                        p,
                                        |s| s.selected_auto_tune_profile = AutoTuneProfile::LaptopBatterySaving,
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
                                    text("System Health Assessment")
                                        .font(rune::tok::type_::TITLE)
                                        .color(p.text_primary),
                                ),
                        )
                        .child(
                            if let Some(ref h) = s.system_health_assessment {
                                col()
                                    .gap(4.0)
                                    .child(
                                        text(format!("Overall Grade: {:?} (Score: {}/100)", h.overall_grade, h.score))
                                            .font(rune::tok::type_::LABEL)
                                            .color(p.accent),
                                    )
                                    .child(
                                        text(format!("Telemetry: {} • Latency: {} • Services: {}", h.telemetry_leak_grade, h.dpc_latency_grade, h.bloat_services_grade))
                                            .font(rune::tok::type_::BODY)
                                            .color(p.text_secondary),
                                    )
                            } else {
                                col()
                                    .gap(4.0)
                                    .child(text("Health audit pending...").font(rune::tok::type_::LABEL).color(p.text_muted))
                                    .child(text("Run an audit or auto-tune to generate a system score.").font(rune::tok::type_::BODY).color(p.text_muted))
                            },
                        ),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false })
        .winblast_scroll(p)
}
