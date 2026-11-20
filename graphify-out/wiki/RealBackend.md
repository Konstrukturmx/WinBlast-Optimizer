# RealBackend

> 33 nodes · cohesion 0.15

## Key Concepts

- **engine.rs** (53 connections) — `src/executor/engine.rs`
- **rollback_entry_with()** (21 connections) — `src/executor/engine.rs`
- **revert_tweak_with()** (19 connections) — `src/executor/engine.rs`
- **apply_action_slice_with()** (17 connections) — `src/executor/engine.rs`
- **apply_tweak_with()** (16 connections) — `src/executor/engine.rs`
- **is_unsupported_power_guid_error()** (12 connections) — `src/core/backend.rs`
- **LedgerEntry** (10 connections) — `src/backup/state_ledger.rs`
- **format_pipe_reg_target()** (9 connections) — `src/core/reg_helpers.rs`
- **decode_hex()** (8 connections) — `src/core/reg_helpers.rs`
- **escape_pipe()** (7 connections) — `src/core/reg_helpers.rs`
- **merge_composite_pair()** (7 connections) — `src/core/reg_helpers.rs`
- **apply_tweaks_by_id_with()** (6 connections) — `src/executor/engine.rs`
- **format_composite_action_key()** (6 connections) — `src/executor/engine.rs`
- **rollback_recorded_ids()** (6 connections) — `src/executor/engine.rs`
- **ensure_tweak_compatible()** (5 connections) — `src/executor/engine.rs`
- **format_dword_action_key()** (5 connections) — `src/executor/engine.rs`
- **apply_tweak()** (4 connections) — `src/executor/engine.rs`
- **apply_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **format_power_indices()** (4 connections) — `src/executor/engine.rs`
- **rollback_entry()** (4 connections) — `src/executor/engine.rs`
- **test_batch1_engine_edge_cases_and_atomic_revert()** (4 connections) — `src/executor/engine.rs`
- **parse_feature_kind()** (3 connections) — `src/executor/engine.rs`
- **rollback_by_id()** (3 connections) — `src/executor/engine.rs`
- **test_absent_key_rolls_back_by_deleting()** (3 connections) — `src/executor/engine.rs`
- **CompositeKeyMeta** (2 connections) — `src/executor/engine.rs`
- *... and 8 more nodes in this community*

## Relationships

- [Dynamicdwordformula Logicalcorecount Totalraminkb](Dynamicdwordformula_Logicalcorecount_Totalraminkb.md) (17 shared connections)
- [parse_guid](parse_guid.md) (14 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (12 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (8 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (7 shared connections)
- [DriverError](DriverError.md) (6 shared connections)
- [super](super.md) (5 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (3 shared connections)
- [Any](Any.md) (3 shared connections)
- [updater.rs](updater.rs.md) (2 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (2 shared connections)
- [AppState](AppState.md) (1 shared connections)

## Source Files

- `src/backup/state_ledger.rs`
- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 141 (82%)
- INFERRED: 30 (18%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*