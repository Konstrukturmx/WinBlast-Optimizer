//! High-performance zero-allocation UI synchronization primitives,
//! filter-strict selection state machine, and monotonic HUD progress.

pub mod bitops;
pub mod hud_progress;
pub mod selection;

#[cfg(test)]
pub mod tests;
