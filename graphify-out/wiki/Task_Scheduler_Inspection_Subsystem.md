# Task Scheduler Inspection Subsystem

> 25 nodes · cohesion 0.16

## Key Concepts

- **StateLedger** (30 connections) — `src/backup/state_ledger.rs`
- **state_ledger.rs** (11 connections) — `src/backup/state_ledger.rs`
- **LedgerEntry** (10 connections) — `src/backup/state_ledger.rs`
- **SystemBaseline** (9 connections) — `src/backup/state_ledger.rs`
- **test_ledger_crud()** (8 connections) — `src/backup/state_ledger.rs`
- **revert_tweaks_by_id_with()** (7 connections) — `src/executor/engine.rs`
- **.get_initial_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **.init_conn()** (4 connections) — `src/backup/state_ledger.rs`
- **.query_single_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **revert_tweak()** (4 connections) — `src/executor/engine.rs`
- **revert_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **Connection** (3 connections)
- **DateTime** (3 connections)
- **Utc** (3 connections)
- **.get_history()** (3 connections) — `src/backup/state_ledger.rs`
- **.get_latest_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.new()** (3 connections) — `src/backup/state_ledger.rs`
- **.reset_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.save_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.in_memory()** (2 connections) — `src/backup/state_ledger.rs`
- **.record_change()** (2 connections) — `src/backup/state_ledger.rs`
- **.rollback_entry()** (2 connections) — `src/backup/state_ledger.rs`
- **params** (1 connections)
- **.is_reversible()** (1 connections) — `src/backup/state_ledger.rs`
- **.clear_history()** (1 connections) — `src/backup/state_ledger.rs`

## Relationships

- [Task Scheduler Inspection Subsystem](Task_Scheduler_Inspection_Subsystem.md) (14 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (5 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (2 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 81 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*