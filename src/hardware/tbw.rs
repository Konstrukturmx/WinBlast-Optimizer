//! Lifetime host-write (TBW) decoding for NVMe and SATA drives.
//!
//! * NVMe: SMART / Health Information log page 0x02, "Data Units Written" (bytes 48..64, u128 LE).
//!   One data unit is 1000 × 512 bytes (NVMe Base Specification, Get Log Page 0x02).
//! * SATA: there is no standard unit for SMART attributes 0xF1 / 0xF6. Each vendor picks its own
//!   (512-byte LBAs, 32 MiB blocks, GiB, decimal GB), so the counter is decoded only for drive
//!   families listed in [`SATA_WRITE_RULES`], keyed on the ATA IDENTIFY DEVICE model string (and
//!   firmware where one model ships with different controllers). Every other drive reports
//!   [`HostWrites::NotAvailable`] instead of a guessed number (AGENTS.md rule 2).
//!
//! The SATA table follows the attribute names in smartmontools' `drivedb.h`
//! (e.g. `241,raw48,Lifetime_Writes_GiB`, `241,raw48,Host_Writes_32MiB`).

use serde::{Deserialize, Serialize};

/// Lifetime host writes as reported by the drive, or why there is no value.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum HostWrites {
    /// Decimal terabytes (10^12 bytes) written by the host over the drive's life.
    Reported(f64),
    /// No decodable host-write counter: HDD, virtual disk, unknown SATA vendor unit, or the
    /// drive returned no SMART data.
    #[default]
    NotAvailable,
    /// The SMART query was refused with ERROR_ACCESS_DENIED (process not elevated).
    RequiresAdmin,
}

impl HostWrites {
    /// Terabytes when the drive reported a value.
    pub fn terabytes(self) -> Option<f64> {
        match self {
            HostWrites::Reported(tb) => Some(tb),
            _ => None,
        }
    }
}

/// Size of one raw count of a SATA host-write attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteUnit {
    /// 512-byte logical blocks.
    Lba512,
    /// 32 MiB blocks (Intel, Plextor).
    MiB32,
    /// Binary gigabytes, 2^30 bytes (Phison, SanDisk, WD, Kingston UV).
    GiB,
    /// Decimal gigabytes, 10^9 bytes (SK hynix client drives).
    Gb,
}

impl WriteUnit {
    pub const fn bytes(self) -> u64 {
        match self {
            WriteUnit::Lba512 => 512,
            WriteUnit::MiB32 => 32 * 1024 * 1024,
            WriteUnit::GiB => 1024 * 1024 * 1024,
            WriteUnit::Gb => 1_000_000_000,
        }
    }
}

/// Which SMART attribute carries host writes for a drive family, and in which unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SataWriteCounter {
    pub attribute: u8,
    pub unit: WriteUnit,
}

/// One drive family. A model matches when it starts with one of `prefixes`, contains one of
/// `contains_any` (if that list is non-empty) and, when `phison_firmware_only` is set, its
/// firmware revision has the Phison layout (the same model also ships with other controllers
/// whose counters use a different unit).
pub struct SataWriteRule {
    pub family: &'static str,
    pub prefixes: &'static [&'static str],
    pub contains_any: &'static [&'static str],
    pub phison_firmware_only: bool,
    pub counter: SataWriteCounter,
}

const F1: u8 = 0xF1;
const F6: u8 = 0xF6;

