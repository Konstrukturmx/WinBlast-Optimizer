//! Privacy Shield in Rune — 100% localized telemetry auditor, hosts blocker,
//! and responsive switch matrix across all screen resolutions.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::{AppState, StagedTransition};
use crate::core::i18n::TextKey;
use crate::core::risk::RiskLevel;
use crate::scanner::{ScanResult, TweakState};
use crate::ui::widgets::button::{
    action_button, action_button_guarded, pill_badge, switch_toggle, tweak_state_meta,
};
use crate::ui::widgets::risk_badge;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

fn state_counts<'b>(results: impl Iterator<Item = &'b ScanResult>) -> (usize, usize, usize) {
    let mut optimized = 0;
    let mut not_optimized = 0;
    let mut partial = 0;
    for r in results {
        match r.current_state {
            TweakState::AlreadyOptimized => optimized += 1,
            TweakState::NotOptimized => not_optimized += 1,
            TweakState::PartiallyOptimized => partial += 1,
            _ => {}
        }
    }
    (optimized, not_optimized, partial)
}

/// Renders the responsive, fully localized Privacy Shield view.
pub fn view(s: &AppState, tier: LayoutTier) -> El {
    let p = s.palette();
    let is_privacy = |r: &&ScanResult| {
        crate::core::tweak_i18n::is_privacy_or_telemetry_category(&r.category)
    };

    let header = col()
        .w_full()
        .min_w(0.0)
        .gap(4.0)
        .child(
            text(s.tr(TextKey::PrivacyTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.text_primary)
                .shrink()
                .wrap(),
        )
        .child(
            text(s.tr(TextKey::PrivacySub))
                .font(rune::tok::type_::BODY)
                .color(p.text_muted)
                .shrink()
                .wrap(),
        );

    let (opt, not_opt, partial) = state_counts(s.scan_results.iter().filter(is_privacy));

    let stats_card = row()
        .w_full()
        .min_w(0.0)
        .padding(p.card_padding)
        .radius(p.card_radius)
        .fill(p.bg_card)
        .border(p.border_width, p.border)
        .flex_wrap()
        .gaps(12.0, 20.0)
        .child(
            col()
                .grow(1.0)
                .min_w(130.0)
                .gap(2.0)
                .center()
                .child(text(format!("{}", opt)).font(rune::tok::type_::TITLE).color(p.success))
                .child(
                    text(s.tr(TextKey::PrivacyBlockedStat))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(130.0)
                .gap(2.0)
                .center()
                .child(text(format!("{}", not_opt)).font(rune::tok::type_::TITLE).color(p.danger))
                .child(
                    text(s.tr(TextKey::PrivacyActiveStat))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(130.0)
                .gap(2.0)
                .center()
                .child(text(format!("{}", partial)).font(rune::tok::type_::TITLE).color(p.warning))
                .child(
                    text(s.tr(TextKey::PrivacyPartialStat))
                        .font(rune::tok::type_::LABEL)
                        .color(p.text_muted)
                        .shrink()
                        .wrap(),
                ),
        );

    let audit_header = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .justify_between()
        .items_center()
        .gaps(8.0, 12.0)
        .child(
            text(s.tr(TextKey::HostsAuditTitle))
                .font(rune::tok::type_::TITLE)
                .color(p.accent)
                .grow(1.0)
                .min_w(160.0)
                .shrink()
                .wrap(),
        )
        .child(action_button(
            s.tr(TextKey::RunAuditBtn),
            p.btn_secondary,
            p.text_primary,
            p,
            |s| s.run_telemetry_audit(),
        ));

    let audit_card = if let Some(aud) = &s.telemetry_audit {
        let hosts_status = format!(
            "{} {} ✔ / {} ⚠",
            s.tr(TextKey::HostsLabel),
            aud.blocked_hosts.len(),
            aud.unblocked_hosts.len()
        );
        let services_status = format!(
            "{}: {} {} / {} {}",
            s.tr(TextKey::SubTabServices),
            aud.active_services.len(),
            s.tr(TextKey::AuditActiveWord),
            aud.disabled_services.len(),
            s.tr(TextKey::StateDisabled)
        );

        col()
            .w_full()
            .min_w(0.0)
            .padding(p.card_padding)
            .radius(p.card_radius)
            .fill(p.bg_card)
            .border(p.border_width, p.border)
            .gap(10.0)
            .child(audit_header)
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .justify_between()
                    .items_center()
                    .gaps(8.0, 12.0)
                    .child(
                        text(hosts_status)
                            .font(rune::tok::type_::BODY)
                            .color(p.text_primary)
                            .grow(1.0)
                            .min_w(160.0)
                            .shrink()
                            .wrap(),
                    )
                    .child(action_button_guarded(
                        if s.telemetry_hosts_inflight {
                            s.tr(TextKey::BusyWorking)
                        } else {
                            s.tr(TextKey::BlockTelemetryHostsBtn)
                        },
                        p.accent,
                        Color::WHITE,
                        !s.telemetry_hosts_inflight && !s.is_rolling_back,
                        p,
                        |s| s.block_telemetry_hosts(),
                    )),
            )
            .child(
                row()
                    .w_full()
                    .min_w(0.0)
                    .flex_wrap()
                    .justify_between()
                    .items_center()
                    .gaps(8.0, 12.0)
                    .child(
                        text(services_status)
                            .font(rune::tok::type_::BODY)
                            .color(p.text_primary)
                            .grow(1.0)
                            .min_w(160.0)
                            .shrink()
                            .wrap(),
                    )
                    .child(action_button_guarded(
                        if s.telemetry_services_inflight {
                            s.tr(TextKey::BusyWorking)
                        } else {
                            s.tr(TextKey::NeutralizeServicesBtn)
                        },
                        p.danger,
                        Color::WHITE,
                        !s.telemetry_services_inflight && !s.is_rolling_back,
                        p,
                        |s| s.neutralize_telemetry_services(),
                    )),
            )
    } else {
        col()
            .w_full()
            .min_w(0.0)
            .padding(p.card_padding)
            .radius(p.card_radius)
            .fill(p.bg_card)
            .border(p.border_width, p.border)
            .gap(8.0)
            .child(audit_header)
    };

    let count_selected = s.selected_tweaks.len();
    let action_bar = row()
        .w_full()
        .min_w(0.0)
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
                    s.tr(TextKey::SelectAll),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| {
                        for r in &s.scan_results {
                            if r.current_state == TweakState::NotApplicable
                                || r.risk.stability_risk >= RiskLevel::High
                            {
                                continue;
                            }
                            if r.category.to_lowercase().contains("privacy")
                                || r.category.to_lowercase().contains("telemetry")
                            {
                                s.selected_tweaks.insert(r.tweak_id.clone());
                                s.staged_transitions
                                    .insert(r.tweak_id.clone(), StagedTransition::StageApply);
                            }
                        }
                        s.refresh_pending_actions();
                    },
                ))
                .child(action_button(
                    s.tr(TextKey::DeselectAll),
                    p.btn_secondary,
                    p.text_primary,
                    p,
                    |s| s.deselect_all(),
                )),
        )
        .child(action_button_guarded(
            if s.is_applying {
                s.tr(TextKey::BusyWorking).to_string()
            } else if count_selected == 0 {
                s.tr(TextKey::ApplySelected).to_string()
            } else {
                format!("{} ({})", s.tr(TextKey::ApplySelected), count_selected)
            },
            p.success,
            Color::WHITE,
            !s.is_applying && !s.is_rolling_back,
            p,
            |s| s.apply_selected(),
        ));

    let mut list = col().gap(6.0).w_full().min_w(0.0);
    let privacy_tweaks: Vec<&ScanResult> = s.scan_results.iter().filter(is_privacy).collect();

    if privacy_tweaks.is_empty() {
        list = list.child(
            text(if s.is_scanning {
                s.tr(TextKey::ScanningEvaluating)
            } else {
                s.tr(TextKey::NoEligibleTweaks)
            })
            .font(rune::tok::type_::BODY)
            .color(p.text_muted)
            .shrink()
            .wrap(),
        );
    } else {
        for sr in privacy_tweaks {
            let is_active = s.is_switch_active(&sr.tweak_id);
            let in_flight = s.in_flight_tweaks.contains(&sr.tweak_id);
            let (state_text, state_color) = tweak_state_meta(&sr.current_state, s);
            let switch_ctrl = if sr.current_state == TweakState::NotApplicable {
                pill_badge(s.tr(TextKey::StatusNotApplicable), p.text_muted, p)
            } else {
                switch_toggle(is_active, in_flight, sr.tweak_id.clone(), p)
            };

            let row_item = row()
                .w_full()
                .min_w(0.0)
                .padding(10.0)
                .radius(6.0)
                .fill(if is_active { p.bg_card_hover } else { p.bg_card })
                .border(
                    1.0,
                    if is_active { p.accent.alpha(0.35) } else { p.border },
                )
                .hover(p.bg_card_hover)
                .motion(rune::tok::motion::SNAPPY)
                .flex_wrap()
                .items_center()
                .gaps(8.0, 12.0)
                .child(switch_ctrl)
                .child(
                    col()
                        .grow(1.0)
                        .min_w(180.0)
                        .gap(2.0)
                        .child(
                            text(s.tweak_title(&sr.tweak_id, &sr.title))
                                .font(rune::tok::type_::SECTION)
                                .color(p.text_primary)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        )
                        .child(
                            text(s.tweak_category(&sr.category))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        ),
                )
                .child(risk_badge::view_localized(&sr.risk, p, s))
                .child(
                    col()
                        .shrink()
                        .min_w(105.0)
                        .gap(2.0)
                        .child(
                            text(state_text)
                                .font(rune::tok::type_::LABEL)
                                .color(state_color)
                                .shrink()
                                .wrap(),
                        )
                        .child(
                            text(s.scan_detail_text(sr))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_muted)
                                .shrink()
                                .wrap()
                                .wrap_anywhere(),
                        ),
                );

            list = list.child(row_item);
        }
    }

    let pad = if tier == LayoutTier::Compact { 10.0 } else { 14.0 };
    col()
        .key("privacy_shield_viewport")
        .w_full()
        .grow(1.0)
        .min_w(0.0)
        .min_h(0.0)
        .padding(pad)
        .gap(12.0)
        .scroll(ScrollSpec { stick_bottom: false })
        .child(header)
        .child(stats_card)
        .child(audit_card)
        .child(action_bar)
        .child(list)
        .winblast_scroll(p)
}
