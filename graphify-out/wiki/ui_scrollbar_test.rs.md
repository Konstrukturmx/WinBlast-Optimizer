# ui_scrollbar_test.rs

> 23 nodes · cohesion 0.15

## Key Concepts

- **autounattend.rs** (24 connections) — `src/tools/autounattend.rs`
- **generate_autounattend_xml()** (17 connections) — `src/tools/autounattend.rs`
- **Reversibility** (10 connections) — `src/core/risk.rs`
- **UnattendOptions** (7 connections) — `src/tools/autounattend.rs`
- **TargetArch** (6 connections) — `src/tools/autounattend.rs`
- **format_reg_add_cmd()** (5 connections) — `src/tools/autounattend.rs`
- **format_reg_delete_cmd()** (5 connections) — `src/tools/autounattend.rs`
- **.export_autounattend_xml()** (3 connections) — `src/app.rs`
- **escape_win32_quoted_arg()** (3 connections) — `src/tools/autounattend.rs`
- **map_hive()** (3 connections) — `src/tools/autounattend.rs`
- **test_autounattend_generation_wellformed_and_escaped()** (3 connections) — `src/tools/autounattend.rs`
- **.default()** (3 connections) — `src/tools/autounattend.rs`
- **escape_xml()** (2 connections) — `src/tools/autounattend.rs`
- **sanitize_computer_name()** (2 connections) — `src/tools/autounattend.rs`
- **sanitize_local_user()** (2 connections) — `src/tools/autounattend.rs`
- **.as_str()** (2 connections) — `src/tools/autounattend.rs`
- **.detect_host()** (2 connections) — `src/tools/autounattend.rs`
- **Instant** (1 connections) — `src/core/risk.rs`
- **RequiresLogoff** (1 connections) — `src/core/risk.rs`
- **RequiresPackageReinstall** (1 connections) — `src/core/risk.rs`
- **RequiresReboot** (1 connections) — `src/core/risk.rs`
- **Amd64** (1 connections) — `src/tools/autounattend.rs`
- **Arm64** (1 connections) — `src/tools/autounattend.rs`

## Relationships

- [theme.rs](theme.rs.md) (6 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (4 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (4 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (4 shared connections)
- [AppState](AppState.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [path](path.md) (1 shared connections)
- [RiskLevel](RiskLevel.md) (1 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/risk.rs`
- `src/tools/autounattend.rs`

## Audit Trail

- EXTRACTED: 58 (85%)
- INFERRED: 10 (15%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*