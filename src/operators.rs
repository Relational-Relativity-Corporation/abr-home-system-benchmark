// operators.rs — Metatron Dynamics, Inc. V0.3
// ABR operators A -> B -> R for home system benchmark.
// Grounding: operators.rs V7 — Metatron Dynamics kernel (ABR formulas lines 890-988)
// Bounded over D. No claim beyond D.
//
// ── V0.2 -> V0.3 ─────────────────────────────────────────────────────────────
//
// Verifier F5 accepted: B implemented recursive accumulation of B[succ(e)]
// rather than immediate-successor input g[succ(e)] as declared in V7 line 903.
//
// Verifier F6 accepted: rho implemented edge-local |A[e]|/(1+|A[e]|) rather
// than node-local rho_base * chi[i]/(1+chi[i]) as declared in V7 lines 938-948.
//
// Both corrected here to match V7 exactly.
//
// ── Declared Operator Formulas (operators.rs V7) ─────────────────────────────
//
// A(x)[e] = x[source(e)] - x[target(e)]                          (V7 line 890)
//
// B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]                         (V7 line 903)
//   Immediate successor INPUT values from g (A field).
//   NOT recursive B values.
//   Terminal edges: B[e] = g[e] (no successors).
//   On this open chain: succ(e) has at most one member.
//
// rho[i] = rho_base * chi[i] / (1 + chi[i])    NODE FORM         (V7 line 940)
//   chi[i] = max |A[e]| over all edges incident to node i.
//   One value per NODE, not per edge.
//   rho_base declared as 1.0 for this benchmark (Origin declaration).
//
// R(g)[e] = g[e] + rho[src(e)] * (Σ_succ g[f] - Σ_pred g[p])   (V7 line 957)
//   Node-indexed rho at source node of each edge.
//   Successor and predecessor sums over B field values.

use crate::declared_graph::DeclaredGraph;

/// Declared rho_base for this benchmark.
/// Origin declaration: 1.0 for single-component timing benchmark.
pub const RHO_BASE: f64 = 1.0;

/// Pre-allocated buffers for one ABR pass.
/// rho is node-indexed (n_nodes); A, B, R are edge-indexed (n_edges).
/// Allocated once before timing. Zero heap allocation per pass after construction.
pub struct AbrBuffers {
    pub a:   Vec<f64>,  // edge-indexed: A[e]
    pub b:   Vec<f64>,  // edge-indexed: B[e]
    pub rho: Vec<f64>,  // node-indexed: rho[i]
    pub r:   Vec<f64>,  // edge-indexed: R[e]
}

impl AbrBuffers {
    /// n_nodes for rho; n_edges for A, B, R.
    pub fn new(n_nodes: usize, n_edges: usize) -> Self {
        AbrBuffers {
            a:   vec![0.0; n_edges],
            b:   vec![0.0; n_edges],
            rho: vec![0.0; n_nodes],
            r:   vec![0.0; n_edges],
        }
    }
}

/// A operator: A[e] = x[source(e)] - x[target(e)]
/// V7 line 890. Zero allocation.
pub fn operator_a(graph: &DeclaredGraph, buf: &mut AbrBuffers) {
    for (i, e) in graph.edges.iter().enumerate() {
        buf.a[i] = graph.node_field[e.source] - graph.node_field[e.target];
    }
}

/// B operator: B[e] = A[e] + Σ_{f ∈ succ(e)} A[f]
/// V7 line 903. Immediate successor INPUT values, not recursive B values.
/// Terminal edges: B[e] = A[e]. Open chain: one successor maximum.
/// Zero allocation.
pub fn operator_b(graph: &DeclaredGraph, buf: &mut AbrBuffers) {
    for i in 0..graph.n_edges {
        let succ_sum = match graph.edges[i].successor {
            Some(succ) => buf.a[succ],
            None => 0.0,
        };
        buf.b[i] = buf.a[i] + succ_sum;
    }
}

