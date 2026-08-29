// cache_latency_model.rs — Metatron Dynamics, Inc. V1.3
// Standalone hardware mechanism characterization: cache-latency curve.
// Bounded over D. No claim beyond D.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// NOT a binary-vs-relational comparison. This module characterizes ONE
// mechanism directly: how per-access latency changes as a scrambled-order
// (cache-unfriendly) working set grows past this hardware's L1, L2, and L3
// cache boundaries. The output is an empirical function — ns per access as
// a function of working-set size — not a verdict about relational math.
//
// This is the mathematical object underlying "cache-unfriendly data slows
// binary processing": not a fixed property of an algorithm, but a
// working-set-size-dependent step function tied to the declared hardware's
// cache hierarchy (see README.md, Declared Hardware — Ryzen 5 7600X:
// L1d 32 KB/core, L2 1 MB/core, L3 32 MB shared, Zen 4).
//
// ── Declared Method ───────────────────────────────────────────────────────────
//
// Reuses the scrambled-access primitive from binary_baselines.rs (fixed
// pseudo-random index permutation, same SCRAMBLE_SEED) so results are
// consistent with Regime 3's ScrambledAccess baseline. For each declared
// working-set size N, measures mean wall-clock time for one full scrambled
// pass over N f64 values, then derives ns_per_access = mean_ns / N.
//
// Sweep spans doublings of N from well within L1 (4 KB) to beyond L3
// (64 MB): 512, 1024, ..., 8,388,608 elements (15 points, each 2x the
// prior). If real cache-boundary effects exist on this hardware, ns_per
// _access should show step increases near the declared L1/L2/L3 boundaries
// rather than a smooth curve.
//
// ── Declared Timing Protocol (LIGHTER than Regime 1/3) ───────────────────────
//
// N_WARM=3, N_TIMED=10 — NOT the same as timing_harness.rs's N_WARM=100/
// N_TIMED=1000. At the largest declared N (8,388,608), a single scrambled
// pass may cost hundreds of milliseconds if memory-bound; the full
// 100-warm/1000-timed protocol would make this sweep impractically slow.
// This is a declared, explicit deviation — see OC-CL-1.
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-CL-1: Lighter timing protocol (3 warm / 10 timed passes vs Regime 1/3's
//   100/1000) means individual ns_per_access figures at each N carry more
//   run-to-run variance than Regime 1's scaling measurement. Adequate for
//   locating order-of-magnitude cache-boundary steps; NOT adequate for
//   precise per-tier figures at the same confidence as Regime 1.
//
// OC-CL-2: The permutation array itself (Vec<usize>, N * 8 bytes) is read
//   sequentially during the pass and also occupies cache. Declared working
//   -set size here counts only the values array (N * 8 bytes) as the
//   primary variable under test; the permutation array's own footprint is
//   a secondary, sequentially-accessed structure not expected to dominate
//   cache pressure the way the scrambled values access does.
//
// OC-CL-3: This characterizes ONE hardware mechanism (cache-boundary
//   latency) on ONE declared machine. It does not extend to branch
//   -misprediction cost (a declared next module) or to any other
//   processor's cache hierarchy.

use std::time::Instant;
use crate::binary_baselines::{declared_permutation, SCRAMBLE_SEED};

/// Lighter warm/timed protocol than Regime 1/3 — see OC-CL-1.
pub const CACHE_SWEEP_WARM: usize = 3;
pub const CACHE_SWEEP_TIMED: usize = 10;

/// Declared sweep sizes: 15 doublings from 4 KB to 64 MB (f64 values).
/// 512 * 8 bytes = 4,096 bytes (well within L1d, 32 KB).
/// 8,388,608 * 8 bytes = 67,108,864 bytes (beyond L3, 32 MB).
pub const SWEEP_SIZES: [usize; 15] = [
    512, 1_024, 2_048, 4_096, 8_192, 16_384, 32_768, 65_536,
    131_072, 262_144, 524_288, 1_048_576, 2_097_152, 4_194_304, 8_388_608,
];

#[derive(Debug, Clone)]
pub struct CachePoint {
    pub n: usize,
    pub working_set_bytes: usize,
    pub mean_ns_per_pass: f64,
    pub ns_per_access: f64,
}

fn scrambled_pass(values: &[f64], perm: &[usize]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        acc += values[perm[i]] - values[perm[i - 1]];
    }
    std::hint::black_box(acc)
}

