# normalize_guid_str

> 22 nodes · cohesion 0.23

## Key Concepts

- **normalize_guid_str()** (14 connections) — `src/hardware/power.rs`
- **parse_guid()** (14 connections) — `src/hardware/power.rs`
- **write_power_setting_index()** (11 connections) — `src/hardware/power.rs`
- **create_and_activate_performance_plan()** (10 connections) — `src/hardware/power.rs`
- **get_active_power_scheme()** (10 connections) — `src/hardware/power.rs`
- **read_power_setting_index()** (10 connections) — `src/hardware/power.rs`
- **list_power_schemes()** (9 connections) — `src/hardware/power.rs`
- **map_win32_err()** (8 connections) — `src/hardware/power.rs`
- **WIN32_ERROR** (8 connections)
- **resolve_scheme()** (7 connections) — `src/hardware/power.rs`
- **unlock_power_setting_range_if_clamped()** (7 connections) — `src/hardware/power.rs`
- **format_guid()** (6 connections) — `src/hardware/power.rs`
- **set_active_power_scheme()** (6 connections) — `src/hardware/power.rs`
- **GUID** (5 connections)
- **test_ac_power_status_and_enumerated_range_noop()** (5 connections) — `src/hardware/power.rs`
- **.set_active_power_scheme()** (4 connections) — `src/core/backend.rs`
- **LocalGuidPtr** (4 connections) — `src/hardware/power.rs`
- **test_guid_roundtrip_and_normalization()** (4 connections) — `src/hardware/power.rs`
- **.list_power_schemes()** (3 connections) — `src/core/backend.rs`
- **.read_power_setting_index()** (3 connections) — `src/core/backend.rs`
- **test_power_scheme_enumeration_nonblocking()** (3 connections) — `src/hardware/power.rs`
- **.drop()** (1 connections) — `src/hardware/power.rs`

## Relationships

- [Core UI Internationalization & Text Catalog](Core_UI_Internationalization_&_Text_Catalog.md) (20 shared connections)
- [RealBackend](RealBackend.md) (7 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (5 shared connections)
- [Embedded Catalog Test](Embedded_Catalog_Test.md) (3 shared connections)
- [app.rs](app.rs.md) (2 shared connections)
- [engine.rs](engine.rs.md) (2 shared connections)
- [State Ledger & Restore Point Engine](State_Ledger_&_Restore_Point_Engine.md) (2 shared connections)
- [Disabled() Delete .Set](Disabled_Delete_.Set.md) (1 shared connections)
- [Win32 Registry Scanner & Mutator](Win32_Registry_Scanner_&_Mutator.md) (1 shared connections)
- [Scandetailkind Appxinstalled Appxinstalledps](Scandetailkind_Appxinstalled_Appxinstalledps.md) (1 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/power.rs`

## Audit Trail

- EXTRACTED: 93 (95%)
- INFERRED: 5 (5%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*