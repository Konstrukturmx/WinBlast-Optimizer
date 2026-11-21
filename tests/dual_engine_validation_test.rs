//! `dual_engine_validation_test.rs`
//! Verification test suite for the Dual-Engine Coding & Algorithmic Delegation Protocol.
//!
//! Validates the Tier 2 (Gemini Neon · High) generated `schedule_optimizations_topological`
//! algorithm integrated by Tier 1 (Antigravity Coordinator).

use winblast0::graph::topological_scheduler::{schedule_optimizations_topological, MAX_NODES};

#[test]
fn test_dual_engine_topological_scheduler_end_to_end() {
    // End-to-end integration scenario:
    // 5 interdependent Windows optimization tweaks:
    // Node 0: Disable Telemetry (no reboot)
    // Node 1: Configure HPET Timer (reboot required)
    // Node 2: Optimize Network Adapter (no reboot, depends on Node 0)
    // Node 3: GPU MSI Mode (reboot required, depends on Node 2)
    // Node 4: Apply Core Parking (no reboot, independent)
    //
    // Dependencies:
    // 0 -> 2 (weight 100)
    // 2 -> 3 (weight 50)
    // Artificial circular dependency: 3 -> 0 (weight 10 - lowest penalty)
    //
    // Expected behavior:
    // 1. Cycle (0 -> 2 -> 3 -> 0) is detected.
    // 2. Lowest-weight edge (3 -> 0, weight 10) is broken.
    // 3. Non-reboot tweaks (0, 2, 4) execute first.
    // 4. Reboot tweaks (1, 3) execute last (batched at tail).

    let num_nodes = 5;
    let dependencies = [
        (0, 2), // 0 -> 2
        (2, 3), // 2 -> 3
        (3, 0), // 3 -> 0 (cycle back-edge, weight 10)
    ];
    let weights = [100, 50, 10];
    let reboot_required = [
        false, // Node 0
        true,  // Node 1 (reboot)
        false, // Node 2
        true,  // Node 3 (reboot)
        false, // Node 4
    ];

    let result =
        schedule_optimizations_topological(num_nodes, &dependencies, &weights, &reboot_required);
    assert!(result.is_ok(), "Scheduler should resolve cycle and succeed");

    let order = result.unwrap();
    assert_eq!(order.len(), num_nodes, "All nodes must be scheduled");

    // Check non-reboot tweaks are executed before reboot tweaks
    let pos_0 = order.iter().position(|&x| x == 0).unwrap();
    let pos_1 = order.iter().position(|&x| x == 1).unwrap();
    let pos_2 = order.iter().position(|&x| x == 2).unwrap();
    let pos_3 = order.iter().position(|&x| x == 3).unwrap();
    let pos_4 = order.iter().position(|&x| x == 4).unwrap();

    // 0 must precede 2
    assert!(pos_0 < pos_2, "Node 0 must precede Node 2");
    // 2 must precede 3
    assert!(pos_2 < pos_3, "Node 2 must precede Node 3");

    // All non-reboot tweaks (0, 2, 4) must precede reboot tweaks (1, 3)
    let max_non_reboot_pos = pos_0.max(pos_2).max(pos_4);
    let min_reboot_pos = pos_1.min(pos_3);
    assert!(
        max_non_reboot_pos < min_reboot_pos,
        "All non-reboot optimizations must precede reboot-required optimizations (pos non-reboot: {max_non_reboot_pos}, pos reboot: {min_reboot_pos})"
    );
}

#[test]
fn test_dual_engine_capacity_boundary() {
    let too_large = MAX_NODES + 1;
    let res = schedule_optimizations_topological(too_large, &[], &[], &vec![false; too_large]);
    assert!(res.is_err());
}
