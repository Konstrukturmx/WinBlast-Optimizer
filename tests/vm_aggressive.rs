//! Aggressive real-host tests. They mutate HKLM, services, scheduled tasks and power
//! settings for real, so they are double-guarded: the "vm-tests" feature AND
//! WINBLAST_VM_AGGRESSIVE=1 AND the BIOS must report a VM product name.
//! Run only inside a disposable VM that is restored from a checkpoint afterwards:
//!   WINBLAST_VM_AGGRESSIVE=1 cargo test --features vm-tests --test vm_aggressive -- --include-ignored --test-threads=1 --nocapture

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use winblast0::backup::state_ledger::{LedgerEntry, StateLedger};
use winblast0::core::backend::{RealBackend, SystemBackend};
use winblast0::core::risk::{load_embedded_rules, Reversibility, TweakAction, TweakDefinition};
use winblast0::executor::engine::{apply_tweak_with, revert_tweak_with, rollback_entry_with};
use winblast0::scanner::{scan_single_tweak_with, TweakState};
use winreg::enums::*;
use winreg::RegKey;

fn guard() -> bool {
    if std::env::var("WINBLAST_VM_AGGRESSIVE").ok().as_deref() != Some("1") {
        eprintln!("skipped: WINBLAST_VM_AGGRESSIVE != 1");
        return false;
    }
    let bios = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"HARDWARE\DESCRIPTION\System\BIOS")
        .and_then(|k| k.get_value::<String, _>("SystemProductName"))
        .unwrap_or_default();
    assert!(
        bios.contains("Virtual Machine") || bios.contains("VMware") || bios.contains("VirtualBox"),
        "refusing to run aggressive tests on a non-VM host (SystemProductName = {bios:?})"
    );
    assert!(RealBackend.is_elevated(), "aggressive VM tests need an elevated token");
    true
}

fn hive(name: &str) -> Option<RegKey> {
    match name.to_ascii_uppercase().as_str() {
        "HKLM" | "HKEY_LOCAL_MACHINE" => Some(RegKey::predef(HKEY_LOCAL_MACHINE)),
        "HKCU" | "HKEY_CURRENT_USER" => Some(RegKey::predef(HKEY_CURRENT_USER)),
        "HKU" | "HKEY_USERS" => Some(RegKey::predef(HKEY_USERS)),
        "HKCR" | "HKEY_CLASSES_ROOT" => Some(RegKey::predef(HKEY_CLASSES_ROOT)),
        _ => None,
    }
}

/// Every value of one key (not recursive), type + bytes, or "<absent key>".
fn key_values(h: &str, path: &str) -> String {
    let Some(root) = hive(h) else {
        return format!("<unknown hive {h}>");
    };
    match root.open_subkey_with_flags(path, KEY_READ | KEY_WOW64_64KEY) {
        Err(_) => "<absent key>".into(),
        Ok(k) => {
            let mut v: Vec<String> = k
                .enum_values()
                .filter_map(|r| r.ok())
                .map(|(n, val)| format!("{n}={:?}:{:02x?}", val.vtype, val.bytes))
                .collect();
            v.sort();
            v.join(" | ")
        }
    }
}

/// Recursive dump of a subtree (for RegistryDeleteTree).
fn tree(h: &str, path: &str, depth: u32, out: &mut Vec<String>) {
    if depth > 12 {
        return;
    }
    out.push(format!("[{path}] {}", key_values(h, path)));
    if let Some(root) = hive(h) {
        if let Ok(k) = root.open_subkey_with_flags(path, KEY_READ | KEY_WOW64_64KEY) {
            let mut subs: Vec<String> = k.enum_keys().filter_map(|r| r.ok()).collect();
            subs.sort();
            for s in subs {
                tree(h, &format!("{path}\\{s}"), depth + 1, out);
            }
        }
    }
}

