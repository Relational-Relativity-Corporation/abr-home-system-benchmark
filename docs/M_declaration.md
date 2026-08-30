# M Declaration — abr-home-system-benchmark V2.0

**Metatron Dynamics, Inc.** Bounded over D. No claim beyond D.

---

## Declared Hardware (D)

- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores / 12 threads, 4.7 GHz base / 5.3 GHz boost)
  - L1d: 32 KB/core
  - L2: 1 MB/core
  - L3: 32 MB shared
- RAM: 32 GB DDR5-5600 (2× 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit
- Compiler: Rust `cargo build --release` (rustc version as reported at build time)

All timing figures in the execution record are wall-clock measurements on
this hardware unless explicitly marked otherwise. Sandbox/virtualized
figures are structural sanity checks only and are never cited as declared.

---

## Declared Observable Mapping (M)

M maps each declared locus (graph node) to a real number in D:

  M(node i) = i / N_NODES   (gradient field, values in [0, 1))

This produces non-trivial A operator output (non-zero pairwise differences)
while remaining fully deterministic and reproducible across runs.

---

## Declared Graph Structure

Open DAG (directed acyclic graph). N_NODES = 8,192. N_EDGES = N_NODES − 1.
Each node connects to its immediate successor (chain topology).
Terminal edge has no successor — open boundary.
Ring topology is inadmissible per kernel V7.

Working set: N_NODES × 8 + N_EDGES × 8 + N_EDGES × 8 = 196,592 bytes (192 KB).
Target ≤ 1 MB (matches abr-infinity-fabric community analysis working set).

---

## Declared Operators

ABR kernel V7. Three operators in sequence per pass:

  A(x)[e] = x[source(e)] − x[target(e)]
  B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]   (immediate successor input values; not recursive)
  rho[i]  = rho_base × chi[i] / (1 + chi[i])   (node-indexed; chi[i] = max |A[e]| incident to i)
  R(g)[e] = g[e] + rho[src(e)] × (Σ_succ g[f] − Σ_pred g[p])

rho_base = 1.0 (Origin declaration for this benchmark).
Zero heap allocation per pass after buffer construction.

Grounding: operators.rs V7, Metatron Dynamics kernel (lines 890–988).

---

## Implementation Admissibility Conditions

1. Buffers (A, B, rho, R) allocated once before the warm phase. No
   allocation inside the timed loop. Confirmed by test
   `abr_buffers_zero_allocation_per_pass` (V0.2+).

2. Warm phase: 100 passes discarded before timing begins. Ensures L3
   residency for the declared working set (192 KB ≪ 32 MB L3).

3. Timed phase: 1,000 passes. Mean wall-clock time reported.

4. Exception — XLARGE and XXLARGE tiers in Regime 3 (V2.0): lighter
   protocol (3 warm / 10 timed). See OC-CC-4.

---

## Declared Scaling Graph

For scaling measurement (Regime 1) and crossover tiers (Regime 3):
`declare_scaling_graph(n)` — same open-chain topology as the benchmark
graph, parameterized by n_nodes. Node field: i / n_nodes.

---

## Declared Regime 3 Tiers (V2.0)

Tiers are grounded in the declared cache-latency curve (declared-hardware
run 2026-08-29). Boundaries are declared hardware observables, not estimates.

| Tier    | N         | WS (values) | Cache context               | NS/ACCESS (declared) |
|---------|-----------|-------------|-----------------------------|-----------------------|
| SMALL   | 64        | 512 B       | L1d flat                    | 0.56                  |
| MEDIUM  | 1,024     | 8 KB        | L1d flat                    | 0.56                  |
| LARGE   | 65,536    | 524 KB      | L2 saturation begins        | 0.70                  |
| XLARGE  | 262,144   | 2 MB        | L3 entry                    | 0.88                  |
| XXLARGE | 2,097,152 | 16 MB       | Past L3-internal step       | 3.38                  |

ALL_PAIRS excluded at LARGE, XLARGE, XXLARGE: O(N^2) is intractable
at those N. The complexity-class comparison is settled at SMALL/MEDIUM.

