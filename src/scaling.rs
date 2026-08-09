// scaling.rs — Metatron Dynamics, Inc. V0.3
// Graph size scaling measurement — addresses OC-HB-4.
// Bounded over D. No claim beyond D.
//
// V0.3: AbrBuffers::new now takes (n_nodes, n_edges) — rho is node-indexed.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// Measures pass time across five declared graph sizes to characterize
// the cost-per-edge relationship on this hardware.
//
// NS/EDGE approximately constant across tested sizes is consistent with
// latency-bound execution. It does not independently identify the binding
// mechanism — operator isolation measurement (A, B, R separately) would
// be required for that. OC-HB-4 remains open.

use std::time::Instant;
use crate::operators::{AbrBuffers, abr_pass};
use crate::declared_graph::{DeclaredGraph, Edge};
use crate::timing_harness::{N_WARM, N_TIMED};

pub const SCALING_SIZES: [usize; 5] = [1_024, 2_048, 4_096, 8_192, 16_384];

#[derive(Debug, Clone)]
pub struct ScalingPoint {
    pub n_edges: usize,
    pub working_set_bytes: usize,
    pub mean_ns: f64,
    pub min_ns: u128,
    pub max_ns: u128,
    pub ns_per_edge: f64,
}

pub fn declare_scaling_graph(n_nodes: usize) -> DeclaredGraph {
    let n_edges = n_nodes - 1;
    let mut edges = Vec::with_capacity(n_edges);
    for i in 0..n_edges {
        edges.push(Edge {
            source: i,
            target: i + 1,
            successor: if i + 1 < n_edges { Some(i + 1) } else { None },
        });
    }
    let node_field: Vec<f64> = (0..n_nodes)
        .map(|i| (i as f64) / (n_nodes as f64))
        .collect();
    DeclaredGraph { n_nodes, n_edges, edges, node_field }
}

pub fn measure_scaling_point(n_nodes: usize) -> ScalingPoint {
    let graph = declare_scaling_graph(n_nodes);
    let n_edges = graph.n_edges;
    let working_set_bytes = n_nodes * 8 + n_edges * 8 + n_edges * 8;

    // rho is node-indexed: AbrBuffers::new(n_nodes, n_edges).
    let mut buf = AbrBuffers::new(graph.n_nodes, graph.n_edges);

    for _ in 0..N_WARM {
        abr_pass(&graph, &mut buf);
    }

    let mut pass_times_ns: Vec<u128> = Vec::with_capacity(N_TIMED);
    for _ in 0..N_TIMED {
        let start = Instant::now();
        abr_pass(&graph, &mut buf);
        pass_times_ns.push(start.elapsed().as_nanos());
    }

    let total_ns: u128 = pass_times_ns.iter().sum();
    let mean_ns = total_ns as f64 / N_TIMED as f64;
    let min_ns = *pass_times_ns.iter().min().unwrap();
    let max_ns = *pass_times_ns.iter().max().unwrap();
    let ns_per_edge = mean_ns / n_edges as f64;

    ScalingPoint { n_edges, working_set_bytes, mean_ns, min_ns, max_ns, ns_per_edge }
}

pub fn run_scaling_measurement() -> Vec<ScalingPoint> {
    SCALING_SIZES.iter().map(|&n| measure_scaling_point(n)).collect()
}

pub fn scaling_report(points: &[ScalingPoint]) -> String {
    let mut report = String::new();
    report.push_str("═══════════════════════════════════════════════════════════\n");
    report.push_str("ABR SCALING MEASUREMENT — COST PER EDGE CHARACTERIZATION\n");
    report.push_str("Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)\n");
    report.push_str("Metatron Dynamics, Inc. · Bounded over D.\n");
    report.push_str("═══════════════════════════════════════════════════════════\n");
    report.push_str(&format!("{:>8}  {:>10}  {:>12}  {:>10}  {:>12}\n",
        "N_EDGES", "WS (KB)", "MEAN (ns)", "NS/EDGE", "MIN (ns)"));
    report.push_str("───────────────────────────────────────────────────────────\n");

    for p in points {
        report.push_str(&format!("{:>8}  {:>10.1}  {:>12.1}  {:>10.3}  {:>12}\n",
            p.n_edges,
            p.working_set_bytes as f64 / 1024.0,
            p.mean_ns,
            p.ns_per_edge,
            p.min_ns));
    }

    report.push_str("───────────────────────────────────────────────────────────\n");
    report.push_str("NS/EDGE ratios (successive sizes):\n");
    for i in 1..points.len() {
        let ratio = points[i].ns_per_edge / points[i-1].ns_per_edge;
        report.push_str(&format!("  {} -> {} edges: {:.4}\n",
            points[i-1].n_edges, points[i].n_edges, ratio));
    }
    report.push_str("───────────────────────────────────────────────────────────\n");
    report.push_str("Interpretation:\n");
    report.push_str("  NS/EDGE ratio near 1.0 -> consistent with constant per-edge cost.\n");
    report.push_str("  Does not independently identify the binding mechanism.\n");
    report.push_str("  OC-HB-4 remains open: operator isolation required.\n");
    report.push_str("═══════════════════════════════════════════════════════════\n");

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_scaling_graphs_admissible() {
        for &n in &SCALING_SIZES {
            let g = declare_scaling_graph(n);
            assert_eq!(g.n_nodes, n);
            assert_eq!(g.n_edges, n - 1);
            for edge in &g.edges {
                assert_ne!(edge.source, edge.target);
                assert_ne!(edge.target, 0);
            }
            assert!(g.edges.last().unwrap().successor.is_none());
        }
    }

    #[test]
    fn all_scaling_working_sets_within_l3() {
        let l3_bytes = 32 * 1024 * 1024;
        for &n in &SCALING_SIZES {
            let ws = n * 8 + (n-1) * 8 + (n-1) * 8;
            assert!(ws < l3_bytes,
                "Working set for n={} exceeds L3: {} bytes", n, ws);
        }
    }

    #[test]
    fn scaling_point_ns_per_edge_positive() {
        let g = declare_scaling_graph(1_024);
        let mut buf = AbrBuffers::new(g.n_nodes, g.n_edges);
        let start = Instant::now();
        abr_pass(&g, &mut buf);
        let elapsed = start.elapsed().as_nanos() as f64;
        let ns_per_edge = elapsed / g.n_edges as f64;
        assert!(ns_per_edge > 0.0, "NS per edge must be positive");
    }
}
