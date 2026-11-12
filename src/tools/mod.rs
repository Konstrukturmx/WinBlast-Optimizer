//! Advanced system utilities ported cleanly in pure Rust:
//! disk cleanup, 4-source startup/service/task management, AppX & DISM feature control,
//! non-elevated Explorer restart, and setup-safe `autounattend.xml` generation.

pub mod autounattend;
pub mod cleaner;
pub mod explorer_ctrl;
pub mod features_apps;
pub mod startup;
