//! CPU Core Parking Management & Real-Time Logical Core Topology View in Rune.
//!
//! Uses pure `.flex_wrap()` grid & fractional `.w_frac(...)` progress indicators so core topology
//! cards scale smoothly from 420x400 to 4K 3840x2160 without pixel overflow.

use rune::color::Color;
use rune::{col, el, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::ui::widgets::button::{action_button, action_button_guarded};
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

/// Renders the CPU Core Parking Manager view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let parking = &s.core_parking;
    let min_core_w = if tier == LayoutTier::Compact { 170.0 } else { 210.0 };

    let header_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
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
                    text(s.tr(TextKey::CoreParkingTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(s.tr(TextKey::CoreParkingSub))
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
                .px(12.0)
                .py(6.0)
                .radius(6.0)
                .fill(if parking.is_fully_unparked {
                    p.success.alpha(0.18)
                } else {
                    p.warning.alpha(0.18)
                })
                .border(
                    1.0,
                    if parking.is_fully_unparked {
                        p.success.alpha(0.4)
                    } else {
                        p.warning.alpha(0.4)
                    },
                )
                .child(
                    text(if parking.is_fully_unparked {
                        s.tr(TextKey::CoreParkingStatusUnparked)
                    } else {
                        s.tr(TextKey::CoreParkingStatusParked)
                    })
                    .font(rune::tok::type_::LABEL)
                    .color(if parking.is_fully_unparked {
                        p.success
                    } else {
                        p.warning
                    })
                    .shrink()
                    .wrap(),
                ),
        );

    let actions_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding.min(12.0))
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
                .child(action_button_guarded(
                    if s.core_parking_inflight {
                        s.tr(TextKey::BusyWorking)
                    } else {
                        s.tr(TextKey::UnparkAllCores)
                    },
                    p.success,
                    Color::WHITE,
                    !s.core_parking_inflight && !s.is_rolling_back,
                    p,
                    |s| s.unpark_all_cores(),
                ))
                .child(action_button_guarded(
                    s.tr(TextKey::RestoreStockParking),
                    p.btn_secondary,
                    p.text_primary,
                    !s.core_parking_inflight && !s.is_rolling_back,
                    p,
                    |s| s.restore_stock_parking(),
                )),
        )
        .child(action_button(
            s.tr(TextKey::RefreshCoresBtn),
            p.btn_secondary,
            p.text_primary,
            p,
            |s| s.refresh_core_parking(),
        ));

    let stats_bar = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .gaps(10.0, 10.0)
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::TotalLogicalCores))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{}", parking.total_cores))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::ActiveUnparked))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{}", parking.unparked_count))
                        .font(rune::tok::type_::TITLE)
                        .color(p.success),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::ParkedSleeping))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!("{}", parking.parked_count))
                        .font(rune::tok::type_::TITLE)
                        .color(if parking.parked_count > 0 {
                            p.warning
                        } else {
                            p.text_muted
                        }),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(150.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .gap(2.0)
                .child(
                    text(s.tr(TextKey::MinCoresAcDc))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                )
                .child(
                    text(format!(
                        "{}% / {}%",
                        parking.min_cores_ac_pct, parking.min_cores_dc_pct
                    ))
                    .font(rune::tok::type_::TITLE)
                    .color(p.accent),
                ),
        );

    let mut core_cards: Vec<El> = Vec::with_capacity(parking.cores.len());
    for core in &parking.cores {
        let is_parked = core.is_parked;
        let status_color = if is_parked { p.warning } else { p.success };
        let status_text = if is_parked {
            s.tr(TextKey::CoreStatusParked)
        } else {
            s.tr(TextKey::CoreStatusUnparked)
        };
        let usage_frac = (core.usage_pct / 100.0).clamp(0.02, 1.0);

        core_cards.push(
            col()
                .grow(1.0)
                .min_w(min_core_w)
                .max_w(380.0)
                .padding(10.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, if is_parked { p.border } else { p.accent.alpha(0.35) })
                .hover(p.bg_card_hover)
                .motion(rune::tok::motion::SNAPPY)
                .gap(5.0)
                .child(
                    row()
                        .w_full()
                        .min_w(0.0)
                        .flex_wrap()
                        .justify_between()
                        .items_center()
                        .gap(6.0)
                        .child({
                            let type_label = match core.core_type_label.as_str() {
                                "P-Core" => s.tr(TextKey::PCoreLabel),
                                "E-Core" => s.tr(TextKey::ECoreLabel),
                                _ => s.tr(TextKey::CoreLabel),
                            };
                            text(format!("{} #{}", type_label, core.core_index))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary)
                                .shrink()
                                .truncate()
                        })
                        .child(
                            text(status_text)
                                .font(rune::tok::type_::LABEL)
                                .color(status_color)
                                .shrink()
                                .wrap(),
                        ),
                )
                .child(
                    text(format!("{:.1}%", core.usage_pct))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted),
                )
                .child(
                    el()
                        .w_full()
                        .h(4.0)
                        .radius(2.0)
                        .fill(p.bg_primary)
                        .child(
                            el()
                                .w_frac(usage_frac)
                                .h_full()
                                .radius(2.0)
                                .fill(if is_parked { p.warning } else { p.accent })
                                .motion(rune::tok::motion::SNAPPY),
                        ),
                ),
        );
    }

    col()
        .key("cpu_parking_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(14.0)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header_card)
        .child(actions_card)
        .child(stats_bar)
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .flex_wrap()
                .gaps(8.0, 8.0)
                .children(core_cards),
        )
        .winblast_scroll(p)
}
