// probe.rs — Metatron Dynamics, Inc. V5.0
// Isolated single-algorithm, single-N measurement binary for uProf instrumentation.
// Bounded over D. No claim beyond D.
//
// ── What Changed in V5.0 ─────────────────────────────────────────────────────
//
// Added D_CHAINED variant (algo: "chained") for OC-HW-5 intervention.
// D promoted to first-class declared variable in R = {N, W, A, B, D, O_count}.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// Runs EXACTLY ONE declared (algorithm, N) pair under uProf so that hardware
// counter events are attributed to one algorithm only. Hardware state H is
// declared as belonging to that pair.
//
// ── Usage ─────────────────────────────────────────────────────────────────────
//
//   probe.exe <ALGO> <N>
//
//   ALGO: linear | scrambled | branchy | chained
//   N:    any positive integer (working set = N * 8 bytes)
//
// V4.0 declared measurement points (M_declaration.md V4.0):
//   probe.exe linear    65536    B-pre  512 KB  control
//   probe.exe branchy   65536    B-pre  512 KB  pre-transition
//   probe.exe branchy   122880   B-on   960 KB  BRANCHY onset
//   probe.exe branchy   524288   B-post 4.1 MB  BRANCHY plateau
//   probe.exe linear    524288   S-pre  4.1 MB  control
//   probe.exe scrambled 524288   S-pre  4.1 MB  pre-transition
//   probe.exe scrambled 2097152  S-on   16 MB   SCRAMBLED onset
//   probe.exe scrambled 4194304  S-post 32 MB   SCRAMBLED post
//
// OC-HW-5 intervention points (M_declaration.md V4.0, V5.0):
//   probe.exe scrambled 524288   D_independent  A=scrambled, W=4.1 MB
//   probe.exe chained   524288   D_chained      A=scrambled, W=4.1 MB
//
//   Comparison: same N, same W, same scrambled access distribution.
//   Only D differs: independent loads vs data-dependent address chain.
//
// ── D_CHAINED declaration (OC-HW-5) ──────────────────────────────────────────
//
// D_independent (scrambled): address sequence a_t = perm[t] for each step t.
//   Each address is computable without waiting for any prior result.
//   Multiple loads can be outstanding concurrently.
//
// D_chained: address sequence a_{t+1} = f(x_{a_t}), where x_{a_t} is the
//   value stored at the prior address. The next address does not exist as
//   an actionable quantity until the preceding load completes and its value
//   is returned. The declared dependency chain prevents the hardware from
//   resolving a_{t+1} before x_{a_t} is available.
//
// Implementation: chain[] is a Vec<usize> where chain[i] = perm[i] — the
//   same scrambled permutation as D_independent, but accessed as a pointer
//   chain. Each step reads chain[current_idx] to get the next index.
//   The read result IS the next address — enforcing a_{t+1} = f(x_{a_t}).
//   values[] is accessed at each chain index for the arithmetic accumulation,
//   keeping O_count and working set identical to D_independent.
//
// Same declared properties as scrambled:
//   N, W, A (same permutation distribution), O_count — all equal.
//   Only D differs.
//
// ── Timing Protocol ───────────────────────────────────────────────────────────
//
// N <= 524,288:  standard (100 warm / 1000 timed).
// N >  524,288:  lighter  (10 warm  / 100  timed) — OC-TG-2.
//
// Under uProf timing is elevated — OC-HW-2. Declared output is H, not timing.
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-HW-2: uProf instrumentation overhead. Timing not comparable to benchmark.
// OC-HW-3: CLOSED. Zen 4 event names confirmed in V4.0 run.
// OC-HW-4: Latency-hiding mechanism in D_independent State 1 — not yet
//   established by counter set. OC-HW-5 intervention addresses this.
// OC-HW-5: D_chained vs D_independent at N=524,288, W=4.1 MB. Prediction:
//   D_chained CPI substantially higher than D_independent at same N/W.
//   DRAM_PTI approximately equal (same working set). If confirmed, latency
//   hiding through concurrent outstanding requests is the State 1 mechanism.

use std::env;
use std::hint::black_box;
use std::time::Instant;
use abr_home_system_benchmark::binary_baselines::{
    declared_permutation, declared_branch_bits, SCRAMBLE_SEED, BRANCH_SEED,
};

const STANDARD_MAX_N: usize = 524_288;
const WARM_STANDARD:  usize = 100;
const TIMED_STANDARD: usize = 1_000;
const WARM_LIGHT:     usize = 10;
const TIMED_LIGHT:    usize = 100;

fn usage() {
    eprintln!("probe — isolated single-algorithm uProf measurement target");
    eprintln!("Metatron Dynamics, Inc. V5.0. Bounded over D.");
    eprintln!();
    eprintln!("Usage: probe <ALGO> <N>");
    eprintln!("  ALGO: linear | scrambled | branchy | chained");
    eprintln!("  N:    positive integer (working set = N * 8 bytes)");
    eprintln!();
    eprintln!("OC-HW-5 intervention (same N/W, D varies):");
    eprintln!("  probe scrambled 524288   D_independent  A=scrambled, W=4.1 MB");
    eprintln!("  probe chained   524288   D_chained      A=scrambled, W=4.1 MB");
    std::process::exit(1);
}

#[inline(never)]
fn run_linear(values: &[f64]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        acc += values[i] - values[i - 1];
    }
    black_box(acc)
}

