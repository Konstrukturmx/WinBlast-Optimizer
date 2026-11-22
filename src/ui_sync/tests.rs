use crate::ui_sync::bitops::BitSet256;
use crate::ui_sync::hud_progress::{BatchDescriptor, MonotonicCubicProgress};
use crate::ui_sync::selection::{ConfirmationRequirement, FilterStrictSelection, FilterableEntry};

#[test]
fn select_all_is_strictly_scoped_to_active_filter_and_tracks_protected() {
    let mut protected = BitSet256::EMPTY;
    protected.set(1); // Core AppX package
    let mut selectable = BitSet256::EMPTY;
    for i in 0..4 {
        selectable.set(i);
    }

    let mut sm = FilterStrictSelection::new(4, protected, selectable);
    let catalog = [
        FilterableEntry {
            id: "net_nagle",
            title: "Disable Nagle",
            category: "Network",
        },
        FilterableEntry {
            id: "appx_store",
            title: "Remove Core Store AppX",
            category: "AppX",
        },
        FilterableEntry {
            id: "appx_xbox",
            title: "Remove Xbox AppX",
            category: "AppX",
        },
        FilterableEntry {
            id: "gpu_hags",
            title: "Enable HAGS",
            category: "GPU",
        },
    ];

    // Select a non-AppX tweak first.
    sm.toggle_row(0);
    assert_eq!(sm.metrics().total_selected, 1);

    // Filter to "appx" and trigger Select All: must strictly select ONLY rows 1 & 2,
    // clearing hidden row 0 and emitting confirmation for protected row 1.
    assert!(sm.apply_filter(
        "appx",
        None,
        catalog.iter().map(|e| FilterableEntry {
            id: e.id,
            title: e.title,
            category: e.category,
        })
    ));
    let _delta = sm.toggle_select_all_filtered();

    assert!(!sm.is_selected(0));
    assert!(sm.is_selected(1));
    assert!(sm.is_selected(2));
    assert!(!sm.is_selected(3));

    let token = match sm.confirmation_requirement() {
        ConfirmationRequirement::Required {
            protected_count,
            first_protected_index,
            confirmation_token,
        } => {
            assert_eq!(protected_count, 1);
            assert_eq!(first_protected_index, 1);
            confirmation_token
        }
        other => panic!("expected Required confirmation, got {other:?}"),
    };

    assert!(sm.confirm_protected(token));
    assert_eq!(
        sm.confirmation_requirement(),
        ConfirmationRequirement::Confirmed { protected_count: 1 }
    );

    // Unchecking then re-checking the protected row must invalidate prior confirmation.
    sm.toggle_row(1);
    assert_eq!(
        sm.confirmation_requirement(),
        ConfirmationRequirement::NotRequired
    );
    sm.toggle_row(1);
    assert!(matches!(
        sm.confirmation_requirement(),
        ConfirmationRequirement::Required {
            protected_count: 1,
            ..
        }
    ));
}

#[test]
fn monotonic_cubic_progress_never_decreases_or_overshoots_on_retarget() {
    let batches = [
        BatchDescriptor {
            step_count: 3,
            total_weight: 10,
        },
        BatchDescriptor {
            step_count: 2,
            total_weight: 20,
        },
    ];
    let mut hud = MonotonicCubicProgress::start(&batches);
    let mut prev = 0.0f32;

    // Simulate rapid and slow steps with mid-flight spline retargets at 120 Hz.
    let step_schedule = [
        (0u16, 2u32, 0.4),
        (0, 3, 1.2),
        (0, 5, 0.2),
        (1, 10, 2.0),
        (1, 10, 0.5),
    ];
    for (batch_idx, weight, est) in step_schedule {
        hud.begin_step(batch_idx, weight, est);
        for _ in 0..45 {
            let frame = hud.tick(1.0 / 120.0);
            assert!(
                frame.progress >= prev,
                "regression: {} < {}",
                frame.progress,
                prev
            );
            assert!(frame.progress <= 1.0, "overshoot: {}", frame.progress);
            prev = frame.progress;
        }
        hud.complete_step(weight);
        // Mid-flight ticks right after step completion
        for _ in 0..10 {
            let frame = hud.tick(1.0 / 120.0);
            assert!(frame.progress >= prev);
            assert!(frame.progress <= 1.0);
            prev = frame.progress;
        }
    }

    hud.finish();
    for _ in 0..120 {
        let frame = hud.tick(1.0 / 120.0);
        assert!(frame.progress >= prev);
        assert!(frame.progress <= 1.0);
        prev = frame.progress;
    }
    assert!((prev - 1.0).abs() < 1e-4);
}
