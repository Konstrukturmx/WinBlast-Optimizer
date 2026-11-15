# Project Documentation, Architecture & Licensing

> 9 nodes · cohesion 0.22

## Key Concepts

- **restart_explorer_safely()** (6 connections) — `src/tools/explorer_ctrl.rs`
- **HandleCloser** (4 connections) — `src/tools/explorer_ctrl.rs`
- **broadcast_shell_settings_changed()** (3 connections) — `src/tools/explorer_ctrl.rs`
- **EnvBlockGuard** (3 connections) — `src/tools/explorer_ctrl.rs`
- **.broadcast_shell_settings_changed()** (2 connections) — `src/core/backend.rs`
- **.restart_explorer_safely()** (2 connections) — `src/core/backend.rs`
- **HANDLE** (2 connections)
- **.drop()** (1 connections) — `src/tools/explorer_ctrl.rs`
- **.drop()** (1 connections) — `src/tools/explorer_ctrl.rs`

## Relationships

- [Bloatware & AppX Deprovisioning Rules](Bloatware_&_AppX_Deprovisioning_Rules.md) (5 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (2 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/explorer_ctrl.rs`

## Audit Trail

- EXTRACTED: 16 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*