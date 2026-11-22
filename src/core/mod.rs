//! Core infrastructure modules for WinBlast0.
//!
//! Re-exports backend abstraction, error handling, OS detection, elevation checks, and the risk/tweak model.

pub mod backend;
pub mod commands;
pub mod elevation;
pub mod error;
pub mod events;
pub mod generational_cache;
pub mod i18n;
pub mod os_detect;
pub mod process;
pub mod reg_helpers;
pub mod risk;
pub mod status_i18n;
pub mod tweak_i18n;
pub mod updater;
pub mod view_i18n;
pub mod theme;
pub mod window_icon;
pub mod settings;
