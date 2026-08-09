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