/// Drive families with a documented host-write unit (smartmontools drivedb.h family names).
/// Model strings are compared upper-case with runs of spaces collapsed.
pub static SATA_WRITE_RULES: &[SataWriteRule] = &[
    SataWriteRule {
        family: "Samsung based SSDs",
        prefixes: &["SAMSUNG SSD", "SAMSUNG MZ7", "MZ7"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::Lba512 },
    },
    SataWriteRule {
        family: "Crucial/Micron Client SSDs",
        prefixes: &["CRUCIAL_CT", "CT"],
        contains_any: &["MX100", "MX200", "MX300", "MX500", "M500", "M550", "BX300", "BX500"],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F6, unit: WriteUnit::Lba512 },
    },
    SataWriteRule {
        family: "Crucial/Micron Client SSDs (Micron OEM)",
        prefixes: &["MICRON_M5", "MICRON_M6", "MICRON_1100", "MICRON_1300", "MICRON 1100", "MICRON M500IT"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F6, unit: WriteUnit::Lba512 },
    },
    SataWriteRule {
        family: "Micron 5100 / 52x0 / 5300 / 5400 SSDs",
        prefixes: &["MICRON_5100", "MICRON_5200", "MICRON_5210", "MICRON_5300", "MICRON_5400"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F6, unit: WriteUnit::Lba512 },
    },
    SataWriteRule {
        family: "Intel / Solidigm SATA SSDs",
        prefixes: &["INTEL SSDSA", "INTEL SSDSC", "INTEL SSDMCE", "SOLIDIGM SSDSC"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::MiB32 },
    },
    SataWriteRule {
        family: "Plextor M3/M5/M6/M7 Series SSDs",
        prefixes: &["PLEXTOR PX-"],
        contains_any: &["M3", "M5", "M6", "M7VC", "M8VC"],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::MiB32 },
    },
    SataWriteRule {
        family: "Kingston SSDNow UV400/500",
        prefixes: &["KINGSTON SUV400", "KINGSTON SUV500"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::GiB },
    },
    SataWriteRule {
        family: "Phison Driven SSDs",
        prefixes: &[
            "GOODRAM IRIDIUM PRO",
            "IR-SSDPR-S25",
            "IRP-SSDPR-S25",
            "KINGSTON SUV300",
            "KINGSTON SKC400",
            "KINGSTON SHSS",
            "PATRIOT BURST",
            "TEAM L3 EVO SSD",
        ],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::GiB },
    },
    SataWriteRule {
        // Kingston A400 and Goodram CX400 also ship with Silicon Motion / other controllers
        // (smartmontools tickets #801 and #1689), whose 0xF1 is not in GiB.
        family: "Phison Driven SSDs (firmware-gated)",
        prefixes: &["KINGSTON SA400", "SSDPR-CX400-"],
        contains_any: &[],
        phison_firmware_only: true,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::GiB },
    },
    SataWriteRule {
        family: "Marvell based SanDisk SSDs",
        prefixes: &[
            "SANDISK SD5SG2",
            "SANDISK SD6S",
            "SANDISK SD7S",
            "SANDISK SD8S",
            "SANDISK SD8T",
            "SANDISK SD9S",
            "SANDISK SDSSDA",
            "SANDISK SDSSDH",
            "SANDISK SDSSDXP",
            "SANDISK ULTRA II",
            "SANDISK SSD PLUS",
        ],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::GiB },
    },
    SataWriteRule {
        family: "WD Blue / Red / Green SSDs",
        prefixes: &["WDC WDS", "WDC WDBNCE", "WD BLUE SA510"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::GiB },
    },
    SataWriteRule {
        family: "SK hynix SATA SSDs",
        prefixes: &["SK HYNIX SC2", "SK HYNIX SC3", "SK HYNIX SC4", "SK HYNIX SH920", "SKHYNIX SC"],
        contains_any: &[],
        phison_firmware_only: false,
        counter: SataWriteCounter { attribute: F1, unit: WriteUnit::Gb },
    },
];

/// Upper-case and collapse whitespace runs ("KINGSTON  SA400S37240G" -> "KINGSTON SA400S37240G").
fn normalize_model(model: &str) -> String {
    model.split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_uppercase()
}

/// Phison SATA firmware revisions look like `SBFM61.3`, `SAFM01.5`, `S9FM02.6`, `SBFK10D7`.
pub fn is_phison_firmware(firmware: &str) -> bool {
    let b = firmware.trim().as_bytes();
    b.len() >= 4
        && b[0] == b'S'
        && matches!(b[1], b'8' | b'9' | b'A' | b'B' | b'H')
        && b[2] == b'F'
        && matches!(b[3], b'D' | b'K' | b'M')
}

/// Look up the host-write attribute and unit for a SATA model string from ATA IDENTIFY DEVICE.
pub fn lookup_sata_write_counter(model: &str, firmware: &str) -> Option<SataWriteCounter> {
    let m = normalize_model(model);
    SATA_WRITE_RULES
        .iter()
        .find(|r| {
            r.prefixes.iter().any(|p| m.starts_with(p))
                && (r.contains_any.is_empty() || r.contains_any.iter().any(|c| m.contains(c)))
                && (!r.phison_firmware_only || is_phison_firmware(firmware))
        })
        .map(|r| r.counter)
}

/// Decimal terabytes from a raw SATA attribute count.
pub fn sata_raw_to_tb(raw: u64, unit: WriteUnit) -> f64 {
    (raw as f64 * unit.bytes() as f64) / 1e12
}

/// NVMe Data Units Written (log page 0x02, bytes 48..64) in decimal terabytes.
pub fn nvme_data_units_written_tb(log: &[u8]) -> Option<f64> {
    let bytes: [u8; 16] = log.get(48..64)?.try_into().ok()?;
    let units = u128::from_le_bytes(bytes);
    Some(units as f64 * 512_000.0 / 1e12)
}

