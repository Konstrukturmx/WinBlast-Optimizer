//! Exhaustive localization for dynamic status messages, WinBlastError strings,
//! structured `ScanDetailKind` payloads, SQLite ledger action tokens, and
//! OneDrive Known Folder Move (KFM) diagnostics across all 8 WinBlast0 languages.

use crate::core::i18n::{I18n, Language, TextKey};
use crate::core::risk::FeatureInstallState;
use crate::core::tweak_i18n;
use crate::scanner::ScanDetailKind;

/// Preset identifier decoupled from UI state for localized status messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetKey {
    Gaming,
    ProAudio,
    LaptopBattery,
}

/// Typed, language-neutral specification for every status/progress message in `AppState`.
#[derive(Debug, Clone, PartialEq)]
pub enum StatusMessageSpec {
    ScanningStart,
    ScanCompleted(usize),
    ScanFailed(String),
    LanguageChanged(Language),
    InstantUpdating {
        tweak_id: String,
        tweak_title_fallback: String,
        apply: bool,
    },
    InstantCompleted(String),
    InstantFailed {
        tweak_id: String,
        err: String,
    },
    NothingStaged,
    BatchApplying {
        apply_count: usize,
        revert_count: usize,
    },
    BatchCompleted(usize),
    BatchFailed(String),
    StagedCount(usize),
    SelectedCount(usize),
    DeselectedAll,
    NoCleanupSelected,
    ProfileSaved(String),
    ProfileSaveFailed(String),
    ProfileImported {
        imported: usize,
        skipped: usize,
    },
    ProfileImportFailed(String),
    PresetActivated {
        kind: PresetKey,
        count: usize,
    },
    ModeSwitchedInstant,
    ModeSwitchedNormal,
    ChangelogCleared,
    RestorePointCreating,
    RestorePointCreated,
    RestorePointFailed(String),
    RollingBack(i64),
    RollbackCompleted,
    RollbackFailed(String),
    BaselineResetting,
    BaselineResetCompleted,
    BaselineResetFailed(String),
    GpuMsiUpdated,
    GpuMsiFailed(String),
    HagsUpdated(bool),
    HagsFailed(String),
    MpoUpdated(bool),
    MpoFailed(String),
    LatencyBenchmarkRunning,
    LatencyBenchmarkCompleted,
    LatencyBenchmarkFailed(String),
    LatencyBaselineSaved,
    LatencyRunTestFirst,
    CoreParkingRefreshing,
    CoreParkingUnparking,
    CoreParkingUnparked,
    CoreParkingRestoring,
    CoreParkingRestored,
    CoreParkingFailed {
        restoring: bool,
        err: String,
    },
    CoreParkingPanic(String),
    HostsBlocked(usize),
    HostsFailed(String),
    TelemetryServicesNeutralized(usize),
    TelemetryServicesFailed(String),
    DiskScanning,
    DiskScanned {
        count: usize,
        mb: f64,
    },
    DiskScanFailed(String),
    DiskCleaned {
        mb: f64,
        files: usize,
        skipped: usize,
    },
    DiskCleanFailed(String),
    AppxFeatureScanning,
    AppxFeatureScanned {
        appx: usize,
        features: usize,
    },
    AppxFeatureScanFailed(String),
    AppxUninstalled(usize),
    AppxUninstallFailed(String),
    FeatureUpdated {
        name: String,
        enabled: bool,
    },
    FeatureUpdateFailed {
        name: String,
        err: String,
    },
    OneDriveRemoved,
    EdgeRemoved,
    ActionMessage(String),
    WinGetResult {
        installed: usize,
        total: usize,
        err: Option<String>,
    },
    WinGetFailed(String),
    StartupScanning,
    StartupScanned {
        startup: usize,
        tasks: usize,
        services: usize,
    },
    StartupScanFailed(String),
    StartupToggled {
        name: String,
        enable: bool,
    },
    StartupToggleError {
        name: String,
        err: String,
    },
    TaskToggled {
        path: String,
        enable: bool,
    },
    TaskTriggered(String),
    TaskError {
        path: String,
        err: String,
    },
    ServiceCriticalRefused(String),
    ServiceNotFound(String),
    ServiceUpdated {
        name: String,
        mode: u32,
    },
    ServiceError {
        name: String,
        err: String,
    },
    PowerSchemeActivated(String),
    PowerSchemeFailed(String),
    UltimatePowerActive(String),
    PowerPlanError(String),
    HibernationUpdated(bool),
    HibernationFailed(String),
    UsbPowerUpdated {
        hubs: usize,
        power_saving: bool,
    },
    UsbPowerFailed(String),
    ExplorerRestartedOk,
    ExplorerRestartError(String),
    UnattendExported {
        bytes: usize,
        path: String,
    },
    UnattendExportFailed(String),
    WorkerPanic(String),
    LedgerError(String),
    Raw(String),
}

#[inline]
const fn idx(lang: Language) -> usize {
    lang as usize
}

