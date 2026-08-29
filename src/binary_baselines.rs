// binary_baselines.rs — Metatron Dynamics, Inc. V1.1
// Declared binary/conventional algorithm variants for Regime 3.
// Bounded over D. No claim beyond D.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// V1.0's Regime 3 compared the relational ABR chain against exactly one
// binary baseline: all-pairs difference, O(N^2). That comparison could not
// distinguish two different possible explanations for "relational wins at
// every tier": (a) relational structure genuinely has lower fixed and
// scaling cost on this hardware, or (b) any O(N) algorithm beats any
// O(N^2) algorithm once N is more than trivial, regardless of whether the
// O(N) side is "relational" in any meaningful sense. OC-CC-2 (V1.0)
// flagged this gap explicitly.
//
// V1.1 declares five binary baselines spanning multiple complexity
// classes, so complexity_crossover.rs can test the relational chain
// against each one and see whether relational still wins uniformly, or
// only against quadratic-class tasks specifically.
//
// ── Declared Baselines ────────────────────────────────────────────────────────
//
// AllPairsDiff   O(N^2)     — every i,j pair, x[i]-x[j]. V1.0's original baseline.
// LinearScanDiff O(N)       — single pass, adjacent difference x[i]-x[i-1].
// WindowedDiff   O(N*K)     — bounded sliding window, K=8 declared. A more
//                             realistic "conventional" pattern than
//                             all-pairs — many real binary algorithms
//                             (convolution, local smoothing) are windowed
//                             rather than globally quadratic.
// PrefixSum      O(N)       — running cumulative sum. Different operation
//                             shape than a pairwise difference (declared
//                             so not every O(N) baseline is a disguised
//                             copy of the same computation).
// SortThenScan   O(N log N) — comparison sort, then one adjacent-difference
//                             pass. Fills the complexity gap between O(N)
//                             and O(N^2).
//
// All five operate on the same declared input values (see
// complexity_crossover.rs) and are timed under the identical warm/timed
// protocol as the relational chain, so results are directly comparable.
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-BB-1: WINDOW_K=8 is a declared choice, not derived from any prior
//   measurement. Different K values could change WindowedDiff's relative
//   position without changing its complexity class.
// OC-BB-2: These five baselines are declared representative binary
//   algorithms, not an exhaustive set. A different chosen algorithm within
//   the same complexity class could still shift results.

use std::hint::black_box;

/// Declared window size for WindowedDiff. OC-BB-1: declared, not derived.
pub const WINDOW_K: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryAlgo {
    AllPairsDiff,
    LinearScanDiff,
    WindowedDiff,
    PrefixSum,
    SortThenScan,
    ScrambledAccess,
    BranchyDataDependent,
}

pub const ALL_ALGOS: [BinaryAlgo; 7] = [
    BinaryAlgo::AllPairsDiff,
    BinaryAlgo::LinearScanDiff,
    BinaryAlgo::WindowedDiff,
    BinaryAlgo::PrefixSum,
    BinaryAlgo::SortThenScan,
    BinaryAlgo::ScrambledAccess,
    BinaryAlgo::BranchyDataDependent,
];

