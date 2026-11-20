//! Backup and rollback subsystem.
//!
//! Provides System Restore point creation, SQLite-backed state ledger for
//! granular undo, and profile import/export.

pub mod restore_point;
pub mod state_ledger;
pub mod profile_io;
pub mod preset_sharing;
