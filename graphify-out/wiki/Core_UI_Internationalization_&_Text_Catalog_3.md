# Core UI Internationalization & Text Catalog

> 11 nodes · cohesion 0.22

## Key Concepts

- **WindowsInfo** (16 connections) — `src/core/os_detect.rs`
- **.detect_os()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_os()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_os()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_os()** (3 connections) — `src/core/backend.rs`
- **.detect_os()** (2 connections) — `src/core/backend.rs`
- **.detect_os()** (2 connections) — `src/core/backend.rs`
- **test_windows_info_detect_host()** (2 connections) — `src/core/os_detect.rs`
- **.detect()** (2 connections) — `src/core/os_detect.rs`
- **.has_recall()** (1 connections) — `src/core/os_detect.rs`
- **.supports_hags()** (1 connections) — `src/core/os_detect.rs`

## Relationships

- [BlastResult](BlastResult.md) (3 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [Privacy, Telemetry & AI Hardening Catalog](Privacy,_Telemetry_&_AI_Hardening_Catalog.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/os_detect.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 30 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*