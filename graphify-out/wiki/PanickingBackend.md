# PanickingBackend

> 26 nodes · cohesion 0.14

## Key Concepts

- **PresetPackage** (15 connections) — `src/backup/preset_sharing.rs`
- **.new()** (13 connections) — `src/backup/preset_sharing.rs`
- **simulate_preset_dry_run()** (9 connections) — `src/backup/preset_sharing.rs`
- **sample_metadata()** (8 connections) — `src/backup/preset_sharing.rs`
- **.encode()** (7 connections) — `src/backup/preset_sharing.rs`
- **test_ed25519_sign_and_verify_policy()** (7 connections) — `src/backup/preset_sharing.rs`
- **.refresh_digest()** (6 connections) — `src/backup/preset_sharing.rs`
- **.sign()** (6 connections) — `src/backup/preset_sharing.rs`
- **SignaturePolicy** (6 connections) — `src/backup/preset_sharing.rs`
- **compute_sha256()** (5 connections) — `src/backup/preset_sharing.rs`
- **PresetMetadata** (5 connections) — `src/backup/preset_sharing.rs`
- **.compute_payload_digest()** (5 connections) — `src/backup/preset_sharing.rs`
- **.verify_signature()** (5 connections) — `src/backup/preset_sharing.rs`
- **test_digest_mismatch_tampering_detection()** (5 connections) — `src/backup/preset_sharing.rs`
- **test_simulate_preset_dry_run_with_catalog()** (5 connections) — `src/backup/preset_sharing.rs`
- **build_signing_transcript()** (4 connections) — `src/backup/preset_sharing.rs`
- **OverrideKey** (4 connections) — `src/backup/preset_sharing.rs`
- **PresetGuards** (4 connections) — `src/backup/preset_sharing.rs`
- **test_orphaned_override_rejection()** (4 connections) — `src/backup/preset_sharing.rs`
- **.new()** (3 connections) — `src/backup/preset_sharing.rs`
- **test_duplicate_tweak_id_rejection()** (3 connections) — `src/backup/preset_sharing.rs`
- **VerifyingKey** (3 connections)
- **SigningKey** (2 connections)
- **Required** (2 connections) — `src/backup/preset_sharing.rs`
- **AllowUnsigned** (1 connections) — `src/backup/preset_sharing.rs`
- *... and 1 more nodes in this community*

## Relationships

- [startup.rs](startup.rs.md) (17 shared connections)
- [path](path.md) (12 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (9 shared connections)
- [privacy_shield.rs](privacy_shield.rs.md) (4 shared connections)
- [.all_physical](all_physical.md) (1 shared connections)
- [progress_bar.rs](progress_bar.rs.md) (1 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)

## Source Files

- `src/backup/preset_sharing.rs`

## Audit Trail

- EXTRACTED: 91 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*