Rationale: V1.x tiers (max N=8,192, 64 KB) were absorbed by L2 at
0.56 ns/access — SCRAMBLED and BRANCHY showed no divergence from
LINEAR_SCAN because the cache-unfriendly and branch-heavy mechanisms
never engaged. V2.0 tiers place the experiment at and across the
declared hardware congestion thresholds so the mechanisms are tested
under conditions where they actually cost something.

---

## Declared Binary Algorithms (Regime 3)

Seven algorithms declared in binary_baselines.rs:

| Label       | Complexity   | Mechanism isolated              |
|-------------|-------------|----------------------------------|
| ALL_PAIRS   | O(N^2)      | Quadratic baseline (SMALL/MEDIUM only) |
| LINEAR_SCAN | O(N)        | Sequential access, branch-free   |
| WINDOWED    | O(N×K), K=8 | Bounded local context            |
| PREFIX_SUM  | O(N)        | Accumulation pattern             |
| SORT_SCAN   | O(N log N)  | Sort cost                        |
| SCRAMBLED   | O(N)        | Cache-unfriendly access pattern  |
| BRANCHY     | O(N)        | Branch misprediction             |

SCRAMBLED and BRANCHY have identical op counts to LINEAR_SCAN —
only the access pattern or branch predictability differs. This holds
the complexity class constant while isolating the hardware mechanism.

---

## Declared Cache-Latency Curve

Standalone characterization of per-access latency vs working-set size
on the declared hardware. 15 points, doubling from 4 KB to 64 MB.
Scrambled-access primitive (fixed seed SCRAMBLE_SEED). Lighter timing
protocol (3 warm / 10 timed) — see OC-CL-1.

Declared-hardware results (2026-08-29):
  L1d region (4–32 KB):   0.56–0.57 ns/access — flat
  L2 region (64 KB–1 MB): 0.56–0.80 ns/access — gradual rise
  L3-internal step:        0.95 → 3.38 ns/access (8 MB → 16 MB, 3.5× jump)
  RAM (>32 MB):            6.15 ns/access
  Total range: ~11×

The L3-internal step (8→16 MB) is the most pronounced single hardware
transition on this chip for this access pattern. No sharp textbook cliffs
at L1d→L2 or L2→L3 boundaries — gradual ramp instead.

---

## Open Conditions

### Regime 1

**OC-HB-1**: L3 bandwidth not directly measured. Home-system throughput
figure is wall-clock time over the declared working set; L3 bandwidth
as a binding mechanism is inferred but not isolated.

**OC-HB-3**: MI355X ratio comparison is MIXED epistemic status.
MI355X declared throughput is structural (abr-infinity-fabric); home
system is measured wall-clock. Direct instrument measurement of MI355X
required to establish correspondence.

**OC-HB-4**: Operator isolation not yet performed. NS/EDGE approximately
constant is consistent with latency-bound execution but does not identify
which operator (A, B, rho, R) is binding.

### Regime 2

**OC-PT-1**: Only activation-timestamp observable ingested (V1.0).
Full uProf hotspot sampling (idle/active utilization per process) required
to test the actual efficiency claim. example_session_trace() is a partial
hand-transcribed trace — not a full export.

**OC-PT-2**: CO_ACTIVATION_WINDOW_SECS = 2.0 is declared, not derived.
Sensitivity untested.

**OC-PT-3**: No comparison against actual OS scheduler behavior
(context-switch counts, redundant wake events) has been made.

### Regime 3

**OC-CC-1**: ADDRESSED (V2.0). Tiers now grounded in declared hardware
observables. Prior open condition (undeclared tier sizes) closed.
New open condition: whether SCRAMBLED/BRANCHY diverge from LINEAR_SCAN
at LARGE/XLARGE/XXLARGE is an empirical question answered by the V2.0
declared-hardware run (PENDING — see execution_record.md).

**OC-CC-2**: NARROWED (V1.1). Seven complexity classes tested (V1.3).
Still a declared representative set, not exhaustive.

**OC-CC-3**: CLOSED (declared-hardware run 2026-08-29, V1.3).

