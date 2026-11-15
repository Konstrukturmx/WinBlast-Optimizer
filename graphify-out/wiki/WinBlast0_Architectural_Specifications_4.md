# WinBlast0 Architectural Specifications

> 6 nodes · cohesion 0.47

## Key Concepts

- **SystemMonitor** (7 connections) — `src/hardware/monitor.rs`
- **.sample()** (7 connections) — `src/hardware/monitor.rs`
- **.new()** (5 connections) — `src/hardware/monitor.rs`
- **System** (4 connections)
- **test_monitor_cold_main_thread_never_sleeps_or_spikes()** (3 connections) — `src/hardware/monitor.rs`
- **test_monitor_sample()** (3 connections) — `src/hardware/monitor.rs`

## Relationships

- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (7 shared connections)
- [Restore Points() .List](Restore_Points_.List.md) (4 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (1 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (1 shared connections)
- [Project Documentation, Architecture & Licensing](Project_Documentation,_Architecture_&_Licensing.md) (1 shared connections)

## Source Files

- `src/hardware/monitor.rs`

## Audit Trail

- EXTRACTED: 22 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*