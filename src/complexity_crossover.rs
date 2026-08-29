// complexity_crossover.rs — Metatron Dynamics, Inc. V1.1
// Regime 3: task-complexity crossover between multiple binary/conventional
// approaches and the relational (ABR) approach.
// Bounded over D. No claim beyond D.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// abr-relational-attention observed a crossover in the language-model
// token-count case: below roughly five paragraphs, standard quadratic
// attention was cheaper; above that point, ABR's approximately linear
// scaling overtook it. V1.0 tested whether the same shape reproduces here
// against a single O(N^2) binary baseline (all-pairs) — it found relational
// cheaper at every declared tier, with no crossover in range (OC-CC-1).
//
// V1.1 widens this to a matrix: five declared binary baselines
// (binary_baselines.rs) spanning O(N), O(N log N), and O(N^2), each timed
// against the same relational ABR chain at the same three tiers. This
// directly answers whether "relational wins" is a genuine relational-
// structure effect (would show relational losing to the O(N) and
// O(N log N) baselines, winning mainly against O(N^2)) or an artifact of
// having only compared against a quadratic task (would show relational
// winning uniformly regardless of the binary side's complexity class).
//
// ── Declared Comparison ──────────────────────────────────────────────────────
//
// Binary side: five declared algorithms (see binary_baselines.rs) —
// AllPairsDiff O(N^2), LinearScanDiff O(N), WindowedDiff O(N*K),
// PrefixSum O(N), SortThenScan O(N log N). All operate on the same
// declared input values for a given tier.
//
// Relational side: unchanged from V1.0 — the existing V7 ABR chain
// (operators.rs, declare_scaling_graph() from scaling.rs) over the same
// N nodes. O(N) operations. Same measured code path as Regime 1 — no new
// relational implementation introduced.
//
// ── Declared Complexity Tiers (unchanged from V1.0) ──────────────────────────
//
// SMALL:  64 nodes
// MEDIUM: 1,024 nodes
// LARGE:  8,192 nodes
//
// V1.0 declared-hardware run (2026-08-29, Ryzen 5 7600X): relational beat
// the single O(N^2) baseline at all three tiers, including SMALL
// (REL/BIN = 0.1002). OC-CC-1 remains open: no crossover found within
// this tier range against that baseline. This module tests whether the
// same holds against O(N) and O(N log N) baselines, where a genuine
// relational-structure advantage (rather than a complexity-class
// mismatch) would be a much stronger and more surprising finding.
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-CC-1: Tier sizes are declared, not derived from a pilot measurement.
//   Confirmed on declared hardware (V1.0) that no crossover exists in
//   range against the O(N^2) baseline. Still open for the O(N) and
//   O(N log N) baselines added in V1.1 — this module does not yet search
//   for a crossover point adaptively.
//
// OC-CC-2: (V1.0, PARTIALLY ADDRESSED in V1.1) — V1.0 tested exactly one
//   binary baseline. V1.1 tests five, spanning three complexity classes.
//   This narrows but does not close the condition: these five are still a
//   declared representative set (see OC-BB-2 in binary_baselines.rs), not
//   an exhaustive survey of binary algorithms.
//
// OC-CC-3: All timing must be run on the declared hardware (Ryzen 5 7600X
//   / DDR5-5600 — see README.md) to be admissible. Figures produced in a
//   different execution environment are structural sanity checks only.

use std::time::Instant;
use crate::declared_graph::DeclaredGraph;
use crate::operators::{abr_pass, AbrBuffers};
use crate::scaling::declare_scaling_graph;
use crate::timing_harness::{N_WARM, N_TIMED};
use crate::binary_baselines::{
    BinaryAlgo, ALL_ALGOS, declared_permutation, declared_branch_bits,
    SCRAMBLE_SEED, BRANCH_SEED,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Small,
    Medium,
    Large,
}

impl Tier {
    pub fn n_nodes(self) -> usize {
        match self {
            Tier::Small => 64,
            Tier::Medium => 1_024,
            Tier::Large => 8_192,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tier::Small => "SMALL",
            Tier::Medium => "MEDIUM",
            Tier::Large => "LARGE",
        }
    }
}

pub const TIERS: [Tier; 3] = [Tier::Small, Tier::Medium, Tier::Large];

