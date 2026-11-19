# latency_bench.rs

> 8 nodes · cohesion 0.32

## Key Concepts

- **RegRawSnapshot** (21 connections) — `src/core/reg_helpers.rs`
- **.reg_get_raw()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_get_raw()** (2 connections) — `src/core/backend.rs`
- **.reg_set_raw()** (2 connections) — `src/core/backend.rs`

## Relationships

- [.Reg State() Vendor()](Reg_State_Vendor.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (2 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (1 shared connections)
- [RegTreeSnapshot](RegTreeSnapshot.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [.take_injected_error](take_injected_error.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 28 (93%)
- INFERRED: 2 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*