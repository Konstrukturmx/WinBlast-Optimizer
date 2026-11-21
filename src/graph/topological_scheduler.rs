//! Zero-allocation, cache-oblivious topological dependency scheduler with
//! Pareto-minimal cycle resolution and reboot-tail batching for up to 256 nodes.
//!
//! Pure Rust (2021 / 2024), `std`-only, zero `unsafe`, zero panicking `unwrap`s.

/// Maximum number of optimization nodes supported by the 256-bit stack bitsets.
pub const MAX_NODES: usize = 256;

/// Fixed-width 256-bit set backed by four 64-bit words on the stack (32 bytes).
///
/// Two `BitSet256` rows fit in a single 64-byte CPU cache line, keeping both
/// `out_adj` and `in_adj` (16 KiB combined) resident in L1 data cache.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BitSet256 {
    words: [u64; 4],
}

impl BitSet256 {
    #[inline]
    const fn new() -> Self {
        Self { words: [0; 4] }
    }

    /// Creates a bitset with bits `0..n` set (`n <= MAX_NODES`).
    #[inline]
    fn full(n: usize) -> Self {
        let mut words = [0u64; 4];
        let clamped = n.min(MAX_NODES);
        for (w, word) in words.iter_mut().enumerate() {
            let start = w << 6;
            if clamped >= start + 64 {
                *word = u64::MAX;
            } else if clamped > start {
                *word = (1u64 << (clamped - start)) - 1;
            }
        }
        Self { words }
    }

    #[inline]
    fn set(&mut self, idx: usize) {
        if idx < MAX_NODES {
            self.words[idx >> 6] |= 1u64 << (idx & 63);
        }
    }

    #[inline]
    fn clear(&mut self, idx: usize) {
        if idx < MAX_NODES {
            self.words[idx >> 6] &= !(1u64 << (idx & 63));
        }
    }

    #[inline]
    fn contains(&self, idx: usize) -> bool {
        if idx < MAX_NODES {
            (self.words[idx >> 6] & (1u64 << (idx & 63))) != 0
        } else {
            false
        }
    }

    #[inline]
    fn count_ones(&self) -> u32 {
        self.words[0].count_ones()
            + self.words[1].count_ones()
            + self.words[2].count_ones()
            + self.words[3].count_ones()
    }

    #[inline]
    fn union(self, other: Self) -> Self {
        Self {
            words: [
                self.words[0] | other.words[0],
                self.words[1] | other.words[1],
                self.words[2] | other.words[2],
                self.words[3] | other.words[3],
            ],
        }
    }

    #[inline]
    fn intersect(self, other: Self) -> Self {
        Self {
            words: [
                self.words[0] & other.words[0],
                self.words[1] & other.words[1],
                self.words[2] & other.words[2],
                self.words[3] & other.words[3],
            ],
        }
    }

    #[inline]
    fn difference(self, other: Self) -> Self {
        Self {
            words: [
                self.words[0] & !other.words[0],
                self.words[1] & !other.words[1],
                self.words[2] & !other.words[2],
                self.words[3] & !other.words[3],
            ],
        }
    }

    /// Removes and returns the lowest set bit index using `tzcnt`/`blsr`.
    #[inline]
    fn pop_first(&mut self) -> Option<usize> {
        for (word_idx, word) in self.words.iter_mut().enumerate() {
            if *word != 0 {
                let bit = word.trailing_zeros() as usize;
                *word &= *word - 1;
                return Some((word_idx << 6) | bit);
            }
        }
        None
    }

    /// Returns the lowest set bit index `>= start`, if any.
    #[inline]
    fn next_set_from(&self, start: usize) -> Option<usize> {
        if start >= MAX_NODES {
            return None;
        }
        let first_word = start >> 6;
        let bit_offset = start & 63;
        let masked = self.words[first_word] & (!0u64 << bit_offset);
        if masked != 0 {
            return Some((first_word << 6) | (masked.trailing_zeros() as usize));
        }
        for (offset, &word) in self.words[first_word + 1..].iter().enumerate() {
            if word != 0 {
                let word_idx = first_word + 1 + offset;
                return Some((word_idx << 6) | (word.trailing_zeros() as usize));
            }
        }
        None
    }

    #[inline]
    fn iter(self) -> BitSetIter {
        BitSetIter {
            words: self.words,
            word_idx: 0,
        }
    }
}

