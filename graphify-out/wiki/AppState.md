# AppState

> 100 nodes · cohesion 0.03

## Key Concepts

- **AppState** (334 connections) — `src/app.rs`
- **.set_status_spec()** (26 connections) — `src/app.rs`
- **.navigate()** (12 connections) — `src/app.rs`
- **.handle_switch_click()** (10 connections) — `src/app.rs`
- **.refresh_pending_actions()** (10 connections) — `src/app.rs`
- **.apply_selected()** (7 connections) — `src/app.rs`
- **.run_scan_internal()** (7 connections) — `src/app.rs`
- **.get_tweak_scan()** (6 connections) — `src/app.rs`
- **.is_tweak_compatible_and_applicable()** (6 connections) — `src/app.rs`
- **test_iso_25010_fault_tolerance_concurrency_and_corruption_recovery()** (6 connections) — `tests/tweak_sandbox_e2e.rs`
- **.apply_preset()** (5 connections) — `src/app.rs`
- **.drain_worker_events_bounded()** (5 connections) — `src/app.rs`
- **.execute_auto_tune_profile()** (5 connections) — `src/app.rs`
- **.import_profile()** (5 connections) — `src/app.rs`
- **.scan_startup_and_tasks_async()** (5 connections) — `src/app.rs`
- **.set_status()** (5 connections) — `src/app.rs`
- **PresetKind** (5 connections) — `src/app.rs`
- **.create_restore_point()** (4 connections) — `src/app.rs`
- **.new_with_backend()** (4 connections) — `src/app.rs`
- **.poll()** (4 connections) — `src/app.rs`
- **.reset_baseline()** (4 connections) — `src/app.rs`
- **.select_all()** (4 connections) — `src/app.rs`
- **.start_latency_benchmark()** (4 connections) — `src/app.rs`
- **.start_scan()** (4 connections) — `src/app.rs`
- **.sync_search_state()** (4 connections) — `src/app.rs`
- *... and 75 more nodes in this community*

## Relationships

- [LayoutTier](LayoutTier.md) (40 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (22 shared connections)
- [Test Composite With()](Test_Composite_With.md) (19 shared connections)
- [action_button](action_button.md) (18 shared connections)
- [action_button_guarded](action_button_guarded.md) (15 shared connections)
- [ThemePalette](ThemePalette.md) (13 shared connections)
- [.sync_clean_apps_busy](sync_clean_apps_busy.md) (12 shared connections)
- [automation.rs](automation.rs.md) (9 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (9 shared connections)
- [View() Color Scrollspec](View_Color_Scrollspec.md) (9 shared connections)
- [.setup](setup.md) (9 shared connections)
- [ui_scrollbar_test.rs](ui_scrollbar_test.rs.md) (8 shared connections)

## Source Files

- `src/app.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 428 (92%)
- INFERRED: 35 (8%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*