//! Hardware detection subsystem.
//!
//! Detects CPU topology, GPU capabilities, and captures latency snapshots.

pub mod cpu;
pub mod gpu;
pub mod latency;
pub mod monitor;
pub mod net_latency;
pub mod parking;
pub mod power;
pub mod sys_inventory;
