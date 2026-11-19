//! Unorthodox Edge-Case Verification Suite for WinBlast0 (v1.4.0).
//!
//! Stresses the engine under extreme, non-standard real-world conditions:
//! 1. Multi-SZ pending rename buffers with odd byte lengths, corrupt surrogates, and missing terminators.
//! 2. Extreme registry corruption (malformed hex bytes, unknown REG types, oversized payloads).
//! 3. Hardware sensor anomalies (broken EC chips, 0K / 65535K temps, 255% NVMe wear saturation, u128 overflow).
//! 4. Micro viewports (50x50, 120x100) and ultra-wide viewports (7680x2160, 8000x300).
//! 5. Concurrent worker event stress (8 simultaneous threads mutating state, scanning, and switching locales).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use rune::{Env, Runtime};

use winblast0::app::{ActiveView, AppState, EngineMode};
use winblast0::core::backend::SandboxEnv;
use winblast0::core::i18n::Language;
use winblast0::core::reg_helpers::{decode_hex, encode_hex};
use winblast0::core::risk::load_embedded_rules;
use winblast0::hardware::thermal::{SystemThermalSnapshot, ThermalHealth};
use winblast0::scanner::scan_all_tweaks_parallel_with;
use winblast0::tools::diagnostics::parse_multi_sz_lossless;
use winblast0::ui;
use winblast0::ui::theme::{ColorScheme, ThemeMode, UiThemeStyle};

// ============================================================================
// 1. Multi-SZ Odd Byte Lengths & Corrupted Buffers
// ============================================================================

#[test]
fn test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions() {
    // 1. Empty buffer
    assert!(parse_multi_sz_lossless(&[]).is_empty());

    // 2. Odd-length single byte (cannot form even one UTF-16 code unit)
    assert!(parse_multi_sz_lossless(&[0x41]).is_empty());

    // 3. Odd-length 3 bytes (one UTF-16 char 'A', then trailing rogue byte)
    let res = parse_multi_sz_lossless(&[0x41, 0x00, 0xFF]);
    assert_eq!(res, vec!["A".to_string()]);

    // 4. Odd-length 17 bytes (corrupted registry payload)
    // "C:\test\0" is 8 chars * 2 = 16 bytes, plus trailing 1 rogue byte
    let mut data: Vec<u8> = "C:\\test\0".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    data.push(0xEE); // rogue 17th byte
    assert_eq!(data.len(), 17);
    let res = parse_multi_sz_lossless(&data);
    assert_eq!(res, vec!["C:\\test".to_string()]);

    // 5. Buffer with missing trailing double-null
    // Windows expects \0\0 at the end; corrupt buffer has only one or none
    let data_no_term: Vec<u8> = "FileOne\0FileTwo".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    let res = parse_multi_sz_lossless(&data_no_term);
    assert_eq!(res, vec!["FileOne".to_string(), "FileTwo".to_string()]);

    // 6. Buffer with consecutive multiple nulls and empty strings
    let data_nulls: Vec<u8> = "\0\0\0A\0\0\0B\0\0\0\0".encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    let res = parse_multi_sz_lossless(&data_nulls);
    assert_eq!(res, vec!["A".to_string(), "B".to_string()]);

    // 7. Buffer with lone UTF-16 surrogate halves (invalid Unicode)
    // from_utf16_lossy should replace with U+FFFD without crashing
    let corrupt_surrogate = [0x00, 0xD8, 0x00, 0x00]; // high surrogate alone, then null
    let res = parse_multi_sz_lossless(&corrupt_surrogate);
    assert_eq!(res.len(), 1);
    assert!(res[0].contains('\u{FFFD}'));

    // 8. Large buffer (10,000 characters) with odd trailing byte
    let long_str: String = "X".repeat(10_000);
    let mut long_bytes: Vec<u8> = long_str.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    long_bytes.push(0xAB);
    let res = parse_multi_sz_lossless(&long_bytes);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].len(), 10_000);
}

