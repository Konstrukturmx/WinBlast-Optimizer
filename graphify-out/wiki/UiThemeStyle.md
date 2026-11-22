# UiThemeStyle

> 14 nodes · cohesion 0.20

## Key Concepts

- **PlatformSecurityInfo** (16 connections) — `src/hardware/sys_inventory.rs`
- **detect_platform_security_info()** (8 connections) — `src/hardware/sys_inventory.rs`
- **RamDimmInfo** (7 connections) — `src/hardware/sys_inventory.rs`
- **query_ram_dimms()** (5 connections) — `src/hardware/sys_inventory.rs`
- **total_physical_ram_kb()** (4 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (3 connections) — `src/core/backend.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.total_physical_ram_kb()** (2 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (2 connections) — `src/core/backend.rs`
- **read_smbios_string()** (2 connections) — `src/hardware/sys_inventory.rs`
- **smbios_mem_type_name()** (2 connections) — `src/hardware/sys_inventory.rs`

## Relationships

- [topological_scheduler.rs](topological_scheduler.rs.md) (8 shared connections)
- [driver_update.rs](driver_update.rs.md) (2 shared connections)
- [.sign](sign.md) (2 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [OverallHealthGrade](OverallHealthGrade.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/sys_inventory.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 46 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*