# SystemBackend

> God node · 124 connections · `src/core/backend.rs`

**Community:** [FailAtMutationBackend](FailAtMutationBackend.md)

## Connections by Relation

### contains
- [backend.rs](backend.rs.md) `EXTRACTED`

### implements
- [SandboxEnv](SandboxEnv.md) `EXTRACTED`
- [SlowScanBackend](SlowScanBackend.md) `EXTRACTED`
- [RealBackend](RealBackend.md) `EXTRACTED`
- PanickingBackend `EXTRACTED`
- [FailAtMutationBackend](FailAtMutationBackend.md) `EXTRACTED`

### imports_from
- tweak_sandbox_e2e.rs `EXTRACTED`
- [app.rs](app.rs.md) `EXTRACTED`
- preset_sharing.rs `EXTRACTED`
- engine.rs `EXTRACTED`
- scanner/mod.rs `EXTRACTED`
- real_windows_env.rs `EXTRACTED`
- fuzz_mutation_properties.rs `EXTRACTED`
- [adaptive_tuning.rs](adaptive_tuning.rs.md) `EXTRACTED`
- [restore_point.rs](restore_point.rs.md) `EXTRACTED`
- [power_governor.rs](power_governor.rs.md) `EXTRACTED`
- [boot_config.rs](boot_config.rs.md) `EXTRACTED`
- [sysprep_engine.rs](sysprep_engine.rs.md) `EXTRACTED`
- telemetry_scanner.rs `EXTRACTED`

### method
- .ensure_preflight_restore_point() `EXTRACTED`
- .apply_parking_snapshot() `EXTRACTED`
- .detect_core_parking_sampled() `EXTRACTED`
- .execute_clean_targets() `EXTRACTED`
- .set_optional_feature() `EXTRACTED`
- .reg_get_string() `EXTRACTED`
- .reg_get_raw() `EXTRACTED`
- .reg_set_raw() `EXTRACTED`
- .reg_snapshot_tree() `EXTRACTED`
- .reg_restore_tree() `EXTRACTED`
- .detect_platform_security() `EXTRACTED`
- .list_power_schemes() `EXTRACTED`
- .query_hibernation_status() `EXTRACTED`
- .set_hibernation_enabled() `EXTRACTED`
- .set_usb_power_saving_enabled() `EXTRACTED`
- .load_task_cache() `EXTRACTED`
- .load_appx_cache() `EXTRACTED`
- .list_restore_points() `EXTRACTED`
- .detect_cpu() `EXTRACTED`
- .detect_gpus() `EXTRACTED`
- *…and 60 more `method` connection(s) not listed (lowest-degree first to go)*

### references
- [AppState](AppState.md) `EXTRACTED`
- rollback_entry_with() `EXTRACTED`
- apply_tweak_with() `EXTRACTED`
- [Changelog](Changelog.md) `EXTRACTED`
- revert_tweak_with() `EXTRACTED`
- scan_single_tweak_with() `EXTRACTED`
- apply_action_slice_with() `EXTRACTED`
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) `EXTRACTED`
- scan_all_tweaks_parallel_with() `EXTRACTED`
- CachedScanBackend `EXTRACTED`
- full_telemetry_audit_with() `EXTRACTED`
- run_full_scan_with() `EXTRACTED`
- resolve_dynamic_dword() `EXTRACTED`
- scan_tweaks_chunked_parallel_with() `EXTRACTED`
- apply_tweaks_by_id_with() `EXTRACTED`
- revert_tweaks_by_id_with() `EXTRACTED`
- rollback_recorded_ids() `EXTRACTED`
- evaluate_tweak_status() `EXTRACTED`
- ensure_tweak_compatible() `EXTRACTED`
- .new_with_backend() `EXTRACTED`
- *…and 5 more `references` connection(s) not listed (lowest-degree first to go)*

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*