/// Independent (winreg / backend getter) probe of everything a tweak can touch.
fn probe(t: &TweakDefinition, b: &RealBackend) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for a in t.actions.iter().chain(t.fallbacks.iter().flatten()) {
        match a {
            TweakAction::RegistryDword { hive: h, path, .. }
            | TweakAction::RegistryString { hive: h, path, .. }
            | TweakAction::RegistryDelete { hive: h, path, .. }
            | TweakAction::RegistryBinaryBit { hive: h, path, .. }
            | TweakAction::CompositeStringKey { hive: h, path, .. }
            | TweakAction::RegistryDynamicDword { hive: h, path, .. } => {
                if hive(h).is_some() {
                    m.insert(format!("reg {h}\\{path}"), key_values(h, path));
                } else {
                    m.insert(format!("reg? {h}\\{path}"), format!("{:?}", b.reg_key_exists(h, path)));
                }
            }
            TweakAction::RegistryDeleteTree { hive: h, path } => {
                let mut v = Vec::new();
                tree(h, path, 0, &mut v);
                m.insert(format!("tree {h}\\{path}"), v.join("\n"));
            }
            TweakAction::PowerCfgSetting { subgroup_guid, setting_guid, scheme_guid, .. } => {
                m.insert("power active".into(), format!("{:?}", b.get_active_power_scheme().ok()));
                m.insert(
                    format!("power {subgroup_guid}/{setting_guid}"),
                    format!("{:?}", b.read_power_setting_index(scheme_guid.as_deref(), subgroup_guid, setting_guid).ok()),
                );
            }
            TweakAction::ServiceState { service_name, .. } => {
                let p = format!(r"SYSTEM\CurrentControlSet\Services\{service_name}");
                m.insert(format!("svc {service_name}"), key_values("HKLM", &p));
            }
            TweakAction::ScheduledTask { task_path, .. } => {
                m.insert(format!("task {task_path}"), format!("{:?}", b.get_task_enabled(task_path)));
            }
            TweakAction::AppXPackage { package_family } => {
                m.insert(format!("appx {package_family}"), "<not probed>".into());
            }
            TweakAction::WindowsFeature { feature_name, .. } => {
                m.insert(format!("feature {feature_name}"), "<not probed>".into());
            }
        }
    }
    m
}

fn diff(a: &BTreeMap<String, String>, b: &BTreeMap<String, String>) -> Vec<String> {
    a.iter()
        .filter(|(k, v)| b.get(*k) != Some(*v))
        .map(|(k, v)| format!("    {k}\n      before: {v}\n      after:  {}", b.get(k).map(String::as_str).unwrap_or("<missing>")))
        .collect()
}

fn testable(t: &TweakDefinition) -> bool {
    let features = std::env::var("WINBLAST_VM_FEATURES").ok().as_deref() == Some("1");
    t.valuation.reversibility != Reversibility::RequiresPackageReinstall
        && t.actions.iter().all(|a| match a {
            TweakAction::AppXPackage { .. } => false,
            TweakAction::WindowsFeature { .. } => features,
            _ => true,
        })
}

fn temp_ledger(tag: &str) -> (PathBuf, StateLedger) {
    let p = std::env::temp_dir().join(format!("wb_aggr_{tag}_{}.db", uuid::Uuid::new_v4()));
    let l = StateLedger::new(&p).expect("ledger");
    (p, l)
}

fn rollback_all(ledger: &StateLedger, b: &RealBackend) -> Vec<String> {
    let mut errs = Vec::new();
    let mut h: Vec<LedgerEntry> = ledger.get_history().expect("history");
    h.sort_by_key(|e| std::cmp::Reverse(e.id));
    for e in h.iter().filter(|e| !e.rolled_back) {
        if let Err(err) = rollback_entry_with(e, ledger, b) {
            errs.push(format!("rollback #{} {} failed: {err}", e.id, e.action_type));
        }
    }
    errs
}

