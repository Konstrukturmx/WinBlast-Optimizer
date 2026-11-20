# app.rs

> 19 nodes · cohesion 0.15

## Key Concepts

- **updater.rs** (21 connections) — `src/core/updater.rs`
- **UpdateState** (9 connections) — `src/core/updater.rs`
- **check_latest_release()** (5 connections) — `src/core/updater.rs`
- **ReleaseInfo** (5 connections) — `src/core/updater.rs`
- **is_version_newer()** (4 connections) — `src/core/updater.rs`
- **fetch_url_json()** (2 connections) — `src/core/updater.rs`
- **GhAsset** (2 connections) — `src/core/updater.rs`
- **GhRelease** (2 connections) — `src/core/updater.rs`
- **test_release_info_structure()** (2 connections) — `src/core/updater.rs`
- **test_version_comparison_logic()** (2 connections) — `src/core/updater.rs`
- **FALLBACK_REPO** (1 connections) — `src/core/updater.rs`
- **GITHUB_REPO** (1 connections) — `src/core/updater.rs`
- **Checking** (1 connections) — `src/core/updater.rs`
- **Downloading** (1 connections) — `src/core/updater.rs`
- **Failed** (1 connections) — `src/core/updater.rs`
- **Idle** (1 connections) — `src/core/updater.rs`
- **ReadyToRestart** (1 connections) — `src/core/updater.rs`
- **UpdateAvailable** (1 connections) — `src/core/updater.rs`
- **UpToDate** (1 connections) — `src/core/updater.rs`

## Relationships

- [WorkerEvent](WorkerEvent.md) (4 shared connections)
- [deserialize](deserialize.md) (2 shared connections)
- [clean_exe_path](clean_exe_path.md) (2 shared connections)
- [FailAtMutationBackend](FailAtMutationBackend.md) (1 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (1 shared connections)
- [.setup](setup.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [UpdateStatus](UpdateStatus.md) (1 shared connections)

## Source Files

- `src/core/updater.rs`

## Audit Trail

- EXTRACTED: 34 (89%)
- INFERRED: 4 (11%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*