// transition_gradient.rs — Metatron Dynamics, Inc. V3.0
// Regime 4: fine-grained gradient sweep through declared transition zone.
// Bounded over D. No claim beyond D.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// V2.0 (Regime 3) established two distinct transition surfaces on this
// hardware using five coarse tiers:
//
//   BRANCHY  transition: LARGE → XLARGE   (524 KB → 2 MB,  B/L: 1.06 → 4.44)
//   SCRAMBLED transition: XLARGE → XXLARGE (2 MB → 16 MB,  S/L: 1.28 → 5.85)
//
// Each transition is bracketed but not located. We know each one occurred
// somewhere within its declared interval. We do not know:
//   - the working-set size at which S/L and B/L first depart from 1.0
//   - whether the departure is a sharp cliff or a gradual ramp
//   - whether the two transitions are separated or overlapping in W-space
//
// This module answers those questions by producing a gradient — a sequence
// of (W, S/L, B/L, dS/L/dW, dB/L/dW) at fine enough N increments that
// the rate of change itself becomes a declared observable.
//
// The mathematical object is not a benchmark number. It is:
//
//   f(W) = S/L(W)   and   g(W) = B/L(W)
//
// and their discrete derivatives:
//
//   Δf(W_i) = f(W_i) - f(W_{i-1})
//   Δg(W_i) = g(W_i) - g(W_{i-1})
//
// The point where Δf or Δg accelerates (second difference > 0 and growing)
// is the declared transition surface, located to within one sweep step.
//
// ── Declared Sweep ────────────────────────────────────────────────────────────
//
// 25 N values, approximately logarithmically spaced, from N=65,536 (524 KB,
// L2 saturation — the last V2.0 point before BRANCHY's transition) through
// N=4,194,304 (32 MB, approaching L3 boundary — beyond SCRAMBLED's transition).
//
// Spacing: not strictly doubling (which would give only ~6 points across
// this range). Instead, steps of approximately 1.35× to 1.5× per increment,
// giving ~25 points across a 64× range of N. Declared explicitly; not
// derived from a formula.
//
// ── Declared Algorithms ───────────────────────────────────────────────────────
//
// Three algorithms only — the ones that matter for the gradient:
//   LINEAR_SCAN   — control (O(N), sequential, no mechanism stress)
//   SCRAMBLED     — access-pattern stress (O(N), same op count)
//   BRANCHY       — branch-predictability stress (O(N), same op count)
//
// ALL_PAIRS excluded: intractable at these N.
// WINDOWED, PREFIX_SUM, SORT_SCAN excluded: not relevant to the gradient
// question. Their comparison data is already declared in Regime 3.
//
// ── Timing Protocol ───────────────────────────────────────────────────────────
//
// N ≤ 524,288 (WS ≤ 4 MB):  standard protocol (100 warm / 1000 timed).
// N > 524,288 (WS > 4 MB):  lighter protocol (10 warm / 100 timed).
//   Declared as OC-TG-2. Heavier than the V2.0 XLARGE/XXLARGE protocol
//   (3/10) but lighter than full standard — a declared intermediate.
//   At N=4,194,304 (32 MB), even 100 timed passes at ~20 ms each is
//   ~2 seconds per algorithm; the full sweep at this protocol takes
//   roughly 10–20 minutes total.
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-TG-1: Sweep N values are declared, not adaptively chosen. A finer
//   or differently spaced sweep could locate transitions more precisely.
//   The current spacing is adequate to determine whether transitions are
//   sharp cliffs or gradual ramps and to locate them within one step.
//
// OC-TG-2: Lighter timing protocol at N > 524,288 (10 warm / 100 timed).
//   Figures at those points carry elevated variance relative to the
//   standard protocol. The gradient (first difference) at those points
//   is less precise than at smaller N.
//
// OC-TG-3: The gradient Δf and Δg are discrete first differences over
//   unequal N intervals. They are normalized per unit log(N) to make
//   steps comparable. This normalization is declared; a different
//   normalization could be chosen.
//
// OC-HW-1 (from execution_record.md V2.0): hardware mechanism(s)
//   responsible for the excess cost are not established by wall-clock
//   timing alone. This module narrows the location of transitions but
//   does not identify the mechanism. Per-operation hardware counter
//   instrumentation remains the declared next instrument pass after this.

use std::time::Instant;
use crate::binary_baselines::{
    BinaryAlgo, declared_permutation, declared_branch_bits,
    SCRAMBLE_SEED, BRANCH_SEED,
};