/// A. Every compatible catalog tweak, for real: apply, scan, apply again, roll back
/// everything; the touched keys/services/tasks/power settings must be byte-identical.
/// Then apply, revert to defaults, roll back the revert: must equal the applied state.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_real_catalog_apply_rollback_is_byte_exact() {
    if !guard() {
        return;
    }
    let b = RealBackend;
    let os = b.detect_os().expect("os");
    let tweaks = load_embedded_rules();
    let only = std::env::var("WINBLAST_ONLY").ok();
    let (mut ran, mut skipped, mut na) = (0, 0, 0);
    let mut fail = Vec::new();
    let mut warn = Vec::new();
    for t in &tweaks {
        if only.as_deref().is_some_and(|o| !t.id.contains(o)) {
            continue;
        }
        if !testable(t) {
            skipped += 1;
            continue;
        }
        let s0 = scan_single_tweak_with(&b, t, os.build_number, &os.edition);
        if s0.current_state == TweakState::NotApplicable {
            na += 1;
            continue;
        }
        ran += 1;
        let before = probe(t, &b);
        let (path, ledger) = temp_ledger("cat");
        let started = Instant::now();
        if let Err(e) = apply_tweak_with(t, &ledger, &b) {
            let d = diff(&before, &probe(t, &b));
            if d.is_empty() {
                warn.push(format!("[{}] apply error (state untouched, OK): {e}", t.id));
            } else {
                fail.push(format!("[{}] apply failed AND left changes behind: {e}\n{}", t.id, d.join("\n")));
                rollback_all(&ledger, &b);
            }
            drop(ledger);
            let _ = std::fs::remove_file(&path);
            continue;
        }
        let applied = probe(t, &b);
        let s1 = scan_single_tweak_with(&b, t, os.build_number, &os.edition);
        if s1.current_state != TweakState::AlreadyOptimized {
            let live: Vec<String> = t
                .actions
                .iter()
                .filter_map(|a| match a {
                    TweakAction::ScheduledTask { task_path, enable } => Some(format!(
                        "task {task_path}: want enabled={enable}, live={:?}, query={:?}",
                        b.get_task_enabled(task_path),
                        winblast0::scanner::task_scanner::query_single_task_detailed(task_path)
                    )),
                    TweakAction::ServiceState { service_name, target_start_type, .. } => Some(format!(
                        "service {service_name}: want {target_start_type}, live={:?}",
                        b.get_service_start_type(service_name)
                    )),
                    _ => None,
                })
                .collect();
            fail.push(format!(
                "[{}] applied OK but scan says {:?} ({}): the UI would show it as not applied\n    {}",
                t.id,
                s1.current_state,
                s1.status_detail,
                live.join("\n    ")
            ));
        }
        if s0.current_state != TweakState::AlreadyOptimized && diff(&before, &applied).is_empty() {
            warn.push(format!("[{}] apply reported success but nothing observable changed (scan before: {:?})", t.id, s0.current_state));
        }
        if let Err(e) = apply_tweak_with(t, &ledger, &b) {
            fail.push(format!("[{}] second apply (idempotence) failed: {e}", t.id));
        }
        if probe(t, &b) != applied {
            fail.push(format!("[{}] second apply changed state again (not idempotent)", t.id));
        }
        let errs = rollback_all(&ledger, &b);
        let d = diff(&before, &probe(t, &b));
        if !errs.is_empty() || !d.is_empty() {
            fail.push(format!("[{}] rollback not exact ({} errors)\n{}\n{}", t.id, errs.len(), errs.join("\n"), d.join("\n")));
        }
        if matches!(t.valuation.reversibility, Reversibility::Instant | Reversibility::RequiresReboot | Reversibility::RequiresLogoff)
            && apply_tweak_with(t, &ledger, &b).is_ok()
        {
            let applied2 = probe(t, &b);
            match revert_tweak_with(t, &ledger, &b) {
                Err(e) => fail.push(format!("[{}] revert to defaults failed: {e}", t.id)),
                Ok(_) => {
                    let dr = diff(&before, &probe(t, &b));
                    if !dr.is_empty() {
                        warn.push(format!("[{}] REVERT-NOT-ORIGINAL: un-toggling writes catalog defaults, not the user's prior state\n{}", t.id, dr.join("\n")));
                    }
                    let mut revs: Vec<LedgerEntry> = ledger
                        .get_history()
                        .expect("history")
                        .into_iter()
                        .filter(|e| !e.rolled_back && e.action_type.starts_with("Revert:"))
                        .collect();
                    revs.sort_by_key(|e| std::cmp::Reverse(e.id));
                    for e in &revs {
                        if let Err(err) = rollback_entry_with(e, &ledger, &b) {
                            fail.push(format!("[{}] rollback of revert #{} failed: {err}", t.id, e.id));
                        }
                    }
                    let d2 = diff(&applied2, &probe(t, &b));
                    if !d2.is_empty() {
                        fail.push(format!("[{}] undoing a revert did not restore the applied state\n{}", t.id, d2.join("\n")));
                    }
                }
            }
            let errs = rollback_all(&ledger, &b);
            let d3 = diff(&before, &probe(t, &b));
            if !errs.is_empty() || !d3.is_empty() {
                fail.push(format!("[{}] final rollback not exact\n{}\n{}", t.id, errs.join("\n"), d3.join("\n")));
            }
        }
        if started.elapsed() > Duration::from_secs(60) {
            warn.push(format!("[{}] slow: {:?}", t.id, started.elapsed()));
        }
        drop(ledger);
        let _ = std::fs::remove_file(&path);
    }
    eprintln!("catalog: {} tweaks, ran {ran}, not applicable {na}, skipped (AppX/feature) {skipped}", tweaks.len());
    eprintln!("WARNINGS ({}):\n{}", warn.len(), warn.join("\n"));
    assert!(fail.is_empty(), "FAILURES ({}):\n{}", fail.len(), fail.join("\n"));
}