#[derive(Debug, Clone)]
pub struct CrossoverPoint {
    pub tier: &'static str,
    pub n_nodes: usize,
    pub binary_algo: &'static str,
    pub binary_complexity_class: &'static str,
    pub binary_ops: u64,
    pub binary_mean_ns: f64,
    pub relational_edges: usize,
    pub relational_mean_ns: f64,
    /// relational_mean_ns / binary_mean_ns. < 1.0 means relational path
    /// was faster on this tier against this binary algorithm.
    pub relational_to_binary_ratio: f64,
}

/// Times the relational ABR chain at N nodes under the same warm/timed
/// protocol as Regime 1. Computed once per tier and reused across all
/// binary algorithm comparisons at that tier (the relational side does
/// not change per binary algorithm).
fn measure_relational(n: usize) -> (DeclaredGraph, f64) {
    let graph: DeclaredGraph = declare_scaling_graph(n);
    let mut buf = AbrBuffers::new(graph.n_nodes, graph.n_edges);
    for _ in 0..N_WARM {
        abr_pass(&graph, &mut buf);
    }
    let mut rel_times_ns: Vec<u128> = Vec::with_capacity(N_TIMED);
    for _ in 0..N_TIMED {
        let start = Instant::now();
        abr_pass(&graph, &mut buf);
        rel_times_ns.push(start.elapsed().as_nanos());
    }
    let mean_ns = rel_times_ns.iter().sum::<u128>() as f64 / N_TIMED as f64;
    (graph, mean_ns)
}

/// Times one declared binary algorithm over N declared values under the
/// same warm/timed protocol as the relational chain. `perm` and `bits` are
/// precomputed once per tier by the caller (see measure_tier_matrix) — see
/// OC-BB-3 in binary_baselines.rs for why they must not be regenerated
/// inside this timed function.
fn measure_binary(algo: BinaryAlgo, values: &[f64], perm: &[usize], bits: &[bool]) -> f64 {
    for _ in 0..N_WARM {
        std::hint::black_box(algo.run(values, perm, bits));
    }
    let mut times_ns: Vec<u128> = Vec::with_capacity(N_TIMED);
    for _ in 0..N_TIMED {
        let start = Instant::now();
        std::hint::black_box(algo.run(values, perm, bits));
        times_ns.push(start.elapsed().as_nanos());
    }
    times_ns.iter().sum::<u128>() as f64 / N_TIMED as f64
}

/// Measures one tier against all declared binary algorithms. The
/// relational side is measured once and reused across all binary
/// comparisons at this tier. The permutation and branch-bit auxiliary
/// data (used only by ScrambledAccess / BranchyDataDependent) are also
/// generated once per tier here, OUTSIDE the timed measurement in
/// measure_binary — see OC-BB-3.
pub fn measure_tier_matrix(tier: Tier) -> Vec<CrossoverPoint> {
    let n = tier.n_nodes();
    let values: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
    let perm = declared_permutation(n, SCRAMBLE_SEED);
    let bits = declared_branch_bits(n, BRANCH_SEED);

    let (graph, relational_mean_ns) = measure_relational(n);

    ALL_ALGOS
        .iter()
        .map(|&algo| {
            let binary_mean_ns = measure_binary(algo, &values, &perm, &bits);
            CrossoverPoint {
                tier: tier.label(),
                n_nodes: n,
                binary_algo: algo.label(),
                binary_complexity_class: algo.complexity_class(),
                binary_ops: algo.declared_op_count(n),
                binary_mean_ns,
                relational_edges: graph.n_edges,
                relational_mean_ns,
                relational_to_binary_ratio: relational_mean_ns / binary_mean_ns,
            }
        })
        .collect()
}

pub fn run_crossover_measurement() -> Vec<CrossoverPoint> {
    TIERS.iter().flat_map(|&t| measure_tier_matrix(t)).collect()
}