// ── Declared sweep N values ───────────────────────────────────────────────────
// 25 points from N=65,536 (524 KB) through N=4,194,304 (32 MB).
// Approximately 1.35–1.50× per step. Declared explicitly.
pub const SWEEP_N: [usize; 25] = [
       65_536,   // 524 KB  — L2 saturation (V2.0 LARGE baseline)
       90_000,   // 720 KB
      122_880,   // 983 KB  — approaching L2/L3 boundary (1 MB L2/core)
      163_840,   // 1.3 MB  — into L3
      196_608,   // 1.6 MB
      229_376,   // 1.8 MB
      262_144,   // 2.0 MB  — V2.0 XLARGE (L3 entry, BRANCHY transition)
      327_680,   // 2.6 MB
      393_216,   // 3.1 MB
      458_752,   // 3.7 MB
      524_288,   // 4.2 MB  — last standard-protocol point
      655_360,   // 5.2 MB
      786_432,   // 6.3 MB
      917_504,   // 7.3 MB
    1_048_576,   // 8.4 MB
    1_245_184,   // 9.9 MB
    1_441_792,   // 11.5 MB
    1_638_400,   // 13.1 MB
    1_835_008,   // 14.7 MB
    2_097_152,   // 16.8 MB — V2.0 XXLARGE (L3-internal step, SCRAMBLED transition)
    2_359_296,   // 18.9 MB
    2_752_512,   // 22.0 MB
    3_145_728,   // 25.2 MB
    3_670_016,   // 29.4 MB
    4_194_304,   // 33.6 MB — approaching L3 boundary (32 MB L3)
];

/// Protocol boundary. N values above this use the lighter protocol.
pub const STANDARD_PROTOCOL_MAX_N: usize = 524_288;

/// Standard protocol (N ≤ STANDARD_PROTOCOL_MAX_N).
pub const TG_WARM_STANDARD: usize = 100;
pub const TG_TIMED_STANDARD: usize = 1_000;

/// Lighter protocol (N > STANDARD_PROTOCOL_MAX_N) — OC-TG-2.
pub const TG_WARM_LIGHT: usize = 10;
pub const TG_TIMED_LIGHT: usize = 100;

fn warm_passes(n: usize) -> usize {
    if n <= STANDARD_PROTOCOL_MAX_N { TG_WARM_STANDARD } else { TG_WARM_LIGHT }
}

fn timed_passes(n: usize) -> usize {
    if n <= STANDARD_PROTOCOL_MAX_N { TG_TIMED_STANDARD } else { TG_TIMED_LIGHT }
}

/// One gradient point: timing for all three algorithms at one N.
#[derive(Debug, Clone)]
pub struct GradientPoint {
    pub n: usize,
    pub working_set_bytes: usize,
    pub linear_mean_ns: f64,
    pub scrambled_mean_ns: f64,
    pub branchy_mean_ns: f64,
    /// S/L ratio at this N.
    pub sl_ratio: f64,
    /// B/L ratio at this N.
    pub bl_ratio: f64,
    /// Whether lighter protocol was used (OC-TG-2).
    pub light_protocol: bool,
}

impl GradientPoint {
    pub fn ns_per_op_linear(&self)   -> f64 { self.linear_mean_ns   / (self.n - 1) as f64 }
    pub fn ns_per_op_scrambled(&self) -> f64 { self.scrambled_mean_ns / (self.n - 1) as f64 }
    pub fn ns_per_op_branchy(&self)  -> f64 { self.branchy_mean_ns  / (self.n - 1) as f64 }
}

/// Gradient between two successive points — the first difference of S/L and B/L,
/// normalized by Δlog(N) so that unequal step sizes are comparable (OC-TG-3).
#[derive(Debug, Clone)]
pub struct GradientDelta {
    pub n_from: usize,
    pub n_to: usize,
    /// Δlog(N) = log(N_to) - log(N_from). Normalization factor.
    pub delta_log_n: f64,
    /// (S/L(N_to) - S/L(N_from)) / Δlog(N)
    pub d_sl: f64,
    /// (B/L(N_to) - B/L(N_from)) / Δlog(N)
    pub d_bl: f64,
}

fn time_algo(algo: BinaryAlgo, values: &[f64], perm: &[usize], bits: &[bool], n: usize) -> f64 {
    let w = warm_passes(n);
    let t = timed_passes(n);
    for _ in 0..w {
        std::hint::black_box(algo.run(values, perm, bits));
    }
    let mut times: Vec<u128> = Vec::with_capacity(t);
    for _ in 0..t {
        let start = Instant::now();
        std::hint::black_box(algo.run(values, perm, bits));
        times.push(start.elapsed().as_nanos());
    }
    times.iter().sum::<u128>() as f64 / t as f64
}

