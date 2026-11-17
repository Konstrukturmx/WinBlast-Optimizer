# Hardware Diagnostics & GPU Silicon Tuning

> 41 nodes · cohesion 0.08

## Key Concepts

- **sys_inventory.rs** (50 connections) — `src/hardware/sys_inventory.rs`
- **disks.rs** (29 connections) — `src/hardware/disks.rs`
- **detect_platform_security_info()** (7 connections) — `src/hardware/sys_inventory.rs`
- **RamDimmInfo** (7 connections) — `src/hardware/sys_inventory.rs`
- **pcwstr** (5 connections)
- **query_ram_dimms()** (5 connections) — `src/hardware/sys_inventory.rs`
- **closehandle** (4 connections)
- **NtQuerySystemInformation()** (3 connections) — `src/hardware/sys_inventory.rs`
- **query_live_code_integrity_options()** (3 connections) — `src/hardware/sys_inventory.rs`
- **test_ram_and_platform_inventory_nonblocking()** (3 connections) — `src/hardware/sys_inventory.rs`
- **total_physical_ram_kb()** (3 connections) — `src/hardware/sys_inventory.rs`
- **createfilew** (2 connections)
- **device_seek_penalty_descriptor** (2 connections)
- **deviceiocontrol** (2 connections)
- **file_share_read** (2 connections)
- **file_share_write** (2 connections)
- **getdiskfreespaceexw** (2 connections)
- **getlogicaldrives** (2 connections)
- **getvolumeinformationw** (2 connections)
- **ioctl_storage_query_property** (2 connections)
- **open_existing** (2 connections)
- **propertystandardquery** (2 connections)
- **read_smbios_string()** (2 connections) — `src/hardware/sys_inventory.rs`
- **smbios_mem_type_name()** (2 connections) — `src/hardware/sys_inventory.rs`
- **storage_descriptor_header** (2 connections)
- *... and 16 more nodes in this community*

## Relationships

- [DiskSubsystemReport](DiskSubsystemReport.md) (7 shared connections)
- [Error.Rs Winblasterror Appx](Error.Rs_Winblasterror_Appx.md) (7 shared connections)
- [deserialize](deserialize.md) (5 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (3 shared connections)
- [Bloatware & AppX Deprovisioning Rules](Bloatware_&_AppX_Deprovisioning_Rules.md) (2 shared connections)
- [backend.rs](backend.rs.md) (2 shared connections)
- [enums](enums.md) (2 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (1 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)

## Source Files

- `src/hardware/disks.rs`
- `src/hardware/sys_inventory.rs`

## Audit Trail

- EXTRACTED: 101 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*