//! D7: how long AppState::new() blocks the UI thread before the window appears.
//! `cargo test --test startup_timing -- --ignored --nocapture`

use winblast0::core::backend::{RealBackend, SystemBackend};

#[test]
#[ignore = "manual timing on a real host"]
fn time_app_state_new() {
    let b = RealBackend;
    let t = |label: &str, f: &dyn Fn()| {
        let s = std::time::Instant::now();
        f();
        println!("{label:>16}: {} ms", s.elapsed().as_millis());
    };
    t("detect_os", &|| { let _ = b.detect_os(); });
    t("detect_cpu", &|| { let _ = b.detect_cpu(); });
    t("detect_gpus", &|| { let _ = b.detect_gpus(); });
    t("hags+mpo", &|| { let _ = (b.is_hags_enabled(), b.is_mpo_disabled()); });
    t("metrics sample", &|| { let mut m = b.create_metrics_sampler(35); let _ = m.sample(); });
    t("core parking", &|| { let _ = b.detect_core_parking(); });
    t("rules", &|| { let _ = winblast0::core::risk::load_rules_with_fallback(None); });
    let start = std::time::Instant::now();
    let state = winblast0::app::AppState::new();
    println!("AppState::new() took {} ms ({} tweaks)", start.elapsed().as_millis(), state.tweaks.len());
}
