# AppState

> 82 nodes · cohesion 0.04

## Key Concepts

- **AppState** (347 connections) — `src/app.rs`
- **.set_status_spec()** (26 connections) — `src/app.rs`
- **.refresh_pending_actions()** (10 connections) — `src/app.rs`
- **.handle_switch_click()** (8 connections) — `src/app.rs`
- **.run_scan_internal()** (7 connections) — `src/app.rs`
- **.get_tweak_scan()** (6 connections) — `src/app.rs`
- **.is_tweak_compatible_and_applicable()** (6 connections) — `src/app.rs`
- **.apply_preset()** (5 connections) — `src/app.rs`
- **.apply_selected()** (5 connections) — `src/app.rs`
- **.set_status()** (5 connections) — `src/app.rs`
- **PresetKind** (5 connections) — `src/app.rs`
- **.execute_auto_tune_profile()** (4 connections) — `src/app.rs`
- **.import_profile()** (4 connections) — `src/app.rs`
- **.new_with_backend()** (4 connections) — `src/app.rs`
- **.rollback_entry()** (4 connections) — `src/app.rs`
- **.select_all()** (4 connections) — `src/app.rs`
- **.start_latency_benchmark()** (4 connections) — `src/app.rs`
- **.start_scan()** (4 connections) — `src/app.rs`
- **.tr()** (4 connections) — `src/app.rs`
- **.cycle_telemetry_interval()** (3 connections) — `src/app.rs`
- **.deselect_all()** (3 connections) — `src/app.rs`
- **.detected_gpu_vendors()** (3 connections) — `src/app.rs`
- **.export_autounattend_xml()** (3 connections) — `src/app.rs`
- **.force_scan()** (3 connections) — `src/app.rs`
- **.install_game_prereq()** (3 connections) — `src/app.rs`
- *... and 57 more nodes in this community*

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (47 shared connections)
- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (33 shared connections)
- [WorkerEvent](WorkerEvent.md) (27 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (19 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (17 shared connections)
- [SlowScanBackend](SlowScanBackend.md) (15 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (15 shared connections)
- [.take_injected_error](take_injected_error.md) (15 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (8 shared connections)
- [parse_guid](parse_guid.md) (7 shared connections)
- [startup.rs](startup.rs.md) (6 shared connections)
- [advanced_matrix.rs](advanced_matrix.rs.md) (6 shared connections)

## Source Files

- `src/app.rs`

## Audit Trail

- EXTRACTED: 423 (95%)
- INFERRED: 24 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*