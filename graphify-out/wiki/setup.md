# .setup

> 12 nodes · cohesion 0.21

## Key Concepts

- **pathbuf** (10 connections)
- **diagnostics.rs** (10 connections) — `src/tools/diagnostics.rs`
- **FaultDiagnosticsReport** (10 connections) — `src/tools/diagnostics.rs`
- **parse_multi_sz_lossless()** (3 connections) — `src/tools/diagnostics.rs`
- **.scan()** (2 connections) — `src/tools/diagnostics.rs`
- **MinidumpEntry** (2 connections) — `src/tools/diagnostics.rs`
- **PendingRebootStatus** (2 connections) — `src/tools/diagnostics.rs`
- **test_unorthodox_multi_sz_odd_byte_lengths_and_corruptions()** (2 connections) — `tests/edge_cases_unorthodox.rs`
- **.clear_cbs_reboot_flags()** (1 connections) — `src/tools/diagnostics.rs`
- **.clear_minidumps()** (1 connections) — `src/tools/diagnostics.rs`
- **.clear_pending_renames()** (1 connections) — `src/tools/diagnostics.rs`
- **.restart_system()** (1 connections) — `src/tools/diagnostics.rs`

## Relationships

- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)

## Source Files

- `src/tools/diagnostics.rs`
- `tests/edge_cases_unorthodox.rs`

## Audit Trail

- EXTRACTED: 30 (97%)
- INFERRED: 1 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*