**OC-CC-4**: NEW (V2.0). XLARGE and XXLARGE tiers use lighter timing
protocol (3 warm / 10 timed). Elevated run-to-run variance at those
tiers — adequate for detecting order-of-magnitude mechanism engagement,
not adequate for precise ratio figures at the same confidence as lower tiers.

### Cache-Latency Curve

**OC-CL-1**: CLOSED (declared-hardware run 2026-08-29).
**OC-CL-2**: CLOSED (L3-internal step located: 8→16 MB transition).
**OC-CL-3**: CLOSED (absence of sharp L1d→L2 cliff is an admissible
  finding on this hardware for this access pattern).

---

## Version History

- V0.1–V0.3: Regime 1 development. Operators corrected to V7.
- V1.0: Three-regime expansion. Declared-hardware baseline established.
- V1.1: Binary algorithm matrix (5 algorithms). OC-CC-2 narrowed.
- V1.2/V1.3: Mechanism-isolating baselines. Cache-latency module.
  Declared-hardware runs obtained. OC-CC-3, OC-CL-1/2/3 closed.
- V2.0: Regime 3 tiers grounded in cache-latency curve observables.
  OC-CC-1 addressed. 65 tests passing (sandbox 2026-08-29).
  V2.0 declared-hardware run PENDING.

- V3.0: Regime 4 added — transition gradient sweep (25 N values, 512 KB–32 MB).
  Two transition surfaces located: BRANCHY (sharp onset 512 KB–1.3 MB, plateau
  at 4.0–4.6×); SCRAMBLED (gradual onset, accelerates at 16 MB, still rising
  at 32 MB). 76 tests passing. Declared-hardware run 2026-08-29.
- V4.0: Hardware counter instrumentation declaration (uProf). Declared
  measurement points, H variable set, normalization forms, and predictions
  derived from V3.0 gradient. uProf data ingested manually.
  Findings: BRANCHY mechanism confirmed as branch misprediction (%BR_MISP
  3.1%→27.8%, DRAM=0). SCRAMBLED mechanism confirmed as DRAM pressure from
  L3 overflow (DRAM_PTI 0.021→36.2, L3_PTI falls). Two distinct hardware
  states observed under scrambled access: State 1 (CPI<linear, L3 fills
  dominant, W=4.1 MB) and State 2 (CPI>linear, DRAM dominant, W=32 MB).
  D (dependency relation) promoted to first-class variable in R alongside
  A and B. OC-HW-4 and OC-HW-5 declared — pointer-chain intervention
  at fixed (A, W) to test latency-hiding interpretation of State 1.

---

## V4.0 — Hardware Counter Instrumentation Declaration

### Purpose

V1.0–V3.0 establish wall-clock timing observables: O_S(N) = O_L(N) by
construction while T_S(N) ∝̸ T_L(N) and T_B(N) ∝̸ T_L(N) after declared
working-set transitions. What accounts for the timing difference is not
established by wall-clock measurement alone (OC-HW-1).

V4.0 opens the hardware state. AMD uProf hardware counter sampling is
applied to LINEAR, SCRAMBLED, and BRANCHY at declared measurement points
to populate the hardware variable set H and establish the joint
observable state:

  R → H → O

where R is the declared computational state, H is the hardware state
observed by uProf, and O is the timing outcome already declared in V3.0.
The arrows denote observed progression, not asserted causation.
Causation requires intervention — that is the declared next pass after
this one.

### Declared Variable Sets

#### R — Computational variables (fully specified by benchmark code)

These are declared independently of uProf. They describe what is given
to the processor at each measurement point.

  N       — element count (declared per measurement point below)
  W       — working-set bytes = N × 8
  A       — access-order relation: LINEAR (sequential i=0..N-1) vs
             SCRAMBLED (fixed pseudo-random permutation, SCRAMBLE_SEED)
  B       — branch-outcome relation: LINEAR/SCRAMBLED (branch-free) vs
             BRANCHY (fixed pseudo-random 50/50 data-dependent sequence,
             BRANCH_SEED)
  D       — dependency relation: INDEPENDENT (each address computable
             without waiting for prior result, a_{t+1} = P(t+1)) vs
             CHAINED (each address depends on the value returned at the
             prior address, a_{t+1} = f(x_{a_t})). See OC-HW-5.
  O_count — declared operation count = N-1 for all three algorithms

