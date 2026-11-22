# State Ledger & Restore Point Engine

> 12 nodes · cohesion 0.23

## Key Concepts

- **AppxCache** (20 connections) — `src/scanner/appx_scanner.rs`
- **.evaluate_package()** (4 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **check_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.iter()** (2 connections) — `src/scanner/appx_scanner.rs`
- **.load()** (2 connections) — `src/scanner/appx_scanner.rs`
- **Item** (1 connections)

## Relationships

- [appx_scanner.rs](appx_scanner.rs.md) (6 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (2 shared connections)
- [CachedScanBackend<'a>](CachedScanBackend_'a_.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [features_apps.rs](features_apps.rs.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [HibernationStatus](HibernationStatus.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/appx_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 34 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*