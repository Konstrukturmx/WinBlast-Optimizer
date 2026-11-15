# WinBlast0 Architectural Specifications

> 3 nodes · cohesion 0.67

## Key Concepts

- **.detect_os()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_os()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_os()** (4 connections) — `tests/tweak_sandbox_e2e.rs`

## Relationships

- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (3 shared connections)
- [Kernel Scheduling & Latency Optimization](Kernel_Scheduling_&_Latency_Optimization.md) (3 shared connections)
- [.Set Enabled() .Remove](Set_Enabled_.Remove.md) (2 shared connections)
- [Windows Unattend Deployment Generator](Windows_Unattend_Deployment_Generator.md) (1 shared connections)

## Source Files

- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 11 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*