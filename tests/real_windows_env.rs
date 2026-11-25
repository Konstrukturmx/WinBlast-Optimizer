//! Wave 4: Real Windows Host Read-Only Inductive Audit + Strictly Isolated
//! Win32 Registry & NTFS Junction Sandbox Verification (`HKCU\Software\WinBlast0_TestEnv_<uuid>`
//! and `%TEMP%\WinBlast0_TestEnv_<uuid>` with RAII `Drop` cleanup).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use uuid::Uuid;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

use winblast0::core::backend::{RealBackend, SystemBackend};
use winblast0::core::os_detect::WindowsInfo;
use winblast0::core::reg_helpers::{is_known_numeric_reg_sz, RegRawSnapshot, RegTreeSnapshot};
use winblast0::core::risk::{load_embedded_rules, TweakAction};
use winblast0::executor::reg_mutator;
use winblast0::hardware::cpu::CpuInfo;
use winblast0::hardware::gpu::{self, GpuVendor};
use winblast0::hardware::parking::CoreParkingState;
use winblast0::scanner::{registry_scanner, run_full_scan_with, TweakState};
use winblast0::tools::cleaner::{self, CleanTargetKind};

const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0400;

/// RAII cleanup guard ensuring any temporary HKCU test subtree is always deleted,
/// even if an assertion panics mid-test.
struct HkcuTestEnvGuard {
    subkey: String,
}

impl Drop for HkcuTestEnvGuard {
    fn drop(&mut self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let _ = hkcu.delete_subkey_all(&self.subkey);
    }
}

/// RAII cleanup guard ensuring temporary `%TEMP%\WinBlast0_TestEnv_<uuid>` directories,
/// NTFS junctions (unlinked safely without deleting targets), and read-only files are
/// always removed cleanly on test pass or panic.
struct TempTestEnvGuard {
    root: PathBuf,
    junctions: Vec<PathBuf>,
    readonly_files: Vec<PathBuf>,
}

impl TempTestEnvGuard {
    fn new(root: PathBuf) -> Self {
        Self {
            root,
            junctions: Vec::new(),
            readonly_files: Vec::new(),
        }
    }
}

