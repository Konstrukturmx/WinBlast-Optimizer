//! Hardware View in Rune — 100% localized CPU topology, Task Manager waveforms,
//! NT latency diagnostics, and adaptive layout across all screen resolutions.

use rune::color::Color;
use rune::{col, el, row, text, El, ScrollSpec};

use crate::app::{ActiveView, AppState};
use crate::core::i18n::TextKey;
use crate::core::view_i18n::ViewTextKey;
use crate::ui::widgets::button::{action_button, action_button_guarded, kv_row, section_card};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::widgets::sparkline;
use crate::ui::LayoutTier;

/// Renders the responsive hardware diagnostics and low-level overview view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let _style = s.theme_style;
    let compact = tier == LayoutTier::Compact;
    let min_spark_w = if compact { 240.0 } else { 300.0 };

    let header = row()
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
                .gap(4.0)
                .child(
                    text(s.tr(TextKey::HardwareTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(s.tr(TextKey::HardwareSub))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(action_button(
            s.tr(TextKey::RefreshHardwareBtn),
            p.btn_secondary,
            p.text_primary,
            p,
            |s| s.refresh_platform_inventory(),
        ));

    let telemetry_card = if let Some(ref m) = s.realtime_metrics {
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
                .min_w(min_spark_w),
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
                .min_w(min_spark_w),
            )
    } else {
        section_card(
            s.tr(TextKey::CpuSparklineTitle),
            text(s.tr(TextKey::Scanning))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
            p,
        )
    };

    let cpu_card = if let Some(cpu) = &s.cpu_info {
        let hybrid_note = if cpu.has_hybrid_topology {
            format!(
                "{} ({}P / {}E)",
                s.tr(TextKey::HybridTopology),
                cpu.p_core_count,
                cpu.e_core_count
            )
        } else {
            s.tr(TextKey::StandardTopology).to_string()
        };

        let mut cpu_body = col()
            .w_full()
            .min_w(0.0)
            .gap(6.0)
            .child(kv_row(s.tr(TextKey::VendorLabel), &cpu.vendor, compact, p))
            .child(kv_row(s.tr(TextKey::ModelLabel), &cpu.model_name, compact, p))
            .child(kv_row(
                s.tr(TextKey::PhysicalCoresLabel),
                cpu.physical_cores.to_string(),
                compact,
                p,
            ))
            .child(kv_row(
                s.tr(TextKey::LogicalProcessorsLabel),
                cpu.logical_cores.to_string(),
                compact,
                p,
            ))
            .child(kv_row(s.tr(TextKey::TopologyLabel), hybrid_note, compact, p));

        if let Some(ref m) = s.realtime_metrics {
            if !m.core_usages.is_empty() {
                let mut core_strip = row().w_full().min_w(0.0).flex_wrap().gaps(4.0, 6.0);
                for (i, u) in m.core_usages.iter().enumerate() {
                    let clean_u = if u.is_finite() { u.clamp(0.0, 100.0) } else { 0.0 };
                    let c_color = if clean_u > 80.0 {
                        p.danger
                    } else if clean_u > 45.0 {
                        p.warning
                    } else {
                        p.accent
                    };
                    core_strip = core_strip.child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .px(6.0)
                            .py(2.0)
                            .radius(4.0)
                            .fill(p.bg_primary)
                            .border(1.0, p.border)
                            .child(
                                text(format!("C{}: {:.0}%", i, clean_u))
                                    .font(rune::tok::type_::LABEL)
                                    .color(c_color),
                            ),
                    );
                }
                cpu_body = cpu_body
                    .child(
                        text(s.tr(TextKey::LiveCores))
                            .font(rune::tok::type_::LABEL)
                            .color(p.text_muted)
                            .shrink()
                            .wrap(),
                    )
                    .child(core_strip);
            }
        }

        section_card(s.tr(TextKey::CpuCardTitle), cpu_body, p)
    } else {
        section_card(
            s.tr(TextKey::CpuCardTitle),
            text(s.tr(TextKey::Scanning))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
            p,
        )
    };

    let latency_card = {
        let mut lat_body = col().w_full().min_w(0.0).gap(6.0);
        if let Some(ref m) = s.realtime_metrics {
            lat_body = lat_body
                .child(kv_row(s.tr(TextKey::TimerResolution), m.timer_res_ms_display(), compact, p))
                .child(kv_row(s.tr(TextKey::RamUsage), m.ram_display(), compact, p))
                .child(kv_row(
                    s.tr(TextKey::BackgroundProcesses),
                    m.process_count.to_string(),
                    compact,
                    p,
                ))
                .child(kv_row(s.tr(TextKey::ThreadsLabel), m.thread_count.to_string(), compact, p));
        }

        if let Some(ref snap) = s.latency {
            lat_body = lat_body
                .child(el().w_full().h(1.0).fill(p.border))
                .child(
                    text(format!(
                        "{} ({})",
                        s.tr(TextKey::LatencySnapshotTitle),
                        snap.timestamp.format("%H:%M:%S UTC")
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.accent)
                    .shrink()
                    .wrap(),
                )
                .child(kv_row(s.tr(TextKey::TimerResolution), snap.timer_resolution_display(), compact, p))
                .child(kv_row(s.tr(TextKey::RamUsage), snap.ram_display(), compact, p))
                .child(kv_row(
                    s.tr(TextKey::BackgroundProcesses),
                    format!("{} · {} {}", snap.process_count, snap.thread_count, s.tr(TextKey::ThreadsLabel)),
                    compact,
                    p,
                ));
        }

        if let Some(ref bench) = s.latest_latency_bench {
            lat_body = lat_body.child(kv_row(
                s.tr(TextKey::DpcJitterPeak),
                format!("{} µs ({:.3} ms)", bench.dpc.peak_jitter_us, bench.dpc.timer_resolution_ms),
                compact,
                p,
            ));
        }

        if s.realtime_metrics.is_none() && s.latency.is_none() {
            lat_body = lat_body.child(
                text(s.tr(TextKey::Scanning))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            );
        }

        lat_body = lat_body.child(
            row().w_full().min_w(0.0).flex_wrap().pt(4.0).child(action_button(
                s.tr(TextKey::RefreshLatencyBtn),
                p.btn_secondary,
                p.text_primary,
                p,
                |s| s.refresh_latency(),
            )),
        );
        section_card(s.tr(TextKey::LatencyCardTitle), lat_body, p)
    };

    let gpu_nav_card = section_card(
        s.tr(TextKey::GpuCardTitle),
        col()
            .w_full()
            .min_w(0.0)
            .gap(8.0)
            .child(
                text(s.tr(TextKey::GpuTuningSub))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_secondary)
                    .shrink()
                    .wrap(),
            )
            .child(
                row().w_full().min_w(0.0).flex_wrap().pt(2.0).child(action_button(
                    s.tr(TextKey::GpuTuning),
                    p.accent,
                    Color::WHITE,
                    p,
                    |s| s.navigate(ActiveView::GpuTuning),
                )),
            ),
        p,
    );

    let security_card = if let Some(ref sec) = s.platform_security {
        let sb_txt = match sec.secure_boot_enabled {
            Some(true) => s.tr(TextKey::StateEnabled),
            Some(false) => s.tr(TextKey::StateDisabled),
            None => s.tr(TextKey::UnknownLabel),
        };
        let vbs_txt = format!(
            "VBS: {} · HVCI: {}",
            if sec.vbs_enabled { s.tr(TextKey::StateEnabled) } else { s.tr(TextKey::StateDisabled) },
            if sec.hvci_enabled { s.tr(TextKey::StateEnabled) } else { s.tr(TextKey::StateDisabled) }
        );
        section_card(
            s.tr(TextKey::PlatformSecurityTitle),
            col()
                .w_full()
                .min_w(0.0)
                .gap(6.0)
                .child(kv_row(s.tr(TextKey::FirmwareLabel), &sec.firmware_mode, compact, p))
                .child(kv_row(s.tr(TextKey::SecureBootLabel), sb_txt, compact, p))
                .child(kv_row(s.tr(TextKey::VbsHvciLabel), vbs_txt, compact, p))
                .child(kv_row(s.tr(TextKey::BoardBiosLabel), format!("{} ({})", sec.motherboard, sec.bios_version), compact, p))
                .child(kv_row(s.tr(TextKey::UptimeLabel), format!("{:.1} {}", sec.uptime_hours, s.tr(TextKey::HoursUnit)), compact, p)),
            p,
        )
    } else {
        section_card(
            s.tr(TextKey::PlatformSecurityTitle),
            text(s.tr(TextKey::Scanning))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
            p,
        )
    };

    let dimms_card = {
        let mut dimm_body = col().w_full().min_w(0.0).gap(6.0);
        if s.ram_dimms.is_empty() {
            dimm_body = dimm_body.child(
                text(
                    s.platform_security
                        .as_ref()
                        .map(|sec| sec.ram_summary.clone())
                        .unwrap_or_else(|| "—".to_string()),
                )
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
            );
        } else {
            for (idx, d) in s.ram_dimms.iter().enumerate() {
                dimm_body = dimm_body.child(
                    row()
                        .key(format!("dimm_{}", idx))
                        .w_full()
                        .min_w(0.0)
                        .padding(8.0)
                        .radius(6.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .flex_wrap()
                        .justify_between()
                        .gaps(6.0, 10.0)
                        .child(
                            text(format!("{} · {} ({})", d.locator, d.manufacturer, d.ddr_type))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_primary)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(format!("{:.1} GB @ {} MT/s", d.capacity_mb as f64 / 1024.0, d.speed_mts))
                                .font(rune::tok::type_::LABEL)
                                .color(p.accent)
                                .shrink()
                                .wrap(),
                        ),
                );
            }
        }
        section_card(s.tr(TextKey::MemoryDimmsTitle), dimm_body, p)
    };

    let storage_card = {
        let mut st_body = col().w_full().min_w(0.0).gap(6.0);
        if s.storage_volumes.is_empty() {
            st_body = st_body.child(
                text(s.tr(TextKey::Scanning))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            );
        }
        for v in &s.storage_volumes {
            st_body = st_body.child(
                row()
                    .key(format!("vol_{}", v.drive_letter))
                    .w_full()
                    .min_w(0.0)
                    .padding(8.0)
                    .radius(6.0)
                    .fill(p.bg_primary)
                    .border(1.0, p.border)
                    .flex_wrap()
                    .justify_between()
                    .gaps(6.0, 10.0)
                    .child(
                        text(format!("{} {} ({} / {})", v.drive_letter, v.label, v.media_kind.as_label(), v.file_system))
                            .font(rune::tok::type_::BODY)
                            .color(p.text_primary)
                            .shrink()
                            .wrap(),
                    )
                    .child(
                        text(format!(
                            "{:.0} / {:.0} GB {} ({:.0}% {})",
                            v.free_gb,
                            v.total_gb,
                            s.tr(TextKey::FreeLabel),
                            v.used_pct,
                            s.tr(TextKey::UsedLabel)
                        ))
                            .font(rune::tok::type_::LABEL)
                            .color(if v.used_pct > 88.0 { p.warning } else { p.success })
                            .shrink()
                            .wrap(),
                    ),
            );
        }
        section_card(s.tr(TextKey::StorageDrivesTitle), st_body, p)
    };

    let power_card = {
        let mut scheme_row = row().w_full().min_w(0.0).min_h(32.0).flex_wrap().gaps(6.0, 6.0);
        if s.power_schemes.is_empty() {
            scheme_row = scheme_row.child(
                text(s.tr(TextKey::Scanning))
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
            );
        }
        for sc in &s.power_schemes {
            let guid = sc.guid.clone();
            let norm_guid = sc.guid.to_ascii_lowercase();
            let display_name = match norm_guid.as_str() {
                "381b4222-f694-41f0-9685-ff5bb260df2e" => s.tr_v(ViewTextKey::PowerPlanBalanced),
                "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c" => s.tr_v(ViewTextKey::PowerPlanHighPerf),
                "a1841308-3541-4fab-bc81-f71556f20b4a" => s.tr_v(ViewTextKey::PowerPlanPowerSaver),
                "e9a42b02-d5df-448d-aa00-03f14749eb61" => s.tr_v(ViewTextKey::PowerPlanUltimate),
                _ => {
                    if sc.friendly_name.contains("WinBlast") {
                        s.tr_v(ViewTextKey::PowerPlanWinBlastUltimate)
                    } else {
                        &sc.friendly_name
                    }
                }
            };
            scheme_row = scheme_row.child(action_button_guarded(
                format!("{}{}", if sc.is_active { "✔ " } else { "" }, display_name),
                if sc.is_active { p.accent } else { p.btn_secondary },
                if sc.is_active { Color::WHITE } else { p.text_primary },
                !s.power_scheme_inflight && !s.is_rolling_back,
                p,
                move |s| s.set_active_power_plan(guid.clone()),
            ));
        }

        let hib_enabled = s.hibernation_status.hibernation_enabled;
        let hib_note = format!(
            "{} ({}: {}, S0ix: {})",
            if hib_enabled { s.tr(TextKey::StateEnabled) } else { s.tr(TextKey::StateDisabled) },
            s.tr(TextKey::FastStartupLabel),
            if s.hibernation_status.fast_startup_enabled { s.tr(TextKey::StatusOn) } else { s.tr(TextKey::StatusOff) },
            if s.hibernation_status.modern_standby_s0ix { "S0ix" } else { "S3" }
        );

        let body = col()
            .w_full()
            .min_w(0.0)
            .gap(12.0)
            .child(
                col()
                    .w_full()
                    .min_w(0.0)
                    .gap(8.0)
                    .child(scheme_row),
            )
            .child(el().w_full().h(1.0).fill(p.border.alpha(0.5)))
            .child(kv_row(s.tr(TextKey::HibernationLabel), hib_note, compact, p))
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .gaps(8.0, 8.0)
                    .child(action_button_guarded(
                        if s.power_scheme_inflight {
                            s.tr(TextKey::BusyWorking)
                        } else {
                            s.tr(TextKey::UltimatePowerPlanBtn)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.power_scheme_inflight && !s.is_rolling_back,
                        p,
                        |s| s.activate_winblast_ultimate_power_plan(),
                    ))
                    .child(action_button_guarded(
                        if s.hibernation_inflight {
                            s.tr(TextKey::BusyWorking).to_string()
                        } else {
                            format!(
                                "{}: {}",
                                s.tr(TextKey::HibernationLabel),
                                if hib_enabled { s.tr(TextKey::ToggleDisable) } else { s.tr(TextKey::ToggleEnable) }
                            )
                        },
                        p.btn_secondary,
                        p.text_primary,
                        !s.hibernation_inflight && !s.is_rolling_back,
                        p,
                        move |s| s.toggle_hibernation(!hib_enabled),
                    ))
                    .child(action_button_guarded(
                        if s.usb_power_inflight {
                            s.tr(TextKey::BusyWorking).to_string()
                        } else {
                            format!("{} ({})", s.tr(TextKey::UsbPowerLabel), s.tr(TextKey::ToggleDisable))
                        },
                        p.btn_secondary,
                        p.text_primary,
                        !s.usb_power_inflight && !s.is_rolling_back,
                        p,
                        |s| s.toggle_usb_power_saving(true),
                    ))
                    .child(action_button_guarded(
                        if s.usb_power_inflight {
                            s.tr(TextKey::BusyWorking).to_string()
                        } else {
                            format!("{} ({})", s.tr(TextKey::UsbPowerLabel), s.tr(TextKey::ToggleEnable))
                        },
                        p.btn_secondary,
                        p.text_primary,
                        !s.usb_power_inflight && !s.is_rolling_back,
                        p,
                        |s| s.toggle_usb_power_saving(false),
                    )),
            );
        section_card(s.tr(TextKey::PowerMgrTitle), body, p)
    };

    let pad = if compact { 10.0 } else { p.card_padding };
    col()
        .key("hardware_view_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .pr(14.0)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header)
        .child(telemetry_card)
        .child(cpu_card)
        .child(latency_card)
        .child(power_card)
        .child(security_card)
        .child(dimms_card)
        .child(storage_card)
        .child(gpu_nav_card)
        .winblast_scroll(p)
}
