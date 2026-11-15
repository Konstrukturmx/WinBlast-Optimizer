# Integration Suite: Tweak Sandbox E2E

> 26 nodes · cohesion 0.14

## Key Concepts

- **startup.rs** (48 connections) — `src/tools/startup.rs`
- **LiveServiceInfo** (14 connections) — `src/tools/startup.rs`
- **list_live_services()** (8 connections) — `src/tools/startup.rs`
- **is_protected_core_service()** (7 connections) — `src/tools/startup.rs`
- **enum_scm_services_by_type()** (5 connections) — `src/tools/startup.rs`
- **query_scm_win32_services()** (5 connections) — `src/tools/startup.rs`
- **SC_HANDLE** (4 connections)
- **expand_system_root_vars()** (4 connections) — `src/tools/startup.rs`
- **is_microsoft_service_binary()** (4 connections) — `src/tools/startup.rs`
- **query_scm_service_config()** (4 connections) — `src/tools/startup.rs`
- **ScHandleGuard** (4 connections) — `src/tools/startup.rs`
- **.list_live_services()** (3 connections) — `src/core/backend.rs`
- **.set_service_start_type()** (3 connections) — `src/core/backend.rs`
- **encode_startup_approved_bytes()** (3 connections) — `src/tools/startup.rs`
- **is_microsoft_run_command()** (3 connections) — `src/tools/startup.rs`
- **ScmServiceRecord** (3 connections) — `src/tools/startup.rs`
- **ENUM_SERVICE_TYPE** (2 connections)
- **.list_live_services()** (2 connections) — `src/core/backend.rs`
- **.list_live_services()** (2 connections) — `src/core/backend.rs`
- **replace_ascii_ci()** (2 connections) — `src/tools/startup.rs`
- **resolve_indirect_display_string()** (2 connections) — `src/tools/startup.rs`
- **.drop()** (2 connections) — `src/tools/startup.rs`
- **test_critical_service_blocklist_protects_core_services()** (2 connections) — `src/tools/startup.rs`
- **test_startup_approved_12_byte_encoding()** (2 connections) — `src/tools/startup.rs`
- **fromregvalue** (1 connections)
- *... and 1 more nodes in this community*

## Relationships

- [.Create Restore Point()](Create_Restore_Point.md) (12 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (5 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (4 shared connections)
- [Telemetry Full Blocked](Telemetry_Full_Blocked.md) (4 shared connections)
- [WinBlast0 Architectural Specifications](WinBlast0_Architectural_Specifications.md) (3 shared connections)
- [Kernel Scheduling & Latency Optimization](Kernel_Scheduling_&_Latency_Optimization.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [Themed Interactive Scrollbar Widget](Themed_Interactive_Scrollbar_Widget.md) (2 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (2 shared connections)
- [Scan Test Single](Scan_Test_Single.md) (1 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (1 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/tools/startup.rs`

## Audit Trail

- EXTRACTED: 92 (98%)
- INFERRED: 2 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*