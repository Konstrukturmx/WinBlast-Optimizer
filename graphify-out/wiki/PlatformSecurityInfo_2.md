# PlatformSecurityInfo

> 16 nodes · cohesion 0.16

## Key Concepts

- **PlatformSecurityInfo** (16 connections) — `src/hardware/sys_inventory.rs`
- **detect_platform_security_info()** (7 connections) — `src/hardware/sys_inventory.rs`
- **RamDimmInfo** (7 connections) — `src/hardware/sys_inventory.rs`
- **query_ram_dimms()** (5 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/sys_inventory.rs`
- **query_live_code_integrity_options()** (3 connections) — `src/hardware/sys_inventory.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- **total_physical_ram_kb()** (3 connections) — `src/hardware/sys_inventory.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.detect_platform_security()** (2 connections) — `src/core/backend.rs`
- **.detect_platform_security()** (2 connections) — `src/core/backend.rs`
- **read_smbios_string()** (2 connections) — `src/hardware/sys_inventory.rs`
- **smbios_mem_type_name()** (2 connections) — `src/hardware/sys_inventory.rs`
- **c_void** (1 connections)

## Relationships

- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (10 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (3 shared connections)
- [RestorePoint](RestorePoint.md) (2 shared connections)
- [WorkerEvent](WorkerEvent.md) (2 shared connections)
- [AppState](AppState.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [super](super.md) (1 shared connections)
- [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/sys_inventory.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 47 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*