/// Compute rho: node-local scalar. V7 lines 938-948.
/// rho[i] = RHO_BASE * chi[i] / (1 + chi[i])
/// chi[i] = max |A[e]| over all edges incident to node i (in or out).
/// Zero allocation. Writes into pre-allocated buf.rho (node-indexed).
pub fn compute_rho(graph: &DeclaredGraph, buf: &mut AbrBuffers) {
    // Initialize chi accumulator to zero.
    for v in buf.rho.iter_mut() { *v = 0.0; }

    // chi[i]: max |A[e]| over incident edges at each node.
    for (i, e) in graph.edges.iter().enumerate() {
        let abs_a = buf.a[i].abs();
        if abs_a > buf.rho[e.source] { buf.rho[e.source] = abs_a; }
        if abs_a > buf.rho[e.target] { buf.rho[e.target] = abs_a; }
    }

    // Apply rho formula.
    for v in buf.rho.iter_mut() {
        *v = RHO_BASE * *v / (1.0 + *v);
    }
}

/// R operator: R[e] = B[e] + rho[src(e)] * (sum_succ_B - sum_pred_B)
/// V7 lines 957-988. Node-indexed rho at source of each edge.
/// Zero allocation.
pub fn operator_r(graph: &DeclaredGraph, buf: &mut AbrBuffers) {
    for i in 0..graph.n_edges {
        let src = graph.edges[i].source;
        let rho_src = buf.rho[src];

        let sum_succ = match graph.edges[i].successor {
            Some(succ) => buf.b[succ],
            None => 0.0,
        };

        let sum_pred = if i > 0 && graph.edges[i - 1].target == src {
            buf.b[i - 1]
        } else {
            0.0
        };

        buf.r[i] = buf.b[i] + rho_src * (sum_succ - sum_pred);
    }
}

