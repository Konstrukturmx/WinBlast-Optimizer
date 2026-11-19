# svc_mutator.rs

> 24 nodes · cohesion 0.08

## Key Concepts

- **SandboxOp** (25 connections) — `src/core/backend.rs`
- **.create_restore_point()** (5 connections) — `src/core/backend.rs`
- **.ensure_preflight_restore_point()** (3 connections) — `src/core/backend.rs`
- **CreateRestorePoint** (2 connections) — `src/core/backend.rs`
- **SetHags** (2 connections) — `src/core/backend.rs`
- **SetMpo** (2 connections) — `src/core/backend.rs`
- **AppxRemove** (1 connections) — `src/core/backend.rs`
- **BlockHosts** (1 connections) — `src/core/backend.rs`
- **CleanExecuted** (1 connections) — `src/core/backend.rs`
- **ExplorerRestarted** (1 connections) — `src/core/backend.rs`
- **FeatureSet** (1 connections) — `src/core/backend.rs`
- **GpuMsi** (1 connections) — `src/core/backend.rs`
- **PowerCfgSet** (1 connections) — `src/core/backend.rs`
- **RebootRequested** (1 connections) — `src/core/backend.rs`
- **RegDelete** (1 connections) — `src/core/backend.rs`
- **RegDeleteTree** (1 connections) — `src/core/backend.rs`
- **RegSet** (1 connections) — `src/core/backend.rs`
- **RegSetBinary** (1 connections) — `src/core/backend.rs`
- **RegSetString** (1 connections) — `src/core/backend.rs`
- **RestoreParking** (1 connections) — `src/core/backend.rs`
- **ServiceSet** (1 connections) — `src/core/backend.rs`
- **StartupToggled** (1 connections) — `src/core/backend.rs`
- **TaskSet** (1 connections) — `src/core/backend.rs`
- **UnparkCores** (1 connections) — `src/core/backend.rs`

## Relationships

- [.take_injected_error](take_injected_error.md) (4 shared connections)
- [SandboxEnv](SandboxEnv.md) (3 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [task_scanner.rs](task_scanner.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 34 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*