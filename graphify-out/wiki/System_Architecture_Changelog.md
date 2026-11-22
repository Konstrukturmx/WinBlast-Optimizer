# System Architecture Changelog

> 53 nodes · cohesion 0.06

## Key Concepts

- **ActiveView** (49 connections) — `src/app.rs`
- **ui_scrollbar_test.rs** (24 connections) — `tests/ui_scrollbar_test.rs`
- **runtime_for()** (9 connections) — `tests/ui_scrollbar_test.rs`
- **thumb_rects()** (8 connections) — `tests/ui_scrollbar_test.rs`
- **content_thumb()** (7 connections) — `tests/ui_scrollbar_test.rs`
- **frames()** (7 connections) — `tests/ui_scrollbar_test.rs`
- **Runtime** (7 connections)
- **create_test_state()** (6 connections) — `tests/ui_scrollbar_test.rs`
- **sidebar_thumb()** (5 connections) — `tests/ui_scrollbar_test.rs`
- **test_advanced_matrix_scrollbar_structure_and_drag()** (5 connections) — `tests/ui_scrollbar_test.rs`
- **test_wheel_scroll_moves_content_and_sidebar_thumbs()** (5 connections) — `tests/ui_scrollbar_test.rs`
- **count_key()** (4 connections) — `tests/ui_scrollbar_test.rs`
- **test_iso_29119_driver_section_scrollbar_bounds_and_filtering()** (4 connections) — `tests/ui_scrollbar_test.rs`
- **test_scrollbars_render_across_all_main_views()** (4 connections) — `tests/ui_scrollbar_test.rs`
- **test_themed_thumb_widens_under_the_pointer()** (4 connections) — `tests/ui_scrollbar_test.rs`
- **.nav_prev_view()** (3 connections) — `src/app.rs`
- **AdvancedMatrix** (2 connections) — `src/app.rs`
- **CoreParking** (2 connections) — `src/app.rs`
- **EasyDashboard** (2 connections) — `src/app.rs`
- **GpuTuning** (2 connections) — `src/app.rs`
- **.prev()** (2 connections) — `src/app.rs`
- **ALL_VIEWS** (2 connections) — `tests/ui_responsive_e2e.rs`
- **Advanced Optimization Matrix UI** (1 connections) — `assets/screenshots/advanced_matrix.png`
- **CPU Core Parking Topology Visualizer UI** (1 connections) — `assets/screenshots/cpu_parking.png`
- **Easy Mode Telemetry Dashboard UI** (1 connections) — `assets/screenshots/dashboard.png`
- *... and 28 more nodes in this community*

## Relationships

- [Disk Cleaning & Temporary File Purger](Disk_Cleaning_&_Temporary_File_Purger.md) (10 shared connections)
- [PowerSchemeInfo](PowerSchemeInfo.md) (3 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (3 shared connections)
- [enums](enums.md) (2 shared connections)
- [RealBackend](RealBackend.md) (2 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (2 shared connections)
- [parse_guid](parse_guid.md) (2 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (2 shared connections)
- [full_telemetry_audit_with](full_telemetry_audit_with.md) (1 shared connections)
- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (1 shared connections)
- [bridge.rs](bridge.rs.md) (1 shared connections)
- [features_apps.rs](features_apps.rs.md) (1 shared connections)

## Source Files

- `assets/screenshots/advanced_matrix.png`
- `assets/screenshots/cpu_parking.png`
- `assets/screenshots/dashboard.png`
- `assets/screenshots/gpu_tuning.png`
- `src/app.rs`
- `tests/ui_responsive_e2e.rs`
- `tests/ui_scrollbar_test.rs`

## Audit Trail

- EXTRACTED: 111 (97%)
- INFERRED: 3 (3%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*