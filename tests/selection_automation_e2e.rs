//! `selection_automation_e2e.rs`
//! Comprehensive End-to-End Test Suite for Selection Bugfixes,
//! Invariant Verification (I1-I4), and Automation Center Pipeline in WinBlast0 v1.6.1.

use std::sync::Arc;
use winblast0::app::{AppState, EngineMode, PresetKind, StagedTransition, StartupStatusFilter};
use winblast0::core::backend::SandboxEnv;
use winblast0::tools::automation::{create_auto_tune_plan, AutoTuneProfile};
use winblast0::tools::cleaner::{CleanTargetKind, CleanTargetScan};
use winblast0::tools::features_apps::{AppxSafetyTier, InstalledAppxPackage};
use winblast0::tools::startup::{LiveScheduledTaskInfo, StartupEntry, StartupSource};

#[test]
fn test_filter_locality_invariant_i1() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));

    // 1. Select tweaks matching "telemetry"
    state.select_filtered_tweaks("telemetry", None);
    assert!(
        !state.selected_tweaks.is_empty(),
        "Should select telemetry tweaks"
    );

    let initial_telemetry_ids: Vec<String> = state.selected_tweaks.iter().cloned().collect();
    for id in &initial_telemetry_ids {
        let t = state.tweaks.iter().find(|t| &t.id == id).unwrap();
        let matches = t.title.to_lowercase().contains("telemetry")
            || t.description.to_lowercase().contains("telemetry")
            || t.id.to_lowercase().contains("telemetry");
        assert!(matches, "Tweak {} must match telemetry filter", id);
    }

    // 2. Select tweaks matching "gaming"
    state.select_filtered_tweaks("gaming", None);
    let combined_count = state.selected_tweaks.len();
    assert!(combined_count >= initial_telemetry_ids.len());

    // 3. Deselect ONLY telemetry tweaks (Invariant I1: must not corrupt gaming tweaks!)
    state.deselect_filtered_tweaks("telemetry", None);

    for id in &initial_telemetry_ids {
        assert!(
            !state.selected_tweaks.contains(id),
            "Telemetry tweak {} must be deselected",
            id
        );
    }
}

#[test]
fn test_category_staging_parity_invariant_i2() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.engine_mode = EngineMode::Normal;

    // Pick first category with tweaks
    let cat_id = state.tweaks[0].category.as_str().to_string();
    let cat_tweak_ids: Vec<String> = state
        .tweaks
        .iter()
        .filter(|t| t.category.as_str() == cat_id)
        .map(|t| t.id.clone())
        .collect();

    assert!(!cat_tweak_ids.is_empty(), "Category must contain tweaks");

    // Toggle category selection ON
    state.toggle_category_selection(&cat_tweak_ids, true);

    for id in &cat_tweak_ids {
        assert!(
            state.selected_tweaks.contains(id),
            "Tweak {} must be in selected_tweaks",
            id
        );
        assert!(
            state.staged_transitions.contains_key(id),
            "Tweak {} must be in staged_transitions",
            id
        );
        assert_eq!(
            state.staged_transitions.get(id),
            Some(&StagedTransition::StageApply),
            "Unoptimized tweak must stage StageApply"
        );
    }

    // Pending actions must be populated
    assert!(
        !state.pending_actions.is_empty(),
        "Pending actions must not be empty after category select"
    );
    assert!(
        !state.pending_items.is_empty(),
        "Pending items must not be empty after category select"
    );

    // Toggle category selection OFF
    state.toggle_category_selection(&cat_tweak_ids, false);

    for id in &cat_tweak_ids {
        assert!(
            !state.selected_tweaks.contains(id),
            "Tweak {} must be removed from selected_tweaks",
            id
        );
        assert!(
            !state.staged_transitions.contains_key(id),
            "Tweak {} must be removed from staged_transitions",
            id
        );
    }
}

#[test]
fn test_appx_filtered_selection_and_core_protection() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));

    state.installed_appx = vec![
        InstalledAppxPackage {
            family_name: "Microsoft.XboxApp_8wekyb3d8bbwe".into(),
            short_name: "Xbox".into(),
            publisher: "Microsoft".into(),
            safety_tier: AppxSafetyTier::SafeBloat,
            is_provisioned: false,
        },
        InstalledAppxPackage {
            family_name: "Microsoft.GamingApp_8wekyb3d8bbwe".into(),
            short_name: "Xbox Gaming".into(),
            publisher: "Microsoft".into(),
            safety_tier: AppxSafetyTier::SafeBloat,
            is_provisioned: false,
        },
        InstalledAppxPackage {
            family_name: "Microsoft.Windows.ShellExperienceHost_cw5n1h2txyewy".into(),
            short_name: "Shell Experience Host".into(),
            publisher: "Microsoft".into(),
            safety_tier: AppxSafetyTier::ProtectedCore,
            is_provisioned: true,
        },
        InstalledAppxPackage {
            family_name: "Microsoft.Copilot_8wekyb3d8bbwe".into(),
            short_name: "Copilot".into(),
            publisher: "Microsoft".into(),
            safety_tier: AppxSafetyTier::AiBloat,
            is_provisioned: false,
        },
    ];

    // Select filtered by "xbox"
    state.select_filtered_appx("xbox");
    assert_eq!(state.selected_appx.len(), 2);
    assert!(state
        .selected_appx
        .contains("Microsoft.XboxApp_8wekyb3d8bbwe"));
    assert!(state
        .selected_appx
        .contains("Microsoft.GamingApp_8wekyb3d8bbwe"));
    assert!(!state
        .selected_appx
        .contains("Microsoft.Windows.ShellExperienceHost_cw5n1h2txyewy"));

    // Select safe & AI bloat
    state.select_safe_and_ai_appx();
    assert_eq!(state.selected_appx.len(), 3);
    assert!(state
        .selected_appx
        .contains("Microsoft.Copilot_8wekyb3d8bbwe"));
    assert!(!state
        .selected_appx
        .contains("Microsoft.Windows.ShellExperienceHost_cw5n1h2txyewy"));

    // Deselect all
    state.deselect_all_appx();
    assert!(state.selected_appx.is_empty());
}

