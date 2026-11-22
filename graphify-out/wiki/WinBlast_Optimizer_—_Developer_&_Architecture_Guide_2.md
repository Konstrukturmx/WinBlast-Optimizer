# WinBlast Optimizer — Developer & Architecture Guide

> 15 nodes · cohesion 0.16

## Key Concepts

- **serialize** (35 connections)
- **instant** (8 connections)
- **tools/gaming_stats.rs** (5 connections) — `src/tools/gaming_stats.rs`
- **tools/performance_tests.rs** (5 connections) — `src/tools/performance_tests.rs`
- **BenchmarkScore** (5 connections) — `src/tools/performance_tests.rs`
- **GameFpsComparison** (4 connections) — `src/tools/gaming_stats.rs`
- **LiveGameTelemetry** (4 connections) — `src/tools/gaming_stats.rs`
- **get_standard_game_benchmarks()** (2 connections) — `src/tools/gaming_stats.rs`
- **.default()** (2 connections) — `src/tools/gaming_stats.rs`
- **.probe()** (2 connections) — `src/tools/gaming_stats.rs`
- **BenchmarkRunner** (2 connections) — `src/tools/performance_tests.rs`
- **.run_quick()** (2 connections) — `src/tools/performance_tests.rs`
- **.fps_gain_pct()** (1 connections) — `src/tools/gaming_stats.rs`
- **.low_gain_pct()** (1 connections) — `src/tools/gaming_stats.rs`
- **.default()** (1 connections) — `src/tools/performance_tests.rs`

## Relationships

- [deserialize](deserialize.md) (4 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (2 shared connections)
- [RestorePoint](RestorePoint.md) (2 shared connections)
- [appx_scanner.rs](appx_scanner.rs.md) (2 shared connections)
- [topological_scheduler.rs](topological_scheduler.rs.md) (2 shared connections)
- [Project History & Release Version Ledger](Project_History_&_Release_Version_Ledger.md) (2 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (2 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [driver_update.rs](driver_update.rs.md) (1 shared connections)
- [ps_runner.rs](ps_runner.rs.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [c_void](c_void.md) (1 shared connections)

## Source Files

- `src/tools/gaming_stats.rs`
- `src/tools/performance_tests.rs`

## Audit Trail

- EXTRACTED: 59 (95%)
- INFERRED: 3 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*