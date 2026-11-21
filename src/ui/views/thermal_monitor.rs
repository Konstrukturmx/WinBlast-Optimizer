//! Sıcaklık ve fanlar (Thermal & Fan Monitor) View in Rune.
//!
//! Live CPU Package temperatures, GPU junction temps, fan speeds, and hardware thermal throttling indicators.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::hardware::thermal::ThermalHealth;
use crate::ui::widgets::button::action_button;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let thermals = s.thermal_snapshot.clone().unwrap_or_default();

    let cpu_color = if thermals.cpu_package_temp_c > 85.0 {
        p.danger
    } else if thermals.cpu_package_temp_c > 75.0 {
        p.warning
    } else {
        p.success
    };

    let gpu_color = if thermals.gpu_core_temp_c > 83.0 {
        p.danger
    } else if thermals.gpu_core_temp_c > 72.0 {
        p.warning
    } else {
        p.success
    };

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
                .center()
                .gaps(12.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text(s.tr_v(ViewTextKey::ThermalTitle))
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text(s.tr_v(ViewTextKey::ThermalSubtitle))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    action_button(
                        s.tr_v(ViewTextKey::ThermalRefreshBtn),
                        p.accent,
                        Color::WHITE,
                        p,
                        |s| {
                            s.scan_thermal_snapshot_async();
                        },
                    )
                    .radius(style.btn_radius()),
                ),
        )
        // Sensor Status Banner (Honest Hardware Truth)
        .child(
            row()
                .w_full()
                .padding(10.0)
                .radius(style.card_radius().min(6.0))
                .fill(if thermals.sensors_available {
                    p.success.alpha(0.12)
                } else {
                    p.bg_card
                })
                .border(
                    1.0,
                    if thermals.sensors_available {
                        p.success.alpha(0.3)
                    } else {
                        p.border
                    },
                )
                .child(
                    text(if thermals.sensors_available {
                        s.tr_v(ViewTextKey::ThermalActiveSensors)
                    } else {
                        s.tr_v(ViewTextKey::ThermalOemBanner)
                    })
                    .font(rune::tok::type_::LABEL)
                    .color(if thermals.sensors_available {
                        p.success
                    } else {
                        p.text_secondary
                    })
                    .shrink()
                    .wrap(),
                ),
        )
        // Temperature KPI Gauges
        .child(
            row()
                .w_full()
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    // CPU Temp Gauge Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), cpu_color.alpha(0.4))
                        .hover(p.bg_card_hover)
                        .motion(p.motion)
                        .gap(4.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("❄").font(rune::tok::type_::LABEL))
                                .child(
                                    text(s.tr_v(ViewTextKey::ThermalCpuPackage))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_secondary),
                                ),
                        )
                        .child(
                            text(format!("{:.1}°C", thermals.cpu_package_temp_c))
                                .font(rune::tok::type_::TITLE)
                                .color(cpu_color),
                        )
                        .child(
                            text(format!(
                                "{}: {:.1}°C • Throttling: {}",
                                s.tr_v(ViewTextKey::ThermalMaxCore),
                                thermals.cpu_max_core_temp_c,
                                if thermals.cpu_throttling_active {
                                    s.tr_v(ViewTextKey::ThermalThrottlingActive)
                                } else {
                                    s.tr_v(ViewTextKey::ThermalThrottlingNone)
                                }
                            ))
                            .font(rune::tok::type_::BODY)
                            .color(
                                if thermals.cpu_throttling_active {
                                    p.danger
                                } else {
                                    p.text_muted
                                },
                            ),
                        ),
                )
                .child(
                    // GPU Temp Gauge Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), gpu_color.alpha(0.4))
                        .hover(p.bg_card_hover)
                        .motion(p.motion)
                        .gap(4.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("🎮").font(rune::tok::type_::LABEL))
                                .child(
                                    text(s.tr_v(ViewTextKey::ThermalGpuCore))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_secondary),
                                ),
                        )
                        .child(
                            text(format!("{:.1}°C", thermals.gpu_core_temp_c))
                                .font(rune::tok::type_::TITLE)
                                .color(gpu_color),
                        )
                        .child(
                            text(format!(
                                "Hotspot: {:.1}°C • Throttling: {}",
                                thermals.gpu_hotspot_temp_c,
                                s.tr_v(ViewTextKey::ThermalThrottlingNone)
                            ))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted),
                        ),
                )
                .child(
                    // Fan & Cooling Gauge Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .hover(p.bg_card_hover)
                        .motion(p.motion)
                        .gap(4.0)
                        .child(
                            row()
                                .gap(6.0)
                                .center()
                                .child(text("💨").font(rune::tok::type_::LABEL))
                                .child(
                                    text(s.tr_v(ViewTextKey::ThermalFanRpm))
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_secondary),
                                ),
                        )
                        .child(
                            text(if thermals.fan_speed_rpm > 0 {
                                format!("{} RPM", thermals.fan_speed_rpm)
                            } else {
                                "Auto PWM Active".to_string()
                            })
                            .font(rune::tok::type_::TITLE)
                            .color(p.accent),
                        )
                        .child(
                            text(
                                if let Some(comp) = thermals
                                    .components
                                    .iter()
                                    .find(|c| c.fan_speed_pct.is_some())
                                {
                                    format!(
                                        "GPU Fan PWM: {:.0}% • Smart Fan Curve Active",
                                        comp.fan_speed_pct.unwrap_or(0.0)
                                    )
                                } else {
                                    "Smart Fan Curve Active • Quiet / Performance Mode".to_string()
                                },
                            )
                            .font(rune::tok::type_::BODY)
                            .color(p.text_muted),
                        ),
                ),
        )
        // Detailed Component Sensor Table
        .child(
            col()
                .w_full()
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .gap(8.0)
                .child(
                    text(s.tr_v(ViewTextKey::ThermalComponentsTitle))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_primary),
                )
                .children(thermals.components.iter().map(|c| {
                    let badge_color = match c.status {
                        ThermalHealth::Cool | ThermalHealth::Optimal => p.success,
                        ThermalHealth::Warm => p.warning,
                        ThermalHealth::Hot => p.danger,
                    };
                    row()
                        .w_full()
                        .padding(10.0)
                        .radius(style.card_radius().min(6.0))
                        .fill(p.bg_card_hover)
                        .center()
                        .gaps(10.0, 6.0)
                        .flex_wrap()
                        .child(
                            col()
                                .grow(1.0)
                                .min_w(180.0)
                                .child(
                                    text(&c.name)
                                        .font(rune::tok::type_::LABEL)
                                        .color(p.text_primary),
                                )
                                .child(
                                    text(format!("Thermal Threshold: {:.0}°C", c.max_temp_c))
                                        .font(rune::tok::type_::BODY)
                                        .color(p.text_muted),
                                ),
                        )
                        .child(
                            col()
                                .min_w(120.0)
                                .child(
                                    text(format!("{:.1}°C", c.current_temp_c))
                                        .font(rune::tok::type_::LABEL)
                                        .color(badge_color),
                                )
                                .child(
                                    text(c.status.label())
                                        .font(rune::tok::type_::BODY)
                                        .color(badge_color),
                                ),
                        )
                })),
        )
        .scroll(ScrollSpec {
            stick_bottom: false,
        })
        .winblast_scroll(p)
}
