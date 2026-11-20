# task_scanner.rs

> 21 nodes · cohesion 0.16

## Key Concepts

- **adaptive_tuning.rs** (16 connections) — `src/executor/adaptive_tuning.rs`
- **storage.rs** (8 connections) — `src/hardware/storage.rs`
- **StorageMedium** (8 connections) — `src/hardware/storage.rs`
- **evaluate_adaptive_tuning()** (7 connections) — `src/executor/adaptive_tuning.rs`
- **apply_adaptive_tuning()** (5 connections) — `src/executor/adaptive_tuning.rs`
- **StorageMediumInfo** (5 connections) — `src/hardware/storage.rs`
- **diagnose_storage_medium()** (4 connections) — `src/hardware/storage.rs`
- **AdaptiveTuningConfig** (3 connections) — `src/executor/adaptive_tuning.rs`
- **MemoryStats** (3 connections) — `src/executor/adaptive_tuning.rs`
- **test_apply_adaptive_tuning_sandbox()** (3 connections) — `src/executor/adaptive_tuning.rs`
- **probe_volume_seek_penalty()** (3 connections) — `src/hardware/storage.rs`
- **query_physical_ram()** (2 connections) — `src/executor/adaptive_tuning.rs`
- **test_evaluate_adaptive_tuning_high_spec_nvme()** (2 connections) — `src/executor/adaptive_tuning.rs`
- **test_evaluate_adaptive_tuning_low_spec_hdd()** (2 connections) — `src/executor/adaptive_tuning.rs`
- **test_storage_medium_info()** (2 connections) — `src/hardware/storage.rs`
- **B** (1 connections)
- **MechanicalHdd** (1 connections) — `src/hardware/storage.rs`
- **Nvme** (1 connections) — `src/hardware/storage.rs`
- **SataSsd** (1 connections) — `src/hardware/storage.rs`
- **Unknown** (1 connections) — `src/hardware/storage.rs`
- **UsbRemovable** (1 connections) — `src/hardware/storage.rs`

## Relationships

- [parking.rs](parking.rs.md) (4 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (4 shared connections)
- [startup.rs](startup.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (1 shared connections)

## Source Files

- `src/executor/adaptive_tuning.rs`
- `src/hardware/storage.rs`

## Audit Trail

- EXTRACTED: 44 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*