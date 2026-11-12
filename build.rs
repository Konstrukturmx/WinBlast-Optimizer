// build.rs — Embeds UAC manifest & Windows metadata
use embed_manifest::{embed_manifest, new_manifest};
use embed_manifest::manifest::ExecutionLevel::{AsInvoker, HighestAvailable};

fn main() {
    println!("cargo:rerun-if-env-changed=WINBLAST_SKIP_MANIFEST");
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some()
        && std::env::var_os("WINBLAST_SKIP_MANIFEST").is_none()
    {
        // In debug profile, use AsInvoker so that `cargo test` and un-elevated debugging
        // run cleanly without OS error 740 (elevation required).
        // In release profile, request HighestAvailable for end-user deployment.
        let is_release = std::env::var("PROFILE").as_deref() == Ok("release");
        let level = if is_release {
            HighestAvailable
        } else {
            AsInvoker
        };

        embed_manifest(
            new_manifest("WinBlast0.Optimizer")
                .requested_execution_level(level),
        )
        .expect("Failed to embed UAC manifest");
    }
}