impl Drop for TempTestEnvGuard {
    fn drop(&mut self) {
        // 1. Unlink NTFS junctions first using remove_dir (never recurses into target)
        for j in &self.junctions {
            if j.exists() || fs::symlink_metadata(j).is_ok() {
                let _ = fs::remove_dir(j);
            }
        }
        // 2. Clear read-only attribute on any remaining files before recursive cleanup
        for ro in &self.readonly_files {
            if let Ok(meta) = fs::symlink_metadata(ro) {
                let mut perms = meta.permissions();
                if perms.readonly() {
                    #[allow(clippy::permissions_set_readonly_false)]
                    perms.set_readonly(false);
                    let _ = fs::set_permissions(ro, perms);
                }
            }
        }
        // 3. Remove any leftover read-only attributes inside root just in case
        fn clear_ro_recursive(dir: &Path) {
            let Ok(rd) = fs::read_dir(dir) else {
                return;
            };
            for ent in rd.flatten() {
                let p = ent.path();
                let Ok(meta) = fs::symlink_metadata(&p) else {
                    continue;
                };
                use std::os::windows::fs::MetadataExt;
                if (meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0 {
                    let _ = fs::remove_dir(&p);
                    continue;
                }
                if meta.is_dir() {
                    clear_ro_recursive(&p);
                } else {
                    let mut perms = meta.permissions();
                    if perms.readonly() {
                        #[allow(clippy::permissions_set_readonly_false)]
                        perms.set_readonly(false);
                        let _ = fs::set_permissions(&p, perms);
                    }
                }
            }
        }
        clear_ro_recursive(&self.root);
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "touches the real host; run only inside the test VM: cargo test --features vm-tests")]
fn test_real_backend_readonly_live_host_audit() {
    let backend = RealBackend;

    // 1. OS Detection (supported: Windows 10 2004 / build 19041 and later, incl. Windows 11)
    let os = backend.detect_os().expect("RealBackend::detect_os failed");
    let os_direct = WindowsInfo::detect().expect("WindowsInfo::detect failed");
    assert_eq!(os.build_number, os_direct.build_number);
    assert_eq!(os.edition, os_direct.edition);
    assert_eq!(os.display_version, os_direct.display_version);
    assert!(
        os.build_number >= 19041,
        "Expected a supported Windows build (>= 19041), got {}",
        os.build_number
    );
    assert_eq!(
        os.is_win11,
        os.build_number >= 22000,
        "is_win11 must match the build number ({})",
        os.build_number
    );
    assert_ne!(os.edition, "Unknown", "OS edition must not be Unknown");
    assert_ne!(
        os.display_version, "Unknown",
        "OS display_version must not be Unknown"
    );

    // 2. GPU Detection (valid PCI instance IDs, no virtual adapter false positives, non-empty class_subkey)
    let gpus = backend.detect_gpus();
    let gpus_direct = gpu::detect_gpus();
    assert_eq!(gpus.len(), gpus_direct.len());

    for g in &gpus {
        assert!(
            !g.class_subkey.trim().is_empty(),
            "GPU '{}' has empty class_subkey",
            g.name
        );

        let pnp_upper = g.pnp_instance_id.to_ascii_uppercase();
        if pnp_upper.starts_with("PCI\\") {
            let segs: Vec<&str> = g
                .pnp_instance_id
                .split('\\')
                .filter(|s| !s.trim().is_empty())
                .collect();
            assert!(
                segs.len() >= 3,
                "PCI GPU '{}' has malformed/bare HardwareId pnp_instance_id '{}': expected >= 3 segments with instance suffix",
                g.name,
                g.pnp_instance_id
            );
            assert!(
                !g.pnp_instance_id.contains('*'),
                "PCI GPU '{}' pnp_instance_id contains wildcard: '{}'",
                g.name,
                g.pnp_instance_id
            );
        }

        let name_lower = g.name.to_ascii_lowercase();
        let is_virtual_name = name_lower.contains("parsec")
            || name_lower.contains("spacedesk")
            || name_lower.contains("virtual display")
            || name_lower.contains("indirect display")
            || name_lower.contains("basic display");
        if is_virtual_name && !pnp_upper.starts_with("PCI\\") {
            assert!(
                !matches!(g.vendor, GpuVendor::Nvidia | GpuVendor::Amd),
                "Virtual display adapter '{}' ({}) was misclassified as {:?}",
                g.name,
                g.pnp_instance_id,
                g.vendor
            );
        }
    }

    // Also assert detect_vendor refuses to misclassify Parsec/spacedesk/virtual adapters
    assert_eq!(
        gpu::detect_vendor("ROOT\\PARSEC_VDA\\0000", "Parsec Virtual Display Adapter"),
        GpuVendor::Unknown
    );
    assert_eq!(
        gpu::detect_vendor("ROOT\\DISPLAY\\0001", "spacedesk Graphics Adapter"),
        GpuVendor::Unknown
    );
    assert_eq!(
        gpu::detect_vendor(
            "ROOT\\BASICDISPLAY\\0000",
            "Microsoft Basic Display Adapter"
        ),
        GpuVendor::Unknown
    );

    // 3. CPU & Core Parking Topology Detection (no "#0 #0" / "#1 #1", valid hybrid counts)
    let cpu = backend.detect_cpu();
    let cpu_direct = CpuInfo::detect();
    assert_eq!(cpu.physical_cores, cpu_direct.physical_cores);
    assert_eq!(cpu.logical_cores, cpu_direct.logical_cores);
    assert!(cpu.physical_cores > 0, "physical_cores must be > 0");
    assert!(
        cpu.logical_cores >= cpu.physical_cores,
        "logical_cores ({}) < physical_cores ({})",
        cpu.logical_cores,
        cpu.physical_cores
    );
    if cpu.has_hybrid_topology {
        assert!(
            cpu.p_core_count > 0,
            "Hybrid CPU must report p_core_count > 0"
        );
        assert!(
            cpu.e_core_count > 0,
            "Hybrid CPU must report e_core_count > 0"
        );
        assert_eq!(
            cpu.p_core_count + cpu.e_core_count,
            cpu.physical_cores,
            "Hybrid P-cores ({}) + E-cores ({}) must equal physical_cores ({})",
            cpu.p_core_count,
            cpu.e_core_count,
            cpu.physical_cores
        );
    }

    let parking = backend.detect_core_parking();
    let parking_direct = CoreParkingState::detect();
    assert_eq!(parking.total_cores, parking_direct.total_cores);
    assert_eq!(parking.cores.len(), parking.total_cores);
    assert!(parking.total_cores >= cpu.logical_cores);
    for core in &parking.cores {
        assert!(
            !core.core_type_label.trim().is_empty(),
            "Core #{} has empty core_type_label",
            core.core_index
        );
        assert!(
            !core.core_type_label.contains('#'),
            "Core #{} core_type_label '{}' must not already include '#' (causes '#0 #0' UI doubling)",
            core.core_index,
            core.core_type_label
        );
        let rendered = format!("{} #{}", core.core_type_label, core.core_index);
        assert_eq!(
            rendered.matches('#').count(),
            1,
            "Rendered core label '{}' has unexpected '#' count",
            rendered
        );
        assert!(!rendered.contains("#0 #0") && !rendered.contains("#1 #1"));
    }

    // 4. Read-only live Power Schemes, Startup Entries, and Win32 Services
    let schemes = backend.list_power_schemes();
    if !schemes.is_empty() {
        assert!(
            schemes.iter().any(|s| s.is_active),
            "Expected an active power scheme among {:?}",
            schemes
        );
        assert!(
            schemes
                .iter()
                .all(|s| !s.friendly_name.trim().is_empty() && !s.guid.trim().is_empty()),
            "Power schemes must have non-empty friendly_name and guid: {:?}",
            schemes
        );
    }

    let startup = backend
        .list_startup_entries()
        .expect("RealBackend::list_startup_entries() failed");
    for ent in &startup {
        assert!(!ent.id.is_empty() && !ent.name.is_empty());
    }

    let services = backend
        .list_live_services()
        .expect("RealBackend::list_live_services() failed");
    assert!(
        !services.is_empty(),
        "Expected non-empty live Windows services list"
    );
    assert!(
        services.iter().any(|s| {
            s.service_name.eq_ignore_ascii_case("EventLog")
                || s.service_name.eq_ignore_ascii_case("RpcSs")
                || s.service_name.eq_ignore_ascii_case("Dnscache")
        }),
        "Expected standard Windows core service (EventLog/RpcSs/Dnscache) in list_live_services()"
    );

    // 5. Read-only full catalog scan over all 230 rules against RealBackend
    // ISO 29119-3 Test Spec:
    // - Preconditions: RealBackend initialized with read-only permissions on host Windows environment.
    // - Equivalence Classes: Host Windows build number (Win10 vs Win11 24H2+), host GPU inventory.
    // - Boundary Conditions: Boundary build 26000 for MPO deprecation check.
    // - Invariants: Catalog length == 230, scan results count == 230, RealBackend performs zero system mutations.
    // - Expected Outcome: All 230 tweaks return deterministic valid TweakState without panic or mutation.
    let tweaks = load_embedded_rules();
    assert_eq!(tweaks.len(), 230, "Expected 230 embedded catalog rules");

    let scan_results = run_full_scan_with(&backend, &tweaks, os.build_number, &os.edition);
    assert_eq!(scan_results.len(), 230, "Expected 230 scan results");

    if os.build_number >= 26000 {
        let mpo_res = scan_results
            .iter()
            .find(|r| r.tweak_id == "disable_mpo")
            .expect("Missing disable_mpo in scan_results");
        assert_eq!(
            mpo_res.current_state,
            TweakState::NotApplicable,
            "disable_mpo (max_build=25999) must be NotApplicable on build {} (got {:?}: {})",
            os.build_number,
            mpo_res.current_state,
            mpo_res.status_detail
        );
    }

    let detected_vendors: Vec<GpuVendor> = gpus
        .iter()
        .map(|g| g.vendor)
        .filter(|v| *v != GpuVendor::Unknown)
        .collect();
    for t in &tweaks {
        if t.gpu_vendor.is_some() && !t.is_gpu_vendor_compatible(&detected_vendors) {
            let sr = scan_results
                .iter()
                .find(|r| r.tweak_id == t.id)
                .expect("Missing GPU tweak scan result");
            assert_eq!(
                sr.current_state,
                TweakState::NotApplicable,
                "Foreign GPU tweak '{}' (requires {:?}, host has {:?}) must be NotApplicable, got {:?}",
                t.id,
                t.gpu_vendor,
                detected_vendors,
                sr.current_state
            );
        }
    }

    if !backend.is_elevated() {
        for t in &tweaks {
            let only_features = !t.actions.is_empty()
                && t.actions
                    .iter()
                    .all(|a| matches!(a, TweakAction::WindowsFeature { .. }));
            if only_features && t.is_compatible(os.build_number, &os.edition) {
                let sr = scan_results
                    .iter()
                    .find(|r| r.tweak_id == t.id)
                    .expect("Missing WindowsFeature scan result");
                assert_eq!(
                    sr.current_state,
                    TweakState::NotApplicable,
                    "Unelevated WindowsFeature tweak '{}' must evaluate NotApplicable (never fake AlreadyOptimized), got {:?} ({})",
                    t.id,
                    sr.current_state,
                    sr.status_detail
                );
            }
        }
    }

    println!(
        "Live Audit OK -> elevated={} build={} edition={} gpus={:?} cpu_phys={} cpu_log={} schemes={} startup={} services={}",
        backend.is_elevated(),
        os.build_number,
        os.edition,
        gpus.iter().map(|g| format!("{} [{:?}] subkey={} pnp={}", g.name, g.vendor, g.class_subkey, g.pnp_instance_id)).collect::<Vec<_>>(),
        cpu.physical_cores,
        cpu.logical_cores,
        schemes.len(),
        startup.len(),
        services.len()
    );
}

#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "touches the real host; run only inside the test VM: cargo test --features vm-tests")]
fn test_real_win32_registry_types_numeric_sz_binary_and_tree_roundtrip() {
    let root_subkey = format!("Software\\WinBlast0_TestEnv_{}", Uuid::new_v4());
    let _guard = HkcuTestEnvGuard {
        subkey: root_subkey.clone(),
    };
    let backend = RealBackend;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) =
        match hkcu.create_subkey_with_flags(&root_subkey, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY) {
            Ok(k) => k,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                eprintln!("Skipping HKCU test: sandbox restricted HKCU write access: {e}");
                return;
            }
            Err(e) => panic!("Failed to create isolated HKCU test key: {e}"),
        };

    // 1. read_dword_from_key (via RealBackend::reg_get_dword & registry_scanner::get_dword):
    //    Returns None on "Hello", "0x26", >u32::MAX QWORD, REG_BINARY; reads neg/pos decimal SZ, QWORD<=u32::MAX, REG_DWORD.
    key.set_value("StrHello", &"Hello").unwrap();
    key.set_value("StrHex", &"0x26").unwrap();
    key.set_value("StrNeg", &"-12").unwrap();
    key.set_value("StrPos", &" 400 ").unwrap();
    key.set_value("RealDword", &0xCAFE_BABEu32).unwrap();
    key.set_raw_value(
        "QwordTooBig",
        &RegValue {
            bytes: (u32::MAX as u64 + 1).to_le_bytes().to_vec(),
            vtype: REG_QWORD,
        },
    )
    .unwrap();
    key.set_raw_value(
        "QwordSmall",
        &RegValue {
            bytes: 777u64.to_le_bytes().to_vec(),
            vtype: REG_QWORD,
        },
    )
    .unwrap();
    key.set_raw_value(
        "ExpandNum",
        &RegValue {
            bytes: "88"
                .encode_utf16()
                .chain(std::iter::once(0))
                .flat_map(|w| w.to_le_bytes())
                .collect(),
            vtype: REG_EXPAND_SZ,
        },
    )
    .unwrap();

    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "StrHello"),
        None
    );
    assert_eq!(
        registry_scanner::get_dword("HKCU", &root_subkey, "StrHex"),
        None
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "QwordTooBig"),
        None
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "QwordSmall"),
        Some(777)
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "StrNeg"),
        Some((-12i32) as u32)
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "StrPos"),
        Some(400)
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "RealDword"),
        Some(0xCAFE_BABE)
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "ExpandNum"),
        Some(88)
    );

    // 2. write_dword_or_numeric_sz (via RealBackend::reg_set_dword & reg_mutator::set_dword):
    //    Preserves REG_SZ, negative "-12", and REG_EXPAND_SZ, plus verify is_known_numeric_reg_sz normalization.
    backend
        .reg_set_dword("HKCU", &root_subkey, "StrPos", 250)
        .expect("reg_set_dword StrPos failed");
    let raw_pos = key.get_raw_value("StrPos").unwrap();
    assert_eq!(raw_pos.vtype, REG_SZ, "StrPos must stay REG_SZ");
    assert_eq!(key.get_value::<String, _>("StrPos").unwrap(), "250");
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "StrPos"),
        Some(250)
    );

    reg_mutator::set_dword("HKCU", &root_subkey, "StrNeg", (-24i32) as u32)
        .expect("set_dword StrNeg failed");
    let raw_neg = key.get_raw_value("StrNeg").unwrap();
    assert_eq!(raw_neg.vtype, REG_SZ, "StrNeg must stay REG_SZ");
    assert_eq!(key.get_value::<String, _>("StrNeg").unwrap(), "-24");
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "StrNeg"),
        Some((-24i32) as u32)
    );

    backend
        .reg_set_dword("HKCU", &root_subkey, "ExpandNum", 95)
        .expect("reg_set_dword ExpandNum failed");
    let raw_exp = key.get_raw_value("ExpandNum").unwrap();
    assert_eq!(
        raw_exp.vtype, REG_EXPAND_SZ,
        "ExpandNum must stay REG_EXPAND_SZ"
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &root_subkey, "ExpandNum"),
        Some(95)
    );

    // Normalization checks for is_known_numeric_reg_sz
    assert!(is_known_numeric_reg_sz(
        r"\Control Panel\Desktop\",
        "MenuShowDelay"
    ));
    assert!(is_known_numeric_reg_sz(
        r"control panel\desktop",
        "menushowdelay"
    ));
    assert!(is_known_numeric_reg_sz(
        r"\Control Panel\Desktop\WindowMetrics\",
        "BorderWidth"
    ));
    assert!(is_known_numeric_reg_sz(
        r"Control Panel\Desktop\WindowMetrics",
        "PaddedBorderWidth"
    ));
    assert!(is_known_numeric_reg_sz(
        r"Control Panel\Desktop\WindowMetrics",
        "MinAnimate"
    ));
    assert!(is_known_numeric_reg_sz(
        r"Control Panel\Keyboard",
        "KeyboardSpeed"
    ));
    assert!(is_known_numeric_reg_sz(
        r"Control Panel\Mouse",
        "MouseHoverTime"
    ));
    assert!(is_known_numeric_reg_sz(
        r"Control Panel\Accessibility\Keyboard Response",
        "AutoRepeatDelay"
    ));
    assert!(!is_known_numeric_reg_sz(&root_subkey, "MenuShowDelay"));

    // 3. reg_set_binary / reg_get_binary strictly enforce REG_BINARY (reject REG_DWORD & REG_SZ)
    key.set_value("DwordSameName", &0x1122_3344u32).unwrap();
    assert_eq!(
        backend.reg_get_binary("HKCU", &root_subkey, "DwordSameName"),
        None,
        "reg_get_binary must reject REG_DWORD"
    );
    assert_eq!(
        registry_scanner::get_binary("HKCU", &root_subkey, "StrHello"),
        None,
        "get_binary must reject REG_SZ"
    );

    let bin_bytes = [0x9e, 0x3e, 0x07, 0x80, 0x12, 0x00, 0x00, 0x00];
    backend
        .reg_set_binary("HKCU", &root_subkey, "RealBin", &bin_bytes)
        .expect("reg_set_binary failed");
    assert_eq!(
        backend.reg_get_binary("HKCU", &root_subkey, "RealBin"),
        Some(bin_bytes.to_vec())
    );
    assert_eq!(backend.reg_get_dword("HKCU", &root_subkey, "RealBin"), None);

    // 4. reg_snapshot_tree + recursive reg_delete_tree + reg_restore_tree on multi-type nested subtree
    //    (DWORD, QWORD, SZ, EXPAND_SZ, BINARY, nested child subkey, (Default) and "@")
    let tree_path = format!("{}\\NestedTree\\Target", root_subkey);
    let child_path = format!("{}\\NestedChild", tree_path);
    let grandchild_path = format!("{}\\DeepGrandChild", child_path);

    // Populate Target root
    backend
        .reg_set_string(
            "HKCU",
            &tree_path,
            "(Default)",
            "RootDefaultValue",
            false,
            None,
        )
        .unwrap();
    backend
        .reg_set_dword("HKCU", &tree_path, "TreeDword", 4242)
        .unwrap();
    backend
        .reg_set_raw(
            "HKCU",
            &tree_path,
            "TreeQword",
            &RegRawSnapshot {
                reg_type: 11,
                hex_bytes: winblast0::core::reg_helpers::encode_hex(
                    &0x0123_4567_89AB_CDEFu64.to_le_bytes(),
                ),
            },
        )
        .unwrap();
    backend
        .reg_set_string(
            "HKCU",
            &tree_path,
            "TreeSz",
            "Hello Win32 Tree",
            false,
            None,
        )
        .unwrap();
    backend
        .reg_set_string(
            "HKCU",
            &tree_path,
            "TreeExpandSz",
            "%SystemRoot%\\System32\\cmd.exe",
            true,
            None,
        )
        .unwrap();
    backend
        .reg_set_binary(
            "HKCU",
            &tree_path,
            "TreeBinary",
            &[0xDE, 0xAD, 0xBE, 0xEF, 0x55],
        )
        .unwrap();

    // Also test "@" alias for (Default) on NestedChild AND literal named "@" value directly in Win32
    backend
        .reg_set_string(
            "HKCU",
            &child_path,
            "@",
            "ChildDefaultViaAtAlias",
            false,
            None,
        )
        .unwrap();
    backend
        .reg_set_dword("HKCU", &child_path, "ChildDword", 99)
        .unwrap();

    let (gc_key, _) = match hkcu
        .create_subkey_with_flags(&grandchild_path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
    {
        Ok(k) => k,
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("Skipping HKCU test: sandbox restricted HKCU write access: {e}");
            return;
        }
        Err(e) => panic!("Failed to create grandchild subkey: {e}"),
    };
    gc_key.set_value("", &"GrandChildDefault").unwrap();
    gc_key.set_value("@", &"LiteralAtValue").unwrap();
    gc_key.set_value("DeepSz", &"DeepValue").unwrap();
    drop(gc_key);

    let snapshot: RegTreeSnapshot = backend
        .reg_snapshot_tree("HKCU", &tree_path)
        .expect("reg_snapshot_tree failed")
        .expect("Expected Some(RegTreeSnapshot) for existing Win32 subtree");

    backend
        .reg_delete_tree("HKCU", &tree_path)
        .expect("reg_delete_tree failed");
    assert!(
        !backend.reg_key_exists("HKCU", &tree_path),
        "Target subtree must be deleted after reg_delete_tree"
    );
    assert!(
        !backend.reg_key_exists("HKCU", &grandchild_path),
        "Nested grandchild must be deleted after reg_delete_tree"
    );

    backend
        .reg_restore_tree("HKCU", &tree_path, &snapshot)
        .expect("reg_restore_tree failed");
    assert!(backend.reg_key_exists("HKCU", &tree_path));
    assert!(backend.reg_key_exists("HKCU", &grandchild_path));

    let snapshot_restored = backend
        .reg_snapshot_tree("HKCU", &tree_path)
        .expect("post-restore reg_snapshot_tree failed")
        .expect("restored subtree missing");
    assert_eq!(
        snapshot_restored, snapshot,
        "Restored multi-type Win32 subtree snapshot must match original losslessly"
    );

    // Explicit per-type verification after restore
    assert_eq!(
        backend
            .reg_get_string("HKCU", &tree_path, "(Default)", None)
            .map(|v| (v.value, v.expand_sz)),
        Some(("RootDefaultValue".to_string(), false))
    );
    assert_eq!(
        backend.reg_get_dword("HKCU", &tree_path, "TreeDword"),
        Some(4242)
    );
    assert_eq!(
        backend
            .reg_get_raw("HKCU", &tree_path, "TreeQword")
            .map(|r| r.reg_type),
        Some(11)
    );
    assert_eq!(
        backend
            .reg_get_string("HKCU", &tree_path, "TreeExpandSz", None)
            .map(|v| (v.value, v.expand_sz)),
        Some(("%SystemRoot%\\System32\\cmd.exe".to_string(), true))
    );
    assert_eq!(
        backend.reg_get_binary("HKCU", &tree_path, "TreeBinary"),
        Some(vec![0xDE, 0xAD, 0xBE, 0xEF, 0x55])
    );
    assert_eq!(
        backend
            .reg_get_string("HKCU", &child_path, "@", None)
            .map(|v| v.value),
        Some("ChildDefaultViaAtAlias".to_string())
    );
    let gc_ro = hkcu
        .open_subkey_with_flags(&grandchild_path, KEY_READ | KEY_WOW64_64KEY)
        .unwrap();
    assert_eq!(
        gc_ro.get_value::<String, _>("").unwrap(),
        "GrandChildDefault"
    );
    assert_eq!(gc_ro.get_value::<String, _>("@").unwrap(), "LiteralAtValue");
    assert_eq!(gc_ro.get_value::<String, _>("DeepSz").unwrap(), "DeepValue");
}

