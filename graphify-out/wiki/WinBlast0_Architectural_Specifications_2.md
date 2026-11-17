# WinBlast0 Architectural Specifications

> 11 nodes · cohesion 0.31

## Key Concepts

- **service_scanner.rs** (12 connections) — `src/scanner/service_scanner.rs`
- **evaluate_service_value_with_kind()** (5 connections) — `src/scanner/service_scanner.rs`
- **evaluate_service()** (4 connections) — `src/scanner/service_scanner.rs`
- **evaluate_service_value()** (4 connections) — `src/scanner/service_scanner.rs`
- **get_service_start_type()** (4 connections) — `src/scanner/service_scanner.rs`
- **.get_service_start_type()** (2 connections) — `src/core/backend.rs`
- **check_service_start_type()** (2 connections) — `src/scanner/service_scanner.rs`
- **start_type_name()** (2 connections) — `src/scanner/service_scanner.rs`
- **test_evaluate_absent_optional_service()** (2 connections) — `src/scanner/service_scanner.rs`
- **test_evaluate_service_value_with_kind_all_branches()** (2 connections) — `src/scanner/service_scanner.rs`
- **hkey_local_machine** (1 connections)

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [enums](enums.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [Windows OS Settings & UX Customization](Windows_OS_Settings_&_UX_Customization.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/scanner/service_scanner.rs`

## Audit Trail

- EXTRACTED: 18 (78%)
- INFERRED: 5 (22%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*