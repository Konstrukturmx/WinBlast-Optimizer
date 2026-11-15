# Integration Suite: Ui Responsive E2E

> 4 nodes · cohesion 0.50

## Key Concepts

- **RamDimmInfo** (7 connections) — `src/hardware/sys_inventory.rs`
- **query_ram_dimms()** (5 connections) — `src/hardware/sys_inventory.rs`
- **read_smbios_string()** (2 connections) — `src/hardware/sys_inventory.rs`
- **smbios_mem_type_name()** (2 connections) — `src/hardware/sys_inventory.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [Async() .Toggle .Refresh](Async_.Toggle_.Refresh.md) (1 shared connections)
- [Test Composite With()](Test_Composite_With.md) (1 shared connections)
- [Startup Tasks & Auto-Run Explorer](Startup_Tasks_&_Auto-Run_Explorer.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)

## Source Files

- `src/hardware/sys_inventory.rs`

## Audit Trail

- EXTRACTED: 13 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*