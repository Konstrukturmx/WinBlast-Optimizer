# Project Documentation, Architecture & Licensing

> 19 nodes · cohesion 0.13

## Key Concepts

- **SilentCommand** (13 connections) — `src/core/process.rs`
- **tools/network_diag.rs** (12 connections) — `src/tools/network_diag.rs`
- **NetworkDiagnostics** (8 connections) — `src/tools/network_diag.rs`
- **duration** (6 connections)
- **Command** (3 connections)
- **process.rs** (3 connections) — `src/core/process.rs`
- **.new()** (3 connections) — `src/core/process.rs`
- **.default()** (3 connections) — `src/tools/network_diag.rs`
- **DnsProbeResult** (2 connections) — `src/tools/network_diag.rs`
- **.probe_live()** (2 connections) — `src/tools/network_diag.rs`
- **query_local_ip()** (2 connections) — `src/tools/network_diag.rs`
- **tcpstream** (2 connections)
- **osstr** (1 connections)
- **S** (1 connections)
- **DNS_CATALOG** (1 connections) — `src/tools/network_diag.rs`
- **.apply_dns_servers()** (1 connections) — `src/tools/network_diag.rs`
- **.flush_dns()** (1 connections) — `src/tools/network_diag.rs`
- **.reset_dns_dhcp()** (1 connections) — `src/tools/network_diag.rs`
- **udpsocket** (1 connections)

## Relationships

- [Reversibility Instant Requireslogoff](Reversibility_Instant_Requireslogoff.md) (4 shared connections)
- [DriverUpdateItem](DriverUpdateItem.md) (3 shared connections)
- [backend.rs](backend.rs.md) (1 shared connections)
- [AppState](AppState.md) (1 shared connections)
- [Hardware Diagnostics & GPU Silicon Tuning](Hardware_Diagnostics_&_GPU_Silicon_Tuning.md) (1 shared connections)
- [AppX Package & Modern App Scanner](AppX_Package_&_Modern_App_Scanner.md) (1 shared connections)
- [theme.rs](theme.rs.md) (1 shared connections)
- [Appx Package() Prefixes](Appx_Package_Prefixes.md) (1 shared connections)
- [Multi-Language Tweak Localization Tables](Multi-Language_Tweak_Localization_Tables.md) (1 shared connections)
- [RegRawSnapshot](RegRawSnapshot.md) (1 shared connections)
- [deserialize](deserialize.md) (1 shared connections)
- [.Reg .Set Power](Reg_.Set_Power.md) (1 shared connections)

## Source Files

- `src/core/process.rs`
- `src/tools/network_diag.rs`

## Audit Trail

- EXTRACTED: 43 (98%)
- INFERRED: 1 (2%)
- AMBIGUOUS: 0 (0%)

---

*Part of the graphify knowledge wiki. See [index](index.md) to navigate.*