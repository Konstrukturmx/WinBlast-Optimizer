//! Advanced system utilities ported cleanly in pure Rust:
//! disk cleanup, 4-source startup/service/task management, AppX & DISM feature control,
//! non-elevated Explorer restart, and setup-safe `autounattend.xml` generation.

pub mod automation;
pub mod autounattend;
pub mod boot_config;
pub mod cleaner;
pub mod diagnostics;
pub mod explorer_ctrl;
pub mod features_apps;
pub mod game_overlay;
pub mod game_prereqs;
pub mod game_profiles;
pub mod gaming_stats;
pub mod heuristic_scanner;
pub mod network_diag;
pub mod performance_tests;
pub mod startup;
pub mod windows_repair;