impl BinaryAlgo {
    pub fn label(self) -> &'static str {
        match self {
            BinaryAlgo::AllPairsDiff => "ALL_PAIRS",
            BinaryAlgo::LinearScanDiff => "LINEAR_SCAN",
            BinaryAlgo::WindowedDiff => "WINDOWED",
            BinaryAlgo::PrefixSum => "PREFIX_SUM",
            BinaryAlgo::SortThenScan => "SORT_SCAN",
            BinaryAlgo::ScrambledAccess => "SCRAMBLED",
            BinaryAlgo::BranchyDataDependent => "BRANCHY",
        }
    }

    pub fn complexity_class(self) -> &'static str {
        match self {
            BinaryAlgo::AllPairsDiff => "O(N^2)",
            BinaryAlgo::LinearScanDiff => "O(N)",
            BinaryAlgo::WindowedDiff => "O(N*K)",
            BinaryAlgo::PrefixSum => "O(N)",
            BinaryAlgo::SortThenScan => "O(N log N)",
            BinaryAlgo::ScrambledAccess => "O(N) cache-unfriendly",
            BinaryAlgo::BranchyDataDependent => "O(N) branch-heavy",
        }
    }

    /// Declared elementary-operation count for this algorithm at size N.
    /// Used for reporting only — timing is measured independently.
    pub fn declared_op_count(self, n: usize) -> u64 {
        let n64 = n as u64;
        match self {
            BinaryAlgo::AllPairsDiff => n64.saturating_mul(n64.saturating_sub(1)),
            BinaryAlgo::LinearScanDiff => n64.saturating_sub(1),
            BinaryAlgo::WindowedDiff => n64.saturating_sub(1).saturating_mul(WINDOW_K as u64),
            BinaryAlgo::PrefixSum => n64,
            BinaryAlgo::SortThenScan => {
                let log2n: u64 = if n64 <= 1 {
                    0
                } else {
                    64 - (n64 - 1).leading_zeros() as u64
                };
                n64.saturating_mul(log2n).saturating_add(n64.saturating_sub(1))
            }
            // Same op count as LinearScanDiff — only access pattern / branch
            // predictability differs, complexity class held constant.
            BinaryAlgo::ScrambledAccess => n64.saturating_sub(1),
            BinaryAlgo::BranchyDataDependent => n64.saturating_sub(1),
        }
    }

    /// Runs the declared computation once over `values`. Real measured
    /// computation (not simulated/estimated). `perm` and `bits` are
    /// precomputed auxiliary data (see declared_permutation,
    /// declared_branch_bits) used only by ScrambledAccess and
    /// BranchyDataDependent respectively; other variants ignore them.
    pub fn run(self, values: &[f64], perm: &[usize], bits: &[bool]) -> f64 {
        match self {
            BinaryAlgo::AllPairsDiff => all_pairs_diff(values),
            BinaryAlgo::LinearScanDiff => linear_scan_diff(values),
            BinaryAlgo::WindowedDiff => windowed_diff(values, WINDOW_K),
            BinaryAlgo::PrefixSum => prefix_sum(values),
            BinaryAlgo::SortThenScan => sort_then_scan(values),
            BinaryAlgo::ScrambledAccess => scrambled_access_diff(values, perm),
            BinaryAlgo::BranchyDataDependent => branchy_data_dependent_diff(values, bits),
        }
    }
}

fn all_pairs_diff(values: &[f64]) -> f64 {
    let n = values.len();
    let mut acc = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            if i != j {
                acc += values[i] - values[j];
            }
        }
    }
    black_box(acc)
}

fn linear_scan_diff(values: &[f64]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        acc += values[i] - values[i - 1];
    }
    black_box(acc)
}

fn windowed_diff(values: &[f64], k: usize) -> f64 {
    let n = values.len();
    let mut acc = 0.0f64;
    for i in 0..n {
        for offset in 1..=k {
            if i + offset < n {
                acc += values[i] - values[i + offset];
            }
        }
    }
    black_box(acc)
}

fn prefix_sum(values: &[f64]) -> f64 {
    let mut acc = 0.0f64;
    let mut running = 0.0f64;
    for &v in values {
        running += v;
        acc += running;
    }
    black_box(acc)
}

fn sort_then_scan(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut acc = 0.0f64;
    for i in 1..sorted.len() {
        acc += sorted[i] - sorted[i - 1];
    }
    black_box(acc)
}

