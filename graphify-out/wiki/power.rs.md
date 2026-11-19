# power.rs

> 77 nodes · cohesion 0.05

## Key Concepts

- **power.rs** (60 connections) — `src/hardware/power.rs`
- **parking.rs** (24 connections) — `src/hardware/parking.rs`
- **CoreParkingState** (21 connections) — `src/hardware/parking.rs`
- **parse_guid()** (11 connections) — `src/hardware/power.rs`
- **ParkingSnapshot** (10 connections) — `src/hardware/parking.rs`
- **create_and_activate_performance_plan()** (10 connections) — `src/hardware/power.rs`
- **normalize_guid_str()** (10 connections) — `src/hardware/power.rs`
- **write_power_setting_index()** (10 connections) — `src/hardware/power.rs`
- **get_active_power_scheme()** (9 connections) — `src/hardware/power.rs`
- **read_power_setting_index()** (9 connections) — `src/hardware/power.rs`
- **write_parking_thresholds()** (8 connections) — `src/hardware/parking.rs`
- **list_power_schemes()** (8 connections) — `src/hardware/power.rs`
- **map_win32_err()** (8 connections) — `src/hardware/power.rs`
- **WIN32_ERROR** (8 connections)
- **.detect_internal()** (7 connections) — `src/hardware/parking.rs`
- **resolve_scheme()** (7 connections) — `src/hardware/power.rs`
- **unlock_power_setting_range_if_clamped()** (7 connections) — `src/hardware/power.rs`
- **fallback_scheme_name()** (6 connections) — `src/hardware/power.rs`
- **format_guid()** (6 connections) — `src/hardware/power.rs`
- **GUID** (5 connections)
- **set_active_power_scheme()** (5 connections) — `src/hardware/power.rs`
- **test_ac_power_status_and_enumerated_range_noop()** (5 connections) — `src/hardware/power.rs`
- **test_parking_snapshot_roundtrip_and_active_rail_logic()** (4 connections) — `src/hardware/parking.rs`
- **LocalGuidPtr** (4 connections) — `src/hardware/power.rs`
- **test_guid_roundtrip_and_normalization()** (4 connections) — `src/hardware/power.rs`
- *... and 52 more nodes in this community*

## Relationships

- [full_telemetry_audit_with](full_telemetry_audit_with.md) (7 shared connections)
- [deserialize](deserialize.md) (6 shared connections)
- [HibernationStatus](HibernationStatus.md) (6 shared connections)
- [backend.rs](backend.rs.md) (4 shared connections)
- [registry_scanner.rs](registry_scanner.rs.md) (4 shared connections)
- [.Reg State() Vendor()](Reg_State_Vendor.md) (3 shared connections)
- [Integration Suite: Tweak Sandbox E2E](Integration_Suite-_Tweak_Sandbox_E2E.md) (2 shared connections)
- [sys_inventory.rs](sys_inventory.rs.md) (2 shared connections)
- [theme.rs](theme.rs.md) (2 shared connections)
- [SandboxEnv](SandboxEnv.md) (2 shared connections)
- [.take_injected_error](take_injected_error.md) (2 shared connections)
- [PanickingBackend](PanickingBackend.md) (2 shared connections)

## Source Files

- `src/core/backend.rs`
- `src/hardware/parking.rs`
- `src/hardware/power.rs`
- `tests/tweak_sandbox_e2e.rs`

## Audit Trail

- EXTRACTED: 189 (94%)
- INFERRED: 12 (6%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*