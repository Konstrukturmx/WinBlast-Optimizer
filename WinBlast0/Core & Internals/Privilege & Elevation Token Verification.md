---
title: Privilege & Elevation Token Verification
tags: [core, security, uac, elevation, win32, manifest]
created: 2026-11-08
---

# 🔐 Privilege & Elevation Token Verification

Modifying system-wide HKLM registry paths, system services, and creating System Restore points requires full Administrator elevation.

---

## 🏛️ Elevation Inspection (`src/core/elevation.rs`)

Rather than relying on basic heuristics, WinBlast0 queries the **Windows Token Information** directly using the official Microsoft Win32 bindings (`windows::Win32`):

```rust
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

pub fn is_elevated() -> bool {
    unsafe {
        let mut token_handle = HANDLE::default();
        let process = GetCurrentProcess();
        if OpenProcessToken(process, TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut return_length = 0u32;
        let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            size,
            &mut return_length,
        );

        let _ = CloseHandle(token_handle);
        result.is_ok() && elevation.TokenIsElevated != 0
    }
}
```

---

## 🛡️ Manifest Configuration & Graceful Inspection Mode

In `build.rs`:
```rust
embed_manifest(
    new_manifest("WinBlast0.Optimizer")
        .requested_execution_level(HighestAvailable),
)
```
- **Normal Execution**: When launched normally, Windows automatically displays the UAC elevation prompt, requesting administrative privileges.
- **Inspection Mode**: If running in an environment without interactive UAC or under un-elevated CI/headless test environments (`WINBLAST_SKIP_MANIFEST=1`), WinBlast0 gracefully falls back to **Inspection Mode (Read-Only)**. Scanning, telemetry, and diagnostics function completely; only system mutations are gated behind an elevation check.

---

## 🔗 Related Notes
- [[Windows OS Detection & Edition Aliases]]
- [[State Ledger & Rollback Engine]]
- [[00 - Index (MOC)]]
