# FailAtMutationBackend

> 34 nodes · cohesion 0.12

## Key Concepts

- **StateLedger** (36 connections) — `src/backup/state_ledger.rs`
- **state_ledger.rs** (13 connections) — `src/backup/state_ledger.rs`
- **LedgerEntry** (12 connections) — `src/backup/state_ledger.rs`
- **test_ledger_crud()** (10 connections) — `src/backup/state_ledger.rs`
- **SystemBaseline** (9 connections) — `src/backup/state_ledger.rs`
- **.get_history()** (6 connections) — `src/backup/state_ledger.rs`
- **.to_owned()** (5 connections) — `src/backup/state_ledger.rs`
- **.find_latest_unrolled_by_tweak()** (5 connections) — `src/backup/state_ledger.rs`
- **.parse_timestamp()** (4 connections) — `src/backup/state_ledger.rs`
- **DateTime** (4 connections)
- **Utc** (4 connections)
- **.for_each_history()** (4 connections) — `src/backup/state_ledger.rs`
- **.get_initial_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **.init_conn()** (4 connections) — `src/backup/state_ledger.rs`
- **.query_single_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **revert_tweak()** (4 connections) — `src/executor/engine.rs`
- **Connection** (3 connections)
- **is_reversible_action()** (3 connections) — `src/backup/state_ledger.rs`
- **LedgerEntryView<'a>** (3 connections) — `src/backup/state_ledger.rs`
- **.for_each_history_by_tweak()** (3 connections) — `src/backup/state_ledger.rs`
- **.get_latest_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.new()** (3 connections) — `src/backup/state_ledger.rs`
- **.reset_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.save_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.is_reversible()** (2 connections) — `src/backup/state_ledger.rs`
- *... and 9 more nodes in this community*

## Relationships

- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (18 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [.maybe_arm_error](maybe_arm_error.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 100 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*