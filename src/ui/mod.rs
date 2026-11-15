//! Top-level Rune UI layout: adaptive multi-resolution header, collapsible navigation sidebar,
//! view router, animated progress bar, bottom transaction changelog drawer, and root modal overlay.

pub mod theme;
pub mod views;
pub mod widgets;

use rune::color::Color;
use rune::{col, el, row, text, Cx, El, ScrollSpec};

use crate::app::{ActiveView, AppMode, AppState, EngineMode};
use crate::core::i18n::{Language, TextKey};
use crate::ui::theme::{ColorScheme, ThemeMode, ThemePalette};
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::diff_modal;
use crate::ui::widgets::scrollbar::WinBlastScrollable;

/// Single-breakpoint responsive viewport tier evaluated cleanly via `Cx::layout_with`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutTier {
    /// Narrow / compact layout (`width < 680pt`, e.g. 420x400 or narrow snap).
    Compact,
    /// Medium layout (`680pt <= width < 1020pt`, e.g. 800x600, split-screen).
    Medium,
    /// Full desktop layout (`width >= 1020pt`, e.g. 720p, 1080p, 1440p, 4K Ultrawide).
    Wide,
}

/// Zero-sized classifier function for zero-echo `Cx::layout_with`.
fn tier_from_rect(r: [f32; 4]) -> LayoutTier {
    if r[2] < 680.0 {
        LayoutTier::Compact
    } else if r[2] < 1020.0 {
        LayoutTier::Medium
    } else {
        LayoutTier::Wide
    }
}

/// Navigation section category title.
fn nav_section_header(title: &str, p: &ThemePalette) -> El {
    text(title)
        .font(rune::tok::type_::LABEL)
        .color(p.text_muted)
        .px(6.0)
        .pt(8.0)
        .pb(2.0)
        .shrink()
        .wrap()
}

/// Navigation button in the sidebar with active bar indicator, icon glyph, and optional status dot/badge.
#[allow(clippy::too_many_arguments)]
fn nav_button(
    key_id: &'static str,
    icon: &'static str,
    label: &str,
    active: bool,
    p: &ThemePalette,
    style: crate::ui::theme::UiThemeStyle,
    status_dot: bool,
    badge_count: usize,
    target: ActiveView,
) -> El {
    let mut btn = row()
        .key(key_id)
        .w_full()
        .min_w(0.0)
        .px(8.0)
        .py(6.0)
        .radius(style.btn_radius())
        .fill(if active { p.accent.alpha(0.18) } else { Color::TRANSPARENT })
        .hover(if active { p.accent.alpha(0.26) } else { p.bg_card_hover })
        .press(p.accent.alpha(0.35))
        .border(1.0, if active { p.accent.alpha(0.6) } else { Color::TRANSPARENT })
        .motion(style.motion())
        .center()
        .on_press(move |s: &mut AppState| s.navigate(target));

    // Active vertical indicator bar on left edge (yellow bar in screenshot)
    if active {
        btn = btn.child(
            el()
                .size(3.0, 16.0)
                .radius(1.5)
                .fill(p.warning)
                .mr(6.0),
        );
    } else {
        btn = btn.child(el().size(3.0, 16.0).mr(6.0));
    }

    // Icon glyph
    btn = btn.child(
        text(icon)
            .font(rune::tok::type_::LABEL)
            .color(if active { p.accent } else { p.text_muted })
            .mr(6.0),
    );

    // Label
    btn = btn.child(
        text(label)
            .font(rune::tok::type_::LABEL)
            .color(if active { p.text_primary } else { p.text_secondary })
            .grow(1.0)
            .min_w(0.0)
            .shrink()
            .truncate(),
    );

    // Right-side indicators (alert badge or amber status dot)
    if badge_count > 0 {
        btn = btn.child(
            row()
                .size(18.0, 18.0)
                .radius(9.0)
                .fill(p.danger)
                .center()
                .child(
                    text(badge_count.to_string())
                        .font(rune::tok::type_::LABEL)
                        .color(Color::WHITE),
                ),
        );
    } else if status_dot {
        btn = btn.child(
            el()
                .size(6.0, 6.0)
                .radius(3.0)
                .fill(p.warning),
        );
    }

    btn
}

/// Mode selection tab button (Easy vs Advanced / Instant vs Normal) with spring transition.
fn mode_tab(
    label: &'static str,
    active: bool,
    p: &ThemePalette,
    on_click: impl Fn(&mut AppState) + 'static,
) -> El {
    row()
        .min_w(0.0)
        .shrink()
        .px(10.0)
        .py(4.0)
        .radius(5.0)
        .fill(if active { p.accent } else { p.bg_card })
        .hover(if active { p.accent_hover } else { p.bg_card_hover })
        .press(p.accent_press)
        .border(1.0, if active { p.accent_hover } else { p.border })
        .motion(rune::tok::motion::SNAPPY)
        .center()
        .on_press(on_click)
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(if active { Color::WHITE } else { p.text_secondary })
                .shrink()
                .wrap(),
        )
}

