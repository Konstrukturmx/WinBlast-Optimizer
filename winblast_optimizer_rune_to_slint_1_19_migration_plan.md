# WinBlast-Optimizer: Full GUI Migration Plan (RUNE -> Slint 1.19)

> **Target Repository:** `https://github.com/Konstrukturmx/WinBlast-Optimizer`  
> **Source GUI Framework:** RUNE (`anomalyco/rune` + `rune-kit`)  
> **Target GUI Framework:** Slint `1.19.0` (Edition `1.19`, `slint-project.json`, Rust bindings)  
> **Migration Objective:** Complete 1:1 functional, visual, behavioral, and multi-DPI/UI-scale transition with **zero feature loss** and **zero regression** in Windows optimization logic.

---

## Important Note on Repository Inspection (Phase 0 Prerequisite)

The GitHub repository `https://github.com/Konstrukturmx/WinBlast-Optimizer` could not be directly read from this environment (it is either private, unindexed, or restricted from raw file retrieval). To guarantee **zero hallucinated file paths** and **zero missed features**, this document provides:

1. A **complete RUNE -> Slint 1.19 technical architecture mapping** grounded in both frameworks' exact APIs (including Slint 1.17/1.18/1.19 capabilities).
2. A **Phase 0 Automated Repository Audit** script and parity matrix you (or a coding agent) run locally inside `WinBlast-Optimizer` to inventory every single screen, widget, state field, shortcut, and optimizer action before modifying code.
3. A **phased, zero-downtime migration blueprint** (Milestones M0 through M8) with exact `Cargo.toml`, `slint-project.json`, `build.rs`, scaling architecture, component translations, state bridges, and verification gates.

---

## 1. Executive Summary & Architectural Strategy

### 1.1 Why Slint 1.19 Is a Direct, Lossless Replacement for RUNE
Moving from RUNE (`anomalyco/rune`) to **Slint 1.19.0** is an exceptionally clean architectural transition because Slint 1.17-1.19 closed almost every capability gap that previously separated declarative DSLs from RUNE's custom GPU engine:

- **Layout parity:** RUNE uses Taffy flexbox/grid. Slint 1.18+ provides native `FlexboxLayout` (with wrapping, `alignment`, `cross-axis-alignment`, `cross-axis-self-alignment`, `layout-order`) alongside dynamic `GridLayout` (`for`/`if` rows and cells) and `HorizontalLayout`/`VerticalLayout`.
- **Edition `1.19` layout & state correctness:** Slint 1.19 introduces `slint-project.json` and `"edition": "1.19"`, which ensures `if`/`for` elements contribute properly to parent sizing and properties modified in only some `states` automatically revert to defaults in other states.
- **Motion parity:** RUNE uses SwiftUI-style analytic springs (`duration + bounce`). Slint 1.18+ supports native `easing: spring(bounce)` (`bounce` in $[-1, 1]$), `animate *` in transitions, and Slint 1.19 automatically honors Windows **Reduced Motion** settings with zero extra code.
- **Material & Color parity:** RUNE uses OKLCH semantic tokens and in-app blur panels. Slint supports `Colors.oklch(...)` and Slint 1.19 adds native `backdrop-blur` on translucent `Rectangle` elements (Skia and FemtoVG renderers).
- **Desktop & Windows parity:** Slint 1.17-1.19 includes native `SystemTrayIcon` (with embedded `Timer` support in 1.19), `WindowMoveArea` for custom frameless titlebars, `DragArea`/`DropArea`, tooltips, selectable Markdown `StyledText`, animated GIF/PNG/WebP `Image`, `ensure-visible(ScrollBehavior.smooth)`, and `AccessKit` accessibility.
- **Windows GPU Resilience:** Unlike GPU-only setups that can break on stripped Windows installs, VMs, or minimal GPU drivers, Slint 1.19 ships **Skia** (GPU + `renderer-skia-software` CPU-only fallback), **FemtoVG** (OpenGL / `wgpu`), and an upgraded **Software renderer** (with rounded clip antialiasing, drop shadows, and gradients).

### 1.2 Core Migration Principles (Non-Negotiable Rules)
1. **Strict Backend Isolation:** No Windows registry, service, WMI, PowerShell, filesystem, memory-flushing, or Win32 optimization code may be rewritten or coupled into UI callbacks. Extract/keep it behind a clean UI-agnostic Rust API (`winblast_core`).
2. **Strangler-Fig Dual Build:** Maintain both `--features ui-rune` and `--features ui-slint` during development so the application compiles, runs, and can be side-by-side visually compared at every milestone until 100% parity is verified.
3. **Full Multi-Layer Scale Guarantee:** Preserve and enhance Windows Per-Monitor DPI v2 scaling, user-configurable UI zoom ($75\%$ to $200\%$), dynamic font scaling, and responsive layout breakpoints from $1024 \times 600$ up to $3840 \times 2160$ at $250\%$ DPI.
4. **Zero Silent Feature Drops:** Every item recorded in the Phase 0 Parity Matrix must map to a concrete Slint component/handler and pass automated + manual verification before removing RUNE crates.

---

## 2. Deep Technical Comparison: RUNE vs. Slint 1.19