/// Model strings of virtual disks, which have no physical write counter.
pub fn is_virtual_disk_model(model: &str) -> bool {
    let m = model.to_ascii_uppercase();
    ["VIRTUAL DISK", "VIRTUAL HD", "VBOX", "VMWARE", "QEMU", "XEN ", "RED HAT VIRTIO", "MSFT VIRTUAL"]
        .iter()
        .any(|k| m.contains(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_sizes() {
        assert_eq!(WriteUnit::Lba512.bytes(), 512);
        assert_eq!(WriteUnit::MiB32.bytes(), 33_554_432);
        assert_eq!(WriteUnit::GiB.bytes(), 1_073_741_824);
        assert_eq!(WriteUnit::Gb.bytes(), 1_000_000_000);
    }

    #[test]
    fn known_families_resolve() {
        let c = |m: &str, fw: &str| lookup_sata_write_counter(m, fw);
        assert_eq!(c("Samsung SSD 870 EVO 1TB", "SVT02B6Q"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::Lba512 }));
        assert_eq!(c("CT1000MX500SSD1", "M3CR046"), Some(SataWriteCounter { attribute: 0xF6, unit: WriteUnit::Lba512 }));
        assert_eq!(c("Crucial_CT525MX300SSD1", "M0CR070"), Some(SataWriteCounter { attribute: 0xF6, unit: WriteUnit::Lba512 }));
        assert_eq!(c("INTEL SSDSC2KW256G8", "LHF002C"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::MiB32 }));
        assert_eq!(c("WDC WDS500G2B0A-00SM50", "401000WD"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::GiB }));
        assert_eq!(c("SanDisk SDSSDH3 1T00", "415020RL"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::GiB }));
        assert_eq!(c("SK hynix SC311 SATA 256GB", "70000P10"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::Gb }));
        // Two spaces in the model string, as some A400s report it.
        assert_eq!(c("KINGSTON  SA400S37480G", "SBFK10D7"), Some(SataWriteCounter { attribute: 0xF1, unit: WriteUnit::GiB }));
    }

    #[test]
    fn controller_variants_without_phison_firmware_are_not_guessed() {
        // Goodram CX400 G2 with non-Phison firmware (this dev PC's drive; smartmontools #1689).
        assert_eq!(lookup_sata_write_counter("SSDPR-CX400-01T-G2", "SN20728"), None);
        assert!(lookup_sata_write_counter("SSDPR-CX400-512-G2", "SBFMLA.5").is_some());
        // Kingston A400 with a Silicon Motion controller.
        assert_eq!(lookup_sata_write_counter("KINGSTON SA400S37240G", "S1Z40102"), None);
    }

    #[test]
    fn crucial_bx100_bx200_are_not_matched() {
        // Silicon Motion based, different unit; not in the Crucial/Micron client family.
        assert_eq!(lookup_sata_write_counter("CT250BX100SSD1", "MU03"), None);
        assert_eq!(lookup_sata_write_counter("CT480BX200SSD1", "MU02.6"), None);
    }

    #[test]
    fn unknown_and_hdd_models_are_none() {
        assert_eq!(lookup_sata_write_counter("ST2000DM008-2FR102", "0001"), None);
        assert_eq!(lookup_sata_write_counter("", ""), None);
        assert_eq!(lookup_sata_write_counter("Generic SSD 256GB", "V1.0"), None);
    }

    #[test]
    fn raw_conversion() {
        // 15,625,000,000 LBAs × 512 B = 8 TB.
        assert!((sata_raw_to_tb(15_625_000_000, WriteUnit::Lba512) - 8.0).abs() < 1e-9);
        // 1000 GiB = 1.073741824 TB.
        assert!((sata_raw_to_tb(1000, WriteUnit::GiB) - 1.073_741_824).abs() < 1e-12);
        // 0x0000_FFFF_FFFF_FFFF raw48 maximum in 32 MiB blocks stays finite.
        assert!(sata_raw_to_tb(0xFFFF_FFFF_FFFF, WriteUnit::MiB32).is_finite());
    }

    #[test]
    fn nvme_units() {
        let mut log = [0u8; 512];
        log[48..64].copy_from_slice(&20_000_000u128.to_le_bytes());
        assert!((nvme_data_units_written_tb(&log).unwrap() - 10.24).abs() < 1e-9);
        log[48..64].copy_from_slice(&u128::MAX.to_le_bytes());
        assert!(nvme_data_units_written_tb(&log).unwrap().is_finite());
        assert_eq!(nvme_data_units_written_tb(&log[..63]), None);
    }

    #[test]
    fn virtual_disks() {
        assert!(is_virtual_disk_model("Msft Virtual Disk"));
        assert!(is_virtual_disk_model("VBOX HARDDISK"));
        assert!(is_virtual_disk_model("VMware Virtual NVMe Disk"));
        assert!(!is_virtual_disk_model("KINGSTON SNV3S500G"));
    }
}