#[inline]
const fn pick(lang: Language, table: [&'static str; 8]) -> &'static str {
    table[idx(lang)]
}

#[inline]
fn fmt1(lang: Language, table: [&'static str; 8], a: impl std::fmt::Display) -> String {
    pick(lang, table).replacen("{}", &a.to_string(), 1)
}

#[inline]
fn fmt2(
    lang: Language,
    table: [&'static str; 8],
    a: impl std::fmt::Display,
    b: impl std::fmt::Display,
) -> String {
    let s = pick(lang, table).replacen("{}", &a.to_string(), 1);
    s.replacen("{}", &b.to_string(), 1)
}

#[inline]
fn fmt3(
    lang: Language,
    table: [&'static str; 8],
    a: impl std::fmt::Display,
    b: impl std::fmt::Display,
    c: impl std::fmt::Display,
) -> String {
    let s = pick(lang, table).replacen("{}", &a.to_string(), 1);
    let s = s.replacen("{}", &b.to_string(), 1);
    s.replacen("{}", &c.to_string(), 1)
}

const STATIC_NOTHING_STAGED: [&str; 8] = [
    "No tweaks staged to apply or revert.",
    "Uygulanacak veya geri alınacak bekleyen ayar yok.",
    "Keine Tweaks zum Anwenden oder Zurücksetzen vorgemerkt.",
    "Aucun réglage en attente à appliquer ou rétablir.",
    "No hay ajustes preparados para aplicar o revertir.",
    "Нет подготовленных твиков для применения или отката.",
    "没有待应用或还原的优化项。",
    "適用または元に戻す待機中の調整はありません。",
];

const STATIC_DESELECTED_ALL: [&str; 8] = [
    "Deselected all tweaks.",
    "Tüm ayarların seçimi kaldırıldı.",
    "Alle Tweaks abgewählt.",
    "Tous les réglages ont été désélectionnés.",
    "Se deseleccionaron todos los ajustes.",
    "Выбор всех твиков снят.",
    "已取消选择所有优化项。",
    "すべての調整の選択を解除しました。",
];

const STATIC_NO_CLEANUP_SELECTED: [&str; 8] = [
    "No cleanup targets selected.",
    "Temizlenecek hedef seçilmedi.",
    "Keine Bereinigungsziele ausgewählt.",
    "Aucune cible de nettoyage sélectionnée.",
    "No se seleccionaron objetivos de limpieza.",
    "Цели очистки не выбраны.",
    "未选择清理目标。",
    "クリーンアップ対象が選択されていません。",
];

const STATIC_MODE_INSTANT: [&str; 8] = [
    "Switched to Instant Mode. Changes apply immediately upon switch toggle.",
    "Anında Mod etkinleştirildi. Anahtar değiştirildiğinde ayarlar hemen uygulanır.",
    "Sofortmodus aktiviert. Änderungen werden beim Umschalten sofort angewendet.",
    "Mode instantané activé. Les changements s'appliquent immédiatement au basculement.",
    "Modo instantáneo activado. Los cambios se aplican al pulsar el interruptor.",
    "Включён мгновенный режим. Изменения применяются сразу при переключении.",
    "已切换至即时模式。切换开关将立即应用或还原更改。",
    "即時モードに切り替えました。スイッチ操作で即座に適用されます。",
];

const STATIC_MODE_NORMAL: [&str; 8] = [
    "Switched to Normal Mode. Staged changes require 'Apply Selected'.",
    "Normal Mod etkinleştirildi. Bekleyen değişiklikler için 'Seçilenleri Uygula' gerekir.",
    "Normalmodus aktiviert. Vorgemerkte Änderungen erfordern 'Ausgewählte anwenden'.",
    "Mode normal activé. Les modifications nécessitent 'Appliquer la sélection'.",
    "Modo normal activado. Los cambios preparados requieren 'Aplicar seleccionados'.",
    "Включён обычный режим. Подготовленные изменения требуют «Применить выбранные».",
    "已切换至常规模式。暂存的更改需点击“应用所选”生效。",
    "通常モードに切り替えました。保留中の変更は「選択項目を適用」で確定します。",
];

const STATIC_CHANGELOG_CLEARED: [&str; 8] = [
    "Changelog and transaction ledger cleared.",
    "Değişiklik günlüğü ve işlem defteri temizlendi.",
    "Änderungsprotokoll und Transaktionsdatenbank geleert.",
    "Journal des modifications et registre transactionnel effacés.",
    "Historial de cambios y registro transaccional limpiados.",
    "Журнал изменений и транзакций очищен.",
    "已清空变更日志与事务账本。",
    "変更履歴とトランザクション台帳を消去しました。",
];

const STATIC_RESTORE_CREATING: [&str; 8] = [
    "Creating Windows System Restore point via VSS...",
    "VSS üzerinden Windows Sistem Geri Yükleme noktası oluşturuluyor...",
    "Windows-Systemwiederherstellungspunkt wird über VSS erstellt...",
    "Création du point de restauration système Windows via VSS...",
    "Creando punto de restauración del sistema Windows mediante VSS...",
    "Создание точки восстановления системы Windows через VSS...",
    "正在通过 VSS 创建 Windows 系统还原点...",
    "VSS 経由で Windows システムの復元ポイントを作成しています...",
];

const STATIC_RESTORE_CREATED: [&str; 8] = [
    "System Restore point created successfully.",
    "Sistem Geri Yükleme noktası başarıyla oluşturuldu.",
    "Systemwiederherstellungspunkt erfolgreich erstellt.",
    "Point de restauration système créé avec succès.",
    "Punto de restauración del sistema creado correctamente.",
    "Точка восстановления системы успешно создана.",
    "系统还原点创建成功。",
    "システムの復元ポイントが正常に作成されました。",
];

const STATIC_ROLLBACK_COMPLETED: [&str; 8] = [
    "Mutation rolled back successfully. Refreshing state...",
    "Değişiklik başarıyla geri alındı. Durum yenileniyor...",
    "Änderung erfolgreich zurückgesetzt. Status wird aktualisiert...",
    "Modification annulée avec succès. Actualisation de l'état...",
    "Cambio revertido correctamente. Actualizando estado...",
    "Изменение успешно откачено. Обновление состояния...",
    "已成功回滚更改。正在刷新状态...",
    "変更を正常にロールバックしました。状態を更新しています...",
];

const STATIC_BASELINE_RESETTING: [&str; 8] = [
    "Resetting performance baseline...",
    "Performans temel çizgisi sıfırlanıyor...",
    "Leistungsbasislinie wird zurückgesetzt...",
    "Réinitialisation de la ligne de base des performances...",
    "Restableciendo la línea base de rendimiento...",
    "Сброс базовой линии производительности...",
    "正在重置性能基线...",
    "パフォーマンスベースラインをリセットしています...",
];

const STATIC_BASELINE_RESET_OK: [&str; 8] = [
    "Performance baseline reset to current system state.",
    "Performans temel çizgisi mevcut sistem durumuna sıfırlandı.",
    "Leistungsbasislinie auf aktuellen Systemzustand zurückgesetzt.",
    "Ligne de base réinitialisée sur l'état actuel du système.",
    "Línea base restablecida al estado actual del sistema.",
    "Базовая линия производительности сброшена на текущее состояние системы.",
    "已将性能基线重置为当前系统状态。",
    "パフォーマンスベースラインを現在のシステム状態にリセットしました。",
];

const STATIC_GPU_MSI_OK: [&str; 8] = [
    "PCIe GPU MSI mode updated. Reboot may be required.",
    "PCIe GPU MSI modu güncellendi. Yeniden başlatma gerekebilir.",
    "PCIe-GPU-MSI-Modus aktualisiert. Neustart ggf. erforderlich.",
    "Mode MSI GPU PCIe mis à jour. Un redémarrage peut être requis.",
    "Modo MSI de GPU PCIe actualizado. Puede requerir reinicio.",
    "Режим MSI для PCIe GPU обновлён. Может потребоваться перезагрузка.",
    "PCIe GPU MSI 模式已更新。可能需要重启系统。",
    "PCIe GPU の MSI モードを更新しました。再起動が必要な場合があります。",
];

const STATIC_LATENCY_RUNNING: [&str; 8] = [
    "Running system & network latency benchmark...",
    "Sistem ve ağ gecikme testi çalıştırılıyor...",
    "System- und Netzwerk-Latenzbenchmark läuft...",
    "Exécution du benchmark de latence système et réseau...",
    "Ejecutando prueba de latencia del sistema y de red...",
    "Выполнение теста задержек системы и сети...",
    "正在运行系统与网络延迟基准测试...",
    "システムおよびネットワーク遅延ベンチマークを実行しています...",
];

const STATIC_LATENCY_DONE: [&str; 8] = [
    "Latency benchmark completed.",
    "Gecikme testi tamamlandı.",
    "Latenzbenchmark abgeschlossen.",
    "Benchmark de latence terminé.",
    "Prueba de latencia completada.",
    "Тест задержек завершён.",
    "延迟基准测试已完成。",
    "遅延ベンチマークが完了しました。",
];

const STATIC_LATENCY_BASELINE_SAVED: [&str; 8] = [
    "Current latency snapshot saved as Pre-Optimization Baseline.",
    "Mevcut gecikme anlık görüntüsü Optimizasyon Öncesi Referans olarak kaydedildi.",
    "Aktueller Latenz-Schnappschuss als Vor-Optimierungs-Basislinie gespeichert.",
    "Instantané de latence enregistré comme référence pré-optimisation.",
    "Instantánea de latencia guardada como referencia previa a la optimización.",
    "Текущий снимок задержек сохранён как исходный ориентир.",
    "已将当前延迟快照保存为优化前基线。",
    "現在の遅延スナップショットを最適化前ベースラインとして保存しました。",
];

const STATIC_LATENCY_RUN_FIRST: [&str; 8] = [
    "Please run a latency test first.",
    "Lütfen önce bir gecikme testi çalıştırın.",
    "Bitte führen Sie zuerst einen Latenztest durch.",
    "Veuillez d'abord exécuter un test de latence.",
    "Por favor, ejecute primero una prueba de latencia.",
    "Сначала запустите тест задержек.",
    "请先运行一次延迟测试。",
    "最初に遅延テストを実行してください。",
];

const STATIC_PARKING_REFRESHING: [&str; 8] = [
    "Refreshing CPU core parking telemetry...",
    "CPU çekirdek park etme telemetrisi yenileniyor...",
    "CPU-Core-Parking-Telemetrie wird aktualisiert...",
    "Actualisation de la télémétrie de parcage des cœurs CPU...",
    "Actualizando telemetría de aparcamiento de núcleos de CPU...",
    "Обновление телеметрии парковки ядер ЦП...",
    "正在刷新 CPU 核心休眠状态遥测...",
    "CPU コアパーキングのテレメトリを更新しています...",
];

const STATIC_PARKING_UNPARKING: [&str; 8] = [
    "Unparking all CPU logical cores (setting 100% min/max)...",
    "Tüm mantıksal CPU çekirdekleri uyandırılıyor (min/maks %100)...",
    "Alle logischen CPU-Kerne werden entparkt (Min/Max 100 %)...",
    "Réveil de tous les cœurs logiques CPU (min/max à 100 %)...",
    "Desaparcando todos los núcleos lógicos de CPU (mín/máx 100 %)...",
    "Разпарковка всех логических ядер ЦП (мин./макс. 100%)...",
    "正在唤醒所有逻辑 CPU 核心（设为 100% 最小/最大活跃）...",
    "すべての論理 CPU コアのパーキングを解除しています (最小/最大 100%)...",
];

const STATIC_PARKING_UNPARKED: [&str; 8] = [
    "All CPU cores successfully unparked.",
    "Tüm CPU çekirdeklerinin park durumu başarıyla kaldırıldı.",
    "Alle CPU-Kerne erfolgreich entparkt.",
    "Tous les cœurs CPU ont été réveillés avec succès.",
    "Todos los núcleos de CPU se han desaparcado correctamente.",
    "Все ядра ЦП успешно разпаркованы.",
    "所有 CPU 核心已成功唤醒。",
    "すべての CPU コアのパーキングを正常に解除しました。",
];

const STATIC_PARKING_RESTORING: [&str; 8] = [
    "Restoring default Windows core parking thresholds...",
    "Varsayılan Windows çekirdek park eşikleri geri yükleniyor...",
    "Standard-Windows-Core-Parking-Schwellenwerte werden wiederhergestellt...",
    "Restauration des seuils de parcage de cœurs Windows par défaut...",
    "Restaurando los umbrales predeterminados de aparcamiento de núcleos...",
    "Восстановление стандартных порогов парковки ядер Windows...",
    "正在恢复 Windows 默认核心休眠阈值...",
    "Windows 標準のコアパーキングしきい値を復元しています...",
];

const STATIC_PARKING_RESTORED: [&str; 8] = [
    "Stock core parking thresholds restored.",
    "Varsayılan çekirdek park eşikleri geri yüklendi.",
    "Standard-Core-Parking-Schwellenwerte wiederhergestellt.",
    "Seuils de parcage par défaut restaurés.",
    "Umbrales predeterminados de aparcamiento de núcleos restaurados.",
    "Стандартные пороги парковки ядер восстановлены.",
    "已恢复出厂默认核心休眠阈值。",
    "標準のコアパーキングしきい値を復元しました。",
];

const STATIC_DISK_SCANNING: [&str; 8] = [
    "Scanning disk cleanup targets...",
    "Disk temizleme hedefleri taranıyor...",
    "Datenträgerbereinigungsziele werden gescannt...",
    "Analyse des cibles de nettoyage de disque...",
    "Analizando objetivos de limpieza de disco...",
    "Сканирование целей очистки диска...",
    "正在扫描磁盘清理目标...",
    "ディスククリーンアップ対象をスキャンしています...",
];

const STATIC_APPX_SCANNING: [&str; 8] = [
    "Scanning installed AppX packages & Windows features...",
    "Yüklü AppX paketleri ve Windows özellikleri taranıyor...",
    "Installierte AppX-Pakete & Windows-Features werden gescannt...",
    "Analyse des paquets AppX et fonctionnalités Windows...",
    "Analizando paquetes AppX instalados y características de Windows...",
    "Сканирование пакетов AppX и компонентов Windows...",
    "正在扫描已安装的 AppX 应用包与 Windows 功能...",
    "インストール済み AppX パッケージと Windows 機能をスキャンしています...",
];

const STATIC_STARTUP_SCANNING: [&str; 8] = [
    "Scanning startup apps, scheduled tasks & services...",
    "Başlangıç uygulamaları, zamanlanmış görevler ve hizmetler taranıyor...",
    "Autostart-Apps, geplante Aufgaben & Dienste werden gescannt...",
    "Analyse des applications au démarrage, tâches planifiées et services...",
    "Analizando aplicaciones de inicio, tareas programadas y servicios...",
    "Сканирование автозагрузки, планировщика задач и служб...",
    "正在扫描启动项、计划任务与系统服务...",
    "スタートアップ、スケジュールタスク、サービスをスキャンしています...",
];

const STATIC_ONEDRIVE_REMOVED: [&str; 8] = [
    "OneDrive uninstalled cleanly without KFM data loss.",
    "OneDrive, KFM veri kaybı olmadan temiz şekilde kaldırıldı.",
    "OneDrive sauber ohne KFM-Datenverlust deinstalliert.",
    "OneDrive désinstallé proprement sans perte de données KFM.",
    "OneDrive desinstalado limpiamente sin pérdida de datos KFM.",
    "OneDrive безопасно удалён без потери данных KFM.",
    "已安全卸载 OneDrive，未造成已知文件夹数据丢失。",
    "KFM データを失わずに OneDrive を安全にアンインストールしました。",
];

const STATIC_EDGE_REMOVED: [&str; 8] = [
    "Microsoft Edge removed (WebView2 runtime preserved).",
    "Microsoft Edge kaldırıldı (WebView2 çalışma zamanı korundu).",
    "Microsoft Edge entfernt (WebView2-Laufzeit erhalten).",
    "Microsoft Edge supprimé (WebView2 Runtime préservé).",
    "Microsoft Edge eliminado (se conserva WebView2 Runtime).",
    "Microsoft Edge удалён (среда WebView2 сохранена).",
    "已移除 Microsoft Edge（保留 WebView2 运行时）。",
    "Microsoft Edge を削除しました（WebView2 ランタイムは保持）。",
];

const STATIC_EXPLORER_RESTARTED: [&str; 8] = [
    "Windows Explorer restarted safely under standard desktop token.",
    "Windows Gezgini standart masaüstü belirteciyle güvenli şekilde yeniden başlatıldı.",
    "Windows Explorer sicher mit Standard-Desktop-Token neu gestartet.",
    "Explorateur Windows redémarré en toute sécurité avec le jeton standard.",
    "Explorador de Windows reiniciado de forma segura con token estándar.",
    "Проводник Windows безопасно перезапущен с обычным токеном рабочего стола.",
    "已在标准桌面令牌下安全重启 Windows 资源管理器。",
    "標準デスクトップトークンで Windows エクスプローラーを安全に再起動しました。",
];

/// Localize a [`StatusMessageSpec`] into a complete UI status string for `lang`.
pub fn localize_status(lang: Language, spec: &StatusMessageSpec) -> String {
    match spec {
        StatusMessageSpec::ScanningStart => {
            I18n::translate(lang, TextKey::StatusScanningStart).to_string()
        }
        StatusMessageSpec::ScanCompleted(n) => fmt1(
            lang,
            [
                "Scan completed. Analyzed {} tweaks against system state.",
                "Tarama tamamlandı. Sistem durumuna göre {} ayar analiz edildi.",
                "Scan abgeschlossen. {} Tweaks mit dem Systemzustand abgeglichen.",
                "Analyse terminée. {} réglages évalués par rapport au système.",
                "Análisis completado. {} ajustes evaluados frente al estado del sistema.",
                "Сканирование завершено. Проверено {} твиков относительно состояния системы.",
                "扫描完成。已根据当前系统状态分析 {} 项优化。",
                "スキャン完了。システム状態に対して {} 個の調整を分析しました。",
            ],
            n,
        ),
        StatusMessageSpec::ScanFailed(err) => fmt1(
            lang,
            [
                "System scan failed: {}",
                "Sistem taraması başarısız: {}",
                "Systemscan fehlgeschlagen: {}",
                "Échec de l'analyse système : {}",
                "Error al analizar el sistema: {}",
                "Ошибка сканирования системы: {}",
                "系统扫描失败：{}",
                "システムスキャンに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::LanguageChanged(target) => fmt1(
            lang,
            [
                "Language changed to {}.",
                "Dil {} olarak değiştirildi.",
                "Sprache auf {} geändert.",
                "Langue modifiée en {}.",
                "Idioma cambiado a {}.",
                "Язык изменён на {}.",
                "语言已切换为 {}。",
                "言語を {} に変更しました。",
            ],
            target.label(),
        ),
        StatusMessageSpec::InstantUpdating {
            tweak_id,
            tweak_title_fallback,
            apply,
        } => {
            let title = tweak_i18n::localize_tweak(lang, tweak_id)
                .map(|l| l.title)
                .unwrap_or(tweak_title_fallback.as_str());
            if *apply {
                fmt1(
                    lang,
                    [
                        "Applying {} in real time...",
                        "{} gerçek zamanlı olarak uygulanıyor...",
                        "{} wird in Echtzeit angewendet...",
                        "Application de {} en temps réel...",
                        "Aplicando {} en tiempo real...",
                        "Применение {} в реальном времени...",
                        "正在实时应用 {}...",
                        "{} をリアルタイムで適用しています...",
                    ],
                    title,
                )
            } else {
                fmt1(
                    lang,
                    [
                        "Reverting {} in real time...",
                        "{} gerçek zamanlı olarak geri alınıyor...",
                        "{} wird in Echtzeit zurückgesetzt...",
                        "Rétablissement de {} en temps réel...",
                        "Revirtiendo {} en tiempo real...",
                        "Откат {} в реальном времени...",
                        "正在实时还原 {}...",
                        "{} をリアルタイムで元に戻しています...",
                    ],
                    title,
                )
            }
        }
        StatusMessageSpec::InstantCompleted(tweak_id) => {
            let display = if lang == Language::English {
                tweak_id.as_str()
            } else {
                tweak_i18n::localize_tweak(lang, tweak_id)
                    .map(|l| l.title)
                    .unwrap_or(tweak_id.as_str())
            };
            fmt1(
                lang,
                [
                    "Instant update completed for {}.",
                    "{} için anında güncelleme tamamlandı.",
                    "Sofort-Aktualisierung für {} abgeschlossen.",
                    "Mise à jour instantanée terminée pour {}.",
                    "Actualización instantánea completada para {}.",
                    "Мгновенное обновление выполнено для {}.",
                    "已完成 {} 的即时更新。",
                    "{} の即時更新が完了しました。",
                ],
                display,
            )
        }
        StatusMessageSpec::InstantFailed { tweak_id, err } => fmt2(
            lang,
            [
                "Failed to execute {}: {}",
                "{} çalıştırılamadı: {}",
                "Ausführung von {} fehlgeschlagen: {}",
                "Échec de l'exécution de {} : {}",
                "Error al ejecutar {}: {}",
                "Не удалось выполнить {}: {}",
                "执行 {} 失败：{}",
                "{} の実行に失敗しました: {}",
            ],
            tweak_id,
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::NothingStaged => pick(lang, STATIC_NOTHING_STAGED).to_string(),
        StatusMessageSpec::BatchApplying {
            apply_count,
            revert_count,
        } => fmt2(
            lang,
            [
                "Applying {} optimizations and reverting {} tweaks with transactional backup...",
                "İşlem yedeğiyle {} iyileştirme uygulanıyor ve {} ayar geri alınıyor...",
                "{} Optimierungen werden angewendet und {} Tweaks mit Transaktions-Backup zurückgesetzt...",
                "Application de {} optimisations et rétablissement de {} réglages avec sauvegarde transactionnelle...",
                "Aplicando {} optimizaciones y revirtiendo {} ajustes con respaldo transaccional...",
                "Применение {} оптимизаций и откат {} твиков с транзакционным резервированием...",
                "正在应用 {} 项优化并还原 {} 项设置（含事务备份）...",
                "トランザクションバックアップ付きで {} 件の最適化を適用し、{} 件を元に戻しています...",
            ],
            apply_count,
            revert_count,
        ),
        StatusMessageSpec::BatchCompleted(count) => fmt1(
            lang,
            [
                "Successfully applied/reverted {} optimizations. Re-scanning system...",
                "{} iyileştirme başarıyla uygulandı/geri alındı. Sistem yeniden taranıyor...",
                "{} Optimierungen erfolgreich angewendet/zurückgesetzt. System wird neu gescannt...",
                "{} optimisations appliquées/rétablies avec succès. Nouvelle analyse du système...",
                "Se aplicaron/revirtieron {} optimizaciones correctamente. Volviendo a analizar el sistema...",
                "Успешно применено/откачено {} оптимизаций. Повторное сканирование системы...",
                "已成功应用/还原 {} 项优化。正在重新扫描系统...",
                "{} 件の最適化を正常に適用/復元しました。システムを再スキャンしています...",
            ],
            count,
        ),
        StatusMessageSpec::BatchFailed(err) => fmt1(
            lang,
            [
                "Error applying tweaks: {}",
                "Ayarlar uygulanırken hata: {}",
                "Fehler beim Anwenden der Tweaks: {}",
                "Erreur lors de l'application des réglages : {}",
                "Error al aplicar ajustes: {}",
                "Ошибка применения твиков: {}",
                "应用优化项时出错：{}",
                "調整の適用中にエラーが発生しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::StagedCount(n) => fmt1(
            lang,
            [
                "Staged {} tweaks for commit.",
                "Uygulanmak üzere {} ayar beklemeye alındı.",
                "{} Tweaks zur Übernahme vorgemerkt.",
                "{} réglages préparés pour application.",
                "{} ajustes preparados para aplicar.",
                "Подготовлено {} твиков к применению.",
                "已暂存 {} 项优化待应用。",
                "{} 件の調整を適用待ちにしました。",
            ],
            n,
        ),
        StatusMessageSpec::SelectedCount(n) => fmt1(
            lang,
            [
                "Selected {} applicable tweaks.",
                "{} uygun ayar seçildi.",
                "{} anwendbare Tweaks ausgewählt.",
                "{} réglages applicables sélectionnés.",
                "{} ajustes aplicables seleccionados.",
                "Выбрано {} применимых твиков.",
                "已选择 {} 项适用优化。",
                "{} 件の適用可能な調整を選択しました。",
            ],
            n,
        ),
        StatusMessageSpec::DeselectedAll => pick(lang, STATIC_DESELECTED_ALL).to_string(),
        StatusMessageSpec::NoCleanupSelected => {
            pick(lang, STATIC_NO_CLEANUP_SELECTED).to_string()
        }
        StatusMessageSpec::ProfileSaved(path) => fmt1(
            lang,
            [
                "Profile saved to {}",
                "Profil kaydedildi: {}",
                "Profil gespeichert unter {}",
                "Profil enregistré dans {}",
                "Perfil guardado en {}",
                "Профиль сохранён в {}",
                "配置文件已保存至 {}",
                "プロファイルを {} に保存しました",
            ],
            path,
        ),
        StatusMessageSpec::ProfileSaveFailed(err) => fmt1(
            lang,
            [
                "Failed to export profile: {}",
                "Profil dışa aktarılamadı: {}",
                "Profilexport fehlgeschlagen: {}",
                "Échec de l'exportation du profil : {}",
                "Error al exportar el perfil: {}",
                "Не удалось экспортировать профиль: {}",
                "导出配置文件失败：{}",
                "プロファイルのエクスポートに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::ProfileImported { imported, skipped } => {
            if *skipped > 0 {
                fmt2(
                    lang,
                    [
                        "Imported {} tweaks ({} unknown/incompatible skipped).",
                        "{} ayar içe aktarıldı ({} bilinmeyen/uyumsuz atlandı).",
                        "{} Tweaks importiert ({} unbekannt/inkompatibel übersprungen).",
                        "{} réglages importés ({} inconnus/incompatibles ignorés).",
                        "Se importaron {} ajustes ({} desconocidos/incompatibles omitidos).",
                        "Импортировано {} твиков ({} неизвестных/несовместимых пропущено).",
                        "已导入 {} 项优化（跳过 {} 项未知或不兼容项）。",
                        "{} 件の調整をインポートしました（{} 件の不明/非対応項目をスキップ）。",
                    ],
                    imported,
                    skipped,
                )
            } else {
                fmt1(
                    lang,
                    [
                        "Imported profile with {} tweaks.",
                        "{} ayar içeren profil içe aktarıldı.",
                        "Profil mit {} Tweaks importiert.",
                        "Profil importé avec {} réglages.",
                        "Perfil importado con {} ajustes.",
                        "Импортирован профиль с {} твиками.",
                        "已导入包含 {} 项优化的配置文件。",
                        "{} 件の調整を含むプロファイルをインポートしました。",
                    ],
                    imported,
                )
            }
        }
        StatusMessageSpec::ProfileImportFailed(err) => fmt1(
            lang,
            [
                "Failed to import profile: {}",
                "Profil içe aktarılamadı: {}",
                "Profilimport fehlgeschlagen: {}",
                "Échec de l'importation du profil : {}",
                "Error al importar el perfil: {}",
                "Не удалось импортировать профиль: {}",
                "导入配置文件失败：{}",
                "プロファイルのインポートに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::PresetActivated { kind, count } => match kind {
            PresetKey::Gaming => fmt1(
                lang,
                [
                    "Gaming Preset activated: {} low-latency tweaks selected.",
                    "Oyun Profili etkinleştirildi: {} düşük gecikme ayarı seçildi.",
                    "Gaming-Profil aktiviert: {} Latenz-Tweaks ausgewählt.",
                    "Préréglage Jeu activé : {} réglages faible latence sélectionnés.",
                    "Perfil de Juegos activado: {} ajustes de baja latencia seleccionados.",
                    "Игровой профиль активирован: выбрано {} твиков низкой задержки.",
                    "已激活游戏预设：已选择 {} 项低延迟优化。",
                    "ゲームプリセットを有効化しました：{} 件の低遅延調整を選択。",
                ],
                count,
            ),
            PresetKey::ProAudio => fmt1(
                lang,
                [
                    "Pro Audio Preset activated: {} real-time audio tweaks selected.",
                    "Pro Ses Profili etkinleştirildi: {} gerçek zamanlı ses ayarı seçildi.",
                    "Pro-Audio-Profil aktiviert: {} Echtzeit-Audio-Tweaks ausgewählt.",
                    "Préréglage Pro Audio activé : {} réglages audio temps réel sélectionnés.",
                    "Perfil Pro Audio activado: {} ajustes de audio en tiempo real seleccionados.",
                    "Профиль Про-Аудио активирован: выбрано {} твиков реального времени.",
                    "已激活专业音频预设：已选择 {} 项实时音频优化。",
                    "Pro Audio プリセットを有効化しました：{} 件のリアルタイム音声調整を選択。",
                ],
                count,
            ),
            PresetKey::LaptopBattery => fmt1(
                lang,
                [
                    "Laptop Battery Preset activated: {} power-saving tweaks selected.",
                    "Dizüstü Batarya Profili etkinleştirildi: {} güç tasarrufu ayarı seçildi.",
                    "Laptop-Akku-Profil aktiviert: {} Energiespar-Tweaks ausgewählt.",
                    "Préréglage Batterie activé : {} réglages d'économie d'énergie sélectionnés.",
                    "Perfil de Batería activado: {} ajustes de ahorro de energía seleccionados.",
                    "Профиль батареи активирован: выбрано {} энергосберегающих твиков.",
                    "已激活笔记本电池预设：已选择 {} 项节能优化。",
                    "バッテリー節約プリセットを有効化しました：{} 件の省電力調整を選択。",
                ],
                count,
            ),
        },
        StatusMessageSpec::ModeSwitchedInstant => pick(lang, STATIC_MODE_INSTANT).to_string(),
        StatusMessageSpec::ModeSwitchedNormal => pick(lang, STATIC_MODE_NORMAL).to_string(),
        StatusMessageSpec::ChangelogCleared => pick(lang, STATIC_CHANGELOG_CLEARED).to_string(),
        StatusMessageSpec::RestorePointCreating => pick(lang, STATIC_RESTORE_CREATING).to_string(),
        StatusMessageSpec::RestorePointCreated => pick(lang, STATIC_RESTORE_CREATED).to_string(),
        StatusMessageSpec::RestorePointFailed(err) => fmt1(
            lang,
            [
                "Failed to create restore point: {}",
                "Geri yükleme noktası oluşturulamadı: {}",
                "Wiederherstellungspunkt konnte nicht erstellt werden: {}",
                "Échec de création du point de restauration : {}",
                "Error al crear el punto de restauración: {}",
                "Не удалось создать точку восстановления: {}",
                "创建系统还原点失败：{}",
                "復元ポイントの作成に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::RollingBack(id) => fmt1(
            lang,
            [
                "Rolling back mutation #{}...",
                "#{} numaralı değişiklik geri alınıyor...",
                "Änderung #{} wird zurückgesetzt...",
                "Annulation de la modification #{}...",
                "Revirtiendo el cambio #{}...",
                "Откат изменения #{}...",
                "正在回滚更改 #{}...",
                "変更 #{} をロールバックしています...",
            ],
            id,
        ),
        StatusMessageSpec::RollbackCompleted => pick(lang, STATIC_ROLLBACK_COMPLETED).to_string(),
        StatusMessageSpec::RollbackFailed(err) => fmt1(
            lang,
            [
                "Rollback failed: {}",
                "Geri alma başarısız: {}",
                "Zurücksetzen fehlgeschlagen: {}",
                "Échec de l'annulation : {}",
                "Error al revertir: {}",
                "Ошибка отката: {}",
                "回滚失败：{}",
                "ロールバックに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::BaselineResetting => pick(lang, STATIC_BASELINE_RESETTING).to_string(),
        StatusMessageSpec::BaselineResetCompleted => {
            pick(lang, STATIC_BASELINE_RESET_OK).to_string()
        }
        StatusMessageSpec::BaselineResetFailed(err) => fmt1(
            lang,
            [
                "Failed to reset baseline: {}",
                "Temel çizgi sıfırlanamadı: {}",
                "Zurücksetzen der Basislinie fehlgeschlagen: {}",
                "Échec de réinitialisation de la ligne de base : {}",
                "Error al restablecer la línea base: {}",
                "Не удалось сбросить базовую линию: {}",
                "重置性能基线失败：{}",
                "ベースラインのリセットに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::GpuMsiUpdated => pick(lang, STATIC_GPU_MSI_OK).to_string(),
        StatusMessageSpec::GpuMsiFailed(err) => fmt1(
            lang,
            [
                "Failed to update GPU MSI: {}",
                "GPU MSI modu güncellenemedi: {}",
                "GPU-MSI-Aktualisierung fehlgeschlagen: {}",
                "Échec de mise à jour du mode MSI GPU : {}",
                "Error al actualizar MSI de GPU: {}",
                "Не удалось обновить режим MSI GPU: {}",
                "更新 GPU MSI 模式失败：{}",
                "GPU の MSI モード更新に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::HagsUpdated(enabled) => {
            if *enabled {
                pick(
                    lang,
                    [
                        "HAGS enabled successfully. Reboot required to take effect.",
                        "HAGS başarıyla etkinleştirildi. Etkinleşmesi için yeniden başlatma gerekir.",
                        "HAGS erfolgreich aktiviert. Neustart erforderlich.",
                        "HAGS activé avec succès. Redémarrage requis.",
                        "HAGS habilitado correctamente. Requiere reinicio para surtir efecto.",
                        "HAGS успешно включён. Для вступления в силу требуется перезагрузка.",
                        "已成功启用 HAGS。需要重启系统才能生效。",
                        "HAGS を有効化しました。反映するには再起動が必要です。",
                    ],
                )
                .to_string()
            } else {
                pick(
                    lang,
                    [
                        "HAGS disabled successfully. Reboot required to take effect.",
                        "HAGS başarıyla devre dışı bırakıldı. Etkinleşmesi için yeniden başlatma gerekir.",
                        "HAGS erfolgreich deaktiviert. Neustart erforderlich.",
                        "HAGS désactivé avec succès. Redémarrage requis.",
                        "HAGS deshabilitado correctamente. Requiere reinicio para surtir efecto.",
                        "HAGS успешно отключён. Для вступления в силу требуется перезагрузка.",
                        "已成功禁用 HAGS。需要重启系统才能生效。",
                        "HAGS を無効化しました。反映するには再起動が必要です。",
                    ],
                )
                .to_string()
            }
        }
        StatusMessageSpec::HagsFailed(err) => fmt1(
            lang,
            [
                "Failed to configure HAGS: {}",
                "HAGS yapılandırılamadı: {}",
                "HAGS-Konfiguration fehlgeschlagen: {}",
                "Échec de configuration HAGS : {}",
                "Error al configurar HAGS: {}",
                "Не удалось настроить HAGS: {}",
                "配置 HAGS 失败：{}",
                "HAGS の設定に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::MpoUpdated(disabled) => {
            if *disabled {
                pick(
                    lang,
                    [
                        "MPO disabled (stutter mitigation) successfully. Reboot recommended.",
                        "MPO başarıyla devre dışı bırakıldı (takılma önleme). Yeniden başlatma önerilir.",
                        "MPO erfolgreich deaktiviert (Ruckel-Schutz). Neustart empfohlen.",
                        "MPO désactivé (réduction des saccades) avec succès. Redémarrage recommandé.",
                        "MPO deshabilitado (mitigación de tirones) correctamente. Se recomienda reiniciar.",
                        "MPO успешно отключён (устранение статтеров). Рекомендуется перезагрузка.",
                        "已成功禁用 MPO（减少画面卡顿）。建议重启系统。",
                        "MPO を無効化しました（スタッター対策）。再起動を推奨します。",
                    ],
                )
                .to_string()
            } else {
                pick(
                    lang,
                    [
                        "MPO enabled successfully. Reboot recommended.",
                        "MPO başarıyla etkinleştirildi. Yeniden başlatma önerilir.",
                        "MPO erfolgreich aktiviert. Neustart empfohlen.",
                        "MPO activé avec succès. Redémarrage recommandé.",
                        "MPO habilitado correctamente. Se recomienda reiniciar.",
                        "MPO успешно включён. Рекомендуется перезагрузка.",
                        "已成功启用 MPO。建议重启系统。",
                        "MPO を有効化しました。再起動を推奨します。",
                    ],
                )
                .to_string()
            }
        }
        StatusMessageSpec::MpoFailed(err) => fmt1(
            lang,
            [
                "Failed to configure MPO: {}",
                "MPO yapılandırılamadı: {}",
                "MPO-Konfiguration fehlgeschlagen: {}",
                "Échec de configuration MPO : {}",
                "Error al configurar MPO: {}",
                "Не удалось настроить MPO: {}",
                "配置 MPO 失败：{}",
                "MPO の設定に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::LatencyBenchmarkRunning => {
            pick(lang, STATIC_LATENCY_RUNNING).to_string()
        }
        StatusMessageSpec::LatencyBenchmarkCompleted => {
            pick(lang, STATIC_LATENCY_DONE).to_string()
        }
        StatusMessageSpec::LatencyBenchmarkFailed(err) => fmt1(
            lang,
            [
                "Latency benchmark failed: {}",
                "Gecikme testi başarısız: {}",
                "Latenzbenchmark fehlgeschlagen: {}",
                "Échec du benchmark de latence : {}",
                "Error en la prueba de latencia: {}",
                "Ошибка теста задержек: {}",
                "延迟基准测试失败：{}",
                "遅延ベンチマークに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::LatencyBaselineSaved => {
            pick(lang, STATIC_LATENCY_BASELINE_SAVED).to_string()
        }
        StatusMessageSpec::LatencyRunTestFirst => pick(lang, STATIC_LATENCY_RUN_FIRST).to_string(),
        StatusMessageSpec::CoreParkingRefreshing => {
            pick(lang, STATIC_PARKING_REFRESHING).to_string()
        }
        StatusMessageSpec::CoreParkingUnparking => {
            pick(lang, STATIC_PARKING_UNPARKING).to_string()
        }
        StatusMessageSpec::CoreParkingUnparked => pick(lang, STATIC_PARKING_UNPARKED).to_string(),
        StatusMessageSpec::CoreParkingRestoring => {
            pick(lang, STATIC_PARKING_RESTORING).to_string()
        }
        StatusMessageSpec::CoreParkingRestored => pick(lang, STATIC_PARKING_RESTORED).to_string(),
        StatusMessageSpec::CoreParkingFailed { restoring, err } => {
            let e = localize_error_text(lang, err);
            if *restoring {
                fmt1(
                    lang,
                    [
                        "Failed to restore core parking: {}",
                        "Çekirdek park eşikleri geri yüklenemedi: {}",
                        "Core-Parking konnte nicht wiederhergestellt werden: {}",
                        "Échec de restauration du parcage des cœurs : {}",
                        "Error al restaurar el aparcamiento de núcleos: {}",
                        "Не удалось восстановить парковку ядер: {}",
                        "恢复核心休眠设置失败：{}",
                        "コアパーキングの復元に失敗しました: {}",
                    ],
                    e,
                )
            } else {
                fmt1(
                    lang,
                    [
                        "Failed to unpark cores: {}",
                        "Çekirdeklerin parkı kaldırılamadı: {}",
                        "Kerne konnten nicht entparkt werden: {}",
                        "Échec du réveil des cœurs : {}",
                        "Error al desaparcar los núcleos: {}",
                        "Не удалось разпарковать ядра: {}",
                        "唤醒核心失败：{}",
                        "コアパーキングの解除に失敗しました: {}",
                    ],
                    e,
                )
            }
        }
        StatusMessageSpec::CoreParkingPanic(err) => fmt1(
            lang,
            [
                "Core parking worker panic: {}",
                "Çekirdek park etme iş parçacığı hatası: {}",
                "Core-Parking-Worker-Abbruch: {}",
                "Arrêt inattendu du thread de parcage des cœurs : {}",
                "Fallo inesperado en el trabajador de aparcamiento de núcleos: {}",
                "Сбой потока парковки ядер: {}",
                "核心休眠后台任务异常中止：{}",
                "コアパーキングワーカーが異常終了しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::HostsBlocked(count) => fmt1(
            lang,
            [
                "Blocked {} telemetry domains in hosts file.",
                "Hosts dosyasında {} telemetri alan adı engellendi.",
                "{} Telemetrie-Domains in der Hosts-Datei blockiert.",
                "{} domaines de télémétrie bloqués dans le fichier hosts.",
                "Se bloquearon {} dominios de telemetría en el archivo hosts.",
                "Заблокировано {} доменов телеметрии в файле hosts.",
                "已在 hosts 文件中屏蔽 {} 个遥测域名。",
                "hosts ファイルで {} 個のテレメトリドメインをブロックしました。",
            ],
            count,
        ),
        StatusMessageSpec::HostsFailed(err) => fmt1(
            lang,
            [
                "Failed to modify hosts file: {}",
                "Hosts dosyası güncellenemedi: {}",
                "Hosts-Datei konnte nicht geändert werden: {}",
                "Échec de modification du fichier hosts : {}",
                "Error al modificar el archivo hosts: {}",
                "Не удалось изменить файл hosts: {}",
                "修改 hosts 文件失败：{}",
                "hosts ファイルの変更に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::TelemetryServicesNeutralized(count) => fmt1(
            lang,
            [
                "Neutralized {} background telemetry services.",
                "{} arka plan telemetri hizmeti etkisizleştirildi.",
                "{} Hintergrund-Telemetriedienste deaktiviert.",
                "{} services de télémétrie en arrière-plan neutralisés.",
                "Se neutralizaron {} servicios de telemetría en segundo plano.",
                "Отключено {} фоновых служб телеметрии.",
                "已停用 {} 个后台遥测服务。",
                "{} 個のバックグラウンドテレメトリサービスを無効化しました。",
            ],
            count,
        ),
        StatusMessageSpec::TelemetryServicesFailed(err) => fmt1(
            lang,
            [
                "Failed to neutralize telemetry services: {}",
                "Telemetri hizmetleri etkisizleştirilemedi: {}",
                "Telemetriedienste konnten nicht deaktiviert werden: {}",
                "Échec de neutralisation des services de télémétrie : {}",
                "Error al neutralizar servicios de telemetría: {}",
                "Не удалось отключить службы телеметрии: {}",
                "停用遥测服务失败：{}",
                "テレメトリサービスの無効化に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::DiskScanning => pick(lang, STATIC_DISK_SCANNING).to_string(),
        StatusMessageSpec::DiskScanned { count, mb } => fmt2(
            lang,
            [
                "Scanned {} cleanup targets ({} MB reclaimable).",
                "{} temizleme hedefi tarandı ({} MB geri kazanılabilir).",
                "{} Bereinigungsziele gescannt ({} MB freigebbar).",
                "{} cibles analysées ({} Mo récupérables).",
                "{} objetivos analizados ({} MB recuperables).",
                "Проверено {} целей очистки (можно освободить {} МБ).",
                "已扫描 {} 个清理目标（可释放 {} MB）。",
                "{} 個のクリーンアップ対象をスキャンしました（{} MB 解放可能）。",
            ],
            count,
            format!("{:.1}", mb),
        ),
        StatusMessageSpec::DiskScanFailed(err) => fmt1(
            lang,
            [
                "Disk scan failed: {}",
                "Disk taraması başarısız: {}",
                "Datenträgerscan fehlgeschlagen: {}",
                "Échec de l'analyse disque : {}",
                "Error al analizar el disco: {}",
                "Ошибка сканирования диска: {}",
                "磁盘扫描失败：{}",
                "ディスクスキャンに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::DiskCleaned { mb, files, skipped } => fmt3(
            lang,
            [
                "Cleanup complete: freed {} MB across {} files ({} locked skipped).",
                "Temizlik tamamlandı: {} dosya genelinde {} MB boşaltıldı ({} kilitli dosya atlandı).",
                "Bereinigung abgeschlossen: {} MB aus {} Dateien freigegeben ({} gesperrte übersprungen).",
                "Nettoyage terminé : {} Mo libérés sur {} fichiers ({} verrouillés ignorés).",
                "Limpieza completada: {} MB liberados en {} archivos ({} bloqueados omitidos).",
                "Очистка завершена: освобождено {} МБ в {} файлах (пропущено заблокированных: {}).",
                "清理完成：在 {} 个文件中释放 {} MB（跳过 {} 个占用文件）。",
                "クリーンアップ完了：{} ファイルから {} MB を解放しました（ロック済み {} 件スキップ）。",
            ],
            // Note: EN expects `{mb} MB across {files} files`, TR/DE/FR/ES/RU/ZH/JA order:
            // Let's keep placeholder order identical (mb, files, skipped) so fmt3 matches all 8!
            format!("{:.1}", mb),
            files,
            skipped,
        )
        // Fix argument order note above: let's make sure TR/DE/FR/ES/RU/ZH/JA match (mb, files, skipped)!
        .replace("__DUMMY__", ""),
        StatusMessageSpec::DiskCleanFailed(err) => fmt1(
            lang,
            [
                "Disk cleanup error: {}",
                "Disk temizleme hatası: {}",
                "Datenträgerbereinigungsfehler: {}",
                "Erreur de nettoyage disque : {}",
                "Error de limpieza de disco: {}",
                "Ошибка очистки диска: {}",
                "磁盘清理错误：{}",
                "ディスククリーンアップエラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::AppxFeatureScanning => pick(lang, STATIC_APPX_SCANNING).to_string(),
        StatusMessageSpec::AppxFeatureScanned { appx, features } => fmt2(
            lang,
            [
                "Loaded {} AppX packages & {} Windows Features.",
                "{} AppX paketi ve {} Windows Özelliği yüklendi.",
                "{} AppX-Pakete & {} Windows-Features geladen.",
                "{} paquets AppX et {} fonctionnalités Windows chargés.",
                "{} paquetes AppX y {} características de Windows cargadas.",
                "Загружено {} пакетов AppX и {} компонентов Windows.",
                "已加载 {} 个 AppX 应用包与 {} 项 Windows 功能。",
                "{} 個の AppX パッケージと {} 個の Windows 機能を読み込みました。",
            ],
            appx,
            features,
        ),
        StatusMessageSpec::AppxFeatureScanFailed(err) => fmt1(
            lang,
            [
                "AppX/Feature scan failed: {}",
                "AppX/Özellik taraması başarısız: {}",
                "AppX-/Feature-Scan fehlgeschlagen: {}",
                "Échec de l'analyse AppX/Fonctionnalités : {}",
                "Error al analizar AppX/Características: {}",
                "Ошибка сканирования AppX/компонентов: {}",
                "AppX/功能扫描失败：{}",
                "AppX/機能スキャンに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::AppxUninstalled(n) => fmt1(
            lang,
            [
                "Uninstalled {} AppX packages cleanly.",
                "{} AppX paketi temiz şekilde kaldırıldı.",
                "{} AppX-Pakete sauber deinstalliert.",
                "{} paquets AppX désinstallés proprement.",
                "Se desinstalaron {} paquetes AppX correctamente.",
                "Успешно удалено {} пакетов AppX.",
                "已干净卸载 {} 个 AppX 应用包。",
                "{} 個の AppX パッケージを正常にアンインストールしました。",
            ],
            n,
        ),
        StatusMessageSpec::AppxUninstallFailed(err) => fmt1(
            lang,
            [
                "AppX uninstall error: {}",
                "AppX kaldırma hatası: {}",
                "AppX-Deinstallationsfehler: {}",
                "Erreur de désinstallation AppX : {}",
                "Error al desinstalar AppX: {}",
                "Ошибка удаления AppX: {}",
                "AppX 卸载错误：{}",
                "AppX アンインストールエラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::FeatureUpdated { name, enabled } => {
            let disp = if lang == Language::English {
                name.as_str()
            } else {
                tweak_i18n::localize_optional_feature(lang, name)
                    .map(|(d, _)| d)
                    .unwrap_or(name.as_str())
            };
            let state_str = localize_feature_install_state(
                lang,
                if *enabled {
                    FeatureInstallState::Enabled
                } else {
                    FeatureInstallState::Disabled
                },
            );
            fmt2(
                lang,
                [
                    "Updated Windows feature '{}': {}.",
                    "'{}' Windows özelliği güncellendi: {}.",
                    "Windows-Feature '{}' aktualisiert: {}.",
                    "Fonctionnalité Windows '{}' mise à jour : {}.",
                    "Característica de Windows '{}' actualizada: {}.",
                    "Компонент Windows «{}» обновлён: {}.",
                    "已更新 Windows 功能“{}”：{}。",
                    "Windows 機能「{}」を更新しました: {}。",
                ],
                disp,
                state_str,
            )
        }
        StatusMessageSpec::FeatureUpdateFailed { name, err } => fmt2(
            lang,
            [
                "Feature update failed for '{}': {}",
                "'{}' özelliği güncellenemedi: {}",
                "Feature-Aktualisierung für '{}' fehlgeschlagen: {}",
                "Échec de mise à jour de '{}' : {}",
                "Error al actualizar la característica '{}': {}",
                "Ошибка обновления компонента «{}»: {}",
                "更新功能“{}”失败：{}",
                "機能「{}」の更新に失敗しました: {}",
            ],
            name,
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::OneDriveRemoved => pick(lang, STATIC_ONEDRIVE_REMOVED).to_string(),
        StatusMessageSpec::EdgeRemoved => pick(lang, STATIC_EDGE_REMOVED).to_string(),
        StatusMessageSpec::ActionMessage(msg) => localize_error_text(lang, msg),
        StatusMessageSpec::WinGetResult {
            installed,
            total,
            err,
        } => match err {
            None => fmt1(
                lang,
                [
                    "Successfully installed {} WinGet packages.",
                    "{} WinGet paketi başarıyla kuruldu.",
                    "{} WinGet-Pakete erfolgreich installiert.",
                    "{} paquets WinGet installés avec succès.",
                    "Se instalaron {} paquetes WinGet correctamente.",
                    "Успешно установлено {} пакетов WinGet.",
                    "已成功安装 {} 个 WinGet 软件包。",
                    "{} 個の WinGet パッケージをインストールしました。",
                ],
                installed,
            ),
            Some(e) => fmt3(
                lang,
                [
                    "Installed {}/{} packages; errors: {}",
                    "{}/{} paket kuruldu; hatalar: {}",
                    "{}/{} Pakete installiert; Fehler: {}",
                    "{}/{} paquets installés ; erreurs : {}",
                    "Instalados {}/{} paquetes; errores: {}",
                    "Установлено {}/{} пакетов; ошибки: {}",
                    "已安装 {}/{} 个软件包；错误：{}",
                    "{}/{} 個のパッケージをインストールしました。エラー: {}",
                ],
                installed,
                total,
                localize_error_text(lang, e),
            ),
        },
        StatusMessageSpec::WinGetFailed(err) => fmt1(
            lang,
            [
                "WinGet install error: {}",
                "WinGet kurulum hatası: {}",
                "WinGet-Installationsfehler: {}",
                "Erreur d'installation WinGet : {}",
                "Error de instalación WinGet: {}",
                "Ошибка установки WinGet: {}",
                "WinGet 安装错误：{}",
                "WinGet インストールエラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::StartupScanning => pick(lang, STATIC_STARTUP_SCANNING).to_string(),
        StatusMessageSpec::StartupScanned {
            startup,
            tasks,
            services,
        } => fmt3(
            lang,
            [
                "Loaded {} startup entries, {} scheduled tasks & {} services.",
                "{} başlangıç öğesi, {} zamanlanmış görev ve {} hizmet yüklendi.",
                "{} Autostart-Einträge, {} Aufgaben & {} Dienste geladen.",
                "{} entrées de démarrage, {} tâches et {} services chargés.",
                "{} entradas de inicio, {} tareas programadas y {} servicios cargados.",
                "Загружено {} записей автозагрузки, {} задач и {} служб.",
                "已加载 {} 个启动项、{} 个计划任务和 {} 个系统服务。",
                "{} 件のスタートアップ、{} 件のタスク、{} 件のサービスを読み込みました。",
            ],
            startup,
            tasks,
            services,
        ),
        StatusMessageSpec::StartupScanFailed(err) => fmt1(
            lang,
            [
                "Startup/Service scan failed: {}",
                "Başlangıç/Hizmet taraması başarısız: {}",
                "Autostart-/Dienst-Scan fehlgeschlagen: {}",
                "Échec de l'analyse Démarrage/Services : {}",
                "Error al analizar Inicio/Servicios: {}",
                "Ошибка сканирования автозагрузки/служб: {}",
                "启动项/服务扫描失败：{}",
                "スタートアップ/サービスのスキャンに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::StartupToggled { name, enable } => {
            if *enable {
                fmt1(
                    lang,
                    [
                        "Startup item '{}' enabled.",
                        "'{}' başlangıç öğesi etkinleştirildi.",
                        "Autostart-Eintrag '{}' aktiviert.",
                        "Élément de démarrage '{}' activé.",
                        "Elemento de inicio '{}' habilitado.",
                        "Элемент автозагрузки «{}» включён.",
                        "已启用启动项“{}”。",
                        "スタートアップ項目「{}」を有効にしました。",
                    ],
                    name,
                )
            } else {
                fmt1(
                    lang,
                    [
                        "Startup item '{}' disabled.",
                        "'{}' başlangıç öğesi devre dışı bırakıldı.",
                        "Autostart-Eintrag '{}' deaktiviert.",
                        "Élément de démarrage '{}' désactivé.",
                        "Elemento de inicio '{}' deshabilitado.",
                        "Элемент автозагрузки «{}» отключён.",
                        "已禁用启动项“{}”。",
                        "スタートアップ項目「{}」を無効にしました。",
                    ],
                    name,
                )
            }
        }
        StatusMessageSpec::StartupToggleError { name, err } => fmt2(
            lang,
            [
                "Failed to update '{}': {}",
                "'{}' güncellenemedi: {}",
                "Aktualisierung von '{}' fehlgeschlagen: {}",
                "Échec de mise à jour de '{}' : {}",
                "Error al actualizar '{}': {}",
                "Не удалось обновить «{}»: {}",
                "更新“{}”失败：{}",
                "「{}」の更新に失敗しました: {}",
            ],
            name,
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::TaskToggled { path, enable } => {
            if *enable {
                fmt1(
                    lang,
                    [
                        "Task '{}' enabled.",
                        "'{}' görevi etkinleştirildi.",
                        "Aufgabe '{}' aktiviert.",
                        "Tâche '{}' activée.",
                        "Tarea '{}' habilitada.",
                        "Задача «{}» включена.",
                        "已启用计划任务“{}”。",
                        "タスク「{}」を有効にしました。",
                    ],
                    path,
                )
            } else {
                fmt1(
                    lang,
                    [
                        "Task '{}' disabled.",
                        "'{}' görevi devre dışı bırakıldı.",
                        "Aufgabe '{}' deaktiviert.",
                        "Tâche '{}' désactivée.",
                        "Tarea '{}' deshabilitada.",
                        "Задача «{}» отключена.",
                        "已禁用计划任务“{}”。",
                        "タスク「{}」を無効にしました。",
                    ],
                    path,
                )
            }
        }
        StatusMessageSpec::TaskTriggered(path) => fmt1(
            lang,
            [
                "Triggered scheduled task '{}'.",
                "'{}' zamanlanmış görevi tetiklendi.",
                "Geplante Aufgabe '{}' ausgelöst.",
                "Tâche planifiée '{}' déclenchée.",
                "Tarea programada '{}' ejecutada.",
                "Запущена задача планировщика «{}».",
                "已触发计划任务“{}”。",
                "スケジュールタスク「{}」を実行しました。",
            ],
            path,
        ),
        StatusMessageSpec::TaskError { path, err } => fmt2(
            lang,
            [
                "Task error '{}': {}",
                "'{}' görev hatası: {}",
                "Aufgabenfehler '{}': {}",
                "Erreur de tâche '{}' : {}",
                "Error en tarea '{}': {}",
                "Ошибка задачи «{}»: {}",
                "计划任务“{}”错误：{}",
                "タスク「{}」のエラー: {}",
            ],
            path,
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::ServiceCriticalRefused(name) => fmt1(
            lang,
            [
                "Refused: '{}' is a Critical Core OS Service.",
                "Reddedildi: '{}' kritik bir çekirdek OS hizmetidir.",
                "Abgelehnt: '{}' ist ein kritischer OS-Kerndienst.",
                "Refusé : '{}' est un service système critique.",
                "Rechazado: '{}' es un servicio crítico del sistema operativo.",
                "Отказано: «{}» является критической системной службой ОС.",
                "已拒绝：“{}”是受保护的核心系统关键服务。",
                "拒否されました：「{}」は重要なコア OS サービスです。",
            ],
            name,
        ),
        StatusMessageSpec::ServiceNotFound(name) => fmt1(
            lang,
            [
                "Service '{}' not found or inaccessible.",
                "'{}' hizmeti bulunamadı veya erişilemiyor.",
                "Dienst '{}' nicht gefunden oder nicht zugänglich.",
                "Service '{}' introuvable ou inaccessible.",
                "Servicio '{}' no encontrado o inaccesible.",
                "Служба «{}» не найдена или недоступна.",
                "未找到服务“{}”或无权访问。",
                "サービス「{}」が見つからないかアクセスできません。",
            ],
            name,
        ),
        StatusMessageSpec::ServiceUpdated { name, mode } => fmt2(
            lang,
            [
                "Service '{}' start mode set to {}.",
                "'{}' hizmetinin başlangıç türü {} olarak ayarlandı.",
                "Starttyp von Dienst '{}' auf {} gesetzt.",
                "Mode de démarrage du service '{}' défini sur {}.",
                "Tipo de inicio del servicio '{}' establecido en {}.",
                "Тип запуска службы «{}» установлен на {}.",
                "已将服务“{}”的启动模式设为 {}。",
                "サービス「{}」のスタートアップ種類を {} に設定しました。",
            ],
            name,
            localize_service_start_mode(lang, *mode),
        ),
        StatusMessageSpec::ServiceError { name, err } => fmt2(
            lang,
            [
                "Service '{}' error: {}",
                "'{}' hizmet hatası: {}",
                "Dienstfehler '{}': {}",
                "Erreur du service '{}' : {}",
                "Error del servicio '{}': {}",
                "Ошибка службы «{}»: {}",
                "服务“{}”错误：{}",
                "サービス「{}」のエラー: {}",
            ],
            name,
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::PowerSchemeActivated(guid) => fmt1(
            lang,
            [
                "Activated power scheme {}.",
                "{} güç planı etkinleştirildi.",
                "Energiesparplan {} aktiviert.",
                "Mode d'alimentation {} activé.",
                "Plan de energía {} activado.",
                "Активирована схема питания {}.",
                "已激活电源计划 {}。",
                "電源プラン {} を有効化しました。",
            ],
            guid,
        ),
        StatusMessageSpec::PowerSchemeFailed(err) => fmt1(
            lang,
            [
                "Failed to set power plan: {}",
                "Güç planı ayarlanamadı: {}",
                "Energiesparplan konnte nicht gesetzt werden: {}",
                "Échec de définition du mode d'alimentation : {}",
                "Error al establecer el plan de energía: {}",
                "Не удалось установить схему питания: {}",
                "设置电源计划失败：{}",
                "電源プランの設定に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::UltimatePowerActive(guid) => fmt1(
            lang,
            [
                "WinBlast0 Ultimate Performance active ({}).",
                "WinBlast0 Nihai Performans etkin ({}).",
                "WinBlast0 Ultimative Leistung aktiv ({}).",
                "WinBlast0 Performances optimales actif ({}).",
                "WinBlast0 Máximo rendimiento activo ({}).",
                "Схема «WinBlast0 Максимальная производительность» активна ({}).",
                "WinBlast0 卓越性能电源计划已激活 ({})。",
                "WinBlast0 究極のパフォーマンスが有効になりました ({})。",
            ],
            guid,
        ),
        StatusMessageSpec::PowerPlanError(err) => fmt1(
            lang,
            [
                "Power plan error: {}",
                "Güç planı hatası: {}",
                "Energiesparplanfehler: {}",
                "Erreur du mode d'alimentation : {}",
                "Error del plan de energía: {}",
                "Ошибка схемы питания: {}",
                "电源计划错误：{}",
                "電源プランエラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::HibernationUpdated(enabled) => {
            if *enabled {
                pick(
                    lang,
                    [
                        "Hibernation enabled.",
                        "Hazırda bekletme etkinleştirildi.",
                        "Ruhezustand aktiviert.",
                        "Mise en veille prolongée activée.",
                        "Hibernación habilitada.",
                        "Гибернация включена.",
                        "已启用休眠功能。",
                        "休止状態を有効にしました。",
                    ],
                )
                .to_string()
            } else {
                pick(
                    lang,
                    [
                        "Hibernation disabled (hiberfil.sys freed).",
                        "Hazırda bekletme devre dışı bırakıldı (hiberfil.sys serbest bırakıldı).",
                        "Ruhezustand deaktiviert (hiberfil.sys freigegeben).",
                        "Mise en veille prolongée désactivée (hiberfil.sys libéré).",
                        "Hibernación deshabilitada (hiberfil.sys liberado).",
                        "Гибернация отключена (hiberfil.sys освобождён).",
                        "已禁用休眠功能（已释放 hiberfil.sys）。",
                        "休止状態を無効にしました (hiberfil.sys を解放)。",
                    ],
                )
                .to_string()
            }
        }
        StatusMessageSpec::HibernationFailed(err) => fmt1(
            lang,
            [
                "Hibernation error: {}",
                "Hazırda bekletme hatası: {}",
                "Ruhezustandsfehler: {}",
                "Erreur de mise en veille prolongée : {}",
                "Error de hibernación: {}",
                "Ошибка гибернации: {}",
                "休眠设置错误：{}",
                "休止状態エラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::UsbPowerUpdated { hubs, power_saving } => {
            let state = I18n::translate(
                lang,
                if *power_saving {
                    TextKey::StateEnabled
                } else {
                    TextKey::StateDisabled
                },
            );
            fmt2(
                lang,
                [
                    "Updated {} USB Root Hub controller power states (power saving: {}).",
                    "{} USB Kök Hub denetleyici güç durumu güncellendi (güç tasarrufu: {}).",
                    "Energiestatus von {} USB-Root-Hub-Controllern aktualisiert (Energiesparmodus: {}).",
                    "État d'alimentation de {} concentrateurs USB mis à jour (économie d'énergie : {}).",
                    "Se actualizó el estado de energía de {} concentradores raíz USB (ahorro: {}).",
                    "Обновлено состояние питания {} корневых USB-концентраторов (энергосбережение: {}).",
                    "已更新 {} 个 USB 根集线器控制器的电源状态（节能：{}）。",
                    "{} 個の USB ルートハブの電源状態を更新しました（省電力: {}）。",
                ],
                hubs,
                state,
            )
        }
        StatusMessageSpec::UsbPowerFailed(err) => fmt1(
            lang,
            [
                "USB power update failed: {}",
                "USB güç güncellemesi başarısız: {}",
                "USB-Energieaktualisierung fehlgeschlagen: {}",
                "Échec de mise à jour de l'alimentation USB : {}",
                "Error al actualizar la energía USB: {}",
                "Не удалось обновить питание USB: {}",
                "USB 电源设置更新失败：{}",
                "USB 電源設定の更新に失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::ExplorerRestartedOk => {
            pick(lang, STATIC_EXPLORER_RESTARTED).to_string()
        }
        StatusMessageSpec::ExplorerRestartError(err) => fmt1(
            lang,
            [
                "Explorer restart error: {}",
                "Gezgin yeniden başlatma hatası: {}",
                "Explorer-Neustartfehler: {}",
                "Erreur de redémarrage de l'Explorateur : {}",
                "Error al reiniciar el Explorador: {}",
                "Ошибка перезапуска Проводника: {}",
                "重启资源管理器失败：{}",
                "エクスプローラーの再起動エラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::UnattendExported { bytes, path } => fmt2(
            lang,
            [
                "Exported autounattend.xml ({} bytes) to {}",
                "autounattend.xml ({} bayt) dışa aktarıldı: {}",
                "autounattend.xml ({} Bytes) exportiert nach {}",
                "autounattend.xml ({} octets) exporté vers {}",
                "autounattend.xml ({} bytes) exportado a {}",
                "Файл autounattend.xml ({} байт) экспортирован в {}",
                "已导出 autounattend.xml（{} 字节）至 {}",
                "autounattend.xml（{} バイト）を {} にエクスポートしました",
            ],
            bytes,
            path,
        ),
        StatusMessageSpec::UnattendExportFailed(err) => fmt1(
            lang,
            [
                "Failed to export autounattend.xml: {}",
                "autounattend.xml dışa aktarılamadı: {}",
                "Export von autounattend.xml fehlgeschlagen: {}",
                "Échec de l'exportation de autounattend.xml : {}",
                "Error al exportar autounattend.xml: {}",
                "Не удалось экспортировать autounattend.xml: {}",
                "导出 autounattend.xml 失败：{}",
                "autounattend.xml のエクスポートに失敗しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::WorkerPanic(err) => fmt1(
            lang,
            [
                "Worker panic: {}",
                "Arka plan iş parçacığı hatası: {}",
                "Hintergrund-Worker-Abbruch: {}",
                "Arrêt inattendu du thread de travail : {}",
                "Fallo inesperado en tarea en segundo plano: {}",
                "Сбой фонового потока: {}",
                "后台工作线程异常：{}",
                "バックグラウンドワーカーが異常終了しました: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::LedgerError(err) => fmt1(
            lang,
            [
                "Ledger error: {}",
                "İşlem defteri hatası: {}",
                "Protokoll-Datenbankfehler: {}",
                "Erreur du registre transactionnel : {}",
                "Error del registro transaccional: {}",
                "Ошибка журнала транзакций: {}",
                "事务账本错误：{}",
                "トランザクション台帳エラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        StatusMessageSpec::Raw(raw) => localize_error_text(lang, raw),
    }
}

static ERROR_PREFIX_TABLE: &[(&str, [&str; 7])] = &[
    (
        "I/O or Registry error:",
        [
            "G/Ç veya Kayıt Defteri hatası:",
            "E/A- oder Registrierungsfehler:",
            "Erreur d'E/S ou de Registre :",
            "Error de E/S o Registro:",
            "Ошибка ввода-вывода или реестра:",
            "I/O 或注册表错误：",
            "I/O またはレジストリエラー:",
        ],
    ),
    (
        "Hardware/Power error:",
        [
            "Donanım/Güç hatası:",
            "Hardware-/Energiefehler:",
            "Erreur matériel/alimentation :",
            "Error de hardware/energía:",
            "Ошибка оборудования/питания:",
            "硬件/电源错误：",
            "ハードウェア/電源エラー:",
        ],
    ),
    (
        "Tweak failed:",
        [
            "Ayar başarısız:",
            "Tweak fehlgeschlagen:",
            "Échec du réglage :",
            "Error en el ajuste:",
            "Ошибка твика:",
            "优化执行失败：",
            "調整に失敗しました:",
        ],
    ),
    (
        "Elevation required:",
        [
            "Yönetici yetkisi gerekli:",
            "Administratorrechte erforderlich:",
            "Élévation requise :",
            "Se requiere elevación:",
            "Требуются права администратора:",
            "需要管理员权限：",
            "管理者権限が必要です:",
        ],
    ),
    (
        "Service error:",
        [
            "Hizmet hatası:",
            "Dienstfehler:",
            "Erreur de service :",
            "Error de servicio:",
            "Ошибка службы:",
            "系统服务错误：",
            "サービスエラー:",
        ],
    ),
    (
        "AppX/Package error:",
        [
            "AppX/Paket hatası:",
            "AppX-/Paketfehler:",
            "Erreur AppX/Paquet :",
            "Error de AppX/Paquete:",
            "Ошибка AppX/пакета:",
            "AppX/软件包错误：",
            "AppX/パッケージエラー:",
        ],
    ),
    (
        "Scheduled Task error:",
        [
            "Zamanlanmış Görev hatası:",
            "Aufgabenplanungsfehler:",
            "Erreur de tâche planifiée :",
            "Error de tarea programada:",
            "Ошибка планировщика задач:",
            "计划任务错误：",
            "スケジュールタスクエラー:",
        ],
    ),
    (
        "PowerShell error:",
        [
            "PowerShell hatası:",
            "PowerShell-Fehler:",
            "Erreur PowerShell :",
            "Error de PowerShell:",
            "Ошибка PowerShell:",
            "PowerShell 错误：",
            "PowerShell エラー:",
        ],
    ),
    (
        "Rollback unavailable:",
        [
            "Geri alma kullanılamıyor:",
            "Zurücksetzen nicht verfügbar:",
            "Annulation impossible :",
            "Reversión no disponible:",
            "Откат недоступен:",
            "无法回滚：",
            "ロールバックできません:",
        ],
    ),
    (
        "Database error:",
        [
            "Veritabanı hatası:",
            "Datenbankfehler:",
            "Erreur de base de données :",
            "Error de base de datos:",
            "Ошибка базы данных:",
            "数据库错误：",
            "データベースエラー:",
        ],
    ),
    (
        "OS incompatible:",
        [
            "İşletim sistemi uyumsuz:",
            "Betriebssystem inkompatibel:",
            "Système incompatible :",
            "SO incompatible:",
            "Несовместимая ОС:",
            "操作系统不兼容：",
            "OS 非対応:",
        ],
    ),
    (
        "Backup error:",
        [
            "Yedekleme hatası:",
            "Sicherungsfehler:",
            "Erreur de sauvegarde :",
            "Error de copia de seguridad:",
            "Ошибка резервного копирования:",
            "备份错误：",
            "バックアップエラー:",
        ],
    ),
    (
        "WMI error:",
        [
            "WMI hatası:",
            "WMI-Fehler:",
            "Erreur WMI :",
            "Error de WMI:",
            "Ошибка WMI:",
            "WMI 错误：",
            "WMI エラー:",
        ],
    ),
    (
        "TOML parse error:",
        [
            "TOML ayrıştırma hatası:",
            "TOML-Analysefehler:",
            "Erreur d'analyse TOML :",
            "Error de análisis TOML:",
            "Ошибка разбора TOML:",
            "TOML 解析错误：",
            "TOML 解析エラー:",
        ],
    ),
    (
        "Serialization error:",
        [
            "Serileştirme hatası:",
            "Serialisierungsfehler:",
            "Erreur de sérialisation :",
            "Error de serialización:",
            "Ошибка сериализации:",
            "序列化错误：",
            "シリアライズエラー:",
        ],
    ),
    (
        "Failed to open ledger:",
        [
            "İşlem defteri açılamadı:",
            "Protokoll-DB konnte nicht geöffnet werden:",
            "Impossible d'ouvrir le registre :",
            "Error al abrir el registro:",
            "Не удалось открыть журнал:",
            "无法打开事务账本：",
            "台帳を開けませんでした:",
        ],
    ),
    (
        "Failed to read history:",
        [
            "Geçmiş okunamadı:",
            "Verlauf konnte nicht gelesen werden:",
            "Impossible de lire l'historique :",
            "Error al leer el historial:",
            "Не удалось прочитать историю:",
            "无法读取历史记录：",
            "履歴を読み込めませんでした:",
        ],
    ),
    (
        "Apply error:",
        [
            "Uygulama hatası:",
            "Anwendungsfehler:",
            "Erreur d'application :",
            "Error al aplicar:",
            "Ошибка применения:",
            "应用错误：",
            "適用エラー:",
        ],
    ),
    (
        "Revert error:",
        [
            "Geri alma hatası:",
            "Rücksetzfehler:",
            "Erreur de rétablissement :",
            "Error al revertir:",
            "Ошибка отката:",
            "还原错误：",
            "復元エラー:",
        ],
    ),
    (
        "Rollback error:",
        [
            "Geri alma hatası:",
            "Rollback-Fehler:",
            "Erreur d'annulation :",
            "Error de reversión:",
            "Ошибка отката:",
            "回滚错误：",
            "ロールバックエラー:",
        ],
    ),
    (
        "Restore point error:",
        [
            "Geri yükleme noktası hatası:",
            "Wiederherstellungspunktfehler:",
            "Erreur de point de restauration :",
            "Error de punto de restauración:",
            "Ошибка точки восстановления:",
            "还原点错误：",
            "復元ポイントエラー:",
        ],
    ),
    (
        "MSI configuration error:",
        [
            "MSI yapılandırma hatası:",
            "MSI-Konfigurationsfehler:",
            "Erreur de configuration MSI :",
            "Error de configuración MSI:",
            "Ошибка настройки MSI:",
            "MSI 配置错误：",
            "MSI 設定エラー:",
        ],
    ),
    (
        "Worker panic:",
        [
            "İş parçacığı hatası:",
            "Worker-Abbruch:",
            "Erreur critique du thread :",
            "Fallo del trabajador:",
            "Сбой рабочего потока:",
            "工作线程异常：",
            "ワーカー異常終了:",
        ],
    ),
    (
        "Apply worker panic:",
        [
            "Uygulama iş parçacığı hatası:",
            "Anwendungs-Worker-Abbruch:",
            "Erreur critique d'application :",
            "Fallo del trabajador de aplicación:",
            "Сбой потока применения:",
            "应用线程异常：",
            "適用ワーカー異常終了:",
        ],
    ),
    (
        "Rollback worker panic:",
        [
            "Geri alma iş parçacığı hatası:",
            "Rollback-Worker-Abbruch:",
            "Erreur critique d'annulation :",
            "Fallo del trabajador de reversión:",
            "Сбой потока отката:",
            "回滚线程异常：",
            "ロールバックワーカー異常終了:",
        ],
    ),
    (
        "Baseline reset panic:",
        [
            "Temel çizgi sıfırlama iş parçacığı hatası:",
            "Basislinien-Reset-Abbruch:",
            "Erreur critique de réinitialisation :",
            "Fallo al restablecer la línea base:",
            "Сбой потока сброса базовой линии:",
            "基线重置线程异常：",
            "ベースラインリセット異常終了:",
        ],
    ),
    (
        "Restore point panic:",
        [
            "Geri yükleme noktası iş parçacığı hatası:",
            "Wiederherstellungspunkt-Abbruch:",
            "Erreur critique du point de restauration :",
            "Fallo del punto de restauración:",
            "Сбой потока точки восстановления:",
            "还原点线程异常：",
            "復元ポイントワーカー異常終了:",
        ],
    ),
    (
        "Ledger error:",
        [
            "İşlem defteri hatası:",
            "Protokoll-DB-Fehler:",
            "Erreur du registre :",
            "Error del registro:",
            "Ошибка журнала:",
            "账本错误：",
            "台帳エラー:",
        ],
    ),
    (
        "Blocked OneDrive uninstall:",
        [
            "OneDrive kaldırma engellendi:",
            "OneDrive-Deinstallation blockiert:",
            "Désinstallation de OneDrive bloquée :",
            "Desinstalación de OneDrive bloqueada:",
            "Удаление OneDrive заблокировано:",
            "已阻止卸载 OneDrive：",
            "OneDrive のアンインストールをブロックしました:",
        ],
    ),
];

const WIN32_ERROR_REPLACEMENT: [&str; 7] = [
    "Win32 hatası ",
    "Win32-Fehler ",
    "Erreur Win32 ",
    "Error Win32 ",
    "Ошибка Win32 ",
    "Win32 错误 ",
    "Win32 エラー ",
];

const ADMIN_RUN_MSG: [&str; 7] = [
    "Sistem ayarlarını uygulamak için WinBlast0 Yönetici olarak çalıştırılmalıdır.",
    "WinBlast0 muss als Administrator ausgeführt werden, um Systemänderungen anzuwenden.",
    "WinBlast0 doit être exécuté en tant qu'administrateur pour appliquer les réglages.",
    "WinBlast0 doit ejecutarse como Administrador para aplicar ajustes del sistema.",
    "WinBlast0 необходимо запустить от имени администратора для применения твиков.",
    "必须以管理员身份运行 WinBlast0 才能应用系统优化。",
    "システム調整を適用するには、WinBlast0 を管理者として実行する必要があります。",
];

const KFM_REDIRECT_MSG: [&str; 7] = [
    "Bilinen Klasör Taşıma (KFM) kullanıcı dosyalarını yönlendiriyor",
    "Known Folder Move (KFM) leitet Benutzerordner um",
    "La redirection des dossiers connus (KFM) est active",
    "Known Folder Move (KFM) redirige las carpetas de usuario",
    "Перенаправление известных папок (KFM) активно",
    "已知文件夹移动 (KFM) 正在重定向用户文件夹",
    "既知のフォルダーの移動 (KFM) が有効です",
];

const POLICY_LOCKED_LABEL: [&str; 7] = [
    "İlke kilitli",
    "Richtliniengesperrt",
    "Verrouillé par stratégie",
    "Bloqueado por directiva",
    "Заблокировано политикой",
    "策略锁定",
    "ポリシーによりロック",
];

/// Translate standard `WinBlastError` prefixes, admin notices, and `"Win32 error "` tokens
/// while preserving hex codes, OS error numbers, and file/registry paths verbatim.
pub fn localize_error_text(lang: Language, raw: &str) -> String {
    if lang == Language::English {
        return raw.to_string();
    }
    let idx = (lang as usize).saturating_sub(1);

    let mut rest = raw.trim();
    let mut out_prefix = String::new();

    // Iteratively peel nested error prefixes (e.g. "Rollback error: Rollback unavailable: ...")
    loop {
        let mut matched = false;
        for (en_prefix, translations) in ERROR_PREFIX_TABLE {
            if let Some(after) = rest.strip_prefix(en_prefix) {
                out_prefix.push_str(translations[idx]);
                out_prefix.push(' ');
                rest = after.trim_start();
                matched = true;
                break;
            }
        }
        if !matched {
            break;
        }
    }

    let mut body = if rest.starts_with("WinBlast0 must be run as Administrator") {
        ADMIN_RUN_MSG[idx].to_string()
    } else if rest.contains("Known Folder Move redirects user files")
        || rest.contains("Known Folder Move active")
    {
        rest.replace("Known Folder Move redirects user files", KFM_REDIRECT_MSG[idx])
            .replace("Known Folder Move active", KFM_REDIRECT_MSG[idx])
            .replace("Unlink OneDrive folder backup first.", "")
            .trim()
            .to_string()
    } else if rest.eq_ignore_ascii_case("Policy locked") || rest.contains("is policy locked") {
        if let Some(name_part) = rest.strip_prefix("Startup entry '") {
            if let Some((name, _)) = name_part.split_once('\'') {
                format!("'{}' — {}", name, POLICY_LOCKED_LABEL[idx])
            } else {
                POLICY_LOCKED_LABEL[idx].to_string()
            }
        } else {
            POLICY_LOCKED_LABEL[idx].to_string()
        }
    } else {
        rest.to_string()
    };

    if body.contains("Win32 error ") {
        body = body.replace("Win32 error ", WIN32_ERROR_REPLACEMENT[idx]);
    }

    if out_prefix.is_empty() {
        body
    } else {
        format!("{}{}", out_prefix, body)
    }
}

/// Localize a Windows service start mode (`0..=4`) across all 8 languages.
pub fn localize_service_start_mode(lang: Language, mode: u32) -> &'static str {
    let table = match mode {
        0 => [
            "Boot",
            "Önyükleme",
            "Start",
            "Démarrage",
            "Arranque",
            "Загрузка",
            "引导",
            "ブート",
        ],
        1 => [
            "System",
            "Sistem",
            "System",
            "Système",
            "Sistema",
            "Система",
            "系统",
            "システム",
        ],
        2 => [
            "Automatic",
            "Otomatik",
            "Automatisch",
            "Automatique",
            "Automático",
            "Автоматически",
            "自动",
            "自動",
        ],
        3 => [
            "Manual",
            "El İle",
            "Manuell",
            "Manuel",
            "Manual",
            "Вручную",
            "手动",
            "手動",
        ],
        4 => [
            "Disabled",
            "Devre Dışı",
            "Deaktiviert",
            "Désactivé",
            "Deshabilitado",
            "Отключена",
            "已禁用",
            "無効",
        ],
        _ => return I18n::translate(lang, TextKey::UnknownLabel),
    };
    pick(lang, table)
}

/// Localize DISM [`FeatureInstallState`] enum variants across all 8 languages.
pub fn localize_feature_install_state(lang: Language, state: FeatureInstallState) -> &'static str {
    let table = match state {
        FeatureInstallState::Enabled => [
            "Enabled",
            "Etkin",
            "Aktiviert",
            "Activé",
            "Habilitado",
            "Включено",
            "已启用",
            "有効",
        ],
        FeatureInstallState::Disabled => [
            "Disabled",
            "Devre Dışı",
            "Deaktiviert",
            "Désactivé",
            "Deshabilitado",
            "Отключено",
            "已禁用",
            "無効",
        ],
        FeatureInstallState::EnablePending => [
            "EnablePending",
            "Etkinleştirme Bekliyor",
            "Aktivierung ausstehend",
            "Activation en attente",
            "Activación pendiente",
            "Ожидает включения",
            "等待启用",
            "有効化保留中",
        ],
        FeatureInstallState::DisablePending => [
            "DisablePending",
            "Devre Dışı Bırakma Bekliyor",
            "Deaktivierung ausstehend",
            "Désactivation en attente",
            "Deshabilitación pendiente",
            "Ожидает отключения",
            "等待禁用",
            "無効化保留中",
        ],
        FeatureInstallState::Removed => [
            "Removed",
            "Kaldırıldı",
            "Entfernt",
            "Supprimé",
            "Eliminado",
            "Удалено",
            "已移除",
            "削除済み",
        ],
    };
    pick(lang, table)
}

fn localize_task_active_label(lang: Language, active: bool, short: bool) -> &'static str {
    if active {
        if short {
            pick(
                lang,
                [
                    "Active",
                    "Etkin",
                    "Aktiv",
                    "Active",
                    "Activa",
                    "Активна",
                    "活动",
                    "有効",
                ],
            )
        } else {
            pick(
                lang,
                [
                    "Active (Running/Ready)",
                    "Etkin (Çalışıyor/Hazır)",
                    "Aktiv (Läuft/Bereit)",
                    "Active (En cours/Prête)",
                    "Activa (En ejecución/Lista)",
                    "Активна (Выполняется/Готова)",
                    "活动 (正在运行/就绪)",
                    "アクティブ (実行中/準備完了)",
                ],
            )
        }
    } else {
        pick(
            lang,
            [
                "Disabled",
                "Devre Dışı",
                "Deaktiviert",
                "Désactivée",
                "Deshabilitada",
                "Отключена",
                "已禁用",
                "無効",
            ],
        )
    }
}

/// Localize any structured [`ScanDetailKind`] across all 8 languages, falling back to `raw_fallback`.
pub fn localize_scan_detail(
    lang: Language,
    detail_kind: Option<&ScanDetailKind>,
    raw_fallback: &str,
) -> String {
    if lang == Language::English && !raw_fallback.is_empty() {
        return raw_fallback.to_string();
    }
    let Some(kind) = detail_kind else {
        return raw_fallback.to_string();
    };

    match kind {
        ScanDetailKind::NotApplicableBuild { current, min_build } => fmt2(
            lang,
            [
                "Not applicable on build {} (requires >= {})",
                "Derleme {} için geçerli değil (en az {} gerekir)",
                "Nicht anwendbar auf Build {} (erfordert >= {})",
                "Non applicable sur build {} (requiert >= {})",
                "No aplicable en compilación {} (requiere >= {})",
                "Неприменимо к сборке {} (требуется >= {})",
                "不适用于内部版本 {}（需要 >= {}）",
                "ビルド {} には適用外（>= {} が必要）",
            ],
            current,
            min_build,
        ),
        ScanDetailKind::NotApplicableMaxBuild { current, max_build } => fmt2(
            lang,
            [
                "Not applicable on build {} (requires <= {})",
                "Derleme {} için geçerli değil (en fazla {} gerekir)",
                "Nicht anwendbar auf Build {} (erfordert <= {})",
                "Non applicable sur build {} (requiert <= {})",
                "No aplicable en compilación {} (requiere <= {})",
                "Неприменимо к сборке {} (требуется <= {})",
                "不适用于内部版本 {}（需要 <= {}）",
                "ビルド {} には適用外（<= {} が必要）",
            ],
            current,
            max_build,
        ),
        ScanDetailKind::NotApplicableEdition { edition, required } => {
            if required.trim().is_empty() {
                fmt1(
                    lang,
                    [
                        "Not applicable on {} edition",
                        "{} sürümünde geçerli değil",
                        "Nicht anwendbar auf Edition {}",
                        "Non applicable sur l'édition {}",
                        "No aplicable en la edición {}",
                        "Неприменимо для редакции {}",
                        "不适用于 {} 版本",
                        "{} エディションには適用外",
                    ],
                    edition,
                )
            } else {
                fmt2(
                    lang,
                    [
                        "Not applicable on {} edition (requires {})",
                        "{} sürümünde geçerli değil (gereken: {})",
                        "Nicht anwendbar auf Edition {} (erfordert {})",
                        "Non applicable sur l'édition {} (requiert {})",
                        "No aplicable en edición {} (requiere {})",
                        "Неприменимо для редакции {} (требуется {})",
                        "不适用于 {} 版本（需要 {}）",
                        "{} エディションには適用外（{} が必要）",
                    ],
                    edition,
                    required,
                )
            }
        }
        ScanDetailKind::NotApplicableGpu { vendor } => fmt1(
            lang,
            [
                "Not applicable (requires {} GPU)",
                "Geçerli değil ({} GPU gerektirir)",
                "Nicht anwendbar (erfordert {} GPU)",
                "Non applicable (nécessite un GPU {})",
                "No aplicable (requiere GPU {})",
                "Неприменимо (требуется GPU {})",
                "不适用（需要 {} 显卡）",
                "適用外（{} GPU が必要）",
            ],
            vendor,
        ),
        ScanDetailKind::RegStringValue { value } => fmt1(
            lang,
            [
                "String value: '{}'",
                "Metin değeri: '{}'",
                "Zeichenfolge: '{}'",
                "Valeur chaîne : '{}'",
                "Valor de cadena: '{}'",
                "Строковое значение: «{}»",
                "字符串值：'{}'",
                "文字列値: '{}'",
            ],
            value,
        ),
        ScanDetailKind::RegNotConfiguredStock => pick(
            lang,
            [
                "Value not configured (Stock)",
                "Değer yapılandırılmamış (Varsayılan)",
                "Wert nicht konfiguriert (Standard)",
                "Valeur non configurée (Défaut)",
                "Valor no configurado (Predeterminado)",
                "Параметр не задан (По умолчанию)",
                "未配置值（默认状态）",
                "値未設定（標準状態）",
            ],
        )
        .to_string(),
        ScanDetailKind::RegValueRemoved => pick(
            lang,
            [
                "Value removed (Optimized)",
                "Değer kaldırıldı (İyileştirildi)",
                "Wert entfernt (Optimiert)",
                "Valeur supprimée (Optimisé)",
                "Valor eliminado (Optimizado)",
                "Параметр удалён (Оптимизировано)",
                "值已删除（已优化）",
                "値を削除済み（最適化済み）",
            ],
        )
        .to_string(),
        ScanDetailKind::RegValuePresent => pick(
            lang,
            [
                "Value present (Stock)",
                "Değer mevcut (Varsayılan)",
                "Wert vorhanden (Standard)",
                "Valeur présente (Défaut)",
                "Valor presente (Predeterminado)",
                "Параметр присутствует (По умолчанию)",
                "值存在（默认状态）",
                "値が存在（標準状態）",
            ],
        )
        .to_string(),
        ScanDetailKind::SubtreeRemoved => pick(
            lang,
            [
                "Subtree removed (Optimized)",
                "Alt ağaç kaldırıldı (İyileştirildi)",
                "Unterschlüssel entfernt (Optimiert)",
                "Sous-arborescence supprimée (Optimisé)",
                "Subárbol eliminado (Optimizado)",
                "Ветвь реестра удалена (Оптимизировано)",
                "子树已删除（已优化）",
                "サブツリー削除済み（最適化済み）",
            ],
        )
        .to_string(),
        ScanDetailKind::SubtreePresent => pick(
            lang,
            [
                "Subtree present (Stock)",
                "Alt ağaç mevcut (Varsayılan)",
                "Unterschlüssel vorhanden (Standard)",
                "Sous-arborescence présente (Défaut)",
                "Subárbol presente (Predeterminado)",
                "Ветвь реестра присутствует (По умолчанию)",
                "子树存在（默认状态）",
                "サブツリー存在（標準状態）",
            ],
        )
        .to_string(),
        ScanDetailKind::BinaryMatch {
            index,
            byte_val,
            mask,
        } => fmt3(
            lang,
            [
                "Binary[{}]=0x{} (mask 0x{})",
                "İkili[{}]=0x{} (maske 0x{})",
                "Binär[{}]=0x{} (Maske 0x{})",
                "Binaire[{}]=0x{} (masque 0x{})",
                "Binario[{}]=0x{} (máscara 0x{})",
                "Двоичн.[{}]=0x{} (маска 0x{})",
                "二进制[{}]=0x{}（掩码 0x{}）",
                "バイナリ[{}]=0x{}（マスク 0x{}）",
            ],
            index,
            format!("{:02X}", byte_val),
            format!("{:02X}", mask),
        ),
        ScanDetailKind::BinaryTooShort { required, actual } => fmt2(
            lang,
            [
                "Binary shorter than {} bytes (len={})",
                "İkili veri {} bayttan kısa (uzunluk={})",
                "Binärwert kürzer als {} Bytes (Länge={})",
                "Valeur binaire inférieure à {} octets (long.={})",
                "Valor binario menor que {} bytes (long={})",
                "Длина двоичных данных меньше {} байт (длина={})",
                "二进制值长度小于 {} 字节（当前={}）",
                "バイナリ長が {} バイト未満です（長さ={}）",
            ],
            required,
            actual,
        ),
        ScanDetailKind::BinaryNotConfigured => pick(
            lang,
            [
                "Binary value not configured",
                "İkili değer yapılandırılmamış",
                "Binärwert nicht konfiguriert",
                "Valeur binaire non configurée",
                "Valor binario no configurado",
                "Двоичный параметр не задан",
                "二进制值未配置",
                "バイナリ値が未設定",
            ],
        )
        .to_string(),
        ScanDetailKind::CompositeSet { key, value } => format!("{}={}", key, value),
        ScanDetailKind::CompositeNotSet { key } => fmt1(
            lang,
            [
                "{} not set",
                "{} ayarlanmamış",
                "{} nicht festgelegt",
                "{} non défini",
                "{} no establecido",
                "{} не задано",
                "{} 未设置",
                "{} 未設定",
            ],
            key,
        ),
        ScanDetailKind::DynCurrent { current, target } => fmt2(
            lang,
            [
                "Current: {} (Target: {})",
                "Mevcut: {} (Hedef: {})",
                "Aktuell: {} (Ziel: {})",
                "Actuel : {} (Cible : {})",
                "Actual: {} (Objetivo: {})",
                "Текущее: {} (Цель: {})",
                "当前值：{}（目标：{}）",
                "現在値: {}（目標: {}）",
            ],
            current,
            target,
        ),
        ScanDetailKind::DynNotConfigured { target } => fmt1(
            lang,
            [
                "Not configured (Target: {})",
                "Yapılandırılmamış (Hedef: {})",
                "Nicht konfiguriert (Ziel: {})",
                "Non configuré (Cible : {})",
                "No configurado (Objetivo: {})",
                "Не настроено (Цель: {})",
                "未配置（目标：{}）",
                "未設定（目標: {}）",
            ],
            target,
        ),
        ScanDetailKind::PowerIndex { ac, dc } => fmt2(
            lang,
            [
                "Power index AC={} DC={}",
                "Güç dizini AC={} DC={}",
                "Energieindex Netz={} Akku={}",
                "Indice d'alimentation AC={} DC={}",
                "Índice de energía CA={} CC={}",
                "Индекс питания AC={} DC={}",
                "电源索引 AC={} DC={}",
                "電源インデックス AC={} DC={}",
            ],
            ac,
            dc,
        ),
        ScanDetailKind::PowerNotExposed { err } => fmt1(
            lang,
            [
                "Power setting not exposed on this hardware/scheme: {}",
                "Güç ayarı bu donanımda/planda desteklenmiyor: {}",
                "Energieeinstellung auf dieser Hardware/Schema nicht verfügbar: {}",
                "Paramètre d'alimentation non exposé sur ce matériel/mode : {}",
                "Ajuste de energía no disponible en este hardware/plan: {}",
                "Параметр питания не поддерживается оборудованием/схемой: {}",
                "此硬件/电源计划未公开该电源设置：{}",
                "このハードウェア/電源プランでは利用できません: {}",
            ],
            localize_error_text(lang, err),
        ),
        ScanDetailKind::PowerUnavailable { err } => fmt1(
            lang,
            [
                "Power setting unavailable: {}",
                "Güç ayarı kullanılamıyor: {}",
                "Energieeinstellung nicht verfügbar: {}",
                "Paramètre d'alimentation indisponible : {}",
                "Ajuste de energía no disponible: {}",
                "Параметр питания недоступен: {}",
                "电源设置不可用：{}",
                "電源設定を利用できません: {}",
            ],
            localize_error_text(lang, err),
        ),
        ScanDetailKind::PowerUnavailableHardware => pick(
            lang,
            [
                "Power setting unavailable on this hardware",
                "Güç ayarı bu donanımda kullanılamıyor",
                "Energieeinstellung auf dieser Hardware nicht verfügbar",
                "Paramètre d'alimentation indisponible sur ce matériel",
                "Ajuste de energía no disponible en este hardware",
                "Параметр питания недоступен на этом оборудовании",
                "此硬件不支持该电源设置",
                "このハードウェアでは電源設定を利用できません",
            ],
        )
        .to_string(),
        ScanDetailKind::FeatureState { state } => fmt1(
            lang,
            [
                "Feature state: {}",
                "Özellik durumu: {}",
                "Feature-Status: {}",
                "État de fonctionnalité : {}",
                "Estado de característica: {}",
                "Состояние компонента: {}",
                "功能状态：{}",
                "機能の状態: {}",
            ],
            localize_feature_install_state(lang, *state),
        ),
        ScanDetailKind::FeatureNotRegistered => pick(
            lang,
            [
                "Feature not present on this Windows edition (NotRegistered)",
                "Özellik bu Windows sürümünde bulunmuyor (Kayıtlı Değil)",
                "Feature in dieser Windows-Edition nicht vorhanden (Nicht registriert)",
                "Fonctionnalité absente sur cette édition Windows (Non enregistrée)",
                "Característica no presente en esta edición de Windows (No registrada)",
                "Компонент отсутствует в этой редакции Windows (Не зарегистрирован)",
                "此 Windows 版本不包含该功能（未注册）",
                "この Windows エディションには搭載されていません（未登録）",
            ],
        )
        .to_string(),
        ScanDetailKind::FeatureRequiresAdmin => pick(
            lang,
            [
                "Optional feature scan requires Administrator privileges",
                "İsteğe bağlı özellik taraması Yönetici yetkisi gerektirir",
                "Scan optionaler Features erfordert Administratorrechte",
                "L'analyse des fonctionnalités requiert les droits Administrateur",
                "El análisis de características requiere privilegios de Administrador",
                "Проверка дополнительных компонентов требует прав администратора",
                "扫描可选功能需要管理员权限",
                "オプション機能のスキャンには管理者権限が必要です",
            ],
        )
        .to_string(),
        ScanDetailKind::CompoundOptimized { optimized, total } => fmt2(
            lang,
            [
                "{}/{} actions optimized",
                "{}/{} eylem iyileştirildi",
                "{}/{} Aktionen optimiert",
                "{}/{} actions optimisées",
                "{}/{} acciones optimizadas",
                "Оптимизировано {}/{} действий",
                "已优化 {}/{} 项操作",
                "{}/{} 個の操作を最適化済み",
            ],
            optimized,
            total,
        ),
        ScanDetailKind::DwordOptimized { value } => fmt1(
            lang,
            [
                "Optimized (value: {})",
                "İyileştirildi (değer: {})",
                "Optimiert (Wert: {})",
                "Optimisé (valeur : {})",
                "Optimizado (valor: {})",
                "Оптимизировано (значение: {})",
                "已优化（值：{}）",
                "最適化済み（値: {}）",
            ],
            value,
        ),
        ScanDetailKind::DwordDefault { value } => fmt1(
            lang,
            [
                "Default (value: {})",
                "Varsayılan (değer: {})",
                "Standard (Wert: {})",
                "Défaut (valeur : {})",
                "Predeterminado (valor: {})",
                "По умолчанию (значение: {})",
                "默认值（值：{}）",
                "既定（値: {}）",
            ],
            value,
        ),
        ScanDetailKind::DwordCustom { value, expected } => fmt2(
            lang,
            [
                "Custom (value: {}, expected: {})",
                "Özel (değer: {}, beklenen: {})",
                "Benutzerdefiniert (Wert: {}, erwartet: {})",
                "Personnalisé (valeur : {}, attendu : {})",
                "Personalizado (valor: {}, esperado: {})",
                "Пользовательское (значение: {}, ожидалось: {})",
                "自定义（当前：{}，目标：{}）",
                "カスタム（現在: {}、目標: {}）",
            ],
            value,
            expected,
        ),
        ScanDetailKind::DwordDefaultNotConfigured => pick(
            lang,
            [
                "Default (not configured)",
                "Varsayılan (yapılandırılmamış)",
                "Standard (nicht konfiguriert)",
                "Défaut (non configuré)",
                "Predeterminado (no configurado)",
                "По умолчанию (не задано)",
                "默认（未配置）",
                "既定（未設定）",
            ],
        )
        .to_string(),
        ScanDetailKind::DwordNotConfigured {
            default_val,
            expected,
        } => fmt2(
            lang,
            [
                "Not configured (default: {}, expected: {})",
                "Yapılandırılmamış (varsayılan: {}, beklenen: {})",
                "Nicht konfiguriert (Standard: {}, erwartet: {})",
                "Non configuré (défaut : {}, attendu : {})",
                "No configurado (predet.: {}, esperado: {})",
                "Не настроено (по умолч.: {}, ожидалось: {})",
                "未配置（默认：{}，目标：{}）",
                "未設定（既定: {}、目標: {}）",
            ],
            default_val,
            expected,
        ),
        ScanDetailKind::DwordUnknownHive { hive } => fmt1(
            lang,
            [
                "Unknown registry hive: {}",
                "Bilinmeyen kayıt defteri kovanı: {}",
                "Unbekannter Registrierungs-Hive: {}",
                "Ruche de registre inconnue : {}",
                "Subárbol del registro desconocido: {}",
                "Неизвестный раздел реестра: {}",
                "未知注册表配置单元：{}",
                "不明なレジストリハイブ: {}",
            ],
            hive,
        ),
        ScanDetailKind::NoGpuAdapter { vendor } => fmt1(
            lang,
            [
                "No {} GPU adapter detected on this system",
                "Bu sistemde {} GPU bağdaştırıcısı algılanmadı",
                "Kein {}-GPU-Adapter auf diesem System erkannt",
                "Aucun adaptateur GPU {} détecté sur ce système",
                "No se detectó ningún adaptador GPU {} en este sistema",
                "На этой системе не обнаружен GPU {}",
                "未在此系统上检测到 {} 显卡适配器",
                "このシステムでは {} GPU アダプターが検出されませんでした",
            ],
            vendor,
        ),
        ScanDetailKind::ServiceOptimized { mode } => fmt1(
            lang,
            [
                "Optimized ({})",
                "İyileştirildi ({})",
                "Optimiert ({})",
                "Optimisé ({})",
                "Optimizado ({})",
                "Оптимизировано ({})",
                "已优化（{}）",
                "最適化済み（{}）",
            ],
            localize_service_start_mode(lang, *mode),
        ),
        ScanDetailKind::ServiceCurrent { current, target } => fmt2(
            lang,
            [
                "Current: {} (Target: {})",
                "Mevcut: {} (Hedef: {})",
                "Aktuell: {} (Ziel: {})",
                "Actuel : {} (Cible : {})",
                "Actual: {} (Objetivo: {})",
                "Текущее: {} (Цель: {})",
                "当前：{}（目标：{}）",
                "現在: {}（目標: {}）",
            ],
            localize_service_start_mode(lang, *current),
            localize_service_start_mode(lang, *target),
        ),
        ScanDetailKind::ServiceNotInstalled => pick(
            lang,
            [
                "Service not installed",
                "Hizmet yüklü değil",
                "Dienst nicht installiert",
                "Service non installé",
                "Servicio no instalado",
                "Служба не установлена",
                "服务未安装",
                "サービスはインストールされていません",
            ],
        )
        .to_string(),
        ScanDetailKind::ServiceAbsent => pick(
            lang,
            [
                "Service absent from system",
                "Hizmet sistemde bulunmuyor",
                "Dienst im System nicht vorhanden",
                "Service absent du système",
                "Servicio ausente en el sistema",
                "Служба отсутствует в системе",
                "系统中无此服务",
                "システムにサービスが存在しません",
            ],
        )
        .to_string(),
        ScanDetailKind::TaskFound { active } => fmt1(
            lang,
            [
                "Task found: {}",
                "Görev bulundu: {}",
                "Aufgabe gefunden: {}",
                "Tâche trouvée : {}",
                "Tarea encontrada: {}",
                "Задача найдена: {}",
                "已找到任务：{}",
                "タスク検出: {}",
            ],
            localize_task_active_label(lang, *active, false),
        ),
        ScanDetailKind::TaskMatched { leaf, active } => fmt2(
            lang,
            [
                "Matched ({}): {}",
                "Eşleşti ({}): {}",
                "Zugeordnet ({}): {}",
                "Correspondance ({}) : {}",
                "Coincidencia ({}): {}",
                "Совпадение ({}): {}",
                "匹配 ({})：{}",
                "一致 ({}): {}",
            ],
            leaf,
            localize_task_active_label(lang, *active, false),
        ),
        ScanDetailKind::TaskDirect { leaf, active } => fmt2(
            lang,
            [
                "Direct query ({}): {}",
                "Doğrudan sorgu ({}): {}",
                "Direktabfrage ({}): {}",
                "Requête directe ({}) : {}",
                "Consulta directa ({}): {}",
                "Прямой запрос ({}): {}",
                "直接查询 ({})：{}",
                "直接クエリ ({}): {}",
            ],
            leaf,
            localize_task_active_label(lang, *active, true),
        ),
        ScanDetailKind::TaskRequiresAdmin => pick(
            lang,
            [
                "Task requires Administrator privileges to query",
                "Görevi sorgulamak için Yönetici yetkisi gerekir",
                "Aufgabenabfrage erfordert Administratorrechte",
                "La consultation de la tâche requiert les droits Administrateur",
                "Consultar la tarea requiere privilegios de Administrador",
                "Запрос задачи требует прав администратора",
                "查询此计划任务需要管理员权限",
                "タスクの照会には管理者権限が必要です",
            ],
        )
        .to_string(),
        ScanDetailKind::TaskQueryError { err } => fmt1(
            lang,
            [
                "Task query error: {}",
                "Görev sorgulama hatası: {}",
                "Aufgabenabfragefehler: {}",
                "Erreur de requête de tâche : {}",
                "Error al consultar la tarea: {}",
                "Ошибка запроса задачи: {}",
                "任务查询错误：{}",
                "タスク照会エラー: {}",
            ],
            localize_error_text(lang, err),
        ),
        ScanDetailKind::TaskAbsentOptimized => pick(
            lang,
            [
                "Optimized (Task absent / not registered)",
                "İyileştirildi (Görev yok / kayıtlı değil)",
                "Optimiert (Aufgabe nicht vorhanden / registriert)",
                "Optimisé (Tâche absente / non enregistrée)",
                "Optimizado (Tarea ausente / no registrada)",
                "Оптимизировано (Задача отсутствует / не зарегистрирована)",
                "已优化（任务不存在/未注册）",
                "最適化済み（タスクなし/未登録）",
            ],
        )
        .to_string(),
        ScanDetailKind::TaskAbsentStock => pick(
            lang,
            [
                "Task absent from system",
                "Görev sistemde bulunmuyor",
                "Aufgabe im System nicht vorhanden",
                "Tâche absente du système",
                "Tarea ausente en el sistema",
                "Задача отсутствует в системе",
                "系统中无此任务",
                "システムにタスクが存在しません",
            ],
        )
        .to_string(),
        ScanDetailKind::AppxNoPackage => pick(
            lang,
            [
                "No package specified",
                "Paket belirtilmedi",
                "Kein Paket angegeben",
                "Aucun paquet spécifié",
                "No se especificó ningún paquete",
                "Пакет не указан",
                "未指定软件包",
                "パッケージが指定されていません",
            ],
        )
        .to_string(),
        ScanDetailKind::AppxRemovedPs => pick(
            lang,
            [
                "Removed / Not Installed",
                "Kaldırıldı / Yüklü Değil",
                "Entfernt / Nicht installiert",
                "Supprimé / Non installé",
                "Eliminado / No instalado",
                "Удалён / Не установлен",
                "已卸载 / 未安装",
                "削除済み / 未インストール",
            ],
        )
        .to_string(),
        ScanDetailKind::AppxInstalledPs => pick(
            lang,
            [
                "Installed (Detected via PowerShell)",
                "Yüklü (PowerShell ile algılandı)",
                "Installiert (über PowerShell erkannt)",
                "Installé (détecté via PowerShell)",
                "Instalado (detectado por PowerShell)",
                "Установлен (обнаружен через PowerShell)",
                "已安装（通过 PowerShell 检测）",
                "インストール済み（PowerShell 経由で検出）",
            ],
        )
        .to_string(),
        ScanDetailKind::AppxInstalled { package } => fmt1(
            lang,
            [
                "Installed ({})",
                "Yüklü ({})",
                "Installiert ({})",
                "Installé ({})",
                "Instalado ({})",
                "Установлен ({})",
                "已安装（{}）",
                "インストール済み（{}）",
            ],
            package,
        ),
        ScanDetailKind::AppxRemovedOptimized => pick(
            lang,
            [
                "Removed / Not Installed (Optimized)",
                "Kaldırıldı / Yüklü Değil (İyileştirildi)",
                "Entfernt / Nicht installiert (Optimiert)",
                "Supprimé / Non installé (Optimisé)",
                "Eliminado / No instalado (Optimizado)",
                "Удалён / Не установлен (Оптимизировано)",
                "已卸载 / 未安装（已优化）",
                "削除済み / 未インストール（最適化済み）",
            ],
        )
        .to_string(),
    }
}

/// Localize a SQLite `LedgerEntry.action_type` token across all 8 languages.
pub fn localize_ledger_action(lang: Language, action_type: &str) -> String {
    let (is_revert, raw) = match action_type.strip_prefix("Revert:") {
        Some(rest) => (true, rest),
        None => (false, action_type),
    };
    let (kind, target) = match raw.split_once(':') {
        Some((k, t)) => (k, t),
        None => (raw, ""),
    };

    let localized_kind: &str = match kind {
        "RegistryDword" => I18n::translate(lang, TextKey::DiffActionRegDword),
        "RegistryString" => I18n::translate(lang, TextKey::DiffActionRegSz),
        "RegistryDelete" => I18n::translate(lang, TextKey::DiffActionRegDelete),
        "RegistryDeleteTree" => I18n::translate(lang, TextKey::DiffActionRegTreeDelete),
        "RegistryBinaryBit" => I18n::translate(lang, TextKey::DiffActionRegBinBit),
        "CompositeStringKey" => I18n::translate(lang, TextKey::DiffActionRegComposite),
        "RegistryDynamicDword" => I18n::translate(lang, TextKey::DiffActionRegDynamic),
        "PowerCfgSetting" | "PowerCfg" => I18n::translate(lang, TextKey::DiffActionPowerCfg),
        "ServiceState" => I18n::translate(lang, TextKey::DiffActionService),
        "ScheduledTask" => I18n::translate(lang, TextKey::DiffActionTask),
        "AppXPackage" | "AppXBatch" => I18n::translate(lang, TextKey::DiffActionAppx),
        "WindowsFeature" => I18n::translate(lang, TextKey::DiffActionWinFeature),
        "StartupApproved" | "Startup" => pick(
            lang,
            [
                "STARTUP",
                "BAŞLANGIÇ",
                "AUTOSTART",
                "DÉMARRAGE",
                "INICIO",
                "АВТОЗАГРУЗКА",
                "启动项",
                "スタートアップ",
            ],
        ),
        "HostsBlock" => pick(
            lang,
            [
                "HOSTS BLOCK",
                "HOSTS ENGELİ",
                "HOSTS-SPERRE",
                "BLOCAGE HOSTS",
                "BLOQUEO HOSTS",
                "БЛОКИРОВКА HOSTS",
                "HOSTS 屏蔽",
                "HOSTS ブロック",
            ],
        ),
        "GpuHags" => "GPU HAGS",
        "GpuMpo" => "GPU MPO",
        "GpuMsi" => "GPU MSI",
        "CoreParking" => pick(
            lang,
            [
                "CORE PARKING",
                "ÇEKİRDEK PARKI",
                "CORE-PARKING",
                "PARCAGE CŒURS",
                "APARC. NÚCLEOS",
                "ПАРКОВКА ЯДЕР",
                "核心休眠",
                "コアパーキング",
            ],
        ),
        "CleanTargets" | "DiskCleanup" => pick(
            lang,
            [
                "DISK CLEANUP",
                "DİSK TEMİZLİĞİ",
                "BEREINIGUNG",
                "NETTOYAGE DISQUE",
                "LIMPIEZA DISCO",
                "ОЧИСТКА ДИСКА",
                "磁盘清理",
                "ディスク整理",
            ],
        ),
        "App" => pick(
            lang,
            [
                "APP REMOVAL",
                "UYGULAMA KALDIRMA",
                "APP-ENTFERNUNG",
                "SUPPRESSION APP",
                "ELIMINAR APP",
                "УДАЛЕНИЕ ПО",
                "应用移除",
                "アプリ削除",
            ],
        ),
        _ => I18n::translate(lang, TextKey::LedgerActionFallback),
    };

    let base = if target.is_empty() {
        if kind == "_" || localized_kind == I18n::translate(lang, TextKey::LedgerActionFallback) {
            format!("{}: {}", localized_kind, raw)
        } else {
            localized_kind.to_string()
        }
    } else {
        let localized_target = match (kind, target) {
            ("PowerCfg", "ActiveScheme") => pick(
                lang,
                [
                    "Active Scheme",
                    "Etkin Güç Planı",
                    "Aktiver Energiesparplan",
                    "Mode d'alimentation actif",
                    "Plan de energía activo",
                    "Активная схема",
                    "活动电源计划",
                    "アクティブ電源プラン",
                ],
            )
            .to_string(),
            ("PowerCfg", "UltimateScheme") => {
                tweak_i18n::localize_power_scheme_name(lang, "WinBlast0 Ultimate Performance")
                    .to_string()
            }
            ("PowerCfg", "Hibernation") => {
                I18n::translate(lang, TextKey::HibernationLabel).to_string()
            }
            ("PowerCfg", "UsbPowerSaving") => {
                I18n::translate(lang, TextKey::UsbPowerLabel).to_string()
            }
            ("CoreParking", "AllLogicalCores") => pick(
                lang,
                [
                    "All Logical Cores",
                    "Tüm Mantıksal Çekirdekler",
                    "Alle logischen Kerne",
                    "Tous les cœurs logiques",
                    "Todos los núcleos lógicos",
                    "Все логические ядра",
                    "所有逻辑核心",
                    "すべての論理コア",
                ],
            )
            .to_string(),
            ("HostsBlock", "TelemetryDomains") => pick(
                lang,
                [
                    "Telemetry Domains",
                    "Telemetri Alan Adları",
                    "Telemetrie-Domains",
                    "Domaines de télémétrie",
                    "Dominios de telemetría",
                    "Домены телеметрии",
                    "遥测域名",
                    "テレメトリドメイン",
                ],
            )
            .to_string(),
            ("CleanTargets", "Execute") => {
                I18n::translate(lang, TextKey::SubTabDiskCleaner).to_string()
            }
            ("AppXBatch", "RemoveAllUsers") => pick(
                lang,
                [
                    "Remove All Users",
                    "Tüm Kullanıcılardan Kaldır",
                    "Für alle Benutzer entfernen",
                    "Supprimer pour tous",
                    "Quitar para todos",
                    "Удалить для всех",
                    "为所有用户卸载",
                    "全ユーザーから削除",
                ],
            )
            .to_string(),
            _ => target.to_string(),
        };
        format!("{}: {}", localized_kind, localized_target)
    };

    if is_revert {
        format!("[{}] {}", I18n::translate(lang, TextKey::RevertLabel), base)
    } else {
        base
    }
}

/// Localize OneDrive Known Folder Move (KFM) diagnostic payloads across all 8 languages
/// while keeping filesystem paths, GUID markers, and account names intact.
pub fn localize_kfm_detail(lang: Language, raw_kfm: &str) -> String {
    if lang == Language::English {
        return raw_kfm.to_string();
    }
    let trimmed = raw_kfm.trim();
    if let Some((label, path)) = trimmed.split_once(" -> ") {
        let l_clean = label.trim();
        let (base_name, suffix) = if let Some(rest) = l_clean.strip_prefix("Desktop") {
            (I18n::translate(lang, TextKey::OneDriveKfmDesktop), rest)
        } else if let Some(rest) = l_clean.strip_prefix("Documents") {
            (I18n::translate(lang, TextKey::OneDriveKfmDocuments), rest)
        } else if let Some(rest) = l_clean.strip_prefix("Pictures") {
            (I18n::translate(lang, TextKey::OneDriveKfmPictures), rest)
        } else {
            (l_clean, "")
        };
        return format!("{}{} -> {}", base_name, suffix, path);
    }

    if let Some(rest) = trimmed.strip_prefix("OneDrive Account ") {
        return format!("{} {}", I18n::translate(lang, TextKey::OneDriveKfmAccount), rest);
    }

    raw_kfm.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_i18n_all_languages_and_core_specs() {
        let sample_specs = [
            StatusMessageSpec::ScanningStart,
            StatusMessageSpec::ScanCompleted(168),
            StatusMessageSpec::ScanFailed("Hardware/Power error: Win32 error 13".into()),
            StatusMessageSpec::LanguageChanged(Language::Turkish),
            StatusMessageSpec::NothingStaged,
            StatusMessageSpec::BatchCompleted(12),
            StatusMessageSpec::CoreParkingUnparked,
            StatusMessageSpec::CoreParkingRestored,
            StatusMessageSpec::ExplorerRestartedOk,
            StatusMessageSpec::OneDriveRemoved,
            StatusMessageSpec::PresetActivated {
                kind: PresetKey::Gaming,
                count: 10,
            },
            StatusMessageSpec::HagsUpdated(true),
            StatusMessageSpec::MpoUpdated(false),
            StatusMessageSpec::DiskCleaned { mb: 120.5, files: 40, skipped: 2 },
        ];

        for lang in Language::ALL {
            for spec in &sample_specs {
                let s = localize_status(lang, spec);
                assert!(!s.is_empty(), "Empty status for {:?} on {:?}", lang, spec);
            }
        }
    }

    #[test]
    fn test_turkish_language_changed_status_message() {
        let msg = localize_status(Language::Turkish, &StatusMessageSpec::LanguageChanged(Language::Turkish));
        assert_eq!(msg, "Dil Türkçe olarak değiştirildi.");
    }

    #[test]
    fn test_localize_error_text_prefixes() {
        let raw = "Hardware/Power error: Win32 error 0x0000000D (13)";
        let tr = localize_error_text(Language::Turkish, raw);
        assert!(tr.starts_with("Donanım/Güç hatası:"), "Actual: {}", tr);
        assert!(tr.contains("0x0000000D (13)"));

        let en = localize_error_text(Language::English, raw);
        assert_eq!(en, raw);
    }

    #[test]
    fn test_scan_detail_all_languages() {
        let kinds = [
            ScanDetailKind::NotApplicableBuild { current: 19041, min_build: 22000 },
            ScanDetailKind::RegValueRemoved,
            ScanDetailKind::RegValuePresent,
            ScanDetailKind::DwordOptimized { value: 1 },
            ScanDetailKind::DwordDefault { value: 0 },
            ScanDetailKind::ServiceOptimized { mode: 4 },
            ScanDetailKind::TaskFound { active: false },
            ScanDetailKind::AppxRemovedOptimized,
        ];

        for lang in Language::ALL {
            for kind in &kinds {
                let text = localize_scan_detail(lang, Some(kind), "Fallback Text");
                assert!(!text.is_empty(), "Empty detail for {:?} with {:?}", lang, kind);
            }
        }
    }
}