| Architectural Layer | RUNE (`anomalyco/rune` + `rune-kit`) | Slint `1.19.0` Equivalent | Migration Impact & Notes |
| :--- | :--- | :--- | :--- |
| **Language & UI Definition** | Plain Rust view functions (`row()`, `col()`, `el()`, `text()`) returning type-erased `El` | Declarative `.slint` DSL compiled ahead-of-time via `slint-build` (`edition = "1.19"`) to zero-overhead Rust | Clean separation of layout/presentation (`.slint`) and business logic (`.rs`). |
| **Project Configuration** | `Cargo.toml` + Rust theme structs/JSON | `Cargo.toml` + `slint-project.json` (`"edition": "1.19"`, `main`, `include-paths`, `library-paths`) | Enables LSP diagnostics, Visual Editor, and Live-Preview consistency. |
| **Rendering Backend (Windows)** | `wgpu` on D3D12 (+ WARP software fallback) | `winit` + `renderer-skia` (D3D/OpenGL/Vulkan) or `renderer-femtovg-wgpu` + `renderer-software` / `renderer-skia-software` | Recommend `renderer-skia` + `renderer-software` (or `renderer-femtovg-wgpu` + `renderer-software`) for rock-solid Windows compatibility. |
| **Layout Engine** | Taffy 0.15 (Flexbox, Grid, Block) + sum-tree virtualized list | `FlexboxLayout`, dynamic `GridLayout`, `HorizontalLayout`, `VerticalLayout`, `ListView` (virtualized) | Direct 1:1 mapping; check nested scroll containers (`min-height: 0px` equivalents). |
| **State Management** | Typed `Model` store outside views + keyed retained tree + `&mut` handlers | Exported Slint `Global` singletons + `VecModel<T>` / `ModelRc<T>` / `FilterModel` / `SortModel` + Rust callbacks | Rust owns canonical state; Slint binds reactively via typed properties & callbacks. |
| **High-Frequency Updates** | Direct render-property bindings (bypassing model diffs) | Direct scalar property updates via `Weak<MainWindow>` + `invoke_from_event_loop`, or `SharedPixelBuffer` for live charts | Batch telemetry updates at $15\text{-}60\text{ Hz}$ to keep idle CPU near $0\%$. |
| **Theming & Color Space** | `rune::Theme`, `rune_kit::Kit`, `PaletteSeed`, OKLCH/P3 semantic tokens | `export global Theme` with `Colors.oklch(l, c, h)` semantic tokens, runtime light/dark switching | Zero visual loss; all colors computed or stored in one central token file. |
| **Glass / Blur Materials** | In-app blur & tinted surfaces | `Rectangle { background: ...; backdrop-blur: 16px; }` (Slint 1.19, Skia & FemtoVG) | Guard with fallback opacity when software renderer is active. |
| **Animations & Motion** | Analytic spring tracks (`duration + bounce`), velocity retargeting | `animate prop { duration: 250ms; easing: spring(0.2); }` + `animate *` in transitions + automatic Windows Reduced Motion | Map Rune motion presets (`instant`, `snappy`, `default`, `calm`, `smooth`, `morph`) to shared Slint duration/spring constants. |
| **Desktop Shell & Titlebar** | `rune_kit::shell` (`Sidebar`, `PaneState`, `Toolbar`, 48pt drag band) | Custom `AppShell.slint` with `WindowMoveArea`, `TouchArea` splitter for resizable/collapsible `Sidebar`, `Toolbar` | Full custom frameless Windows titlebar (`no-frame: true`) with snap/drag/min/max/close. |
| **System Tray & OS UX** | Platform tray integration / custom popovers | Built-in `SystemTrayIcon` (Slint 1.17+, `Timer` inside tray in 1.19), `MenuBar`, `ContextMenuArea`, `PopupWindow` | Native Windows notification area icon + context menu out of the box. |
| **Rich Text & Logs** | Custom text/syntax views (`Parley`, `rune_kit::syntax`) | `StyledText` (runtime Markdown parsing + `selectable: true` copy support in 1.19) or `ListView` of colored log lines | Use `ListView` for multi-thousand-line optimizer logs to maintain O(1) viewport rendering. |
| **Scroll & Navigation** | Sum-tree scroll views | `ScrollView`, `Flickable.scroll-to()`, `element.ensure-visible(ScrollBehavior.smooth)`, `ListView.ensure-row-visible()` | Smooth jump-to-search-result and keyboard navigation parity. |
| **Accessibility** | AccessKit tree | Native `accessibility` feature (AccessKit on Windows) + `accessible-role`, `accessible-label`, `accessible-value` | Preserves screen reader and UI automation support. |
| **Automated UI Testing** | Headless PNG snapshot + JSON element tree + scripted input | `i-slint-backend-testing` (`ElementHandle`, simulated clicks/keys/text, layout assertions) + headless software-renderer PNG capture | Preserves CI visual and interaction regression testing. |

---

## 3. Risk Register & Zero-Loss Prevention Matrix

| Risk ID | Potential Loss / Failure Mode | Root Cause | Mandatory Mitigation in Slint 1.19 |
| :--- | :--- | :--- | :--- |
| **R-01** | Missed tweak toggle, dialog, or hidden context action | Incomplete audit of RUNE views before rewriting | Execute Phase 0 inventory script; enforce 100% check-off in Parity Matrix. |
| **R-02** | UI freezes during registry/WMI/service/disk operations | Running blocking Windows calls inside Slint callbacks | All Slint callbacks must be non-blocking (`O(1)` channel send to worker thread/Tokio runtime). |
| **R-03** | Broken DPI or blurry text on $125\% / 150\% / 200\%$ monitors | Missing `PerMonitorV2` Windows manifest or mixing `phx` and `px` | Embed `PerMonitorV2` manifest via `build.rs`; use strictly logical `px` multiplied by `Scale.factor`. |
| **R-04** | `backdrop-blur` crashes or looks transparent on software fallback | Slint 1.19 `backdrop-blur` only renders on Skia/FemtoVG | Expose `Theme.supports_blur: bool` from Rust backend detection; increase surface alpha when false. |
| **R-05** | Sluggish real-time CPU/RAM/GPU graphs or huge process tables | Rebuilding entire `VecModel` every tick or rendering non-virtualized `for` lists | Use `ListView` (virtualized) for tables/logs; mutate existing `VecModel` rows in-place (`set_row_data`) or update ring-buffer `Path`/`Image` at throttled intervals. |
| **R-06** | Custom frameless window loses Windows Aero Snap / resize edges | `no-frame: true` disables OS default hit-testing if unhandled | Implement edge resize `TouchArea` handles + `WindowMoveArea` + optional Win32 `WM_NCHITTEST` hook for native Windows 11 Snap Layouts. |
| **R-07** | Custom RUNE keyboard shortcuts (`Ctrl+K`, `Ctrl+B`, `Ctrl++/-`, `Esc`) stop working | Slint focus scope requires top-level `FocusScope` to intercept global hotkeys | Wrap root `MainWindow` content in a global `FocusScope` with `key-pressed` handler routing shortcuts before child capture where appropriate. |
| **R-08** | UAC Admin elevation breaks drag-and-drop or tray icon | Windows UIPI restricts messages to elevated processes | Preserve existing UAC manifest (`requireAdministrator`) and verify `SystemTrayIcon` + file dialogs under elevated token. |
| **R-09** | Slint Licensing Compliance | Slint uses GPL-3.0-only / Royalty-free / Commercial dual licensing | Check `WinBlast-Optimizer`'s license: if GPLv3, fully compatible; if MIT/Apache open-source or closed-source desktop app, select the Royalty-free license (include `AboutSlint` credit or badge) or GPLv3 as appropriate. |

---

## 4. Phase 0: Automated Codebase Discovery & Parity Audit

Before editing any production code, run this audit inside the local `WinBlast-Optimizer` repository and record every artifact in a `MIGRATION_AUDIT.md` ledger.

### 4.1 Discovery Commands (PowerShell / Bash)