V4.0 rationale for D as a first-class variable:

The V4.0 uProf measurement found that at N=524,288 (4.1 MB), SCRAMBLED
CPI = 0.394 — below LINEAR CPI = 0.800 — while at N=4,194,304 (32 MB),
SCRAMBLED CPI = 1.800, above LINEAR. Two executions can have identical
N, W, A, O_count, and data, yet present radically different opportunity
for concurrent execution solely because their dependency relations differ.

D_independent permits multiple load addresses to be resolved
concurrently. D_chained serializes them: the next address does not
exist as an actionable quantity until the preceding load completes.
This is a property of the computation presented to the hardware — not
a hardware counter. It is independently declared before any measurement.

A, B, and D are declared as separate dimensions of R because:
  - A and B produce distinct transition surfaces at different W (V3.0)
  - D controls available memory-level concurrency independently of A and W
  Collapsing any two of these into a single variable would erase
  structural findings already in the record.

#### H — Hardware state variables (from uProf counter sampling)

Capture the following at each declared measurement point. Use whatever
event names uProf exposes for this Zen 4 CPU — do not substitute
Intel-style generic names if they differ.

Primary counters:
  CYC       — total CPU cycles
  INS       — instructions retired
  IPC       — instructions per cycle (= INS / CYC)
  BR        — branch instructions
  BR_M      — branch mispredictions
  L1_M      — L1 data cache misses
  L2_M      — L2 cache misses
  L3_M      — L3 cache misses
  DTLB_M    — data TLB misses
  STALL_F   — frontend stall cycles
  STALL_B   — backend stall cycles
  MEM_BW    — memory bandwidth (bytes/second, if available)

Derived normalized forms (compute from raw counts after collection):
  BR_M / BR       — misprediction rate per branch
  BR_M / N        — mispredictions per element
  L2_M / N        — L2 misses per element
  L3_M / N        — L3 misses per element
  DTLB_M / N      — TLB misses per element
  INS / N         — instructions retired per element
  CYC / N         — cycles per element
  STALL_B / CYC   — fraction of cycles stalled in backend

Both raw counts and normalized rates are declared as distinct
observables. BR_M and BR_M/BR tell different things. Preserve both.

INS/N is a critical control. It establishes whether O_S(N) = O_L(N)
at the algorithmic level corresponds to INS_S/N ≈ INS_L/N at the
machine level. If instructions retired per element are approximately
equal while cycles per element diverge, the processor is not doing
more work — it is taking longer to do the same work. The question
then becomes: where did those cycles go? The remaining H variables
answer that without requiring a prior assumption about which mechanism
is responsible.

#### O — Outcome variables (already declared, from V3.0)

  T        — mean wall-clock time per pass (ns)
  T/N      — mean ns per operation
  S/L      — T_SCRAMBLED / T_LINEAR at this N
  B/L      — T_BRANCHY / T_LINEAR at this N

These are already declared in the execution record. uProf figures
are NOT wall-clock comparable (instrumentation adds overhead) and
must not be mixed with benchmark timing. Record them separately.

### Declared Measurement Points

Six points: three per mechanism (pre-transition, onset, post-transition),
selected from the V3.0 gradient declared-hardware run.

#### BRANCHY measurement points

| Point | N       | W       | B/L (V3.0) | Region           |
|-------|---------|---------|------------|------------------|
| B-pre | 65,536  | 512 KB  | 0.972      | Pre-transition   |
| B-on  | 122,880 | 960 KB  | 3.251      | Onset            |
| B-post| 524,288 | 4.1 MB  | 4.404      | Plateau (post)   |

B-pre: B/L ≈ 1.0 — branch mechanism not yet engaged.
B-on:  B/L = 3.25 — transition in progress, steep gradient.
B-post: B/L = 4.40, stable — mechanism fully engaged, plateau confirmed.

