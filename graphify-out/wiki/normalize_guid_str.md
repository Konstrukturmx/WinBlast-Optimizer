# normalize_guid_str

> 10 nodes · cohesion 0.27

## Key Concepts

- **HibernationStatus** (22 connections) — `src/hardware/power.rs`
- **.query_hibernation_status()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **query_hibernation_and_standby()** (3 connections) — `src/hardware/power.rs`
- **set_hibernation_enabled()** (3 connections) — `src/hardware/power.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.query_hibernation_status()** (2 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (2 connections) — `src/core/backend.rs`
- **.query_hibernation_status()** (2 connections) — `src/core/backend.rs`
- **.set_hibernation_enabled()** (2 connections) — `src/core/backend.rs`

## Relationships

- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [elevation.rs](elevation.rs.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)
- [WorkerEvent](WorkerEvent.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)
- [super](super.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 34 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*