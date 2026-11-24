// build.rs — Embeds UAC manifest, Windows icon & PE metadata
use embed_manifest::manifest::ExecutionLevel::{AsInvoker, HighestAvailable};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    println!("cargo:rerun-if-env-changed=WINBLAST_SKIP_MANIFEST");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/winblast.ico");
    println!("cargo:rerun-if-changed=slint-project.json");
    println!("cargo:rerun-if-changed=ui");

    let is_release = std::env::var("PROFILE").as_deref() == Ok("release");
    // Debug builds carry element debug info so the headless layout tests can query geometry
    // (tests/ui_layout_collision.rs); release builds stay lean.
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into())
        .with_debug_info(!is_release);
    slint_build::compile_with_config("slint-project.json", config)
        .expect("Failed to compile Slint 1.19 project");

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // Set permanent executable icon and rich PE version metadata for Windows builds
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/winblast.ico");
        res.set("CompanyName", "Konstrukturmx");
        res.set(
            "FileDescription",
            "WinBlast Optimizer — Windows System Optimizer & Hardware Latency Engine",
        );
        // PE version always follows Cargo.toml (it was hard-coded and went stale in 1.8.0).
        let version = format!("{}.0", std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".into()));
        res.set("FileVersion", &version);
        res.set("InternalName", "winblast0");
        res.set(
            "LegalCopyright",
            "Copyright © 2026 Konstrukturmx. Licensed under the MIT License.",
        );
        res.set("OriginalFilename", "winblast0.exe");
        res.set("ProductName", "WinBlast Optimizer");
        res.set("ProductVersion", &version);
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resource: {e}");
        }

        if std::env::var_os("WINBLAST_SKIP_MANIFEST").is_none() {
            // In debug profile, use AsInvoker so that `cargo test` and un-elevated debugging
            // run cleanly without OS error 740 (elevation required).
            // In release profile, request HighestAvailable for end-user deployment.
            let level = if is_release {
                HighestAvailable
            } else {
                AsInvoker
            };

            embed_manifest(new_manifest("WinBlast0.Optimizer").requested_execution_level(level))
                .expect("Failed to embed UAC manifest");
        }
    }
}