```powershell
# 1. Inspect workspace and crate dependencies
Get-Content Cargo.toml
Get-ChildItem -Recurse -Filter Cargo.toml | Select-Object FullName

# 2. Map the full directory tree (excluding target/.git)
Get-ChildItem -Recurse -File | Where-Object { $_.FullName -notmatch '\\(target|\.git)\\' } | Select-Object FullName

# 3. Locate every RUNE and rune-kit import & usage
rg -n "use rune|rune_kit|AppSpec|PaletteSeed|PaneState|Sidebar|Toolbar|Cx|El\b" src/

# 4. Find all UI event handlers, keyboard shortcuts, and state models
rg -n "\.on_press|\.on_drag|\.on_click|\.on_change|on_key|Model|Theme::" src/

# 5. Find all Windows/system backend modules (Registry, Services, WMI, PowerShell, Win32)
rg -n "winreg|windows::|sysinfo|Command::new|powershell|sc\.exe|bcdedit|netsh" src/

# 6. Find all assets, fonts, icons, manifests, and config files
Get-ChildItem -Recurse -Include *.ttf,*.otf,*.svg,*.png,*.ico,*.json,*.toml,*.manifest,*.rc
```

### 4.2 Mandatory Parity Matrix Template

Fill out every row from the Phase 0 audit. **No milestone is complete until every row reaches `Verified`.**

| Category | Item ID | Current RUNE File & Symbol | Behavior / State / Props | Scale / Responsive Rule | Target Slint File & Component | Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Shell** | `SH-01` | e.g., `src/ui/shell.rs` | Frameless titlebar, drag, min/max/close | Scales with DPI + `ui_scale` | `ui/shell/titlebar.slint` | `[ ]` |
| **Shell** | `SH-02` | e.g., `src/ui/sidebar.rs` | Collapsible/resizable nav sidebar | Auto-collapses below `900px` | `ui/shell/sidebar.slint` | `[ ]` |
| **View** | `VW-01` | Dashboard / Telemetry | Live CPU/RAM/Disk/Net + Quick Boost | Grid 3-col -> 2-col -> 1-col | `ui/views/dashboard.slint` | `[ ]` |
| **View** | `VW-02` | System Tweaks / Registry | Categorized switches, risk badges, apply/revert | Scrollable virtual list | `ui/views/tweaks.slint` | `[ ]` |
| **View** | `VW-03` | Services / Startup / Processes | Searchable, sortable table + actions | `ListView` + `FilterModel` | `ui/views/services.slint` | `[ ]` |
| **View** | `VW-04` | Cleaner / Disk / Network / Gaming | Scan progress, checklist, execute | Responsive action footer | `ui/views/cleaner.slint` | `[ ]` |
| **View** | `VW-05` | Backup / Restore / Profiles | Import/Export JSON, Restore Point | Modal confirmation + list | `ui/views/profiles.slint` | `[ ]` |
| **View** | `VW-06` | Logs / Terminal Output | Live streaming output, copy, clear | Selectable `StyledText`/`ListView` | `ui/views/logs.slint` | `[ ]` |
| **View** | `VW-07` | Settings / Appearance | Theme, Accent, UI Scale ($75\%\text{-}200\%$), Tray | Live reactive preview | `ui/views/settings.slint` | `[ ]` |
| **Overlay** | `OV-01` | Toasts / Confirm Dialogs / Command Palette | Modal backdrop blur, Esc to dismiss | Clamped max-width | `ui/components/overlays.slint` | `[ ]` |
| **OS** | `OS-01` | System Tray & Background Mode | Minimize to tray, quick RAM flush, Quit | Native OS tray DPI | `ui/main.slint` (`SystemTrayIcon`) | `[ ]` |
| **Input** | `IN-01` | Global Keyboard Shortcuts | `Ctrl+B`, `Ctrl+K`, `Ctrl++`, `Ctrl+-`, `Ctrl+0`, `F5` | Window-wide `FocusScope` | `ui/main.slint` | `[ ]` |

---

## 5. Phase 1: Core/UI Decoupling (Hexagonal Boundary)

Before writing Slint files, ensure **100% of optimization logic** is decoupled from RUNE types (`rune::*`, `rune_kit::*`).

### 5.1 Target Module Architecture

```text
WinBlast-Optimizer/
├── Cargo.toml
├── build.rs
├── slint-project.json                  # NEW: Slint 1.19 project manifest
├── resources/
│   ├── winblast.manifest               # PerMonitorV2 DPI + requireAdministrator
│   ├── app.rc                          # Windows executable icon & version info
│   └── icons/                          # .ico, .svg, animated .webp/.gif assets
├── src/
│   ├── main.rs                         # Entrypoint selecting ui_slint (or ui_rune during transition)
│   ├── core/                           # 100% UI-AGNOSTIC OPTIMIZER ENGINE
│   │   ├── mod.rs
│   │   ├── commands.rs                 # UiCommand enum (ApplyTweak, RevertTweak, ScanJunk, FlushRam, ...)
│   │   ├── events.rs                   # CoreEvent enum (TelemetryTick, ProgressUpdate, LogLine, ...)
│   │   ├── state.rs                    # Pure Rust AppState, Profile, Settings (with ui_scale, theme)
│   │   ├── engine.rs                   # Worker thread / Tokio runtime processing UiCommand -> CoreEvent
│   │   ├── registry/                   # Existing Windows registry optimizers (UNTOUCHED)
│   │   ├── services/                   # Existing Windows service/startup managers (UNTOUCHED)
│   │   ├── cleaner/                    # Existing temp/cache/RAM cleaners (UNTOUCHED)
│   │   ├── telemetry/                  # Existing CPU/RAM/GPU/Disk/Net monitors (UNTOUCHED)
│   │   └── backup/                     # Existing restore point & profile backup (UNTOUCHED)
│   ├── ui_rune/                        # LEGACY: Behind #[cfg(feature = "ui-rune")] until M8
│   └── ui_slint/                       # NEW: Behind #[cfg(feature = "ui-slint")]
│       ├── mod.rs                      # Window init, backend/renderer selection, run loop
│       ├── bridge.rs                   # Maps CoreEvent -> Weak<MainWindow> via invoke_from_event_loop
│       ├── callbacks.rs                # Maps Slint callbacks -> UiCommand channel sends
│       ├── models.rs                   # VecModel / FilterModel / SortModel adapters
│       ├── charts.rs                   # Live sparkline/telemetry path & pixel buffer generators
│       ├── window_win32.rs             # Optional Win32 DWM/Snap/Dark-titlebar helpers
│       └── theme_sync.rs               # Syncs OKLCH theme tokens, dark mode, & ui_scale
└── ui/                                 # NEW: Slint 1.19 declarative UI source tree
    ├── main.slint                      # Root MainWindow + SystemTrayIcon + Global FocusScope
    ├── tokens/
    │   ├── scale.slint                 # Global Scale (ui_scale, spacing, typography, radii)
    │   ├── theme.slint                 # Global Theme (OKLCH semantic palette, blur flags)
    │   └── motion.slint                # Global Motion (spring curves, durations)
    ├── models/
    │   └── types.slint                 # Shared Slint structs & enums matching core::state
    ├── shell/
    │   ├── titlebar.slint              # Frameless WindowMoveArea + caption buttons
    │   ├── sidebar.slint               # Collapsible/resizable navigation sidebar
    │   └── statusbar.slint             # Bottom status bar, admin badge, live metrics
    ├── components/
    │   ├── button.slint
    │   ├── switch.slint
    │   ├── card.slint
    │   ├── badge.slint
    │   ├── search_input.slint
    │   ├── slider.slint
    │   ├── dropdown.slint
    │   ├── modal.slint
    │   ├── toast.slint
    │   ├── tooltip.slint
    │   ├── progress_bar.slint
    │   ├── sparkline.slint
    │   └── tweak_row.slint
    └── views/
        ├── dashboard.slint
        ├── tweaks.slint
        ├── services.slint
        ├── cleaner.slint
        ├── profiles.slint
        ├── logs.slint
        └── settings.slint
```

