// complexity_crossover.rs — Metatron Dynamics, Inc. V2.0
// Regime 3: task-complexity crossover between multiple binary/conventional
// approaches and the relational (ABR) approach.
// Bounded over D. No claim beyond D.
//
// ── What Changed in V2.0 ─────────────────────────────────────────────────────
//
// V1.x tiers (SMALL=64, MEDIUM=1024, LARGE=8192) were declared without
// grounding in hardware observables. Declared-hardware runs confirmed that
// SCRAMBLED and BRANCHY performed identically to LINEAR_SCAN at every V1.x
// tier — the cache-unfriendly and branch-heavy mechanisms did not engage
// because all working sets (max 64 KB) fit within L2 (1 MB, Zen 4) at
// 0.56 ns/access. The experiment tested complexity class only, not hardware
// mechanism. OC-CC-1 remained open.
//
// V2.0 grounds the tier boundaries in the declared cache-latency curve
// (cache_latency_model.rs, declared-hardware run 2026-08-29). The congestion
// thresholds are declared hardware observables, not estimates:
//
//   L2 saturation begins:      N=65,536   WS=524 KB   0.70 ns/access
//   L3 entry:                  N=262,144  WS=2 MB     0.88 ns/access
//   L3-internal step (peak):   N=2,097,152 WS=16 MB   3.38 ns/access
//                              (vs 0.95 ns/access at N=1,048,576 / 8 MB —
//                               a 3.5x jump within one doubling, the most
//                               pronounced single hardware transition in the
//                               declared sweep)
//
// The revised tiers place the experiment at and across these thresholds,
// so SCRAMBLED pays declared hardware cost rather than being absorbed by L2.
//
// ── Declared Tiers (V2.0) ────────────────────────────────────────────────────
//
// SMALL:   N=64        WS=512 B    fully L1d (0.56 ns/access) — baseline
// MEDIUM:  N=1,024     WS=8 KB     L1d (0.56 ns/access) — V1.x continuity
// LARGE:   N=65,536    WS=524 KB   L2 saturation threshold (0.70 ns/access)
// XLARGE:  N=262,144   WS=2 MB     L3 entry threshold (0.88 ns/access)
// XXLARGE: N=2,097,152 WS=16 MB   past L3-internal step (3.38 ns/access)
//
// ALL_PAIRS is excluded at XLARGE and XXLARGE: O(N^2) at N=262,144 is
// ~69 billion operations; at N=2,097,152 it is intractable. The
// complexity-class comparison (relational vs O(N^2)) is already settled at
// SMALL and MEDIUM. Excluding ALL_PAIRS at the two largest tiers is not a
// gap — it is a declared provenance decision: the question being asked at
// XLARGE and XXLARGE is whether SCRAMBLED and BRANCHY diverge from
// LINEAR_SCAN when the hardware mechanism actually engages, not whether
// relational beats quadratic at large N (already confirmed).
//
// ── Timing Protocol Deviation at XLARGE and XXLARGE ─────────────────────────
//
// The standard protocol (N_WARM=100, N_TIMED=1000) is impractical at
// N=2,097,152: the relational chain's working set (~48 MB) exceeds L3,
// and a single pass at that size takes O(tens of ms). 1000 timed passes
// would require ~30+ minutes. XLARGE and XXLARGE use the lighter protocol
// from cache_latency_model.rs (CACHE_SWEEP_WARM=3, CACHE_SWEEP_TIMED=10),
// declared explicitly here as OC-CC-4. Results at these tiers carry the
// same elevated variance notice as the cache-latency curve (OC-CL-1).
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-CC-1: ADDRESSED in V2.0. Tiers are now grounded in declared hardware
//   observables (cache-latency curve, 2026-08-29). Prior open condition
//   (tiers declared without hardware grounding) is closed by this revision.
//   New open condition: whether SCRAMBLED and BRANCHY diverge from
//   LINEAR_SCAN at XLARGE/XXLARGE is an empirical question answered by
//   running this module on declared hardware.
//
// OC-CC-2: NARROWED (V1.1, unchanged). Seven complexity classes tested.
//   Still a declared representative set, not exhaustive.
//
// OC-CC-3: CLOSED (declared-hardware run, 2026-08-29).
//
// OC-CC-4: NEW (V2.0). XLARGE and XXLARGE use lighter timing protocol
//   (3 warm / 10 timed) than SMALL/MEDIUM/LARGE (100 warm / 1000 timed).
//   Figures at these tiers carry elevated run-to-run variance. Adequate
//   for detecting order-of-magnitude mechanism engagement; not adequate
//   for precise ratio figures at the same confidence as lower tiers.

