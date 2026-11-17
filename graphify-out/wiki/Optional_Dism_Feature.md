# Optional Dism Feature()

> 21 nodes · cohesion 0.13

## Key Concepts

- **ps_runner.rs** (16 connections) — `src/executor/ps_runner.rs`
- **block_telemetry_hosts_in_content()** (6 connections) — `src/executor/ps_runner.rs`
- **block_telemetry_hosts()** (4 connections) — `src/executor/ps_runner.rs`
- **unblock_telemetry_hosts_in_content()** (4 connections) — `src/executor/ps_runner.rs`
- **unblock_telemetry_hosts_with()** (4 connections) — `src/executor/ps_runner.rs`
- **.block_telemetry_hosts()** (3 connections) — `src/core/backend.rs`
- **find_exact_marker_line()** (3 connections) — `src/executor/ps_runner.rs`
- **set_scheduled_task_enabled()** (3 connections) — `src/executor/ps_runner.rs`
- **test_hosts_bom_ipv6_tabs_and_exact_footer_matching()** (3 connections) — `src/executor/ps_runner.rs`
- **write_hosts_file_raw()** (3 connections) — `src/executor/ps_runner.rs`
- **.block_telemetry_hosts()** (2 connections) — `src/core/backend.rs`
- **.remove_appx_package()** (2 connections) — `src/core/backend.rs`
- **.set_scheduled_task_enabled()** (2 connections) — `src/core/backend.rs`
- **.write_hosts_file()** (2 connections) — `src/core/backend.rs`
- **remove_appx_package()** (2 connections) — `src/executor/ps_runner.rs`
- **test_hosts_blocking_ignores_comments_and_subdomains()** (2 connections) — `src/executor/ps_runner.rs`
- **openoptions** (1 connections)
- **add_firewall_block_rule()** (1 connections) — `src/executor/ps_runner.rs`
- **HOSTS_BLOCK_FOOTER** (1 connections) — `src/executor/ps_runner.rs`
- **HOSTS_BLOCK_HEADER** (1 connections) — `src/executor/ps_runner.rs`
- **write** (1 connections)

## Relationships

- [RealBackend](RealBackend.md) (4 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (2 shared connections)
- [.Create Metrics Sampler()](Create_Metrics_Sampler.md) (1 shared connections)
- [.Create Restore Point()](Create_Restore_Point.md) (1 shared connections)
- [engine.rs](engine.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/executor/ps_runner.rs`

## Audit Trail

- EXTRACTED: 37 (97%)
- INFERRED: 1 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*