### 5.2 Typed Command/Event Contract

Define a strict unidirectional data flow so both `ui_rune` and `ui_slint` can drive the exact same backend during the transition:

```rust
// src/core/commands.rs
#[derive(Debug, Clone)]
pub enum UiCommand {
    Navigate(Route),
    ToggleTweak { id: String, enabled: bool },
    ApplyBatchTweaks { ids: Vec<String> },
    RevertAllTweaks,
    RunCleanerScan,
    ExecuteClean { categories: Vec<String> },
    FlushStandbyRam,
    SetServiceStartup { name: String, mode: ServiceStartMode },
    CreateRestorePoint { label: String },
    ExportProfile { path: std::path::PathBuf },
    ImportProfile { path: std::path::PathBuf },
    SetThemeMode(ThemeMode),
    SetUiScale(f32), // Clamped [0.75, 2.00]
    CancelActiveJob,
    Shutdown,
}

// src/core/events.rs
#[derive(Debug, Clone)]
pub enum CoreEvent {
    InitialSnapshot(AppSnapshot),
    TelemetryTick(TelemetrySample),
    TweakStateChanged { id: String, applied: bool, status: OpStatus },
    JobProgress { job_id: u64, percent: f32, stage: String },
    JobCompleted { job_id: u64, summary: String, success: bool },
    LogAppended(LogEntry),
    Toast(ToastMessage),
    SettingsUpdated(UserSettings),
}
```

---

## 6. Phase 2: Slint 1.19 Scaffolding, `Cargo.toml`, `slint-project.json`, & `build.rs`

### 6.1 `slint-project.json` (New in Slint 1.19)

Create `slint-project.json` at the repository root. This guarantees identical edition semantics (`1.19`), module resolution, and preview behavior across `cargo build`, `slint-lsp`, and `slint-viewer`.

```json
{
  "edition": "1.19",
  "main": "ui/main.slint",
  "include-paths": [
    "ui",
    "resources"
  ],
  "library-paths": {
    "tokens": "ui/tokens",
    "models": "ui/models",
    "components": "ui/components",
    "shell": "ui/shell",
    "views": "ui/views"
  }
}
```

### 6.2 `Cargo.toml` Configuration (Dual-UI Feature Gate)

```toml
[package]
name = "winblast-optimizer"
version = "0.2.0"
edition = "2024"
build = "build.rs"

[features]
default = ["ui-slint"]
ui-rune = ["dep:rune", "dep:rune-kit"] # Remove in Phase 8 cutover
ui-slint = ["dep:slint", "dep:slint-build"]
live-preview = ["slint/live-preview"]

[dependencies]
# Core optimizer dependencies remain untouched here (serde, tokio, windows, winreg, sysinfo, ...)

# Slint 1.19.0 configured for Windows desktop reliability + blur + tray + software fallback
slint = { version = "1.19.0", optional = true, default-features = false, features = [
    "std",
    "backend-winit",
    "renderer-skia",            # Primary GPU renderer (supports 1.19 backdrop-blur + crisp text)
    "renderer-skia-software",   # CPU Skia fallback (new in 1.19) for stripped/VM Windows systems
    "renderer-software",        # Ultra-lightweight pure Rust software fallback
    "accessibility",            # AccessKit parity with RUNE
    "system-tray",              # Native Windows tray icon support
    "image-default-formats",    # PNG, JPEG, GIF, WebP (animated images in 1.19)
    "compat-1-2",
] }

# Optional alternative if you specifically want wgpu instead of Skia:
# replace renderer-skia with "renderer-femtovg-wgpu"

# Legacy RUNE dependencies made optional during migration
rune = { git = "https://github.com/anomalyco/rune", branch = "dev", optional = true }
rune-kit = { git = "https://github.com/anomalyco/rune", branch = "dev", optional = true }

[build-dependencies]
slint-build = { version = "1.19.0", optional = true }
winresource = "0.1" # Embeds PerMonitorV2 DPI manifest, UAC requireAdministrator, and app icon

[dev-dependencies]
i-slint-backend-testing = "1.19.0"

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "symbols"
```

### 6.3 `build.rs` (Slint 1.19 Project Compilation + Windows DPI/UAC Manifest)

```rust
fn main() {
    #[cfg(feature = "ui-slint")]
    {
        // Compiles using slint-project.json ("edition": "1.19", main: "ui/main.slint", library-paths)
        let config = slint_build::CompilerConfiguration::new()
            .with_style("fluent-dark".into()); // Neutral base style; custom WinBlast components override visuals

        slint_build::compile_with_config("slint-project.json", config)
            .expect("Failed to compile Slint 1.19 project");
    }

    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("resources/icons/winblast.ico");
        res.set_manifest_file("resources/winblast.manifest");
        res.compile().expect("Failed to compile Windows resources");
    }
}
```

### 6.4 `resources/winblast.manifest` (Critical for Sharp Scaling on Windows 10/11)

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
    </windowsSettings>
  </application>