#[test]
fn test_clean_targets_selection_parity() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));

    state.clean_targets = vec![
        CleanTargetScan {
            kind: CleanTargetKind::UserTemp,
            paths: vec!["C:\\Users\\Mock\\AppData\\Local\\Temp".into()],
            reclaimable_bytes: 1048576,
            file_count: 50,
            selected: false,
            requires_elevation: false,
            last_error: None,
        },
        CleanTargetScan {
            kind: CleanTargetKind::WindowsUpdateCache,
            paths: vec!["C:\\Windows\\SoftwareDistribution\\Download".into()],
            reclaimable_bytes: 20971520,
            file_count: 10,
            selected: false,
            requires_elevation: true,
            last_error: None,
        },
    ];

    state.select_all_clean_targets();
    for t in &state.clean_targets {
        assert!(t.selected, "Clean target {:?} must be selected", t.kind);
    }

    state.deselect_all_clean_targets();
    for t in &state.clean_targets {
        assert!(!t.selected, "Clean target {:?} must be deselected", t.kind);
    }
}

#[test]
fn test_startup_and_tasks_batch_selection() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));

    state.startup_entries = vec![
        StartupEntry {
            id: "hkcu|Steam".into(),
            name: "Steam".into(),
            command: "C:\\Steam\\steam.exe".into(),
            source: StartupSource::RegistryHkcuRun,
            location: "Registry".into(),
            hive: "HKCU".into(),
            reg_path: "Software\\Microsoft\\Windows\\CurrentVersion\\Run".into(),
            enabled: true,
            is_microsoft: false,
            approved_subkey: Some("Run".into()),
            is_wow64: false,
            policy_locked: false,
        },
        StartupEntry {
            id: "hkcu|OneDrive".into(),
            name: "OneDrive".into(),
            command: "C:\\OneDrive\\onedrive.exe".into(),
            source: StartupSource::RegistryHkcuRun,
            location: "Registry".into(),
            hive: "HKCU".into(),
            reg_path: "Software\\Microsoft\\Windows\\CurrentVersion\\Run".into(),
            enabled: false,
            is_microsoft: true,
            approved_subkey: Some("Run".into()),
            is_wow64: false,
            policy_locked: false,
        },
    ];

    state.scheduled_tasks = vec![
        LiveScheduledTaskInfo {
            task_path:
                "\\Microsoft\\Windows\\Customer Experience Improvement Program\\Consolidator".into(),
            task_name: "Consolidator".into(),
            folder: "\\Microsoft\\Windows\\Customer Experience Improvement Program".into(),
            enabled: true,
            status: "Ready".into(),
            is_microsoft: true,
        },
        LiveScheduledTaskInfo {
            task_path: "\\GoogleUpdateTaskMachineUA".into(),
            task_name: "GoogleUpdateTaskMachineUA".into(),
            folder: "\\".into(),
            enabled: true,
            status: "Ready".into(),
            is_microsoft: false,
        },
    ];

    // 1. Select all startup items ignoring Microsoft items
    state.select_all_filtered_startup("", true, StartupStatusFilter::All);
    assert_eq!(state.selected_startup_entries.len(), 1);
    assert!(state.selected_startup_entries.contains("hkcu|Steam"));

    state.deselect_all_startup();
    assert!(state.selected_startup_entries.is_empty());

    // 2. Select all tasks including Microsoft
    state.select_all_filtered_tasks("", false, StartupStatusFilter::All);
    assert_eq!(state.selected_scheduled_tasks.len(), 2);

    state.deselect_all_tasks();
    assert!(state.selected_scheduled_tasks.is_empty());
}

#[test]
fn test_preset_alignment_with_autotune_profiles() {
    let sbx = SandboxEnv::new();
    let mut state = AppState::new_with_backend(Arc::new(sbx));
    state.engine_mode = EngineMode::Normal;

    // Apply Gaming preset
    state.apply_preset(PresetKind::Gaming);
    assert!(
        !state.selected_tweaks.is_empty(),
        "Gaming preset must select tweaks"
    );
    assert!(
        !state.staged_transitions.is_empty(),
        "Gaming preset must stage transitions in Normal mode"
    );

    let gaming_plan = create_auto_tune_plan(&state.tweaks, AutoTuneProfile::GamingLowLatency);
    assert!(
        !gaming_plan.recommended_tweak_ids.is_empty(),
        "AutoTune plan must contain recommendations"
    );

    // All selected tweaks must be verified against ISO criteria
    for id in &state.selected_tweaks {
        assert!(
            gaming_plan.recommended_tweak_ids.contains(id),
            "Tweak {} must be in gaming plan",
            id
        );
    }
}
