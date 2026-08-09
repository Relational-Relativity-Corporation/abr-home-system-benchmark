// main.rs — Metatron Dynamics, Inc. V0.3
// ABR home system benchmark entry point.
// Bounded over D. No claim beyond D.

use abr_home_system_benchmark::{
    declared_graph::{declare_benchmark_graph, WORKING_SET_BYTES},
    timing_harness::{run_timing_harness, N_WARM, N_TIMED},
    throughput_derivation::{derive_throughput, throughput_report},
    scaling::{run_scaling_measurement, scaling_report},
};

fn main() {
    println!("ABR Home System Benchmark V0.3 — Metatron Dynamics, Inc.");
    println!("Bounded over D. No claim beyond D.\n");

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
}