/// Measures one point on the cache-latency curve at declared size N.
/// Permutation generated once, outside the timed loop (OC-BB-3 principle,
/// binary_baselines.rs) — its cost must not be conflated with access cost.
pub fn measure_cache_point(n: usize) -> CachePoint {
    let values: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
    let perm = declared_permutation(n, SCRAMBLE_SEED);

    for _ in 0..CACHE_SWEEP_WARM {
        std::hint::black_box(scrambled_pass(&values, &perm));
    }
    let mut times_ns: Vec<u128> = Vec::with_capacity(CACHE_SWEEP_TIMED);
    for _ in 0..CACHE_SWEEP_TIMED {
        let start = Instant::now();
        std::hint::black_box(scrambled_pass(&values, &perm));
        times_ns.push(start.elapsed().as_nanos());
    }
    let mean_ns = times_ns.iter().sum::<u128>() as f64 / CACHE_SWEEP_TIMED as f64;

    CachePoint {
        n,
        working_set_bytes: n * 8,
        mean_ns_per_pass: mean_ns,
        ns_per_access: mean_ns / n as f64,
    }
}

pub fn run_cache_sweep() -> Vec<CachePoint> {
    SWEEP_SIZES.iter().map(|&n| measure_cache_point(n)).collect()
}

/// Declared cache boundaries for the declared hardware (Ryzen 5 7600X,
/// Zen 4) — used only to annotate the report, not to force the measured
/// curve. See README.md, Declared Hardware.
const L1D_BYTES: usize = 32 * 1024;
const L2_BYTES: usize = 1024 * 1024;
const L3_BYTES: usize = 32 * 1024 * 1024;

pub fn cache_sweep_report(points: &[CachePoint]) -> String {
    let mut report = String::new();
    report.push_str("═══════════════════════════════════════════════════════════════════\n");
    report.push_str("CACHE-LATENCY CURVE — SCRAMBLED-ACCESS WORKING SET SWEEP\n");
    report.push_str("Ryzen 5 7600X / DDR5-5600 / L1d 32KB, L2 1MB, L3 32MB (Zen 4)\n");
    report.push_str("Metatron Dynamics, Inc. · Bounded over D.\n");
    report.push_str("NOT a binary-vs-relational comparison — see OC-CL-1..3.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════\n");
    report.push_str(&format!(
        "{:>10}  {:>12}  {:>14}  {:>12}  {}\n",
        "N", "WS (bytes)", "MEAN (ns)", "NS/ACCESS", "TIER"
    ));
    report.push_str("───────────────────────────────────────────────────────────────────\n");
    for p in points {
        let tier = if p.working_set_bytes <= L1D_BYTES {
            "<= L1d"
        } else if p.working_set_bytes <= L2_BYTES {
            "<= L2"
        } else if p.working_set_bytes <= L3_BYTES {
            "<= L3"
        } else {
            "> L3 (RAM)"
        };
        report.push_str(&format!(
            "{:>10}  {:>12}  {:>14.1}  {:>12.4}  {}\n",
            p.n, p.working_set_bytes, p.mean_ns_per_pass, p.ns_per_access, tier
        ));
    }
    report.push_str("───────────────────────────────────────────────────────────────────\n");
    report.push_str("Interpretation:\n");
    report.push_str("  Declared boundaries: L1d=32KB, L2=1MB, L3=32MB (Zen 4, this chip).\n");
    report.push_str("  A real cache-boundary effect shows as a STEP UP in NS/ACCESS near\n");
    report.push_str("  each boundary, not a smooth curve. Absence of a step at a declared\n");
    report.push_str("  boundary is itself informative (OC-CL-3) — it would mean this\n");
    report.push_str("  mechanism is not the dominant cost driver at that transition on\n");
    report.push_str("  this hardware for this access pattern.\n");
    report.push_str("═══════════════════════════════════════════════════════════════════\n");
    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_sizes_span_declared_range() {
        assert_eq!(*SWEEP_SIZES.first().unwrap() * 8, 4_096);
        assert_eq!(*SWEEP_SIZES.last().unwrap() * 8, 67_108_864);
    }

    #[test]
    fn sweep_sizes_each_double_prior() {
        for w in SWEEP_SIZES.windows(2) {
            assert_eq!(w[1], w[0] * 2, "each declared size must be double the prior");
        }
    }

    #[test]
    fn measure_cache_point_small_produces_finite_positive() {
        let p = measure_cache_point(512);
        assert_eq!(p.n, 512);
        assert_eq!(p.working_set_bytes, 4_096);
        assert!(p.mean_ns_per_pass > 0.0);
        assert!(p.ns_per_access > 0.0 && p.ns_per_access.is_finite());
    }

    #[test]
    fn measure_cache_point_working_set_matches_n_times_8() {
        let p = measure_cache_point(4_096);
        assert_eq!(p.working_set_bytes, 4_096 * 8);
    }

    #[test]
    fn cache_sweep_report_does_not_panic() {
        let p = measure_cache_point(512);
        let report = cache_sweep_report(&[p]);
        assert!(report.contains("CACHE-LATENCY CURVE"));
        assert!(report.contains("<= L1d"));
    }

    #[test]
    fn cache_sweep_report_labels_beyond_l3_tier() {
        let mut p = measure_cache_point(512);
        p.working_set_bytes = 64 * 1024 * 1024; // force beyond-L3 for label test
        let report = cache_sweep_report(&[p]);
        assert!(report.contains("RAM"));
    }
}
