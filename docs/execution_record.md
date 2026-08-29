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

---

# Execution Record — abr-home-system-benchmark

Metatron Dynamics, Inc. Bounded over D. No claim beyond D.

## V1.0 — Sandbox Build/Test Verification (2026-08-28)

IMPORTANT: The run below was executed in a Linux sandbox (Ubuntu, apt-installed
rustc 1.75), NOT on the declared hardware (Ryzen 5 7600X / DDR5-5600 / Windows
11). It confirms the V1.0 code compiles, all tests pass, and the program runs
end to end. It does NOT constitute a declared measurement under D, D2, or D3 —
see OC-CC-3 and OC-HB-1/2. Run `cargo test` and `cargo run --release` on the
declared hardware to produce admissible figures for Regime 3 and any future
Regime 2 measurement.

### cargo test --release

test complexity_crossover::tests::binary_all_pairs_deterministic ... ok
test complexity_crossover::tests::binary_all_pairs_zero_for_uniform_input ... ok
test complexity_crossover::tests::binary_op_count_matches_all_pairs_formula ... ok
test complexity_crossover::tests::crossover_report_does_not_panic ... ok
test complexity_crossover::tests::measure_tier_small_produces_finite_positive_timings ... ok
test complexity_crossover::tests::relational_edge_count_matches_declared_topology ... ok
test complexity_crossover::tests::tier_node_counts_declared_correctly ... ok
test declared_graph::tests::graph_has_declared_edge_count ... ok
test declared_graph::tests::graph_has_declared_node_count ... ok
test declared_graph::tests::no_ring_topology ... ok
test declared_graph::tests::node_field_declared_length ... ok
test declared_graph::tests::node_field_finite ... ok
test declared_graph::tests::terminal_edge_has_no_successor ... ok
test declared_graph::tests::working_set_within_declared_bound ... ok
test operators::tests::a_operator_correct_length ... ok
test operators::tests::a_operator_finite ... ok
test operators::tests::a_operator_sign_consistent ... ok
test operators::tests::abr_pass_deterministic ... ok
test operators::tests::b_immediate_successor_not_recursive ... ok
test operators::tests::b_operator_finite ... ok
test operators::tests::b_terminal_equals_a ... ok
test operators::tests::r_operator_correct_length ... ok
test operators::tests::r_operator_finite ... ok
test operators::tests::r_uses_node_indexed_rho ... ok
test operators::tests::rho_base_applied ... ok
test operators::tests::rho_bounded_zero_to_one ... ok
test operators::tests::rho_is_node_indexed ... ok
test operators::tests::rho_uses_incident_edges_both_directions ... ok
test process_topology::tests::abr_operators_apply_to_process_graph_unmodified ... ok
test process_topology::tests::declared_records_normalize_to_zero_min ... ok
test process_topology::tests::example_trace_parses_and_normalizes ... ok
test process_topology::tests::parses_hms_correctly ... ok
test process_topology::tests::process_graph_declares_expected_edge_count ... ok
test process_topology::tests::process_graph_edges_are_temporally_ordered ... ok
test process_topology::tests::process_graph_has_no_ring_topology ... ok
test process_topology::tests::process_graph_terminal_edge_has_no_successor ... ok
test process_topology::tests::rejects_malformed_timestamp ... ok
test process_topology::tests::report_does_not_panic_on_empty_graph ... ok
test scaling::tests::all_scaling_graphs_admissible ... ok
test scaling::tests::all_scaling_working_sets_within_l3 ... ok
test scaling::tests::scaling_point_ns_per_edge_positive ... ok
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.85s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

41/41 tests passing. 24 from Regime 1 (unchanged from V0.3), 10 from
Regime 2 (process_topology), 6 from Regime 3 (complexity_crossover),
1 shared graph test.

### cargo run --release (sandbox — informational only)

ABR Home System Benchmark V1.0 — Metatron Dynamics, Inc.
Bounded over D. No claim beyond D.

V1.0: three-regime binary-vs-relational substrate comparison.
Regime 1: ABR operator cost (below, unchanged from V0.3).
Regime 2: OS-to-CPU process exchange layer.
Regime 3: task-complexity crossover (binary all-pairs vs ABR chain).