/// Child half of the crash test: applies tweaks one after another until killed.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_crash_child() {
    let Ok(db) = std::env::var("WINBLAST_CRASH_LEDGER") else {
        return;
    };
    let b = RealBackend;
    let ledger = StateLedger::new(Path::new(&db)).expect("ledger");
    let os = b.detect_os().expect("os");
    for t in load_embedded_rules().iter().filter(|t| testable(t)) {
        if scan_single_tweak_with(&b, t, os.build_number, &os.edition).current_state == TweakState::NotApplicable {
            continue;
        }
        let _ = apply_tweak_with(t, &ledger, &b);
    }
    std::thread::sleep(Duration::from_secs(600));
}

/// B. Kill the process at several points during a batch apply, then recover from the ledger
/// alone. Everything the batch touched must be back to the original bytes.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_crash_mid_apply_recovers_from_ledger() {
    if !guard() {
        return;
    }
    let b = RealBackend;
    let os = b.detect_os().expect("os");
    let tweaks: Vec<TweakDefinition> = load_embedded_rules()
        .into_iter()
        .filter(testable)
        .filter(|t| scan_single_tweak_with(&b, t, os.build_number, &os.edition).current_state != TweakState::NotApplicable)
        .collect();
    let snapshot = |b: &RealBackend| -> BTreeMap<String, String> {
        let mut all = BTreeMap::new();
        for t in &tweaks {
            all.extend(probe(t, b));
        }
        all
    };
    let before = snapshot(&b);
    let exe = std::env::current_exe().expect("exe");
    let mut fail = Vec::new();
    for (round, kill_ms) in [150u64, 400, 900, 1700, 3000, 5200].into_iter().enumerate() {
        let db = std::env::temp_dir().join(format!("wb_crash_{round}_{}.db", uuid::Uuid::new_v4()));
        let mut child = Command::new(&exe)
            .args(["vm_crash_child", "--exact", "--include-ignored", "--test-threads=1"])
            .env("WINBLAST_CRASH_LEDGER", &db)
            .spawn()
            .expect("spawn child");
        std::thread::sleep(Duration::from_millis(kill_ms));
        let _ = child.kill(); // TerminateProcess: no destructors, no flush, like a real crash
        let _ = child.wait();
        let ledger = StateLedger::new(&db).expect("ledger must reopen after a crash");
        let n = ledger.get_history().map(|h| h.len()).unwrap_or(0);
        let errs = rollback_all(&ledger, &b);
        let d = diff(&before, &snapshot(&b));
        eprintln!("round {round}: killed after {kill_ms} ms, {n} ledger entries, {} rollback errors, {} keys differ", errs.len(), d.len());
        if !errs.is_empty() || !d.is_empty() {
            fail.push(format!("round {round} (kill at {kill_ms} ms, {n} entries):\n{}\n{}", errs.join("\n"), d.join("\n")));
            // restore by brute force so later rounds start clean: roll back again is all we have
        }
        drop(ledger);
        let _ = std::fs::remove_file(&db);
    }
    assert!(fail.is_empty(), "crash recovery left the system changed:\n{}", fail.join("\n"));
}