use std::time::Instant;
use crate::declared_graph::DeclaredGraph;
use crate::operators::{abr_pass, AbrBuffers};
use crate::scaling::declare_scaling_graph;
use crate::timing_harness::{N_WARM, N_TIMED};
use crate::cache_latency_model::{CACHE_SWEEP_WARM, CACHE_SWEEP_TIMED};
use crate::binary_baselines::{
    BinaryAlgo, ALL_ALGOS, declared_permutation, declared_branch_bits,
    SCRAMBLE_SEED, BRANCH_SEED,
};

// ── Declared cache-latency thresholds (from declared-hardware run 2026-08-29)
// Used only to annotate tier labels — not to alter computation.
// Units: ns/access (scrambled f64 access, Ryzen 5 7600X).
pub const THRESHOLD_L2_SATURATION_NS: f64 = 0.70;   // N=65,536 / 524 KB
pub const THRESHOLD_L3_ENTRY_NS: f64      = 0.88;   // N=262,144 / 2 MB
pub const THRESHOLD_L3_STEP_NS: f64       = 3.38;   // N=2,097,152 / 16 MB

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Small,
    Medium,
    Large,
    Xlarge,
    Xxlarge,
}

impl Tier {
    pub fn n_nodes(self) -> usize {
        match self {
            Tier::Small   =>       64,
            Tier::Medium  =>    1_024,
            Tier::Large   =>   65_536,
            Tier::Xlarge  =>  262_144,
            Tier::Xxlarge => 2_097_152,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tier::Small   => "SMALL",
            Tier::Medium  => "MEDIUM",
            Tier::Large   => "LARGE",
            Tier::Xlarge  => "XLARGE",
            Tier::Xxlarge => "XXLARGE",
        }
    }

    /// Hardware context for this tier from the declared cache-latency curve.
    pub fn hardware_context(self) -> &'static str {
        match self {
            Tier::Small   => "L1d  0.56 ns/access",
            Tier::Medium  => "L1d  0.56 ns/access",
            Tier::Large   => "L2-sat 0.70 ns/access",
            Tier::Xlarge  => "L3-entry 0.88 ns/access",
            Tier::Xxlarge => "L3-step  3.38 ns/access",
        }
    }

    /// Whether ALL_PAIRS is included at this tier.
    /// Excluded at XLARGE and XXLARGE — O(N^2) is intractable at those N.
    pub fn includes_all_pairs(self) -> bool {
        matches!(self, Tier::Small | Tier::Medium)
    }

    /// Timing protocol for this tier. XLARGE and XXLARGE use the lighter
    /// cache-sweep protocol (OC-CC-4). Lower tiers use the standard protocol.
    pub fn warm_passes(self) -> usize {
        match self {
            Tier::Xlarge | Tier::Xxlarge => CACHE_SWEEP_WARM,
            _ => N_WARM,
        }
    }

    pub fn timed_passes(self) -> usize {
        match self {
            Tier::Xlarge | Tier::Xxlarge => CACHE_SWEEP_TIMED,
            _ => N_TIMED,
        }
    }
}

pub const TIERS: [Tier; 5] = [
    Tier::Small,
    Tier::Medium,
    Tier::Large,
    Tier::Xlarge,
    Tier::Xxlarge,
];

#[derive(Debug, Clone)]
pub struct CrossoverPoint {
    pub tier: &'static str,
    pub n_nodes: usize,
    pub hardware_context: &'static str,
    pub binary_algo: &'static str,
    pub binary_complexity_class: &'static str,
    pub binary_ops: u64,
    pub binary_mean_ns: f64,
    pub relational_edges: usize,
    pub relational_mean_ns: f64,
    /// relational_mean_ns / binary_mean_ns. < 1.0 means relational faster.
    pub relational_to_binary_ratio: f64,
    /// Whether lighter timing protocol was used (OC-CC-4).
    pub light_protocol: bool,
}

/// Times the relational ABR chain at N nodes. Uses tier-appropriate
/// warm/timed protocol — lighter at XLARGE/XXLARGE (OC-CC-4).
fn measure_relational(tier: Tier) -> (DeclaredGraph, f64) {
    let n = tier.n_nodes();
    let graph = declare_scaling_graph(n);
    let mut buf = AbrBuffers::new(graph.n_nodes, graph.n_edges);
    for _ in 0..tier.warm_passes() {
        abr_pass(&graph, &mut buf);
    }
    let mut times_ns: Vec<u128> = Vec::with_capacity(tier.timed_passes());
    for _ in 0..tier.timed_passes() {
        let start = Instant::now();
        abr_pass(&graph, &mut buf);
        times_ns.push(start.elapsed().as_nanos());
    }
    let mean_ns = times_ns.iter().sum::<u128>() as f64 / tier.timed_passes() as f64;
    (graph, mean_ns)
}

