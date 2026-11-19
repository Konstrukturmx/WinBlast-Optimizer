//! Raven Multi-Criteria Decision & Conflict Resolution Engine for WinBlast0 1.0.
//!
//! Evaluates Pareto-optimal trade-offs across Performance, Privacy, Stability Risk,
//! and Reversibility. Detects and resolves mutual exclusions and conflicts between
//! staged tweaks before any mutations reach the Windows operating system.

use std::collections::{HashMap, HashSet};

/// Multi-criteria valuation vector for a single tweak candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct TweakCriteria {
    pub id: String,
    pub title: String,
    pub performance_gain: f32, // 0.0 .. 10.0
    pub privacy_gain: f32,     // 0.0 .. 10.0
    pub stability_risk: f32,   // 0.0 (Safe) .. 10.0 (Dangerous)
    pub reversibility_score: f32, // 10.0 (Instant) .. 1.0 (Reinstall)
    pub conflicts_with: Vec<String>,
}

/// The result of an automated conflict resolution evaluation.
#[derive(Debug, Clone, PartialEq)]
pub struct ConflictResolution {
    /// The conflicting tweak pair.
    pub tweak_a: String,
    pub tweak_b: String,
    /// The winner chosen by the multi-criteria evaluator.
    pub recommended_winner: String,
    /// Score difference between winner and loser.
    pub margin: f32,
    /// Detailed rationale for the resolution.
    pub rationale: String,
}

/// Multi-Criteria Decision Engine based on the Raven algorithm.
#[derive(Debug, Clone)]
pub struct RavenEngine {
    pub weight_perf: f32,
    pub weight_privacy: f32,
    pub weight_stability: f32,
    pub weight_reversibility: f32,
}

impl Default for RavenEngine {
    fn default() -> Self {
        Self {
            weight_perf: 0.35,
            weight_privacy: 0.25,
            weight_stability: 0.25,
            weight_reversibility: 0.15,
        }
    }
}

impl RavenEngine {
    pub fn new(
        weight_perf: f32,
        weight_privacy: f32,
        weight_stability: f32,
        weight_reversibility: f32,
    ) -> Self {
        let total = weight_perf + weight_privacy + weight_stability + weight_reversibility;
        let norm = if total > 0.0 { total } else { 1.0 };
        Self {
            weight_perf: weight_perf / norm,
            weight_privacy: weight_privacy / norm,
            weight_stability: weight_stability / norm,
            weight_reversibility: weight_reversibility / norm,
        }
    }

    /// Calculate the composite Raven utility score for a given tweak.
    #[inline]
    pub fn compute_score(&self, t: &TweakCriteria) -> f32 {
        let perf_component = t.performance_gain * self.weight_perf;
        let priv_component = t.privacy_gain * self.weight_privacy;
        let rev_component = t.reversibility_score * self.weight_reversibility;
        // Stability risk is a penalty: subtract weighted risk
        let risk_penalty = t.stability_risk * self.weight_stability;

        perf_component + priv_component + rev_component - risk_penalty
    }

    /// Check if tweak A strictly Pareto-dominates tweak B
    /// (A is at least as good as B in all criteria, and strictly better in at least one).
    /// Uses branchless bitwise evaluations for maximum instruction-level throughput.
    #[inline(always)]
    pub fn dominates(&self, a: &TweakCriteria, b: &TweakCriteria) -> bool {
        let all_ge = (a.performance_gain >= b.performance_gain)
            & (a.privacy_gain >= b.privacy_gain)
            & (a.stability_risk <= b.stability_risk)
            & (a.reversibility_score >= b.reversibility_score);

        let any_gt = (a.performance_gain > b.performance_gain)
            | (a.privacy_gain > b.privacy_gain)
            | (a.stability_risk < b.stability_risk)
            | (a.reversibility_score > b.reversibility_score);

        all_ge & any_gt
    }