/// Writes a valid database whose ledger table has a foreign column name.
fn foreign_schema_db(p: &Path) {
    let tmp = p.with_extension("tmp.db");
    {
        let l = StateLedger::new(&tmp).expect("tmp ledger");
        l.record_change("x", "y", "z", "w").expect("rec");
    }
    let mut bytes = std::fs::read(&tmp).expect("read");
    let needle = b"rolled_back";
    if let Some(pos) = bytes.windows(needle.len()).position(|w| w == needle) {
        bytes[pos..pos + needle.len()].copy_from_slice(b"rolled_xxxx");
    }
    std::fs::write(p, bytes).expect("write");
    let _ = std::fs::remove_file(&tmp);
}

type Maker = Box<dyn Fn(&Path)>;

/// C. Corrupt, truncated, foreign, locked and read-only ledger files must never panic.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_ledger_corruption_is_survivable() {
    if !guard() {
        return;
    }
    let dir = std::env::temp_dir().join(format!("wb_corrupt_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).expect("dir");
    let mut report = Vec::new();
    let mut panics = Vec::new();
    let good = dir.join("good.db");
    {
        let l = StateLedger::new(&good).expect("good");
        for i in 0..200 {
            l.record_change("t", "RegistryDword:HKCU|Software\\X|v", &i.to_string(), "1").expect("rec");
        }
    }
    let g = std::fs::read(&good).unwrap_or_default();
    let (g1, g2, g3, g4) = (g.clone(), g.clone(), g.clone(), g.clone());
    let cases: Vec<(&str, Maker)> = vec![
        ("zero bytes", Box::new(|p| std::fs::write(p, b"").expect("w"))),
        ("random garbage", Box::new(|p| std::fs::write(p, (0..65536u32).map(|i| (i.wrapping_mul(2_654_435_761) >> 13) as u8).collect::<Vec<_>>()).expect("w"))),
        ("truncated half", Box::new(move |p| std::fs::write(p, &g1[..g1.len() / 2]).expect("w"))),
        ("header only", Box::new(move |p| std::fs::write(p, &g2[..100.min(g2.len())]).expect("w"))),
        ("bit flips", Box::new(move |p| {
            let mut v = g3.clone();
            for i in (200..v.len()).step_by(97) {
                v[i] ^= 0x5a;
            }
            std::fs::write(p, v).expect("w")
        })),
        ("text file", Box::new(|p| std::fs::write(p, "this is not a database\n").expect("w"))),
        ("is a directory", Box::new(|p| std::fs::create_dir_all(p).expect("d"))),
        ("read-only good db", Box::new(move |p| {
            std::fs::write(p, &g4).expect("w");
            let mut perm = std::fs::metadata(p).expect("m").permissions();
            perm.set_readonly(true);
            std::fs::set_permissions(p, perm).expect("ro");
        })),
        ("foreign schema", Box::new(foreign_schema_db)),
    ];
    for (name, make) in &cases {
        let p = dir.join(format!("{}.db", name.replace(' ', "_")));
        make(&p);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match StateLedger::new(&p) {
            Err(e) => format!("open -> Err({e})"),
            Ok(l) => {
                let h = l.get_history().map(|h| h.len());
                let w = l.record_change("probe", "RegistryDword:HKCU|Software\\X|v", "0", "1");
                format!("open -> Ok; history={h:?}; write={:?}", w.map(|_| ()))
            }
        }));
        match r {
            Ok(s) => report.push(format!("{name}: {s}")),
            Err(_) => panics.push(name.to_string()),
        }
    }
    {
        let a = StateLedger::new(&good).expect("a");
        let b2 = StateLedger::new(&good).expect("b");
        let t = std::thread::spawn(move || {
            let mut e = 0;
            for i in 0..300 {
                if a.record_change("a", "RegistryDword:HKCU|Software\\X|v", &i.to_string(), "1").is_err() {
                    e += 1;
                }
            }
            e
        });
        let mut errs = 0;
        for i in 0..300 {
            if b2.record_change("b", "RegistryDword:HKCU|Software\\X|v", &i.to_string(), "1").is_err() {
                errs += 1;
            }
        }
        let errs_a = t.join().expect("thread");
        let total = StateLedger::new(&good).expect("c").count_history().unwrap_or(0);
        report.push(format!("two concurrent writers: {errs_a}+{errs}/600 writes failed; rows {total} (expected {})", 800 - errs - errs_a));
    }
    let _ = std::fs::remove_dir_all(&dir);
    eprintln!("LEDGER CORRUPTION REPORT:\n{}", report.join("\n"));
    assert!(panics.is_empty(), "StateLedger panicked on: {panics:?}");
}

