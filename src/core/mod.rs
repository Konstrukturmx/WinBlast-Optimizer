//! Core infrastructure modules for WinBlast0.
//!
//! Re-exports backend abstraction, error handling, OS detection, elevation checks, and the risk/tweak model.

pub mod backend;
pub mod error;
pub mod os_detect;
pub mod elevation;
pub mod risk;
pub mod process;
pub mod reg_helpers;
pub mod i18n;
pub mod tweak_i18n;
pub mod status_i18n;
