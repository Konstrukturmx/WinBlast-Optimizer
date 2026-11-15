//! Performans testleri (Performance Benchmark Suite) View in Rune.
//!
//! Multi-core CPU compute, memory bandwidth, and storage throughput benchmarking engine.

use rune::color::Color;
use rune::{col, row, text, El, ScrollSpec};

use crate::app::AppState;
use crate::ui::widgets::button::action_button_guarded;
use crate::ui::widgets::scrollbar::WinBlastScrollable;
use crate::ui::LayoutTier;

pub fn view(s: &AppState, _tier: LayoutTier) -> El {
    let p = s.palette();
    let style = s.theme_style;
    let score = s.perf_score.clone().unwrap_or_default();

    col()
        .w_full()
        .min_w(0.0)
        .padding(style.card_padding())
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
                            text("Sistem Performans Testleri")
                                .font(rune::tok::type_::TITLE)
                                .color(p.text_primary),
                        )
                        .child(
                            text("Real-world CPU arithmetic compute, L3 cache/RAM copy throughput, and storage I/O response.")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    action_button_guarded(
                        if s.is_benchmarking_perf { "Test Yapılıyor..." } else { "Testi Başlat" },
                        p.accent,
                        Color::WHITE,
                        !s.is_benchmarking_perf,
                        p,
                        |s| s.start_perf_benchmark_async(),
                    )
                    .radius(style.btn_radius()),
                ),
        )
        // Composite Index Hero Card
        .child(
            col()
                .w_full()
                .padding(16.0)
                .radius(style.card_radius())
                .fill(p.accent.alpha(0.12))
                .border(1.5, p.accent)
                .gap(6.0)
                .child(
                    text("WinBlast Genel Performans Skoru")
                        .font(rune::tok::type_::LABEL)
                        .color(p.accent),
                )
                .child(
                    text(format!("{} Puan", score.total_composite_score))
                        .font(rune::tok::type_::TITLE)
                        .color(p.text_primary),
                )
                .child(
                    text("Sisteminiz modern oyunlar, düşük giriş gecikmesi ve yüksek FPS için üst düzey performansa sahiptir.")
                        .font(rune::tok::type_::BODY)
                        .color(p.text_secondary),
                ),
        )
        // Benchmark Metrics Grid
        .child(
            row()
                .w_full()
                .flex_wrap()
                .gaps(12.0, 12.0)
                .child(
                    // CPU Score Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(4.0)
                        .child(text("İşlemci (CPU Hesaplama)").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{} Puan (Multi-Core)", score.cpu_multi_score))
                                .font(rune::tok::type_::TITLE)
                                .color(p.success),
                        )
                        .child(
                            text(format!("Tek Çekirdek Skoru: {} Puan", score.cpu_single_score))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    // RAM Score Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(4.0)
                        .child(text("Bellek (RAM Bant Genişliği)").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{:.1} GB/s", score.ram_read_gb_s))
                                .font(rune::tok::type_::TITLE)
                                .color(p.accent),
                        )
                        .child(
                            text(format!("Rastgele Gecikme: {:.1} ns", score.ram_latency_ns))
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                )
                .child(
                    // Disk Score Card
                    col()
                        .grow(1.0)
                        .min_w(240.0)
                        .padding(style.card_padding())
                        .radius(style.card_radius())
                        .fill(p.card_fill_styled(style))
                        .border(style.border_width(), p.border_styled(style))
                        .gap(4.0)
                        .child(text("Depolama (NVMe Burst)").font(rune::tok::type_::LABEL).color(p.text_secondary))
                        .child(
                            text(format!("{:.0} MB/s", score.disk_seq_read_mb_s))
                                .font(rune::tok::type_::TITLE)
                                .color(p.warning),
                        )
                        .child(
                            text("PCIe 4.0 NVMe Sıralı Okuma Hızı")
                                .font(rune::tok::type_::BODY)
                                .color(p.text_muted),
                        ),
                ),
        )
        .scroll(ScrollSpec { stick_bottom: false }).winblast_scroll(p)
}
