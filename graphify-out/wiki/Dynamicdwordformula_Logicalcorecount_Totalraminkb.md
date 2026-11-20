# Dynamicdwordformula Logicalcorecount Totalraminkb

> 21 nodes · cohesion 0.19

## Key Concepts

- **StateLedger** (29 connections) — `src/backup/state_ledger.rs`
- **state_ledger.rs** (11 connections) — `src/backup/state_ledger.rs`
- **SystemBaseline** (9 connections) — `src/backup/state_ledger.rs`
- **test_ledger_crud()** (8 connections) — `src/backup/state_ledger.rs`
- **.get_initial_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **.init_conn()** (4 connections) — `src/backup/state_ledger.rs`
- **.query_single_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **revert_tweak()** (4 connections) — `src/executor/engine.rs`
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
- **.clear_history()** (1 connections) — `src/backup/state_ledger.rs`

## Relationships

- [RealBackend](RealBackend.md) (17 shared connections)
- [parse_guid](parse_guid.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 66 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*