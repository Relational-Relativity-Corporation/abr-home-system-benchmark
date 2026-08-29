# M Declaration -- abr-home-system-benchmark

**Metatron Dynamics, Inc.** V0.3. Bounded over D. No claim beyond D.

---

## Declared Domain (D)

D := timing observations (nanoseconds per A -> B -> R pass) over a declared
graph with working set <= 1 MB, resident in L3 cache, on the following
declared hardware:

- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2x 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit

## Declared Observable (M)

Wall-clock time per A -> B -> R pass.
Instrument: std::time::Instant (Rust standard library).
Units: nanoseconds per pass.

## Cache Residency Protocol

N_WARM = 100 passes executed and discarded before timing begins.
Declared purpose: ensure working set resident in L3 (32 MB) before measurement.
Candidate declared bottleneck: L3 cache -- not yet confirmed.
Basis: working set (<= 1 MB) < L3 capacity (32 MB).
See OC-HB-1, OC-HB-2, OC-HB-4.

## Declared Graph

Open DAG: N_NODES = 8,192 nodes, N_EDGES = 8,191 directed edges.
Topology: open chain (node i -> node i+1). Ring topology inadmissible.
Working set: 192 KB (measured at runtime -- within declared <= 1 MB bound).

---

## Declared Operator Formulas

All operators declared from and consistent with kernel operators.rs V7
-- Metatron Dynamics (ABR formulas lines 890-988). No deviation.

### A operator (V7 line 890)
A(x)[e] = x[source(e)] - x[target(e)]
NodeField -> edge values. Directed difference only.

### B operator (V7 line 903)
B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]
Immediate successor INPUT values from g (A field) -- not recursive B values.
Terminal edges: B[e] = g[e] (no successors -- open boundary).
On this open chain: succ(e) has at most one member.

### rho (V7 lines 938-948) -- NODE FORM
rho[i] = rho_base * chi[i] / (1 + chi[i])
chi[i] = max |A[e]| over all edges incident to node i (in or out).
One value per NODE. Not per edge.
rho_base = 1.0 (Origin declaration for this benchmark).

### R operator (V7 lines 957-988)
R(g)[e] = g[e] + rho[src(e)] * (Σ_{f∈succ(e)} g[f] - Σ_{p∈pred(e)} g[p])
Node-indexed rho at source node of each edge.
Successor and predecessor sums over B field values.

### Component pairs
No component pairs declared for this benchmark. The cross-topology
term in V7 R (lines 971-984) is structurally zero and correctly absent.
This is an admissible single-component projection of the full operator.

---

## Implementation Admissibility Condition

The declared operators define relational structure over declared observables.
They do not specify memory allocation strategy. However, an implementation
that introduces memory operations not required by the operator mathematics
breaks the provenance chain between the declared observable and the measured
result -- the timing measurement then reflects implementation artifacts, not
operator traversal.

For a timing measurement to be admissible as M(observable):

  1. The working set must be declared and resident before timing begins
     (warm-pass protocol -- declared above).
  2. No memory allocation may occur during a timed pass that is not
     required by the operator mathematics.
  3. Buffers used by A, B, rho, and R must be pre-allocated once before
     the timed phase and reused across all timed passes.
     Note: rho buffer is node-indexed (n_nodes); A, B, R are edge-indexed
     (n_edges). Both allocated in AbrBuffers before warm phase.

A timing measurement taken over an implementation that violates condition 2
is a measurement of a different observable: implementation cost, not
operator traversal cost. It must be declared as such.

---

## Version History

### V0.1 -- 2026-08-08 (superseded)
Four heap allocations per ABR pass. B implemented as recursive accumulation
of B[succ(e)] -- not V7. rho implemented as edge-local |A[e]|/(1+|A[e]|)
-- not V7.
Measured: 113,927 ns/pass, 8,778 analyses/second.
Epistemic status: MEASURES IMPLEMENTATION COST and NON-V7 OPERATORS.
Not admissible as basis for MI355X ratio comparison.

### V0.2 -- 2026-08-08 (superseded)
Allocation removed. B and rho still non-V7.
Measured: 35,349 ns/pass, 28,290 analyses/second, ~4.5 ns/edge.
Epistemic status: MEASURES OPERATOR TRAVERSAL COST but NON-V7 OPERATORS.
Not admissible as basis for MI355X ratio comparison.