/// Shared sidebar navigation column rendered inline on `Wide` or inside the mobile drawer on `Compact`/`Medium`.
fn build_sidebar(s: &AppState, p: &ThemePalette, tier: LayoutTier) -> El {
    let style = s.theme_style;
    let fault_count = s
        .fault_report
        .as_ref()
        .map(|r| r.total_critical_faults.max(1))
        .unwrap_or(1);

    let mut nav_items = col()
        .w_full()
        .min_w(0.0)
        .gap(3.0)
        // Group 1: Overview
        .child(nav_section_header(s.tr(TextKey::NavGroupOverview), p))
        .child(nav_button("nav_easy", "⚡", s.tr(TextKey::NavSystemPanel), s.active_view == ActiveView::EasyDashboard, p, style, false, 0, ActiveView::EasyDashboard))
        .child(nav_button("nav_hardware", "🖹", s.tr(TextKey::NavHardware), s.active_view == ActiveView::HardwareView, p, style, false, 0, ActiveView::HardwareView))
        .child(nav_button("nav_driver_bios", "💾", s.tr(TextKey::NavDriverBios), s.active_view == ActiveView::DriverBios, p, style, true, 0, ActiveView::DriverBios))

        // Group 2: Testing & Diagnostics
        .child(nav_section_header(s.tr(TextKey::NavGroupDiagnostics), p))
        .child(nav_button("nav_perf_tests", "📈", s.tr(TextKey::NavPerfTests), s.active_view == ActiveView::PerformanceTests, p, style, false, 0, ActiveView::PerformanceTests))
        .child(nav_button("nav_fault_diag", "🛡️", s.tr(TextKey::NavFaultDiag), s.active_view == ActiveView::FaultDiagnostics, p, style, false, fault_count, ActiveView::FaultDiagnostics))
        .child(nav_button("nav_latency", "⏱", s.tr(TextKey::NavLatency), s.active_view == ActiveView::LatencyTester, p, style, true, 0, ActiveView::LatencyTester))
        .child(nav_button("nav_core_parking", "🔲", s.tr(TextKey::NavCoreParking), s.active_view == ActiveView::CoreParking, p, style, true, 0, ActiveView::CoreParking))
        .child(nav_button("nav_thermal", "❄", s.tr(TextKey::NavThermal), s.active_view == ActiveView::ThermalMonitor, p, style, true, 0, ActiveView::ThermalMonitor))
        .child(nav_button("nav_disks", "💽", s.tr(TextKey::NavDisks), s.active_view == ActiveView::DiskHealth, p, style, true, 0, ActiveView::DiskHealth))
        .child(nav_button("nav_gaming_stats", "🎮", s.tr(TextKey::NavGamingStats), s.active_view == ActiveView::GamingStats, p, style, false, 0, ActiveView::GamingStats))
        .child(nav_button("nav_fps_bench", "📊", s.tr(TextKey::NavFpsBench), s.active_view == ActiveView::FpsBenchmark, p, style, false, 0, ActiveView::FpsBenchmark))
        .child(nav_button("nav_network", "📶", s.tr(TextKey::NavNetwork), s.active_view == ActiveView::NetworkDiag, p, style, false, 0, ActiveView::NetworkDiag))

        // Group 3: Optimization
        .child(nav_section_header(s.tr(TextKey::NavGroupOptimization), p))
        .child(nav_button("nav_advanced", "⚡", s.tr(TextKey::NavFpsOptimization), s.active_view == ActiveView::AdvancedMatrix, p, style, false, 0, ActiveView::AdvancedMatrix))
        .child(nav_button("nav_goal_planner", "🎯", s.tr(TextKey::NavGoalPlanner), s.active_view == ActiveView::GoalPlanner, p, style, false, 0, ActiveView::GoalPlanner))
        .child(nav_button("nav_game_profiles", "🕹️", s.tr(TextKey::NavGameProfiles), s.active_view == ActiveView::GameProfiles, p, style, false, 0, ActiveView::GameProfiles))
        .child(nav_button("nav_game_overlay", "🖥️", s.tr(TextKey::NavGameOverlay), s.active_view == ActiveView::GameOverlay, p, style, false, 0, ActiveView::GameOverlay))
        .child(nav_button("nav_game_suitability", "🎯", s.tr(TextKey::NavGameSuitability), s.active_view == ActiveView::GameSuitability, p, style, false, 0, ActiveView::GameSuitability))
        .child(nav_button("nav_clean_apps", "🗑️", s.tr(TextKey::NavCleanApps), s.active_view == ActiveView::CleanAndApps, p, style, false, 0, ActiveView::CleanAndApps))
        .child(nav_button("nav_startup_tasks", "⚙️", s.tr(TextKey::NavStartupTasks), s.active_view == ActiveView::StartupAndTasks, p, style, false, 0, ActiveView::StartupAndTasks))
        .child(nav_button("nav_win_repair", "🔧", s.tr(TextKey::NavWindowsRepair), s.active_view == ActiveView::WindowsRepair, p, style, false, 0, ActiveView::WindowsRepair))
        .child(nav_button("nav_backup", "📄", s.tr(TextKey::NavBackupReport), s.active_view == ActiveView::BackupCenter, p, style, false, 0, ActiveView::BackupCenter))
        .child(nav_button("nav_gpu", "🎛️", s.tr(TextKey::NavGpuMsiHags), s.active_view == ActiveView::GpuTuning, p, style, false, 0, ActiveView::GpuTuning))
        .child(nav_button("nav_privacy", "🛡️", s.tr(TextKey::NavPrivacyShield), s.active_view == ActiveView::PrivacyShield, p, style, false, 0, ActiveView::PrivacyShield))

        // Group 4: Account / Support (Lisans deleted)
        .child(nav_section_header(s.tr(TextKey::NavGroupAccount), p))
        .child(nav_button("nav_support", "✉", s.tr(TextKey::NavSupportAbout), s.active_view == ActiveView::SupportAbout, p, style, false, 0, ActiveView::SupportAbout));

    if tier == LayoutTier::Compact {
        nav_items = nav_items
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .gap(4.0)
                    .pt(8.0)
                    .pb(4.0)
                    .child(mode_tab(
                        s.tr(TextKey::EasyMode),
                        s.mode == AppMode::Easy,
                        p,
                        |s| {
                            s.mode = AppMode::Easy;
                            s.navigate(ActiveView::EasyDashboard);
                        },
                    ))
                    .child(mode_tab(
                        s.tr(TextKey::AdvancedMatrix),
                        s.mode == AppMode::Advanced,
                        p,
                        |s| {
                            s.mode = AppMode::Advanced;
                            s.navigate(ActiveView::AdvancedMatrix);
                        },
                    )),
            )
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .gap(4.0)
                    .pb(4.0)
                    .child(mode_tab(
                        s.tr(TextKey::InstantMode),
                        s.engine_mode == EngineMode::Instant,
                        p,
                        |s| {
                            if s.engine_mode != EngineMode::Instant {
                                s.toggle_engine_mode();
                            }
                        },
                    ))
                    .child(mode_tab(
                        s.tr(TextKey::NormalMode),
                        s.engine_mode == EngineMode::Normal,
                        p,
                        |s| {
                            if s.engine_mode != EngineMode::Normal {
                                s.toggle_engine_mode();
                            }
                        },
                    ))
                    .child(action_button(
                        s.tr(s.color_scheme.text_key()),
                        p.bg_card,
                        p.accent,
                        p,
                        |s| {
                            let next = match s.color_scheme {
                                ColorScheme::CyberIndigo => ColorScheme::MidnightStealth,
                                ColorScheme::MidnightStealth => ColorScheme::EmeraldMint,
                                ColorScheme::EmeraldMint => ColorScheme::CrimsonSunset,
                                ColorScheme::CrimsonSunset => ColorScheme::CyberIndigo,
                            };
                            s.set_color_scheme(next);
                        },
                    ))
                    .child(action_button(
                        s.theme_style.short_label(),
                        p.accent.alpha(0.18),
                        p.accent,
                        p,
                        |s| {
                            let next = match s.theme_style {
                                crate::ui::theme::UiThemeStyle::Glassy => crate::ui::theme::UiThemeStyle::Minimalistic,
                                crate::ui::theme::UiThemeStyle::Minimalistic => crate::ui::theme::UiThemeStyle::Classic,
                                crate::ui::theme::UiThemeStyle::Classic => crate::ui::theme::UiThemeStyle::Compact,
                                crate::ui::theme::UiThemeStyle::Compact => crate::ui::theme::UiThemeStyle::Materialistic,
                                crate::ui::theme::UiThemeStyle::Materialistic => crate::ui::theme::UiThemeStyle::Glassy,
                            };
                            s.set_theme_style(next);
                        },
                    )),
            );
    }

    col()
        .key("sidebar")
        .w(p.sidebar_width)
        .max_w(270.0)
        .h_full()
        .min_h(0.0)
        .padding(10.0)
        .fill(p.bg_sidebar)
        .border(p.border_width, p.border)
        .gap(8.0)
        .child(
            col()
                .key("nav_sidebar_scroll")
                .w_full()
                .grow(1.0)
                .min_w(0.0)
                .min_h(0.0)
                .scroll(ScrollSpec { stick_bottom: false })
                .child(nav_items)
                .winblast_scroll(p),
        )
        .child(
            col()
                .key("sidebar_pinned_footer")
                .gap(6.0)
                .w_full()
                .pt(6.0)
                .child(
                    action_button_guarded(
                        if s.is_scanning {
                            s.tr(TextKey::Scanning)
                        } else {
                            s.tr(TextKey::ScanSystem)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.is_scanning,
                        p,
                        |s: &mut AppState| s.start_scan(),
                    )
                    .w_full(),
                )
                .child(
                    text(format!(
                        "{}: {} / {}",
                        s.tr(TextKey::NavStagedCount),
                        s.selected_tweaks.len(),
                        s.tweaks.len()
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap()
                    .center(),
                ),
        )
}

/// Top-level view rendering function matching Rune's `AppSpec::view` signature.
pub fn view(s: &AppState, cx: &mut Cx<AppState>) -> El {
    let tier = cx
        .layout_with("winblast_root", tier_from_rect)
        .unwrap_or(LayoutTier::Wide);
    let p = s.palette();
    let is_elevated = s.is_elevated;

    // ── Top Header Bar (Responsive Wrap, No Rigid Height or Mid-Row Spacer) ─
    let os_desc = if let Some(os) = &s.os_info {
        format!("Win {} ({})", os.edition, os.build_number)
    } else {
        "Windows 10/11".to_string()
    };

    let mut brand_group = row().min_w(0.0).shrink().gap(8.0).items_center().flex_wrap();
    if tier != LayoutTier::Wide {
        brand_group = brand_group.child(
            row()
                .key("nav_menu_toggle")
                .px(9.0)
                .py(5.0)
                .radius(5.0)
                .fill(if s.sidebar_open { p.accent } else { p.bg_card })
                .hover(if s.sidebar_open { p.accent_hover } else { p.bg_card_hover })
                .border(1.0, if s.sidebar_open { p.accent } else { p.border })
                .motion(rune::tok::motion::SNAPPY)
                .center()
                .on_press(|s: &mut AppState| s.toggle_sidebar())
                .child(
                    text("☰")
                        .font(rune::tok::type_::TITLE)
                        .color(if s.sidebar_open { Color::WHITE } else { p.text_primary }),
                ),
        );
    }
    brand_group = brand_group.child(
        text(s.tr(TextKey::AppTitle))
            .font(rune::tok::type_::TITLE)
            .color(p.accent)
            .shrink()
            .wrap(),
    );
    if tier == LayoutTier::Wide {
        brand_group = brand_group.child(
            text(s.tr(TextKey::AppSubtitle))
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted)
                .shrink()
                .truncate(),
        );
    }

    let mut controls_group = row()
        .min_w(0.0)
        .shrink()
        .flex_wrap()
        .items_center()
        .gaps(6.0, 6.0)
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .px(8.0)
                .py(4.0)
                .radius(5.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .hover(p.bg_card_hover)
                .motion(rune::tok::motion::SNAPPY)
                .center()
                .on_press(|s: &mut AppState| {
                    let next = match s.language {
                        Language::English => Language::Turkish,
                        Language::Turkish => Language::German,
                        Language::German => Language::French,
                        Language::French => Language::Spanish,
                        Language::Spanish => Language::Russian,
                        Language::Russian => Language::Chinese,
                        Language::Chinese => Language::Japanese,
                        Language::Japanese => Language::English,
                    };
                    s.set_language(next);
                })
                .child(
                    text(format!("🌐 {}", s.language.label()))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            row()
                .min_w(0.0)
                .shrink()
                .px(8.0)
                .py(4.0)
                .radius(5.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .hover(p.bg_card_hover)
                .motion(rune::tok::motion::SNAPPY)
                .center()
                .on_press(|s: &mut AppState| s.toggle_theme_mode())
                .child(
                    text(if s.theme_mode == ThemeMode::Dark {
                        s.tr(TextKey::ThemeDark)
                    } else {
                        s.tr(TextKey::ThemeLight)
                    })
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_primary)
                    .shrink()
                    .wrap(),
                ),
        );

    let theme_btn_label = if tier == LayoutTier::Compact {
        s.theme_style.short_label()
    } else {
        s.theme_style.label()
    };

    controls_group = controls_group.child(
        row()
            .key("theme_style_selector")
            .min_w(0.0)
            .shrink()
            .px(8.0)
            .py(4.0)
            .radius(p.btn_radius)
            .fill(p.accent.alpha(0.18))
            .border(p.border_width, p.accent.alpha(0.60))
            .hover(p.accent.alpha(0.30))
            .press(p.accent.alpha(0.45))
            .motion(p.motion)
            .center()
            .on_press(|s: &mut AppState| {
                let next = match s.theme_style {
                    crate::ui::theme::UiThemeStyle::Glassy => crate::ui::theme::UiThemeStyle::Minimalistic,
                    crate::ui::theme::UiThemeStyle::Minimalistic => crate::ui::theme::UiThemeStyle::Classic,
                    crate::ui::theme::UiThemeStyle::Classic => crate::ui::theme::UiThemeStyle::Compact,
                    crate::ui::theme::UiThemeStyle::Compact => crate::ui::theme::UiThemeStyle::Materialistic,
                    crate::ui::theme::UiThemeStyle::Materialistic => crate::ui::theme::UiThemeStyle::Glassy,
                };
                s.set_theme_style(next);
            })
            .child(
                text(theme_btn_label)
                    .font(rune::tok::type_::LABEL)
                    .color(p.accent)
                    .shrink()
                    .wrap(),
            ),
    );

    if tier != LayoutTier::Compact {
        controls_group = controls_group
            .child(
                row()
                    .min_w(0.0)
                    .shrink()
                    .px(8.0)
                    .py(4.0)
                    .radius(p.btn_radius)
                    .fill(p.bg_card)
                    .border(p.border_width, p.border)
                    .hover(p.bg_card_hover)
                    .motion(p.motion)
                    .center()
                    .on_press(|s: &mut AppState| {
                        let next = match s.color_scheme {
                            ColorScheme::CyberIndigo => ColorScheme::MidnightStealth,
                            ColorScheme::MidnightStealth => ColorScheme::EmeraldMint,
                            ColorScheme::EmeraldMint => ColorScheme::CrimsonSunset,
                            ColorScheme::CrimsonSunset => ColorScheme::CyberIndigo,
                        };
                        s.set_color_scheme(next);
                    })
                    .child(
                        text(s.tr(s.color_scheme.text_key()))
                            .font(rune::tok::type_::LABEL)
                            .color(p.accent)
                            .shrink()
                            .wrap(),
                    ),
            )
            .child(
                row()
                    .min_w(0.0)
                    .shrink()
                    .flex_wrap()
                    .padding(2.0)
                    .radius(7.0)
                    .fill(p.bg_primary)
                    .border(1.0, p.border)
                    .gap(3.0)
                    .child(mode_tab(
                        s.tr(TextKey::EasyMode),
                        s.mode == AppMode::Easy,
                        p,
                        |s| {
                            s.mode = AppMode::Easy;
                            s.navigate(ActiveView::EasyDashboard);
                        },
                    ))
                    .child(mode_tab(
                        s.tr(TextKey::AdvancedMatrix),
                        s.mode == AppMode::Advanced,
                        p,
                        |s| {
                            s.mode = AppMode::Advanced;
                            s.navigate(ActiveView::AdvancedMatrix);
                        },
                    )),
            )
            .child(
                row()
                    .min_w(0.0)
                    .shrink()
                    .flex_wrap()
                    .padding(2.0)
                    .radius(7.0)
                    .fill(p.bg_primary)
                    .border(1.0, p.border)
                    .gap(3.0)
                    .child(mode_tab(
                        s.tr(TextKey::InstantMode),
                        s.engine_mode == EngineMode::Instant,
                        p,
                        |s| {
                            if s.engine_mode != EngineMode::Instant {
                                s.toggle_engine_mode();
                            }
                        },
                    ))
                    .child(mode_tab(
                        s.tr(TextKey::NormalMode),
                        s.engine_mode == EngineMode::Normal,
                        p,
                        |s| {
                            if s.engine_mode != EngineMode::Normal {
                                s.toggle_engine_mode();
                            }
                        },
                    )),
            );
    }

    if tier != LayoutTier::Compact {
        if let Some(ref m) = s.realtime_metrics {
            controls_group = controls_group
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(7.0)
                        .py(3.0)
                        .radius(4.0)
                        .fill(p.bg_card)
                        .border(1.0, p.border)
                        .child(
                            text(format!("CPU: {:.1}%", m.cpu_usage))
                                .font(rune::tok::type_::LABEL)
                                .color(p.sparkline_cpu)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(7.0)
                        .py(3.0)
                        .radius(4.0)
                        .fill(p.bg_card)
                        .border(1.0, p.border)
                        .child(
                            text(format!("RAM: {:.1}%", m.ram_percentage))
                                .font(rune::tok::type_::LABEL)
                                .color(p.sparkline_ram)
                                .shrink()
                                .wrap(),
                        ),
                );
        }

        controls_group = controls_group
            .child(
                row()
                    .min_w(0.0)
                    .shrink()
                    .px(8.0)
                    .py(4.0)
                    .radius(5.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .child(
                        text(os_desc)
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_secondary)
                            .shrink()
                            .wrap(),
                    ),
            )
            .child(
                row()
                    .min_w(0.0)
                    .shrink()
                    .px(8.0)
                    .py(4.0)
                    .radius(5.0)
                    .fill(if is_elevated {
                        p.success.alpha(0.18)
                    } else {
                        p.warning.alpha(0.18)
                    })
                    .border(
                        1.0,
                        if is_elevated {
                            p.success.alpha(0.5)
                        } else {
                            p.warning.alpha(0.5)
                        },
                    )
                    .child(
                        text(s.tr(if is_elevated {
                            TextKey::AdminElevated
                        } else {
                            TextKey::AdminStandard
                        }))
                        .font(rune::tok::type_::LABEL)
                        .color(if is_elevated { p.success } else { p.warning })
                        .shrink()
                        .wrap(),
                    ),
            );
    }

    let top_bar = row()
        .key("top_bar")
        .w_full()
        .min_w(0.0)
        .min_h(44.0)
        .px(12.0)
        .py(6.0)
        .fill(p.bg_sidebar)
        .border(1.0, p.border)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(6.0, 10.0)
        .child(brand_group)
        .child(controls_group);

    // ── Content View Router ──────────────────────────────────────────────────
    let content_view = match s.active_view {
        ActiveView::EasyDashboard => views::easy_dashboard::view(s, tier),
        ActiveView::AdvancedMatrix => views::advanced_matrix::view(s, tier),
        ActiveView::GoalPlanner => views::goal_planner::view(s, tier),
        ActiveView::GpuTuning => views::gpu_tuning::view(s, tier),
        ActiveView::HardwareView => views::hardware_view::view(s, tier),
        ActiveView::PrivacyShield => views::privacy_shield::view(s, tier),
        ActiveView::BackupCenter => views::backup_center::view(s, tier),
        ActiveView::CoreParking => views::cpu_parking::view(s, tier),
        ActiveView::LatencyTester => views::latency_bench::view(s, tier),
        ActiveView::CleanAndApps => views::clean_apps::view(s, tier),
        ActiveView::StartupAndTasks => views::startup_tasks::view(s, tier),
        // v1.1.0 New Views
        ActiveView::DriverBios => views::driver_bios::view(s, tier),
        ActiveView::PerformanceTests => views::performance_tests::view(s, tier),
        ActiveView::FaultDiagnostics => views::fault_diagnostics::view(s, tier),
        ActiveView::ThermalMonitor => views::thermal_monitor::view(s, tier),
        ActiveView::DiskHealth => views::disk_health::view(s, tier),
        ActiveView::GamingStats => views::gaming_stats::view(s, tier),
        ActiveView::FpsBenchmark => views::fps_benchmark::view(s, tier),
        ActiveView::NetworkDiag => views::network_diag::view(s, tier),
        ActiveView::GameProfiles => views::game_profiles::view(s, tier),
        ActiveView::GameOverlay => views::game_overlay::view(s, tier),
        ActiveView::GameSuitability => views::game_suitability::view(s, tier),
        ActiveView::WindowsRepair => views::windows_repair::view(s, tier),
        ActiveView::SupportAbout => views::support_about::view(s, tier),
    };

    // Centered max-reading-width container on 4K/ultrawide while 100% fluid on normal sizes
    let content_host = col()
        .key("content_host")
        .grow(1.0)
        .h_full()
        .min_w(0.0)
        .min_h(0.0)
        .fill(p.bg_primary)
        .items_center()
        .child(
            col()
                .w_full()
                .max_w(1680.0)
                .grow(1.0)
                .min_w(0.0)
                .min_h(0.0)
                .child(content_view),
        );

    let mut main_body = row()
        .key("main_body")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(120.0);

    if tier == LayoutTier::Wide {
        main_body = main_body.child(build_sidebar(s, p, tier));
    }
    main_body = main_body.child(content_host);

    // ── Live Progress Bar Section ────────────────────────────────────────────
    let opt_pct = s.optimization_progress();
    let active_count = s
        .scan_results
        .iter()
        .filter(|r| r.current_state == crate::scanner::TweakState::AlreadyOptimized)
        .count();
    let total_count = s
        .scan_results
        .iter()
        .filter(|r| r.current_state != crate::scanner::TweakState::NotApplicable)
        .count();

    let progress_val = if s.is_scanning || s.progress_message.is_some() {
        s.progress.clamp(0.1, 0.95)
    } else {
        opt_pct
    };

    let progress_color = if s.is_scanning {
        p.warning
    } else if opt_pct > 0.65 {
        p.success
    } else if opt_pct > 0.35 {
        p.accent
    } else {
        p.warning
    };

    let progress_text = if let Some(ref msg) = s.progress_message {
        msg.clone()
    } else {
        format!(
            "{}: {:.0}% ({} / {} {})",
            s.tr(TextKey::ProgressBarOptimized),
            opt_pct * 100.0,
            active_count,
            total_count,
            s.tr(TextKey::TweaksGained),
        )
    };

    let progress_bar_section = row()
        .key("progress_bar_section")
        .w_full()
        .min_w(0.0)
        .min_h(26.0)
        .px(12.0)
        .py(4.0)
        .fill(p.bg_sidebar)
        .border(1.0, p.border)
        .flex_wrap()
        .items_center()
        .gaps(6.0, 10.0)
        .child(
            text(progress_text)
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted)
                .shrink()
                .truncate(),
        )
        .child(
            row()
                .grow(1.0)
                .min_w(80.0)
                .h(6.0)
                .radius(3.0)
                .fill(p.bg_primary)
                .border(1.0, p.border)
                .child(
                    el()
                        .w_frac(progress_val.clamp(0.01, 1.0))
                        .h_full()
                        .radius(3.0)
                        .fill(progress_color)
                        .motion(rune::tok::motion::SNAPPY),
                ),
        )
        .child(
            row()
                .key("changelog_toggle")
                .min_w(0.0)
                .shrink()
                .px(8.0)
                .py(2.0)
                .radius(4.0)
                .fill(if s.show_changelog {
                    p.accent.alpha(0.2)
                } else {
                    p.bg_card
                })
                .border(1.0, if s.show_changelog { p.accent } else { p.border })
                .hover(p.bg_card_hover)
                .motion(rune::tok::motion::SNAPPY)
                .center()
                .on_press(|s: &mut AppState| s.toggle_changelog())
                .child(
                    text(if s.show_changelog {
                        s.tr(TextKey::ChangelogToggleHide)
                    } else {
                        s.tr(TextKey::ChangelogToggleShow)
                    })
                    .font(rune::tok::type_::LABEL)
                    .color(if s.show_changelog { p.accent } else { p.text_secondary })
                    .shrink()
                    .wrap(),
                ),
        );

    // ── Bottom Changelog Drawer (Expandable, Localized, Responsive) ──────────
    let changelog_drawer = if s.show_changelog {
        let mut entries_col = col().w_full().min_w(0.0).gap(6.0).padding(10.0);
        if s.ledger_history.is_empty() {
            entries_col = entries_col.child(
                text(s.tr(TextKey::ChangelogEmpty))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap()
                    .center(),
            );
        } else {
            for entry in s.ledger_history.iter().take(12) {
                let entry_id = entry.id;
                let is_rolled_back = entry.rolled_back;
                let can_revert = entry.is_reversible();
                let status_badge = if is_rolled_back {
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(5.0)
                        .py(1.0)
                        .radius(3.0)
                        .fill(p.neutral_badge.alpha(0.35))
                        .child(
                            text(s.tr(TextKey::RolledBackBadge))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                } else if !can_revert {
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(5.0)
                        .py(1.0)
                        .radius(3.0)
                        .fill(p.neutral_badge.alpha(0.35))
                        .child(
                            text(s.tr(TextKey::NonReversibleBadge))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap(),
                        )
                } else {
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(5.0)
                        .py(1.0)
                        .radius(3.0)
                        .fill(p.success.alpha(0.18))
                        .child(
                            text(s.tr(TextKey::AppliedBadge))
                                .font(rune::tok::type_::LABEL)
                                .color(p.success)
                                .shrink()
                                .wrap(),
                        )
                };

                let undo_btn = if !is_rolled_back && can_revert {
                    let can_undo = !s.is_rolling_back && !s.is_applying && s.in_flight_tweaks.is_empty();
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(6.0)
                        .py(2.0)
                        .radius(4.0)
                        .fill(if can_undo { p.warning.alpha(0.15) } else { p.btn_secondary.alpha(0.65) })
                        .border(1.0, if can_undo { p.warning.alpha(0.4) } else { p.border.alpha(0.65) })
                        .hover(if can_undo { p.warning.alpha(0.3) } else { p.btn_secondary.alpha(0.65) })
                        .motion(rune::tok::motion::SNAPPY)
                        .center()
                        .on_press(move |s: &mut AppState| {
                            if !s.is_rolling_back {
                                s.rollback_entry(entry_id);
                            }
                        })
                        .child(
                            text(if s.is_rolling_back {
                                s.tr(TextKey::BusyWorking)
                            } else {
                                s.tr(TextKey::UndoBtn)
                            })
                            .font(rune::tok::type_::LABEL)
                            .color(if can_undo { p.warning } else { p.text_muted })
                            .shrink()
                            .wrap(),
                        )
                } else {
                    el()
                };

                let row_item = row()
                    .w_full()
                    .min_w(0.0)
                    .px(8.0)
                    .py(5.0)
                    .radius(4.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .flex_wrap()
                    .items_center()
                    .gaps(4.0, 8.0)
                    .child(
                        text(entry.timestamp.format("%m-%d %H:%M:%S").to_string())
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted),
                    )
                    .child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .max_w(240.0)
                            .px(5.0)
                            .py(1.0)
                            .radius(3.0)
                            .fill(p.accent.alpha(0.18))
                            .child(
                                text(crate::core::status_i18n::localize_ledger_action(s.language, &entry.action_type))
                                    .font(rune::tok::type_::LABEL)
                                    .color(p.accent)
                                    .shrink()
                                    .truncate(),
                            ),
                    )
                    .child(
                        text(s.tweak_title(&entry.tweak_id, &entry.tweak_id))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_primary)
                            .grow(1.0)
                            .shrink()
                            .min_w(100.0)
                            .wrap_anywhere(),
                    )
                    .child(status_badge)
                    .child(undo_btn);

                entries_col = entries_col.child(row_item);
            }
        }

        let drawer_h = if tier == LayoutTier::Compact { 110.0 } else { 150.0 };
        col()
            .key("changelog_drawer")
            .w_full()
            .min_w(0.0)
            .h(drawer_h)
            .max_h(drawer_h)
            .min_h(0.0)
            .fill(p.bg_sidebar)
            .border(1.0, p.border)
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .px(12.0)
                    .py(6.0)
                    .fill(p.bg_card)
                    .border(1.0, p.border)
                    .flex_wrap()
                    .justify_between()
                    .items_center()
                    .gap(8.0)
                    .child(
                        text(s.tr(TextKey::ChangelogTitle))
                            .font(rune::tok::type_::TITLE)
                            .color(p.accent)
                            .grow(1.0)
                            .min_w(0.0)
                            .shrink()
                            .wrap(),
                    )
                    .child(action_button(
                        s.tr(TextKey::ChangelogClear),
                        p.danger.alpha(0.15),
                        p.danger,
                        p,
                        |s| s.clear_changelog(),
                    )),
            )
            .child(
                entries_col
                    .key("changelog_drawer_scroll")
                    .grow(1.0)
                    .min_h(0.0)
                    .scroll(ScrollSpec { stick_bottom: false })
                    .winblast_scroll(p),
            )
    } else {
        el()
    };

    // ── Footer Status Bar ────────────────────────────────────────────────────
    let mut footer = row()
        .key("footer_bar")
        .w_full()
        .min_w(0.0)
        .min_h(26.0)
        .px(12.0)
        .py(4.0)
        .fill(p.bg_primary)
        .border(1.0, p.border)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(4.0, 10.0)
        .child(
            text(&s.status_message)
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted)
                .grow(1.0)
                .min_w(0.0)
                .shrink()
                .truncate(),
        );

    if tier != LayoutTier::Compact {
        footer = footer.child(
            text(s.tr(TextKey::FooterStatus))
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted)
                .shrink()
                .truncate(),
        );
    }

    // ── Root Frame & Window-Level Diff Preview Overlay ───────────────────────
    let mut root = col()
        .key("winblast_root")
        .w_full()
        .h_full()
        .min_w(0.0)
        .min_h(0.0)
        .fill(p.bg_primary)
        .child(top_bar)
        .child(main_body)
        .child(changelog_drawer)
        .child(progress_bar_section)
        .child(footer);

    if tier != LayoutTier::Wide && s.sidebar_open {
        root = root.child(
            row()
                .key("nav_drawer")
                .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .fill(Color::BLACK.alpha(0.45))
                .modal(true, |s: &mut AppState| s.sidebar_open = false)
                .items_start()
                .child(build_sidebar(s, p, tier)),
        );
    }

    if s.show_diff_preview {
        root = root.child(
            col()
                .key("diff_modal_overlay")
                .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
                .padding(16.0)
                .fill(Color::BLACK.alpha(0.55))
                .center()
                .modal(true, |s: &mut AppState| {
                    s.show_diff_preview = false;
                    s.inspected_tweak = None;
                })
                .child(diff_modal::view_state(s, p)),
        );
    }

    root
}
