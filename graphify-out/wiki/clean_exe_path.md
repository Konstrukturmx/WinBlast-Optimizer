# clean_exe_path

> 5 nodes · cohesion 0.60

## Key Concepts

- **.sample()** (8 connections) — `src/hardware/monitor.rs`
- **SystemMonitor** (7 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **test_monitor_cold_main_thread_never_sleeps_or_spikes()** (3 connections) — `src/hardware/monitor.rs`
- **test_monitor_sample()** (3 connections) — `src/hardware/monitor.rs`

## Relationships

- [ps_runner.rs](ps_runner.rs.md) (6 shared connections)
- [ThemePalette](ThemePalette.md) (4 shared connections)
- [FeatureInstallState](FeatureInstallState.md) (1 shared connections)
- [DateTime](DateTime.md) (1 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [.sync_startup_tasks_busy](sync_startup_tasks_busy.md) (1 shared connections)

## Source Files

- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 20 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*