### V0.3 -- 2026-08-08 (current)
B corrected to immediate-successor input values (V7 line 903).
rho corrected to node-indexed form with rho_base (V7 lines 938-948).
R confirmed node-indexed rho at source (V7 lines 957-988).
Pre-allocated buffers maintained. rho buffer node-indexed (n_nodes).
24/24 tests passing. Two independent runs recorded in docs/execution_record.md.

Run 1: 30,775.9 ns/pass, 32,493 analyses/second, MI355X ratio 234.8x.
Run 2: 28,922.4 ns/pass, 34,575 analyses/second, MI355X ratio 220.7x.
Run-to-run variation ~6% -- consistent with OC-HB-2 (OS scheduling).

Epistemic status: MEASURES V7 ABR OPERATOR TRAVERSAL COST.
Admissible as basis for MI355X ratio comparison subject to OC-HB-1
through OC-HB-4.

---

## Scaling Measurement (V0.3)

Five declared graph sizes. Same V7-consistent A -> B -> rho -> R sequence.
Same warm-pass protocol. Same hardware. Two independent runs recorded.

### Run 1 -- 2026-08-08
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,862     | 3.775   | 3,100    |
| 2,047   | 48.0    | 7,734     | 3.778   | 6,100    |
| 4,095   | 96.0    | 14,792    | 3.612   | 12,500   |
| 8,191   | 192.0   | 31,050    | 3.791   | 25,100   |
| 16,383  | 384.0   | 62,997    | 3.845   | 52,900   |
NS/EDGE ratios: 1.0008, 0.9561, 1.0494, 1.0144

### Run 2 -- 2026-08-08
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,509     | 3.430   | 3,200    |
| 2,047   | 48.0    | 7,040     | 3.439   | 6,400    |
| 4,095   | 96.0    | 14,105    | 3.445   | 13,100   |
| 8,191   | 192.0   | 27,883    | 3.404   | 26,500   |
| 16,383  | 384.0   | 59,770    | 3.648   | 55,100   |
NS/EDGE ratios: 1.0026, 1.0015, 0.9883, 1.0717

### Interpretation
NS/EDGE is approximately constant across all five declared graph sizes
in both runs (range 3.404-3.845 ns/edge across all observations, ratios
0.956-1.072). This result is consistent with approximately constant
per-edge cost on this hardware for V7 ABR operators and the declared
open-chain topology.

Run-to-run variation in absolute timing (~6%) is consistent with OS
scheduling effects declared in OC-HB-2. The approximately constant
NS/EDGE relationship is reproduced across both independent runs.

This result does not independently identify the binding mechanism.
Operator isolation measurement (timing A, B, rho, R separately) would
be required to attribute the cost to a specific operator or hardware
constraint. OC-HB-4 remains open.

---

## Derived Compute Relationship (V0.3)

For the declared benchmark topology and tested range on this hardware:

  ABR compute time scales approximately as n_declared_relations x ~3.4-3.8 ns/edge

Measured over 1,023-16,383 edges under the declared warm-pass protocol.
Hardware-specific and declaration-specific. Does not generalize to
arbitrary ABR declarations or graph topologies without further measurement.

STRUCTURAL EXTRAPOLATION NOTE: Application of this relationship to
declared graphs outside the measured range (1,023-16,383 edges) or with
different declared topology, field structure, or component pairs is a
structural extrapolation under the constant-per-edge assumption -- not a
direct measurement from this repository. For example, application to the
1MLC antibody-antigen interface (29 declared relations) would give
approximately 29 x 3.8 ns = ~110 ns -- but 29 edges lies far outside
the measured range (minimum 1,023 edges), the topology differs from the
open chain declared here, and no component pairs are declared in this
benchmark. Such extrapolations require explicit declaration and are not
established by this measurement alone.

---

## Structural Parallel to MI355X (abr-infinity-fabric)

| Property | MI355X | Home System |
|---|---|---|
| Working set | 1 MB | 192 KB (primary benchmark) |
| Residency | HBM3E (288 GB) | L3 (32 MB) |
| Declared bottleneck | Under revision (OC-IF-5) | Candidate L3 (OC-HB-4) |
| Operators | V7 ABR declared | V7 ABR confirmed (V0.3) |
| Throughput status | Structural upper bound | Measured (V0.3, two runs) |

