# Task Scheduler Inspection Subsystem

> 23 nodes · cohesion 0.18

## Key Concepts

- **StateLedger** (30 connections) — `src/backup/state_ledger.rs`
- **state_ledger.rs** (11 connections) — `src/backup/state_ledger.rs`
- **LedgerEntry** (10 connections) — `src/backup/state_ledger.rs`
- **SystemBaseline** (9 connections) — `src/backup/state_ledger.rs`
- **test_ledger_crud()** (8 connections) — `src/backup/state_ledger.rs`
- **.get_initial_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **.init_conn()** (4 connections) — `src/backup/state_ledger.rs`
- **.query_single_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **rollback_entry()** (4 connections) — `src/executor/engine.rs`
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

- [engine.rs](engine.rs.md) (17 shared connections)
- [app.rs](app.rs.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 72 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*