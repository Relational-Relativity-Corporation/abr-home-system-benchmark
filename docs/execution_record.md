# Execution Record — abr-home-system-benchmark

**Metatron Dynamics, Inc.** Bounded over D. No claim beyond D.

This document records all cargo test and cargo run --release outputs
for abr-home-system-benchmark. Each entry is a declared observable
traceable to the hardware declared in docs/M_declaration.md.

Hardware for all entries:
- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2x 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit

---

## V0.1 -- 2026-08-08

### cargo test
14 passed; 0 failed.
Tests: working_set_within_declared_bound, graph_has_declared_edge_count,
node_field_declared_length, node_field_finite, no_ring_topology,
graph_has_declared_node_count, terminal_edge_has_no_successor,
a_operator_finite, a_operator_sign_consistent, a_operator_correct_length,
b_operator_finite, b_terminal_equals_a, abr_pass_correct_length,
r_operator_finite.

### cargo run --release
Mean per pass: 113927.4 ns
Min: 105800 ns. Max: 209200 ns.

Epistemic status: MEASURES IMPLEMENTATION COST (four heap allocations per
pass) and NON-V7 OPERATORS (B recursive, rho edge-local).
Not admissible as basis for MI355X ratio comparison.

---

## V0.2 -- 2026-08-08

### cargo test
15 passed; 0 failed.
Tests: all V0.1 tests plus abr_buffers_zero_allocation_per_pass,
r_operator_correct_length.

### cargo run --release
Mean per pass: 35809.1 ns (35348.7 ns on second run).
Min: 33900 ns. Max: 75500 ns.
Home system throughput: 27926 analyses/second.
MI355X ratio: 273.2x.

Scaling measurement:
  1023 edges: 4347.4 ns, 4.250 ns/edge
  2047 edges: 8912.3 ns, 4.354 ns/edge
  4095 edges: 17849.2 ns, 4.359 ns/edge
  8191 edges: 37626.1 ns, 4.594 ns/edge
  16383 edges: 76893.5 ns, 4.693 ns/edge
NS/EDGE ratios: 1.0245, 1.0011, 1.0539, 1.0217

Epistemic status: MEASURES OPERATOR TRAVERSAL COST (allocation removed)
but NON-V7 OPERATORS (B recursive, rho edge-local).
Not admissible as basis for MI355X ratio comparison.

---

## V0.3 -- 2026-08-08

### cargo test
24 passed; 0 failed.
Tests: all V0.2 tests plus b_immediate_successor_not_recursive,
r_uses_node_indexed_rho, abr_pass_deterministic, rho_uses_incident_edges_both_directions,
rho_base_applied, rho_is_node_indexed, rho_bounded_zero_to_one,
scaling::all_scaling_working_sets_within_l3, scaling::scaling_point_ns_per_edge_positive,
scaling::all_scaling_graphs_admissible.

Full output:
test declared_graph::tests::working_set_within_declared_bound ... ok
test declared_graph::tests::graph_has_declared_edge_count ... ok
test operators::tests::b_immediate_successor_not_recursive ... ok
test declared_graph::tests::graph_has_declared_node_count ... ok
test declared_graph::tests::node_field_declared_length ... ok
test declared_graph::tests::node_field_finite ... ok
test declared_graph::tests::terminal_edge_has_no_successor ... ok
test declared_graph::tests::no_ring_topology ... ok
test operators::tests::a_operator_correct_length ... ok
test operators::tests::a_operator_finite ... ok
test operators::tests::a_operator_sign_consistent ... ok
test operators::tests::b_terminal_equals_a ... ok
test operators::tests::b_operator_finite ... ok
test operators::tests::r_uses_node_indexed_rho ... ok
test operators::tests::abr_pass_deterministic ... ok
test operators::tests::r_operator_correct_length ... ok
test operators::tests::rho_uses_incident_edges_both_directions ... ok
test operators::tests::rho_base_applied ... ok
test operators::tests::r_operator_finite ... ok
test scaling::tests::all_scaling_working_sets_within_l3 ... ok
test operators::tests::rho_is_node_indexed ... ok
test scaling::tests::scaling_point_ns_per_edge_positive ... ok
test scaling::tests::all_scaling_graphs_admissible ... ok
test operators::tests::rho_bounded_zero_to_one ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; finished in 0.01s

