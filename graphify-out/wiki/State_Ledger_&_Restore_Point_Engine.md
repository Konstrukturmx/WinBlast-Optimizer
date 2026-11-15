# State Ledger & Restore Point Engine

> 19 nodes · cohesion 0.12

## Key Concepts

- **real_windows_env.rs** (32 connections) — `tests/real_windows_env.rs`
- **is_known_numeric_reg_sz()** (7 connections) — `src/core/reg_helpers.rs`
- **TempTestEnvGuard** (3 connections) — `tests/real_windows_env.rs`
- **test_real_backend_readonly_live_host_audit()** (3 connections) — `tests/real_windows_env.rs`
- **fs** (2 connections)
- **regvalue** (2 connections)
- **test_known_numeric_reg_sz()** (2 connections) — `src/core/reg_helpers.rs`
- **HkcuTestEnvGuard** (2 connections) — `tests/real_windows_env.rs`
- **.new()** (2 connections) — `tests/real_windows_env.rs`
- **test_ntfs_junction_and_readonly_cleanup_sandbox()** (2 connections) — `tests/real_windows_env.rs`
- **test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip()** (2 connections) — `tests/real_windows_env.rs`
- **cleantargetkind** (1 connections)
- **coreparkingstate** (1 connections)
- **cpuinfo** (1 connections)
- **systembackend** (1 connections)
- **FILE_ATTRIBUTE_REPARSE_POINT** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **.drop()** (1 connections) — `tests/real_windows_env.rs`
- **windowsinfo** (1 connections)

## Relationships

- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [Integration Suite: Ui Responsive E2E](Integration_Suite-_Ui_Responsive_E2E.md) (3 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (2 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (2 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (1 shared connections)

## Source Files

- `src/core/reg_helpers.rs`
- `tests/real_windows_env.rs`

## Audit Trail

- EXTRACTED: 42 (89%)
- INFERRED: 5 (11%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*