MI355X ratio range across two V0.3 runs: 220.7x - 234.8x.
Mixed epistemic: MI355X figure is structural (abr-infinity-fabric);
home system figure is measured. Both carry open conditions (OC-HB-3).

---

## Open Conditions

- OC-HB-1: L3 bandwidth not directly measured via hardware performance
  counters. Cache residency declared via warm-pass protocol only.
  Direct measurement via performance counters closes this condition.

- OC-HB-2: L3 residency assumes no OS interruption or cache eviction
  during timed passes. Not directly verifiable without performance
  counter access on Windows 11. Run-to-run variation (~6%) is
  consistent with OS scheduling effects.

- OC-HB-3: MI355X throughput figure is a structural derivation
  (abr-infinity-fabric throughput_invariants.rs). The ratio
  (MI355X / home system) is a mixed epistemic quantity. Correspondence
  requires direct MI355X measurement (OC-IF-3 in abr-infinity-fabric).

- OC-HB-4: The scaling measurement is consistent with approximately
  constant per-edge cost across the tested range and topology. This
  does not isolate individual operators (A, B, rho, R) or identify
  the binding hardware constraint. Operator isolation measurement
  required to attribute cost. Declared open.

---

## V1.0 Addendum — Three-Regime Expansion (2026-08-28)

Everything above (D, M, operator formulas, V0.1-V0.3 history, Regime 1
scaling tables) is preserved unchanged. It remains the declared basis for
OC-IF-5 in abr-infinity-fabric. V1.0 adds two new declared domains
alongside D, without modifying D itself.

### D2 — Process Topology Domain (Regime 2)

D2 := activation-time observations over a real or recorded OS process
list on the declared hardware (see README.md, Declared Hardware).

M2 (declared observable, V1.0): start_offset_secs — wall-clock activation
timestamp of each process, normalized to seconds since the earliest
recorded activation in the session. Instrument: manual transcription from
AMD uProf "Select Profile Target" (V1.0) or a full uProf CSV export
(future work).

Declared edge rule: two temporally-consecutive processes (sorted by
start_offset_secs) are connected if their gap <= CO_ACTIVATION_WINDOW_SECS
(2.0s, declared, not derived — OC-PT-2).

D2 is explicitly declared INCOMPLETE relative to the actual question under
discussion (whether relational structure in process dependency/idle
patterns could reduce redundant OS-to-CPU switching). Activation timing
alone does not carry idle/active state. See OC-PT-1.

Test result: V7 operators (operators.rs, unmodified) execute over a
DeclaredGraph built from example_session_trace() (26 real processes,
hand-transcribed 2026-08-28) and produce finite output. This confirms
computability, not efficiency.

DECLARED HARDWARE RUN (Ryzen 5 7600X / DDR5-5600 / Windows 11 Home),
2026-08-29: 26 declared processes, 24 declared co-activation edges,
edge density 0.960 (identical structure to the sandbox run, as expected
since this domain's input is a fixed hand-transcribed trace rather than
a live hardware measurement — see OC-PT-1 for what remains to be
measured).

### D3 — Task-Complexity Crossover Domain (Regime 3)

D3 := wall-clock timing of two declared computations over the same
declared hardware, for direct comparison:

  1. Binary baseline: all-pairs difference over N values (O(N^2)),
     same elementary subtraction as ABR operator A.
  2. Relational baseline: the exact Regime 1 ABR chain (declare_scaling_graph,
     operators::abr_pass) over the same N nodes (O(N)).

M3 (declared observable): mean wall-clock ns per computation, same
N_WARM/N_TIMED protocol as Regime 1 (timing_harness.rs), applied
identically to both sides of the comparison.

Three declared tiers: SMALL (N=64), MEDIUM (N=1,024), LARGE (N=8,192).
Declared, not derived — OC-CC-1.

DECLARED HARDWARE RUN (Ryzen 5 7600X / DDR5-5600 / Windows 11 Home), 2026-08-29:

