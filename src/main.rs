// main.rs — Metatron Dynamics, Inc. V3.0
// ABR home system benchmark entry point — three-regime substrate comparison
// plus Regime 4: transition gradient sweep.
// Bounded over D. No claim beyond D.
//
// V3.0: Regime 4 added — fine-grained gradient sweep through declared
// transition zone (524 KB – 32 MB, 25 N values). Produces f(W) = S/L(W)
// and g(W) = B/L(W) and their first differences, locating the two
// transition surfaces identified in V2.0 to within one sweep step.
//
// WARNING: Regime 4 will take approximately 10–30 minutes depending on
// hardware load. Regimes 1–3 complete first and are unaffected.

use abr_home_system_benchmark::{
    declared_graph::{declare_benchmark_graph, WORKING_SET_BYTES},
    timing_harness::{run_timing_harness, N_WARM, N_TIMED},
    throughput_derivation::{derive_throughput, throughput_report},
    scaling::{run_scaling_measurement, scaling_report},
    process_topology::{build_process_graph, example_session_trace, process_graph_report},
    complexity_crossover::{run_crossover_measurement, crossover_report},
    cache_latency_model::{run_cache_sweep, cache_sweep_report},
    transition_gradient::{run_transition_gradient, compute_gradient_deltas, gradient_report},
};

fn main() {
    println!("ABR Home System Benchmark V3.0 — Metatron Dynamics, Inc.");
    println!("Bounded over D. No claim beyond D.\n");
    println!("V3.0: Regime 4 — transition gradient sweep.");
    println!("  25 N values, 524 KB through 32 MB.");
    println!("  Produces S/L(W) and B/L(W) and their first differences.");
    println!("  Locates SCRAMBLED and BRANCHY transition surfaces within one step.\n");
    println!("WARNING: Regime 4 takes 10–30 minutes. Regimes 1–3 complete first.\n");

    println!("V0.3: operators corrected to match kernel V7.");
    println!("B: immediate-successor input values (not recursive).");
    println!("rho: node-indexed, max incident |A|, rho_base=1.0.");
    println!("Pre-allocated buffers — zero heap allocation per pass.\n");

    // ── Regime 1: Primary benchmark ───────────────────────────────────────
    let graph = declare_benchmark_graph();
    println!("Declared graph: {} nodes, {} edges", graph.n_nodes, graph.n_edges);
    println!("Working set: {} bytes ({:.1} KB)",
        WORKING_SET_BYTES,
        WORKING_SET_BYTES as f64 / 1024.0);
    println!("Warm passes (discarded): {}", N_WARM);
    println!("Timed passes: {}\n", N_TIMED);

    println!("Running primary benchmark...");
    let timing = run_timing_harness(&graph);
    println!("Timing complete.\n");
    println!("Min per pass:  {} ns", timing.min_ns);
    println!("Max per pass:  {} ns", timing.max_ns);
    println!("Mean per pass: {:.1} ns\n", timing.mean_ns);

    let throughput = derive_throughput(timing.mean_ns);
    println!("{}", throughput_report(&throughput));

    println!("\nRunning scaling measurement...");
    let scaling_points = run_scaling_measurement();
    println!("{}", scaling_report(&scaling_points));

    // ── Regime 2: Process topology ────────────────────────────────────────
    println!("\n\n=== REGIME 2: PROCESS TOPOLOGY (OC-PT-1, OC-PT-2, OC-PT-3 open) ===\n");
    let process_graph = build_process_graph(example_session_trace());
    println!("{}", process_graph_report(&process_graph));
    println!("NOTE: example_session_trace() is a hand-transcribed partial trace.");
    println!("Replace with a full uProf export before treating as declared over D.\n");

    // ── Regime 3: Crossover matrix (V2.0 — unchanged) ────────────────────
    println!("\n=== REGIME 3: TASK-COMPLEXITY CROSSOVER V2.0 ===");
    println!("OC-CC-1 closed. Two distinct transition surfaces confirmed.");
    println!("OC-CC-4: XLARGE/XXLARGE use lighter protocol — elevated variance.\n");
    let crossover_points = run_crossover_measurement();
    println!("{}", crossover_report(&crossover_points));

    // ── Cache-latency curve (V2.0 — unchanged) ────────────────────────────
    println!("\n=== HARDWARE MECHANISM: CACHE-LATENCY CURVE (OC-CL-1/2/3 closed) ===\n");
    println!("NOTE: sweep can take 30–90 seconds at largest sizes.\n");
    let cache_points = run_cache_sweep();
    println!("{}", cache_sweep_report(&cache_points));

    // ── Regime 4: Transition gradient sweep (V3.0 — new) ─────────────────
    println!("\n=== REGIME 4: TRANSITION GRADIENT SWEEP V3.0 ===");
    println!("25 N values, 524 KB through 32 MB.");
    println!("Standard protocol (100w/1000t) through N=524,288.");
    println!("Lighter protocol (10w/100t) above N=524,288 — OC-TG-2.");
    println!("This regime will take 10–30 minutes. Do not interrupt.\n");
    let gradient_points = run_transition_gradient();
    let gradient_deltas = compute_gradient_deltas(&gradient_points);
    println!("{}", gradient_report(&gradient_points, &gradient_deltas));
}
