# AppX Package & Modern App Scanner

> 24 nodes · cohesion 0.17

## Key Concepts

- **tools/game_prereqs.rs** (32 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_runtime_detection_heuristics_execution()** (11 connections) — `src/tools/game_prereqs.rs`
- **GamePrerequisiteItem** (6 connections) — `src/tools/game_prereqs.rs`
- **get_system32_path()** (6 connections) — `src/tools/game_prereqs.rs`
- **detect_openal()** (4 connections) — `src/tools/game_prereqs.rs`
- **install_prereq_cascade()** (4 connections) — `src/tools/game_prereqs.rs`
- **scan_game_prereqs()** (4 connections) — `src/tools/game_prereqs.rs`
- **detect_directx_legacy()** (3 connections) — `src/tools/game_prereqs.rs`
- **detect_vcredist_2015_2022_x64()** (3 connections) — `src/tools/game_prereqs.rs`
- **detect_vcredist_2015_2022_x86()** (3 connections) — `src/tools/game_prereqs.rs`
- **detect_vcredist_all_in_one()** (3 connections) — `src/tools/game_prereqs.rs`
- **detect_vulkan_runtime()** (3 connections) — `src/tools/game_prereqs.rs`
- **get_syswow64_path()** (3 connections) — `src/tools/game_prereqs.rs`
- **detect_dotnet_desktop_8()** (2 connections) — `src/tools/game_prereqs.rs`
- **detect_dotnet_desktop_9()** (2 connections) — `src/tools/game_prereqs.rs`
- **detect_dotnet_framework_481()** (2 connections) — `src/tools/game_prereqs.rs`
- **detect_xna_framework_4()** (2 connections) — `src/tools/game_prereqs.rs`
- **PREREQ_DESCRIPTORS** (2 connections) — `src/tools/game_prereqs.rs`
- **PrereqDescriptor** (2 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_cascade_error_recovery_empty_source()** (2 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_cascade_fallback_url_validity()** (2 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_scan_returns_all_10_prereqs()** (2 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_serde_roundtrip()** (2 connections) — `src/tools/game_prereqs.rs`
- **.is_installed()** (1 connections) — `src/tools/game_prereqs.rs`

## Relationships

- [deserialize](deserialize.md) (2 shared connections)
- [PrereqInstallState](PrereqInstallState.md) (2 shared connections)
- [path](path.md) (1 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [AppxCache](AppxCache.md) (1 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)
- [.maybe_arm_error](maybe_arm_error.md) (1 shared connections)
- [WinBlast Optimizer — Developer & Architecture Guide](WinBlast_Optimizer_—_Developer_&_Architecture_Guide.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/tools/game_prereqs.rs`

## Audit Trail

- EXTRACTED: 58 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*