# app.rs

> 22 nodes · cohesion 0.16

## Key Concepts

- **AppxCache** (20 connections) — `src/scanner/appx_scanner.rs`
- **appx_scanner.rs** (13 connections) — `src/scanner/appx_scanner.rs`
- **.evaluate_package_with_kind()** (7 connections) — `src/scanner/appx_scanner.rs`
- **.from_packages()** (5 connections) — `src/scanner/appx_scanner.rs`
- **appx_package_matches_family()** (4 connections) — `src/scanner/appx_scanner.rs`
- **.evaluate_package()** (4 connections) — `src/scanner/appx_scanner.rs`
- **split_appx_name_and_publisher()** (4 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.is_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **check_package_removed()** (3 connections) — `src/scanner/appx_scanner.rs`
- **test_appx_cache_loaded_empty_no_fallback()** (3 connections) — `src/scanner/appx_scanner.rs`
- **test_split_appx_name_and_publisher_with_underscores_and_prefix()** (3 connections) — `src/scanner/appx_scanner.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.load_appx_cache()** (2 connections) — `src/core/backend.rs`
- **.iter()** (2 connections) — `src/scanner/appx_scanner.rs`
- **.load()** (2 connections) — `src/scanner/appx_scanner.rs`
- **check_package_removed_ps()** (2 connections) — `src/scanner/appx_scanner.rs`
- **test_appx_publisher_suffix_matching()** (2 connections) — `src/scanner/appx_scanner.rs`
- **test_evaluate_package_with_kind_all_cache_branches()** (2 connections) — `src/scanner/appx_scanner.rs`
- **Item** (1 connections)

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (5 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/appx_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 55 (96%)
- INFERRED: 2 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*