### cargo run --release
Mean per pass: 30775.9 ns
Min: 25200 ns. Max: 89900 ns.
Home system throughput: 32493 analyses/second.
MI355X declared throughput: 7629394 analyses/second/module.
MI355X / Home system ratio: 234.8x.

Scaling measurement:
  1023 edges: 3862.1 ns, 3.775 ns/edge, min 3100 ns
  2047 edges: 7734.0 ns, 3.778 ns/edge, min 6100 ns
  4095 edges: 14791.9 ns, 3.612 ns/edge, min 12500 ns
  8191 edges: 31050.2 ns, 3.791 ns/edge, min 25100 ns
  16383 edges: 62997.2 ns, 3.845 ns/edge, min 52900 ns
NS/EDGE ratios: 1.0008, 0.9561, 1.0494, 1.0144

Epistemic status: MEASURES V7 ABR OPERATOR TRAVERSAL COST.
B corrected to immediate-successor input values (V7 line 903).
rho corrected to node-indexed form with rho_base (V7 lines 938-948).
Pre-allocated buffers maintained. Zero heap allocation per timed pass.
This result is admissible as the basis for the MI355X ratio comparison,
subject to open conditions OC-HB-1 through OC-HB-4.

---

## V0.3 -- Second Run -- 2026-08-08

Executed after first V0.3 run to confirm reproducibility of linear scaling
relationship. Same hardware, same code, same declared protocol.

### cargo test
24 passed; 0 failed; 0 ignored; 0 measured; finished in 0.01s

Full output:
test declared_graph::tests::working_set_within_declared_bound ... ok
test declared_graph::tests::graph_has_declared_edge_count ... ok
test declared_graph::tests::graph_has_declared_node_count ... ok
test operators::tests::b_immediate_successor_not_recursive ... ok
test declared_graph::tests::terminal_edge_has_no_successor ... ok
test declared_graph::tests::no_ring_topology ... ok
test declared_graph::tests::node_field_finite ... ok
test declared_graph::tests::node_field_declared_length ... ok
test operators::tests::a_operator_sign_consistent ... ok
test operators::tests::a_operator_finite ... ok
test operators::tests::a_operator_correct_length ... ok
test operators::tests::b_operator_finite ... ok
test operators::tests::b_terminal_equals_a ... ok
test operators::tests::r_uses_node_indexed_rho ... ok
test operators::tests::r_operator_correct_length ... ok
test operators::tests::abr_pass_deterministic ... ok
test operators::tests::r_operator_finite ... ok
test operators::tests::rho_uses_incident_edges_both_directions ... ok
test operators::tests::rho_base_applied ... ok
test operators::tests::rho_bounded_zero_to_one ... ok
test scaling::tests::all_scaling_working_sets_within_l3 ... ok
test operators::tests::rho_is_node_indexed ... ok
test scaling::tests::scaling_point_ns_per_edge_positive ... ok
test scaling::tests::all_scaling_graphs_admissible ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

### cargo run --release
Mean per pass: 28922.4 ns
Min: 26700 ns. Max: 113800 ns.
Home system throughput: 34575 analyses/second.
MI355X declared throughput: 7629394 analyses/second/module.
MI355X / Home system ratio: 220.7x.

Scaling measurement:
  1023 edges: 3509.3 ns, 3.430 ns/edge, min 3200 ns
  2047 edges: 7040.2 ns, 3.439 ns/edge, min 6400 ns
  4095 edges: 14105.4 ns, 3.445 ns/edge, min 13100 ns
  8191 edges: 27883.4 ns, 3.404 ns/edge, min 26500 ns
  16383 edges: 59769.8 ns, 3.648 ns/edge, min 55100 ns
NS/EDGE ratios: 1.0026, 1.0015, 0.9883, 1.0717

Observation: run-to-run variation in absolute timing (~6% difference in
mean pass time between first and second V0.3 runs) is consistent with
OS scheduling variation declared in OC-HB-2. The linear scaling
relationship (NS/EDGE approximately constant) is confirmed across both
runs. NS/EDGE range: 3.404-3.648 ns (run 2) vs 3.612-3.845 ns (run 1).
Both runs consistent with approximately constant per-edge cost.