#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "touches the real host; run only inside the test VM: cargo test --features vm-tests")]
fn test_ntfs_junction_and_readonly_cleanup_sandbox() {
    // Child-process isolation mode: only executed when spawned explicitly by this test
    // with TMP & TEMP strictly pointing at the synthetic sandbox <cleanup_dir>.
    if let Ok(expected_dir) = std::env::var("WINBLAST_CLEAN_ISOLATED_DIR") {
        let actual_temp =
            fs::canonicalize(std::env::temp_dir()).expect("canonicalize std::env::temp_dir failed");
        let expected_canon = fs::canonicalize(&expected_dir)
            .expect("canonicalize WINBLAST_CLEAN_ISOLATED_DIR failed");
        assert_eq!(
            actual_temp, expected_canon,
            "Safety abort: std::env::temp_dir() ({:?}) != isolated sandbox ({:?})",
            actual_temp, expected_canon
        );

        let scans = cleaner::scan_all_clean_targets();
        let user_temp_scan = scans
            .iter()
            .find(|s| s.kind == CleanTargetKind::UserTemp)
            .expect("Missing UserTemp scan");
        assert_eq!(
            user_temp_scan.file_count, 3,
            "UserTemp scan must count only the 3 real temp files (excluding .net and junction target), got {:?}",
            user_temp_scan
        );
        assert_eq!(user_temp_scan.reclaimable_bytes, 27);

        let report = cleaner::execute_clean_targets(&[CleanTargetKind::UserTemp])
            .expect("execute_clean_targets failed");
        assert_eq!(report.results.len(), 1);
        assert_eq!(report.results[0].error, None);
        assert_eq!(report.total_deleted_files, 3);
        assert_eq!(report.total_freed_bytes, 27);
        return;
    }

    // Parent orchestrator: creates isolated %TEMP%\WinBlast0_TestEnv_<uuid> tree with RAII cleanup
    let sandbox_root = std::env::temp_dir().join(format!("WinBlast0_TestEnv_{}", Uuid::new_v4()));
    let mut guard = TempTestEnvGuard::new(sandbox_root.clone());

    let cleanup_dir = sandbox_root.join("cleanup_dir");
    let outside_secret_dir = sandbox_root.join("outside_secret_dir");
    let outside_nested = outside_secret_dir.join("nested");
    let dotnet_dir = cleanup_dir.join(".net").join("app");
    let sub_dir = cleanup_dir.join("sub_temp");

    fs::create_dir_all(&cleanup_dir).unwrap();
    fs::create_dir_all(&outside_nested).unwrap();
    fs::create_dir_all(&dotnet_dir).unwrap();
    fs::create_dir_all(&sub_dir).unwrap();

    // Outside secret files (must remain 100% untouched across junction)
    let secret_file = outside_secret_dir.join("secret.txt");
    let nested_secret = outside_nested.join("treasure.dat");
    fs::write(&secret_file, b"DO NOT DELETE VIA JUNCTION").unwrap();
    fs::write(&nested_secret, b"INTACT PAYLOAD").unwrap();

    // .net preservation file inside cleanup_dir (must remain untouched)
    let dotnet_file = dotnet_dir.join("runtime.dll");
    fs::write(&dotnet_file, b"keep-dotnet-singlefile").unwrap();

    // Normal & FILE_ATTRIBUTE_READONLY temp files inside cleanup_dir (total = 5 + 13 + 9 = 27 bytes)
    let normal_tmp = cleanup_dir.join("normal.tmp");
    let ro_tmp = cleanup_dir.join("readonly.tmp");
    let nested_ro_tmp = sub_dir.join("nested_ro.tmp");
    fs::write(&normal_tmp, b"junk1").unwrap(); // 5 bytes
    fs::write(&ro_tmp, b"readonly_junk").unwrap(); // 13 bytes
    fs::write(&nested_ro_tmp, b"nested_ro").unwrap(); // 9 bytes

    for ro in [&ro_tmp, &nested_ro_tmp] {
        let mut perms = fs::metadata(ro).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(ro, perms).unwrap();
        guard.readonly_files.push(ro.clone());
    }

    // Create real NTFS directory junction: <cleanup_dir>\escape_link -> <outside_secret_dir>
    let junction_path = cleanup_dir.join("escape_link");
    guard.junctions.push(junction_path.clone());
    let mklink_out = Command::new("cmd")
        .args([
            "/C",
            "mklink",
            "/J",
            &junction_path.to_string_lossy(),
            &outside_secret_dir.to_string_lossy(),
        ])
        .output()
        .expect("Failed to invoke cmd /C mklink /J");
    if !mklink_out.status.success() {
        let err_str = String::from_utf8_lossy(&mklink_out.stderr);
        if err_str.to_lowercase().contains("access is denied")
            || err_str.to_lowercase().contains("erişim engellendi")
        {
            eprintln!("Skipping NTFS junction test: sandbox or un-elevated token does not permit mklink /J: {err_str}");
            return;
        }
        panic!(
            "mklink /J failed: stdout={} stderr={}",
            String::from_utf8_lossy(&mklink_out.stdout),
            err_str
        );
    }

    use std::os::windows::fs::MetadataExt;
    let j_meta = fs::symlink_metadata(&junction_path).expect("junction symlink_metadata failed");
    assert_ne!(
        j_meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT,
        0,
        "escape_link must have FILE_ATTRIBUTE_REPARSE_POINT"
    );
    assert!(
        junction_path.join("secret.txt").exists(),
        "Junction must resolve to outside_secret_dir before cleaner runs"
    );

    // Run cleaner::execute_clean_targets(&[CleanTargetKind::UserTemp]) inside isolated child process
    let exe = std::env::current_exe().expect("current_exe failed");
    let child_out = Command::new(exe)
        .args([
            "--exact",
            "test_ntfs_junction_and_readonly_cleanup_sandbox",
            "--nocapture",
        ])
        .env("TMP", &cleanup_dir)
        .env("TEMP", &cleanup_dir)
        .env("WINBLAST_CLEAN_ISOLATED_DIR", &cleanup_dir)
        .output()
        .expect("Failed to spawn isolated cleaner child test");

    assert!(
        child_out.status.success(),
        "Isolated cleaner child test failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&child_out.stdout),
        String::from_utf8_lossy(&child_out.stderr)
    );

    // Assert outside_secret_dir and ALL files inside it remain 100% untouched
    assert_eq!(
        fs::read(&secret_file).expect("secret.txt disappeared!"),
        b"DO NOT DELETE VIA JUNCTION"
    );
    assert_eq!(
        fs::read(&nested_secret).expect("nested treasure.dat disappeared!"),
        b"INTACT PAYLOAD"
    );
    assert!(
        fs::symlink_metadata(&junction_path).is_ok(),
        "Junction reparse point should remain skipped rather than followed"
    );

    // Assert normal & read-only temp files in cleanup_dir were removed cleanly, and .net preserved
    assert!(!normal_tmp.exists(), "normal.tmp should have been deleted");
    assert!(!ro_tmp.exists(), "readonly.tmp should have been deleted");
    assert!(
        !nested_ro_tmp.exists(),
        "nested_ro.tmp should have been deleted"
    );
    assert!(
        !sub_dir.exists(),
        "empty sub_temp directory should have been removed"
    );
    assert_eq!(
        fs::read(&dotnet_file).expect(".net/app/runtime.dll should be preserved!"),
        b"keep-dotnet-singlefile"
    );
}

#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "touches the real host; run only inside the test VM: cargo test --features vm-tests")]
fn test_real_windows_disk_and_thermal_telemetry() {
    let report = winblast0::hardware::disks::DiskSubsystemReport::probe();
    println!("Found {} physical drives on host:", report.drives.len());
    for drv in &report.drives {
        println!(
            "Drive {}: {} ({}) -> {:.1} GB, Health: {:?}%, Temp: {:.1}°C, TBW: {:?}, POH: {} h",
            drv.index,
            drv.model,
            drv.interface_type,
            drv.total_size_gb,
            drv.health_percentage,
            drv.temperature_c,
            drv.host_writes,
            drv.power_on_hours
        );
        assert!(
            drv.total_size_gb > 0.0,
            "Physical drive {} size must not be 0.0 GB! Got {:.1} GB",
            drv.index,
            drv.total_size_gb
        );
    }
}