    /// Detect and resolve all conflicts among a set of staged tweak IDs.
    /// Returns:
    /// - Sanitized list of tweak IDs with conflicts pruned.
    /// - List of resolution explanations.
    pub fn resolve_conflicts(
        &self,
        staged_tweaks: &[TweakCriteria],
    ) -> (Vec<String>, Vec<ConflictResolution>) {
        let n = staged_tweaks.len();
        let mut tweak_map: HashMap<&str, &TweakCriteria> = HashMap::with_capacity(n);
        for t in staged_tweaks {
            tweak_map.insert(t.id.as_str(), t);
        }

        let mut eliminated: HashSet<&str> = HashSet::with_capacity(n / 4);
        let mut resolutions: Vec<ConflictResolution> = Vec::new();

        for a in staged_tweaks {
            let a_id = a.id.as_str();
            if eliminated.contains(a_id) {
                continue;
            }

            for conflict_id in &a.conflicts_with {
                let conf_id = conflict_id.as_str();
                if let Some(&b) = tweak_map.get(conf_id) {
                    let b_id = b.id.as_str();
                    if eliminated.contains(b_id) || a_id == b_id {
                        continue;
                    }

                    // Conflict found between a and b! Evaluate winner using Raven score
                    let score_a = self.compute_score(a);
                    let score_b = self.compute_score(b);
                    let margin = (score_a - score_b).abs();

                    let (winner, loser, rationale) = if self.dominates(a, b) {
                        (
                            a.id.clone(),
                            b.id.clone(),
                            format!(
                                "'{}' strictly Pareto-dominates '{}' across all performance and risk metrics.",
                                a.title, b.title
                            ),
                        )
                    } else if self.dominates(b, a) {
                        (
                            b.id.clone(),
                            a.id.clone(),
                            format!(
                                "'{}' strictly Pareto-dominates '{}' across all performance and risk metrics.",
                                b.title, a.title
                            ),
                        )
                    } else if score_a >= score_b {
                        (
                            a.id.clone(),
                            b.id.clone(),
                            format!(
                                "'{}' scored {:.2} vs {:.2} for '{}' under the Raven multi-criteria utility function.",
                                a.title, score_a, score_b, b.title
                            ),
                        )
                    } else {
                        (
                            b.id.clone(),
                            a.id.clone(),
                            format!(
                                "'{}' scored {:.2} vs {:.2} for '{}' under the Raven multi-criteria utility function.",
                                b.title, score_b, score_a, a.title
                            ),
                        )
                    };

                    let loser_ref = if loser == a.id { a_id } else { b_id };
                    eliminated.insert(loser_ref);
                    resolutions.push(ConflictResolution {
                        tweak_a: a.id.clone(),
                        tweak_b: b.id.clone(),
                        recommended_winner: winner,
                        margin,
                        rationale,
                    });
                }
            }
        }

        let survivors = staged_tweaks
            .iter()
            .filter(|t| !eliminated.contains(t.id.as_str()))
            .map(|t| t.id.clone())
            .collect();

        (survivors, resolutions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raven_pareto_dominance() {
        let raven = RavenEngine::default();

        let superior = TweakCriteria {
            id: "clean_perf".to_string(),
            title: "Clean High Performance".to_string(),
            performance_gain: 8.0,
            privacy_gain: 5.0,
            stability_risk: 0.0,
            reversibility_score: 10.0,
            conflicts_with: vec!["dirty_perf".to_string()],
        };

        let inferior = TweakCriteria {
            id: "dirty_perf".to_string(),
            title: "Risky Performance".to_string(),
            performance_gain: 5.0,
            privacy_gain: 2.0,
            stability_risk: 4.0,
            reversibility_score: 5.0,
            conflicts_with: vec!["clean_perf".to_string()],
        };

        assert!(raven.dominates(&superior, &inferior));
        assert!(!raven.dominates(&inferior, &superior));

        let staged = vec![superior, inferior];
        let (survivors, resolutions) = raven.resolve_conflicts(&staged);

        assert_eq!(survivors, vec!["clean_perf".to_string()]);
        assert_eq!(resolutions.len(), 1);
        assert_eq!(resolutions[0].recommended_winner, "clean_perf");
    }

    #[test]
    fn test_raven_weighted_conflict_resolution() {
        let raven = RavenEngine::new(0.5, 0.1, 0.3, 0.1); // Heavy performance bias

        let tweak_perf = TweakCriteria {
            id: "fps_boost".to_string(),
            title: "FPS Boost".to_string(),
            performance_gain: 9.0,
            privacy_gain: 0.0,
            stability_risk: 1.0,
            reversibility_score: 8.0,
            conflicts_with: vec!["max_battery".to_string()],
        };

        let tweak_battery = TweakCriteria {
            id: "max_battery".to_string(),
            title: "Battery Saver".to_string(),
            performance_gain: 1.0,
            privacy_gain: 0.0,
            stability_risk: 0.0,
            reversibility_score: 10.0,
            conflicts_with: vec!["fps_boost".to_string()],
        };

        let staged = vec![tweak_perf, tweak_battery];
        let (survivors, resolutions) = raven.resolve_conflicts(&staged);

        assert_eq!(survivors, vec!["fps_boost".to_string()]);
        assert_eq!(resolutions[0].recommended_winner, "fps_boost");
    }
}
