# AppState

> 89 nodes · cohesion 0.04

## Key Concepts

- **AppState** (333 connections) — `src/app.rs`
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
- **.import_profile()** (5 connections) — `src/app.rs`
- **.new_with_backend()** (5 connections) — `src/app.rs`
- **.rollback_entry()** (5 connections) — `src/app.rs`
- **.scan_startup_and_tasks_async()** (5 connections) — `src/app.rs`
- **.set_status()** (5 connections) — `src/app.rs`
- **PresetKind** (5 connections) — `src/app.rs`
- **.create_restore_point()** (4 connections) — `src/app.rs`
- **.poll()** (4 connections) — `src/app.rs`
- **.reset_baseline()** (4 connections) — `src/app.rs`
- **.select_all()** (4 connections) — `src/app.rs`
- **.start_latency_benchmark()** (4 connections) — `src/app.rs`
- **.sync_search_state()** (4 connections) — `src/app.rs`
- **.tr()** (4 connections) — `src/app.rs`
- *... and 64 more nodes in this community*

## Relationships

- [spawn_guarded_worker](spawn_guarded_worker.md) (40 shared connections)
- [LayoutTier](LayoutTier.md) (35 shared connections)
- [Test Composite With()](Test_Composite_With.md) (31 shared connections)
- [automation.rs](automation.rs.md) (15 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (15 shared connections)
- [View() Color Scrollspec](View_Color_Scrollspec.md) (15 shared connections)
- [action_button](action_button.md) (15 shared connections)
- [app.rs](app.rs.md) (14 shared connections)
- [ThemePalette](ThemePalette.md) (13 shared connections)
- [action_button_guarded](action_button_guarded.md) (13 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (11 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (7 shared connections)

## Source Files

- `src/app.rs`
- `src/core/elevation.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 441 (95%)
- INFERRED: 25 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*