// main.rs — Metatron Dynamics, Inc. V1.0
// ABR home system benchmark entry point — three-regime substrate comparison.
// Bounded over D. No claim beyond D.

use abr_home_system_benchmark::{
    declared_graph::{declare_benchmark_graph, WORKING_SET_BYTES},
    timing_harness::{run_timing_harness, N_WARM, N_TIMED},
    throughput_derivation::{derive_throughput, throughput_report},
    scaling::{run_scaling_measurement, scaling_report},
    process_topology::{build_process_graph, example_session_trace, process_graph_report},
    complexity_crossover::{run_crossover_measurement, crossover_report},
    cache_latency_model::{run_cache_sweep, cache_sweep_report},
};

fn main() {
    println!("ABR Home System Benchmark V1.0 — Metatron Dynamics, Inc.");
    println!("Bounded over D. No claim beyond D.\n");
    println!("V1.0: three-regime binary-vs-relational substrate comparison.");
    println!("Regime 1: ABR operator cost (below, unchanged from V0.3).");
    println!("Regime 2: OS-to-CPU process exchange layer.");
    println!("Regime 3: task-complexity crossover (5 binary algos vs ABR chain).\n");

    println!("V0.3: operators corrected to match kernel V7.");
    println!("B: immediate-successor input values (not recursive).");
    println!("rho: node-indexed, max incident |A|, rho_base=1.0.");
    println!("Pre-allocated buffers — zero heap allocation per pass.\n");

    // ── Primary benchmark ─────────────────────────────────────────────────
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

    // ── Scaling measurement ───────────────────────────────────────────────
    println!("\nRunning scaling measurement across declared graph sizes...");
    println!("(addresses OC-HB-4 -- condition remains open pending operator isolation)\n");
    let scaling_points = run_scaling_measurement();
    println!("{}", scaling_report(&scaling_points));

    // ── Regime 2: OS-to-CPU process exchange layer ────────────────────────
    println!("\n\n=== REGIME 2: PROCESS TOPOLOGY (OC-PT-1, OC-PT-2, OC-PT-3 open) ===\n");
    let process_graph = build_process_graph(example_session_trace());
    println!("{}", process_graph_report(&process_graph));
    println!("NOTE: example_session_trace() is a hand-transcribed partial trace");
    println!("from a single uProf 'Select Profile Target' screen. Replace with a");
    println!("full uProf export before treating any result here as declared over D.\n");

    // ── Regime 3: task-complexity crossover ────────────────────────────────
    println!("\n=== REGIME 3: TASK-COMPLEXITY CROSSOVER (OC-CC-1, OC-CC-2, OC-CC-3 open) ===\n");
    let crossover_points = run_crossover_measurement();
    println!("{}", crossover_report(&crossover_points));

    // ── Cache-latency mechanism characterization (NOT a comparison) ────────
    println!("\n=== HARDWARE MECHANISM: CACHE-LATENCY CURVE (OC-CL-1, OC-CL-2, OC-CL-3 open) ===\n");
    println!("This sweep does NOT compare relational vs binary. It characterizes\n");
    println!("ONE mechanism directly: how access latency changes with working-set\n");
    println!("size, on the declared hardware, independent of any algorithm.\n");
    println!("NOTE: this sweep can take 30-90+ seconds at the largest declared\n");
    println!("sizes (up to 64MB working set) — this is expected, not a hang.\n");
    let cache_points = run_cache_sweep();
    println!("{}", cache_sweep_report(&cache_points));
}
