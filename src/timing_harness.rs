// timing_harness.rs — Metatron Dynamics, Inc. V0.3
// Wall-clock timing harness for ABR pass measurement.
// Bounded over D. No claim beyond D.
//
// V0.3: AbrBuffers::new now takes (n_nodes, n_edges) — rho is node-indexed.

use std::time::Instant;
use crate::declared_graph::DeclaredGraph;
use crate::operators::{AbrBuffers, abr_pass};

pub const N_WARM: usize = 100;
pub const N_TIMED: usize = 1_000;

#[derive(Debug, Clone)]
pub struct TimingResult {
    pub total_ns: u128,
    pub mean_ns: f64,
    pub min_ns: u128,
    pub max_ns: u128,
    pub n_passes: usize,
}

pub fn run_timing_harness(graph: &DeclaredGraph) -> TimingResult {
    // Allocate buffers once -- rho is node-indexed, A/B/R are edge-indexed.
    let mut buf = AbrBuffers::new(graph.n_nodes, graph.n_edges);

    // Warm phase -- ensure L3 residency.
    for _ in 0..N_WARM {
        abr_pass(graph, &mut buf);
    }

    // Timed phase -- zero allocation per pass.
    let mut pass_times_ns: Vec<u128> = Vec::with_capacity(N_TIMED);
    for _ in 0..N_TIMED {
        let start = Instant::now();
        abr_pass(graph, &mut buf);
        pass_times_ns.push(start.elapsed().as_nanos());
    }

    let total_ns: u128 = pass_times_ns.iter().sum();
    let mean_ns = total_ns as f64 / N_TIMED as f64;
    let min_ns = *pass_times_ns.iter().min().unwrap();
    let max_ns = *pass_times_ns.iter().max().unwrap();

    TimingResult { total_ns, mean_ns, min_ns, max_ns, n_passes: N_TIMED }
}