Declared prediction for BRANCHY (falsifiable):
  BR_M/BR rises from B-pre to B-on, then stabilizes at B-post,
  tracking the B/L shape. If BR_M/BR plateaus at B-on while B/L
  plateaus at B-post, that is itself a declared finding — the
  predictor saturates one step before the cost ratio does.
  If BR_M/BR does not track B/L, branch misprediction is not the
  primary mechanism and the responsible variable remains in H.

#### SCRAMBLED measurement points

| Point | N         | W      | S/L (V3.0) | Region           |
|-------|-----------|--------|------------|------------------|
| S-pre | 524,288   | 4.1 MB | 1.319      | Pre-transition   |
| S-on  | 2,097,152 | 16 MB  | 3.021      | Onset            |
| S-post| 4,194,304 | 32 MB  | 8.330      | Post (rising)    |

S-pre:  S/L = 1.32 — access mechanism not yet dominant.
S-on:   S/L = 3.02 — transition onset, S/L crossing 3.0.
S-post: S/L = 8.33, still rising — mechanism fully engaged, no plateau.

Note: S-on and S-post are in the light-protocol region (OC-TG-2).
The S/L values carry elevated variance. uProf counter readings at
these points carry the same caveat — they are indicative, not
as precise as the standard-protocol BRANCHY points.

Declared prediction for SCRAMBLED (falsifiable):
  L3_M/N rises from S-pre to S-on to S-post, tracking S/L.
  If L3_M/N is approximately flat while S/L rises, L3 misses
  are not the primary mechanism — watch DTLB_M/N and STALL_B/CYC.
  If multiple H variables rise together, preserve all of them.
  Do not assign a single mechanism before seeing the counter data.

### Declared uProf Protocol

1. Run uProf in hardware counter sampling mode against benchmark.exe,
   targeting the timed inner loop (not warm phase, not buffer setup).
   Declare which uProf sampling mode is used and at what sampling rate.

2. Run each of the six measurement points (B-pre, B-on, B-post,
   S-pre, S-on, S-post) as separate uProf sessions. Each session
   runs LINEAR, SCRAMBLED, and BRANCHY at the declared N so that
   all three can be compared at the same hardware state.

3. Record raw counter values and compute normalized rates.
   Record the uProf version and event names used — Zen 4 event
   names may differ from generic documentation.

4. Do not collapse H variables before recording. Preserve the full
   vector H at each measurement point. Reduction to a summary
   statistic is a downstream analysis step, not a collection step.

5. Enter results in execution_record.md under
   "V4.0 — uProf Declared Hardware Run" with the same structure
   as prior declared runs: raw output first, findings second.

### Open Conditions

**OC-HW-1**: PARTIALLY ADDRESSED. This declaration specifies what to
  instrument and what to look for. It remains open until the uProf
  run is completed and H is populated at the declared measurement
  points. A declared prediction is now on record for each mechanism —
  both are falsifiable by the counter data.

**OC-HW-2**: NEW. uProf instrumentation adds overhead. Counter-mode
  timing figures are not comparable to benchmark wall-clock timing.
  The two must be recorded separately and never mixed. The declared
  outcome variables O come from the benchmark; H comes from uProf.

**OC-HW-3**: NEW. Zen 4 hardware counter event names must be confirmed
  in uProf before the run. Generic counter names (Intel-style) may
  not match AMD event names on this CPU. Declare the actual event
  names used in the execution record entry.

**OC-TG-1**: OPEN. Finer sweep around BRANCHY onset (512–960 KB) and
  SCRAMBLED onset (14–18 MB) would tighten transition location.
  Lower priority than OC-HW-1 — counter data may reveal mechanism
  directly, making a finer timing sweep redundant.

**OC-TG-2**: OPEN. Light-protocol figures at SCRAMBLED measurement
  points (S-on, S-post) carry elevated variance. Standard-protocol
  confirmation at those N values would reduce uncertainty in S/L
  figures used as the baseline for uProf comparison.

---

## V5.0 — Available Relational Progression (declared mathematical object)

### Motivation