/// Zero-allocation ascending bit iterator over a `BitSet256`.
struct BitSetIter {
    words: [u64; 4],
    word_idx: usize,
}

impl Iterator for BitSetIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.word_idx < 4 {
            let word = self.words[self.word_idx];
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                self.words[self.word_idx] = word & (word - 1);
                return Some((self.word_idx << 6) | bit);
            }
            self.word_idx += 1;
        }
        None
    }
}

/// Stack-resident output buffer for iterative Tarjan Strongly Connected Components.
struct SccResult {
    id: [u16; MAX_NODES],
    size: [u16; MAX_NODES],
    has_cycle: bool,
}

impl SccResult {
    #[inline]
    const fn new() -> Self {
        Self {
            id: [0; MAX_NODES],
            size: [0; MAX_NODES],
            has_cycle: false,
        }
    }
}

/// Schedules Windows system optimization tweaks in topological dependency order,
/// resolving cycles via minimum-weight edge removal and batching reboot-required
/// tweaks at the tail.
///
/// # Graph & Weight Semantics
/// - `num_nodes`: Number of tweak nodes (`0..num_nodes`), up to [`MAX_NODES`] (256).
/// - `dependencies`: Directed edges `(prereq, dependent)` meaning `prereq` must execute
///   before `dependent`. Self-loops `(u, u)` are treated as trivial cycles and dropped.
/// - `weights`: Per-edge penalty weights aligned 1-to-1 with `dependencies`. When a cycle
///   exists, the intra-SCC dependency edge with the lowest weight (saturating sum for
///   parallel edges) is dropped to minimize Pareto penalty.
/// - `reboot_required`: Per-node flag aligned 1-to-1 with `0..num_nodes`.
///
/// # Allocation & Cache Profile
/// All graph storage, Tarjan SCC frames, and Kahn priority sets live on the stack
/// (~20 KiB peak, L1-cache resident). The only heap allocation is the single
/// pre-sized `Vec<usize>` materialized for the return value.
pub fn schedule_optimizations_topological(
    num_nodes: usize,
    dependencies: &[(usize, usize)],
    weights: &[u32],
    reboot_required: &[bool],
) -> Result<Vec<usize>, String> {
    if num_nodes > MAX_NODES {
        return Err(format!(
            "num_nodes ({num_nodes}) exceeds maximum supported capacity of {MAX_NODES}"
        ));
    }
    if reboot_required.len() != num_nodes {
        return Err(format!(
            "reboot_required length ({}) does not match num_nodes ({num_nodes})",
            reboot_required.len()
        ));
    }
    if weights.len() != dependencies.len() {
        return Err(format!(
            "weights length ({}) does not match dependencies length ({})",
            weights.len(),
            dependencies.len()
        ));
    }

    // Build bit-parallel forward and reverse adjacency matrices on the stack.
    let mut out_adj = [BitSet256::new(); MAX_NODES];
    let mut in_adj = [BitSet256::new(); MAX_NODES];
    let mut has_parallel_edges = false;

    for &(u, v) in dependencies {
        if u >= num_nodes || v >= num_nodes {
            return Err(format!(
                "dependency edge ({u}, {v}) out of bounds for num_nodes = {num_nodes}"
            ));
        }
        // Self-loops form a 1-node cycle that can only be broken by dropping (u, u).
        if u == v {
            continue;
        }
        if out_adj[u].contains(v) {
            has_parallel_edges = true;
        } else {
            out_adj[u].set(v);
            in_adj[v].set(u);
        }
    }

    if num_nodes == 0 {
        return Ok(Vec::new());
    }

    // Fast path: run reboot-aware Kahn's algorithm directly.
    // On acyclic graphs this finishes in a single O(V + E) pass with zero SCC work.
    let mut order_buf = [0u8; MAX_NODES];
    let scheduled = kahn_schedule(
        num_nodes,
        &out_adj,
        &in_adj,
        reboot_required,
        &mut order_buf,
    );

    if scheduled < num_nodes {
        // Cycle detected: eliminate all strongly connected components of size > 1
        // by iteratively dropping the lowest-weight intra-SCC edge, then re-run
        // Kahn's algorithm on the resulting DAG for globally optimal reboot batching.
        resolve_cycles(
            num_nodes,
            dependencies,
            weights,
            &mut out_adj,
            &mut in_adj,
            has_parallel_edges,
        )?;

        let final_scheduled = kahn_schedule(
            num_nodes,
            &out_adj,
            &in_adj,
            reboot_required,
            &mut order_buf,
        );

        if final_scheduled != num_nodes {
            return Err(format!(
                "internal invariant error: scheduled {final_scheduled} of {num_nodes} nodes after cycle resolution"
            ));
        }
    }

    let mut result = Vec::with_capacity(num_nodes);
    for &node in &order_buf[..num_nodes] {
        result.push(usize::from(node));
    }
    Ok(result)
}

