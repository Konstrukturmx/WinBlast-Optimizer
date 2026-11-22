# normalize_guid_str

> 23 nodes · cohesion 0.20

## Key Concepts

- **normalize_guid_str()** (14 connections) — `src/hardware/power.rs`
- **parse_guid()** (14 connections) — `src/hardware/power.rs`
- **create_and_activate_performance_plan()** (11 connections) — `src/hardware/power.rs`
- **write_power_setting_index()** (11 connections) — `src/hardware/power.rs`
- **get_active_power_scheme()** (10 connections) — `src/hardware/power.rs`
- **list_power_schemes()** (9 connections) — `src/hardware/power.rs`
- **map_win32_err()** (8 connections) — `src/hardware/power.rs`
- **WIN32_ERROR** (8 connections)
- **resolve_scheme()** (7 connections) — `src/hardware/power.rs`
- **unlock_power_setting_range_if_clamped()** (7 connections) — `src/hardware/power.rs`
- **format_guid()** (6 connections) — `src/hardware/power.rs`
- **set_active_power_scheme()** (6 connections) — `src/hardware/power.rs`
- **GUID** (5 connections)
- **.set_active_power_scheme()** (5 connections) — `src/core/backend.rs`
- **.read_power_setting_index()** (4 connections) — `src/core/backend.rs`
- **LocalGuidPtr** (4 connections) — `src/hardware/power.rs`
- **test_guid_roundtrip_and_normalization()** (4 connections) — `src/hardware/power.rs`
- **test_power_scheme_enumeration_nonblocking()** (3 connections) — `src/hardware/power.rs`
- **.create_and_activate_performance_plan()** (2 connections) — `src/core/backend.rs`
- **.get_active_power_scheme()** (2 connections) — `src/core/backend.rs`
- **.set_active_power_scheme()** (2 connections) — `src/core/backend.rs`
- **.write_power_setting_index()** (2 connections) — `src/core/backend.rs`
- **.drop()** (1 connections) — `src/hardware/power.rs`

## Relationships

- [WinBlast Optimizer](WinBlast_Optimizer.md) (20 shared connections)
- [AutoTuneProfile](AutoTuneProfile.md) (6 shared connections)
- [driver_update.rs](driver_update.rs.md) (4 shared connections)
- [spawn_guarded_worker](spawn_guarded_worker.md) (4 shared connections)
- [WinBlastError](WinBlastError.md) (3 shared connections)
- [startup.rs](startup.rs.md) (2 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (2 shared connections)
- [svc_mutator.rs](svc_mutator.rs.md) (1 shared connections)
- [PlatformSecurityInfo](PlatformSecurityInfo.md) (1 shared connections)
- [in_progress_hud.rs](in_progress_hud.rs.md) (1 shared connections)
- [set_usb_root_hub_power_saving](set_usb_root_hub_power_saving.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/power.rs`

## Audit Trail

- EXTRACTED: 91 (96%)
- INFERRED: 4 (4%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*