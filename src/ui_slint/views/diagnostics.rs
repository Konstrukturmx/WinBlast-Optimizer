//! Fault Diagnostics, Windows Repair and Performance Tests binders.

use std::cell::Cell;
use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::app::AppState;
use crate::core::view_i18n::ViewTextKey;
use crate::tools::diagnostics::FaultAction;

use super::super::ctx::Ctx;
use super::super::models::{new_model, sync_vec};
use super::super::{FaultStore, MainWindow, PerfStore, RepairRow, RepairStore, TitledRow};

pub const ISSUES_URL: &str = "https://github.com/Konstrukturmx/WinBlast-Optimizer/issues/new";

pub(crate) fn open_external(target: &str) {
    #[cfg(windows)]
    {
        let _ = crate::core::process::SilentCommand::new("rundll32").args(["url.dll,FileProtocolHandler", target]).spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = target;
    }
}

pub struct DiagnosticsView {
    files: Rc<VecModel<SharedString>>,
    dumps: Rc<VecModel<TitledRow>>,
    tasks: Rc<VecModel<RepairRow>>,
    log: Rc<VecModel<SharedString>>,
    /// Restart needs a second click within the same visit (v1.6.1 rebooted on the first click).
    restart_armed: Rc<Cell<bool>>,
}

impl DiagnosticsView {
    pub fn new(window: &MainWindow) -> Self {
        let f = window.global::<FaultStore>();
        let r = window.global::<RepairStore>();
        let (files, a) = new_model();
        let (dumps, b) = new_model();
        let (tasks, c) = new_model();
        let (log, d) = new_model();
        f.set_pending_files(a);
        f.set_minidumps(b);
        r.set_tasks(c);
        r.set_log(d);
        Self { files, dumps, tasks, log, restart_armed: Rc::new(Cell::new(false)) }
    }

    pub fn bind(&self, window: &MainWindow, ctx: &Ctx) {
        let f = window.global::<FaultStore>();
        let c = ctx.clone();
        f.on_rescan(move || {
            c.act(|s| s.scan_fault_report_async());
        });
        let c = ctx.clone();
        f.on_clear_dumps(move || {
            c.act(|s| s.run_fault_action_async(FaultAction::ClearMinidumps));
        });
        let c = ctx.clone();
        f.on_clear_renames(move || {
            c.act(|s| s.run_fault_action_async(FaultAction::ClearPendingRenames));
        });
        let c = ctx.clone();
        f.on_clear_cbs(move || {
            c.act(|s| s.run_fault_action_async(FaultAction::ClearCbsRebootFlags));
        });
        let (c, armed, w) = (ctx.clone(), self.restart_armed.clone(), window.as_weak());
        f.on_restart_now(move || {
            if armed.get() {
                armed.set(false);
                c.act(|s| s.run_fault_action_async(FaultAction::RestartSystem));
            } else {
                armed.set(true);
            }
            if let Some(w) = w.upgrade() {
                w.global::<FaultStore>().set_confirm_restart(armed.get());
            }
        });
        f.on_report_github(|| open_external(ISSUES_URL));
        f.on_open_minidump_folder(|| open_external(r"C:\Windows\Minidump"));

        let r = window.global::<RepairStore>();
        let c = ctx.clone();
        r.on_run(move |id| {
            c.act(|s| s.run_repair_task_async(id.as_str()));
        });
        let c = ctx.clone();
        r.on_clear_log(move || {
            c.act(|s| s.repair_log.clear());
        });
        let c = ctx.clone();
        window.global::<PerfStore>().on_start(move || {
            c.act(|s| s.start_perf_benchmark_async());
        });
    }

    /// Forget a pending restart confirmation (called when the user leaves the view).
    pub fn disarm(&self) {
        self.restart_armed.set(false);
    }

