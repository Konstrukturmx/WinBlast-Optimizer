# .sync_startup_tasks_busy

> 7 nodes · cohesion 0.29

## Key Concepts

- **SystemMetrics** (14 connections) — `src/hardware/monitor.rs`
- **SystemMonitor** (2 connections) — `src/core/backend.rs`
- **.sample()** (2 connections) — `src/core/backend.rs`
- **DateTime** (2 connections)
- **Utc** (2 connections)
- **.ram_display()** (1 connections) — `src/hardware/monitor.rs`
- **.timer_res_ms_display()** (1 connections) — `src/hardware/monitor.rs`

## Relationships

- [backend.rs](backend.rs.md) (3 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (3 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [app.rs](app.rs.md) (1 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [clean_exe_path](clean_exe_path.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 18 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*