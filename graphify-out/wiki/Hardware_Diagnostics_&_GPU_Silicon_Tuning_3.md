# Hardware Diagnostics & GPU Silicon Tuning

> 22 nodes · cohesion 0.16

## Key Concepts

- **autounattend.rs** (24 connections) — `src/tools/autounattend.rs`
- **generate_autounattend_xml()** (16 connections) — `src/tools/autounattend.rs`
- **Reversibility** (10 connections) — `src/core/risk.rs`
- **TargetArch** (6 connections) — `src/tools/autounattend.rs`
- **UnattendOptions** (6 connections) — `src/tools/autounattend.rs`
- **format_reg_add_cmd()** (5 connections) — `src/tools/autounattend.rs`
- **format_reg_delete_cmd()** (5 connections) — `src/tools/autounattend.rs`
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

- [super](super.md) (4 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (4 shared connections)
- [updater.rs](updater.rs.md) (4 shared connections)
- [parse_guid](parse_guid.md) (3 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [LazyLoader](LazyLoader.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)
- [DriverError](DriverError.md) (1 shared connections)

## Source Files

- `src/core/risk.rs`
- `src/tools/autounattend.rs`

## Audit Trail

- EXTRACTED: 54 (84%)
- INFERRED: 10 (16%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*