/// Measures one gradient point at declared N.
pub fn measure_gradient_point(n: usize) -> GradientPoint {
    let values: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
    let perm = declared_permutation(n, SCRAMBLE_SEED);
    let bits = declared_branch_bits(n, BRANCH_SEED);

    let linear_mean_ns   = time_algo(BinaryAlgo::LinearScanDiff,       &values, &perm, &bits, n);
    let scrambled_mean_ns = time_algo(BinaryAlgo::ScrambledAccess,      &values, &perm, &bits, n);
    let branchy_mean_ns  = time_algo(BinaryAlgo::BranchyDataDependent,  &values, &perm, &bits, n);

    let sl_ratio = scrambled_mean_ns / linear_mean_ns;
    let bl_ratio = branchy_mean_ns   / linear_mean_ns;

    GradientPoint {
        n,
        working_set_bytes: n * 8,
        linear_mean_ns,
        scrambled_mean_ns,
        branchy_mean_ns,
        sl_ratio,
        bl_ratio,
        light_protocol: n > STANDARD_PROTOCOL_MAX_N,
    }
}

/// Runs the full declared sweep.
pub fn run_transition_gradient() -> Vec<GradientPoint> {
    SWEEP_N.iter().map(|&n| measure_gradient_point(n)).collect()
}

/// Computes first differences of S/L and B/L, normalized by Δlog(N).
pub fn compute_gradient_deltas(points: &[GradientPoint]) -> Vec<GradientDelta> {
    points.windows(2).map(|w| {
        let delta_log_n = (w[1].n as f64).ln() - (w[0].n as f64).ln();
        GradientDelta {
            n_from: w[0].n,
            n_to:   w[1].n,
            delta_log_n,
            d_sl: (w[1].sl_ratio - w[0].sl_ratio) / delta_log_n,
            d_bl: (w[1].bl_ratio - w[0].bl_ratio) / delta_log_n,
        }
    }).collect()
}