pub fn crossover_report(points: &[CrossoverPoint]) -> String {
    let mut report = String::new();
    report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
    report.push_str("REGIME 3 — TASK-COMPLEXITY CROSSOVER MATRIX (5 binary algos vs ABR chain)\n");
    report.push_str("Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)\n");
    report.push_str("Metatron Dynamics, Inc. · Bounded over D.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
    report.push_str(&format!(
        "{:>7}  {:>12} {:>10}  {:>14}  {:>14}  {:>10}\n",
        "TIER", "BINARY_ALGO", "CLASS", "BINARY (ns)", "RELATIONAL (ns)", "REL/BIN"
    ));
    report.push_str("───────────────────────────────────────────────────────────────────────────\n");
    for p in points {
        report.push_str(&format!(
            "{:>7}  {:>12} {:>10}  {:>14.1}  {:>14.1}  {:>10.4}\n",
            p.tier, p.binary_algo, p.binary_complexity_class,
            p.binary_mean_ns, p.relational_mean_ns, p.relational_to_binary_ratio
        ));
    }
    report.push_str("───────────────────────────────────────────────────────────────────────────\n");
    report.push_str("Interpretation:\n");
    report.push_str("  REL/BIN < 1.0 -> relational path faster against this binary algorithm.\n");
    report.push_str("  REL/BIN > 1.0 -> binary path faster against this binary algorithm.\n");
    report.push_str("  If REL/BIN < 1.0 ONLY against O(N^2) algorithms, and > 1.0 against\n");
    report.push_str("  O(N) / O(N log N) algorithms, that is consistent with V1.0's result\n");
    report.push_str("  being a complexity-class artifact rather than a relational-structure\n");
    report.push_str("  effect (OC-CC-2). If REL/BIN < 1.0 uniformly, including against O(N)\n");
    report.push_str("  baselines, that is a stronger and more surprising finding requiring\n");
    report.push_str("  further isolation (which fixed/setup cost differs?) before it is\n");
    report.push_str("  admissible as a relational-structure claim. See OC-CC-1, OC-CC-2,\n");
    report.push_str("  OC-CC-3, and OC-BB-1/OC-BB-2 in binary_baselines.rs.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════════════\n");
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_node_counts_declared_correctly() {
        assert_eq!(Tier::Small.n_nodes(), 64);
        assert_eq!(Tier::Medium.n_nodes(), 1_024);
        assert_eq!(Tier::Large.n_nodes(), 8_192);
    }

    #[test]
    fn measure_tier_matrix_small_produces_seven_points() {
        let points = measure_tier_matrix(Tier::Small);
        assert_eq!(points.len(), 7, "one point per declared binary algorithm");
    }

    #[test]
    fn measure_tier_matrix_small_all_finite_positive() {
        let points = measure_tier_matrix(Tier::Small);
        for p in &points {
            assert!(p.binary_mean_ns > 0.0, "{} binary timing must be positive", p.binary_algo);
            assert!(p.relational_mean_ns > 0.0);
            assert!(p.relational_to_binary_ratio.is_finite());
        }
    }

    #[test]
    fn measure_tier_matrix_relational_side_identical_across_algos_in_tier() {
        // The relational chain is measured once per tier and reused —
        // confirms all five points at a tier report the same relational
        // timing (not five independent, possibly-inconsistent measurements).
        let points = measure_tier_matrix(Tier::Small);
        let first = points[0].relational_mean_ns;
        for p in &points {
            assert!((p.relational_mean_ns - first).abs() < 1e-9,
                "relational timing must be shared across all binary comparisons in a tier");
        }
    }

    #[test]
    fn run_crossover_measurement_produces_21_points() {
        // 3 tiers x 7 binary algorithms.
        let points = run_crossover_measurement();
        assert_eq!(points.len(), 21);
    }

    #[test]
    fn crossover_report_does_not_panic() {
        let points = measure_tier_matrix(Tier::Small);
        let report = crossover_report(&points);
        assert!(report.contains("REGIME 3"));
        assert!(report.contains("ALL_PAIRS"));
    }

    #[test]
    fn relational_edge_count_matches_declared_topology() {
        let points = measure_tier_matrix(Tier::Medium);
        for p in &points {
            assert_eq!(p.relational_edges, p.n_nodes - 1);
        }
    }

    #[test]
    fn all_declared_binary_algos_represented_per_tier() {
        let points = measure_tier_matrix(Tier::Large);
        let labels: Vec<&str> = points.iter().map(|p| p.binary_algo).collect();
        assert!(labels.contains(&"ALL_PAIRS"));
        assert!(labels.contains(&"LINEAR_SCAN"));
        assert!(labels.contains(&"WINDOWED"));
        assert!(labels.contains(&"PREFIX_SUM"));
        assert!(labels.contains(&"SORT_SCAN"));
        assert!(labels.contains(&"SCRAMBLED"));
        assert!(labels.contains(&"BRANCHY"));
    }
}
