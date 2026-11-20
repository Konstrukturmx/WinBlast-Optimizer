# test_wave1_compound_actions_deep_edge_cases

> 6 nodes · cohesion 0.47

## Key Concepts

- **RegStringValue** (11 connections) — `src/core/reg_helpers.rs`
- **.reg_get_string()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (2 connections) — `src/core/backend.rs`
- **.reg_get_string()** (2 connections) — `src/core/backend.rs`

## Relationships

- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [super](super.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (1 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [PanickingBackend](PanickingBackend.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 18 (95%)
- INFERRED: 1 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*