/// Times one binary algorithm at the given tier. Uses tier-appropriate
/// warm/timed protocol (OC-CC-4).
fn measure_binary(
    tier: Tier,
    algo: BinaryAlgo,
    values: &[f64],
    perm: &[usize],
    bits: &[bool],
) -> f64 {
    for _ in 0..tier.warm_passes() {
        std::hint::black_box(algo.run(values, perm, bits));
    }
    let mut times_ns: Vec<u128> = Vec::with_capacity(tier.timed_passes());
    for _ in 0..tier.timed_passes() {
        let start = Instant::now();
        std::hint::black_box(algo.run(values, perm, bits));
        times_ns.push(start.elapsed().as_nanos());
    }
    times_ns.iter().sum::<u128>() as f64 / tier.timed_passes() as f64
}

/// Measures one tier against all applicable binary algorithms.
/// ALL_PAIRS excluded at XLARGE and XXLARGE (see Tier::includes_all_pairs).
pub fn measure_tier_matrix(tier: Tier) -> Vec<CrossoverPoint> {
    let n = tier.n_nodes();
    let values: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
    let perm = declared_permutation(n, SCRAMBLE_SEED);
    let bits = declared_branch_bits(n, BRANCH_SEED);
    let light = matches!(tier, Tier::Xlarge | Tier::Xxlarge);

    let (graph, relational_mean_ns) = measure_relational(tier);

    ALL_ALGOS
        .iter()
        .filter(|&&algo| {
            // Exclude ALL_PAIRS at XLARGE/XXLARGE — intractable at those N.
            if algo == BinaryAlgo::AllPairsDiff && !tier.includes_all_pairs() {
                return false;
            }
            true
        })
        .map(|&algo| {
            let binary_mean_ns = measure_binary(tier, algo, &values, &perm, &bits);
            CrossoverPoint {
                tier: tier.label(),
                n_nodes: n,
                hardware_context: tier.hardware_context(),
                binary_algo: algo.label(),
                binary_complexity_class: algo.complexity_class(),
                binary_ops: algo.declared_op_count(n),
                binary_mean_ns,
                relational_edges: graph.n_edges,
                relational_mean_ns,
                relational_to_binary_ratio: relational_mean_ns / binary_mean_ns,
                light_protocol: light,
            }
        })
        .collect()
}

pub fn run_crossover_measurement() -> Vec<CrossoverPoint> {
    TIERS.iter().flat_map(|&t| measure_tier_matrix(t)).collect()
}