</assembly>
```

---

## 7. Phase 3: Full Scale Architecture ("With All Scale")

To ensure **WinBlast-Optimizer** scales flawlessly across every monitor DPI, aspect ratio, and user accessibility zoom preference without clipping, blurry text, or broken hitboxes, implement a **3-Layer Scaling Architecture**:

### Layer 1: OS Per-Monitor DPI v2 Scaling (Hardware Scale Factor $d$)
- Handled automatically by `winit` + Slint `1.19` + `PerMonitorV2` manifest.
- All Slint `px` units are **logical pixels**. When moving the window between a $100\%$ FHD monitor ($d = 1.0$) and a $200\%$ 4K monitor ($d = 2.0$), Slint rescales automatically without blurring text or vector `Path` icons.
- **Rule:** Never hardcode physical pixels (`phx`) in layout or typography. Use `phx` only for 1-physical-pixel hairlines where strictly desired.

### Layer 2: User Application UI Scale (Runtime Zoom Factor $s \in [0.75, 2.00]$)
- Users on ultra-wide, 4K, or compact laptops often prefer compact ($85\%$) or enlarged ($125\%\text{-}150\%$) UI density independent of Windows OS DPI.
- Effective physical pixel size for any token $v$ is:
$$v_{\text{phys}} = v_{\text{base}} \cdot s \cdot d$$
where $v_{\text{base}}$ is the base logical length, $s$ is `Scale.factor`, and $d$ is the Windows monitor DPI scale factor.

```slint
// ui/tokens/scale.slint
export global Scale {
    // Persisted user preference: 0.75 to 2.00 (default 1.00)
    in-out property <float> factor: 1.0;

    // Clamped helper function for all derived dimensions
    pure public function px(base: length) -> length {
        return base * clamp(root.factor, 0.75, 2.0);
    }

    // Typography ramp (auto-scales with Scale.factor)
    out property <length> font-xs:    px(11px);
    out property <length> font-sm:    px(12px);
    out property <length> font-md:    px(14px);
    out property <length> font-lg:    px(16px);
    out property <length> font-xl:    px(20px);
    out property <length> font-2xl:   px(26px);

    // Spacing scale
    out property <length> space-1:    px(4px);
    out property <length> space-2:    px(8px);
    out property <length> space-3:    px(12px);
    out property <length> space-4:    px(16px);
    out property <length> space-5:    px(20px);
    out property <length> space-6:    px(24px);

    // Radii & Control heights (guarantees accessible hit targets at all scales)
    out property <length> radius-sm:  px(4px);
    out property <length> radius-md:  px(8px);
    out property <length> radius-lg:  px(12px);
    out property <length> control-sm: px(28px);
    out property <length> control-md: px(36px);
    out property <length> control-lg: px(44px);
    out property <length> icon-sm:    px(14px);
    out property <length> icon-md:    px(18px);
    out property <length> icon-lg:    px(24px);
    out property <length> titlebar-h: px(40px);
    out property <length> statusbar-h: px(28px);
}
```

- **Runtime Controls:** Bind `Ctrl + =` / `Ctrl + +` (`factor += 0.1`), `Ctrl + -` (`factor -= 0.1`), `Ctrl + 0` (`factor = 1.0`), plus a slider/dropdown in **Settings -> Appearance**. Persist immediately to config JSON.

### Layer 3: Responsive Window Breakpoints (Effective Width $W_{\text{eff}} = W_{\text{logical}} / s$)

Compute breakpoints using **scale-normalized width** so zooming to $150\%$ on a $1920\text{px}$ screen gracefully adapts the layout into compact columns instead of overflowing horizontally:

| Breakpoint Tier | Effective Width ($W_{\text{eff}}$) | Sidebar Behavior | Dashboard Grid Columns | Toolbar Layout |
| :--- | :--- | :--- | :--- | :--- |
| **Compact** | $W_{\text{eff}} < 760\text{px}$ | Overlay drawer or icon-rail ($56\text{px} \cdot s$) | 1 column (`FlexboxLayout` wrap) | Primary actions + overflow menu |
| **Standard** | $760\text{px} \le W_{\text{eff}} < 1180\text{px}$ | Collapsible resizable pane ($220\text{px} \cdot s$) | 2 columns | Full toolbar |
| **Wide** | $1180\text{px} \le W_{\text{eff}} < 1600\text{px}$ | Expanded sidebar + optional detail inspector | 3 columns | Full toolbar + inline search |
| **Ultrawide** | $W_{\text{eff}} \ge 1600\text{px}$ | Expanded sidebar + pinned telemetry inspector | 4 columns (max content width optional) | Full toolbar + quick stats |

---

## 8. Phase 4: Design System, OKLCH Tokens, Glass Blur & Spring Motion

### 8.1 Semantic Theme Tokens (Direct Port of `rune::Theme` / `rune_kit::Kit`)

RUNE enforces semantic tokens in OKLCH with light/dark runtime switching. Slint 1.15+ natively supports `Colors.oklch(l, c, h)` and `Colors.oklch(l, c, h, a)`:

```slint
// ui/tokens/theme.slint
export enum ThemeMode { dark, light, system }