// ── V1.2 Addition ─────────────────────────────────────────────────────────────
//
// The five baselines above vary complexity class (op count) but are all
// cache-friendly, branch-free, sequential-access computations. That leaves
// open exactly what Robin flagged: which MECHANISM makes real data
// "problematic" for binary processing, and does the relational chain (which
// is itself cache-friendly and branch-free by construction — see Regime 1's
// warm-pass protocol) hold any advantage specifically against binary tasks
// that hit those mechanisms, even at the SAME O(N) complexity class where
// relational already lost to LinearScanDiff and PrefixSum.
//
// Two new O(N) baselines, same op count as LinearScanDiff (N-1), isolating
// one mechanism each:
//
// ScrambledAccess — same one-subtraction-per-step computation as
//   LinearScanDiff, but walked via a fixed pseudo-random permutation of
//   indices instead of sequential order. Defeats hardware prefetching.
//   Same complexity class as LinearScanDiff; only the memory ACCESS PATTERN
//   differs.
//
// BranchyDataDependent — same one-subtraction-per-step computation, but each
//   step passes through a data-dependent conditional whose outcome is
//   effectively unpredictable (fixed pseudo-random 50/50 sequence). Defeats
//   branch prediction. Same complexity class as LinearScanDiff; only branch
//   PREDICTABILITY differs.
//
// Both use a fixed-seed deterministic PRNG (xorshift64) so results are
// reproducible — not true entropy, but a declared, repeatable stand-in for
// "unpredictable to this hardware's predictor," which is the property that
// actually matters here.
//
// METHODOLOGY NOTE (OC-BB-3): the permutation and branch-bit sequences are
// generated ONCE per tier, outside the warm/timed measurement loop, and
// passed in — the same principle as Regime 1's pre-allocated-buffer
// requirement (M_declaration.md, Implementation Admissibility Condition).
// Generating them inside the timed pass would conflate O(N) setup cost with
// the access-pattern or branch-pattern effect being measured.
//
// ── Open Conditions (V1.2 addition) ──────────────────────────────────────────
//
// OC-BB-3: Permutation/branch-bit generation is excluded from timing by
//   precomputing once per tier before the warm/timed loop (see
//   complexity_crossover.rs). If this precomputation were included, it
//   would add real O(N) cost not attributable to the access/branch
//   mechanism itself.
// OC-BB-4: SCRAMBLE_SEED and BRANCH_SEED are declared fixed constants, not
//   derived or randomized per run. A different seed could shift results by
//   chance, though the qualitative cache/branch-defeat property should hold
//   for any sufficiently mixed seed.

/// Declared fixed seed for ScrambledAccess's index permutation. OC-BB-4.
pub const SCRAMBLE_SEED: u64 = 0x9E3779B97F4A7C15;
/// Declared fixed seed for BranchyDataDependent's branch-bit sequence. OC-BB-4.
pub const BRANCH_SEED: u64 = 0xD1B54A32D192ED03;

/// xorshift64 PRNG. Deterministic given a seed — chosen for reproducibility,
/// not cryptographic quality (not needed here; only mixing is needed).
fn xorshift64_next(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// Builds a fixed pseudo-random permutation of 0..n via Fisher-Yates,
/// seeded deterministically. Declared to be generated ONCE per tier,
/// outside the timed loop — see OC-BB-3.
pub fn declared_permutation(n: usize, seed: u64) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    let mut s = if seed == 0 { 0x1 } else { seed }; // xorshift requires nonzero state
    for i in (1..n).rev() {
        let r = (xorshift64_next(&mut s) % (i as u64 + 1)) as usize;
        perm.swap(i, r);
    }
    perm
}

/// Builds a fixed pseudo-random boolean sequence of length n, seeded
/// deterministically. Declared to be generated ONCE per tier, outside the
/// timed loop — see OC-BB-3.
pub fn declared_branch_bits(n: usize, seed: u64) -> Vec<bool> {
    let mut s = if seed == 0 { 0x1 } else { seed };
    (0..n).map(|_| xorshift64_next(&mut s) & 1 == 1).collect()
}

fn scrambled_access_diff(values: &[f64], perm: &[usize]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        acc += values[perm[i]] - values[perm[i - 1]];
    }
    black_box(acc)
}

