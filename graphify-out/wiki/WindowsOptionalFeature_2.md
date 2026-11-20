# WindowsOptionalFeature

> 4 nodes · cohesion 0.50

## Key Concepts

- **set_startup_entry_enabled()** (5 connections) — `src/tools/startup.rs`
- **set_scheduled_task_enabled()** (4 connections) — `src/executor/ps_runner.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`

## Relationships

- [WorkerEvent](WorkerEvent.md) (3 shared connections)
- [DriverUpdateState](DriverUpdateState.md) (1 shared connections)
- [ThemePalette](ThemePalette.md) (1 shared connections)
- [latency_bench.rs](latency_bench.rs.md) (1 shared connections)
- [restart_explorer_safely](restart_explorer_safely.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)

## Source Files

- `src/executor/ps_runner.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 11 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*