//! Threat and risk level badge component in Rune.

use rune::{row, text, El};
use crate::core::risk::{RiskLevel, RiskValuation};
use crate::ui::theme::{self, ThemePalette};

/// Renders a localized multi-axis threat valuation badge using dynamic theme palette.
pub fn view_localized(risk: &RiskValuation, p: &ThemePalette, s: &crate::app::AppState) -> El {
    let (label, bg_color, text_color) = match risk.stability_risk {
        RiskLevel::Safe => (s.tr(crate::core::i18n::TextKey::RiskSafe), p.success.alpha(0.18), p.success),
        RiskLevel::Low => (s.tr(crate::core::i18n::TextKey::RiskLow), p.accent.alpha(0.18), p.accent),
        RiskLevel::Moderate => (s.tr(crate::core::i18n::TextKey::RiskCaution), p.warning.alpha(0.20), p.warning),
        RiskLevel::High => (s.tr(crate::core::i18n::TextKey::RiskHigh), p.warning.alpha(0.28), p.warning),
        RiskLevel::Extreme => (s.tr(crate::core::i18n::TextKey::RiskExtreme), p.danger.alpha(0.28), p.danger),
    };

    row()
        .min_w(0.0)
        .shrink()
        .px(8.0)
        .py(3.0)
        .radius(4.0)
        .fill(bg_color)
        .border(1.0, text_color.alpha(0.4))
        .center()
        .child(
            text(label.to_string())
                .font(rune::tok::type_::LABEL)
                .color(text_color)
                .shrink()
                .wrap(),
        )
}

/// Renders a multi-axis threat valuation badge using dynamic theme palette.
pub fn view_with_palette(risk: &RiskValuation, p: &ThemePalette) -> El {
    let (label, bg_color, text_color) = match risk.stability_risk {
        RiskLevel::Safe => ("SAFE", p.success.alpha(0.18), p.success),
        RiskLevel::Low => ("LOW", p.accent.alpha(0.18), p.accent),
        RiskLevel::Moderate => ("MODERATE", p.warning.alpha(0.20), p.warning),
        RiskLevel::High => ("HIGH", p.warning.alpha(0.28), p.warning),
        RiskLevel::Extreme => ("EXTREME", p.danger.alpha(0.28), p.danger),
    };

    row()
        .min_w(0.0)
        .shrink()
        .px(8.0)
        .py(3.0)
        .radius(4.0)
        .fill(bg_color)
        .border(1.0, text_color.alpha(0.4))
        .center()
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(text_color)
                .shrink()
                .wrap(),
        )
}

/// Renders a multi-axis threat valuation badge with semantic OKLCH colors.
#[allow(dead_code)]
pub fn view(risk: &RiskValuation) -> El {
    let (label, bg_color, text_color) = match risk.stability_risk {
        RiskLevel::Safe => ("SAFE", theme::SUCCESS.alpha(0.18), theme::SUCCESS),
        RiskLevel::Low => ("LOW", theme::ACCENT.alpha(0.18), theme::ACCENT),
        RiskLevel::Moderate => ("MODERATE", theme::WARNING.alpha(0.20), theme::WARNING),
        RiskLevel::High => ("HIGH", theme::WARNING.alpha(0.28), theme::WARNING),
        RiskLevel::Extreme => ("EXTREME", theme::DANGER.alpha(0.28), theme::DANGER),
    };

    row()
        .px(8.0)
        .py(3.0)
        .radius(4.0)
        .fill(bg_color)
        .border(1.0, text_color.alpha(0.4))
        .center()
        .child(
            text(label)
                .font(rune::tok::type_::LABEL)
                .color(text_color),
        )
}
