# SandboxEnv

> God node · 119 connections · `src/core/backend.rs`

**Community:** [GPU Silicon & Hardware Tuning Policies](GPU_Silicon_&_Hardware_Tuning_Policies.md)

## Connections by Relation

### contains
- [backend.rs](backend.rs.md) `EXTRACTED`

### implements
- [SystemBackend](SystemBackend.md) `EXTRACTED`

### imports_from
- tweak_sandbox_e2e.rs `EXTRACTED`
- engine.rs `EXTRACTED`
- scanner/mod.rs `EXTRACTED`
- [adaptive_tuning.rs](adaptive_tuning.rs.md) `EXTRACTED`
- restore_point.rs `EXTRACTED`
- [power_governor.rs](power_governor.rs.md) `EXTRACTED`
- [boot_config.rs](boot_config.rs.md) `EXTRACTED`
- [sysprep_engine.rs](sysprep_engine.rs.md) `EXTRACTED`

### method
- .state_read() `EXTRACTED`
- .take_injected_error() `EXTRACTED`
- .state_write() `EXTRACTED`
- .reg_key() `EXTRACTED`
- .seed_actions_with_vendor() `EXTRACTED`
- .normalize_hive() `EXTRACTED`
- .ensure_tree_path_in_state() `EXTRACTED`
- .resolve_target_paths_in_state() `EXTRACTED`
- .reg_delete_value_for_vendor() `EXTRACTED`
- .reg_set_string() `EXTRACTED`
- .normalize_reg_path() `EXTRACTED`
- .clear_reg_value_in_state() `EXTRACTED`
- .write_dword_in_state() `EXTRACTED`
- .write_raw_in_state() `EXTRACTED`
- .restore_tree_into_state() `EXTRACTED`
- .reg_set_dword_for_vendor() `EXTRACTED`
- .reg_set_binary() `EXTRACTED`
- .sync_parking_in_state() `EXTRACTED`
- .reg_set_raw() `EXTRACTED`
- .reg_delete_tree() `EXTRACTED`
- *…and 79 more `method` connection(s) not listed (lowest-degree first to go)*

### references
- [SlowScanBackend](SlowScanBackend.md) `EXTRACTED`
- [PanickingBackend](PanickingBackend.md) `EXTRACTED`
- [FailAtMutationBackend](FailAtMutationBackend.md) `EXTRACTED`
- SandboxState `EXTRACTED`
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) `EXTRACTED`
- SandboxTempDir `EXTRACTED`
- RwLock `EXTRACTED`
- .new() `EXTRACTED`
- .new() `EXTRACTED`
- .new() `EXTRACTED`

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*