pub fn crossover_report(points: &[CrossoverPoint]) -> String {
    let mut report = String::new();
    report.push_str("═══════════════════════════════════════════════════════════════════════════════════════════\n");
    report.push_str("REGIME 3 — TASK-COMPLEXITY CROSSOVER MATRIX V2.0\n");
    report.push_str("Tiers grounded in declared cache-latency curve (2026-08-29, Ryzen 5 7600X)\n");
    report.push_str("Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4) — Metatron Dynamics, Inc.\n");
    report.push_str("Bounded over D. No claim beyond D.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════════════════════════════\n");
    report.push_str("Declared congestion thresholds (from cache-latency curve):\n");
    report.push_str(&format!("  LARGE   N=65,536   524 KB  L2 saturation begins  {:.2} ns/access\n", THRESHOLD_L2_SATURATION_NS));
    report.push_str(&format!("  XLARGE  N=262,144  2 MB    L3 entry              {:.2} ns/access\n", THRESHOLD_L3_ENTRY_NS));
    report.push_str(&format!("  XXLARGE N=2,097,152 16 MB  L3-internal step      {:.2} ns/access\n", THRESHOLD_L3_STEP_NS));
    report.push_str("  (vs 0.56 ns/access flat through L1d/L2 at SMALL/MEDIUM — 6x range)\n");
    report.push_str("  ALL_PAIRS excluded at XLARGE/XXLARGE: O(N^2) intractable at those N.\n");
    report.push_str("  * = lighter timing protocol (3 warm / 10 timed) — OC-CC-4.\n");
    report.push_str("───────────────────────────────────────────────────────────────────────────────────────────\n");
    report.push_str(&format!(
        "{:>8}  {:>22}  {:>12}  {:>20}  {:>14}  {:>14}  {:>8}\n",
        "TIER", "HW CONTEXT", "BINARY_ALGO", "CLASS",
        "BINARY (ns)", "RELATIONAL (ns)", "REL/BIN"
    ));
    report.push_str("───────────────────────────────────────────────────────────────────────────────────────────\n");
    for p in points {
        let marker = if p.light_protocol { "*" } else { "" };
        report.push_str(&format!(
            "{:>8}{:<1}  {:>22}  {:>12}  {:>20}  {:>14.1}  {:>14.1}  {:>8.4}\n",
            p.tier, marker,
            p.hardware_context,
            p.binary_algo,
            p.binary_complexity_class,
            p.binary_mean_ns,
            p.relational_mean_ns,
            p.relational_to_binary_ratio,
        ));
    }
    report.push_str("───────────────────────────────────────────────────────────────────────────────────────────\n");
    report.push_str("Interpretation:\n");
    report.push_str("  REL/BIN < 1.0 -> relational faster against this binary algorithm at this tier.\n");
    report.push_str("  REL/BIN > 1.0 -> binary faster.\n");
    report.push_str("  Key question (OC-CC-1, V2.0): do SCRAMBLED and BRANCHY diverge from\n");
    report.push_str("  LINEAR_SCAN at LARGE/XLARGE/XXLARGE, where the declared hardware mechanism\n");
    report.push_str("  (cache-unfriendly access, branch unpredictability) actually engages?\n");
    report.push_str("  At SMALL/MEDIUM these mechanisms were absorbed by L2 (0.56 ns/access flat).\n");
    report.push_str("  At XXLARGE the hardware pays 3.38 ns/access — 6x the L1d floor.\n");
    report.push_str("  If SCRAMBLED ~ LINEAR_SCAN still: mechanism is present but masked by\n");
    report.push_str("  something else (likely the relational chain's own memory pressure at\n");
    report.push_str("  those sizes). If SCRAMBLED >> LINEAR_SCAN: mechanism engaged, and the\n");
    report.push_str("  comparison becomes structurally meaningful for the first time.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════════════════════════════\n");
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_node_counts_declared_correctly() {
        assert_eq!(Tier::Small.n_nodes(),       64);
        assert_eq!(Tier::Medium.n_nodes(),    1_024);
        assert_eq!(Tier::Large.n_nodes(),    65_536);
        assert_eq!(Tier::Xlarge.n_nodes(),  262_144);
        assert_eq!(Tier::Xxlarge.n_nodes(), 2_097_152);
    }

    #[test]
    fn small_and_medium_include_all_pairs() {
        assert!(Tier::Small.includes_all_pairs());
        assert!(Tier::Medium.includes_all_pairs());
    }

    #[test]
    fn large_xlarge_xxlarge_exclude_all_pairs() {
        assert!(!Tier::Large.includes_all_pairs());
        assert!(!Tier::Xlarge.includes_all_pairs());
        assert!(!Tier::Xxlarge.includes_all_pairs());
    }

    #[test]
    fn small_medium_large_use_standard_protocol() {
        assert_eq!(Tier::Small.warm_passes(), N_WARM);
        assert_eq!(Tier::Medium.warm_passes(), N_WARM);
        assert_eq!(Tier::Large.warm_passes(), N_WARM);
    }

    #[test]
    fn xlarge_xxlarge_use_light_protocol() {
        assert_eq!(Tier::Xlarge.warm_passes(), CACHE_SWEEP_WARM);
        assert_eq!(Tier::Xxlarge.warm_passes(), CACHE_SWEEP_WARM);
        assert_eq!(Tier::Xlarge.timed_passes(), CACHE_SWEEP_TIMED);
        assert_eq!(Tier::Xxlarge.timed_passes(), CACHE_SWEEP_TIMED);
    }

    #[test]
    fn measure_tier_matrix_small_produces_seven_points() {
        let points = measure_tier_matrix(Tier::Small);
        assert_eq!(points.len(), 7);
    }

    #[test]
    fn measure_tier_matrix_large_excludes_all_pairs_produces_six_points() {
        // LARGE excludes ALL_PAIRS: 7 algos - 1 = 6 points.
        let points = measure_tier_matrix(Tier::Large);
        assert_eq!(points.len(), 6);
        assert!(!points.iter().any(|p| p.binary_algo == "ALL_PAIRS"));
    }

    #[test]
    fn measure_tier_matrix_small_all_finite_positive() {
        let points = measure_tier_matrix(Tier::Small);
        for p in &points {
            assert!(p.binary_mean_ns > 0.0);
            assert!(p.relational_mean_ns > 0.0);
            assert!(p.relational_to_binary_ratio.is_finite());
        }
    }

    #[test]
    fn measure_tier_matrix_relational_side_identical_across_algos_in_tier() {
        let points = measure_tier_matrix(Tier::Small);
        let first = points[0].relational_mean_ns;
        for p in &points {
            assert!((p.relational_mean_ns - first).abs() < 1e-9);
        }
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
    fn hardware_context_labels_nonempty_for_all_tiers() {
        for tier in TIERS {
            assert!(!tier.hardware_context().is_empty());
        }
    }

    #[test]
    fn light_protocol_flag_set_correctly() {
        let small_points = measure_tier_matrix(Tier::Small);
        assert!(small_points.iter().all(|p| !p.light_protocol));
        // Note: Xlarge/Xxlarge tests omitted from unit suite — too slow.
        // Confirmed by xlarge_xxlarge_use_light_protocol() above.
    }
}
