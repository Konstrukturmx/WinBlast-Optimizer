//! Oyun uygunluğu (Gaming Readiness & Capability Matrix) View in Rune.
//!
//! Evaluates PC hardware against modern AAA gaming standards (DX12 Ultimate, ReBAR, AVX2, DirectStorage).

use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;

    let cpu_cores = s.cpu_info.as_ref().map(|c| c.physical_cores as u32).unwrap_or(4);
    let ram_mb = s.realtime_metrics.as_ref().map(|m| m.total_ram_mb).unwrap_or(8192);
    let gpu_name = s.gpus.first().map(|g| g.name.as_str()).unwrap_or("GPU");
    let hags_active = s.hags_enabled;
    let has_nvme = s.platform_security.as_ref().map(|ps| {
        ps.volumes.iter().any(|v| v.media_kind == crate::hardware::sys_inventory::DriveMediaKind::Nvme)
    }).unwrap_or(false);

    let evaluated_report = crate::tools::game_overlay::GameSuitabilityReport::evaluate(
        cpu_cores,
        ram_mb,
        gpu_name,
        hags_active,
        has_nvme,
    );
    let report = &evaluated_report;

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .pr(16.0)
        .gap(12.0)
        // Header
        .child(
            col()
                .w_full()
                .gap(2.0)
                .child(
                    text(s.tr_v(ViewTextKey::SuitabilityTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr_v(ViewTextKey::SuitabilitySubtitle))
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                ),
        )
        // Score Hero Card
        .child(
            row()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.accent)
                .hover(p.bg_card_hover)
                .motion(p.motion)
                .center()
                .gaps(14.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(200.0)
                        .child(
                            text(s.tr_v(ViewTextKey::SuitabilityHeroTitle))
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_secondary),
                        )
                        .child(
                            text(report.tier_label)
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(s.tr_v(ViewTextKey::SuitabilityRating))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .px(14.0)
                        .py(8.0)
                        .radius(style.badge_radius())
                        .fill(p.success.alpha(0.2))
                        .border(1.5, p.success)
                        .child(
                            text(format!(
                                "{}% {}",
                                report.overall_score_pct,
                                if report.overall_score_pct >= 70 {
                                    s.tr_v(ViewTextKey::SuitabilityReadyBadge)
                                } else {
                                    s.tr_v(ViewTextKey::SuitabilityLimitedBadge)
                                }
                            ))
                            .font(rune::tok::type_::TITLE)
                            .color(p.success),
                        ),
                ),
        )
        // Feature Checks List
        .children(report.checks.iter().map(|chk| {
            let chk_color = if chk.is_supported { p.success } else { p.warning };
            col()
                .w_full()
                .padding(12.0)
                .radius(style.card_radius().min(6.0))
                .fill(p.bg_card_hover)
                .border(1.0, p.border)
                .gap(4.0)
                .child(
                    row()
                        .w_full()
                        .center()
                        .gaps(8.0, 4.0)
                        .flex_wrap()
                        .child(
                            text(chk.feature_name)
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary)
                                .grow(1.0),
                        )
                        .child(
                            row()
                                .px(8.0)
                                .py(2.0)
                                .radius(style.badge_radius())
                                .fill(chk_color.alpha(0.18))
                                .border(1.0, chk_color)
                                .child(
                                    text(if chk.is_supported {
                                        format!("✔ {}", s.tr_v(ViewTextKey::SuitabilityReadyBadge))
                                    } else {
                                        s.tr_v(ViewTextKey::SuitabilityLimitedBadge).to_string()
                                    })
                                    .font(rune::tok::type_::LABEL)
                                    .color(chk_color),
                                ),
                        ),
                )
                .child(
                    text(&chk.description)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_muted),
                )
                .child(
                    text(chk.recommendation)
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                )
        }))
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
