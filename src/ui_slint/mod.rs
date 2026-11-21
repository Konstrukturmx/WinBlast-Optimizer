//! WinBlast0 Slint 1.19 Presentation Layer.
//!
//! Exposes the compiled Slint UI components, bridge orchestration,
//! and native desktop event loop.

#[cfg(feature = "ui-slint")]
slint::include_modules!();

#[cfg(feature = "ui-slint")]
pub mod bridge;

#[cfg(feature = "ui-slint")]
pub use MainWindow as SlintMainWindow;