pub fn gradient_report(points: &[GradientPoint], deltas: &[GradientDelta]) -> String {
    let mut r = String::new();

    r.push_str("═══════════════════════════════════════════════════════════════════════════════════\n");
    r.push_str("REGIME 4 — TRANSITION GRADIENT SWEEP\n");
    r.push_str("Fine-grained S/L and B/L ratios across declared transition zone (524 KB – 32 MB)\n");
    r.push_str("Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4) — Metatron Dynamics, Inc.\n");
    r.push_str("Bounded over D. No claim beyond D.\n");
    r.push_str("═══════════════════════════════════════════════════════════════════════════════════\n");
    r.push_str("Mathematical object: f(W) = S/L(W),  g(W) = B/L(W)\n");
    r.push_str("Gradient: Δf, Δg = first difference normalized by Δlog(N) — see OC-TG-3.\n");
    r.push_str("Transition surface located where Δf or Δg accelerates.\n");
    r.push_str("* = lighter protocol (10 warm / 100 timed) — OC-TG-2.\n");
    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");
    r.push_str(&format!(
        "{:>12}  {:>10}  {:>10}  {:>10}  {:>10}  {:>8}  {:>8}  {}\n",
        "N", "WS (KB)", "LIN ns/op", "SCR ns/op", "BRN ns/op", "S/L", "B/L", "PROTO"
    ));
    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");

    for p in points {
        let marker = if p.light_protocol { "*" } else { " " };
        r.push_str(&format!(
            "{:>12}  {:>10.1}  {:>10.4}  {:>10.4}  {:>10.4}  {:>8.4}  {:>8.4}  {}\n",
            p.n,
            p.working_set_bytes as f64 / 1024.0,
            p.ns_per_op_linear(),
            p.ns_per_op_scrambled(),
            p.ns_per_op_branchy(),
            p.sl_ratio,
            p.bl_ratio,
            marker,
        ));
    }

    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");
    r.push_str("GRADIENT — first differences of S/L and B/L, normalized by Δlog(N)\n");
    r.push_str("A spike in Δf or Δg locates the transition surface.\n");
    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");
    r.push_str(&format!(
        "  {:>12} → {:>12}  {:>8}  {:>8}\n",
        "N_from", "N_to", "Δ(S/L)", "Δ(B/L)"
    ));
    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");

    for d in deltas {
        r.push_str(&format!(
            "  {:>12} → {:>12}  {:>8.4}  {:>8.4}\n",
            d.n_from, d.n_to, d.d_sl, d.d_bl
        ));
    }

    r.push_str("───────────────────────────────────────────────────────────────────────────────────\n");
    r.push_str("Interpretation:\n");
    r.push_str("  Δ(S/L) and Δ(B/L) near 0.0: ratio stable — mechanism not yet engaged.\n");
    r.push_str("  Δ(S/L) or Δ(B/L) accelerating: transition surface entering this interval.\n");
    r.push_str("  Peak Δ value locates the steepest part of the transition.\n");
    r.push_str("  Separate peaks for S/L and B/L confirm two distinct transition surfaces.\n");
    r.push_str("  Overlapping peaks would suggest a shared underlying mechanism.\n");
    r.push_str("  OC-HW-1: mechanism identification requires hardware counter instrumentation.\n");
    r.push_str("═══════════════════════════════════════════════════════════════════════════════════\n");

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_n_has_declared_count() {
        assert_eq!(SWEEP_N.len(), 25);
    }

    #[test]
    fn sweep_n_starts_at_large_tier() {
        assert_eq!(SWEEP_N[0], 65_536, "sweep must begin at V2.0 LARGE tier");
    }

    #[test]
    fn sweep_n_ends_at_declared_upper_bound() {
        assert_eq!(*SWEEP_N.last().unwrap(), 4_194_304);
    }

    #[test]
    fn sweep_n_is_strictly_increasing() {
        for w in SWEEP_N.windows(2) {
            assert!(w[1] > w[0], "N values must be strictly increasing");
        }
    }

    #[test]
    fn sweep_n_contains_v2_anchor_points() {
        // V2.0 LARGE, XLARGE, XXLARGE must appear in the sweep for continuity.
        assert!(SWEEP_N.contains(&65_536),    "must contain V2.0 LARGE");
        assert!(SWEEP_N.contains(&262_144),   "must contain V2.0 XLARGE");
        assert!(SWEEP_N.contains(&2_097_152), "must contain V2.0 XXLARGE");
    }

    #[test]
    fn protocol_boundary_applied_correctly() {
        assert_eq!(warm_passes(524_288), TG_WARM_STANDARD);
        assert_eq!(warm_passes(524_289), TG_WARM_LIGHT);
        assert_eq!(timed_passes(524_288), TG_TIMED_STANDARD);
        assert_eq!(timed_passes(524_289), TG_TIMED_LIGHT);
    }

    #[test]
    fn gradient_point_small_produces_finite_positive() {
        let p = measure_gradient_point(65_536);
        assert!(p.linear_mean_ns > 0.0);
        assert!(p.scrambled_mean_ns > 0.0);
        assert!(p.branchy_mean_ns > 0.0);
        assert!(p.sl_ratio.is_finite() && p.sl_ratio > 0.0);
        assert!(p.bl_ratio.is_finite() && p.bl_ratio > 0.0);
        assert!(!p.light_protocol);
    }

    #[test]
    fn gradient_point_above_boundary_uses_light_protocol() {
        let p = measure_gradient_point(655_360);
        assert!(p.light_protocol);
    }

    #[test]
    fn gradient_deltas_length_is_points_minus_one() {
        // Can't run full sweep in test — use two synthetic points.
        let p1 = GradientPoint {
            n: 65_536, working_set_bytes: 65_536 * 8,
            linear_mean_ns: 100.0, scrambled_mean_ns: 110.0, branchy_mean_ns: 105.0,
            sl_ratio: 1.10, bl_ratio: 1.05, light_protocol: false,
        };
        let p2 = GradientPoint {
            n: 131_072, working_set_bytes: 131_072 * 8,
            linear_mean_ns: 200.0, scrambled_mean_ns: 240.0, branchy_mean_ns: 220.0,
            sl_ratio: 1.20, bl_ratio: 1.10, light_protocol: false,
        };
        let deltas = compute_gradient_deltas(&[p1, p2]);
        assert_eq!(deltas.len(), 1);
    }

    #[test]
    fn gradient_delta_normalized_by_log_n() {
        let p1 = GradientPoint {
            n: 65_536, working_set_bytes: 65_536 * 8,
            linear_mean_ns: 100.0, scrambled_mean_ns: 110.0, branchy_mean_ns: 105.0,
            sl_ratio: 1.10, bl_ratio: 1.05, light_protocol: false,
        };
        let p2 = GradientPoint {
            n: 131_072, working_set_bytes: 131_072 * 8,
            linear_mean_ns: 200.0, scrambled_mean_ns: 240.0, branchy_mean_ns: 220.0,
            sl_ratio: 1.20, bl_ratio: 1.10, light_protocol: false,
        };
        let deltas = compute_gradient_deltas(&[p1, p2]);
        let expected_delta_log_n = (131_072_f64).ln() - (65_536_f64).ln();
        assert!((deltas[0].delta_log_n - expected_delta_log_n).abs() < 1e-10);
        let expected_d_sl = (1.20 - 1.10) / expected_delta_log_n;
        assert!((deltas[0].d_sl - expected_d_sl).abs() < 1e-10);
    }

    #[test]
    fn gradient_report_does_not_panic() {
        let p = measure_gradient_point(65_536);
        let deltas = compute_gradient_deltas(&[p.clone(), p]);
        let report = gradient_report(&[], &deltas);
        assert!(report.contains("REGIME 4"));
    }
}