/// Runs reboot-aware Kahn's topological sort using three stack-allocated priority bitsets:
/// 1. `ready_non_reboot`: non-reboot tweaks (always executed first).
/// 2. `ready_blocking_reboot`: reboot tweaks that have at least one non-reboot descendant
///    (executed only when Tier 1 is empty, immediately unblocking remaining non-reboot work).
/// 3. `ready_tail_reboot`: pure-tail reboot tweaks with no non-reboot descendants
///    (guaranteed to batch cleanly at the end of the schedule).
fn kahn_schedule(
    num_nodes: usize,
    out_adj: &[BitSet256; MAX_NODES],
    in_adj: &[BitSet256; MAX_NODES],
    reboot_required: &[bool],
    order_buf: &mut [u8; MAX_NODES],
) -> usize {
    // Reverse bitset BFS to identify every node that can reach a non-reboot node.
    // Terminates in <= num_nodes steps even when cycles are present.
    let mut non_reboot_mask = BitSet256::new();
    for (i, &needs_reboot) in reboot_required.iter().enumerate() {
        if !needs_reboot {
            non_reboot_mask.set(i);
        }
    }
    let mut reaches_non_reboot = non_reboot_mask;
    let mut frontier = non_reboot_mask;
    while let Some(v) = frontier.pop_first() {
        let new_preds = in_adj[v].difference(reaches_non_reboot);
        reaches_non_reboot = reaches_non_reboot.union(new_preds);
        frontier = frontier.union(new_preds);
    }

    let mut in_degree = [0u16; MAX_NODES];
    let mut ready_non_reboot = BitSet256::new();
    let mut ready_blocking_reboot = BitSet256::new();
    let mut ready_tail_reboot = BitSet256::new();

    for v in 0..num_nodes {
        let deg = in_adj[v].count_ones() as u16;
        in_degree[v] = deg;
        if deg == 0 {
            if !reboot_required[v] {
                ready_non_reboot.set(v);
            } else if reaches_non_reboot.contains(v) {
                ready_blocking_reboot.set(v);
            } else {
                ready_tail_reboot.set(v);
            }
        }
    }

    let mut scheduled = 0usize;
    while let Some(u) = ready_non_reboot
        .pop_first()
        .or_else(|| ready_blocking_reboot.pop_first())
        .or_else(|| ready_tail_reboot.pop_first())
    {
        order_buf[scheduled] = u as u8;
        scheduled += 1;

        for v in out_adj[u].iter() {
            in_degree[v] = in_degree[v].saturating_sub(1);
            if in_degree[v] == 0 {
                if !reboot_required[v] {
                    ready_non_reboot.set(v);
                } else if reaches_non_reboot.contains(v) {
                    ready_blocking_reboot.set(v);
                } else {
                    ready_tail_reboot.set(v);
                }
            }
        }
    }

    scheduled
}

/// Iteratively computes SCCs via non-recursive Tarjan's algorithm and breaks each
/// cycle by removing the lowest-weight intra-SCC edge until the graph is a DAG.
fn resolve_cycles(
    num_nodes: usize,
    dependencies: &[(usize, usize)],
    weights: &[u32],
    out_adj: &mut [BitSet256; MAX_NODES],
    in_adj: &mut [BitSet256; MAX_NODES],
    has_parallel_edges: bool,
) -> Result<(), String> {
    let active_nodes = BitSet256::full(num_nodes);
    let mut scc = SccResult::new();

    loop {
        tarjan_scc(active_nodes, out_adj, &mut scc);
        if !scc.has_cycle {
            return Ok(());
        }

        let Some((u, v)) = find_min_weight_cycle_edge(
            num_nodes,
            dependencies,
            weights,
            out_adj,
            &scc,
            has_parallel_edges,
        ) else {
            return Err(
                "internal invariant error: cycle detected with no removable intra-SCC edge"
                    .to_string(),
            );
        };

        out_adj[u].clear(v);
        in_adj[v].clear(u);
    }
}

