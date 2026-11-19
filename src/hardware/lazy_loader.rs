//! Non-blocking background lazy-loading subsystem for WinBlast0 hardware views.
//!
//! Offloads slow SMBIOS, driver update, thermal sensor, and disk health probes
//! onto named worker threads, deduplicates in-flight probes per subsystem,
//! catches panics via `std::panic::catch_unwind`, and delivers payloads over a
//! non-blocking crossbeam channel polled every UI frame.

use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::hardware::bios::BiosSystemInfo;
use crate::hardware::disks::DiskSubsystemReport;
use crate::hardware::driver_update::DriverUpdateItem;
use crate::hardware::thermal::SystemThermalSnapshot;

/// Payload returned asynchronously by background hardware probes.
#[derive(Debug)]
pub enum LazyPayload {
    Bios(Box<BiosSystemInfo>),
    Drivers(Vec<DriverUpdateItem>),
    Thermals(Box<SystemThermalSnapshot>),
    Disks(Box<DiskSubsystemReport>),
}

/// Thread-safe background probe manager polled non-blockingly by `AppState::poll()`.
pub struct LazyLoader {
    tx: Sender<LazyPayload>,
    rx: Receiver<LazyPayload>,
    bios_in_flight: Arc<AtomicBool>,
    drivers_in_flight: Arc<AtomicBool>,
    thermals_in_flight: Arc<AtomicBool>,
    disks_in_flight: Arc<AtomicBool>,
}

impl Default for LazyLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl LazyLoader {
    /// Creates a new `LazyLoader` with an unbounded crossbeam channel.
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            tx,
            rx,
            bios_in_flight: Arc::new(AtomicBool::new(false)),
            drivers_in_flight: Arc::new(AtomicBool::new(false)),
            thermals_in_flight: Arc::new(AtomicBool::new(false)),
            disks_in_flight: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Spawns a non-blocking background probe for BIOS / SMBIOS system information.
    /// No-ops if a BIOS probe is already in flight.
    pub fn spawn_probe_bios(&self) {
        self.spawn_probe("wb0-probe-bios", &self.bios_in_flight, || {
            let info = BiosSystemInfo::probe();
            LazyPayload::Bios(Box::new(info))
        });
    }

    /// Spawns a non-blocking background probe for driver update inventory.
    /// No-ops if a driver probe is already in flight.
    pub fn spawn_probe_drivers(&self) {
        self.spawn_probe("wb0-probe-drivers", &self.drivers_in_flight, || {
            let items = crate::hardware::driver_update::scan_online_drivers();
            LazyPayload::Drivers(items)
        });
    }

    /// Spawns a non-blocking background probe for thermal sensors and fan telemetry.
    /// No-ops if a thermal probe is already in flight.
    pub fn spawn_probe_thermals(&self) {
        self.spawn_probe("wb0-probe-thermals", &self.thermals_in_flight, || {
            let snapshot = SystemThermalSnapshot::read();
            LazyPayload::Thermals(Box::new(snapshot))
        });
    }

    /// Spawns a non-blocking background probe for disk health and SMART telemetry.
    /// No-ops if a disk probe is already in flight.
    pub fn spawn_probe_disks(&self) {
        self.spawn_probe("wb0-probe-disks", &self.disks_in_flight, || {
            let report = DiskSubsystemReport::probe();
            LazyPayload::Disks(Box::new(report))
        });
    }

    /// Non-blocking single-item poll intended to be called every frame in `AppState::poll()`.
    pub fn try_recv(&self) -> Option<LazyPayload> {
        self.rx.try_recv().ok()
    }

    /// Drains all currently queued payloads without blocking the UI thread.
    pub fn drain_ready(&self) -> Vec<LazyPayload> {
        let mut ready = Vec::new();
        while let Ok(payload) = self.rx.try_recv() {
            ready.push(payload);
        }
        ready
    }

    /// Returns `true` if a BIOS probe is currently executing.
    pub fn is_bios_loading(&self) -> bool {
        self.bios_in_flight.load(Ordering::Acquire)
    }

