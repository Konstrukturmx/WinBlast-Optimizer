# deserialize

> 27 nodes · cohesion 0.11

## Key Concepts

- **backend.rs** (77 connections) — `src/core/backend.rs`
- **SystemMetrics** (14 connections) — `src/hardware/monitor.rs`
- **MetricsSampler** (11 connections) — `src/core/backend.rs`
- **RegStringValue** (11 connections) — `src/core/reg_helpers.rs`
- **test_sandbox_core_parking_labels_metrics_and_power_sync()** (7 connections) — `src/core/backend.rs`
- **.unpark_all_cores()** (5 connections) — `src/core/backend.rs`
- **SandboxMetricsSampler** (5 connections) — `src/core/backend.rs`
- **.reg_get_string()** (5 connections) — `tests/tweak_sandbox_e2e.rs`
- **.create_metrics_sampler()** (4 connections) — `tests/tweak_sandbox_e2e.rs`
- **RwLock** (3 connections)
- **.create_metrics_sampler()** (3 connections) — `src/core/backend.rs`
- **.sample()** (3 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.create_metrics_sampler()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **.reg_get_string()** (3 connections) — `tests/tweak_sandbox_e2e.rs`
- **OnceLock** (2 connections)
- **.sample()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **.reg_get_string()** (2 connections) — `src/core/backend.rs`
- **.create_metrics_sampler()** (2 connections) — `src/core/backend.rs`
- **.reg_get_string()** (2 connections) — `src/core/backend.rs`
- **SystemMonitor** (2 connections) — `src/core/backend.rs`
- **.sample()** (2 connections) — `src/core/backend.rs`
- **normalize_task_path** (1 connections)
- *... and 2 more nodes in this community*

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (6 shared connections)
- [power.rs](power.rs.md) (5 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (5 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (4 shared connections)
- [features_apps.rs](features_apps.rs.md) (4 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (4 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (4 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (4 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (3 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (3 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (3 shared connections)
- [edge_cases_unorthodox.rs](edge_cases_unorthodox.rs.md) (3 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/reg_helpers.rs`
- `src/hardware/monitor.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 141 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*