| TIER   | N     | BINARY (ns)   | RELATIONAL (ns) | REL/BIN |
|--------|-------|---------------|------------------|---------|
| SMALL  | 64    | 2,416.1       | 242.2            | 0.1002  |
| MEDIUM | 1,024 | 660,536.0     | 3,720.3          | 0.0056  |
| LARGE  | 8,192 | 42,218,566.8  | 31,357.4         | 0.0007  |

Confirmed on the declared hardware: no crossover within the current tiers
— relational was already cheaper at SMALL (REL/BIN = 0.1002), consistent
with the sandbox sanity run. This is now a DECLARED finding, not an
informational sandbox result. OC-CC-1 is confirmed open and requires
tier revision downward (e.g. N=4, 8, 16, 32) to locate the actual
crossover point on this hardware.

cargo test --release on declared hardware: 41/41 passing (2026-08-29).

Prior sandbox sanity run (2026-08-28, non-declared, retained for
comparison — see below) showed the identical qualitative pattern.

SANDBOX SANITY RUN (2026-08-28, NOT on declared hardware — informational
only, superseded by the declared run above, see OC-CC-3):

| TIER   | N     | BINARY (ns)   | RELATIONAL (ns) | REL/BIN |
|--------|-------|---------------|------------------|---------|
| SMALL  | 64    | 2,891.7       | 381.4            | 0.1319  |
| MEDIUM | 1,024 | 741,890.8     | 5,803.9          | 0.0078  |
| LARGE  | 8,192 | 47,786,852.6  | 53,948.9         | 0.0011  |

No crossover observed within the declared tiers — relational path was
already cheaper at SMALL. If this shape holds on the declared hardware,
OC-CC-1 requires tier revision downward (e.g. N=4, 8, 16) before this
domain can locate the actual crossover point, rather than only confirming
it already occurred by N=64.

### V1.0 Open Conditions Summary

- OC-PT-1, OC-PT-2, OC-PT-3 (Regime 2) — see README.md
- OC-CC-1, OC-CC-2, OC-CC-3 (Regime 3) — see README.md

### Statement on D (Regime 1) Integrity

D, M, and the V0.3 scaling/throughput results are unmodified by this
addendum. OC-IF-5 in abr-infinity-fabric continues to reference the
same measured Regime 1 figures cited above.

---

## V1.1 Addendum — Binary Algorithm Matrix (2026-08-28, sandbox — see below)

Addresses OC-CC-2 directly: V1.0 tested exactly one binary baseline
(all-pairs, O(N^2)). V1.1 declares four additional binary baselines
(binary_baselines.rs) spanning O(N) and O(N log N), and times all five
against the same relational ABR chain at the same three tiers (15 total
comparison points).

SANDBOX SANITY RUN (2026-08-28, NOT on declared hardware — informational
only, see OC-CC-3 — must be reproduced on Ryzen 5 7600X before treating
as declared):

| TIER   | BINARY_ALGO | CLASS      | BINARY (ns) | RELATIONAL (ns) | REL/BIN |
|--------|-------------|------------|-------------|------------------|---------|
| SMALL  | ALL_PAIRS   | O(N^2)     | 2,918.2     | 365.8            | 0.1254  |
| SMALL  | LINEAR_SCAN | O(N)       | 79.6        | 365.8            | 4.5948  |
| SMALL  | WINDOWED    | O(N*K)     | 372.6       | 365.8            | 0.9818  |
| SMALL  | PREFIX_SUM  | O(N)       | 77.4        | 365.8            | 4.7289  |
| SMALL  | SORT_SCAN   | O(N log N) | 152.3       | 365.8            | 2.4017  |
| MEDIUM | ALL_PAIRS   | O(N^2)     | 780,528.4   | 5,677.6          | 0.0073  |
| MEDIUM | LINEAR_SCAN | O(N)       | 763.0       | 5,677.6          | 7.4413  |
| MEDIUM | WINDOWED    | O(N*K)     | 6,121.6     | 5,677.6          | 0.9275  |
| MEDIUM | PREFIX_SUM  | O(N)       | 761.5       | 5,677.6          | 7.4554  |
| MEDIUM | SORT_SCAN   | O(N log N) | 1,673.7     | 5,677.6          | 3.3922  |
| LARGE  | ALL_PAIRS   | O(N^2)     | 48,430,694.0| 50,936.0         | 0.0011  |
| LARGE  | LINEAR_SCAN | O(N)       | 5,829.7     | 50,936.0         | 8.7373  |
| LARGE  | WINDOWED    | O(N*K)     | 47,575.7    | 50,936.0         | 1.0706  |
| LARGE  | PREFIX_SUM  | O(N)       | 6,021.8     | 50,936.0         | 8.4586  |
| LARGE  | SORT_SCAN   | O(N log N) | 14,415.8    | 50,936.0         | 3.5334  |

