//! Game Prerequisites & Runtimes View in Rune.
//!
//! Provides one-click verification, batch installation, and cascade fallback
//! (winget -> choco -> direct web) for all 10 essential PC gaming runtimes.

use rune::color::Color;
use rune::{col, row, text, text_input, El, ScrollSpec};

use crate::app::AppState;
use crate::core::i18n::TextKey;
use crate::tools::game_prereqs::PrereqInstallState;
use crate::ui::widgets::button::action_button_guarded;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;

    let total = s.game_prereqs.len();
    let installed_count = s.game_prereqs.iter().filter(|i| i.installed).count();
    let missing_count = total.saturating_sub(installed_count);

    let q_raw = s.prereq_filter_edit.text.trim().to_lowercase();

    // Top Header
    let header = row()
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
                    text(s.tr(TextKey::GamePrereqsTitle))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text(s.tr(TextKey::GamePrereqsSub))
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
                        if s.is_prereqs_scanning { s.tr(TextKey::Scanning) } else { s.tr(TextKey::GamePrereqsScan) },
                        p.btn_secondary,
                        p.text_primary,
                        !s.is_prereqs_scanning,
                        p,
                        |s| s.scan_game_prereqs(),
                    )
                    .motion(style.motion())
                    .radius(style.btn_radius()),
                )
                .child(
                    action_button_guarded(
                        s.tr(TextKey::GamePrereqsInstallAll),
                        p.accent,
                        Color::WHITE,
                        missing_count > 0 && s.prereq_install_in_flight.is_none(),
                        p,
                        |s| s.install_all_missing_prereqs(),
                    )
                    .motion(style.motion())
                    .radius(style.btn_radius()),
                ),
        );

    // Summary Metric Cards
    let summary_cards = row()
        .w_full()
        .min_w(0.0)
        .flex_wrap()
        .gaps(10.0, 10.0)
        .child(
            col()
                .grow(1.0)
                .min_w(160.0)
                .padding(10.0)
                .radius(style.card_radius().min(8.0))
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .child(text("Total Tracked Runtimes").font(rune::tok::type_::LABEL).color(p.text_secondary))
                .child(text(format!("{}", total)).font(rune::tok::type_::TITLE).color(p.text_primary)),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(160.0)
                .padding(10.0)
                .radius(style.card_radius().min(8.0))
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .child(text("Installed & Verified").font(rune::tok::type_::LABEL).color(p.success))
                .child(text(format!("{}", installed_count)).font(rune::tok::type_::TITLE).color(p.success)),
        )
        .child(
            col()
                .grow(1.0)
                .min_w(160.0)
                .padding(10.0)
                .radius(style.card_radius().min(8.0))
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .child(text("Missing / Requires Action").font(rune::tok::type_::LABEL).color(if missing_count > 0 { p.warning } else { p.text_secondary }))
                .child(text(format!("{}", missing_count)).font(rune::tok::type_::TITLE).color(if missing_count > 0 { p.warning } else { p.text_secondary })),
        );

    // Filter Bar with Text Input
    let filter_bar = row()
        .w_full()
        .min_w(0.0)
        .items_center()
        .gaps(8.0, 8.0)
        .flex_wrap()
        .child(
            row()
                .grow(1.0)
                .min_w(220.0)
                .min_h(32.0)
                .px(8.0)
                .py(4.0)
                .radius(6.0)
                .fill(p.bg_card)
                .border(1.0, p.border)
                .items_center()
                .child(
                    text_input(&s.prereq_filter_edit, rune::tok::type_::BODY)
                        .key("prereq_search_input")
                        .single_line()
                        .placeholder("Filter runtimes (e.g. DirectX, C++, Vulkan, .NET)...", p.text_muted)
                        .color(p.text_primary)
                        .on_edit(|s: &mut AppState| &mut s.prereq_filter_edit)
                        .grow(1.0)
                        .min_w(0.0),
                ),
        );

    // List of Prerequisites inside bounded scroll container
    let mut list_col = col().w_full().min_w(0.0).gap(6.0);

    let filtered_items: Vec<_> = s
        .game_prereqs
        .iter()
        .filter(|item| {
            if q_raw.is_empty() {
                return true;
            }
            item.name.to_lowercase().contains(&q_raw)
                || item.category.to_lowercase().contains(&q_raw)
                || item.description.to_lowercase().contains(&q_raw)
                || item.id.to_lowercase().contains(&q_raw)
        })
        .collect();

    if filtered_items.is_empty() {
        list_col = list_col.child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(20.0)
                .center()
                .gap(6.0)
                .child(text("🔍").font(rune::tok::type_::TITLE).color(p.text_muted))
                .child(
                    text(if s.game_prereqs.is_empty() {
                        "No runtime prerequisites loaded. Click 'Rescan Runtimes' to start."
                    } else {
                        "No gaming runtimes match the current search filter."
                    })
                    .font(rune::tok::type_::BODY)
                    .color(p.text_muted),
                ),
        );
    } else {
        for item in filtered_items {
            let is_busy = s.prereq_install_in_flight.as_deref() == Some(&item.id);
            let item_id = item.id.clone();

            let (state_text, state_color) = match item.state {
                PrereqInstallState::Installed => ("Installed ✔", p.success),
                PrereqInstallState::NotInstalled => ("Missing ⚠", p.warning),
                PrereqInstallState::Installing => ("Installing ⟳", p.accent),
                PrereqInstallState::Failed => ("Failed ✘", p.danger),
            };

            let ver_text = item
                .installed_version
                .as_deref()
                .unwrap_or("Not Detected");

            let card = row()
                .w_full()
                .min_w(0.0)
                .padding(10.0)
                .pr(12.0)
                .radius(style.card_radius().min(6.0))
                .fill(p.bg_card_hover)
                .border(1.0, p.border)
                .center()
                .gaps(10.0, 8.0)
                .flex_wrap()
                .child(
                    col()
                        .grow(1.0)
                        .min_w(220.0)
                        .child(
                            text(&item.name)
                                .font(rune::tok::type_::LABEL)
                                .color(p.text_primary),
                        )
                        .child(
                            text(format!("{} • {}", item.category, item.description))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    col()
                        .min_w(140.0)
                        .child(
                            text(format!("Version: {}", ver_text))
                                .font(rune::tok::type_::LABEL)
                                .color(if item.installed { p.success } else { p.text_secondary }),
                        )
                        .child(
                            text(&item.status_message)
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    row()
                        .px(8.0)
                        .py(3.0)
                        .radius(style.badge_radius())
                        .fill(state_color.alpha(0.18))
                        .border(1.0, state_color)
                        .child(
                            text(state_text)
                                .font(rune::tok::type_::LABEL)
                                .color(state_color),
                        ),
                )
                .child(
                    action_button_guarded(
                        if is_busy {
                            "Installing..."
                        } else if item.installed {
                            "Reinstall / Verify"
                        } else {
                            "Install Runtime"
                        },
                        if item.installed { p.btn_secondary } else { p.accent },
                        if item.installed { p.text_primary } else { Color::WHITE },
                        !is_busy && s.prereq_install_in_flight.is_none(),
                        p,
                        move |s| s.install_game_prereq(item_id.clone()),
                    )
                    .motion(style.motion())
                    .radius(style.btn_radius()),
                );

            list_col = list_col.child(card);
        }
    }

    let list_container = col()
        .w_full()
        .min_w(0.0)
        .padding(4.0)
        .child(list_col);

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
        .pr(16.0)
        .gap(12.0)
        .child(header)
        .child(summary_cards)
        .child(
            col()
                .w_full()
                .min_w(0.0)
                .padding(style.card_padding())
                .radius(style.card_radius())
                .fill(p.card_fill_styled(style))
                .border(style.border_width(), p.border_styled(style))
                .gap(10.0)
                .child(filter_bar)
                .child(list_container),
        )
        .scroll(ScrollSpec { stick_bottom: false })
        .winblast_scroll(p)
}
