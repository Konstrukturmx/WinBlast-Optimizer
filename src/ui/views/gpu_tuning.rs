//! Dedicated GPU Tuning & Vendor Silicon Optimization View in Rune.
//!
//! Pure CSS Flexbox `.flex_wrap()` grid scales fluidly across 1, 2, 3, or 4 columns
//! from 420x400 to 4K 3840x2160 with full localization, HAGS/MPO/MSI controls,
//! and unified palette-aware overlays.

use rune::color::Color;
use rune::{col, el, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::core::risk::TweakDefinition;
use crate::hardware::gpu::GpuVendor;
use crate::scanner::{ScanResult, TweakState};
use crate::ui::theme::ThemePalette;
use crate::ui::widgets::button::{
    action_button, action_button_guarded, kv_row, pill_badge, section_card, switch_toggle,
    tweak_state_meta,
};
use crate::ui::widgets::risk_badge;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

/// Renders a locked/incompatible GPU card when host silicon does not match target vendor.
fn render_incompatible_card(
    tweak: &TweakDefinition,
    req_vendor: &str,
    s: &AppState,
    p: &ThemePalette,
    min_card_w: f32,
) -> El {
    let amber_warning = p.warning;
    let amber_box_bg = p.warning.alpha(0.10);
    let amber_border = p.warning.alpha(0.32);

    let tweak_id_hide = tweak.id.clone();
    let tweak_id_inspect = tweak.id.clone();

    col()
        .grow(1.0)
        .min_w(min_card_w)
        .max_w(540.0)
        .padding(14.0)
        .radius(8.0)
        .fill(p.bg_card.alpha(0.75))
        .border(1.0, p.border.alpha(0.7))
        .hover(p.bg_card_hover)
        .motion(rune::tok::motion::SNAPPY)
        .gap(10.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gap(6.0)
                .items_center()
                .child(
                    text("⚠")
                        .font(rune::tok::type_::LABEL)
                        .color(amber_warning),
                )
                .child(
                    text(s.tr(TextKey::NotSupportedOnSystem))
                        .font(rune::tok::type_::SECTION)
                        .color(p.text_primary)
                        .grow(1.0)
                        .min_w(0.0)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .gap(2.0)
                .child(
                    text(tweak.id.clone())
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted.alpha(0.75))
                        .shrink()
                        .wrap_anywhere(),
                )
                .child(
                    text(s.tweak_title(&tweak.id, &tweak.title).to_string())
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                ),
        )
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .min_h(76.0)
                .padding(10.0)
                .radius(6.0)
                .fill(amber_box_bg)
                .border(1.0, amber_border)
                .gap(3.0)
                .child(
                    row()
                        .w_full()
                        .min_w(0.0)
                        .flex_wrap()
                        .gap(4.0)
                        .items_center()
                        .child(text("⚠").font(rune::tok::type_::LABEL).color(amber_warning))
                        .child(
                            text(s.tr(TextKey::ConditionFailed))
                                .font(rune::tok::type_::LABEL)
                                .color(amber_warning)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(
                    text(format!("{} {}", req_vendor, s.tr(TextKey::GpuRequiredAlert)))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_primary)
                        .shrink()
                        .wrap()
                        .wrap_anywhere(),
                ),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .pt(2.0)
                .gap(8.0)
                .items_center()
                .child(action_button(
                    s.tr(TextKey::Hide),
                    p.btn_secondary,
                    p.text_secondary,
                    p,
                    move |s| s.toggle_hide_tweak(tweak_id_hide.clone()),
                ))
                .child(action_button(
                    format!("ⓘ {}", s.tr(TextKey::WhatIsThis)),
                    p.btn_secondary,
                    p.text_secondary,
                    p,
                    move |s| s.inspect_tweak(&tweak_id_inspect),
                )),
        )
}

/// Renders a compatible GPU tweak card with active toggle and risk badge.
fn render_compatible_card(
    tweak: &TweakDefinition,
    scan_res: Option<&ScanResult>,
    is_active: bool,
    in_flight: bool,
    s: &AppState,
    p: &ThemePalette,
    min_card_w: f32,
) -> El {
    let vendor_label = tweak.gpu_vendor.as_deref().unwrap_or("GPU");

    let (state_text, _) = match scan_res {
        Some(sr) => tweak_state_meta(&sr.current_state, s),
        None => (s.tr(TextKey::StatusReady), p.text_muted),
    };
    let is_not_applicable = scan_res.is_some_and(|sr| sr.current_state == TweakState::NotApplicable);
    let switch_ctrl = if is_not_applicable {
        pill_badge(s.tr(TextKey::StatusNotApplicable), p.text_muted, p)
    } else {
        switch_toggle(is_active, in_flight, tweak.id.clone(), p)
    };

    let tweak_id_diff = tweak.id.clone();
    let tweak_id_hide = tweak.id.clone();

    col()
        .grow(1.0)
        .min_w(min_card_w)
        .max_w(540.0)
        .padding(14.0)
        .radius(8.0)
        .fill(p.bg_card)
        .border(1.0, if is_active { p.accent.alpha(0.6) } else { p.border })
        .hover(p.bg_card_hover)
        .motion(rune::tok::motion::SNAPPY)
        .gap(10.0)
        .child(
            text(tweak.id.clone())
                .font(rune::tok::type_::LABEL)
                .color(p.text_muted.alpha(0.75))
                .shrink()
                .wrap_anywhere(),
        )
        .child(
            text(s.tweak_title(&tweak.id, &tweak.title).to_string())
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap()
                .wrap_anywhere(),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gap(6.0)
                .items_center()
                .child(
                    row()
                        .min_w(0.0)
                        .shrink()
                        .px(6.0)
                        .py(2.0)
                        .radius(4.0)
                        .fill(p.accent.alpha(0.18))
                        .border(1.0, p.accent.alpha(0.45))
                        .child(
                            text(format!("🖵 {}", vendor_label))
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
                        .px(6.0)
                        .py(2.0)
                        .radius(4.0)
                        .fill(p.bg_primary)
                        .border(1.0, p.border)
                        .child({
                            let sub_cat_display = tweak
                                .sub_category
                                .as_deref()
                                .map(|sc| s.tweak_sub_category(sc))
                                .unwrap_or("GPU");
                            text(format!("⚡ {}", sub_cat_display))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                        }),
                ),
        )
        .child(
            text(s.tweak_description(&tweak.id, &tweak.description).to_string())
                .font(rune::tok::type_::BODY)
                .color(p.text_secondary)
                .shrink()
                .wrap()
                .wrap_anywhere()
                .min_h(48.0),
        )
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .pt(4.0)
                .items_center()
                .gaps(6.0, 8.0)
                .child(risk_badge::view_localized(&tweak.valuation, p, s))
                .child(
                    text(state_text)
                        .font(rune::tok::type_::LABEL)
                        .color(if is_not_applicable {
                            p.text_muted
                        } else if is_active {
                            p.success
                        } else {
                            p.text_muted
                        })
                        .grow(1.0)
                        .min_w(80.0)
                        .shrink()
                        .wrap(),
                )
                .child(switch_ctrl)
                .child(action_button(
                    "< / >",
                    p.btn_secondary,
                    p.text_muted,
                    p,
                    move |s| s.inspect_tweak(&tweak_id_diff),
                ))
                .child(action_button(
                    "👁",
                    p.btn_secondary,
                    p.text_muted,
                    p,
                    move |s| s.toggle_hide_tweak(tweak_id_hide.clone()),
                )),
        )
}

/// Renders the responsive GPU Tuning view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let compact = tier == LayoutTier::Compact;
    let detected_vendors = s.detected_gpu_vendors();
    let min_card_w = if compact { 240.0 } else { 300.0 };

    let header = col()
        .w_full()
        .min_w(0.0)
        .gap(4.0)
        .child(
            text(s.tr(TextKey::GpuTuningTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        )
        .child(
            text(s.tr(TextKey::GpuTuningSub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );

    let mut gpu_summary_body = col().gap(8.0).w_full().min_w(0.0);
    if s.gpus.is_empty() {
        gpu_summary_body = gpu_summary_body.child(
            text(s.tr(TextKey::NoGpuDetected))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );
    } else {
        for (idx, gpu) in s.gpus.iter().enumerate() {
            let vendor_color = match gpu.vendor {
                GpuVendor::Nvidia => p.accent,
                GpuVendor::Amd => p.danger,
                GpuVendor::Intel => p.sparkline_cpu,
                GpuVendor::Unknown => p.text_muted,
            };

            gpu_summary_body = gpu_summary_body.child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .items_center()
                    .gaps(6.0, 10.0)
                    .child(text("🟢").font(rune::tok::type_::LABEL))
                    .child(
                        text(format!("#{} {}", idx, gpu.name))
                            .font(rune::tok::type_::SECTION)
                            .color(p.text_primary)
                            .grow(1.0)
                            .min_w(140.0)
                            .shrink()
                            .wrap()
                            .wrap_anywhere(),
                    )
                    .child(
                        row()
                            .min_w(0.0)
                            .shrink()
                            .px(8.0)
                            .py(3.0)
                            .radius(4.0)
                            .fill(vendor_color.alpha(0.18))
                            .border(1.0, vendor_color.alpha(0.5))
                            .child(
                                text(format!("{} (✔)", gpu.vendor.as_str()))
                                    .font(rune::tok::type_::LABEL)
                                    .color(vendor_color)
                                    .shrink()
                                    .wrap(),
                            ),
                    ),
            );
        }
    }
    let hardware_banner = section_card(s.tr(TextKey::DetectedGpuHeader), gpu_summary_body, p);

    let hags_enabled = s.hags_enabled;
    let mpo_disabled = s.mpo_disabled;
    let hags_supported = s.os_info.as_ref().is_none_or(|o| o.supports_hags());

    let hags_status_text = if !hags_supported {
        s.tr(TextKey::HagsUnsupportedNote)
    } else if hags_enabled {
        s.tr(TextKey::HagsActiveStatus)
    } else {
        s.tr(TextKey::HagsDisabledStatus)
    };
    let hags_status_color = if !hags_supported {
        p.text_muted
    } else if hags_enabled {
        p.success
    } else {
        p.warning
    };
    let hags_btn = if !hags_supported {
        action_button_guarded(
            s.tr(TextKey::StatusNotApplicable),
            p.btn_secondary,
            p.text_muted,
            false,
            p,
            |_| {},
        )
    } else {
        action_button_guarded(
            if s.hags_inflight {
                s.tr(TextKey::BusyWorking)
            } else if hags_enabled {
                s.tr(TextKey::DisableHags)
            } else {
                s.tr(TextKey::EnableHags)
            },
            if hags_enabled { p.btn_secondary } else { p.accent },
            if hags_enabled { p.text_primary } else { Color::WHITE },
            !s.hags_inflight && !s.is_rolling_back,
            p,
            move |s| s.toggle_hags(!hags_enabled),
        )
    };

    let display_engine_card = section_card(
        s.tr(TextKey::DisplayEngineTitle),
        col()
            .w_full()
            .min_w(0.0)
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
                                text(s.tr(TextKey::HagsTitle))
                                    .font(rune::tok::type_::SECTION)
                                    .color(p.text_primary)
                                    .shrink()
                                    .wrap(),
                            )
                            .child(
                                text(hags_status_text)
                                    .font(rune::tok::type_::LABEL)
                                    .color(hags_status_color)
                                    .shrink()
                                    .wrap(),
                            ),
                    )
                    .child(hags_btn),
            )
            .child(el().w_full().h(1.0).fill(p.border))
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
                                text(s.tr(TextKey::MpoTitle))
                                    .font(rune::tok::type_::SECTION)
                                    .color(p.text_primary)
                                    .shrink()
                                    .wrap(),
                            )
                            .child(
                                text(if mpo_disabled {
                                    s.tr(TextKey::MpoFixedStatus)
                                } else {
                                    s.tr(TextKey::MpoEnabledStatus)
                                })
                                .font(rune::tok::type_::LABEL)
                                .color(if mpo_disabled { p.success } else { p.warning })
                                .shrink()
                                .wrap(),
                            ),
                    )
                    .child(action_button_guarded(
                        if s.mpo_inflight {
                            s.tr(TextKey::BusyWorking)
                        } else if mpo_disabled {
                            s.tr(TextKey::RestoreMpoDefault)
                        } else {
                            s.tr(TextKey::DisableMpoFix)
                        },
                        if mpo_disabled { p.btn_secondary } else { p.accent },
                        if mpo_disabled { p.text_primary } else { Color::WHITE },
                        !s.mpo_inflight && !s.is_rolling_back,
                        p,
                        move |s| s.toggle_mpo(!mpo_disabled),
                    )),
            ),
        p,
    );

    let mut msi_section = col()
        .gap(10.0)
        .w_full()
        .min_w(0.0)
        .child(
            text(s.tr(TextKey::PcieMsiTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.accent)
                .shrink()
                .wrap(),
        );

    if s.gpus.is_empty() {
        msi_section = msi_section.child(
            text(s.tr(TextKey::NoPcieGpuDetected))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );
    } else {
        for (i, gpu) in s.gpus.iter().enumerate() {
            let msi_status_text = if gpu.msi_enabled {
                s.tr(TextKey::MsiEnabledStatus)
            } else {
                s.tr(TextKey::MsiDisabledStatus)
            };
            let msi_color = if gpu.msi_enabled { p.success } else { p.warning };

            let pnp_id = gpu.pnp_instance_id.clone();
            let pnp_valid = !pnp_id.trim().is_empty();
            let msi_busy = s.gpu_msi_inflight.contains(&pnp_id);
            let can_edit_msi = pnp_valid && !msi_busy && !s.is_rolling_back;
            let msi_en = gpu.msi_enabled;
            let current_priority = gpu.interrupt_priority;

            let p1 = pnp_id.clone();
            let p2 = pnp_id.clone();
            let p3 = pnp_id.clone();
            let p4 = pnp_id.clone();

            let priority_row = row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(6.0, 8.0)
                .items_center()
                .child(
                    text(format!(
                        "{}: {}",
                        s.tr(TextKey::InterruptPriority),
                        match current_priority {
                            1 => s.tr(TextKey::PriorityLow),
                            3 => s.tr(TextKey::PriorityHigh),
                            _ => s.tr(TextKey::PriorityNormal),
                        }
                    ))
                    .font(rune::tok::type_::LABEL)
                    .color(p.text_muted)
                    .shrink()
                    .wrap(),
                )
                .child(action_button_guarded(
                    s.tr(TextKey::PriorityLow),
                    if current_priority == 1 { p.accent } else { p.btn_secondary },
                    if current_priority == 1 { Color::WHITE } else { p.text_primary },
                    can_edit_msi,
                    p,
                    move |s| s.toggle_gpu_msi(p1.clone(), msi_en, 1),
                ))
                .child(action_button_guarded(
                    s.tr(TextKey::PriorityNormal),
                    if current_priority == 2 { p.accent } else { p.btn_secondary },
                    if current_priority == 2 { Color::WHITE } else { p.text_primary },
                    can_edit_msi,
                    p,
                    move |s| s.toggle_gpu_msi(p2.clone(), msi_en, 2),
                ))
                .child(action_button_guarded(
                    s.tr(TextKey::PriorityHigh),
                    if current_priority == 3 { p.accent } else { p.btn_secondary },
                    if current_priority == 3 { Color::WHITE } else { p.text_primary },
                    can_edit_msi,
                    p,
                    move |s| s.toggle_gpu_msi(p3.clone(), msi_en, 3),
                ));

            let pnp_display = if pnp_valid {
                gpu.pnp_instance_id.as_str()
            } else {
                s.tr(TextKey::StatusNotApplicable)
            };

            let gpu_body = col()
                .w_full()
                .min_w(0.0)
                .gap(8.0)
                .child(kv_row(s.tr(TextKey::AdapterName), gpu.name.clone(), compact, p))
                .child(kv_row(s.tr(TextKey::DevicePnpId), pnp_display, compact, p))
                .child(
                    row()
                        .w_full()
                        .min_w(0.0)
                        .flex_wrap()
                        .justify_between()
                        .items_center()
                        .gaps(8.0, 12.0)
                        .child(
                            text(msi_status_text)
                                .font(rune::tok::type_::LABEL)
                                .color(msi_color)
                                .grow(1.0)
                                .min_w(140.0)
                                .shrink()
                                .wrap(),
                        )
                        .child(action_button_guarded(
                            if msi_busy {
                                s.tr(TextKey::BusyWorking)
                            } else if !pnp_valid {
                                s.tr(TextKey::StatusNotApplicable)
                            } else if msi_en {
                                s.tr(TextKey::SwitchToLineIrq)
                            } else {
                                s.tr(TextKey::EnableMsiMode)
                            },
                            if msi_en { p.btn_secondary } else { p.accent },
                            if msi_en { p.text_primary } else { Color::WHITE },
                            can_edit_msi,
                            p,
                            move |s| s.toggle_gpu_msi(p4.clone(), !msi_en, current_priority),
                        )),
                )
                .child(priority_row);

            msi_section = msi_section.child(section_card(
                format!("{} #{}: {}", s.tr(TextKey::PcieAdapterPrefix), i, gpu.name),
                gpu_body,
                p,
            ));
        }
    }

    let mut vendor_tweaks: Vec<&TweakDefinition> = s
        .tweaks
        .iter()
        .filter(|t| {
            (t.category == "GPU Tuning" || t.gpu_vendor.is_some())
                && !s.hidden_tweaks.contains(&t.id)
        })
        .collect();

    const PREFERRED_GPU_ORDER: &[&str] = &[
        "4433ce93-b632-4d67-ae71-0241c31099c0",
        "8a4b4c4d-000e-42db-8bd7-293067f8fae7",
        "fa58802f-b98c-4e98-aa67-067c5a394296",
        "df8739ff-e322-4958-b369-df584bb9b0c1",
        "ebfb0ded-5d1a-4cd1-b5c0-4f7018dd6054",
        "a3836587-c461-4beb-9c4c-88493b36c138",
        "db379bb5-fc8c-4ad3-be32-11be09e76fba",
        "9e7702be-9e03-4c96-a316-c4e435cb993e",
        "5f1a2b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c",
    ];

    vendor_tweaks.sort_by_key(|t| {
        PREFERRED_GPU_ORDER
            .iter()
            .position(|&id| id == t.id)
            .unwrap_or(999)
    });

    let mut cards: Vec<El> = Vec::with_capacity(vendor_tweaks.len());
    for tweak in vendor_tweaks {
        let is_supported = tweak.is_gpu_vendor_compatible(&detected_vendors);
        if !is_supported {
            let req = tweak
                .gpu_vendor
                .as_deref()
                .unwrap_or_else(|| s.tr(TextKey::CompatibleLabel));
            cards.push(render_incompatible_card(tweak, req, s, p, min_card_w));
        } else {
            let scan_res = s.get_tweak_scan(&tweak.id);
            let is_active = s.is_switch_active(&tweak.id);
            let in_flight = s.in_flight_tweaks.contains(&tweak.id);
            cards.push(render_compatible_card(
                tweak,
                scan_res,
                is_active,
                in_flight,
                s,
                p,
                min_card_w,
            ));
        }
    }

    let vendor_cards_grid = col()
        .gap(12.0)
        .w_full()
        .min_w(0.0)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .justify_between()
                .items_center()
                .gap(8.0)
                .child(
                    text(s.tr(TextKey::VendorSiliconTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.accent)
                        .grow(1.0)
                        .min_w(0.0)
                        .shrink()
                        .wrap(),
                )
                .child(if !s.hidden_tweaks.is_empty() {
                    action_button(
                        format!("{} ({})", s.tr(TextKey::UnhideAll), s.hidden_tweaks.len()),
                        p.btn_secondary,
                        p.accent,
                        p,
                        |s| s.hidden_tweaks.clear(),
                    )
                } else {
                    el()
                }),
        )
        .child(
            // Pure Flex-Wrap Grid: 1 col (<680), 2 cols (680-1100), 3-4 cols (1440-3840)
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(12.0, 12.0)
                .children(cards),
        );

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    col()
        .key("gpu_tuning_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(14.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header)
        .child(hardware_banner)
        .child(display_engine_card)
        .child(msi_section)
        .child(vendor_cards_grid)
        .winblast_scroll(p)
}