/// Non-recursive, zero-allocation Tarjan's Strongly Connected Components algorithm.
///
/// An edge `(u, v)` participates in a directed cycle if and only if `scc.id[u] == scc.id[v]`
/// and `scc.size[scc.id[u]] > 1`. Isolating SCCs guarantees non-cyclic edges (such as
/// downstream tails with low weights) are never erroneously dropped.
fn tarjan_scc(active_nodes: BitSet256, out_adj: &[BitSet256; MAX_NODES], scc: &mut SccResult) {
    const UNVISITED: u16 = u16::MAX;
    scc.id.fill(0);
    scc.size.fill(0);
    scc.has_cycle = false;

    let mut index = [UNVISITED; MAX_NODES];
    let mut lowlink = [0u16; MAX_NODES];
    let mut on_stack = BitSet256::new();
    let mut tarjan_stack = [0u8; MAX_NODES];
    let mut tarjan_top = 0usize;

    // Explicit DFS call stack: (node_id, next_neighbor_bit_offset).
    let mut call_node = [0u8; MAX_NODES];
    let mut call_next = [0u16; MAX_NODES];

    let mut current_index = 0u16;
    let mut scc_count = 0u16;

    for start in active_nodes.iter() {
        if index[start] != UNVISITED {
            continue;
        }

        index[start] = current_index;
        lowlink[start] = current_index;
        current_index += 1;

        tarjan_stack[tarjan_top] = start as u8;
        tarjan_top += 1;
        on_stack.set(start);

        call_node[0] = start as u8;
        call_next[0] = 0;
        let mut call_depth = 1usize;

        while call_depth > 0 {
            let frame = call_depth - 1;
            let v = usize::from(call_node[frame]);
            let search_from = usize::from(call_next[frame]);

            if let Some(w) = out_adj[v]
                .intersect(active_nodes)
                .next_set_from(search_from)
            {
                call_next[frame] = (w + 1) as u16;

                if index[w] == UNVISITED {
                    index[w] = current_index;
                    lowlink[w] = current_index;
                    current_index += 1;

                    tarjan_stack[tarjan_top] = w as u8;
                    tarjan_top += 1;
                    on_stack.set(w);

                    call_node[call_depth] = w as u8;
                    call_next[call_depth] = 0;
                    call_depth += 1;
                } else if on_stack.contains(w) && index[w] < lowlink[v] {
                    lowlink[v] = index[w];
                }
            } else {
                call_depth -= 1;

                if lowlink[v] == index[v] {
                    let id = scc_count;
                    scc_count += 1;
                    let mut size = 0u16;

                    while tarjan_top > 0 {
                        tarjan_top -= 1;
                        let member = usize::from(tarjan_stack[tarjan_top]);
                        on_stack.clear(member);
                        scc.id[member] = id;
                        size += 1;
                        if member == v {
                            break;
                        }
                    }
                    scc.size[usize::from(id)] = size;
                    if size > 1 {
                        scc.has_cycle = true;
                    }
                }

                if call_depth > 0 {
                    let parent = usize::from(call_node[call_depth - 1]);
                    if lowlink[v] < lowlink[parent] {
                        lowlink[parent] = lowlink[v];
                    }
                }
            }
        }
    }
}

