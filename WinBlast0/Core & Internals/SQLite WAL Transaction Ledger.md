---
title: SQLite WAL Transaction Ledger Internals
tags: [core, sqlite, wal, concurrency, ledger, database]
created: 2026-11-08
---

# 🗄️ SQLite WAL Transaction Ledger Internals

Located in `src/backup/state_ledger.rs`, the SQLite state ledger provides a local, self-contained, transactional audit log that records every mutation performed by the application and stores performance baselines.

---

## 🏛️ Configuration for Multi-Threaded Stability

In v0.2.0, the ledger database initialization was fortified against `database is locked (busy)` race conditions across background workers:

```rust
let conn = Connection::open(db_path)?;

// Configure 5-second busy timeout for concurrent transactions
conn.busy_timeout(std::time::Duration::from_secs(5))?;

// Enable Write-Ahead Logging (WAL) and normal synchronous mode
conn.execute_batch(
    "PRAGMA journal_mode = WAL;
     PRAGMA synchronous = NORMAL;
     CREATE TABLE IF NOT EXISTS ledger (...);
     CREATE TABLE IF NOT EXISTS baselines (...);"
)?;
```

### Why WAL (Write-Ahead Logging)?
- Traditional rollback journal locks the entire database file for both readers and writers simultaneously.
- In **WAL mode**, readers never block writers, and writers never block readers. Background scanning threads and monitoring workers can freely query baseline history while an executor worker writes mutation logs.

---

## 📊 Database Tables

### 1. `ledger` Table
Stores granular change records:
- `tweak_id`: The identifier of the applied tweak (e.g. `"win32_priority_separation"`).
- `action_type`: Type of action (`"RegistryDword"`, `"ServiceState"`, `"ScheduledTask"`).
- `original_value`: Exact serialized system state captured immediately before mutation.
- `new_value`: Value written to the system.
- `timestamp`: ISO-8601 UTC timestamp.
- `rolled_back`: 1 if undone, 0 if active.

### 2. `baselines` Table
Stores performance snapshots:
- `captured_at`: Timestamp.
- `total_ram_mb`: Total physical RAM.
- `used_ram_mb`: Memory in use.
- `process_count`: Total active processes.
- `thread_count`: Total active threads.
- `timer_resolution_ns`: Current kernel timer resolution.
- `optimized_count`: Number of active tweaks.
- `label`: Description (e.g. `"Initial Pre-Optimization"`, `"Post-Optimization"`).

---

## 🔗 Related Notes
- [[State Ledger & Rollback Engine]]
- [[Debugging & Edge Cases Resolved]]
- [[00 - Index (MOC)]]
