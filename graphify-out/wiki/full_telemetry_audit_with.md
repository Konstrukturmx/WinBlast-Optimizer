# full_telemetry_audit_with

> 38 nodes · cohesion 0.14

## Key Concepts

- **engine.rs** (53 connections) — `src/executor/engine.rs`
- **reg_helpers.rs** (24 connections) — `src/core/reg_helpers.rs`
- **rollback_entry_with()** (21 connections) — `src/executor/engine.rs`
- **revert_tweak_with()** (19 connections) — `src/executor/engine.rs`
- **apply_action_slice_with()** (17 connections) — `src/executor/engine.rs`
- **apply_tweak_with()** (16 connections) — `src/executor/engine.rs`
- **is_unsupported_power_guid_error()** (12 connections) — `src/core/backend.rs`
- **format_pipe_reg_target()** (9 connections) — `src/core/reg_helpers.rs`
- **decode_hex()** (8 connections) — `src/core/reg_helpers.rs`
- **escape_pipe()** (7 connections) — `src/core/reg_helpers.rs`
- **merge_composite_pair()** (7 connections) — `src/core/reg_helpers.rs`
- **parse_pipe_reg_target()** (6 connections) — `src/core/reg_helpers.rs`
- **apply_tweaks_by_id_with()** (6 connections) — `src/executor/engine.rs`
- **format_composite_action_key()** (6 connections) — `src/executor/engine.rs`
- **rollback_recorded_ids()** (6 connections) — `src/executor/engine.rs`
- **ensure_tweak_compatible()** (5 connections) — `src/executor/engine.rs`
- **format_dword_action_key()** (5 connections) — `src/executor/engine.rs`
- **read_composite_entry()** (4 connections) — `src/core/reg_helpers.rs`
- **unescape_pipe()** (4 connections) — `src/core/reg_helpers.rs`
- **apply_tweak()** (4 connections) — `src/executor/engine.rs`
- **apply_tweaks_by_id()** (4 connections) — `src/executor/engine.rs`
- **format_power_indices()** (4 connections) — `src/executor/engine.rs`
- **test_batch1_engine_edge_cases_and_atomic_revert()** (4 connections) — `src/executor/engine.rs`
- **parse_composite_pairs()** (3 connections) — `src/core/reg_helpers.rs`
- **test_composite_string_merge_preserves_siblings()** (3 connections) — `src/core/reg_helpers.rs`
- *... and 13 more nodes in this community*

## Relationships

- [FailAtMutationBackend](FailAtMutationBackend.md) (19 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (11 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (8 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (7 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (6 shared connections)
- [WinBlastError](WinBlastError.md) (5 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (5 shared connections)
- [parse_guid](parse_guid.md) (4 shared connections)
- [parking.rs](parking.rs.md) (3 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (3 shared connections)
- [path](path.md) (2 shared connections)
- [deserialize](deserialize.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/executor/engine.rs`

## Audit Trail

- EXTRACTED: 156 (83%)
- INFERRED: 33 (17%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*