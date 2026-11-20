# Hardware Diagnostics & GPU Silicon Tuning

> 35 nodes · cohesion 0.12

## Key Concepts

- **StateLedger** (36 connections) — `src/backup/state_ledger.rs`
- **state_ledger.rs** (13 connections) — `src/backup/state_ledger.rs`
- **LedgerEntry** (12 connections) — `src/backup/state_ledger.rs`
- **test_ledger_crud()** (10 connections) — `src/backup/state_ledger.rs`
- **SystemBaseline** (9 connections) — `src/backup/state_ledger.rs`
- **Utc** (7 connections)
- **DateTime** (6 connections)
- **.get_history()** (6 connections) — `src/backup/state_ledger.rs`
- **.to_owned()** (5 connections) — `src/backup/state_ledger.rs`
- **.find_latest_unrolled_by_tweak()** (5 connections) — `src/backup/state_ledger.rs`
- **.parse_timestamp()** (4 connections) — `src/backup/state_ledger.rs`
- **.for_each_history()** (4 connections) — `src/backup/state_ledger.rs`
- **.get_initial_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **.init_conn()** (4 connections) — `src/backup/state_ledger.rs`
- **.query_single_baseline()** (4 connections) — `src/backup/state_ledger.rs`
- **revert_tweak()** (4 connections) — `src/executor/engine.rs`
- **rollback_entry()** (4 connections) — `src/executor/engine.rs`
- **Connection** (3 connections)
- **is_reversible_action()** (3 connections) — `src/backup/state_ledger.rs`
- **LedgerEntryView<'a>** (3 connections) — `src/backup/state_ledger.rs`
- **.for_each_history_by_tweak()** (3 connections) — `src/backup/state_ledger.rs`
- **.get_latest_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.new()** (3 connections) — `src/backup/state_ledger.rs`
- **.reset_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- **.save_baseline()** (3 connections) — `src/backup/state_ledger.rs`
- *... and 10 more nodes in this community*

## Relationships

- [clean_apps.rs](clean_apps.rs.md) (18 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (2 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (2 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (2 shared connections)
- [parking.rs](parking.rs.md) (1 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 107 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*