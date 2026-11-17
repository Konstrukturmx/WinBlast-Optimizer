# Integration Suite: Tweak Sandbox E2E

> 28 nodes · cohesion 0.07

## Key Concepts

- **SandboxOp** (25 connections) — `src/core/backend.rs`
- **.create_restore_point()** (4 connections) — `src/core/backend.rs`
- **.ensure_preflight_restore_point()** (4 connections) — `src/core/backend.rs`
- **.list_restore_points()** (3 connections) — `src/core/backend.rs`
- **.set_hags()** (3 connections) — `src/core/backend.rs`
- **.set_mpo_disabled()** (3 connections) — `src/core/backend.rs`
- **.recorded_ops()** (2 connections) — `src/core/backend.rs`
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
- *... and 3 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (8 shared connections)
- [RestorePoint](RestorePoint.md) (1 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [restore_point.rs](restore_point.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`

## Audit Trail

- EXTRACTED: 41 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*