fn branchy_data_dependent_diff(values: &[f64], bits: &[bool]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        if bits[i] {
            acc += values[i] - values[i - 1];
        } else {
            acc -= values[i] - values[i - 1];
        }
    }
    black_box(acc)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample(n: usize) -> Vec<f64> {
        (0..n).map(|i| (i as f64) / (n as f64)).collect()
    }

    fn aux(n: usize) -> (Vec<usize>, Vec<bool>) {
        (
            declared_permutation(n, SCRAMBLE_SEED),
            declared_branch_bits(n, BRANCH_SEED),
        )
    }

    #[test]
    fn all_algos_deterministic() {
        let values = sample(32);
        let (perm, bits) = aux(32);
        for algo in ALL_ALGOS {
            let a = algo.run(&values, &perm, &bits);
            let b = algo.run(&values, &perm, &bits);
            assert!((a - b).abs() < 1e-9, "{:?} must be deterministic", algo);
        }
    }

    #[test]
    fn all_algos_finite() {
        let values = sample(32);
        let (perm, bits) = aux(32);
        for algo in ALL_ALGOS {
            assert!(algo.run(&values, &perm, &bits).is_finite(),
                "{:?} must produce finite output", algo);
        }
    }

    #[test]
    fn op_counts_ordered_by_declared_complexity_at_large_n() {
        // At large N, O(N) has the fewest declared ops and O(N^2) the most.
        // WindowedDiff (K=8) and SortThenScan (log2(8192)=13) sit between
        // them; their relative order depends on K vs log2(N) and is not
        // asserted here — only the two clear endpoints are.
        let n = 8_192;
        let linear = BinaryAlgo::LinearScanDiff.declared_op_count(n);
        let sort_scan = BinaryAlgo::SortThenScan.declared_op_count(n);
        let windowed = BinaryAlgo::WindowedDiff.declared_op_count(n);
        let all_pairs = BinaryAlgo::AllPairsDiff.declared_op_count(n);
        assert!(linear < windowed);
        assert!(linear < sort_scan);
        assert!(windowed < all_pairs);
        assert!(sort_scan < all_pairs);
    }

    #[test]
    fn scrambled_and_branchy_op_counts_match_linear_scan() {
        // Same complexity class as LinearScanDiff — mechanism isolation
        // requires holding op count constant. See V1.2 addition comment.
        let n = 8_192;
        let linear = BinaryAlgo::LinearScanDiff.declared_op_count(n);
        assert_eq!(BinaryAlgo::ScrambledAccess.declared_op_count(n), linear);
        assert_eq!(BinaryAlgo::BranchyDataDependent.declared_op_count(n), linear);
    }

    #[test]
    fn windowed_op_count_matches_formula() {
        assert_eq!(
            BinaryAlgo::WindowedDiff.declared_op_count(100),
            99 * WINDOW_K as u64
        );
    }

    #[test]
    fn all_pairs_op_count_matches_formula() {
        assert_eq!(BinaryAlgo::AllPairsDiff.declared_op_count(64), 64 * 63);
    }

    #[test]
    fn label_and_complexity_class_nonempty_for_all_algos() {
        for algo in ALL_ALGOS {
            assert!(!algo.label().is_empty());
            assert!(!algo.complexity_class().is_empty());
        }
    }

    #[test]
    fn declared_permutation_is_a_valid_permutation() {
        let n = 256;
        let perm = declared_permutation(n, SCRAMBLE_SEED);
        assert_eq!(perm.len(), n);
        let mut sorted = perm.clone();
        sorted.sort();
        let expected: Vec<usize> = (0..n).collect();
        assert_eq!(sorted, expected, "permutation must be a bijection on 0..n");
    }

    #[test]
    fn declared_permutation_deterministic_for_same_seed() {
        let a = declared_permutation(128, SCRAMBLE_SEED);
        let b = declared_permutation(128, SCRAMBLE_SEED);
        assert_eq!(a, b);
    }

    #[test]
    fn declared_permutation_not_identity() {
        // Sanity check the shuffle actually shuffles (extremely unlikely
        // to be identity by chance for n=256 with this seed).
        let n = 256;
        let perm = declared_permutation(n, SCRAMBLE_SEED);
        let identity: Vec<usize> = (0..n).collect();
        assert_ne!(perm, identity, "permutation should not equal identity order");
    }

    #[test]
    fn declared_branch_bits_deterministic_and_correct_length() {
        let a = declared_branch_bits(256, BRANCH_SEED);
        let b = declared_branch_bits(256, BRANCH_SEED);
        assert_eq!(a.len(), 256);
        assert_eq!(a, b);
    }

    #[test]
    fn declared_branch_bits_roughly_balanced() {
        // Not a statistical proof, just a sanity check that the sequence
        // isn't degenerate (all-true or all-false) for this seed/length.
        let bits = declared_branch_bits(1_000, BRANCH_SEED);
        let true_count = bits.iter().filter(|&&b| b).count();
        assert!(true_count > 300 && true_count < 700,
            "branch bit sequence should be roughly balanced, got {} true of 1000", true_count);
    }
}