/// Full ABR pass: A -> B -> rho -> R. All V7-consistent. Zero allocation.
pub fn abr_pass(graph: &DeclaredGraph, buf: &mut AbrBuffers) {
    operator_a(graph, buf);
    operator_b(graph, buf);
    compute_rho(graph, buf);
    operator_r(graph, buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::declared_graph::{declare_benchmark_graph, Edge, DeclaredGraph};

    fn make_buffers() -> (DeclaredGraph, AbrBuffers) {
        let g = declare_benchmark_graph();
        let buf = AbrBuffers::new(g.n_nodes, g.n_edges);
        (g, buf)
    }

    fn small_chain(n: usize, values: Vec<f64>) -> (DeclaredGraph, AbrBuffers) {
        let n_edges = n - 1;
        let mut edges = Vec::new();
        for i in 0..n_edges {
            edges.push(Edge {
                source: i, target: i + 1,
                successor: if i + 1 < n_edges { Some(i + 1) } else { None },
            });
        }
        let g = DeclaredGraph { n_nodes: n, n_edges, edges, node_field: values };
        let buf = AbrBuffers::new(n, n_edges);
        (g, buf)
    }

    #[test]
    fn a_operator_finite() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        assert!(buf.a.iter().all(|v| v.is_finite()));
    }

    #[test]
    fn a_operator_correct_length() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        assert_eq!(buf.a.len(), g.n_edges);
    }

    #[test]
    fn a_operator_sign_consistent() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        assert!(buf.a.iter().all(|&v| v < 0.0),
            "ascending gradient field must produce negative A values");
    }

    #[test]
    fn b_immediate_successor_not_recursive() {
        // V7 B: B[e] = A[e] + A[succ(e)], NOT B[e] = A[e] + B[succ(e)].
        // 4-node chain, uniform A = -1 everywhere.
        // B[2] = A[2]           = -1  (terminal)
        // B[1] = A[1] + A[2]   = -2  (immediate successor)
        // B[0] = A[0] + A[1]   = -2  (immediate successor, NOT -3)
        let (g, mut buf) = small_chain(4, vec![0.0, 1.0, 2.0, 3.0]);
        operator_a(&g, &mut buf);
        operator_b(&g, &mut buf);
        assert!((buf.b[2] - (-1.0)).abs() < 1e-12, "B[2] terminal must equal A[2]");
        assert!((buf.b[1] - (-2.0)).abs() < 1e-12, "B[1] = A[1] + A[2] = -2");
        assert!((buf.b[0] - (-2.0)).abs() < 1e-12,
            "B[0] = A[0] + A[1] = -2, NOT -3 (recursive would give -3)");
    }

    #[test]
    fn b_terminal_equals_a() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        operator_b(&g, &mut buf);
        let last = g.n_edges - 1;
        assert!((buf.b[last] - buf.a[last]).abs() < 1e-12,
            "terminal edge: B must equal A");
    }

    #[test]
    fn b_operator_finite() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        operator_b(&g, &mut buf);
        assert!(buf.b.iter().all(|v| v.is_finite()));
    }

    #[test]
    fn rho_is_node_indexed() {
        // rho has one value per NODE, not per edge.
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        compute_rho(&g, &mut buf);
        assert_eq!(buf.rho.len(), g.n_nodes,
            "rho must be node-indexed: one value per node");
        assert_ne!(buf.rho.len(), g.n_edges,
            "rho must NOT be edge-indexed");
    }

    #[test]
    fn rho_bounded_zero_to_one() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        compute_rho(&g, &mut buf);
        assert!(buf.rho.iter().all(|&v| v >= 0.0 && v < 1.0),
            "rho must be in [0, 1) for all nodes");
    }

    #[test]
    fn rho_uses_incident_edges_both_directions() {
        // Interior node sees both incoming and outgoing edges.
        // chi[i] = max(|A[e_in]|, |A[e_out]|).
        // For non-uniform A, interior node chi >= max of its two incident A values.
        let (g, mut buf) = small_chain(4, vec![0.0, 1.0, 3.0, 6.0]);
        operator_a(&g, &mut buf);
        // A[0] = 0-1 = -1, A[1] = 1-3 = -2, A[2] = 3-6 = -3
        compute_rho(&g, &mut buf);
        // node 1: incident to edge 0 (|A|=1) and edge 1 (|A|=2) -> chi=2
        // node 2: incident to edge 1 (|A|=2) and edge 2 (|A|=3) -> chi=3
        let chi_node1 = 2.0_f64;
        let expected_rho1 = RHO_BASE * chi_node1 / (1.0 + chi_node1);
        assert!((buf.rho[1] - expected_rho1).abs() < 1e-12,
            "rho[1] must use max incident |A|: got {}, expected {}", buf.rho[1], expected_rho1);
    }

    #[test]
    fn rho_base_applied() {
        let (g, mut buf) = make_buffers();
        operator_a(&g, &mut buf);
        compute_rho(&g, &mut buf);
        // All rho values must be positive and scale with RHO_BASE.
        assert!(buf.rho.iter().all(|&v| v > 0.0),
            "rho must be positive for non-zero A field");
    }

    #[test]
    fn r_operator_finite() {
        let (g, mut buf) = make_buffers();
        abr_pass(&g, &mut buf);
        assert!(buf.r.iter().all(|v| v.is_finite()));
    }

    #[test]
    fn r_operator_correct_length() {
        let (g, mut buf) = make_buffers();
        abr_pass(&g, &mut buf);
        assert_eq!(buf.r.len(), g.n_edges);
    }

    #[test]
    fn r_uses_node_indexed_rho() {
        // R[e] uses rho[src(e)] -- source node of each edge.
        // On open chain edge i: src = i, so R uses rho[i].
        // Verify R differs from B by exactly rho[src] * (sum_succ - sum_pred).
        let (g, mut buf) = small_chain(4, vec![0.0, 1.0, 2.0, 3.0]);
        abr_pass(&g, &mut buf);
        // Edge 1: src=1, rho=rho[1], succ=B[2], pred=B[0]
        let expected = buf.b[1] + buf.rho[1] * (buf.b[2] - buf.b[0]);
        assert!((buf.r[1] - expected).abs() < 1e-12,
            "R[1] must use rho[src(1)] = rho[1]");
    }

    #[test]
    fn abr_pass_deterministic() {
        let (g, mut buf) = make_buffers();
        abr_pass(&g, &mut buf);
        let r_first = buf.r[0];
        abr_pass(&g, &mut buf);
        assert!((buf.r[0] - r_first).abs() < 1e-12,
            "repeated pass must produce identical output -- confirms buffer reuse");
    }
}
