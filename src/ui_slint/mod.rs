//! WinBlast0 Slint 1.19 presentation layer.
//!
//! Architecture: every screen area owns a Slint `global` store (see `ui/stores/*.slint`) and a
//! Rust module here that (a) binds the store's callbacks to `AppState` actions and (b) syncs the
//! store's properties/models from `AppState` on every UI tick. Callbacks only mutate state and
//! request a sync; they never touch Slint while holding the `AppState` borrow, which rules out
//! RefCell double-borrow panics from re-entrant Slint callbacks.

slint::include_modules!();

pub mod bridge;
pub mod ctx;
pub mod diff;
pub mod i18n;
pub mod models;
pub mod palette;
pub mod shell;
pub mod theme;
pub mod views;

pub use MainWindow as SlintMainWindow;
