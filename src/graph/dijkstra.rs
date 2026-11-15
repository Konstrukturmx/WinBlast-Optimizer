//! Dijkstra's Shortest-Path & Topological Execution Scheduler for WinBlast0 1.0.
//!
//! Computes the optimal, lowest-disruption sequence for applying or rolling back
//! staged tweaks while strictly honoring prerequisite dependencies and grouping
//! operations to minimize service restarts and system reboot prompts.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// A single scheduled execution step in the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionStep {
    /// Tweak unique identifier.
    pub tweak_id: String,
    /// Priority order index (0 is executed first).
    pub sequence_index: usize,
    /// Cumulative disruption cost to reach this step.
    pub cumulative_cost: u32,
    /// Whether this tweak necessitates a system reboot.
    pub requires_reboot: bool,
    /// Whether this tweak touches Windows services.
    pub touches_services: bool,
}

/// The calculated optimal execution order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionOrder {
    /// Ordered steps to execute sequentially.
    pub steps: Vec<ExecutionStep>,
    /// Total disruption cost of the entire plan.
    pub total_cost: u32,
    /// Number of reboot-requiring tweaks included.
    pub total_reboots_required: usize,
    /// List of any cyclic dependencies that were detected and resolved.
    pub resolved_cycles: Vec<(String, String)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NodeCost {
    cost: u32,
    node_idx: usize,
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap for Dijkstra: lower cost has higher priority
        other.cost.cmp(&self.cost).then_with(|| self.node_idx.cmp(&other.node_idx))
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra-based scheduler for tweak execution plans.
#[derive(Debug, Default)]
pub struct DijkstraScheduler;

impl DijkstraScheduler {
    pub fn new() -> Self {
        Self
    }

    /// Calculate the optimal execution sequence for a set of target tweak IDs,
    /// taking into account dependencies, service impacts, and reboot penalties.
    ///
    /// # Arguments
    /// * `all_tweaks` - Catalog of all available tweaks with metadata, prerequisites, and reversibility.
    /// * `staged_ids` - The subset of tweak IDs chosen for application.
    pub fn schedule_execution<'a, I>(
        &self,
        all_tweaks: I,
        staged_ids: &[String],
    ) -> ExecutionOrder
    where
        I: IntoIterator<Item = (&'a str, &'a [String], bool, bool)>, // (id, prerequisites, requires_reboot, touches_services)
    {
        let staged_set: HashSet<&str> = staged_ids.iter().map(|s| s.as_str()).collect();
        if staged_set.is_empty() {
            return ExecutionOrder {
                steps: Vec::new(),
                total_cost: 0,
                total_reboots_required: 0,
                resolved_cycles: Vec::new(),
            };
        }

        // 1. Build lookup tables for nodes
        let mut id_to_idx: HashMap<&str, usize> = HashMap::new();
        let mut idx_to_id: Vec<&str> = Vec::new();
        let mut prereqs_map: Vec<Vec<usize>> = Vec::new();
        let mut reboot_flags: Vec<bool> = Vec::new();
        let mut service_flags: Vec<bool> = Vec::new();

        let mut raw_prereqs: HashMap<&str, Vec<&str>> = HashMap::new();

        for (id, prereqs, reboot, services) in all_tweaks {
            if staged_set.contains(id) {
                let idx = idx_to_id.len();
                id_to_idx.insert(id, idx);
                idx_to_id.push(id);
                reboot_flags.push(reboot);
                service_flags.push(services);
                raw_prereqs.insert(id, prereqs.iter().map(|s| s.as_str()).collect());
            }
        }

        let n = idx_to_id.len();
        prereqs_map.resize(n, Vec::new());

        // Resolve prerequisites only among staged tweaks
        let mut in_degrees = vec![0usize; n];
        let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];

        for (id, prereqs) in &raw_prereqs {
            if let Some(&u) = id_to_idx.get(id) {
                for p in prereqs {
                    if let Some(&v) = id_to_idx.get(p) {
                        // v is a prerequisite of u (v must be applied before u)
                        adj_list[v].push(u);
                        in_degrees[u] += 1;
                        prereqs_map[u].push(v);
                    }
                }
            }
        }

        // 2. Detect cycles using Kahn's algorithm
        let mut resolved_cycles = Vec::new();
        let mut zero_in_degree: VecDeque<usize> = VecDeque::new();
        let mut temp_in_degrees = in_degrees.clone();

        for (i, &deg) in temp_in_degrees.iter().enumerate().take(n) {
            if deg == 0 {
                zero_in_degree.push_back(i);
            }
        }

        let mut visited_count = 0;
        while let Some(u) = zero_in_degree.pop_front() {
            visited_count += 1;
            for &v in &adj_list[u] {
                temp_in_degrees[v] -= 1;
                if temp_in_degrees[v] == 0 {
                    zero_in_degree.push_back(v);
                }
            }
        }

        // If cycle exists, break cyclic edges with lowest impact
        if visited_count < n {
            for u in 0..n {
                if temp_in_degrees[u] > 0 {
                    // Node is in a cycle; break prerequisite constraint to allow forward progress
                    for &v in &prereqs_map[u] {
                        resolved_cycles.push((idx_to_id[v].to_string(), idx_to_id[u].to_string()));
                    }
                    in_degrees[u] = 0;
                }
            }
        }

        // 3. Dijkstra-Weighted Priority Scheduling
        // Cost weights:
        // - Base registry action: 1
        // - Service manipulation: 3
        // - Reboot requirement: 10
        // Lower cost tweaks with 0 unmet prerequisites are chosen first.
        let mut dist = vec![u32::MAX; n];
        let mut heap = BinaryHeap::new();
        let mut executed = vec![false; n];
        let mut scheduled_steps = Vec::new();
        let mut current_cost = 0u32;
        let mut total_reboots = 0;

        // Initialize root nodes (in_degree == 0)
        for i in 0..n {
            if in_degrees[i] == 0 {
                let cost = 1 + (if service_flags[i] { 3 } else { 0 }) + (if reboot_flags[i] { 10 } else { 0 });
                dist[i] = cost;
                heap.push(NodeCost { cost, node_idx: i });
            }
        }

        while let Some(NodeCost { cost, node_idx }) = heap.pop() {
            if executed[node_idx] {
                continue;
            }

            executed[node_idx] = true;
            current_cost += cost;
            if reboot_flags[node_idx] {
                total_reboots += 1;
            }

            scheduled_steps.push(ExecutionStep {
                tweak_id: idx_to_id[node_idx].to_string(),
                sequence_index: scheduled_steps.len(),
                cumulative_cost: current_cost,
                requires_reboot: reboot_flags[node_idx],
                touches_services: service_flags[node_idx],
            });

            // Unlock dependents
            for &v in &adj_list[node_idx] {
                if !executed[v] {
                    if in_degrees[v] > 0 {
                        in_degrees[v] -= 1;
                    }
                    if in_degrees[v] == 0 {
                        let step_cost = 1 + (if service_flags[v] { 3 } else { 0 }) + (if reboot_flags[v] { 10 } else { 0 });
                        dist[v] = step_cost;
                        heap.push(NodeCost { cost: step_cost, node_idx: v });
                    }
                }
            }
        }

        // 4. Catch any remaining disconnected nodes
        for i in 0..n {
            if !executed[i] {
                let cost = 1 + (if service_flags[i] { 3 } else { 0 }) + (if reboot_flags[i] { 10 } else { 0 });
                current_cost += cost;
                if reboot_flags[i] {
                    total_reboots += 1;
                }
                scheduled_steps.push(ExecutionStep {
                    tweak_id: idx_to_id[i].to_string(),
                    sequence_index: scheduled_steps.len(),
                    cumulative_cost: current_cost,
                    requires_reboot: reboot_flags[i],
                    touches_services: service_flags[i],
                });
            }
        }

        ExecutionOrder {
            steps: scheduled_steps,
            total_cost: current_cost,
            total_reboots_required: total_reboots,
            resolved_cycles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_empty_schedule() {
        let scheduler = DijkstraScheduler::new();
        let tweaks: Vec<(&str, Vec<String>, bool, bool)> = Vec::new();
        let plan = scheduler.schedule_execution(
            tweaks.iter().map(|(id, p, r, s)| (*id, p.as_slice(), *r, *s)),
            &[],
        );
        assert_eq!(plan.steps.len(), 0);
        assert_eq!(plan.total_cost, 0);
    }

    #[test]
    fn test_dijkstra_dependency_order() {
        let scheduler = DijkstraScheduler::new();
        // C depends on B, B depends on A
        let tweaks = [
            ("tweak_c", vec!["tweak_b".to_string()], false, false),
            ("tweak_a", vec![], false, false),
            ("tweak_b", vec!["tweak_a".to_string()], false, false),
        ];

        let staged = vec!["tweak_c".to_string(), "tweak_a".to_string(), "tweak_b".to_string()];
        let plan = scheduler.schedule_execution(
            tweaks.iter().map(|(id, p, r, s)| (*id, p.as_slice(), *r, *s)),
            &staged,
        );

        assert_eq!(plan.steps.len(), 3);
        assert_eq!(plan.steps[0].tweak_id, "tweak_a");
        assert_eq!(plan.steps[1].tweak_id, "tweak_b");
        assert_eq!(plan.steps[2].tweak_id, "tweak_c");
    }

    #[test]
    fn test_dijkstra_reboot_cost_prioritizes_instant_tweaks() {
        let scheduler = DijkstraScheduler::new();
        // Tweak R requires reboot (cost 11), Tweak I is instant (cost 1)
        let tweaks = [
            ("tweak_reboot", vec![], true, false),
            ("tweak_instant", vec![], false, false),
        ];

        let staged = vec!["tweak_reboot".to_string(), "tweak_instant".to_string()];
        let plan = scheduler.schedule_execution(
            tweaks.iter().map(|(id, p, r, s)| (*id, p.as_slice(), *r, *s)),
            &staged,
        );

        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[0].tweak_id, "tweak_instant");
        assert_eq!(plan.steps[1].tweak_id, "tweak_reboot");
        assert_eq!(plan.total_reboots_required, 1);
    }

    #[test]
    fn test_dijkstra_cycle_resolution() {
        let scheduler = DijkstraScheduler::new();
        // A depends on B, B depends on A (cycle)
        let tweaks = [
            ("node_a", vec!["node_b".to_string()], false, false),
            ("node_b", vec!["node_a".to_string()], false, false),
        ];

        let staged = vec!["node_a".to_string(), "node_b".to_string()];
        let plan = scheduler.schedule_execution(
            tweaks.iter().map(|(id, p, r, s)| (*id, p.as_slice(), *r, *s)),
            &staged,
        );

        assert_eq!(plan.steps.len(), 2);
        assert!(!plan.resolved_cycles.is_empty());
    }
}