    pub fn sync_fault(&self, window: &MainWindow, s: &AppState) {
        let f = window.global::<FaultStore>();
        f.set_scanning(s.fault_scanning);
        f.set_action_busy(s.fault_action_inflight);
        f.set_confirm_restart(self.restart_armed.get());
        let r = s.fault_report.clone().unwrap_or_default();
        f.set_critical(r.total_critical_faults as i32);
        let pr = &r.pending_reboot;
        f.set_pending(pr.is_pending);
        let yn = |b: bool| if b { s.tr_v(ViewTextKey::FaultYes) } else { s.tr_v(ViewTextKey::FaultNo) };
        f.set_pending_summary(
            format!(
                "{}: {} • {}: {} • {}: {}",
                s.tr_v(ViewTextKey::FaultPendingRebootCbs),
                yn(pr.cbs_reboot),
                s.tr_v(ViewTextKey::FaultPendingRebootWu),
                yn(pr.windows_update_reboot),
                s.tr_v(ViewTextKey::FaultPendingRebootRename),
                pr.pending_file_count
            )
            .into(),
        );
        f.set_pending_rename(pr.pending_file_rename);
        f.set_cbs_or_wu(pr.cbs_reboot || pr.windows_update_reboot);
        sync_vec(&self.files, pr.pending_files.iter().take(6).map(|p| SharedString::from(format!("• {p}"))).collect());
        f.set_pending_more(if pr.pending_files.len() > 6 {
            s.tr_v(ViewTextKey::FaultMorePendingFiles).replace("{}", &(pr.pending_files.len() - 6).to_string()).into()
        } else {
            SharedString::new()
        });
        sync_vec(
            &self.dumps,
            r.minidumps
                .iter()
                .map(|d| TitledRow {
                    title: d.file_name.as_str().into(),
                    sub: format!("{}: {} KB • {}: {}", s.tr_v(ViewTextKey::FaultSize), d.size_kb, s.tr_v(ViewTextKey::FaultCreated), d.created_at).into(),
                    tone: "danger".into(),
                })
                .collect(),
        );
    }

    pub fn sync_repair(&self, window: &MainWindow, s: &AppState) {
        let r = window.global::<RepairStore>();
        r.set_running(s.repair_running.clone().unwrap_or_default().into());
        sync_vec(
            &self.tasks,
            s.repair_tasks
                .iter()
                .map(|t| RepairRow {
                    id: t.id.into(),
                    icon: t.icon_glyph.into(),
                    title: t.title.into(),
                    command: format!("{}: {}", s.tr_v(ViewTextKey::RepairCommandLabel), t.command_preview).into(),
                    desc: t.description.into(),
                })
                .collect(),
        );
        sync_vec(&self.log, s.repair_log.iter().map(|l| SharedString::from(l.as_str())).collect());
    }

    pub fn sync_perf(&self, window: &MainWindow, s: &AppState) {
        let p = window.global::<PerfStore>();
        p.set_running(s.is_benchmarking_perf);
        let pts = s.tr_v(ViewTextKey::PerfPoints);
        match &s.perf_score {
            Some(sc) if sc.total_composite_score > 0 => {
                p.set_has_score(true);
                p.set_composite(format!("{} {pts}", sc.total_composite_score).into());
                p.set_cpu_multi(format!("{} {}", sc.cpu_multi_score, s.tr_v(ViewTextKey::PerfMultiCoreSuffix)).into());
                p.set_cpu_single(format!("1-Core: {} {pts}", sc.cpu_single_score).into());
                p.set_ram_read(format!("{:.1} {}", sc.ram_read_gb_s, s.tr_v(ViewTextKey::PerfThroughputSuffix)).into());
                p.set_ram_latency(format!("Latency: {:.1} ns", sc.ram_latency_ns).into());
                p.set_disk_read(format!("{:.0} {}", sc.disk_seq_read_mb_s, s.tr_v(ViewTextKey::PerfIoSuffix)).into());
            }
            _ => {
                // No run yet (or an empty result): show placeholders, never zeros as if measured.
                p.set_has_score(false);
                p.set_composite("—".into());
                p.set_cpu_multi("—".into());
                p.set_ram_read("—".into());
                p.set_disk_read("—".into());
                p.set_cpu_single(SharedString::new());
                p.set_ram_latency(SharedString::new());
            }
        }
    }
}