The OC-HW-5 intervention (2026-08-29) established that varying D alone
at fixed (N, W, A, O_count) produces a 10.25× CPI difference at
identical memory pressure. This result is not explainable by operation
count, working-set size, access distribution, or cache miss rate.
It requires a declared description of how much of the computation is
available for execution at each step — independent of the hardware
mechanism that exploits or fails to exploit that availability.

### Declared object: A_t — available relational progression

At each step t in an execution, define the set of operations whose
declared predecessor relations are already resolved:

  A_t = { o_j : all declared predecessors required by o_j
                 are resolved at step t }

A_t is not reduced to a scalar. It is preserved as a set — its
cardinality, structure, and change over time are all potentially
informative.

For D_independent (a_{t+1} = P(t+1)):
  Future addresses a_{t+1}, a_{t+2}, ... are computable without
  waiting for any prior result. A_t contains many operations
  simultaneously — the progression frontier is wide.

For D_chained (a_{t+1} = f(x_{a_t})):
  a_{t+1} is not a member of A_t until x_{a_t} is returned.
  The progression frontier is narrow — typically one operation.

The V5.0 result establishes that |A_t| — the width of the available
progression frontier — has measurable, isolated hardware consequence
(10.25× CPI) independent of conventional operation count.

### Declared relation

  R_t → A_t → H_t → O_t

where:
  R_t  — declared computational state at step t (N, W, A, B, D, O_count)
  A_t  — available relational progression (set of operations whose
          declared predecessors are resolved at t)
  H_t  — hardware state (CPI, %BR_MISP, DRAM_PTI, L3_PTI, ...)
  O_t  — timing outcome (ns/op, S/L, B/L)

The arrows denote observed progression within declared domain D.
They do not assert causation beyond D. Each arrow is a declared
relation, not a mechanism claim.

A_t is placed between R_t and H_t because it is a property of the
declared computation — derivable from R without hardware measurement —
that mediates the effect of R on H. The V5.0 intervention manipulates
A_t (by changing D) while holding R otherwise constant, and observes
the resulting change in H. This is the experimental basis for placing
A_t in the declared chain.

### Open condition OC-RP-1 (new)

OC-RP-1: A_t is declared as a set. Its cardinality |A_t|, rate of
  change, and distribution over time are not yet measured directly.
  The V5.0 result establishes that varying |A_t| (wide vs narrow
  progression frontier) has isolated hardware effect. It does not
  yet characterize the functional relation between |A_t| and H_t
  across intermediate values — only the two extreme cases
  (D_independent: wide; D_chained: narrow) have been declared.
  Intermediate dependency structures (partial chains, branching
  dependency graphs) are the declared next experimental space.

---

## V7.0 — ABR A Operator: Declared Experimental Graph and Contrast Field

### Purpose

The experimental graph maps declared measurement points to a relational
structure. Each node is a fully declared (N, W, A, B, D) configuration
with measured H = (CPI, DRAM_PTI, L3_PTI, L2_PTI, %BR_MISP). Edges
connect configurations differing in exactly one declared variable.

The A operator computes contrast across each edge:
  A(H)[e] = H(target(e)) − H(source(e))

This produces a contrast vector at each edge — not a scalar — preserving
the full H structure. Non-uniform contrast across edges in the same
dimension reveals interaction effects: where the effect of one variable
depends on the state of another.

### Declared Nodes (V7.0)

| Label      | N       | W (MB) | A   | B       | D          | CPI   |
|------------|---------|--------|-----|---------|------------|-------|
| lin_pre    | 524,288 | 4.1    | seq | none    | ind        | 0.800 |
| scr_pre    | 524,288 | 4.1    | scr | none    | ind        | 0.394 |
| scr_on     | 2,097,152 | 16.0 | scr | none    | ind        | 1.178 |
| scr_post   | 4,194,304 | 32.0 | scr | none    | ind        | 1.800 |
| brn_pre    | 65,536  | 0.5    | seq | branchy | ind        | 0.558 |
| brn_on     | 122,880 | 1.0    | seq | branchy | ind        | 1.422 |
| brn_post   | 524,288 | 4.1    | seq | branchy | ind        | 1.663 |
| d1         | 524,288 | 4.1    | scr | none    | chain-1    | 3.993 |
| d2..d64    | 524,288 | 4.1    | scr | none    | chain-k    | ~1.00 |
| d8_ind     | 524,288 | 4.1    | scr | none    | chain-8-nd | 1.009 |