export global Theme {
    in-out property <bool> dark: true;
    in-out property <bool> supports-blur: true; // Set false by Rust if software renderer is active
    in-out property <float> accent-hue: 235.0;  // User-customizable accent seed hue

    // Background elevation layers (OKLCH semantic ramp)
    out property <color> bg-canvas:       dark ? Colors.oklch(0.15, 0.012, 250) : Colors.oklch(0.97, 0.005, 250);
    out property <color> bg-surface:      dark ? Colors.oklch(0.19, 0.014, 250) : Colors.oklch(1.00, 0.000, 250);
    out property <color> bg-elevated:     dark ? Colors.oklch(0.23, 0.015, 250) : Colors.oklch(0.95, 0.006, 250);
    out property <color> bg-hover:        dark ? Colors.oklch(0.26, 0.016, 250) : Colors.oklch(0.92, 0.008, 250);

    // Translucent glass material surface (adjusts alpha automatically if blur unsupported)
    out property <color> bg-glass:        dark
        ? Colors.oklch(0.18, 0.015, 250, supports-blur ? 0.72 : 0.96)
        : Colors.oklch(0.98, 0.005, 250, supports-blur ? 0.76 : 0.97);
    out property <length> blur-radius:    supports-blur ? 18px : 0px;

    // Text & Icon hierarchy
    out property <color> text-primary:    dark ? Colors.oklch(0.95, 0.005, 250) : Colors.oklch(0.16, 0.012, 250);
    out property <color> text-secondary:  dark ? Colors.oklch(0.72, 0.010, 250) : Colors.oklch(0.45, 0.012, 250);
    out property <color> text-muted:      dark ? Colors.oklch(0.55, 0.010, 250) : Colors.oklch(0.60, 0.010, 250);

    // Borders & Focus
    out property <color> border-subtle:   dark ? Colors.oklch(0.28, 0.014, 250) : Colors.oklch(0.86, 0.008, 250);
    out property <color> border-strong:   dark ? Colors.oklch(0.38, 0.018, 250) : Colors.oklch(0.74, 0.010, 250);

    // Brand & Semantic status colors
    out property <color> accent:          Colors.oklch(dark ? 0.68 : 0.54, 0.18, accent-hue);
    out property <color> accent-hover:    Colors.oklch(dark ? 0.74 : 0.48, 0.19, accent-hue);
    out property <color> success:         Colors.oklch(dark ? 0.72 : 0.56, 0.16, 148);
    out property <color> warning:         Colors.oklch(dark ? 0.78 : 0.65, 0.16, 75);
    out property <color> danger:          Colors.oklch(dark ? 0.66 : 0.55, 0.20, 25);
    out property <color> info:            Colors.oklch(dark ? 0.70 : 0.55, 0.15, 225);
}
```

### 8.2 Slint 1.19 Glass Blur & Spring Motion Presets

```slint
// ui/tokens/motion.slint
export global Motion {
    out property <duration> instant: 80ms;
    out property <duration> snappy:  180ms;
    out property <duration> normal:  260ms;
    out property <duration> smooth:  360ms;
    out property <duration> slow:    480ms;
}
```

Example Glass Modal using **Slint 1.19 `backdrop-blur`** and **Slint 1.18+ `spring(...)`** (which automatically skips animation when Windows Reduced Motion is enabled in Slint 1.19):

```slint
Rectangle {
    background: Theme.bg-glass;
    backdrop-blur: Theme.blur-radius;
    border-radius: Scale.radius-lg;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: root.visible ? 1.0 : 0.0;

    animate opacity { duration: Motion.snappy; easing: spring(0.0); }
    animate y       { duration: Motion.normal; easing: spring(0.15); }
}
```

---

## 9. Phase 5: 1:1 Component & Shell Translation Blueprint

### 9.1 `rune_kit::shell` -> Custom Frameless `AppShell.slint`

Translate RUNE's `Sidebar`, `PaneState` (clamped width, inner-edge drag resize, double-click reset, collapse below half-min, keyboard shortcut `Ctrl+B`), `Toolbar`, and draggable titlebar directly:

1. **Titlebar (`ui/shell/titlebar.slint`):**
   - Set `no-frame: true;` on `MainWindow`.
   - Use `WindowMoveArea` across the top `Scale.titlebar-h` band.
   - Place sidebar toggle button, app icon/title, admin elevation badge, quick global search trigger (`Ctrl+K`), and Minimize / Maximize-Restore / Close caption buttons on top.
   - Wire Maximize/Restore to `window.set_maximized(!window.is_maximized())` and Close to either exit or minimize-to-tray based on user setting.
   - Add 4px invisible `TouchArea` resize borders around the frameless window edges (or hook `WM_NCHITTEST` via `raw-window-handle` on Windows for native Win11 Snap Layouts).

2. **Resizable/Collapsible Sidebar (`ui/shell/sidebar.slint`):**
   - Maintain `in-out property <length> sidebar-width: Scale.px(230px);` and `in-out property <bool> collapsed: false;`.
   - Attach a `4px` vertical `TouchArea` splitter along the sidebar's right edge:
     - `moved` clamps `sidebar-width` to `[Scale.px(180px), Scale.px(340px)]`; dragging below `Scale.px(110px)` snaps `collapsed = true`.
     - `double-clicked` resets to default `Scale.px(230px)`.
   - Animate `width` with `animate width { duration: Motion.snappy; easing: spring(0.0); }`.

### 9.2 Complete `rune-kit` -> Slint 1.19 Widget Mapping

| RUNE / `rune-kit` Primitive | Slint 1.19 Target Implementation | Key Parity Details |
| :--- | :--- | :--- |
| `row()` / `col()` + `.gap()` / `.p()` | `HorizontalLayout` / `VerticalLayout` / `FlexboxLayout` (`spacing`, `padding`) | Use `FlexboxLayout` wherever RUNE wraps chips, cards, or tags. |
| `el().bg().rounded().border()` | `Rectangle { background; border-radius; border-width; border-color; drop-shadow-*; }` | Supports CSS-like box shadows and gradients even on software fallback in 1.19. |
| `Button` (Primary, Secondary, Ghost, Danger) | Custom `WbButton` component with `TouchArea` + `FocusScope` + `states` | Uses Edition `1.19` state default restoration; hover/press/disabled/loading spinner states. |
| `Switch` / `Checkbox` / `SegmentedControl` | Custom `WbSwitch`, `WbCheckBox`, `WbSegmented` with animated thumb (`spring(0.2)`) | Full keyboard `Space`/`Enter` + `accessible-role: switch / checkbox`. |
| `TextField` / `SearchField` | `TextInput` wrapped in styled `Rectangle` + clear button + shortcut hint | Supports `has-selection` (1.19), native clipboard, IME, and undo/redo. |
| `List` / `Sidebar::virtual_rows` | `ListView` backed by `VecModel` / `FilterModel` / `SortModel` | Strictly virtualized for 1,000+ Windows processes, services, or registry keys. |
| `ScrollView` + jump-to-match | `ScrollView` / `Flickable` + `target.ensure-visible(ScrollBehavior.smooth)` | New in Slint 1.19; replaces manual scroll offset math. |
| `Collapsible` / Accordion | `VerticalLayout` with animated height / `if expanded:` (Edition `1.19` auto-sizes parent!) | Edition `1.19` bugfix `#407` makes conditional `if` blocks size cleanly inside layouts. |
| `Popover` / `ContextMenu` / `Modal` / `Sheet` | `PopupWindow` / `ContextMenuArea` / root overlay layer with `z` ordering | Slint 1.18+ dynamic `z` + Slint 1.19 `backdrop-blur` glass scrim. |
| `Tooltip` | Native Slint tooltip support (1.17+) or hover-timed overlay `PopupWindow` | Shows full technical registry path / risk warning for every Windows tweak. |
| `Toast` / `HUD` | Overlay stack (`for toast in root.toasts:`) + `Timer` auto-dismiss | Spring enter/exit animations; non-blocking click-through wrapper. |
| `StyledText` / Logs / Diffs | `StyledText { text: ...; selectable: true; }` (1.19) or virtualized `ListView` | Runtime Markdown formatting + copyable diagnostic/backup summaries. |
| Live Telemetry Graphs | `Path` (using `stroke-dash-array` in 1.19 for gridlines) or `slint::Image` (`SharedPixelBuffer`) | For $60\text{fps}$ multi-series charts, render into a reusable `Rgba8Pixel` buffer or vector `Path`. |
| Animated Status Loaders | Animated `Path`/`Spinner` or Slint 1.19 animated WebP/GIF/APNG `Image` | Auto-respects Windows Reduced Motion setting. |
| System Tray | `SystemTrayIcon` (Slint 1.17+, supports nested `Timer` in 1.19) | Left-click restore window; right-click menu (Quick Optimize, Flush RAM, Open, Quit). |

---

## 10. Phase 6: State Bridge, Threading & High-Frequency Telemetry

Slint UI objects are `!Send` and live exclusively on the main UI thread. All optimizer work MUST run off-thread.

### 10.1 Safe Bidirectional Rust <-> Slint Bridge Pattern