/// D. Threads with their own ledger connection on ONE file (as the app's workers do)
/// hammer apply/rollback of disjoint HKCU tweaks. Must end byte-identical.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_concurrent_apply_rollback_stress() {
    if !guard() {
        return;
    }
    let b = RealBackend;
    let os = b.detect_os().expect("os");
    let hkcu_only = |t: &TweakDefinition| {
        t.actions.iter().all(|a| match a {
            TweakAction::RegistryDword { hive, .. }
            | TweakAction::RegistryString { hive, .. }
            | TweakAction::RegistryDelete { hive, .. }
            | TweakAction::RegistryBinaryBit { hive, .. } => hive.eq_ignore_ascii_case("HKCU"),
            _ => false,
        })
    };
    let mut seen = std::collections::HashSet::new();
    let picked: Vec<TweakDefinition> = load_embedded_rules()
        .into_iter()
        .filter(testable)
        .filter(|t| hkcu_only(t))
        .filter(|t| scan_single_tweak_with(&b, t, os.build_number, &os.edition).current_state != TweakState::NotApplicable)
        .filter(|t| probe(t, &b).keys().all(|k| seen.insert(k.to_ascii_lowercase())))
        .take(16)
        .collect();
    assert!(picked.len() >= 4, "need some HKCU tweaks, found {}", picked.len());
    let before: Vec<_> = picked.iter().map(|t| probe(t, &b)).collect();
    let db = std::env::temp_dir().join(format!("wb_conc_{}.db", uuid::Uuid::new_v4()));
    drop(StateLedger::new(&db).expect("init"));
    let iters = 25;
    let handles: Vec<_> = picked
        .iter()
        .cloned()
        .map(|t| {
            let db = db.clone();
            std::thread::spawn(move || {
                let b = RealBackend;
                let l = StateLedger::new(&db).expect("thread ledger");
                let mut errs = Vec::new();
                for _ in 0..iters {
                    if let Err(e) = apply_tweak_with(&t, &l, &b) {
                        errs.push(format!("[{}] apply: {e}", t.id));
                        continue;
                    }
                    let mut mine: Vec<LedgerEntry> = match l.get_history() {
                        Ok(h) => h.into_iter().filter(|e| e.tweak_id == t.id && !e.rolled_back).collect(),
                        Err(e) => {
                            errs.push(format!("[{}] history: {e}", t.id));
                            continue;
                        }
                    };
                    mine.sort_by_key(|e| std::cmp::Reverse(e.id));
                    for e in &mine {
                        if let Err(err) = rollback_entry_with(e, &l, &b) {
                            errs.push(format!("[{}] rollback: {err}", t.id));
                        }
                    }
                }
                errs
            })
        })
        .collect();
    let mut fail: Vec<String> = handles.into_iter().flat_map(|h| h.join().expect("thread panicked")).collect();
    for (t, b0) in picked.iter().zip(&before) {
        let d = diff(b0, &probe(t, &b));
        if !d.is_empty() {
            fail.push(format!("[{}] state differs after stress\n{}", t.id, d.join("\n")));
        }
    }
    let l = StateLedger::new(&db).expect("reopen");
    let left = l.get_history().expect("h").iter().filter(|e| !e.rolled_back).count();
    if left != 0 {
        fail.push(format!("{left} ledger rows not marked rolled back"));
    }
    drop(l);
    let _ = std::fs::remove_file(&db);
    eprintln!("stress: {} tweaks x {iters} iterations", picked.len());
    assert!(fail.is_empty(), "concurrency failures ({}):\n{}", fail.len(), fail.join("\n"));
}

