# WinBlast0 Architectural Specifications

> 3 nodes · cohesion 0.67

## Key Concepts

- **.set_startup_entry_enabled_by_id()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_startup_entry_enabled_by_id()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.set_startup_entry_enabled_by_id()** (3 connections) — `tests/tweak_sandbox_e2e.rs`

## Relationships

- [Kernel Scheduling & Latency Optimization](Kernel_Scheduling_&_Latency_Optimization.md) (3 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Windows Unattend Deployment Generator](Windows_Unattend_Deployment_Generator.md) (1 shared connections)

## Source Files

- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 8 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*