    /// Returns `true` if a driver update probe is currently executing.
    pub fn is_drivers_loading(&self) -> bool {
        self.drivers_in_flight.load(Ordering::Acquire)
    }

    /// Returns `true` if a thermal/fan probe is currently executing.
    pub fn is_thermals_loading(&self) -> bool {
        self.thermals_in_flight.load(Ordering::Acquire)
    }

    /// Returns `true` if a disk health probe is currently executing.
    pub fn is_disks_loading(&self) -> bool {
        self.disks_in_flight.load(Ordering::Acquire)
    }

    /// Returns `true` if any background hardware probe is currently in flight.
    pub fn is_any_loading(&self) -> bool {
        self.is_bios_loading()
            || self.is_drivers_loading()
            || self.is_thermals_loading()
            || self.is_disks_loading()
    }

    /// Core worker spawner: deduplicates concurrent runs via `in_flight`,
    /// protects against OS thread spawn failure, and isolates panics with `catch_unwind`.
    fn spawn_probe<F>(&self, thread_name: &str, in_flight: &Arc<AtomicBool>, probe: F)
    where
        F: FnOnce() -> LazyPayload + Send + 'static,
    {
        if in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        let guard = InFlightGuard {
            flag: Arc::clone(in_flight),
        };
        let tx = self.tx.clone();
        let label = thread_name.to_string();

        let spawn_result = thread::Builder::new().name(label.clone()).spawn(move || {
            let _guard = guard;

            match panic::catch_unwind(AssertUnwindSafe(probe)) {
                Ok(payload) => {
                    let _ = tx.send(payload);
                }
                Err(panic_payload) => {
                    let detail = extract_panic_message(&panic_payload);
                    eprintln!(
                        "[WinBlast0][LazyLoader] Background probe '{}' panicked: {}",
                        label, detail
                    );
                }
            }
        });

        if let Err(err) = spawn_result {
            eprintln!(
                "[WinBlast0][LazyLoader] Failed to spawn probe thread '{}': {}",
                thread_name, err
            );
        }
    }
}

/// RAII guard ensuring an in-flight flag is always cleared when a probe exits,
/// panics, or fails to spawn.
struct InFlightGuard {
    flag: Arc<AtomicBool>,
}

impl Drop for InFlightGuard {
    fn drop(&mut self) {
        self.flag.store(false, Ordering::Release);
    }
}

fn extract_panic_message(payload: &(dyn Any + Send)) -> &str {
    if let Some(msg) = payload.downcast_ref::<&str>() {
        msg
    } else if let Some(msg) = payload.downcast_ref::<String>() {
        msg.as_str()
    } else {
        "non-string panic payload"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn lazy_loader_is_send_and_sync() {
        assert_send_sync::<LazyLoader>();
    }

    #[test]
    fn try_recv_is_non_blocking_when_empty() {
        let loader = LazyLoader::new();
        assert!(loader.try_recv().is_none());
        assert!(!loader.is_any_loading());
    }

    #[test]
    fn background_panic_is_contained_and_clears_in_flight_flag() {
        let loader = LazyLoader::new();
        let flag = Arc::clone(&loader.drivers_in_flight);

        loader.spawn_probe("test-panic-probe", &flag, || -> LazyPayload {
            panic!("simulated WMI driver failure");
        });

        let deadline = Instant::now() + Duration::from_secs(2);
        while loader.is_drivers_loading() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(5));
        }

        assert!(!loader.is_drivers_loading());
        assert!(loader.try_recv().is_none());
    }

    #[test]
    fn background_probe_delivers_payload_without_blocking() {
        let loader = LazyLoader::new();
        let flag = Arc::clone(&loader.drivers_in_flight);

        loader.spawn_probe("test-drivers-probe", &flag, || {
            LazyPayload::Drivers(Vec::new())
        });

        let deadline = Instant::now() + Duration::from_secs(2);
        let mut received = None;
        while Instant::now() < deadline {
            if let Some(payload) = loader.try_recv() {
                received = Some(payload);
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }

        assert!(matches!(received, Some(LazyPayload::Drivers(items)) if items.is_empty()));
        assert!(!loader.is_drivers_loading());
    }
}
