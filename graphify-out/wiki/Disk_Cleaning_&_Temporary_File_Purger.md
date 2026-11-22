# Disk Cleaning & Temporary File Purger

> 73 nodes · cohesion 0.04

## Key Concepts

- **AppState** (371 connections) — `src/app.rs`
- **views/network_diag.rs** (14 connections) — `src/ui/views/network_diag.rs`
- **view()** (8 connections) — `src/ui/views/network_diag.rs`
- **StartupStatusFilter** (7 connections) — `src/app.rs`
- **.drain_worker_events_bounded()** (6 connections) — `src/app.rs`
- **.poll()** (5 connections) — `src/app.rs`
- **.sync_search_state()** (5 connections) — `src/app.rs`
- **.new_with_backend()** (4 connections) — `src/app.rs`
- **.poll_headless()** (4 connections) — `src/app.rs`
- **.cycle_telemetry_interval()** (3 connections) — `src/app.rs`
- **.is_in_progress()** (3 connections) — `src/app.rs`
- **.new()** (3 connections) — `src/app.rs`
- **.set_theme_mode()** (3 connections) — `src/app.rs`
- **.telemetry_interval_ms()** (3 connections) — `src/app.rs`
- **.toggle_maintenance_task()** (3 connections) — `src/app.rs`
- **localize_dns_category()** (3 connections) — `src/ui/views/network_diag.rs`
- **localize_dns_status()** (3 connections) — `src/ui/views/network_diag.rs`
- **.calculate_before_after_impact()** (2 connections) — `src/app.rs`
- **.default()** (2 connections) — `src/app.rs`
- **.drain_worker_events()** (2 connections) — `src/app.rs`
- **.execute_memory_trim()** (2 connections) — `src/app.rs`
- **.install_all_missing_prereqs()** (2 connections) — `src/app.rs`
- **.install_game_prereq()** (2 connections) — `src/app.rs`
- **.refresh_maintenance_tasks()** (2 connections) — `src/app.rs`
- **.select_all_filtered_startup()** (2 connections) — `src/app.rs`
- *... and 48 more nodes in this community*

## Relationships

- [features_apps.rs](features_apps.rs.md) (40 shared connections)
- [super](super.md) (31 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (20 shared connections)
- [clean_apps.rs](clean_apps.rs.md) (20 shared connections)
- [startup.rs](startup.rs.md) (17 shared connections)
- [app.rs](app.rs.md) (17 shared connections)
- [parse_guid](parse_guid.md) (16 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (15 shared connections)
- [RealBackend](RealBackend.md) (13 shared connections)
- [System Architecture Changelog](System_Architecture_Changelog.md) (10 shared connections)
- [WindowsOptionalFeature](WindowsOptionalFeature.md) (10 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (9 shared connections)

## Source Files

- `src/app.rs`
- `src/ui/views/network_diag.rs`

## Audit Trail

- EXTRACTED: 393 (93%)
- INFERRED: 28 (7%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*