/// Selects the active intra-SCC edge `(u, v)` with minimum Pareto penalty `(weight, u, v)`.
#[allow(clippy::needless_range_loop)]
fn find_min_weight_cycle_edge(
    num_nodes: usize,
    dependencies: &[(usize, usize)],
    weights: &[u32],
    out_adj: &[BitSet256; MAX_NODES],
    scc: &SccResult,
    has_parallel_edges: bool,
) -> Option<(usize, usize)> {
    let mut best: Option<(u32, usize, usize)> = None;

    if !has_parallel_edges {
        for (&(u, v), &w) in dependencies.iter().zip(weights.iter()) {
            if u == v || !out_adj[u].contains(v) {
                continue;
            }
            let su = scc.id[u];
            if su == scc.id[v] && scc.size[usize::from(su)] > 1 {
                let candidate = (w, u, v);
                let is_better = match best {
                    None => true,
                    Some(curr) => candidate < curr,
                };
                if is_better {
                    best = Some(candidate);
                }
            }
        }
        return best.map(|(_, u, v)| (u, v));
    }

    // Parallel edges exist: sum weights per unique active intra-SCC edge (u, v)
    // using a single 1 KiB stack row buffer (zero heap allocation).
    let mut row_weights = [0u32; MAX_NODES];
    for u in 0..num_nodes {
        let su = scc.id[u];
        if scc.size[usize::from(su)] <= 1 {
            continue;
        }

        let mut active_targets = BitSet256::new();
        for (&(src, dst), &w) in dependencies.iter().zip(weights.iter()) {
            if src == u && src != dst && out_adj[u].contains(dst) && scc.id[dst] == su {
                if !active_targets.contains(dst) {
                    active_targets.set(dst);
                    row_weights[dst] = w;
                } else {
                    row_weights[dst] = row_weights[dst].saturating_add(w);
                }
            }
        }

        for v in active_targets.iter() {
            let candidate = (row_weights[v], u, v);
            let is_better = match best {
                None => true,
                Some(curr) => candidate < curr,
            };
            if is_better {
                best = Some(candidate);
            }
        }
    }

    best.map(|(_, u, v)| (u, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position_of(order: &[usize], node: usize) -> usize {
        order
            .iter()
            .position(|&x| x == node)
            .expect("scheduled output must contain every node")
    }

    fn assert_valid_permutation(order: &[usize], num_nodes: usize) {
        assert_eq!(order.len(), num_nodes);
        let mut seen = [false; MAX_NODES];
        for &n in order {
            assert!(n < num_nodes, "node {n} out of range");
            assert!(!seen[n], "duplicate node {n} in schedule");
            seen[n] = true;
        }
    }

    /// (a) Linear dependency sequence given in scrambled input order.
    #[test]
    fn test_linear_dependency_sequence() {
        let num_nodes = 5;
        let deps = [(3, 4), (1, 2), (0, 1), (2, 3)];
        let weights = [10, 20, 30, 40];
        let reboots = [false; 5];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("linear graph should schedule cleanly");
        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    }

    /// (b) Diamond dependency graph: 0 -> {1, 2} -> 3.
    #[test]
    fn test_diamond_dependency_graph() {
        let num_nodes = 4;
        let deps = [(0, 1), (0, 2), (1, 3), (2, 3)];
        let weights = [10, 10, 10, 10];
        let reboots = [false; 4];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("diamond graph should schedule cleanly");
        assert_valid_permutation(&order, num_nodes);
        assert_eq!(order[0], 0);
        assert_eq!(order[3], 3);
        for &(u, v) in &deps {
            assert!(position_of(&order, u) < position_of(&order, v));
        }
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    /// (c) Cyclic dependency resolved by dropping ONLY the lowest-weight intra-SCC edge,
    /// leaving a lower-weight non-cyclic downstream edge (2 -> 3, weight 1) intact.
    #[test]
    fn test_cyclic_dependency_lowest_weight_break() {
        let num_nodes = 4;
        // Cycle: 0 -> 1 (50), 1 -> 2 (10), 2 -> 0 (30).
        // Downstream tail (NOT in cycle): 2 -> 3 (1).
        let deps = [(0, 1), (1, 2), (2, 0), (2, 3)];
        let weights = [50, 10, 30, 1];
        let reboots = [false; 4];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("cyclic graph should resolve and schedule");
        assert_valid_permutation(&order, num_nodes);

        // Edge (1, 2) must be dropped; (2, 0), (0, 1), and (2, 3) must all hold.
        assert!(position_of(&order, 2) < position_of(&order, 0));
        assert!(position_of(&order, 0) < position_of(&order, 1));
        assert!(position_of(&order, 2) < position_of(&order, 3));
        assert_eq!(order, vec![2, 0, 1, 3]);
    }

    /// (d) Reboot grouping at the tail: non-reboot tweaks execute first even when
    /// reboot tweaks have lower node IDs and zero prerequisites.
    #[test]
    fn test_reboot_grouping_at_tail() {
        let num_nodes = 6;
        // Non-reboot: 1 -> 4, and isolated 5.
        // Reboot: 0 -> 2, and 1 -> 3.
        let deps = [(0, 2), (1, 4), (1, 3)];
        let weights = [10, 10, 10];
        let reboots = [true, false, true, true, false, false];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("reboot-grouped DAG should schedule");
        assert_valid_permutation(&order, num_nodes);

        for &(u, v) in &deps {
            assert!(position_of(&order, u) < position_of(&order, v));
        }

        // All 3 non-reboot tweaks must precede all 3 reboot tweaks.
        assert!(order[..3].iter().all(|&n| !reboots[n]));
        assert!(order[3..].iter().all(|&n| reboots[n]));
        assert_eq!(order, vec![1, 4, 5, 0, 2, 3]);
    }

    /// Reboot tweak blocking a non-reboot descendant is scheduled before pure-tail
    /// reboot tweaks, keeping unrelated reboot tweaks strictly at the end.
    #[test]
    fn test_blocking_reboot_precedes_pure_tail_reboot() {
        let num_nodes = 4;
        // 0 (reboot) -> 1 (non-reboot); 2 (non-reboot) independent; 3 (reboot) independent.
        let deps = [(0, 1)];
        let weights = [10];
        let reboots = [true, false, false, true];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("should schedule with minimal tail disruption");
        assert_eq!(order, vec![2, 0, 1, 3]);
    }

    /// Figure-eight overlapping cycles: dropping the cheapest edge breaks one loop
    /// and re-running Tarjan isolates the second loop without disturbing acyclic edges.
    #[test]
    fn test_overlapping_figure_eight_cycles() {
        let num_nodes = 3;
        // Loop A: 0 -> 1 (5), 1 -> 0 (20)
        // Loop B: 1 -> 2 (15), 2 -> 1 (8)
        let deps = [(0, 1), (1, 0), (1, 2), (2, 1)];
        let weights = [5, 20, 15, 8];
        let reboots = [false; 3];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("overlapping cycles should resolve");
        // Drops (0, 1) [w=5] then (2, 1) [w=8]; retains (1, 0) and (1, 2).
        assert_eq!(order, vec![1, 0, 2]);
    }

    /// Self-loops are dropped and parallel edge weights accumulate via saturating_add.
    #[test]
    fn test_self_loop_and_parallel_edge_weights() {
        let num_nodes = 2;
        // Parallel (0, 1) edges have combined weight 4 + 4 = 8 > weight 6 of (1, 0).
        let deps = [(0, 0), (0, 1), (0, 1), (1, 0)];
        let weights = [100, 4, 4, 6];
        let reboots = [false, false];

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("self-loop and parallel cycle should resolve");
        // (1, 0) [w=6] is dropped instead of combined (0, 1) [w=8].
        assert_eq!(order, vec![0, 1]);
    }

    /// Exercises all four 64-bit bitset words at full 256-node capacity.
    #[test]
    fn test_max_capacity_256_nodes() {
        let num_nodes = MAX_NODES;
        let mut deps = Vec::with_capacity(MAX_NODES);
        let mut weights = Vec::with_capacity(MAX_NODES);
        let mut reboots = [false; MAX_NODES];

        for i in 0..MAX_NODES - 1 {
            deps.push((i, i + 1));
            weights.push((i as u32) + 10);
        }
        // Close a full 256-node cycle with the lowest-weight back-edge (255 -> 0, weight 1).
        deps.push((MAX_NODES - 1, 0));
        weights.push(1);
        reboots[MAX_NODES - 1] = true;

        let order = schedule_optimizations_topological(num_nodes, &deps, &weights, &reboots)
            .expect("256-node cyclic graph should resolve");

        let expected: Vec<usize> = (0..MAX_NODES).collect();
        assert_eq!(order, expected);
    }

    /// Boundary and malformed input validation returns descriptive errors without panicking.
    #[test]
    fn test_validation_and_empty_graph() {
        assert_eq!(
            schedule_optimizations_topological(0, &[], &[], &[]),
            Ok(Vec::new())
        );

        let too_many_reboots = vec![false; MAX_NODES + 1];
        assert!(
            schedule_optimizations_topological(MAX_NODES + 1, &[], &[], &too_many_reboots).is_err()
        );
        assert!(schedule_optimizations_topological(3, &[(0, 1)], &[10, 20], &[false; 3]).is_err());
        assert!(schedule_optimizations_topological(3, &[(0, 1)], &[10], &[false; 2]).is_err());
        assert!(schedule_optimizations_topological(3, &[(0, 3)], &[10], &[false; 3]).is_err());
    }
}
