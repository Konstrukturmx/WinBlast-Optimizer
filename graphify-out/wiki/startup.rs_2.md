# startup.rs

> 30 nodes · cohesion 0.12

## Key Concepts

- **tools/game_prereqs.rs** (32 connections) — `src/tools/game_prereqs.rs`
- **test_iso_29119_runtime_detection_heuristics_execution()** (11 connections) — `src/tools/game_prereqs.rs`
- **PrereqInstallState** (7 connections) — `src/tools/game_prereqs.rs`
- **GamePrerequisiteItem** (6 connections) — `src/tools/game_prereqs.rs`
- **get_system32_path()** (6 connections) — `src/tools/game_prereqs.rs`
- **install_prereq_cascade()** (5 connections) — `src/tools/game_prereqs.rs`
- **scan_game_prereqs()** (5 connections) — `src/tools/game_prereqs.rs`
- **detect_openal()** (4 connections) — `src/tools/game_prereqs.rs`
- **.scan_game_prereqs()** (3 connections) — `src/app.rs`
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
- *... and 5 more nodes in this community*

## Relationships

- [AppState](AppState.md) (3 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [.setup](setup.md) (1 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (1 shared connections)
- [InstalledAppxPackage](InstalledAppxPackage.md) (1 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)

## Source Files

- `src/app.rs`
- `src/tools/game_prereqs.rs`

## Audit Trail

- EXTRACTED: 67 (99%)
- INFERRED: 1 (1%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*