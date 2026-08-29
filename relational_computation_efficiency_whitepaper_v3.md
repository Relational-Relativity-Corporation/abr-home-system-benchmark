# Relational Organization of Computation as a Hardware Efficiency Variable

**Metatron Dynamics, Inc.**
Robin Macomber, Founder
August 2026

---

## Executive Summary

A series of controlled hardware experiments on an AMD Ryzen 5 7600X processor demonstrates that the *organization* of computation — how operations relate to each other in dependency and access structure — can produce an order-of-magnitude difference in processor efficiency within the declared experimental domain, independently of operation count. The same number of operations, on the same data, in the same amount of memory, can cost 0.39 CPU cycles per instruction or 3.99 CPU cycles per instruction depending solely on the dependency topology presented to the hardware.

This finding has a direct implication for infrastructure sizing. Operation count alone does not determine hardware requirement. If a significant fraction of measured computational demand reflects organizational structure that relational mathematics could reorganize, then the capacity requirement is not fixed — it is a function of how the computation is declared.

The experiments described here establish the measurement framework, the hardware evidence, and the declared path to that claim. They do not yet prove it for production workloads. They establish that the question is experimentally tractable and that the answer, within the declared domain, points in one direction.

---

## 1. Operation Count Does Not Specify Computational Demand

Datacenter capacity planning uses throughput as the unit of demand: tokens per second, queries per second, training steps per hour. That throughput translates to hardware requirements through efficiency assumptions — how many operations the hardware performs per unit time at some target utilization.

This framing contains a consequential gap: operation count alone does not determine hardware requirement. The experiments described here establish that within a controlled hardware domain, the same operation count can produce order-of-magnitude differences in hardware cost depending on the relational structure of the computation. If that variation persists in production workloads, measured throughput demand does not uniquely determine hardware requirement — it determines hardware requirement *given the current organization of the computation*.

---

## 2. The Experimental Framework

### 2.1 Declared Variables

The experiments operate over a declared computational state with five independently manipulable dimensions:

**N** — element count (scale of the computation)
**W** — working-set size in bytes (memory footprint)
**A** — access-order relation: sequential versus scrambled pseudo-random order
**B** — branch-outcome relation: predictable versus data-dependent unpredictable
**D** — dependency relation: independent (each address computable without waiting) versus chained (each address depends on the value returned by the prior step)

The key constraint: **O_count** — the declared operation count — is held equal across all variants of A, B, and D. The experiments compare computations that perform the same number of operations on the same data in the same memory footprint, differing only in relational organization.

### 2.2 Hardware Measurement

All experiments run on a declared machine: AMD Ryzen 5 7600X (Zen 4), 32 GB DDR5-5600, 32 MB L3 cache. Results are wall-clock timing measurements confirmed across multiple independent runs, supplemented by AMD uProf hardware performance counter data (version 5.3.521.0): cycles retired, instructions retired, branch mispredictions, and L2/L3/DRAM demand fill rates (Zen 4 event names PMCx076, PMCx0C0, PMCx0C2, PMCx0C3, PMCx043, PMCx060).

Hardware counter data separates *what the processor is doing* from *how long it takes*, allowing attribution of timing differences to specific hardware mechanisms rather than leaving them as unexplained performance gaps.

---

## 3. Three Independently Measured Efficiency Gaps

### 3.1 Access Order (A): up to 8× cost increase at large working sets

A sequential scan (LINEAR) and a scrambled scan (SCRAMBLED) perform identical operations on the same N values in the same W bytes. Only access order differs.

At small working sets, cost is essentially equal. As working set grows past L3 capacity:

| Working set | SCRAMBLED / LINEAR cost ratio |
|-------------|-------------------------------|
| 512 KB      | 1.05× (essentially equal)     |
| 2 MB        | 1.28×                         |
| 16 MB       | 3.02×                         |
| 32 MB       | 8.33× (still rising)          |

Hardware counters identify the mechanism: at 4.1 MB working set, DRAM fills are 0.021 per thousand instructions — L3 cache absorbs scrambled access. At 32 MB, DRAM fills rise to 36.2 per thousand instructions as the working set exceeds L3 capacity and random accesses must be served from main memory.

Notably, at 4.1 MB, scrambled access produces a *lower* CPI (0.394) than sequential access (0.800). The hardware's out-of-order execution engine services multiple independent cache requests concurrently, achieving higher instruction throughput than sequential access can. The cost emerges only when working set grows large enough that main memory latency can no longer be hidden by concurrent outstanding requests.

### 3.2 Branch Configuration (B): 4× cost increase with a hard ceiling