/// E. Scan fingerprint: every tweak's scan state plus live probes, as JSON, so runs under
/// different user locales and non-admin tokens can be diffed by the harness.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "reads the real host; VM only")]
fn vm_scan_fingerprint() {
    let Ok(out) = std::env::var("WINBLAST_FINGERPRINT_OUT") else {
        return;
    };
    let b = RealBackend;
    let os = b.detect_os().expect("os");
    let mut m = BTreeMap::new();
    for t in &load_embedded_rules() {
        let s = scan_single_tweak_with(&b, t, os.build_number, &os.edition);
        m.insert(format!("scan {}", t.id), format!("{:?}", s.current_state));
    }
    m.insert("os".into(), format!("{} {} {}", os.build_number, os.edition, os.is_win11));
    m.insert("elevated".into(), b.is_elevated().to_string());
    m.insert("ram_kb".into(), b.total_physical_ram_kb().to_string());
    m.insert("cores".into(), b.logical_core_count().to_string());
    m.insert("power active".into(), format!("{:?}", b.get_active_power_scheme().ok()));
    m.insert("power schemes".into(), format!("{:?}", b.list_power_schemes()));
    m.insert("hibernation".into(), format!("{:?}", b.query_hibernation_status()));
    m.insert("gpus".into(), format!("{:?}", b.detect_gpus()));
    m.insert("cpu".into(), format!("{:?}", b.detect_cpu()));
    m.insert("parking".into(), format!("{:?}", b.detect_core_parking()));
    m.insert("security".into(), format!("{:?}", b.detect_platform_security()));
    let json = serde_json::to_string_pretty(&m).expect("json");
    std::fs::write(&out, json).expect("write fingerprint");
}

fn utf16z(s: &str) -> Vec<u8> {
    s.encode_utf16().chain(std::iter::once(0)).flat_map(u16::to_le_bytes).collect()
}

