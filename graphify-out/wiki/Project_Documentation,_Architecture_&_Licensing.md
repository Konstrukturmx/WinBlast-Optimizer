# Project Documentation, Architecture & Licensing

> 21 nodes · cohesion 0.11

## Key Concepts

- **AutoTuneProfile** (17 connections) — `src/tools/automation.rs`
- **RiskLevel** (15 connections) — `src/core/risk.rs`
- **create_auto_tune_plan()** (6 connections) — `src/tools/automation.rs`
- **AutoTunePlan** (4 connections) — `src/tools/automation.rs`
- **.target_risk_level()** (4 connections) — `src/tools/automation.rs`
- **test_iso_29119_auto_tune_plan_generation_all_profiles()** (4 connections) — `src/tools/automation.rs`
- **AutoTuneResult** (3 connections) — `src/tools/automation.rs`
- **.generate_auto_tune_plan()** (2 connections) — `src/app.rs`
- **Extreme** (1 connections) — `src/core/risk.rs`
- **High** (1 connections) — `src/core/risk.rs`
- **Low** (1 connections) — `src/core/risk.rs`
- **Moderate** (1 connections) — `src/core/risk.rs`
- **Safe** (1 connections) — `src/core/risk.rs`
- **.all()** (1 connections) — `src/tools/automation.rs`
- **.description()** (1 connections) — `src/tools/automation.rs`
- **.display_name()** (1 connections) — `src/tools/automation.rs`
- **GamingLowLatency** (1 connections) — `src/tools/automation.rs`
- **.icon()** (1 connections) — `src/tools/automation.rs`
- **LaptopBatterySaving** (1 connections) — `src/tools/automation.rs`
- **ProAudioDpc** (1 connections) — `src/tools/automation.rs`
- **WorkstationStability** (1 connections) — `src/tools/automation.rs`

## Relationships

- [power.rs](power.rs.md) (6 shared connections)
- [AppState](AppState.md) (5 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (3 shared connections)
- [super](super.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)
- [RealBackend](RealBackend.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [parse_guid](parse_guid.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/core/risk.rs`
- `src/tools/automation.rs`

## Audit Trail

- EXTRACTED: 42 (93%)
- INFERRED: 3 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*