#[inline(never)]
fn run_scrambled(values: &[f64], perm: &[usize]) -> f64 {
    let mut acc = 0.0f64;
    for i in 1..values.len() {
        acc += values[perm[i]] - values[perm[i - 1]];
    }
    black_box(acc)
}

#[inline(never)]
fn run_branchy(values: &[f64], bits: &[bool]) -> f64 {
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

/// D_chained: a_{t+1} = f(x_{a_t}).
///
/// chain[] encodes the same scrambled permutation as D_independent but as a
/// pointer chain: chain[i] = perm[i]. Each step reads chain[current] to get
/// the next index. The read result IS the next address — the dependency is
/// real and not eliminable by the compiler (black_box on current each step
/// prevents hoisting). values[] is accessed at each chain index for the
/// arithmetic, keeping O_count and WS equal to D_independent.
///
/// #[inline(never)] ensures uProf attributes samples to this function
/// specifically, not to an inlined caller.
#[inline(never)]
fn run_chained(values: &[f64], chain: &[usize]) -> f64 {
    let n = chain.len();
    let mut acc = 0.0f64;
    let mut current = chain[0];
    for _ in 1..n {
        let next = chain[current];          // a_{t+1} = f(x_{a_t})
        acc += values[current] - values[next];
        current = black_box(next);          // dependency: next step cannot
                                            // begin until this value is known
    }
    black_box(acc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }

    let algo = args[1].to_lowercase();
    let n: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("ERROR: N must be a positive integer, got '{}'", args[2]);
        std::process::exit(1);
    });

    if n < 2 {
        eprintln!("ERROR: N must be >= 2");
        std::process::exit(1);
    }

    if !["linear","scrambled","branchy","chained"].contains(&algo.as_str()) {
        eprintln!("ERROR: ALGO must be linear, scrambled, branchy, or chained — got '{}'", algo);
        usage();
    }

    let warm     = if n <= STANDARD_MAX_N { WARM_STANDARD  } else { WARM_LIGHT  };
    let timed    = if n <= STANDARD_MAX_N { TIMED_STANDARD } else { TIMED_LIGHT };
    let protocol = if n <= STANDARD_MAX_N { "standard (100w/1000t)" } else { "light (10w/100t) — OC-TG-2" };

    let ws_bytes = n * 8;
    let ws_mb    = ws_bytes as f64 / (1024.0 * 1024.0);

    let d_label = match algo.as_str() {
        "chained"  => "D_chained   (a_{t+1} = f(x_{a_t}) — serialized dependency)",
        "scrambled"=> "D_independent (a_{t+1} = P(t+1)   — concurrent loads permitted)",
        _          => "D_independent (sequential / branch — no memory dependency chain)",
    };

    println!("probe V5.0 — Metatron Dynamics, Inc. Bounded over D.");
    println!("Algorithm: {}  N: {}  WS: {:.2} MB ({} bytes)", algo, n, ws_mb, ws_bytes);
    println!("D: {}", d_label);
    println!("Protocol: {}  Warm: {}  Timed: {}", protocol, warm, timed);
    println!("OC-HW-2: timing under uProf is not comparable to benchmark.exe");
    println!();

    // Allocate outside timed loop.
    let values: Vec<f64> = (0..n).map(|i| (i as f64) / (n as f64)).collect();
    let perm  = declared_permutation(n, SCRAMBLE_SEED);
    let bits  = declared_branch_bits(n, BRANCH_SEED);
    // chain[] encodes the same permutation as a pointer chain for D_chained.
    // chain[i] = perm[i]: reading chain[current] yields the next index.
    let chain: Vec<usize> = perm.clone();

    // Warm phase.
    println!("Warming ({} passes)...", warm);
    for _ in 0..warm {
        match algo.as_str() {
            "linear"    => { run_linear(&values); }
            "scrambled" => { run_scrambled(&values, &perm); }
            "branchy"   => { run_branchy(&values, &bits); }
            "chained"   => { run_chained(&values, &chain); }
            _           => unreachable!(),
        }
    }

    // Timed phase — hot region for uProf sampling.
    println!("Timing ({} passes) — this is the uProf sampling window...", timed);
    let mut times: Vec<u128> = Vec::with_capacity(timed);
    for _ in 0..timed {
        let start = Instant::now();
        match algo.as_str() {
            "linear"    => { run_linear(&values); }
            "scrambled" => { run_scrambled(&values, &perm); }
            "branchy"   => { run_branchy(&values, &bits); }
            "chained"   => { run_chained(&values, &chain); }
            _           => unreachable!(),
        }
        times.push(start.elapsed().as_nanos());
    }

    let mean_ns   = times.iter().sum::<u128>() as f64 / timed as f64;
    let min_ns    = *times.iter().min().unwrap();
    let max_ns    = *times.iter().max().unwrap();
    let ns_per_op = mean_ns / (n - 1) as f64;

    println!();
    println!("Timing complete (informational only under uProf — OC-HW-2):");
    println!("  Mean: {:.1} ns  Min: {} ns  Max: {} ns", mean_ns, min_ns, max_ns);
    println!("  ns/op: {:.4}", ns_per_op);
    println!();
    if algo == "chained" || algo == "scrambled" {
        println!("OC-HW-5: compare chained vs scrambled at same N/W under uProf.");
        println!("  Prediction: chained CPI substantially higher than scrambled.");
        println!("  DRAM_PTI approximately equal (same working set).");
    }
    println!("Run under uProf assess_ext to capture H variables.");
    println!("Record counter output in execution_record.md V5.0.");
}
