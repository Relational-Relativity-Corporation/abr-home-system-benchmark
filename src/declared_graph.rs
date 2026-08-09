// declared_graph.rs — Metatron Dynamics, Inc.
// Declared graph structure for home system benchmark.
// Bounded over D. No claim beyond D.
//
// ── Declaration ───────────────────────────────────────────────────────────────
//
// Graph is declared to produce a working set ≤ 1 MB — matching the
// community analysis working set declared in abr-infinity-fabric.
// This is the structural basis for the ratio comparison.
//
// Working set composition:
//   Node field:  N_NODES × f64 = N_NODES × 8 bytes
//   Edge field:  N_EDGES × f64 = N_EDGES × 8 bytes
//   Adjacency:   N_EDGES × usize (successor indices)
//
// Target: total ≤ 1,048,576 bytes (1 MB declared upper bound).
//
// Declared topology: open DAG (directed acyclic graph).
// Ring topology is inadmissible per kernel declaration.

/// Declared node count.
/// Working set target: ≤ 1 MB total.
pub const N_NODES: usize = 8_192;

/// Declared edge count (each node connects to next — open chain topology).
pub const N_EDGES: usize = N_NODES - 1;

/// Declared working set size in bytes.
/// node_field + edge_field + adjacency = 3 × N_EDGES × 8 + N_NODES × 8
pub const WORKING_SET_BYTES: usize =
    N_NODES * 8 + N_EDGES * 8 + N_EDGES * 8;

/// A declared directed edge.
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
    /// Successor edge index (None for terminal edges — open boundary).
    pub successor: Option<usize>,
}

/// Declared graph structure.
#[derive(Debug, Clone)]
pub struct DeclaredGraph {
    pub n_nodes: usize,
    pub n_edges: usize,
    pub edges: Vec<Edge>,
    /// Node field — observable values at each declared locus.
    pub node_field: Vec<f64>,
}

/// Declares the benchmark graph.
/// Open DAG: node i → node i+1 for i in 0..N_NODES-1.
/// No closing edge. Ring topology inadmissible.
pub fn declare_benchmark_graph() -> DeclaredGraph {
    let mut edges = Vec::with_capacity(N_EDGES);
    for i in 0..N_EDGES {
        edges.push(Edge {
            source: i,
            target: i + 1,
            successor: if i + 1 < N_EDGES { Some(i + 1) } else { None },
        });
    }

    // Node field: declared observable values.
    // Values chosen to produce non-trivial A output (gradient field).
    let node_field: Vec<f64> = (0..N_NODES)
        .map(|i| (i as f64) / (N_NODES as f64))
        .collect();

    DeclaredGraph {
        n_nodes: N_NODES,
        n_edges: N_EDGES,
        edges,
        node_field,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_set_within_declared_bound() {
        assert!(WORKING_SET_BYTES <= 1_048_576,
            "Working set must be ≤ 1 MB: got {} bytes", WORKING_SET_BYTES);
    }

    #[test]
    fn graph_has_declared_node_count() {
        let g = declare_benchmark_graph();
        assert_eq!(g.n_nodes, N_NODES);
    }

    #[test]
    fn graph_has_declared_edge_count() {
        let g = declare_benchmark_graph();
        assert_eq!(g.n_edges, N_EDGES);
        assert_eq!(g.edges.len(), N_EDGES);
    }

    #[test]
    fn no_ring_topology() {
        let g = declare_benchmark_graph();
        for edge in &g.edges {
            assert_ne!(edge.source, edge.target,
                "Self-loop detected — inadmissible");
        }
        // No edge targets node 0 (would close a ring)
        for edge in &g.edges {
            assert_ne!(edge.target, 0,
                "Edge targeting node 0 detected — ring inadmissible");
        }
    }

    #[test]
    fn terminal_edge_has_no_successor() {
        let g = declare_benchmark_graph();
        let last = g.edges.last().unwrap();
        assert!(last.successor.is_none(),
            "Terminal edge must have no successor — open boundary");
    }

    #[test]
    fn node_field_declared_length() {
        let g = declare_benchmark_graph();
        assert_eq!(g.node_field.len(), N_NODES);
    }

    #[test]
    fn node_field_finite() {
        let g = declare_benchmark_graph();
        assert!(g.node_field.iter().all(|v| v.is_finite()),
            "All node field values must be finite");
    }
}
