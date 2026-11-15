# Core UI Internationalization & Text Catalog

> 33 nodes · cohesion 0.14

## Key Concepts

- **registry_scanner.rs** (37 connections) — `src/scanner/registry_scanner.rs`
- **normalize_reg_value_name()** (21 connections) — `src/core/risk.rs`
- **resolve_hive()** (16 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_read_64()** (13 connections) — `src/scanner/registry_scanner.rs`
- **get_raw()** (8 connections) — `src/scanner/registry_scanner.rs`
- **get_string_for_vendor()** (8 connections) — `src/scanner/registry_scanner.rs`
- **open_subkey_flags_64()** (8 connections) — `src/scanner/registry_scanner.rs`
- **RegKey** (8 connections)
- **delete_value_for_vendor()** (7 connections) — `src/executor/reg_mutator.rs`
- **get_dword_for_vendor()** (7 connections) — `src/scanner/registry_scanner.rs`
- **resolve_target_paths()** (7 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_key_recursive()** (7 connections) — `src/scanner/registry_scanner.rs`
- **get_binary()** (6 connections) — `src/scanner/registry_scanner.rs`
- **snapshot_tree()** (6 connections) — `src/scanner/registry_scanner.rs`
- **get_dword()** (5 connections) — `src/scanner/registry_scanner.rs`
- **resolve_net_interface_paths()** (5 connections) — `src/scanner/registry_scanner.rs`
- **encode_hex()** (4 connections) — `src/core/reg_helpers.rs`
- **evaluate_dword()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value()** (4 connections) — `src/scanner/registry_scanner.rs`
- **evaluate_dword_value_with_kind()** (4 connections) — `src/scanner/registry_scanner.rs`
- **key_exists()** (4 connections) — `src/scanner/registry_scanner.rs`
- **read_dword_from_key()** (4 connections) — `src/scanner/registry_scanner.rs`
- **reg_type_to_u32()** (4 connections) — `src/scanner/registry_scanner.rs`
- **has_active_ip()** (3 connections) — `src/scanner/registry_scanner.rs`
- **replace_placeholder_ci()** (3 connections) — `src/scanner/registry_scanner.rs`
- *... and 8 more nodes in this community*

## Relationships

- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (21 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (11 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (6 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (4 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (4 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (3 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (2 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/core/risk.rs`
- `src/executor/reg_mutator.rs`
- `src/scanner/registry_scanner.rs`

## Audit Trail

- EXTRACTED: 113 (81%)
- INFERRED: 27 (19%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*