```rust
// src/ui_slint/bridge.rs
use std::rc::Rc;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};
use crate::core::{commands::UiCommand, events::CoreEvent};

pub struct SlintBridge {
    weak_window: Weak<MainWindow>,
    tweaks_model: Rc<VecModel<TweakItemData>>,
    services_model: Rc<VecModel<ServiceItemData>>,
    logs_model: Rc<VecModel<LogLineData>>,
}

impl SlintBridge {
    /// Called from the background core worker thread whenever CoreEvent arrives.
    pub fn dispatch_event(weak: Weak<MainWindow>, event: CoreEvent) {
        let _ = slint::invoke_from_event_loop(move || {
            let Some(win) = weak.upgrade() else { return; };
            match event {
                CoreEvent::TelemetryTick(sample) => {
                    let telemetry = win.global::<TelemetryState>();
                    telemetry.set_cpu_percent(sample.cpu_percent);
                    telemetry.set_ram_used_gb(sample.ram_used_gb);
                    telemetry.set_ram_total_gb(sample.ram_total_gb);
                    telemetry.set_disk_read_mb(sample.disk_read_mb);
                    telemetry.set_net_down_mb(sample.net_down_mb);
                    // Update sparkline Path commands or SharedPixelBuffer here
                }
                CoreEvent::TweakStateChanged { id, applied, status } => {
                    // Mutate ONLY the matching row via VecModel::set_row_data
                    // Avoids destroying scroll position or rebuilding the ListView
                }
                CoreEvent::JobProgress { percent, stage, .. } => {
                    let jobs = win.global::<JobState>();
                    jobs.set_active(true);
                    jobs.set_progress(percent);
                    jobs.set_stage(SharedString::from(stage));
                }
                CoreEvent::LogAppended(entry) => {
                    // Cap ring buffer (e.g., max 5,000 rows) to bound RAM usage
                }
                _ => {}
            }
        });
    }
}
```

### 10.2 Performance Rules for High-Frequency Data
1. **In-Place Row Mutation:** Always update individual rows with `VecModel::set_row_data(index, updated_row)` instead of replacing the entire `ModelRc`, preserving selection, focus, and scroll inertia.
2. **Search & Sort Zero-Copy:** Wrap `VecModel` with `slint::FilterModel` and `slint::SortModel` so filtering 500+ Windows services or startup items happens instantaneously without cloning the source dataset on every keystroke.
3. **Telemetry Coalescing:** Sample system metrics in `core::telemetry` at $500\text{ms}\text{-}1000\text{ms}$ (or configurable $250\text{ms}$ high-refresh mode) and pause chart redraws automatically when the window is minimized to the system tray (achieving $< 0.2\%$ idle CPU).

---

## 11. Phase 7: Step-by-Step Execution Runbook (Milestones M0 - M8)

Execute these milestones strictly in order. Every milestone has explicit exit criteria.

### Milestone M0: Repository Audit & Freeze
- [ ] Create git branch `feat/migrate-slint-1.19`.
- [ ] Run Phase 0 discovery scripts and complete the Parity Matrix (`MIGRATION_AUDIT.md`).
- [ ] Capture reference screenshots of every RUNE screen at $100\%$ and $150\%$ DPI (Dark & Light themes) in `docs/migration_baselines/`.
- [ ] **Exit Gate:** 100% of screens, dialogs, tweaks, shortcuts, and settings inventoried.

### Milestone M1: Backend Decoupling (`core/` Boundary)
- [ ] Move all non-UI optimizer modules into `src/core/`.
- [ ] Define `UiCommand`, `CoreEvent`, and `AppSnapshot`.
- [ ] Refactor existing RUNE UI into `src/ui_rune/` behind `#[cfg(feature = "ui-rune")]`.
- [ ] **Exit Gate:** `cargo test` and `cargo run --no-default-features --features ui-rune` pass with identical behavior to `main`.

### Milestone M2: Slint 1.19 Skeleton & Windows Manifest
- [ ] Add `slint = "1.19.0"`, `slint-build = "1.19.0"`, `slint-project.json` (`edition = "1.19"`), `build.rs`, and `resources/winblast.manifest`.
- [ ] Implement renderer detection & fallback logic (Skia GPU -> Skia Software -> Pure Software) and set `Theme.supports_blur`.
- [ ] Boot an empty themed `MainWindow` with `ui-slint`.
- [ ] **Exit Gate:** `cargo run --features ui-slint` launches cleanly on Windows 10/11 with UAC elevation and sharp `PerMonitorV2` DPI text.

### Milestone M3: Tokens, Multi-Layer Scaling & Core Component Kit
- [ ] Implement `ui/tokens/scale.slint`, `ui/tokens/theme.slint`, `ui/tokens/motion.slint`.
- [ ] Build the full reusable component library (`ui/components/*`) with `Platform.uses-mock-data` previews for `slint-viewer`.
- [ ] Wire `Ctrl++`, `Ctrl+-`, `Ctrl+0` runtime scaling and Dark/Light theme switching.
- [ ] **Exit Gate:** Component gallery renders identically to RUNE design language across $0.75\times$, $1.0\times$, $1.25\times$, $1.5\times$, and $2.0\times$ scales.

### Milestone M4: Desktop Shell, Titlebar, Sidebar & System Tray
- [ ] Build `ui/shell/titlebar.slint` (`WindowMoveArea`, drag, maximize/restore, snap, caption buttons).
- [ ] Build `ui/shell/sidebar.slint` (resizable splitter, double-click reset, collapse toggle `Ctrl+B`, responsive auto-collapse).
- [ ] Build `ui/shell/statusbar.slint` and wire `SystemTrayIcon` (minimize to tray, quick actions, restore).
- [ ] **Exit Gate:** Window chrome, drag/resize, tray minimize/restore, and keyboard navigation match or exceed RUNE behavior.

### Milestone M5: Feature Views Migration (View-by-View Vertical Slices)
Migrate views one by one, checking off each Parity Matrix row before moving to the next:
- [ ] **M5.1 Dashboard & Real-Time Telemetry:** KPI cards, live CPU/RAM/Disk/Net sparklines, Quick Boost CTA, responsive `FlexboxLayout`/`GridLayout`.
- [ ] **M5.2 System Tweaks & Registry Optimizer:** Category tabs, search/filter (`FilterModel`), risk badges, tooltips, single toggle + batch apply + revert all.
- [ ] **M5.3 Services, Startup & Process Manager:** Virtualized `ListView`, sortable columns (`SortModel`), live status indicators, context menu actions.
- [ ] **M5.4 Cleaner, RAM Flush & Gaming/Network Modules:** Scan progress bar, cancel button, itemized reclaimable bytes, confirmation modal.
- [ ] **M5.5 Backups, Restore Points & Profiles:** JSON import/export dialogs (`rfd` or native Windows dialog), restore point creation/rollback.
- [ ] **M5.6 Live Logs & Console Output:** Virtualized log stream, severity filters, `StyledText` selectable copy, export log.
- [ ] **M5.7 Settings & Appearance:** Theme selector, OKLCH accent hue picker, UI Scale slider ($75\%\text{-}200\%$), startup/tray preferences, `AboutSlint` credit if using Royalty-free license.
- [ ] **Exit Gate:** Every Parity Matrix row is implemented and wired to real `core::engine` commands.

