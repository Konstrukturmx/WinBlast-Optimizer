//! Driver & BIOS binder: online driver catalog (category chips, folded search, WHQL, backup,
//! install), motherboard / firmware / Secure Boot and display-driver inventory.

use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::i18n::{I18n, TextKey};
use crate::hardware::driver_update::{DriverCategory, DriverUpdateState};

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{DisplayDriverRow, DriverRow, DriverStore, MainWindow};

pub const FILTER_CATEGORIES: [DriverCategory; 7] = [
    DriverCategory::Display,
    DriverCategory::Network,
    DriverCategory::Audio,
    DriverCategory::Storage,
    DriverCategory::Chipset,
    DriverCategory::Usb,
    DriverCategory::Bluetooth,
];

pub fn category_from_id(id: &str) -> Option<DriverCategory> {
    FILTER_CATEGORIES.iter().copied().find(|c| format!("{c:?}") == id)
}

pub struct DriversView {
    drivers: Rc<VecModel<DriverRow>>,
    display: Rc<VecModel<DisplayDriverRow>>,
}

impl DriversView {
    pub fn new(window: &MainWindow) -> Self {
        let st = window.global::<DriverStore>();
        let (drivers, a) = new_model();
        let (display, b) = new_model();
        st.set_drivers(a);
        st.set_display_drivers(b);
        Self { drivers, display }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let st = window.global::<DriverStore>();
        let c = ctx.clone();
        st.on_scan_online(move || {
            c.act(|s| s.scan_online_drivers());
        });
        let c = ctx.clone();
        st.on_test_sandbox_install(move || {
            c.act(|s| s.test_sandbox_driver_install());
        });
        let c = ctx.clone();
        st.on_rescan_bios(move || {
            c.act(|s| s.scan_bios_info_async());
        });
        let c = ctx.clone();
        st.on_search_edited(move |t| {
            c.act(|s| s.driver_search_edit.text = t.to_string());
        });
        let c = ctx.clone();
        st.on_set_category(move |id| {
            c.act(|s| s.driver_category_filter = category_from_id(id.as_str()));
        });
        let c = ctx.clone();
        st.on_install(move |id| {
            c.act(|s| s.install_driver_update(id.to_string()));
        });
        let c = ctx.clone();
        st.on_backup(move |id| {
            c.act(|s| s.backup_driver(id.to_string()));
        });
    }

    pub fn sync(&self, window: &MainWindow, s: &AppState) {
        let st = window.global::<DriverStore>();
        st.set_scanning(s.is_driver_scanning);
        st.set_bios_scanning(s.is_bios_scanning);
        st.set_any_in_flight(s.driver_install_in_flight.is_some());
        if st.get_search().as_str() != s.driver_search_edit.text {
            st.set_search(s.driver_search_edit.text.as_str().into());
        }
        st.set_category(s.driver_category_filter.map(|c| format!("{c:?}")).unwrap_or_default().into());
        st.set_total(s.driver_updates.len() as i32);
        st.set_devices_scanned(format!("{} Devices Scanned", s.driver_updates.len()).into());

        let q = s.driver_search_edit.text.trim();
        let q = if q.is_empty() { String::new() } else { I18n::fold_search_str(q) };
        sync_vec(
            &self.drivers,
            s.driver_updates
                .iter()
                .filter(|d| s.driver_category_filter.is_none_or(|c| d.category == c))
                .filter(|d| {
                    q.is_empty()
                        || [
                            d.name.as_str(),
                            d.provider.as_str(),
                            d.hardware_id.as_str(),
                            d.category.display_name(),
                            d.current_version.as_str(),
                            d.latest_version.as_str(),
                        ]
                        .iter()
                        .any(|f| I18n::fold_search_str(f).contains(&q))
                })
                .map(|d| {
                    let (badge, tone): (&str, &str) = match d.state {
                        DriverUpdateState::Installed => (s.tr(TextKey::DriverInstalledSuccess), "success"),
                        DriverUpdateState::UpdateAvailable => (s.tr(TextKey::DriverUpdateAvailable), "warning"),
                        DriverUpdateState::UpToDate => (s.tr(TextKey::DriverUpToDate), "muted"),
                        DriverUpdateState::Downloading => (s.tr(TextKey::DriverDownloading), "accent"),
                        DriverUpdateState::Installing => (s.tr(TextKey::DriverInstalling), "accent"),
                        DriverUpdateState::Failed => (s.tr(TextKey::PrereqStateFailed), "danger"),
                        DriverUpdateState::ReadyToInstall => (s.tr(TextKey::DriverReadyToInstall), "accent"),
                    };
                    let mut details = format!("{} • {}: {}", d.category.display_name(), s.tr(TextKey::DriverProviderLabel), d.provider);
                    if !d.hardware_id.is_empty() {
                        details.push_str(&format!(" • HW: {}", d.hardware_id));
                    }
                    if !d.status_message.is_empty() {
                        details.push_str(&format!(" • {}", d.status_message));
                    }
                    DriverRow {
                        id: d.id.as_str().into(),
                        icon: d.category.icon().into(),
                        name: d.name.as_str().into(),
                        details: details.into(),
                        versions: format!("{}: {} → {}: {}", s.tr(TextKey::DriverCurrentLabel), d.current_version, s.tr(TextKey::DriverLatestLabel), d.latest_version).into(),
                        date: if d.download_size_mb > 0.0 {
                            format!("{}: {} ({:.1} MB)", s.tr(TextKey::DriverDateLabel), d.release_date, d.download_size_mb).into()
                        } else {
                            format!("{}: {}", s.tr(TextKey::DriverDateLabel), d.release_date).into()
                        },
                        whql: d.is_whql_signed,
                        badge: badge.into(),
                        badge_tone: tone.into(),
                        update: d.is_update_available(),
                        busy: s.driver_install_in_flight.as_deref() == Some(d.id.as_str()),
                    }
                })
                .collect(),
        );

        let b = s.bios_info.clone().unwrap_or_default();
        let detecting = s.tr(TextKey::ValueDetecting);
        let have_board = !b.motherboard_vendor.is_empty() || !b.motherboard_product.is_empty();
        st.set_board_title(if have_board { format!("{} {}", b.motherboard_vendor, b.motherboard_product).into() } else { detecting.into() });
        st.set_board_sub(if have_board { format!("Rev: {} • SMBIOS {}", b.motherboard_version, b.smbios_version).into() } else { SharedString::new() });
        let have_fw = !b.bios_vendor.is_empty() || !b.bios_version.is_empty();
        st.set_firmware_title(if have_fw { format!("{} ver. {}", b.bios_vendor, b.bios_version).into() } else { detecting.into() });
        st.set_firmware_sub(if have_fw {
            format!("{}: {} • {}: {}", s.tr(TextKey::BiosReleaseDateLabel), b.bios_release_date, s.tr(TextKey::BiosUefiBootLabel), if b.uefi_boot { s.tr(TextKey::BiosBootActive) } else { s.tr(TextKey::BiosBootLegacy) }).into()
        } else {
            SharedString::new()
        });
        st.set_secure_boot(b.secure_boot);
        sync_vec(
            &self.display,
            b.display_drivers
                .iter()
                .map(|d| DisplayDriverRow {
                    name: d.gpu_name.as_str().into(),
                    provider: format!("Provider: {} • INF: {}", d.provider, d.inf_path).into(),
                    version: format!("Version: {}", d.driver_version).into(),
                    date: format!("Date: {}", d.driver_date).into(),
                })
                .collect(),
        );
    }
}
