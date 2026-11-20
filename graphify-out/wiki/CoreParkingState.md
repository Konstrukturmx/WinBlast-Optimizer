# CoreParkingState

> 21 nodes · cohesion 0.12

## Key Concepts

- **enums** (20 connections)
- **RegKey** (20 connections)
- **WindowsInfo** (15 connections) — `src/core/os_detect.rs`
- **BiosSystemInfo** (8 connections) — `src/hardware/bios.rs`
- **bios.rs** (6 connections) — `src/hardware/bios.rs`
- **GameProfile** (6 connections) — `src/tools/game_profiles.rs`
- **os_detect.rs** (5 connections) — `src/core/os_detect.rs`
- **tools/game_profiles.rs** (5 connections) — `src/tools/game_profiles.rs`
- **.detect_os()** (2 connections) — `src/core/backend.rs`
- **.detect_os()** (2 connections) — `src/core/backend.rs`
- **test_windows_info_detect_host()** (2 connections) — `src/core/os_detect.rs`
- **.detect()** (2 connections) — `src/core/os_detect.rs`
- **DisplayDriverInfo** (2 connections) — `src/hardware/bios.rs`
- **.all()** (2 connections) — `src/tools/game_profiles.rs`
- **.check_applied()** (2 connections) — `src/tools/game_profiles.rs`
- **.has_recall()** (1 connections) — `src/core/os_detect.rs`
- **.supports_hags()** (1 connections) — `src/core/os_detect.rs`
- **.has_warning()** (1 connections) — `src/hardware/bios.rs`
- **.probe()** (1 connections) — `src/hardware/bios.rs`
- **.apply()** (1 connections) — `src/tools/game_profiles.rs`
- **.revert()** (1 connections) — `src/tools/game_profiles.rs`

## Relationships

- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (4 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (3 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (3 shared connections)
- [deserialize](deserialize.md) (3 shared connections)
- [theme.rs](theme.rs.md) (3 shared connections)
- [.Toggle .Set .Apply](Toggle_.Set_.Apply.md) (3 shared connections)
- [WinBlast Optimizer](WinBlast_Optimizer.md) (3 shared connections)
- [test_wave1_compound_actions_deep_edge_cases](test_wave1_compound_actions_deep_edge_cases.md) (2 shared connections)
- [reg_mutator.rs](reg_mutator.rs.md) (2 shared connections)
- [.logical_core_count](logical_core_count.md) (2 shared connections)
- [Any](Any.md) (2 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/core/os_detect.rs`
- `src/hardware/bios.rs`
- `src/tools/game_profiles.rs`

## Audit Trail

- EXTRACTED: 77 (96%)
- INFERRED: 3 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*