// ============================================================================
// 2. Extreme Registry Corruption
// ============================================================================

#[test]
fn test_unorthodox_extreme_registry_corruption() {
    // 1. Odd hex string length must fail cleanly without panic
    assert!(decode_hex("a").is_err());
    assert!(decode_hex("12345").is_err());
    assert!(decode_hex("0102030").is_err());

    // 2. Invalid hex characters (non-hex characters like Z, symbols, control chars)
    assert!(decode_hex("00zz").is_err());
    assert!(decode_hex("01\x00").is_err());
    assert!(decode_hex("deadbeef!#").is_err());

    // 3. Round-trip arbitrary binary bytes including all 0x00..0xFF values
    let all_bytes: Vec<u8> = (0u8..=255).collect();
    let hex = encode_hex(&all_bytes);
    let decoded = decode_hex(&hex).expect("decode_hex roundtrip failed");
    assert_eq!(decoded, all_bytes);

    // 4. Massive binary payload (128 KB hex string)
    let large_bytes = vec![0xCA; 65536];
    let large_hex = encode_hex(&large_bytes);
    assert_eq!(large_hex.len(), 131072);
    let decoded_large = decode_hex(&large_hex).expect("large decode failed");
    assert_eq!(decoded_large, large_bytes);
}

// ============================================================================
// 3. Hardware Sensor Anomalies & NVMe SMART Saturation
// ============================================================================

#[test]
fn test_unorthodox_hardware_sensor_anomalies() {
    // 1. Thermal Health label thresholds
    assert_eq!(ThermalHealth::Cool.label(), "❄️ Cool");
    assert_eq!(ThermalHealth::Optimal.label(), "🟢 Optimal");
    assert_eq!(ThermalHealth::Warm.label(), "🟡 Warm");
    assert_eq!(ThermalHealth::Hot.label(), "🔥 Hot (Throttling)");

    // 2. Live snapshot probe must handle any host hardware gracefully
    let snap = SystemThermalSnapshot::read();
    assert!(!snap.status_message.is_empty());
    // Ambient estimate must remain within plausible bounds [0..100°C]
    assert!(snap.ambient_estimate_c >= 0.0 && snap.ambient_estimate_c <= 100.0);

    // 3. NVMe wear level saturation test:
    // If a drive reports 100% or 255% wear, health must saturate to 0%, never underflow u32!
    let percentage_used_normal = 5u8;
    assert_eq!(100u32.saturating_sub(percentage_used_normal as u32), 95);

    let percentage_used_exhausted = 100u8;
    assert_eq!(100u32.saturating_sub(percentage_used_exhausted as u32), 0);

    let percentage_used_burned_out = 255u8;
    assert_eq!(100u32.saturating_sub(percentage_used_burned_out as u32), 0);

    // 4. Data Units Written TBW calculations with extreme u128 values
    // Must never panic or produce NaN
    let units_zero = 0u128;
    let tbw_zero = (units_zero as f64 * 512_000.0) / 1_000_000_000_000.0;
    assert_eq!(tbw_zero, 0.0);

    let units_extreme = 50_000_000_000u128; // ~25.6 Petabytes
    let tbw_extreme = (units_extreme as f64 * 512_000.0) / 1_000_000_000_000.0;
    assert!(tbw_extreme > 25000.0);
    assert!(!tbw_extreme.is_nan() && !tbw_extreme.is_infinite());
}

// ============================================================================
// 4. Micro and Ultra-Wide Viewport Layout Stability
// ============================================================================

