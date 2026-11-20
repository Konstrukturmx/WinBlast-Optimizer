# DriverUpdateState

> 12 nodes · cohesion 0.20

## Key Concepts

- **RegRawSnapshot** (22 connections) — `src/core/reg_helpers.rs`
- **get_raw()** (7 connections) — `src/scanner/registry_scanner.rs`
- **encode_hex()** (4 connections) — `src/core/reg_helpers.rs`
- **reg_type_to_u32()** (4 connections) — `src/scanner/registry_scanner.rs`
- **.reg_get_raw()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (2 connections) — `src/core/backend.rs`
- **RegType** (1 connections)

## Relationships

- [registry_scanner.rs](registry_scanner.rs.md) (5 shared connections)
- [PanickingBackend](PanickingBackend.md) (3 shared connections)
- [updater.rs](updater.rs.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [reg_helpers.rs](reg_helpers.rs.md) (2 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [enums](enums.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/scanner/registry_scanner.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 40 (93%)
- INFERRED: 3 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*