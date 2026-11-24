//! Offline visual check of the Slint UI (no system access): renders `MainWindow` with Slint's
//! software renderer into PPM images. Manual only:
//!   WINBLAST_PREVIEW_DIR=<dir> cargo test --test ui_render_preview -- --ignored --nocapture
//! Store values set here are render fixtures for layout review, never shown by the product.

use std::io::Write;
use std::rc::Rc;
use std::time::{Duration, Instant};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, WindowAdapter};
use slint::{ComponentHandle, ModelRc, PhysicalSize, Rgb8Pixel, VecModel};

use winblast0::core::i18n::Language;
use winblast0::core::theme::{ColorScheme, ThemeMode, ThemePalette, UiThemeStyle};
use winblast0::ui_slint::{palette, theme, AppStore, DashboardStore, DiskStore, MainWindow, ToastItem, Tr};

struct Headless {
    window: Rc<MinimalSoftwareWindow>,
    start: Instant,
}

impl Platform for Headless {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> Duration {
        self.start.elapsed()
    }
}

const W: u32 = 1280;
const H: u32 = 800;

fn settle_and_save(win: &MinimalSoftwareWindow, dir: &std::path::Path, name: &str, ms: u64) {
    let size = win.size();
    let (w, h) = (size.width, size.height);
    let mut buf = vec![Rgb8Pixel::default(); (w * h) as usize];
    // Draw every frame like a real event loop (conditional elements are created on draw).
    let frames = ms / 16;
    for _ in 0..frames.max(2) {
        slint::platform::update_timers_and_animations();
        win.draw_if_needed(|r| {
            r.render(&mut buf, w as usize);
        });
        std::thread::sleep(Duration::from_millis(16));
    }
    slint::platform::update_timers_and_animations();
    win.request_redraw();
    win.draw_if_needed(|r| {
        r.render(&mut buf, w as usize);
    });
    let mut f = std::fs::File::create(dir.join(format!("{name}.ppm"))).expect("create ppm");
    write!(f, "P6\n{w} {h}\n255\n").expect("header");
    let bytes: Vec<u8> = buf.iter().flat_map(|p| [p.r, p.g, p.b]).collect();
    f.write_all(&bytes).expect("pixels");
}

#[test]
#[ignore = "manual visual check; writes images to WINBLAST_PREVIEW_DIR"]
fn render_preview_frames() {
    let Some(dir) = std::env::var_os("WINBLAST_PREVIEW_DIR").map(std::path::PathBuf::from) else {
        eprintln!("WINBLAST_PREVIEW_DIR not set; nothing rendered");
        return;
    };
    std::fs::create_dir_all(&dir).expect("preview dir");

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    window.set_size(PhysicalSize::new(W, H));
    slint::platform::set_platform(Box::new(Headless { window: window.clone(), start: Instant::now() }))
        .expect("platform");

    let ui = MainWindow::new().expect("MainWindow");
    ui.global::<Tr>().on_lookup(|key, _| winblast0::ui_slint::i18n::lookup(Language::English, key.as_str()));
    theme::apply(&ui, &ThemePalette::new(ThemeMode::Dark, ColorScheme::default(), UiThemeStyle::default()), false);
    let app = ui.global::<AppStore>();
    app.set_app_version(env!("CARGO_PKG_VERSION").into());
    app.set_metrics_available(true);
    app.set_cpu_usage(37.0);
    app.set_ram_usage(58.0);
    let dash = ui.global::<DashboardStore>();
    dash.set_health_score(72);
    dash.set_health_tone("warning".into());
    dash.set_metrics_available(true);
    dash.set_cpu_title("CPU".into());
    dash.set_cpu_text("37% • 8 cores".into());
    dash.set_ram_title("RAM".into());
    dash.set_ram_text("58% • 9.3 / 16 GB".into());
    ui.show().expect("show");

    settle_and_save(&window, &dir, "01-dashboard", 1500);

    app.set_toasts(ModelRc::new(VecModel::from(vec![
        ToastItem { id: 1, text: "Scan completed: 41 tweaks evaluated".into(), tone: "success".into() },
        ToastItem { id: 2, text: "Restore point could not be created".into(), tone: "danger".into() },
    ])));
    settle_and_save(&window, &dir, "02-toasts", 600);

    app.set_palette_items(ModelRc::new(VecModel::from(palette::search(Language::English, ""))));
    app.set_palette_open(true);
    settle_and_save(&window, &dir, "03-palette", 800);
    app.set_palette_open(false);
    settle_and_save(&window, &dir, "03a-closed", 100);
    theme::apply(&ui, &ThemePalette::new(ThemeMode::Dark, ColorScheme::default(), UiThemeStyle::default()), true);
    app.set_palette_open(true);
    settle_and_save(&window, &dir, "03b-palette-rm", 300);
    app.set_palette_open(false);
    theme::apply(&ui, &ThemePalette::new(ThemeMode::Dark, ColorScheme::default(), UiThemeStyle::default()), false);
    settle_and_save(&window, &dir, "03c-closed", 100);
    app.set_palette_open(true);
    settle_and_save(&window, &dir, "03e-open", 500);
    app.set_palette_open(false);
    app.set_toasts(ModelRc::new(VecModel::from(Vec::<ToastItem>::new())));

    ui.global::<DiskStore>().set_scanning(true);
    app.set_active_view("disk_health".into());
    settle_and_save(&window, &dir, "04-disk-shimmer", 900);

    app.set_active_view("support_about".into());
    settle_and_save(&window, &dir, "05-about", 900);

    theme::apply(&ui, &ThemePalette::new(ThemeMode::Light, ColorScheme::default(), UiThemeStyle::default()), false);
    app.set_active_view("dashboard".into());
    settle_and_save(&window, &dir, "06-dashboard-light", 1500);

    window.set_size(PhysicalSize::new(560, 760));
    settle_and_save(&window, &dir, "07-compact-light", 1200);

    // 1.7.0 palettes: every scheme in dark and light (Glassy style), dashboard view.
    window.set_size(PhysicalSize::new(W, H));
    for scheme in ColorScheme::ALL {
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            theme::apply(&ui, &ThemePalette::new(mode, scheme, UiThemeStyle::default()), false);
            settle_and_save(&window, &dir, &format!("10-{scheme:?}-{mode:?}"), 700);
        }
    }

    // In-progress HUD with its Close button, then a hovered card with the GPU depth layer.
    theme::apply(&ui, &ThemePalette::new(ThemeMode::Dark, ColorScheme::default(), UiThemeStyle::default()), false);
    app.set_hud_status("Probing SMBIOS motherboard tables & UEFI firmware...".into());
    app.set_hud_elapsed("12s".into());
    app.set_hud_visible(true);
    settle_and_save(&window, &dir, "11-hud", 900);
    // Glyph probe: which symbols the UI font + system fallback can actually draw.
    if let Ok(probe) = std::env::var("WINBLAST_GLYPH_PROBE") {
        app.set_hud_status(probe.into());
        settle_and_save(&window, &dir, "11b-glyph-probe", 300);
    }
    app.set_hud_visible(false);
    ui.global::<winblast0::ui_slint::Fx>().set_gpu(true);
    window.dispatch_event(slint::platform::WindowEvent::PointerMoved { position: slint::LogicalPosition::new(560.0, 260.0) });
    settle_and_save(&window, &dir, "12-depth-hover", 700);
    for (w, h) in [(800, 760), (1024, 768)] {
        window.set_size(PhysicalSize::new(w, h));
        settle_and_save(&window, &dir, &format!("13-topbar-{w}"), 700);
    }
}