A branch-free scan (LINEAR) and a data-dependent branch scan (BRANCHY) perform identical operations. Only branch predictability differs.

At small working sets, cost is essentially equal. The transition occurs rapidly between 512 KB and 1.3 MB:

| Working set | BRANCHY / LINEAR cost ratio |
|-------------|-----------------------------|
| 512 KB      | 0.97× (slightly faster)     |
| 960 KB      | 3.25× (transition)          |
| 4.1 MB      | 4.40× (plateau)             |
| 32 MB       | 4.30× (plateau, unchanged)  |

Hardware counters confirm: branch misprediction rate rises from 3.1% at 512 KB to 22.8% at 960 KB, then plateaus at 27.8% through 32 MB. The observed cost reaches a plateau within the declared measurement range, corresponding to the measured plateau in branch-misprediction rate.

BRANCHY reaches a ceiling. SCRAMBLED does not. These are qualitatively different transition shapes, confirming that A and B are distinct computational dimensions that cannot be collapsed into a single measure.

### 3.3 Dependency Structure (D): 10× cost difference at identical memory pressure

This is the strongest finding. A scrambled scan (D_independent) and a pointer-chained scan (D_chained) are compared at identical N, W, A, and O_count. The access distribution is the same scrambled permutation in both cases. Only the dependency structure differs:

D_independent: a_{t+1} = P(t+1) — next address computable without waiting for any prior result
D_chained:     a_{t+1} = f(x_{a_t}) — next address requires the value returned by the prior step

At N=524,288, W=4.1 MB:

| Variable     | D_independent | D_chained | Ratio  |
|--------------|---------------|-----------|--------|
| CPI          | 0.390         | 3.993     | 10.25× |
| Branch miss  | 0.10%         | 0.18%     | ~equal |
| DRAM fills   | 0.020 PTI     | 0.020 PTI | identical |
| L3 fills     | 116 PTI       | 148 PTI   | 1.28×  |

DRAM pressure is identical. L3 fill rates are comparable — the chained case actually makes *more* L3 requests. Branch misprediction is negligible in both. The 10.25× CPI difference is not a memory effect, not a branch effect, and not an operation-count effect.

The dependency chain prevents each next dependent address from being resolved until the preceding load completes, sharply reducing the available progression exposed by this workload. In D_independent, the same L3 fills are served at CPI=0.390 because multiple independent requests can be outstanding simultaneously. The wall-clock timing confirms: 1.16 ns/op (D_independent) versus 12.87 ns/op (D_chained) — an 11.1× ratio at identical working set and operation count.

---

## 4. The Mathematical Object: Available Relational Progression

These three findings converge on a declared mathematical object that is not operation count.

At each step t in an execution, define:

**A_t = { o_j : all declared predecessors required by o_j are resolved at step t }**

A_t is the set of operations available for execution at step t — whose inputs are already known. Its cardinality |A_t| is the width of the available progression frontier.

For D_independent, |A_t| is wide — many future addresses are computable in advance, with no dependency on prior results. For D_chained, |A_t| is narrow — the next address cannot be resolved until the prior load completes.

V5 establishes dependency topology D as an independently manipulable computational variable with order-of-magnitude hardware consequence in the declared experiment. Available relational progression A_t, and particularly its width |A_t|, is the declared candidate structure through which intermediate dependency topologies will now be measured. The V5 intervention compares two endpoint structures — fully independent and fully chained. Whether |A_t| specifically, rather than some correlated property of the dependency graph, is the operative variable is the subject of the next declared experiment.

The declared relation is:

**R_t → A_t → H_t → O_t**

where R_t is the full computational state, A_t is the available progression derived from R_t, H_t is the observed hardware state, and O_t is the timing outcome. Each arrow is an observed progression within the declared hardware domain. A_t is placed in the chain because it is derivable from R_t without hardware measurement — it is a property of the declared computation that mediates R_t's effect on H_t.

---

## 5. The Datacenter Implication

### 5.1 What the experiments establish

Operation count alone does not determine hardware requirement. Within the declared domain, the same operation count at the same working set produces a 10× difference in CPI depending solely on dependency structure. That difference is not attributable to doing more work or accessing more memory — it is attributable to how the computation is organized.

The mechanism is identified: the dependency chain prevents concurrent resolution of successive dependent addresses, reducing available relational progression. The same L3 fills that cost 0.390 CPI in D_independent cost 3.993 CPI in D_chained because in the latter case each must complete before the next can begin.

### 5.2 The correspondence yet to be established

