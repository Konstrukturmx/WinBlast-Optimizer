# startup.rs

> 21 nodes · cohesion 0.20

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **list_live_services()** (7 connections) — `src/tools/startup.rs`
- **scan_run_key()** (7 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (6 connections) — `src/tools/startup.rs`
- **list_startup_entries()** (6 connections) — `src/tools/startup.rs`
- **scan_startup_folder()** (6 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled()** (5 connections) — `src/tools/startup.rs`
- **HKEY** (4 connections)
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **read_startup_approved_enabled()** (4 connections) — `src/tools/startup.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **scan_uwp_startup_tasks()** (3 connections) — `src/tools/startup.rs`
- **set_startup_entry_enabled_by_id()** (3 connections) — `src/tools/startup.rs`
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- **CRITICAL_CORE_SERVICES** (1 connections) — `src/tools/startup.rs`

## Relationships

- [LiveServiceInfo](LiveServiceInfo.md) (9 shared connections)
- [StartupEntry](StartupEntry.md) (6 shared connections)
- [deserialize](deserialize.md) (4 shared connections)
- [StartupSource](StartupSource.md) (3 shared connections)
- [real_windows_env.rs](real_windows_env.rs.md) (2 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (2 shared connections)
- [LiveScheduledTaskInfo](LiveScheduledTaskInfo.md) (2 shared connections)
- [decode_oem_bytes](decode_oem_bytes.md) (2 shared connections)
- [.setup](setup.md) (2 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [TaskCache](TaskCache.md) (1 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (1 shared connections)

## Source Files

- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 80 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*