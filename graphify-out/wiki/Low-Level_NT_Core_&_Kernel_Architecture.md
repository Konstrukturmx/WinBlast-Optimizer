# Low-Level NT Core & Kernel Architecture

> 13 nodes · cohesion 0.17

## Key Concepts

- **elevation.rs** (13 connections) — `src/core/elevation.rs`
- **is_elevated()** (5 connections) — `src/core/elevation.rs`
- **closehandle** (4 connections)
- **.is_elevated()** (3 connections) — `src/core/backend.rs`
- **openprocesstoken** (2 connections)
- **check_elevation()** (2 connections) — `src/core/elevation.rs`
- **relaunch_as_admin()** (2 connections) — `src/core/elevation.rs`
- **token_query** (2 connections)
- **getcurrentprocess** (1 connections)
- **gettokeninformation** (1 connections)
- **HANDLE** (1 connections)
- **token_elevation** (1 connections)
- **tokenelevation** (1 connections)

## Relationships

- [Appx Package() Prefixes](Appx_Package_Prefixes.md) (4 shared connections)
- [topological_scheduler.rs](topological_scheduler.rs.md) (2 shared connections)
- [updater.rs](updater.rs.md) (2 shared connections)
- [decode_oem_bytes](decode_oem_bytes.md) (1 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/elevation.rs`

## Audit Trail

- EXTRACTED: 25 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*