The experiments do not yet establish that production workloads — transformer inference, large-scale training, database operations — carry the kind of serialized dependency structure demonstrated in the controlled benchmark. That correspondence requires declared work connecting the synthetic result to real workload classes. It is the most important near-term open item.

What the experiments do establish is that the question is experimentally tractable. The measurement framework is declared, hardware-confirmed, and reproducible. The same uProf instrumentation protocol applied to a real workload would produce comparable H observations, allowing direct comparison between workload dependency topology and the benchmark baselines.

### 5.3 The relational mathematics connection

The ABR/ABRCE relational mathematics framework (Metatron Dynamics) operates over declared relations between loci, not over sequences of dependent scalar operations. Its operators — A (contrast), B (propagation), R (relational field) — produce output over all declared relations simultaneously. The natural computational topology of the framework is wide A_t rather than chained.

The chip substrate benchmark was designed to test whether that architectural difference has measurable hardware consequence. The answer within the declared domain is yes, by an order of magnitude, on commodity hardware not designed with relational mathematics in mind. This motivates a testable hardware-software co-design hypothesis: processors or accelerators designed around wide A_t — with deeper outstanding-request capacity, memory systems optimized for concurrent independent access, and scheduling aware of relational dependency topology — may recover additional efficiency beyond what software reorganization alone achieves.

### 5.4 The infrastructure sizing argument

If production workloads carry significant unnecessary dependency serialization — computation organized as chains where relational reorganization could expose wider A_t — then measured computational demand overstates fundamental requirement by whatever fraction of the measured cost is organizational overhead rather than necessary computation.

The experiments cannot yet quantify that fraction. But they establish that:

- The efficiency gap is real, large (10×), and hardware-confirmed on declared commodity equipment.
- The measured difference is organizational within the declared experiment: the same operations on the same data in the same memory produce approximately 10× higher instruction throughput when the tested dependency topology is independent rather than chained.
- The mathematical framework to describe and reorganize that topology is documented (V7 kernel, ABR operators, Metatron Dynamics technical record).
- The measurement protocol to test the claim on real workloads is declared and operational.

The hypothesis — that relational reorganization of production workloads can recover sufficient efficiency to reduce infrastructure scale requirements — is falsifiable, hardware-grounded, and declared on specific equipment with specific measurement protocols. It is not yet confirmed. It is testable.

---

## 6. Declared Next Steps

**Near term — intermediate dependency structures (V6):**
Map the functional relation between |A_t| and H_t across intermediate dependency depths — partial chains, branching dependency graphs, variable chain depth. If H_t changes systematically with declared changes in A_t across intermediate structures, the mathematical object introduced here moves from an interpretation of V5 to an independently mapped experimental variable. This is the most important next experiment.

**Near term — correspondence to real workloads:**
Apply the declared measurement framework to a real production workload. Transformer inference is the most direct target given existing abr-relational-attention work. Measure A_t width at declared points in the inference computation and compare to benchmark baselines.

**Medium term — hardware co-design hypothesis:**
Design and test accelerator configurations with explicit support for wide A_t — deeper outstanding-request buffers, relational dependency-aware scheduling. Measure whether hardware co-design recovers additional efficiency beyond software reorganization alone.

---

## 7. Summary of Declared Results

All results are bounded to the declared hardware domain: AMD Ryzen 5 7600X / DDR5-5600 / Windows 11. No claim is made beyond this domain.

| Finding | Declared result | Hardware confirmation |
|---------|----------------|----------------------|
| A (access order) produces up to 8.3× cost increase at 32 MB | Wall-clock, gradient sweep | DRAM_PTI rises 1,723× as WS exceeds L3 |
| B (branch config) produces 4.4× cost increase with hard ceiling | Wall-clock, gradient sweep | %BR_MISP rises 3.1%→27.8%, plateaus |
| D (dependency) produces 10.25× CPI difference at identical memory pressure | uProf CPI, controlled intervention | DRAM_PTI identical; L3_PTI comparable |
| Two distinct hardware states under scrambled access | uProf CPI, 3 WS sizes | CPI below linear at 4.1 MB; above at 32 MB |
| D intervention is clean: N, W, A, O_count all held constant | Controlled experiment | No confounding in H variables |

Operation count does not specify computational demand. The observed computational cost is relationally conditioned:

**(O_count, N, W, A, B, D) → H → O**

V5 provides interventional evidence for D. The mathematical object A_t provides the declared structure for investigating intermediate dependency topologies in V6.

---

*Metatron Dynamics, Inc. · Lompoc, California · relationalrelativity.dev*
*Bounded over the declared hardware domain. No claim is made beyond D.*