### Interpretation

Relational wins ONLY against ALL_PAIRS (O(N^2)) at every tier. Against
every O(N) and O(N log N) baseline, binary wins, with the margin
widening as N grows (LINEAR_SCAN and PREFIX_SUM: relational is 4.6x-8.7x
SLOWER, not faster). WINDOWED (O(N*K), K=8) sits close to parity, binary
edging ahead at SMALL/MEDIUM/LARGE (REL/BIN 0.98, 0.93, 1.07).

This is consistent with the interpretation flagged as the concern in
OC-CC-2: V1.0's "relational wins at every declared tier" finding appears
to have been a complexity-class artifact of comparing against a single,
deliberately poor-scaling binary baseline (all-pairs), not evidence of a
general relational-structure advantage on this hardware. Against binary
algorithms with ordinary linear or near-linear scaling, relational does
not currently show an advantage in this V1.1 test.

OC-CC-2 is narrowed but not closed by this addendum — five algorithms
across three complexity classes is still a declared representative set
(OC-BB-2), not an exhaustive survey.

REQUIRES DECLARED-HARDWARE CONFIRMATION before this interpretation is
admissible over D. Run on Ryzen 5 7600X and update this section with the
real figures before any Verifier pass.

---

## V1.3 Addendum — Cache-Latency Mechanism Model (2026-08-29, sandbox — see below)

Distinct in kind from V1.0-V1.2: this is NOT a binary-vs-relational
comparison. It characterizes one hardware mechanism directly — per-access
latency as a function of scrambled-access working-set size — as its own
declared mathematical object, independent of any algorithm comparison.

Reuses the ScrambledAccess permutation primitive (binary_baselines.rs,
SCRAMBLE_SEED) across 15 declared working-set sizes, doubling from 4 KB
(within L1d) to 64 MB (beyond L3). Lighter timing protocol (3 warm / 10
timed passes, OC-CL-1) than Regime 1/3's 100/1000, declared necessary
because the largest sizes cost hundreds of milliseconds per pass.

SANDBOX SANITY RUN (2026-08-29, NOT on declared hardware — informational
only, must be reproduced on Ryzen 5 7600X before treating as declared;
sandboxed/virtualized environments may show materially different cache
behavior than bare metal):

| N | WS (bytes) | MEAN (ns) | NS/ACCESS | TIER |
|---|---|---|---|---|
| 512 | 4,096 | 345.4 | 0.6746 | <= L1d |
| 4,096 | 32,768 | 2,458.3 | 0.6002 | <= L1d |
| 131,072 | 1,048,576 | 126,554.9 | 0.9655 | <= L2 |
| 262,144 | 2,097,152 | 430,119.5 | 1.6408 | <= L3 |
| 4,194,304 | 33,554,432 | 20,803,006.8 | 4.9598 | <= L3 |
| 8,388,608 | 67,108,864 | 60,179,692.9 | 7.1740 | > L3 (RAM) |

(Full 15-point table in execution_record.md.)

### Interpretation

Roughly flat through L1/L2 (0.60-0.97 ns/access), a step at the L2->L3
crossing (0.97 -> 1.64 ns/access at N=262,144, working set 2 MB),
continued rise through L3, and a further increase past L3 into RAM
(7.17 ns/access at 64 MB) — an ~11x range from smallest to largest
working set. The transition reads as a gradual ramp rather than sharp
textbook cache-tier cliffs, on this access pattern in this sandbox.

REQUIRES DECLARED-HARDWARE CONFIRMATION. Sandbox/virtualized timing may
not reflect real Ryzen 5 7600X cache behavior — see OC-CL-1 through
OC-CL-3.