V0.3: operators corrected to match kernel V7.
B: immediate-successor input values (not recursive).
rho: node-indexed, max incident |A|, rho_base=1.0.
Pre-allocated buffers — zero heap allocation per pass.

Declared graph: 8192 nodes, 8191 edges
Working set: 196592 bytes (192.0 KB)
Warm passes (discarded): 100
Timed passes: 1000

Running primary benchmark...
Timing complete.

Min per pass:  43853 ns
Max per pass:  111764 ns
Mean per pass: 47054.0 ns

═══════════════════════════════════════════════════════════
ABR HOME SYSTEM BENCHMARK — THROUGHPUT DERIVATION
Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)
Metatron Dynamics, Inc. · Bounded over D.
═══════════════════════════════════════════════════════════
Mean time per ABR pass (A->B->R):  47054.0 ns
Home system throughput:            21252 analyses/second
MI355X declared throughput:        7629394 analyses/second/module
MI355X / Home system ratio:        359.0x
───────────────────────────────────────────────────────────
Epistemic status: MIXED: MI355X is STRUCTURAL (abr-infinity-fabric); home system is MEASURED (wall-clock, L3-resident). OC-HB-1: L3 bandwidth not directly measured. OC-HB-3: MI355X correspondence requires instrument measurement.
═══════════════════════════════════════════════════════════

Running scaling measurement across declared graph sizes...
(addresses OC-HB-4 -- condition remains open pending operator isolation)

═══════════════════════════════════════════════════════════
ABR SCALING MEASUREMENT — COST PER EDGE CHARACTERIZATION
Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)
Metatron Dynamics, Inc. · Bounded over D.
═══════════════════════════════════════════════════════════
 N_EDGES     WS (KB)     MEAN (ns)     NS/EDGE      MIN (ns)
───────────────────────────────────────────────────────────
    1023        24.0        5698.7       5.571          5400
    2047        48.0       11755.7       5.743         10929
    4095        96.0       26986.0       6.590         24919
    8191       192.0       47083.3       5.748         44097
   16383       384.0       95254.9       5.814         88415
───────────────────────────────────────────────────────────
NS/EDGE ratios (successive sizes):
  1023 -> 2047 edges: 1.0309
  2047 -> 4095 edges: 1.1475
  4095 -> 8191 edges: 0.8723
  8191 -> 16383 edges: 1.0115
───────────────────────────────────────────────────────────
Interpretation:
  NS/EDGE ratio near 1.0 -> consistent with constant per-edge cost.
  Does not independently identify the binding mechanism.
  OC-HB-4 remains open: operator isolation required.
═══════════════════════════════════════════════════════════



=== REGIME 2: PROCESS TOPOLOGY (OC-PT-1, OC-PT-2, OC-PT-3 open) ===

─────────────────────────────────────────────
PROCESS TOPOLOGY — REGIME 2 STRUCTURAL SUMMARY
Bounded over D. No claim beyond D.
─────────────────────────────────────────────
Declared processes (nodes): 26
Declared co-activation edges: 24
Edge density (declared/possible consecutive pairs): 0.960
Co-activation window: 2.0s (OC-PT-2: declared, not derived)
Observable: activation-time offset only (OC-PT-1: idle/active
utilization not yet ingested — this is a proxy observable).
─────────────────────────────────────────────

NOTE: example_session_trace() is a hand-transcribed partial trace
from a single uProf 'Select Profile Target' screen. Replace with a
full uProf export before treating any result here as declared over D.


=== REGIME 3: TASK-COMPLEXITY CROSSOVER (OC-CC-1, OC-CC-2, OC-CC-3 open) ===

═══════════════════════════════════════════════════════════════════
REGIME 3 — TASK-COMPLEXITY CROSSOVER (binary all-pairs vs ABR chain)
Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)
Metatron Dynamics, Inc. · Bounded over D.
═══════════════════════════════════════════════════════════════════
    TIER           N     BINARY (ns)  RELATIONAL (ns)     REL/BIN
───────────────────────────────────────────────────────────────────
   SMALL          64          2875.1           359.3      0.1250
  MEDIUM        1024        745239.5          5919.6      0.0079
   LARGE        8192      47588118.6         46819.3      0.0010