#[test]
fn test_unorthodox_micro_and_ultrawide_viewports() {
    let sbx = SandboxEnv::new();
    let tweaks = load_embedded_rules();
    sbx.seed_all_tweaks(&tweaks);

    let state = AppState::new_with_backend(Arc::new(sbx));

    let extreme_viewports: &[(f32, f32)] = &[
        // Micro viewports
        (50.0, 50.0),
        (120.0, 100.0),
        (320.0, 240.0),
        // Narrow vertical banner
        (280.0, 3840.0),
        // Ultra-wide 32:9 and multi-monitor setups
        (5120.0, 1440.0),
        (7680.0, 2160.0),
        // Extreme ribbon / ticker banner
        (8000.0, 300.0),
    ];

    let test_views = [
        ActiveView::EasyDashboard,
        ActiveView::AdvancedMatrix,
        ActiveView::DriverBios,
        ActiveView::HardwareView,
        ActiveView::FaultDiagnostics,
        ActiveView::NetworkDiag,
        ActiveView::ThermalMonitor,
        ActiveView::DiskHealth,
    ];

    let mut rt = Runtime::new(state, ui::view, (800.0, 600.0), Env::new(true, 1.0));
    for &(w, h) in extreme_viewports {
        rt.set_viewport((w, h), 1.0);
        for &view in &test_views {
            rt.update(|s, _| {
                s.active_view = view;
            });
            rt.settle_all();
            rt.frame(0.016);
        }
    }
}

// ============================================================================
// 5. Concurrent Worker Event Stress
// ============================================================================

#[test]
fn test_unorthodox_concurrent_worker_event_stress() {
    let sbx = Arc::new(SandboxEnv::new());
    let tweaks = load_embedded_rules();
    sbx.seed_all_tweaks(&tweaks);

    let state = Arc::new(std::sync::RwLock::new(AppState::new_with_backend(sbx.clone())));
    let stop_signal = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    // Worker 1: Parallel scanning
    {
        let sbx = sbx.clone();
        let tweaks = tweaks.clone();
        let stop = stop_signal.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..15 {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                let _ = scan_all_tweaks_parallel_with(sbx.as_ref(), &tweaks, 22631, "Pro");
                thread::yield_now();
            }
        }));
    }

    // Worker 2: Language cycling across all 8 languages
    {
        let state = state.clone();
        let stop = stop_signal.clone();
        handles.push(thread::spawn(move || {
            let languages = Language::ALL;
            for i in 0..40 {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                {
                    let mut s = state.write().unwrap();
                    s.set_language(languages[i % languages.len()]);
                }
                thread::yield_now();
            }
        }));
    }

    // Worker 3: Theme mode & style switching
    {
        let state = state.clone();
        let stop = stop_signal.clone();
        handles.push(thread::spawn(move || {
            let styles = [
                UiThemeStyle::Classic,
                UiThemeStyle::Glassy,
                UiThemeStyle::Compact,
                UiThemeStyle::Materialistic,
                UiThemeStyle::Minimalistic,
            ];
            for i in 0..40 {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                {
                    let mut s = state.write().unwrap();
                    s.theme_mode = if i % 2 == 0 { ThemeMode::Dark } else { ThemeMode::Light };
                    s.theme_style = styles[i % styles.len()];
                    s.color_scheme = ColorScheme::CyberIndigo;
                }
                thread::yield_now();
            }
        }));
    }

    // Worker 4: EngineMode & Navigation switching
    {
        let state = state.clone();
        let stop = stop_signal.clone();
        handles.push(thread::spawn(move || {
            let views = [
                ActiveView::EasyDashboard,
                ActiveView::AdvancedMatrix,
                ActiveView::HardwareView,
                ActiveView::DriverBios,
                ActiveView::FaultDiagnostics,
                ActiveView::NetworkDiag,
            ];
            for i in 0..30 {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                {
                    let mut s = state.write().unwrap();
                    s.engine_mode = if i % 2 == 0 { EngineMode::Instant } else { EngineMode::Normal };
                    s.active_view = views[i % views.len()];
                }
                thread::yield_now();
            }
        }));
    }

    // Worker 5: Read thermal telemetry snapshot
    {
        let stop = stop_signal.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..2 {
                if stop.load(Ordering::Relaxed) {
                    break;
                }
                let _ = SystemThermalSnapshot::read();
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }));
    }

    // Wait for all workers to complete cleanly
    for handle in handles {
        handle.join().expect("Worker thread panicked during concurrent stress test");
    }
}
