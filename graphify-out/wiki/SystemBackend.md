# SystemBackend

> God node · 117 connections · `src/core/backend.rs`

**Community:** [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md)

## Connections by Relation

### contains
- [backend.rs](backend.rs.md) `EXTRACTED`

### implements
- [SandboxEnv](SandboxEnv.md) `EXTRACTED`
- [SlowScanBackend](SlowScanBackend.md) `EXTRACTED`
- [RealBackend](RealBackend.md) `EXTRACTED`
- [PanickingBackend](PanickingBackend.md) `EXTRACTED`
- [FailAtMutationBackend](FailAtMutationBackend.md) `EXTRACTED`

### imports_from
- tweak_sandbox_e2e.rs `EXTRACTED`
- [app.rs](app.rs.md) `EXTRACTED`
- [engine.rs](engine.rs.md) `EXTRACTED`
- scanner/mod.rs `EXTRACTED`
- [restore_point.rs](restore_point.rs.md) `EXTRACTED`
- telemetry_scanner.rs `EXTRACTED`

### method
- .ensure_preflight_restore_point() `EXTRACTED`
- .apply_parking_snapshot() `EXTRACTED`
- .detect_core_parking_sampled() `EXTRACTED`
- .execute_clean_targets() `EXTRACTED`
- .set_optional_feature() `EXTRACTED`
- .query_hibernation_status() `EXTRACTED`
- .set_hibernation_enabled() `EXTRACTED`
- .set_usb_power_saving_enabled() `EXTRACTED`
- .load_task_cache() `EXTRACTED`
- .load_appx_cache() `EXTRACTED`
- .list_restore_points() `EXTRACTED`
- .detect_cpu() `EXTRACTED`
- .detect_gpus() `EXTRACTED`
- .detect_core_parking() `EXTRACTED`
- .capture_latency() `EXTRACTED`
- .run_latency_benchmark() `EXTRACTED`
- .create_metrics_sampler() `EXTRACTED`
- .scan_clean_targets() `EXTRACTED`
- .list_installed_appx_detailed() `EXTRACTED`
- .list_optional_features() `EXTRACTED`
- *…and 60 more `method` connection(s) not listed (lowest-degree first to go)*

### references
- [AppState](AppState.md) `EXTRACTED`
- rollback_entry_with() `EXTRACTED`
- revert_tweak_with() `EXTRACTED`
- Changelog `EXTRACTED`
- apply_tweak_with() `EXTRACTED`
- apply_action_slice_with() `EXTRACTED`
- scan_single_tweak_with() `EXTRACTED`
- WinBlast Optimizer — Developer & Architecture Guide `EXTRACTED`
- CachedScanBackend `EXTRACTED`
- run_full_scan_with() `EXTRACTED`
- scan_all_tweaks_parallel_with() `EXTRACTED`
- full_telemetry_audit_with() `EXTRACTED`
- apply_tweaks_by_id_with() `EXTRACTED`
- revert_tweaks_by_id_with() `EXTRACTED`
- resolve_dynamic_dword() `EXTRACTED`
- scan_tweaks_chunked_parallel_with() `EXTRACTED`
- rollback_recorded_ids() `EXTRACTED`
- evaluate_tweak_status() `EXTRACTED`
- ensure_tweak_compatible() `EXTRACTED`
- .new_with_backend() `EXTRACTED`
- *…and 5 more `references` connection(s) not listed (lowest-degree first to go)*

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*