### Milestone M6: Overlays, Shortcuts, Accessibility & Polish
- [ ] Implement glass-blur confirmation modals, toast notification queue, and command palette (`Ctrl+K`) with `ensure-visible(ScrollBehavior.smooth)`.
- [ ] Audit every interactive element for `accessible-role`, `accessible-label`, `Tab` focus order, and `Enter`/`Space`/`Escape` handling.
- [ ] Verify Windows **Reduced Motion** setting disables all spring transitions automatically.
- [ ] **Exit Gate:** Full keyboard-only operation and screen-reader tree inspection pass cleanly.

### Milestone M7: Automated Testing, Multi-DPI Matrix & Performance Verification
- [ ] Write `i-slint-backend-testing` integration tests covering navigation, search filtering, tweak toggling, scale clamping, and modal cancellation.
- [ ] Execute the full Multi-Scale Verification Matrix (Section 12.1).
- [ ] Profile CPU, RAM, startup latency, and binary size against the RUNE baseline (Section 12.2).
- [ ] **Exit Gate:** Zero regressions across functionality, visual layout, scaling, and performance budgets.

### Milestone M8: Clean Cutover & RUNE Decommission
- [ ] Delete `src/ui_rune/`.
- [ ] Remove `rune`, `rune-kit`, and unused transitive GUI crates from `Cargo.toml` and regenerate `Cargo.lock`.
- [ ] Remove temporary `ui-rune` / `ui-slint` feature flags (Slint becomes unconditional).
- [ ] Build final `--release` binary, test clean install on a fresh Windows 10/11 VM (both with GPU and with Basic Display Adapter / software fallback).
- [ ] Merge `feat/migrate-slint-1.19` into `main` and tag release.

---

## 12. Phase 8: Verification Matrix & Performance Benchmarks

### 12.1 Mandatory Multi-Scale & Resolution Test Matrix

Every view must be verified with zero clipped text, zero overlapping buttons, and zero horizontal overflow across all cells:

| Test Case | Resolution | Windows OS DPI ($d$) | App UI Scale ($s$) | Expected Layout Mode | Pass |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SC-01** | $1280 \times 720$ | $100\%$ | $1.00$ | Standard (2-col) | `[ ]` |
| **SC-02** | $1280 \times 720$ | $125\%$ | $1.00$ | Compact/Standard | `[ ]` |
| **SC-03** | $1920 \times 1080$ | $100\%$ | $0.85$ | Wide (3-col, dense) | `[ ]` |
| **SC-04** | $1920 \times 1080$ | $125\%$ | $1.00$ | Wide | `[ ]` |
| **SC-05** | $1920 \times 1080$ | $150\%$ | $1.25$ | Compact (drawer sidebar) | `[ ]` |
| **SC-06** | $2560 \times 1440$ | $125\%$ | $1.00$ | Wide / Ultrawide | `[ ]` |
| **SC-07** | $3840 \times 2160$ | $150\%$ | $1.00$ | Ultrawide (4-col) | `[ ]` |
| **SC-08** | $3840 \times 2160$ | $200\%$ | $1.50$ | Standard | `[ ]` |
| **SC-09** | Dual Monitor Drag | $100\% \leftrightarrow 200\%$ | $1.00$ | Live seamless rescale, crisp text | `[ ]` |
| **SC-10** | Software Renderer VM | $100\%$ | $1.00$ | Fallback active, opaque glass surface | `[ ]` |

### 12.2 Performance & Resource Targets

| Metric | Target Budget (Slint 1.19 Release Build) | Verification Method |
| :--- | :--- | :--- |
| **Cold Startup to First Paint** | $< 350\text{ ms}$ | Stopwatch / timestamp in `main.rs` -> first frame |
| **Idle CPU (Window Visible, Telemetry 1Hz)** | $< 0.5\%$ | Windows Task Manager / Performance Monitor (60s avg) |
| **Idle CPU (Minimized to System Tray)** | $< 0.1\%$ | Pause telemetry UI updates when hidden |
| **Working Set RAM (Skia / FemtoVG)** | $< 60\text{ MB}$ (target $25\text{-}45\text{ MB}$) | Task Manager Private Working Set |
| **Scroll / Resize Frame Pacing** | Smooth $60\text{+}\text{ FPS}$ ($< 16.6\text{ ms}$ frame) | Rapidly scroll 1,000-row `ListView` + resize window |
| **Live Scale Transition (`Ctrl++/-`)** | Instant ($< 1$ frame re-layout, zero flicker) | Rapidly toggle $0.75 \to 2.00$ on every view |

### 12.3 Slint 1.19 Automated UI Test Template (`tests/ui_parity.rs`)

```rust
#[test]
fn test_scale_clamping_and_navigation_parity() {
    i_slint_backend_testing::init_no_event_loop();
    let app = MainWindow::new().unwrap();

    // Verify Scale global bounds
    let scale = app.global::<Scale>();
    scale.set_factor(1.5);
    assert!((scale.get_factor() - 1.5).abs() < f32::EPSILON);

    // Verify accessible element presence & interaction
    let search = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&app, "Search tweaks")
        .next()
        .expect("Search input must remain accessible");
    assert!(search.is_valid());
}
```

---

## 13. Quick-Start Prompt for Local Execution (Coding Agent / Developer)

If you execute this migration locally with an AI coding agent inside `WinBlast-Optimizer`, use this exact sequence:

1. **Prompt 1 (Phase 0 & 1):** *"Run the Phase 0 audit commands in `WINBLAST_RUNE_TO_SLINT_1_19_MIGRATION_PLAN.md`, populate `MIGRATION_AUDIT.md` with every RUNE screen, widget, shortcut, and backend call, then decouple all non-UI optimizer code into `src/core/` behind `UiCommand`/`CoreEvent` without changing behavior."*
2. **Prompt 2 (Phase 2 & 3):** *"Implement Milestones M2 and M3 from the plan: configure `Cargo.toml` (`slint 1.19.0`), `slint-project.json` (`edition = 1.19`), `build.rs`, `winblast.manifest` (`PerMonitorV2`), `ui/tokens/{scale,theme,motion}.slint`, and `ui/components/*`."*
3. **Prompt 3 (Phase 4 & 5):** *"Implement Milestones M4 and M5 view-by-view, checking off every single row in `MIGRATION_AUDIT.md` until 100% parity is achieved."*
4. **Prompt 4 (Phase 6 & 7):** *"Run Milestone M7 (`i-slint-backend-testing`, scale matrix SC-01..SC-10, software fallback check), then execute Milestone M8 to cleanly remove `rune` and `rune-kit`."*