───────────────────────────────────────────────────────────────────
Interpretation:
  REL/BIN < 1.0 -> relational path faster at this tier.
  REL/BIN > 1.0 -> binary path faster at this tier.
  A crossing from >1.0 to <1.0 across tiers is consistent with the
  quadratic-vs-linear crossover shape observed in the language-model
  token-count case (abr-relational-attention). It does not by itself
  establish the mechanism — see OC-CC-1, OC-CC-2, OC-CC-3.
═══════════════════════════════════════════════════════════════════


---

## V1.0 — Declared Hardware Run (2026-08-29)

Executed on the declared hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11
Home 64-bit. This is the admissible run superseding the 2026-08-28 sandbox
sanity check for Regime 3, and confirming Regime 2 structure.

### cargo test --release

41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out.

### cargo run --release

Regime 1 (V1.0 confirmation, third run on this hardware):
Min per pass:  25,200 ns
Max per pass:  86,800 ns
Mean per pass: 30,057.8 ns
Home system throughput: 33,269 analyses/second
MI355X / Home system ratio: 229.3x

Scaling:
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 4,003.8   | 3.914   | 3,100    |
| 2,047   | 48.0    | 7,121.5   | 3.479   | 6,200    |
| 4,095   | 96.0    | 15,171.1  | 3.705   | 12,400   |
| 8,191   | 192.0   | 29,871.6  | 3.647   | 25,200   |
| 16,383  | 384.0   | 61,058.1  | 3.727   | 52,600   |

Consistent with Run 1 (3.4-3.8 ns/edge) and Run 2 (3.4-3.8 ns/edge) —
third independent confirmation of approximately constant per-edge cost.

Regime 2: 26 declared processes, 24 declared co-activation edges,
edge density 0.960.

Regime 3 (DECLARED — supersedes 2026-08-28 sandbox sanity run):
| TIER   | N     | BINARY (ns)  | RELATIONAL (ns) | REL/BIN |
|--------|-------|--------------|------------------|---------|
| SMALL  | 64    | 2,416.1      | 242.2            | 0.1002  |
| MEDIUM | 1,024 | 660,536.0    | 3,720.3          | 0.0056  |
| LARGE  | 8,192 | 42,218,566.8 | 31,357.4         | 0.0007  |

No crossover within declared tiers, confirmed on hardware. OC-CC-1
requires tier revision downward before the next run.

---

## V1.1 — Sandbox Build/Test Verification (2026-08-28)

Added binary_baselines.rs (5 declared binary algorithms) and rewrote
complexity_crossover.rs to run the full 5-algorithm x 3-tier matrix
(15 comparison points) instead of a single binary baseline. Addresses
OC-CC-2.

### cargo test --release (sandbox)

48/48 tests passing (41 from V1.0 + 7 new: 6 in binary_baselines.rs,
1 additional in complexity_crossover.rs matrix restructure — net test
count changed because measure_tier() was replaced with measure_tier_matrix()).

One test failure caught and fixed during this build: op_counts_ordered_by
_declared_complexity_at_large_n asserted sort_scan < windowed, which does
not hold for the declared constants (K=8, log2(8192)=13) — corrected to
assert only the two clear complexity-class endpoints.

### cargo run --release (sandbox — informational only, NOT declared hardware)

Regime 3 matrix (15 points):

