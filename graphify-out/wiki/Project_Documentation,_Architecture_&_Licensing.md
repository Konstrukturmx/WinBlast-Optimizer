# Project Documentation, Architecture & Licensing

> 8 nodes · cohesion 0.43

## Key Concepts

- **cpu.rs** (9 connections) — `src/hardware/cpu.rs`
- **classify_efficiency_records()** (4 connections) — `src/hardware/cpu.rs`
- **query_win32_core_topology()** (4 connections) — `src/hardware/cpu.rs`
- **.detect()** (3 connections) — `src/hardware/cpu.rs`
- **Win32CoreTopology** (3 connections) — `src/hardware/cpu.rs`
- **test_classify_efficiency_records_hybrid_and_non_ht_and_uniform()** (2 connections) — `src/hardware/cpu.rs`
- **test_cpu_detect_host_safe()** (2 connections) — `src/hardware/cpu.rs`
- **cpuid** (1 connections)

## Relationships

- [backend.rs](backend.rs.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)

## Source Files

- `src/hardware/cpu.rs`

## Audit Trail

- EXTRACTED: 16 (100%)
- INFERRED: 0 (0%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*