### Declared Edges and A Operator Values (ΔCPI)

A dimension (access order):
  lin_pre → scr_pre:  ΔCPI = −0.406  (A=seq→scr, W=4.1 MB, D=ind)

B dimension (branch):
  lin_pre_s → brn_pre: ΔCPI = −0.257  (B=none→branchy, W=0.5 MB, D=ind)

W dimension (working set):
  scr_pre → scr_on:   ΔCPI = +0.784  (A=scr, B=none, D=ind)
  scr_on → scr_post:  ΔCPI = +0.622  (A=scr, B=none, D=ind)
  brn_pre → brn_on:   ΔCPI = +0.864  (A=seq, B=branchy, D=ind)
  brn_on → brn_post:  ΔCPI = +0.241  (A=seq, B=branchy, D=ind)

D dimension (dependency structure):
  scr_pre → d1:    ΔCPI = +3.599  (ind → chain-1)
  d1 → d2:         ΔCPI = −2.991  (chain-1 → chain-2, large)
  d2 → d4:         ΔCPI = +0.004  (chain-2 → chain-4, flat)
  d4..d64:         ΔCPI ≈ 0.000   (flat throughout)
  d8 → d8_ind:     ΔCPI = +0.009  (chain-8 → chain-8-no-dep, noise)
  d8_ind → scr_pre: ΔCPI = −0.615 (chain-8-no-dep → full-ind, large)

### Declared Relational Structure

The A operator field is non-uniform — the contrast along D is not
constant. The D dimension has a single large contrast at the
chain-1→chain-2 edge (ΔCPI = −2.991), then near-zero contrast
throughout chain-2 through chain-64, then a large contrast at the
chain-8-nd→ind edge (ΔCPI = −0.615). This non-uniformity is a declared
relational property of the experimental graph — not a scalar threshold.

### Declared Gaps in the Graph (Undeclared Interactions)

The following interaction edges are not yet declared:

  D × W: No edge varies D at W=16 MB or W=32 MB. The D dimension
    has been mapped only at W=4.1 MB. Whether the D contrast field
    changes at larger W is undeclared.

  A × D: No edge varies A while holding D=chain-k. The A dimension
    has been mapped only at D=ind. Whether the A contrast changes
    under chained access is undeclared.

  B × W (large W): B confirmed at W=4.1 MB by uProf. Whether the
    B contrast field changes at W=16 MB or W=32 MB is undeclared
    by hardware counter measurement (timing only from gradient sweep).

  D (segmentation) × W: OC-RP-3 addresses this — whether the
    D_k/D_independent contrast tracks segment length L or W.

These gaps define the declared next experimental space. Each undeclared
edge represents a potential interaction that cannot be attributed or
eliminated without measurement.

### B Operator Application (declared, not yet computed)

The B operator propagates contrast relations through the graph:
  B(A(H))[e] = A(H)[e] + Σ_{f ∈ succ(e)} A(H)[f]

Applied to the experimental graph, B would show which edges carry
cumulative contrast from their successors — where upstream contrast
propagates into downstream measurements. This is the declared next
computation once the graph is more fully populated. Premature
application with a sparse graph risks propagating noise rather
than signal.

### R Operator (declared, not yet computed)

The R operator applies a relational field weighted by node connectivity:
  R(A(H))[e] = A(H)[e] + ρ[src(e)] × (Σ_succ − Σ_pred)

In the experimental graph, ρ[node] reflects how many declared edges
connect to that node — its relational centrality. Nodes with many
declared edges (e.g., scr_pre, d8) carry more weight than isolated
nodes. This is the declared next operator once B is computed.

### Provenance

All node H values are declared-hardware measurements on Ryzen 5 7600X
unless explicitly marked as sandbox. The A operator values are derived
from declared measurements — they inherit the same provenance bounds.
No claim is made beyond D.
