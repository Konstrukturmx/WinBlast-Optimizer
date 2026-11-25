//! Hardware detection subsystem.
//!
//! Detects CPU topology, GPU capabilities, and captures latency snapshots.

pub mod bios;
pub mod cpu;
pub mod disks;
pub mod driver_update;
pub mod gpu;
pub mod latency;
pub mod lazy_loader;
pub mod monitor;
pub mod net_latency;
pub mod parking;
pub mod power;
pub mod power_governor;
pub mod storage;
pub mod tbw;
pub mod sys_inventory;
pub mod thermal;