| TIER | BINARY_ALGO | CLASS | BINARY (ns) | RELATIONAL (ns) | REL/BIN |
|---|---|---|---|---|---|
| SMALL | ALL_PAIRS | O(N^2) | 2,918.2 | 365.8 | 0.1254 |
| SMALL | LINEAR_SCAN | O(N) | 79.6 | 365.8 | 4.5948 |
| SMALL | WINDOWED | O(N*K) | 372.6 | 365.8 | 0.9818 |
| SMALL | PREFIX_SUM | O(N) | 77.4 | 365.8 | 4.7289 |
| SMALL | SORT_SCAN | O(N log N) | 152.3 | 365.8 | 2.4017 |
| MEDIUM | ALL_PAIRS | O(N^2) | 780,528.4 | 5,677.6 | 0.0073 |
| MEDIUM | LINEAR_SCAN | O(N) | 763.0 | 5,677.6 | 7.4413 |
| MEDIUM | WINDOWED | O(N*K) | 6,121.6 | 5,677.6 | 0.9275 |
| MEDIUM | PREFIX_SUM | O(N) | 761.5 | 5,677.6 | 7.4554 |
| MEDIUM | SORT_SCAN | O(N log N) | 1,673.7 | 5,677.6 | 3.3922 |
| LARGE | ALL_PAIRS | O(N^2) | 48,430,694.0 | 50,936.0 | 0.0011 |
| LARGE | LINEAR_SCAN | O(N) | 5,829.7 | 50,936.0 | 8.7373 |
| LARGE | WINDOWED | O(N*K) | 47,575.7 | 50,936.0 | 1.0706 |
| LARGE | PREFIX_SUM | O(N) | 6,021.8 | 50,936.0 | 8.4586 |
| LARGE | SORT_SCAN | O(N log N) | 14,415.8 | 50,936.0 | 3.5334 |

Finding: relational wins only against ALL_PAIRS (O(N^2)) at every tier.
Loses to every O(N) and O(N log N) baseline, margin widening with N.
Requires declared-hardware confirmation before treating as admissible
over D — see M_declaration.md V1.1 addendum.

---

## V1.3 — Sandbox Build/Test Verification (2026-08-29)

Added binary_baselines.rs mechanism-isolating baselines (ScrambledAccess,
BranchyDataDependent) and cache_latency_model.rs (standalone cache-latency
characterization, not a comparison).

### cargo test --release (sandbox)

60/60 tests passing.

### Regime 3 matrix with mechanism-isolating baselines (sandbox, 2026-08-29)

Added SCRAMBLED and BRANCHY (both O(N), same op count as LINEAR_SCAN) to
the matrix. Result: both perform essentially identically to plain
LINEAR_SCAN — binary wins by 4.6x-8.5x at every tier, same as the
structureless O(N) baselines. Likely explanation: at the tested tier
sizes (max 8,192 elements = 64 KB), the working set stays within L2
cache even under scrambled access, so the cache-unfriendly mechanism
never actually triggers. This motivated the cache_latency_model.rs sweep.

### cargo run --release — cache-latency sweep (sandbox, NOT declared hardware)

| N | WS (bytes) | MEAN (ns) | NS/ACCESS | TIER |
|---|---|---|---|---|
| 512 | 4,096 | 345.4 | 0.6746 | <= L1d |
| 1,024 | 8,192 | 637.4 | 0.6225 | <= L1d |
| 2,048 | 16,384 | 1,238.8 | 0.6049 | <= L1d |
| 4,096 | 32,768 | 2,458.3 | 0.6002 | <= L1d |
| 8,192 | 65,536 | 4,956.9 | 0.6051 | <= L2 |
| 16,384 | 131,072 | 11,446.6 | 0.6986 | <= L2 |
| 32,768 | 262,144 | 23,264.8 | 0.7100 | <= L2 |
| 65,536 | 524,288 | 50,116.7 | 0.7647 | <= L2 |
| 131,072 | 1,048,576 | 126,554.9 | 0.9655 | <= L2 |
| 262,144 | 2,097,152 | 430,119.5 | 1.6408 | <= L3 |
| 524,288 | 4,194,304 | 1,035,998.5 | 1.9760 | <= L3 |
| 1,048,576 | 8,388,608 | 2,526,576.1 | 2.4095 | <= L3 |
| 2,097,152 | 16,777,216 | 7,195,032.1 | 3.4309 | <= L3 |
| 4,194,304 | 33,554,432 | 20,803,006.8 | 4.9598 | <= L3 |
| 8,388,608 | 67,108,864 | 60,179,692.9 | 7.1740 | > L3 (RAM) |

Gradual ramp from 0.60 ns/access (L1) to 7.17 ns/access (past L3) —
~11.9x range. Notable step at the L2->L3 crossing (0.97 -> 1.64
ns/access). Requires declared-hardware confirmation — sandbox/
virtualized environments may not reflect bare-metal Ryzen cache
behavior.