/// F. A tweak's target value already exists with a foreign type or data (REG_SZ text under a
/// DWORD tweak, REG_QWORD, REG_BINARY, REG_MULTI_SZ, or a DWORD under a string tweak).
/// apply + roll back must restore exactly that foreign value, type included.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "mutates the real host; VM only")]
fn vm_foreign_prior_types_survive_round_trip() {
    if !guard() {
        return;
    }
    let b = RealBackend;
    let os = b.detect_os().expect("os");
    let priors: Vec<(&str, RegType, Vec<u8>)> = vec![
        ("REG_SZ text", REG_SZ, utf16z("not-a-number")),
        ("REG_SZ numeric", REG_SZ, utf16z("42")),
        ("REG_QWORD", REG_QWORD, 0x1_0000_0001u64.to_le_bytes().to_vec()),
        ("REG_BINARY", REG_BINARY, vec![1, 2, 3]),
        ("REG_MULTI_SZ", REG_MULTI_SZ, [utf16z("a"), utf16z("b"), vec![0, 0]].concat()),
        ("REG_DWORD 7", REG_DWORD, 7u32.to_le_bytes().to_vec()),
        ("REG_EXPAND_SZ", REG_EXPAND_SZ, utf16z("%SystemRoot%\\x")),
    ];
    let mut targets = Vec::new();
    for t in load_embedded_rules().into_iter().filter(testable) {
        if t.actions.len() != 1 || !t.fallbacks.is_empty() {
            continue;
        }
        let (h, p, n) = match &t.actions[0] {
            TweakAction::RegistryDword { hive, path, name, .. } | TweakAction::RegistryString { hive, path, name, .. } => (hive.clone(), path.clone(), name.clone()),
            _ => continue,
        };
        if p.contains('<') || hive(&h).is_none() {
            continue;
        }
        if scan_single_tweak_with(&b, &t, os.build_number, &os.edition).current_state == TweakState::NotApplicable {
            continue;
        }
        targets.push((t, h, p, n));
        if targets.len() >= 60 {
            break;
        }
    }
    let mut fail = Vec::new();
    let mut runs = 0;
    for (t, h, p, n) in &targets {
        let root = hive(h).expect("hive");
        let (key, disp) = match root.create_subkey_with_flags(p, KEY_ALL_ACCESS | KEY_WOW64_64KEY) {
            Ok(k) => k,
            Err(e) => {
                fail.push(format!("[{}] cannot open {h}\\{p}: {e}", t.id));
                continue;
            }
        };
        let original = key.get_raw_value(n).ok();
        for (label, vtype, bytes) in &priors {
            let seeded = winreg::RegValue { bytes: bytes.clone(), vtype: vtype.clone() };
            if key.set_raw_value(n, &seeded).is_err() {
                continue;
            }
            runs += 1;
            let before = probe(t, &b);
            let (path, ledger) = temp_ledger("foreign");
            let applied = apply_tweak_with(t, &ledger, &b);
            let errs = rollback_all(&ledger, &b);
            let d = diff(&before, &probe(t, &b));
            if !errs.is_empty() || !d.is_empty() {
                fail.push(format!("[{}] prior {label}: apply={:?}; rollback errors {:?}\n{}", t.id, applied.map(|_| ()), errs, d.join("\n")));
            }
            drop(ledger);
            let _ = std::fs::remove_file(&path);
        }
        match &original {
            Some(v) => {
                let _ = key.set_raw_value(n, v);
            }
            None => {
                let _ = key.delete_value(n);
            }
        }
        drop(key);
        if disp == REG_CREATED_NEW_KEY {
            let _ = root.delete_subkey_with_flags(p, KEY_WOW64_64KEY);
        }
    }
    eprintln!("foreign priors: {} tweaks, {runs} seeded round trips, {} failures", targets.len(), fail.len());
    assert!(fail.is_empty(), "FOREIGN-TYPE FAILURES ({}):\n{}", fail.len(), fail.join("\n"));
}

/// G. A rules file planted by a standard user in the working directory (or C:\assets\rules for
/// an exe under Program Files) must not replace the embedded catalog of an elevated app.
#[test]
#[cfg_attr(not(feature = "vm-tests"), ignore = "changes the working directory; VM only")]
fn vm_planted_rules_file_is_not_loaded() {
    if !guard() {
        return;
    }
    let dir = std::env::temp_dir().join(format!("wb_planted_{}", uuid::Uuid::new_v4()));
    let rules = dir.join("assets").join("rules");
    std::fs::create_dir_all(&rules).expect("dir");
    std::fs::write(
        rules.join("planted.toml"),
        r#"[[tweaks]]
id = "planted_probe"
title = "PLANTED"
category = "Privacy"
description = "planted by the VM test"
easy_mode_included = true
[tweaks.os_req]
min_build = 0
editions = ["Home", "Pro", "Enterprise", "Education"]
[tweaks.valuation]
broken_features = []
performance_gain = 1
privacy_gain = 1
stability_risk = "Safe"
reversibility = "Instant"
[[tweaks.actions]]
type = "RegistryDword"
hive = "HKCU"
path = "Software\\WinBlastPlantedProbe"
name = "x"
optimized_value = 1
default_value = 0
"#,
    )
    .expect("write");
    let old = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(&dir).expect("cd");
    let loaded = winblast0::core::risk::load_rules_with_fallback(Some(Path::new("assets/rules")));
    std::env::set_current_dir(old).expect("cd back");
    let planted = loaded.iter().any(|t| t.id == "planted_probe");
    // the planted file must itself be a valid catalog, otherwise this test proves nothing
    let direct = winblast0::core::risk::load_rules_from_dir(&rules).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(direct.iter().any(|t| t.id == "planted_probe"), "the planted test catalog does not parse");
    eprintln!("rules loaded from planted dir: {} tweaks, planted present: {planted}", loaded.len());
    assert!(!planted, "SECURITY: a rules file in the working directory replaced the embedded catalog ({} tweaks loaded, planted tweak present)", loaded.len());
}
