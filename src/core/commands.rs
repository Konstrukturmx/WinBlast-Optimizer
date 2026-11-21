//! Unidirectional UI-to-Core Command Contract for WinBlast0.
//!
//! Every user-initiated interaction in the GUI (whether Rune or Slint) maps to
//! a strongly-typed `UiCommand`, ensuring zero backend coupling to UI callbacks.

use crate::app::ActiveView;
use crate::core::i18n::Language;
use crate::tools::automation::AutoTuneProfile;
use crate::ui::theme::{ColorScheme, ThemeMode, UiThemeStyle};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum UiCommand {
    /// Navigate to a specific view
    Navigate(ActiveView),

    /// Scan operations
    TriggerScan,
    TriggerHardwareProbe,
    AssessSystemHealth,

    /// Tweak operations
    ToggleTweak {
        id: String,
        enable: bool,
    },
    StageTweakTransition {
        id: String,
    },
    ApplyStagedTweaks,
    DiscardStagedTweaks,
    RevertSelectedTweaks,
    RevertAllOptimizedTweaks,
    SelectFilteredTweaks {
        search_query: String,
        category: Option<String>,
    },
    DeselectFilteredTweaks {
        search_query: String,
        category: Option<String>,
    },

    /// Automation & AutoTune
    ExecuteAutoTuneProfile(AutoTuneProfile),
    TrimWorkingSetMemory,

    /// Cleaner actions
    StartCleanerScan,
    ExecuteCleanerPurge {
        target_ids: Vec<String>,
    },
    SelectAllCleanTargets,
    DeselectAllCleanTargets,

    /// AppX Uninstaller actions
    SelectFilteredAppx {
        filter: String,
    },
    SelectSafeAndAiAppx,
    RemoveAppxPackage {
        package_full_name: String,
    },

    /// Startup apps and scheduled tasks
    ToggleStartupEntrySelection {
        id: String,
    },
    BatchToggleSelectedStartup {
        enable: bool,
    },
    ToggleScheduledTaskSelection {
        task_path: String,
    },
    BatchToggleSelectedTasks {
        enable: bool,
    },

    /// Hardware & Power
    SetPowerScheme {
        scheme_guid: String,
    },
    SetCoreParkingValues {
        min_cores: u32,
        max_cores: u32,
    },
    ExecuteDiskTrim {
        drive_letter: char,
    },
    TogglePcieMsi {
        device_id: String,
        enable: bool,
    },

    /// Backup & Ledger
    CreateSystemRestorePoint {
        description: String,
    },
    RollbackLedgerTransaction {
        transaction_id: i64,
    },
    ExportPreset {
        path: PathBuf,
    },
    ImportAndVerifyPreset {
        path: PathBuf,
    },

    /// Appearance & Preferences
    SetThemeMode(ThemeMode),
    SetColorScheme(ColorScheme),
    SetThemeStyle(UiThemeStyle),
    SetUiScale(f32),
    SetLanguage(Language),
    ToggleInstantMode(bool),

    /// Application Lifecycle
    MinimizeToSystemTray,
    RestoreFromSystemTray,
    Shutdown,
}
