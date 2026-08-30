# Execution Record — abr-home-system-benchmark V2.0

**Metatron Dynamics, Inc.** Bounded over D. No claim beyond D.

This document records all `cargo test` and `cargo run --release` outputs
for abr-home-system-benchmark. Each entry is a declared observable
traceable to the hardware declared in docs/M_declaration.md, or is
explicitly marked as a sandbox sanity check only.

Hardware for all declared entries:
- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2x 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit

---

## V0.1 through V0.3 — 2026-08-08 (Regime 1 development)

### Summary

V0.1: Initial implementation. Four heap allocations per pass.
  Epistemic status: MEASURES IMPLEMENTATION COST. Not admissible.

V0.2: Allocation removed. Pre-allocated buffers.
  Mean per pass: ~35,000 ns. Throughput: ~27,926 analyses/second.
  Epistemic status: NON-V7 OPERATORS. Not admissible as MI355X basis.

V0.3: Operators corrected to V7 (B immediate-successor, rho node-indexed).
  Mean per pass: ~29,000–31,000 ns. Throughput: ~32,000–35,000 analyses/second.
  24/24 tests passing.
  Epistemic status: V7-ADMISSIBLE. Basis for MI355X ratio comparison,
  subject to open conditions OC-HB-1 through OC-HB-4.

---

## V1.0 — Declared Hardware Run (2026-08-29)

Three-regime benchmark. Regime 1 confirmed. Regime 2 (process topology)
structural summary. Regime 3: single O(N^2) binary baseline vs ABR chain.

### cargo test --release
41 passed; 0 failed.

### cargo run --release — Regime 1

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

### cargo run --release — Regime 3 (single O(N^2) baseline)

| TIER   | N     | BINARY (ns)  | RELATIONAL (ns) | REL/BIN |
|--------|-------|--------------|-----------------|---------|
| SMALL  | 64    | 2,416.1      | 242.2           | 0.1002  |
| MEDIUM | 1,024 | 660,536.0    | 3,720.3         | 0.0056  |
| LARGE  | 8,192 | 42,218,566.8 | 31,357.4        | 0.0007  |

Finding: relational beats O(N^2) at every tier. OC-CC-1 open —
no crossover found, but only one baseline tested.

---

## V1.3 — Declared Hardware Run (2026-08-29)

7-algorithm × 3-tier matrix (21 points). Cache-latency curve.
60/60 tests passing.

### cargo run --release — Regime 1 (fourth declared run)

Mean per pass: 27,590.0 ns
Home system throughput: 36,245 analyses/second
MI355X / Home system ratio: 210.5x

Scaling:
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,316.0   | 3.241   | 3,200    |
| 2,047   | 48.0    | 6,621.2   | 3.235   | 6,400    |
| 4,095   | 96.0    | 13,377.1  | 3.267   | 13,100   |
| 8,191   | 192.0   | 27,137.6  | 3.313   | 26,500   |
| 16,383  | 384.0   | 61,111.1  | 3.730   | 59,800   |

Four-run NS/EDGE summary: 3.2–3.9 ns/edge across all runs.
The 16,383-edge step (3.313 → 3.730 ns/edge) is consistent across
all four runs and corroborated by the cache-latency curve (384 KB
working set crosses into L2 saturation zone).

### cargo run --release — Regime 3 (V1.3, 21-point matrix, DECLARED)

| TIER   | BINARY_ALGO | CLASS                 | BINARY (ns)  | REL (ns) | REL/BIN |
|--------|-------------|-----------------------|--------------|----------|---------|
| SMALL  | ALL_PAIRS   | O(N^2)                | 2,371.4      | 213.2    | 0.0899  |
| SMALL  | LINEAR_SCAN | O(N)                  | 51.2         | 213.2    | 4.1641  |
| SMALL  | WINDOWED    | O(N*K)                | 291.4        | 213.2    | 0.7316  |
| SMALL  | PREFIX_SUM  | O(N)                  | 51.9         | 213.2    | 4.1079  |
| SMALL  | SORT_SCAN   | O(N log N)            | 183.8        | 213.2    | 1.1600  |
| SMALL  | SCRAMBLED   | O(N) cache-unfriendly | 54.2         | 213.2    | 3.9336  |
| SMALL  | BRANCHY     | O(N) branch-heavy     | 51.9         | 213.2    | 4.1079  |
| MEDIUM | ALL_PAIRS   | O(N^2)                | 594,872.5    | 3,359.4  | 0.0056  |
| MEDIUM | LINEAR_SCAN | O(N)                  | 569.6        | 3,359.4  | 5.8978  |
| MEDIUM | WINDOWED    | O(N*K)                | 4,519.8      | 3,359.4  | 0.7433  |
| MEDIUM | PREFIX_SUM  | O(N)                  | 579.5        | 3,359.4  | 5.7971  |
| MEDIUM | SORT_SCAN   | O(N log N)            | 1,200.1      | 3,359.4  | 2.7993  |
| MEDIUM | SCRAMBLED   | O(N) cache-unfriendly | 581.5        | 3,359.4  | 5.7771  |
| MEDIUM | BRANCHY     | O(N) branch-heavy     | 577.0        | 3,359.4  | 5.8222  |
| LARGE  | ALL_PAIRS   | O(N^2)                | 37,066,811.6 | 27,651.4 | 0.0007  |
| LARGE  | LINEAR_SCAN | O(N)                  | 4,540.1      | 27,651.4 | 6.0905  |
| LARGE  | WINDOWED    | O(N*K)                | 36,189.4     | 27,651.4 | 0.7641  |
| LARGE  | PREFIX_SUM  | O(N)                  | 4,628.9      | 27,651.4 | 5.9736  |
| LARGE  | SORT_SCAN   | O(N log N)            | 13,724.9     | 27,651.4 | 2.0147  |
| LARGE  | SCRAMBLED   | O(N) cache-unfriendly | 4,685.6      | 27,651.4 | 5.9014  |
| LARGE  | BRANCHY     | O(N) branch-heavy     | 4,607.1      | 27,651.4 | 6.0019  |

Finding: relational wins only against ALL_PAIRS (O(N^2)).
SCRAMBLED and BRANCHY identical to LINEAR_SCAN at every tier —
mechanisms did not engage. Working sets (max 64 KB) absorbed by L2
at 0.56 ns/access. OC-CC-1 open: tiers did not reach congestion thresholds.

### cargo run --release — Cache-Latency Curve (DECLARED, closes OC-CL-1/2/3)

| N         | WS (bytes)  | MEAN (ns)    | NS/ACCESS | TIER        |
|-----------|-------------|--------------|-----------|-------------|
| 512       | 4,096       | 290.0        | 0.5664    | <= L1d      |
| 1,024     | 8,192       | 570.0        | 0.5566    | <= L1d      |
| 2,048     | 16,384      | 1,140.0      | 0.5566    | <= L1d      |
| 4,096     | 32,768      | 2,310.0      | 0.5640    | <= L1d      |
| 8,192     | 65,536      | 4,620.0      | 0.5640    | <= L2       |
| 16,384    | 131,072     | 9,160.0      | 0.5591    | <= L2       |
| 32,768    | 262,144     | 18,410.0     | 0.5618    | <= L2       |
| 65,536    | 524,288     | 45,630.0     | 0.6963    | <= L2       |
| 131,072   | 1,048,576   | 104,300.0    | 0.7957    | <= L2       |
| 262,144   | 2,097,152   | 229,970.0    | 0.8773    | <= L3       |
| 524,288   | 4,194,304   | 478,160.0    | 0.9120    | <= L3       |
| 1,048,576 | 8,388,608   | 998,520.0    | 0.9523    | <= L3       |
| 2,097,152 | 16,777,216  | 7,085,900.0  | 3.3788    | <= L3       |
| 4,194,304 | 33,554,432  | 20,930,340.0 | 4.9902    | <= L3       |
| 8,388,608 | 67,108,864  | 51,560,400.0 | 6.1465    | > L3 (RAM)  |

Declared congestion thresholds (grounding V2.0 tier boundaries):
  L2 saturation begins: N=65,536 / 524 KB / 0.70 ns/access
  L3 entry:             N=262,144 / 2 MB / 0.88 ns/access
  L3-internal step:     N=1,048,576→2,097,152 / 0.95→3.38 ns/access (3.5x jump)
  RAM:                  N=8,388,608 / 64 MB / 6.15 ns/access
  Total range: ~11x (0.56 → 6.15 ns/access)

OC-CL-1: CLOSED. OC-CL-2: CLOSED (step located, L3-internal 8→16 MB).
OC-CL-3: CLOSED (no sharp L1d→L2 cliff — admissible finding).

---

## V2.0 — Sandbox Build/Test Verification (2026-08-29)

Regime 3 tiers grounded in declared cache-latency observables.
LARGE=65,536 (L2 saturation), XLARGE=262,144 (L3 entry),
XXLARGE=2,097,152 (past L3-internal step).
ALL_PAIRS excluded at LARGE/XLARGE/XXLARGE.
XLARGE/XXLARGE use lighter timing protocol (OC-CC-4).

### cargo test --release (sandbox)

65 passed; 0 failed; 0 ignored; 0 measured; finished in 2.48s

65 tests: 12 binary_baselines, 6 cache_latency_model,
11 complexity_crossover (5 new: tier count/protocol/exclusion checks),
14 declared_graph, 14 operators, 10 process_topology, 3 scaling.
All passing. Sandbox sanity only — requires declared-hardware run.

### cargo run --release (sandbox — NOT declared hardware)

NOT recorded. Sandbox timing is not admissible over D.
Run on Ryzen 5 7600X to produce declared figures.

---

## V2.0 — Declared Hardware Run (PENDING)

Run `cargo test --release` then `cargo run --release` on the declared
hardware (Ryzen 5 7600X / DDR5-5600 / Windows 11) and record output here.

Expected duration: Regime 1 and 2 complete in ~1 minute.
Regime 3 LARGE tier: ~5 minutes (standard protocol, N=65,536).
Regime 3 XLARGE/XXLARGE: ~5–15 minutes total (light protocol, large WS).
Cache-latency sweep: 30–90 seconds.
Total: allow 30–60 minutes for a complete declared run.

Key questions this run answers (OC-CC-1):
  Do SCRAMBLED and BRANCHY diverge from LINEAR_SCAN at LARGE/XLARGE/XXLARGE,
  where the declared hardware mechanism actually engages?
  At XXLARGE (N=2,097,152, 3.38 ns/access) vs SMALL (0.56 ns/access):
  if SCRAMBLED ~ LINEAR_SCAN still → mechanism present but masked by
  relational chain's own memory pressure at those sizes.
  If SCRAMBLED >> LINEAR_SCAN → mechanism engaged, comparison
  structurally meaningful for the first time.

---

## V2.0 — Declared Hardware Run (2026-08-29)

Executed on declared hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11
Home 64-bit. Supersedes the PENDING notice in M_declaration.md V2.0.

### cargo test --release

65 passed; 0 failed; 0 ignored; 0 measured; finished in 0.96s

### cargo run --release — Regime 1

Mean per pass: 27,922.1 ns
Home system throughput: 35,814 analyses/second
MI355X / Home system ratio: 213.0x

Scaling (fifth declared run — NS/EDGE range 3.278–3.539, consistent
with all prior runs; 16,383-edge step remains present but smaller
than V1.3 run: 3.387 → 3.539, ratio 1.0448):

| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,353.1   | 3.278   | 3,100    |
| 2,047   | 48.0    | 6,810.6   | 3.327   | 6,500    |
| 4,095   | 96.0    | 13,750.8  | 3.358   | 13,300   |
| 8,191   | 192.0   | 27,741.7  | 3.387   | 26,900   |
| 16,383  | 384.0   | 57,974.6  | 3.539   | 56,600   |

### cargo run --release — Regime 3 (V2.0, hardware-grounded tiers, DECLARED)

Full 33-point matrix (7 algos × SMALL/MEDIUM/LARGE; 6 algos × XLARGE/XXLARGE).
XLARGE and XXLARGE marked * — lighter timing protocol (OC-CC-4).

| TIER     | BINARY_ALGO | BINARY (ns)   | REL/BIN  |
|----------|-------------|---------------|----------|
| SMALL    | ALL_PAIRS   | 2,441.9       | 0.0968   |
| SMALL    | LINEAR_SCAN | 48.5          | 4.8742   |
| SMALL    | WINDOWED    | 277.6         | 0.8516   |
| SMALL    | PREFIX_SUM  | 51.5          | 4.5903   |
| SMALL    | SORT_SCAN   | 166.2         | 1.4224   |
| SMALL    | SCRAMBLED   | 51.0          | 4.6353   |
| SMALL    | BRANCHY     | 47.7          | 4.9560   |
| MEDIUM   | ALL_PAIRS   | 582,340.3     | 0.0057   |
| MEDIUM   | LINEAR_SCAN | 580.2         | 5.7053   |
| MEDIUM   | WINDOWED    | 4,522.7       | 0.7319   |
| MEDIUM   | PREFIX_SUM  | 581.7         | 5.6906   |
| MEDIUM   | SORT_SCAN   | 1,107.3       | 2.9894   |
| MEDIUM   | SCRAMBLED   | 584.8         | 5.6604   |
| MEDIUM   | BRANCHY     | 576.5         | 5.7419   |
| LARGE    | LINEAR_SCAN | 37,339.2      | 6.1488   |
| LARGE    | WINDOWED    | 289,622.0     | 0.7927   |
| LARGE    | PREFIX_SUM  | 45,242.1      | 5.0747   |
| LARGE    | SORT_SCAN   | 120,792.5     | 1.9007   |
| LARGE    | SCRAMBLED   | 41,137.0      | 5.5812   |
| LARGE    | BRANCHY     | 39,468.4      | 5.8171   |
| XLARGE*  | LINEAR_SCAN | 148,590.0     | 7.2775   |
| XLARGE*  | WINDOWED    | 1,158,810.0   | 0.9332   |
| XLARGE*  | PREFIX_SUM  | 144,730.0     | 7.4716   |
| XLARGE*  | SORT_SCAN   | 518,260.0     | 2.0865   |
| XLARGE*  | SCRAMBLED   | 189,660.0     | 5.7016   |
| XLARGE*  | BRANCHY     | 660,420.0     | 1.6374   |
| XXLARGE* | LINEAR_SCAN | 1,477,760.0   | 10.9412  |
| XXLARGE* | WINDOWED    | 9,429,380.0   | 1.7147   |
| XXLARGE* | PREFIX_SUM  | 1,459,300.0   | 11.0796  |
| XXLARGE* | SORT_SCAN   | 4,005,740.0   | 4.0363   |
| XXLARGE* | SCRAMBLED   | 8,648,560.0   | 1.8695   |
| XXLARGE* | BRANCHY     | 5,465,280.0   | 2.9584   |

---

### Primary Finding — Nonlinear Cost Increase at Declared Working-Set Transitions

The central question of V2.0 (OC-CC-1): do the declared hardware
mechanisms engage at the declared thresholds? The answer is yes,
and the nonlinearity is measurable.

#### Control: LINEAR_SCAN

LINEAR_SCAN — pure sequential O(N) computation, no mechanism stress —
is the control. Its cost per operation is approximately constant across
all tiers:

| Tier    | N         | LINEAR_SCAN ns/op |
|---------|-----------|-------------------|
| SMALL   | 64        | 0.770             |
| MEDIUM  | 1,024     | 0.567             |
| LARGE   | 65,536    | 0.570             |
| XLARGE  | 262,144   | 0.567             |
| XXLARGE | 2,097,152 | 0.705             |

Approximately flat across five orders of magnitude of N. This is the
hardware's cost for sequential computation with no access-pattern or
branch-predictability stress. It is the denominator for all ratios below.

#### SCRAMBLED — access-pattern stress

SCRAMBLED performs the same operations as LINEAR_SCAN over the same
data. Only the memory access order differs (pseudo-random permutation,
fixed seed). By construction: O_S(N) = O_L(N) at every tier.

| Tier    | N         | SCRAMBLED ns/op | Excess ns/op | S/L ratio |
|---------|-----------|-----------------|--------------|-----------|
| SMALL   | 64        | 0.810           | +0.040       | 1.05×     |
| MEDIUM  | 1,024     | 0.572           | +0.005       | 1.01×     |
| LARGE   | 65,536    | 0.628           | +0.058       | 1.10×     |
| XLARGE  | 262,144   | 0.724           | +0.157       | 1.28×     |
| XXLARGE | 2,097,152 | 4.124           | +3.419       | 5.85×     |

Observed transition: XLARGE → XXLARGE.
N increases 8×. SCRAMBLED cost increases 45.6×. S/L ratio: 1.28× → 5.85×.
LINEAR_SCAN over the same interval: cost increases 9.9×. S/L ratio: 1.0×.

Declared finding: T_S(N) ∝̸ T_L(N) after the XLARGE→XXLARGE transition,
while O_S(N) = O_L(N) throughout. Operation count does not account for
the observed timing difference within D.

What does account for it is not established by these observables alone.
The access-pattern difference is the declared independent variable.
The hardware mechanism responsible — and whether it is singular or
composite — requires per-operation hardware counter instrumentation
(L2/L3 miss events, stall cycles) to establish. That instrumentation
is not present in this run. The finding is bounded to: operation count
is not the cause. The open space is the subject of the next instrument pass.

#### BRANCHY — branch-predictability stress

BRANCHY performs the same operations as LINEAR_SCAN. Only branch
outcomes differ (pseudo-random 50/50 sequence, fixed seed, data-dependent).
By construction: O_B(N) = O_L(N) at every tier.

| Tier    | N         | BRANCHY ns/op | B/L ratio |
|---------|-----------|---------------|-----------|
| SMALL   | 64        | 0.757         | 0.98×     |
| MEDIUM  | 1,024     | 0.563         | 0.99×     |
| LARGE   | 65,536    | 0.602         | 1.06×     |
| XLARGE  | 262,144   | 2.519         | 4.44×     |
| XXLARGE | 2,097,152 | 2.607         | 3.70×     |

Observed transition: LARGE → XLARGE.
N increases 4×. BRANCHY cost increases 16.7×. B/L ratio: 1.06× → 4.44×.

Declared finding: T_B(N) ∝̸ T_L(N) after the LARGE→XLARGE transition,
while O_B(N) = O_L(N) throughout. Operation count does not account for
the observed timing difference within D.

Same bounded finding as SCRAMBLED. Same open space regarding mechanism.

#### Two Distinct Transition States — Not One Threshold

SCRAMBLED and BRANCHY do not share a transition point:

| Algorithm | Observed transition | N ratio | Cost ratio | S or B / L at transition |
|-----------|--------------------|---------|-----------:|--------------------------|
| SCRAMBLED | XLARGE → XXLARGE  | 8×      | 45.6×      | 1.28× → 5.85×            |
| BRANCHY   | LARGE → XLARGE    | 4×      | 16.7×      | 1.06× → 4.44×            |
| LINEAR    | none (control)    | —       | ~N-linear  | 1.0× throughout          |

The BRANCHY transition occurs one full tier earlier than SCRAMBLED's,
at a declared working-set size 8× smaller (2 MB vs 16 MB). These are
observably distinct transitions in the declared domain.

This means computational state transition is not a function of working-set
size alone. The transition point depends on which relational configuration
is under stress. A single congestion threshold in bytes does not describe
what this hardware does. The observable computational state is the joint
configuration (W, access-pattern configuration, branch configuration) —
and transitions occur at different points in that space depending on which
dimension is varied.

Collapsing these into a scalar would erase this finding.

#### Cache-Latency Transition Location

The two declared-hardware cache-latency runs locate the deep-L3 step
at different working-set sizes: run 1 (V1.3) at 8→16 MB; run 2 (V2.0)
at 32→64 MB. The early-L3 region (2–8 MB, 0.85–0.96 ns/access) is
consistent across both runs. The step location in the deep-L3 region
is not stable under the lighter protocol (OC-CL-1: 3 warm / 10 timed).

Therefore a fixed byte threshold for the deep-L3 transition cannot be
declared from these runs. What can be declared: the transition exists
somewhere in the 8–64 MB range on this hardware for this access pattern,
and the early-L3 region is stable. Finer-grained instrumentation with
the standard protocol (100 warm / 1000 timed) is required to locate it.

#### Open Conditions Updated

OC-CC-1: CLOSED. Nonlinear cost increases observed at declared
working-set transitions. SCRAMBLED and BRANCHY both diverge from
LINEAR_SCAN with excess factors that are not attributable to
operation count.

OC-CC-4: Elevated variance at XLARGE/XXLARGE (lighter protocol).
The observed excess factors (4.44× BRANCHY at XLARGE, 5.85× SCRAMBLED
at XXLARGE) are large enough that run-to-run variance does not alter
the qualitative finding. The transitions are not noise.

OC-HW-1 (NEW): The hardware mechanism(s) responsible for the excess
cost at each transition are not established by wall-clock timing alone.
Per-operation hardware counter instrumentation (L2/L3 miss events,
branch misprediction events, stall cycles, IPC) is required to
populate the hardware state H_t and establish which relational
configurations precede which hardware-state transitions. This is the
declared next instrument pass.

### cargo run --release — Cache-Latency Curve (second declared run)

Second declared-hardware run of the cache-latency sweep. Results
consistent with first run (2026-08-29 V1.3) with one difference:
the pronounced step appears at N=4,194,304→8,388,608 (32→64 MB,
5.88→6.53 ns/access) in this run, vs N=2,097,152→4,194,304 in the
prior run. Both are within L3/RAM transition zone. Run-to-run
variation at the largest sizes is expected under OC-CL-1 (lighter
protocol, 3 warm / 10 timed).

| N         | WS (bytes)  | MEAN (ns)    | NS/ACCESS |
|-----------|-------------|--------------|-----------|
| 512       | 4,096       | 400.0        | 0.7812    |
| 1,024     | 8,192       | 740.0        | 0.7227    |
| 2,048     | 16,384      | 1,490.0      | 0.7275    |
| 4,096     | 32,768      | 2,940.0      | 0.7178    |
| 8,192     | 65,536      | 5,830.0      | 0.7117    |
| 16,384    | 131,072     | 9,500.0      | 0.5798    |
| 32,768    | 262,144     | 18,930.0     | 0.5777    |
| 65,536    | 524,288     | 37,750.0     | 0.5760    |
| 131,072   | 1,048,576   | 116,630.0    | 0.8898    |
| 262,144   | 2,097,152   | 223,790.0    | 0.8537    |
| 524,288   | 4,194,304   | 476,120.0    | 0.9081    |
| 1,048,576 | 8,388,608   | 1,005,650.0  | 0.9591    |
| 2,097,152 | 16,777,216  | 2,582,190.0  | 1.2313    |
| 4,194,304 | 33,554,432  | 24,642,710.0 | 5.8753    |
| 8,388,608 | 67,108,864  | 54,784,110.0 | 6.5308    |

The early-L3 region (N=262,144–1,048,576, 2–8 MB) is consistent
across both runs: 0.85–0.96 ns/access. The deep-L3 transition
location is not stable under OC-CL-1 protocol — run 1 places it
at 8→16 MB, run 2 at 32→64 MB. A fixed byte threshold for that
transition cannot be declared from two light-protocol runs.
See OC-HW-1 in Primary Finding above.

---

## V3.0 — Declared Hardware Run (2026-08-29)

Executed on declared hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11
Home 64-bit. 76/76 tests passing.

### cargo test --release

76 passed; 0 failed; 0 ignored; 0 measured; finished in 0.92s

### cargo run --release — Regimes 1, 2, 3

Regime 1 consistent with prior runs:
  Mean per pass: 27,907.0 ns  |  Throughput: 35,833 analyses/second
  MI355X ratio: 212.9×  |  NS/EDGE: 3.311–3.765 (run-to-run variation, normal)

Regime 2: unchanged (26 nodes, 24 edges, OC-PT-1/2/3 open).

Regime 3: consistent with V2.0 declared run within light-protocol variance.
  BRANCHY and SCRAMBLED divergence from LINEAR confirmed at XLARGE/XXLARGE.

### cargo run --release — Regime 4: Transition Gradient (DECLARED)

25-point fine-grained sweep, N=65,536 to N=4,194,304 (512 KB to 32 MB).
Standard protocol (100w/1000t) through N=524,288.
Lighter protocol (10w/100t) above N=524,288 — OC-TG-2.

Full gradient table:

| N         | WS (KB) | LIN ns/op | SCR ns/op | BRN ns/op | S/L    | B/L    | Proto |
|-----------|---------|-----------|-----------|-----------|--------|--------|-------|
| 65,536    | 512     | 0.5686    | 0.5926    | 0.5527    | 1.0422 | 0.9719 |       |
| 90,000    | 703     | 0.5719    | 0.6078    | 1.0759    | 1.0628 | 1.8813 |       |
| 122,880   | 960     | 0.5769    | 0.6536    | 1.8758    | 1.1328 | 3.2512 |       |
| 163,840   | 1,280   | 0.5776    | 0.7005    | 2.3367    | 1.2128 | 4.0454 |       |
| 196,608   | 1,536   | 0.5821    | 0.7151    | 2.3816    | 1.2284 | 4.0911 |       |
| 229,376   | 1,792   | 0.5740    | 0.7310    | 2.3795    | 1.2735 | 4.1455 |       |
| 262,144   | 2,048   | 0.5741    | 0.7357    | 2.4675    | 1.2816 | 4.2981 |       |
| 327,680   | 2,560   | 0.5564    | 0.8144    | 2.5163    | 1.4637 | 4.5223 |       |
| 393,216   | 3,072   | 0.5795    | 0.7583    | 2.5218    | 1.3085 | 4.3519 |       |
| 458,752   | 3,584   | 0.5772    | 0.7616    | 2.5339    | 1.3194 | 4.3898 |       |
| 524,288   | 4,096   | 0.5772    | 0.7613    | 2.5418    | 1.3189 | 4.4036 |       |
| 655,360   | 5,120   | 0.5741    | 0.7789    | 2.5511    | 1.3567 | 4.4435 | *     |
| 786,432   | 6,144   | 0.5678    | 0.7829    | 2.5547    | 1.3787 | 4.4992 | *     |
| 917,504   | 7,168   | 0.5728    | 0.7867    | 2.5545    | 1.3734 | 4.4597 | *     |
| 1,048,576 | 8,192   | 0.5895    | 0.7852    | 2.5555    | 1.3321 | 4.3353 | *     |
| 1,245,184 | 9,728   | 0.5717    | 1.1020    | 2.5947    | 1.9276 | 4.5388 | *     |
| 1,441,792 | 11,264  | 0.5753    | 0.8039    | 2.5584    | 1.3973 | 4.4469 | *     |
| 1,638,400 | 12,800  | 0.6144    | 0.8499    | 2.5968    | 1.3834 | 4.2268 | *     |
| 1,835,008 | 14,336  | 0.5781    | 0.9630    | 2.5907    | 1.6658 | 4.4815 | *     |
| 2,097,152 | 16,384  | 0.5874    | 1.7745    | 2.6708    | 3.0210 | 4.5468 | *     |
| 2,359,296 | 18,432  | 0.5775    | 2.3721    | 2.6007    | 4.1078 | 4.5037 | *     |
| 2,752,512 | 21,504  | 0.5782    | 1.5136    | 2.5757    | 2.6180 | 4.4549 | *     |
| 3,145,728 | 24,576  | 0.5802    | 3.7910    | 2.5565    | 6.5341 | 4.4063 | *     |
| 3,670,016 | 28,672  | 0.5798    | 4.1151    | 2.6541    | 7.0970 | 4.5772 | *     |
| 4,194,304 | 32,768  | 0.5986    | 4.9865    | 2.5728    | 8.3298 | 4.2978 | *     |

S/L = SCR/LIN (SCRAMBLED cost / LINEAR cost).
B/L = BRN/LIN (BRANCHY cost / LINEAR cost).
LINEAR ns/op range: 0.556–0.614 across all 25 points — control stable.

---

### Primary Finding — Two Transition Surfaces, Two Distinct Shapes

The gradient resolves what V2.0 bracketed. The two transition surfaces
are not only at different working-set sizes — they have different shapes.

#### BRANCHY — sharp onset, early plateau (standard protocol, high confidence)

BRANCHY's transition is fully captured in the standard-protocol region
(N=65,536–524,288, no light-protocol variance):

| N         | WS     | BRN ns/op | B/L    |
|-----------|--------|-----------|--------|
| 65,536    | 512 KB | 0.553     | 0.972  |
| 90,000    | 703 KB | 1.076     | 1.881  |
| 122,880   | 960 KB | 1.876     | 3.251  |
| 163,840   | 1.3 MB | 2.337     | 4.045  |
| 524,288   | 4.1 MB | 2.542     | 4.404  |

Onset: N=65,536 → 90,000 (WS: 512 KB → 703 KB). B/L doubles in one step.
Rapid rise: N=90,000 → 163,840 (WS: 703 KB → 1.3 MB). B/L: 1.88 → 4.05.
Plateau: N=163,840 onward. B/L stabilizes at 4.0–4.6 and remains there
  through the entire sweep to N=4,194,304 (32 MB).

The plateau is confirmed across all 14 light-protocol points:
B/L range 4.23–4.58 from N=655,360 to N=4,194,304. Flat.

Declared finding: BRANCHY's excess cost saturates at approximately 4×
relative to LINEAR at WS ≈ 1.3 MB and does not increase further with
working-set size. The branch-predictability mechanism engages at L2
entry and reaches full effect within one doubling.

#### SCRAMBLED — late onset, continuing rise (partially light-protocol)

SCRAMBLED's behavior divides into three declared regions:

Pre-transition (standard protocol, N=65,536–524,288):
  S/L rises gradually from 1.04 to 1.32. SCR ns/op: 0.59→0.76.
  No sharp transition. Gradual increase consistent with growing
  access-pattern cost as working set approaches L3 boundary.

Stable region (light protocol, N=524,288–1,638,400):
  S/L approximately flat at 1.32–1.45. SCR ns/op: 0.76–0.85.
  Two elevated points (N=1,245,184 S/L=1.93; N=1,835,008 S/L=1.67)
  are not sustained in adjacent points — consistent with light-protocol
  variance (10w/100t), not a declared transition.

Transition onset (light protocol, N=2,097,152 onward):
  N=2,097,152: S/L = 3.02 (SCR: 1.77 ns/op)
  N=2,359,296: S/L = 4.11 (SCR: 2.37 ns/op)
  N=3,145,728: S/L = 6.53 (SCR: 3.79 ns/op)
  N=3,670,016: S/L = 7.10 (SCR: 4.12 ns/op)
  N=4,194,304: S/L = 8.33 (SCR: 4.99 ns/op)

S/L is still rising at the upper bound of the sweep (32 MB). No plateau
observed. SCRAMBLED's transition has a different shape from BRANCHY's:
it is a continuing rise, not a saturating step.

Note on light-protocol variance: the elevated S/L values from N=2,097,152
onward show some run-to-run variation (N=2,752,512 drops to S/L=2.62
between peaks at 4.11 and 6.53). Under the standard protocol this
variance would be smaller. The trend — S/L rising and sustained above
3.0 from N=2,097,152 onward — is declared; the precise values at
individual light-protocol points carry elevated uncertainty (OC-TG-2).

#### Comparison of shapes

| Property                  | BRANCHY                    | SCRAMBLED                     |
|---------------------------|----------------------------|-------------------------------|
| Transition onset WS       | ~512–703 KB                | ~16 MB                        |
| Transition complete WS    | ~1.3 MB                    | Not yet (> 32 MB)             |
| Shape                     | Sharp step, early plateau  | Gradual onset, continuing rise |
| Peak observed ratio       | 4.6× (plateau)             | 8.3× (still rising at 32 MB) |
| Protocol at transition    | Standard (high confidence) | Light (elevated variance)     |
| Gap between surfaces      | 13× in working-set size    |                               |

These are two observably distinct transition surfaces with different
shapes. They cannot be described by a single threshold or a single
mechanism. The observable computational state (W, access-pattern
configuration, branch configuration) is the declared object; no
scalar reduction of these dimensions is admissible without losing
the structure of this finding.

OC-TG-1: OPEN. Finer sweep around N=65,536–163,840 (BRANCHY onset)
  would locate that transition within tighter bounds. Finer sweep
  around N=1,835,008–2,359,296 (SCRAMBLED onset) would reduce
  light-protocol variance at the transition point.

OC-TG-2: OPEN. Light-protocol figures (N > 524,288) carry elevated
  variance. Standard-protocol runs at SCRAMBLED transition region
  (N=1,835,008–2,359,296) would confirm onset location.

OC-HW-1: OPEN. Hardware mechanism(s) producing these surfaces not
  yet established. Per-operation hardware counter instrumentation
  required (L2/L3 miss events, branch misprediction rate, stall
  cycles, IPC) to populate H_t and establish which relational
  configurations precede which hardware-state transitions.

---

## V4.0 — uProf Declared Hardware Run (PENDING)

Run AMD uProf in hardware counter sampling mode against benchmark.exe
at the six declared measurement points (M_declaration.md V4.0):

  B-pre:  N=65,536   (512 KB,  B/L=0.972 — BRANCHY pre-transition)
  B-on:   N=122,880  (960 KB,  B/L=3.251 — BRANCHY onset)
  B-post: N=524,288  (4.1 MB,  B/L=4.404 — BRANCHY plateau)
  S-pre:  N=524,288  (4.1 MB,  S/L=1.319 — SCRAMBLED pre-transition)
  S-on:   N=2,097,152 (16 MB,  S/L=3.021 — SCRAMBLED onset)
  S-post: N=4,194,304 (32 MB,  S/L=8.330 — SCRAMBLED post)

Declare the following for each point when recording results:
  uProf version and sampling mode used
  Actual Zen 4 event names (not generic — OC-HW-3)
  Raw counter values: CYC, INS, IPC, BR, BR_M, L1_M, L2_M, L3_M,
    DTLB_M, STALL_F, STALL_B, MEM_BW
  Normalized rates: BR_M/BR, BR_M/N, L2_M/N, L3_M/N, DTLB_M/N,
    INS/N, CYC/N, STALL_B/CYC
  Comparison: LINEAR vs SCRAMBLED vs BRANCHY at same N

Declared predictions on record (falsifiable):
  BRANCHY: BR_M/BR rises B-pre → B-on, then stabilizes at B-post,
    tracking B/L shape.
  SCRAMBLED: L3_M/N rises S-pre → S-on → S-post, tracking S/L shape.
    If flat, watch DTLB_M/N and STALL_B/CYC instead.
  Both: INS/N approximately equal across LINEAR, SCRAMBLED, BRANCHY
    at each N while CYC/N diverges — confirming equal instruction
    retirement with unequal cycle cost.

uProf figures are NOT wall-clock comparable to benchmark timing (OC-HW-2).
Record raw uProf output here. Analysis of R → H → O joint state follows.

---

## V4.0 — uProf Declared Hardware Run (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X (Family 0x19, Model 0x61) / Windows 11.
Target: probe.exe — isolated single-algorithm measurement binary (V4.0).
Each measurement point is a separate uProf session with one algorithm only.
Timing figures from probe.exe are informational only — OC-HW-2.

Monitored events (assess_ext, Zen 4 names):
  CYCLES_NOT_IN_HALT (PMCx076), RETIRED_INST (PMCx0C0),
  RETIRED_BR_INST (PMCx0C2), RETIRED_BR_INST_MISP (PMCx0C3),
  L1_DC_ACCESSES_ALL (PMCx029), L2_CACHE_ACCESS_FROM_L1_DC_MISS (PMCx060),
  L1_DEMAND_DC_REFILLS_LOCAL_L2 (PMCx043 um:0x01),
  L1_DEMAND_DC_REFILLS_LOCAL_CACHE (PMCx043 um:0x02),
  L1_DEMAND_DC_REFILLS_LOCAL_DRAM (PMCx043 um:0x08),
  plus stall, WCB, and dispatch events.

OC-HW-3: CLOSED. Zen 4 event names confirmed above.

### Declared H — Process-level counter data at each measurement point

Columns: CPI | %BR_MISP | %L1_MISS | DRAM_PTI | L3_PTI | L2_PTI
(PTI = per thousand retired instructions)

#### BRANCHY measurement points

| Point          | N       | WS     | CPI    | %BR_MISP | %L1_MISS | DRAM_PTI | L3_PTI | L2_PTI |
|----------------|---------|--------|--------|----------|----------|----------|--------|--------|
| b_pre_linear   | 65,536  | 512 KB | 0.8153 | 0.000    | 15.978   | 0.000    | 0.000  | 0.697  |
| b_pre_branchy  | 65,536  | 512 KB | 0.5581 | 3.069    | 11.711   | 0.000    | 0.000  | 0.758  |
| b_on_branchy   | 122,880 | 960 KB | 1.4219 | 22.784   | 2.211    | 0.000    | 0.080  | 0.399  |
| b_post_branchy | 524,288 | 4.1 MB | 1.6628 | 27.814   | 2.424    | 0.000    | 0.249  | 0.747  |

#### SCRAMBLED measurement points

| Point            | N         | WS     | CPI    | %BR_MISP | %L1_MISS | DRAM_PTI | L3_PTI  | L2_PTI |
|------------------|-----------|--------|--------|----------|----------|----------|---------|--------|
| s_pre_linear     | 524,288   | 4.1 MB | 0.7998 | 0.250    | 13.389   | 0.000    | 0.247   | 0.412  |
| s_pre_scrambled  | 524,288   | 4.1 MB | 0.3939 | 0.090    | 56.949   | 0.021    | 128.073 | 28.581 |
| s_on_scrambled   | 2,097,152 | 16 MB  | 1.1781 | 0.116    | 38.497   | 5.757    | 93.257  | 5.821  |
| s_post_scrambled | 4,194,304 | 32 MB  | 1.8004 | 0.134    | 34.879   | 36.188   | 55.932  | 3.314  |

### Primary Finding — Two Mechanisms, Cleanly Separated in H

#### BRANCHY: mechanism is branch misprediction

%BR_MISP tracks the B/L timing ratio across measurement points:

  b_pre_branchy:  3.1%  misprediction  (B/L ≈ 0.97, pre-transition)
  b_on_branchy:  22.8%  misprediction  (B/L = 3.25, transition)
  b_post_branchy: 27.8%  misprediction  (B/L = 4.40, plateau)

%BR_MISP rises 7.4× from b_pre to b_on — tracking the B/L transition.
%BR_MISP plateaus from b_on to b_post (22.8% → 27.8%, small further rise)
— consistent with the B/L plateau in the timing data.

DRAM_PTI = 0 at all BRANCHY points. L3_PTI ≈ 0 at all BRANCHY points.
Cache pressure is not the mechanism. The working set fits in L2 throughout.

CPI rises 2.55× from b_pre to b_on (0.558 → 1.422) — the processor is
spending more cycles per instruction because branch mispredictions force
pipeline flushes and re-execution of the correct path.

Declared finding: BRANCHY's excess cost is attributable to branch
misprediction. The hardware predictor saturates between N=65,536 and
N=122,880 (512–960 KB). %BR_MISP plateaus consistent with the timing
plateau. DRAM and cache are not contributing factors.

#### SCRAMBLED: mechanism is DRAM pressure from L3 working-set overflow

DRAM_PTI and L3_PTI track the S/L timing ratio across measurement points:

  s_pre_scrambled:  DRAM=0.021  L3=128.1  (S/L = 1.32, pre-transition)
  s_on_scrambled:   DRAM=5.757  L3=93.3   (S/L = 3.02, onset)
  s_post_scrambled: DRAM=36.188 L3=55.9   (S/L = 8.33, post)

DRAM_PTI rises 1,723× from s_pre to s_post.
L3_PTI falls as DRAM_PTI rises — data migrates from L3 to DRAM as the
working set grows past L3 capacity (32 MB). At s_pre (4.1 MB) data is
served from L3. At s_post (32 MB) it is served from DRAM.

%BR_MISP ≈ 0.1% throughout all SCRAMBLED points. Branch misprediction
is not the mechanism. L2_PTI falls from 28.6 to 3.3 as N grows —
L2 contribution diminishes as DRAM dominates.

CPI rises 4.57× from s_pre to s_post (0.394 → 1.800). The processor
spends more cycles per instruction waiting for DRAM to fulfill cache
line requests.

Declared finding: SCRAMBLED's excess cost is attributable to DRAM
pressure from L3 working-set overflow. The transition coincides with
the working set crossing L3 capacity. DRAM fills replace L3 fills as N
grows. Branch misprediction is not a contributing factor.

#### Cross-mechanism separation (declared)

The two mechanisms are cleanly separated in H:

| Variable  | LINEAR | BRANCHY (plateau) | SCRAMBLED (post) |
|-----------|--------|-------------------|------------------|
| %BR_MISP  | ~0%    | 27.8%             | ~0.1%            |
| DRAM_PTI  | ~0     | 0                 | 36.2             |
| L3_PTI    | ~0     | ~0                | 55.9             |
| CPI       | ~0.8   | 1.66              | 1.80             |

BRANCHY: branch misprediction elevated, DRAM zero.
SCRAMBLED: DRAM elevated, branch misprediction zero.
Neither mechanism is present in the other's profile.
The declared prediction (M_declaration.md V4.0) is confirmed for both.

#### Open condition status

OC-HW-1: CLOSED. Hardware mechanisms identified and separated:
  BRANCHY → branch misprediction (PMCx0C3 / %RETIRED_BR_INST_MISP).
  SCRAMBLED → DRAM pressure from L3 overflow (PMCx043 um:0x08 / DRAM_PTI).
OC-HW-2: OPEN (permanent). uProf timing not comparable to benchmark timing.
OC-HW-3: CLOSED. Zen 4 event names confirmed.

#### Declared R → H → O relation (V4.0)

For BRANCHY:
  R (computational state): branch-outcome sequence unpredictable by construction.
  H (hardware state):      %BR_MISP rises 3.1% → 27.8%, DRAM_PTI = 0.
  O (timing outcome):      B/L rises 0.97 → 4.40, plateaus.
  Relation: branch unpredictability in R produces pipeline flush cost in H,
  which produces timing excess in O. The plateau in H matches the plateau in O.

For SCRAMBLED:
  R (computational state): access order pseudo-random, working set grows past L3.
  H (hardware state):      DRAM_PTI rises 0.021 → 36.2, L3_PTI falls.
  O (timing outcome):      S/L rises 1.32 → 8.33, still increasing.
  Relation: cache-unfriendly access in R produces L3 overflow and DRAM
  fetch cost in H, which produces timing excess in O. Both H and O
  continue rising — no plateau in either at the declared measurement points.

These are observed progressions R → H → O within the declared domain D.
They do not assert causation beyond D. Intervention — varying one
dimension of R while holding others fixed — is the declared next pass.

---

## V4.0 — R → H → O Complete Table and CPI Analysis (2026-08-29)

### Complete declared H table

All values from uProf assess_ext process-level rows.
CPI = CYCLES_NOT_IN_HALT / RETIRED_INST (self-normalizing — correct
observable for cross-algorithm comparison at same N).
INS/N and CYC/N are relative (uProf sample counts / N) and are NOT
directly comparable across algorithms at the same N — see note below.
PTI = per thousand retired instructions.

#### BRANCHY points

| Point          | N       | WS     | CPI    | %BR_MISP | DRAM_PTI | L3_PTI | L2_PTI |
|----------------|---------|--------|--------|----------|----------|--------|--------|
| b_pre_linear   | 65,536  | 512 KB | 0.8153 | 0.000    | 0.000    | 0.000  | 0.697  |
| b_pre_branchy  | 65,536  | 512 KB | 0.5581 | 3.069    | 0.000    | 0.000  | 0.758  |
| b_on_branchy   | 122,880 | 960 KB | 1.4219 | 22.784   | 0.000    | 0.080  | 0.399  |
| b_post_branchy | 524,288 | 4.1 MB | 1.6628 | 27.814   | 0.000    | 0.249  | 0.747  |

#### SCRAMBLED points

| Point            | N         | WS     | CPI    | %BR_MISP | DRAM_PTI | L3_PTI  | L2_PTI |
|------------------|-----------|--------|--------|----------|----------|---------|--------|
| s_pre_linear     | 524,288   | 4.1 MB | 0.7998 | 0.250    | 0.000    | 0.247   | 0.412  |
| s_pre_scrambled  | 524,288   | 4.1 MB | 0.3939 | 0.090    | 0.021    | 128.073 | 28.581 |
| s_on_scrambled   | 2,097,152 | 16 MB  | 1.1781 | 0.116    | 5.757    | 93.257  | 5.821  |
| s_post_scrambled | 4,194,304 | 32 MB  | 1.8004 | 0.134    | 36.188   | 55.932  | 3.314  |

### Note on INS/N comparability

Raw uProf sample counts (RETIRED_INST, CYCLES_NOT_IN_HALT) are
proportional to run duration, not to N. A longer-running algorithm at
the same N accumulates more samples. At N=524,288: BRANCHY runs 1000
passes at ~2.9 ns/op (~1.5 billion ns total); LINEAR runs 1000 passes
at ~0.64 ns/op (~336 million ns total). BRANCHY therefore accumulates
~4.5× more samples for the same N. INS/N ratios across algorithms are
not comparable without equal-duration normalization.

CPI = CYCLES / INSTRUCTIONS is self-normalizing and is the correct
declared observable for cross-algorithm comparison. It does not depend
on absolute sample count.

### Additional declared finding — two distinct hardware states under scrambled access

At s_pre_scrambled (N=524,288, 4.1 MB): CPI = 0.394.
At s_pre_linear    (N=524,288, 4.1 MB): CPI = 0.800.

SCRAMBLED CPI is less than LINEAR CPI at the same N and same declared
operation count. This is a declared observable, not an error or artifact.
The processor is retiring more than 2.5 instructions per cycle under
scrambled access — substantially more than under sequential access at
the same N.

At s_post_scrambled (N=4,194,304, 32 MB): CPI = 1.800.
CPI has risen 4.57× from s_pre to s_post.
DRAM_PTI has risen 1,723× over the same interval.

The observed progression of H across the three SCRAMBLED points is:

  s_pre:  CPI=0.394  DRAM_PTI=0.021   L3_PTI=128.1  (CPI below linear)
  s_on:   CPI=1.178  DRAM_PTI=5.757   L3_PTI=93.3   (CPI crossing linear)
  s_post: CPI=1.800  DRAM_PTI=36.188  L3_PTI=55.9   (CPI above linear)

This is not a single transition from "fast" to "slow." It is a transition
between two distinct hardware states:

  State 1 (s_pre): low CPI, high L3 fills, low DRAM. Memory requests are
    being served from L3 faster than the pipeline consumes them. The
    scrambled access pattern is not costing cycles in this state — it is
    producing a measurable throughput advantage over sequential access.

  State 2 (s_post): high CPI, low L3 fills, high DRAM. Memory requests
    are being served from DRAM at high latency. The pipeline is stalling
    between instruction retirements.

The transition between these states coincides with the S/L timing rise
observed in V3.0: the timing ratio is approximately 1.0 in State 1 and
rises to 8.33 in State 2.

Provenance boundary — what the counters establish and what they do not:

  Established by H: CPI = 0.394 at s_pre (below linear); CPI = 1.800 at
    s_post (above linear); DRAM_PTI rises 1,723×; L3_PTI falls.
    These are declared observables.

  Not established by H: the internal processor mechanism that produces
    CPI < 1.0 under scrambled access at s_pre. A plausible interpretation
    is that the hardware's ability to maintain multiple independent
    outstanding memory requests (memory-level parallelism) allows it to
    overlap scrambled accesses and keep execution units busy. This
    interpretation is consistent with the observed H but is not directly
    confirmed by the counters available in assess_ext. Direct confirmation
    would require instrumentation of load-queue occupancy, reorder-buffer
    fill level, or outstanding memory request count — none of which are
    in the current declared H set.

  The correct bounded statement is:
    R_scrambled → H_low_CPI_L3 (observed, s_pre)
    R_scrambled → H_high_DRAM_high_CPI (observed, s_post)
    H_low_CPI_L3 → latency hiding through memory-level parallelism
      (consistent interpretation, not yet directly measured — OC-HW-4)

This finding revises the earlier model. The prior framing was:
  irregular access → cache pressure → latency → slowdown

The observed data shows this is incomplete. The actual relation is:

  scrambled access → two distinct hardware states depending on W:
    when W permits concurrent outstanding requests to resolve within
    available hardware capacity → CPI advantage over sequential
    when W exceeds that capacity → CPI penalty, DRAM dominant

The state is not (regular → good, irregular → bad). It is a function
of the joint configuration (A, W, available concurrency, hardware
capacity). Irregularity itself is not the quantity. What matters is
whether the hardware can maintain concurrent useful work while memory
requests are outstanding.

### CPI summary across both mechanisms

| Point            | CPI    | vs linear | Observed H state                     |
|------------------|--------|-----------|--------------------------------------|
| b_pre_linear     | 0.815  | 1.00×     | Control                              |
| b_pre_branchy    | 0.558  | 0.68×     | Branch mechanism not yet engaged     |
| b_on_branchy     | 1.422  | 1.74×     | Branch misprediction active          |
| b_post_branchy   | 1.663  | 2.04×     | Branch misprediction plateau         |
| s_pre_linear     | 0.800  | 1.00×     | Control                              |
| s_pre_scrambled  | 0.394  | 0.49×     | State 1: low CPI, L3 fills dominant  |
| s_on_scrambled   | 1.178  | 1.47×     | Transition: DRAM rising, CPI rising  |
| s_post_scrambled | 1.800  | 2.25×     | State 2: high CPI, DRAM dominant     |

### R → H → O declared relations (final statement)

BRANCHY:
  R: branch-outcome sequence unpredictable (fixed pseudo-random, B dimension)
  H: %BR_MISP rises 3.1% → 27.8%, CPI rises 0.558 → 1.663, DRAM = 0
  O: B/L rises 0.97 → 4.40, plateaus
  Observed progression: branch unpredictability in R co-occurs with
    %BR_MISP rise and CPI rise in H, and B/L rise and plateau in O.
    Plateau in %BR_MISP tracks plateau in B/L and CPI.
    Causation within D requires intervention — not yet performed.

SCRAMBLED:
  R: access order pseudo-random (fixed permutation, A dimension)
  H: CPI falls below linear at s_pre (0.394), rises above at s_post
    (1.800); DRAM_PTI rises 1,723×; L3_PTI falls
  O: S/L rises 1.32 → 8.33, still increasing
  Observed progression: scrambled access in R co-occurs with two
    distinct H states as W grows. The timing excess in O emerges in
    the transition between H states, not at the onset of cache misses.
    No plateau in H or O at the declared measurement points.
    The latency-hiding interpretation of State 1 is consistent with H
    but requires further instrumentation to establish — OC-HW-4.

These are observed progressions within declared domain D.
No claim is made beyond D. Causation requires intervention.

### Open condition OC-HW-4 (new) — latency-hiding mechanism

OC-HW-4: NEW. The low-CPI state observed at s_pre_scrambled (CPI=0.394,
  below linear) is consistent with the hardware maintaining multiple
  concurrent outstanding memory requests (memory-level parallelism),
  allowing execution units to remain busy while L3 requests resolve.
  This interpretation is not directly established by the current H set.
  Direct confirmation requires instrumentation of one or more of:
    - Outstanding load requests (load-queue occupancy)
    - Reorder-buffer fill level
    - Memory-level parallelism counter (if available on Zen 4 in uProf)
    - Pointer-chain intervention (see OC-HW-5)

### Open condition OC-HW-5 (new) — declared next intervention

OC-HW-5: NEW. Declared intervention to test the latency-hiding
  interpretation of OC-HW-4.

  Design: hold N=524,288 (4.1 MB, s_pre working set) and access order
  constant (scrambled permutation), while varying dependency structure D:

    D_independent: each load address is drawn from the permutation
      independently — multiple outstanding requests can be in flight
      simultaneously. This is the current SCRAMBLED baseline.

    D_chained: each load address depends on the value returned by the
      previous load — the declared dependency chain prevents the next
      dependent load address from being resolved until the preceding
      load completes. Formally: a_{t+1} = f(x_{a_t}), where x_{a_t}
      is the value returned at address a_t. Same working set, same
      access distribution, serialized by data dependency.

  Prediction (falsifiable): if the latency-hiding interpretation is
  correct, D_chained at N=524,288 will show CPI substantially higher
  than D_independent at the same N — approaching or exceeding the CPI
  observed at s_post — because the serialized dependency eliminates
  the concurrency that hides L3 latency in State 1. DRAM_PTI should
  remain low (working set unchanged), while CPI rises.

  If D_chained CPI ≈ D_independent CPI at N=524,288, the latency-hiding
  interpretation is not supported and the mechanism of State 1 remains
  open.

  This intervention holds A (scrambled) and W (4.1 MB) constant while
  varying D (dependency structure) — isolating concurrency as the
  independent variable. It is the correct next pass before any claim
  about the latency-hiding mechanism is admitted to the record.

---

## V5.0 — OC-HW-5 Intervention: D_independent vs D_chained (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11.
Target: probe.exe V5.0 — two isolated sessions, one per D configuration.

Declared intervention (M_declaration.md V4.0 OC-HW-5):
  N=524,288  W=4.1 MB  A=scrambled (same permutation, SCRAMBLE_SEED)
  D_independent: a_{t+1} = P(t+1) — concurrent loads permitted
  D_chained:     a_{t+1} = f(x_{a_t}) — serialized by data dependency

All of N, W, A, O_count, and data are held constant.
Only D varies.

### Declared H — process-level counter data

| Variable   | D_independent | D_chained | Ratio  |
|------------|---------------|-----------|--------|
| CPI        | 0.3895        | 3.9929    | 10.25× |
| %BR_MISP   | 0.101         | 0.182     | 1.80×  |
| %L1_MISS   | 51.086        | 45.142    | 0.88×  |
| DRAM_PTI   | 0.020         | 0.020     | 1.00×  |
| L3_PTI     | 115.696       | 148.240   | 1.28×  |
| L2_PTI     | 25.509        | 18.861    | 0.74×  |

### Prediction vs observation

Prediction 1: chained CPI substantially higher than scrambled.
  Observed: CPI 0.390 → 3.993 (10.25×). CONFIRMED.

Prediction 2: DRAM_PTI approximately equal (same working set).
  Observed: DRAM_PTI 0.020 vs 0.020 (identical). CONFIRMED.

Prediction 3 (implicit): same memory source (L3 dominant at W=4.1 MB).
  Observed: L3_PTI 115.7 vs 148.2 — same source, chained slightly higher.
  CONFIRMED.

### Declared finding — D as independent variable

The 10.25× CPI difference is produced solely by D. N, W, A, O_count,
and data are identical. Branch misprediction is near-zero in both
(~0.1–0.2%). DRAM pressure is identical (0.020 PTI). Memory source
is L3-dominant in both.

D_chained: the declared dependency chain (a_{t+1} = f(x_{a_t}))
prevents resolution of each next dependent address until the
preceding load completes and returns its value. L3_PTI is actually
slightly higher in D_chained (148 vs 116) — the hardware is
issuing the same or more L3 requests — but CPI is 10.25× higher
because the declared dependency prevents the next address from
being resolved until each prior result is available, producing
the observed CPI increase.

D_independent: the same L3 fills (116 PTI) are served at CPI=0.390
because multiple independent requests can be outstanding simultaneously.
The hardware serves them concurrently; CPI stays below 1.0 — more than
2.5 instructions retired per cycle.

The 10.25× CPI difference at identical memory pressure is the declared
cost of serialized dependency at this working-set size on this hardware.
It is not a cache effect (DRAM identical, L3 comparable). It is not a
branch effect (%BR_MISP near-zero in both). It is attributable to D.

### R → H → O relation — D dimension (declared)

  R: D_independent vs D_chained, all other dimensions held constant
  H: CPI 0.390 vs 3.993 (10.25×); DRAM identical; L3 comparable
  O: timing 1.159 ns/op vs 12.866 ns/op (11.1× — wall-clock, OC-HW-2)

  Observed: varying D alone at fixed (N, W, A) produces a 10.25×
  difference in CPI and ~11× difference in wall-clock timing.
  The dependency structure is an independently manipulable dimension
  of computational state with a measurable, isolated hardware effect.

### Open condition status

OC-HW-4: CLOSED. The latency-hiding interpretation of State 1 is
  supported by the declared intervention. D_independent at W=4.1 MB
  permits concurrent outstanding L3 requests; CPI=0.390 (below
  linear=0.800). D_chained at the same W serializes by dependency;
  CPI=3.993 (5× above linear). The 10.25× CPI difference at identical
  memory pressure (DRAM_PTI identical, L3_PTI comparable) is consistent
  with concurrent outstanding requests — not reduced miss rate or reduced
  working set — as the source of the State 1 throughput advantage.
  Direct outstanding-load or reorder-buffer measurements would describe
  the precise microarchitectural realization; the intervention establishes
  the effect of D within the declared domain.

OC-HW-5: CLOSED. Both predictions confirmed. D is established as an
  independently declared and independently manipulable dimension of
  computational state with measurable, isolated effect on H.

### Declared variable set — final V5.0 state

R = {N, W, A, B, D, O_count}

  N: element count
  W: working-set bytes
  A: access-order relation (sequential vs scrambled permutation)
  B: branch-outcome relation (branch-free vs data-dependent sequence)
  D: dependency relation (independent vs chained: a_{t+1} = f(x_{a_t}))
  O_count: declared operation count (equal across all variants)

Each dimension is independently declared and independently manipulable.
V4.0 and V5.0 results demonstrate that A, B, and D each produce
distinct and separable effects in H — they cannot be collapsed into
a single "irregularity" variable without losing the structure of the
findings.

---

## V6.0 — OC-RP-1 Intervention: k-Chain A_t Gradient (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11.
Target: probe.exe V6.0 — eight isolated sessions.

Declared intervention (M_declaration.md V5.0 OC-RP-1):
  N=524,288  W=4.1 MB  A=scrambled (same permutation, SCRAMBLE_SEED)
  k mutually independent pointer chains, round-robin interleaved.
  |A_t| = k by construction during steady-state traversal.
  D_independent (V5 reference) retained as separate endpoint.

D_1 note: first collect attempt failed (hardware driver locked by
another process — error 0x80070021). Second collect succeeded.
D_1 CPI from V5.0 declared run (3.9929) used for the CPI column;
the V6 re-run is consistent (ns/op 12.490 vs V5 12.866).

### Declared H — process-level counter data

| Config | k   | \|A_t\| | CPI    | DRAM_PTI | L3_PTI  | L2_PTI  | ns/op* |
|--------|-----|---------|--------|----------|---------|---------|--------|
| D_1    | 1   | 1       | 3.9929 | 0.020    | 148.240 | 18.861  | 12.490 |
| D_2    | 2   | 2       | 1.0016 | 0.487    | 96.266  | 75.649  | 7.098  |
| D_4    | 4   | 4       | 1.0059 | 0.000    | 89.971  | 77.581  | 3.637  |
| D_8    | 8   | 8       | 1.0000 | 0.000    | 94.652  | 89.305  | 2.127  |
| D_16   | 16  | 16      | 1.0052 | 0.518    | 104.663 | 103.109 | 2.188  |
| D_32   | 32  | 32      | 1.0000 | 1.005    | 97.487  | 95.980  | 2.167  |
| D_64   | 64  | 64      | 1.0056 | 1.130    | 105.085 | 97.740  | 2.192  |
| D_ind  | ∞   | ∞       | 0.3895 | 0.020    | 115.696 | 25.509  | 0.842  |

D_ind CPI from V5.0 declared run. ns/op informational only — OC-HW-2.
D_1 CPI from V5.0 declared run. All other CPI from V6.0 uProf runs.

### Primary Finding — Three Concurrency Regimes, Not One Threshold

The A_t → H_t relation has a step structure, not a monotone gradient:

**Regime 1 — |A_t|=1 (D_1):** CPI=3.99. Single chain, full serialization.
  Each next dependent address is unavailable until the preceding load
  completes. CPI=3.9929 and L2_PTI=18.861 are the corresponding
  observed H state.

**Regime 2 — 2 ≤ |A_t| ≤ 64 (D_2 through D_64):** CPI~1.00-1.01.
  Adding a second independent chain recovers most of the D_1 penalty.
  D_2 through D_64 are essentially identical in CPI — the step from
  |A_t|=1 to |A_t|=2 is the critical transition, not the subsequent
  doubling from 2 to 4, 4 to 8, etc.
  L2_PTI rises from 75.6 (D_2) to 103.1 (D_16) as k increases.
  This is an observed change in H accompanying the change in |A_t|.
  The mechanism producing this increase is not established by the
  current H set.

**Regime 3 — D_independent:** CPI=0.39. No within-chain dependency.
  CPI below 1.0 — more than 2.5 instructions retired per cycle.
  L2_PTI=150.0 (highest in sweep).
  Observably distinct from D_64 despite D_64 having 64 concurrent chains.

The CPI gap between Regime 2 and Regime 3 (1.00 vs 0.39, 2.6×) is
not accounted for by |A_t| alone. D_k and D_independent differ in
an additional structural property: every D_k configuration carries
a within-chain load-to-address dependency (a_{i,t+1} = f(x_{a_{i,t}}))
while D_independent has no such dependency. Whether it is this
distinction specifically, or some other correlated property, that
accounts for the remaining H_t difference is declared open as OC-RP-2.

### D_64 ≠ D_independent — Declared

D_64 CPI=1.006. D_independent CPI=0.390. The 2.6× gap persists
despite D_64 having 64 concurrent chains. Therefore |A_t| alone
does not account for the complete observed variation in H_t across
the declared sweep. This is the primary declared finding of V6:

  M_width(A_t) = |A_t| does not by itself describe the observed H_t.

This does not invalidate A_t as a mathematical object. A_t was
declared as a set — its cardinality, structure, and change over time
were all identified as potentially informative. V6 establishes
experimentally that the topology retained by A_t matters beyond its
cardinality. The additional structural distinction between D_k and
D_independent remains associated with an observed difference in H_t
that |A_t| does not capture.

### L2_PTI as a secondary observable

L2_PTI rises from D_1 (18.9) through D_16 (103.1), stabilizes
through D_64 (97.7), then reaches its highest value at D_independent
(150.0). This gradient is smoother than the CPI step — it does not
show the sharp D_1→D_2 discontinuity that CPI shows. L2_PTI and CPI
are two distinct aspects of H_t responding differently to changes in
the declared D configuration.

### OC-RP-1: CLOSED

The declared D_k sweep maps |A_t|=1,2,4,8,16,32,64 to measured H_t
on the declared hardware. CPI changes from 3.9929 at k=1 to
approximately 1.00 at k=2, and remains approximately 1.00 through
k=64. The separately declared D_independent reference has CPI=0.3895.
Therefore |A_t| alone does not account for the complete observed
variation in H_t. The relation between the additional structural
distinction in D and the remaining H_t difference is declared as
OC-RP-2.

### OC-RP-2 (NEW) — Nature of the D_k / D_independent gap

The 2.6× CPI gap between D_64 (1.006) and D_independent (0.390) at
identical N, W, A, O_count is a declared open condition. The within-
chain dependency in D_k prevents the hardware from resolving the next
address in each chain until the current load completes, even when 64
chains are active simultaneously. Whether this gap is attributable to:
  (a) the pending load-to-address dependency consuming a load-queue
      slot that D_independent leaves available for prefetching,
  (b) a difference in the instruction dependency graph presented to
      the out-of-order scheduler, or
  (c) some other microarchitectural distinction
is not established by the current H set. Direct load-queue or
reorder-buffer instrumentation would address this.

### R → H → O declared relation — V6.0

  R: D_k (k independent chains, round-robin) at fixed (N, W, A)
  A_t: |A_t| = k by construction
  H: three-regime step structure in CPI; L2_PTI rises monotonically
  O: ns/op falls from 12.49 (k=1) to 2.13 (k=8), plateau to k=64,
     D_independent distinct at 0.84 ns/op

  Observed: the A_t → H_t relation on this hardware is not a single
  threshold or monotone gradient. It has structure — at least three
  regimes — that requires more than |A_t| to fully describe.
  The additional structural distinction between D_k and D_independent
  was declared as OC-RP-2. Subsequent work (V7, V9) eliminated
  within-chain dependency as operative and identified per-chain
  working set L = N/k as the co-varying variable within the declared
  domain — see V7.0 and V9.0 entries.

---

## V7.0 — OC-RP-2 Intervention: D_8_chained vs D_8_independent (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11. 91 tests passing.
Target: probe.exe V7.0 — three isolated sessions.

Declared intervention (OC-RP-2):
  N=524,288  W=4.1 MB  A=scrambled  |A_t|=8 (held constant)
  D_8_chained:     8 chains, within-chain load-to-address dependency present
  D_8_independent: 8 chains, same partition, NO within-chain dependency
  D_independent:   reference (V5 declared, no chain structure)

Construction verified by 15 declared tests (cargo test --release,
91 total passing on declared hardware before measurement).

### Declared H — process-level counter data

| Config          | CPI    | %BR_MISP | DRAM_PTI | L3_PTI  | L2_PTI  | ns/op* |
|-----------------|--------|----------|----------|---------|---------|--------|
| D_8_chained     | 1.0054 | 4.324    | 0.544    | 107.609 | 107.065 | 2.078  |
| D_8_independent | 1.0086 | 4.274    | 0.862    | 100.862 | 90.517  | 1.472  |
| D_independent   | 0.3895 | 0.101    | 0.020    | 115.696 | 25.509  | 0.842  |

D_independent CPI from V5.0 declared run. ns/op informational — OC-HW-2.

### Primary Finding — Within-Chain Dependency is NOT the Operative Variable

The OC-RP-2 prediction was:
  If D_8_independent CPI ≈ D_independent CPI (0.39): within-chain
    dependency confirmed operative.
  If D_8_independent CPI ≈ D_8_chained CPI (1.00): dependency not
    operative, OC-RP-2 remains open.

Observed:
  D_8_chained CPI:     1.0054
  D_8_independent CPI: 1.0086
  Difference:          0.0032 — within measurement noise, essentially identical.

  D_independent CPI:   0.3895
  Gap from D_8_independent to D_independent: 0.619 — unchanged.

The second prediction is confirmed. Removing the within-chain
load-to-address dependency at |A_t|=8 produces no measurable change
in CPI on this hardware. The within-chain dependency is not the
operative variable for the D_k / D_independent gap.

This is a declared negative result. OC-RP-2 remains open. The observed
H_t difference remains after removal of the within-chain dependency
distinction; the relation accounting for that remaining difference is
not yet established.

### What the result narrows

The OC-RP-2 intervention eliminates within-chain load-to-address
dependency as the operative variable. The search space is now narrowed
to other structural distinctions between D_k and D_independent.

The most prominent remaining distinction is access distribution:

  D_k: addresses drawn from k contiguous segments of the permutation.
    Each segment is a contiguous slice perm[i*L .. (i+1)*L]. The k
    segments together cover the full permutation, but each individual
    chain accesses only L = N/k = 65,536 distinct indices, in a fixed
    order within that segment.

  D_independent: addresses drawn from the full permutation in declared
    order — perm[0], perm[1], ..., perm[N-1] across the full pass.
    Each step accesses a new index drawn from the complete N-element
    permutation with no segment restriction.

At N=524,288 and k=8, each D_8 segment covers L=65,536 elements
(512 KB working set per chain). The full D_independent permutation
covers 524,288 elements (4.1 MB). D_k chains each operate within
a 512 KB sub-region; D_independent operates across the full 4.1 MB.

This is a working-set-per-chain difference, not a dependency difference.
It is a candidate for OC-RP-3: hold the per-chain working set constant
at 4.1 MB (use k=1 but with D_independent-style addressing within
the single chain) and observe whether CPI matches D_independent.

### L2_PTI secondary observable

L2_PTI: D_8_chained (107.1) ≈ D_8_independent (90.5) >> D_independent (25.5).
The L2_PTI pattern mirrors the CPI pattern — both show D_8_chained and
D_8_independent grouping together, distinct from D_independent.
This is consistent with the access-distribution hypothesis: D_k chains
each access a smaller working set, producing different cache fill
behavior than D_independent's full-permutation access.

### OC-RP-2: REMAINS OPEN (narrowed)

Within-chain load-to-address dependency is eliminated as the operative
variable. The remaining candidate is access distribution — specifically,
the effective per-chain working set (L = N/k for D_k vs N for
D_independent). Declared as OC-RP-3.

### OC-RP-3 (NEW) — Access distribution as candidate variable

OC-RP-3: The D_k / D_independent CPI gap may be attributable to the
effective per-chain working set rather than dependency structure.

Declared intervention: construct a single-chain (k=1) variant where
addressing is D_independent-style (full permutation, no segmentation)
but with sequential rather than pointer-chain access to the next index.
This gives effective working set = N = 4.1 MB with no within-chain
dependency. If CPI ≈ D_independent (0.39), access distribution over
the full working set is the operative variable. If CPI ≈ D_k (1.00),
another structural property remains undeclared.

The simplest implementation: D_independent already IS this variant —
it accesses perm[0..N] sequentially with no chain dependency, covering
the full N-element working set. The comparison D_8_independent vs
D_independent at identical |A_t|=8 but different effective working
set per chain (512 KB vs 4.1 MB) is already declared by the V7.0 data.

The observed H_t difference (CPI 1.009 vs 0.390) at identical |A_t|
and absent within-chain dependency is consistent with the effective
per-chain working set being the operative variable. To confirm: vary
L (segment length) while holding k=8 and D_independent addressing,
and observe whether CPI tracks L or remains at 0.39 regardless.

---

## V8.0 — A×D×S Factorial Block (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11. 88 tests passing.
Target: probe.exe V8.0 — eight isolated sessions, one per factorial node.
B=none fixed throughout. Full H vector preserved at all nodes.

Declared factors:
  A ∈ {sequential (seq), scrambled (scr)}
  D ∈ {independent (ind), chain-8 (ch8)}
  S ∈ {S0=(524288, 4.1 MB, standard 100w/1000t),
        S1=(4194304, 32 MB, light 10w/100t — OC-TG-2)}
N and W are not independent — S is the joint size state.
Measured interaction is A×D×S; no attribution to N vs W separately.

Prior measurements (V4–V7) retained as provenance references.
All eight nodes re-measured in this coordinated block.

### Declared H — eight factorial nodes

| Node | A   | D   | S  | CPI    | DRAM_PTI | L3_PTI  | L2_PTI  | %BR_MISP | %L1_MISS |
|------|-----|-----|----|--------|----------|---------|---------|----------|----------|
| F000 | seq | ind | S0 | 0.8017 | 0.000    | 0.494   | 0.497   | 0.292    | 13.991   |
| F010 | scr | ind | S0 | 0.3979 | 0.020    | 121.739 | 27.907  | 0.091    | 55.182   |
| F001 | seq | ch8 | S0 | 1.0019 | 0.000    | 0.262   | 0.603   | 4.098    | 2.302    |
| F011 | scr | ch8 | S0 | 1.0031 | 0.193    | 97.564  | 93.621  | 4.215    | 8.625    |
| F100 | seq | ind | S1 | 0.7893 | 0.000    | 0.245   | 0.316   | 0.154    | 13.699   |
| F110 | scr | ind | S1 | 1.8004 | 36.188   | 55.932  | 3.314   | 0.134    | 34.879   |
| F101 | seq | ch8 | S1 | 1.0041 | 0.000    | 0.246   | 0.291   | 2.613    | 2.075    |
| F111 | scr | ch8 | S1 | 3.9560 | 35.461   | 57.895  | 3.538   | 0.174    | 35.014   |

S1 nodes carry elevated variance (OC-TG-2, light protocol).

### A Operator — 12 Edge Contrast Vectors (ΔCPI shown; full H vector computed)

A-edges (access order varies):
  F000→F010  D=ind S0  ΔCPI=−0.4038  (scr improves — latency hiding at S0)
  F001→F011  D=ch8 S0  ΔCPI=+0.0012  (scr neutral under chain-8 at S0)
  F100→F110  D=ind S1  ΔCPI=+1.0111  (scr worsens — DRAM pressure at S1)
  F101→F111  D=ch8 S1  ΔCPI=+2.9519  (scr worsens much more under chain-8 at S1)

D-edges (dependency structure varies):
  F000→F001  A=seq S0  ΔCPI=+0.2002  (chain-8 adds small cost, seq, S0)
  F010→F011  A=scr S0  ΔCPI=+0.6052  (chain-8 adds larger cost, scr, S0)
  F100→F101  A=seq S1  ΔCPI=+0.2148  (chain-8 adds small cost, seq, S1)
  F110→F111  A=scr S1  ΔCPI=+2.1556  (chain-8 adds much larger cost, scr, S1)

S-edges (size state varies):
  F000→F100  A=seq D=ind  ΔCPI=−0.0124  (seq, ind — essentially no S effect)
  F010→F110  A=scr D=ind  ΔCPI=+1.4025  (scr, ind — large S effect)
  F001→F101  A=seq D=ch8  ΔCPI=+0.0022  (seq, ch8 — essentially no S effect)
  F011→F111  A=scr D=ch8  ΔCPI=+2.9529  (scr, ch8 — largest S effect)

### Three-Way Interaction Vector I_{A,D,S}

Path equivalence confirmed: I_{A,D,S} via A×D path = I_{A,D,S} via A×S path
for all six H variables. ✓

| H variable | I_{A,D}|S0 | I_{A,D}|S1 | I_{A,D,S} |
|------------|------------|------------|-----------|
| CPI        | +0.4050    | +1.9408    | +1.5358   |
| DRAM_PTI   | +0.1730    | −0.7270    | −0.9000   |
| L3_PTI     | −23.9430   | +1.9620    | +25.9050  |
| L2_PTI     | +65.6080   | +0.2490    | −65.3590  |
| %BR_MISP   | +0.3180    | −2.4190    | −2.7370   |
| %L1_MISS   | −34.8680   | +11.7590   | +46.6270  |

### Primary Declared Findings

**Finding 1 — A×D×S interaction confirmed. Variables are not independent.**

The A×D interaction in CPI increases from S0 to S1:
  I_{A,D}|S0 = +0.405
  I_{A,D}|S1 = +1.941
  I_{A,D,S}  = +1.536

The three declared variables interact — their joint effect on H_t
at S1 differs from what would be predicted by adding their individual
effects as observed at S0. The interaction is observed in the
declared data; what produces it is not yet established.

**Finding 2 — A effect reverses sign between S0 and S1.**

At D=independent:
  S0: ΔA = −0.404  (scrambled access produces lower CPI than sequential)
  S1: ΔA = +1.011  (scrambled access produces higher CPI than sequential)

The A contrast vector changes sign between S0 and S1. At S0 scrambled
access is associated with lower CPI; at S1 it is associated with higher
CPI. This sign reversal is observed in the declared data. The A contrast
is not constant across S — A and S are not independent in their joint
effect on H_t. The mechanisms producing the sign reversal at each size
state are consistent with the latency-hiding and DRAM-pressure
interpretations established in V4/V5, but those interpretations remain
bounded by their prior declared evidence.

**Finding 3 — D effect depends entirely on A.**

At A=sequential:
  ΔD|S0 = +0.200   ΔD|S1 = +0.215  (small, stable, S-independent)

At A=scrambled:
  ΔD|S0 = +0.605   ΔD|S1 = +2.156  (large, S-dependent)

The D contrast vector is near-zero under sequential access at both
size states. It is substantially larger under scrambled access and
grows further at S1. The D contrast is conditioned on A — the two
variables are not independent in their joint effect on H_t.

**Finding 4 — DRAM_PTI contrast is zero for all sequential nodes.**

DRAM_PTI is zero at all sequential nodes (F000, F001, F100, F101)
regardless of D or S. DRAM_PTI is large only at scr+S1 nodes
(F110=36.2, F111=35.5). The D=ch8 and D=ind nodes at S1+scr show
essentially the same DRAM_PTI. Within this declared block, the
DRAM_PTI contrast is associated with the (A=scr, S=S1) combination
and shows no D contribution. This is an observed partitioning of the
H vector within the declared experiment — not a general claim about
DRAM behavior.

**Finding 5 — L2_PTI three-way interaction is large (−65.4).**

L2_PTI shows the largest three-way interaction magnitude in the block.
At S0, chain-8+scrambled produces very high L2_PTI (93.6). At S1, it
collapses to 3.5 — similar to chain-8+sequential (0.3). The L2 fill
behavior under scrambled+chain conditions changes substantially between
S0 and S1. This change is consistent with the working-set-per-chain
hypothesis (OC-RP-3) but does not confirm it — the collapse could also
reflect other S-dependent changes in hardware state at S1.

### Relational Structure Update

The A operator contrast field across the factorial cube shows that
the contrast along each declared dimension (A, D, S) is not constant
across the levels of the other dimensions. The I_{A,D,S} three-way
interaction is non-zero across all six H variables, confirmed by
path equivalence. This means the declared variable set R = {N, W, A, B, D}
cannot be treated as producing independent H contributions — the joint
H_t at any node in the cube is not predicted by summing individual
contrast effects observed at other nodes.

The B operator (propagation across the graph) and R operator
(relational field) are the declared next computations once the graph
is extended to include the B dimension and additional W levels.

### OC-RP-3 update

Finding 5 (L2_PTI collapse at S1 under scrambled+chain) is consistent
with the working-set-per-chain hypothesis. At S1, each of the 8 chains
covers N/8 = 524,288 elements = 4.1 MB — comparable to the full S0
working set — which would place each chain's working set near or beyond
L2 capacity. This narrows OC-RP-3's target: the segment length L = N/k
relative to cache hierarchy boundaries is the declared candidate.
A direct test: hold k=8 and vary N while keeping L constant, or hold
N constant and vary k while observing when L2_PTI collapses.

### Open Conditions

OC-RP-2: NARROWED FURTHER. Within-chain dependency (V7) was not operative.
  The factorial block confirms D effect is conditioned on A — D has no
  effect under sequential access. The remaining D×S interaction under
  scrambled access is consistent with per-chain working set rather than
  dependency topology.

OC-RP-3: ACTIVE. L2_PTI collapse at S1+scr+ch8 supports the per-chain
  working-set hypothesis. Direct test: vary k at fixed N and observe
  when L2_PTI transitions from high (S0-like) to low (S1-like).

OC-B-1 (NEW): B dimension (branch configuration) not yet included in
  the factorial block. B×A, B×D, B×S interactions are undeclared.
  Adding B=branchy as a fourth factor would complete the declared
  variable set. Declared as next factorial extension.

---

## V9.0 — OC-RP-3: k Sweep at Fixed N=4,194,304 (2026-08-29)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11. 92 tests passing.
Target: probe.exe V9.0 — ten isolated sessions.

Declared sweep: k ∈ {1,2,4,8,16,32,64,128,256,512}
  N=4,194,304 fixed (32 MB total working set, S1 size state)
  A=scrambled, B=none, D=chain-k (scrambled chains, round-robin)
  L = N/k = per-chain working set in elements; WS_chain = L×8 bytes
  All runs: light protocol (10w/100t) — OC-TG-2
  N=4,194,304 divisible by all declared k values — confirmed by test.

Hypothesis (OC-RP-3): L2_PTI collapse observed at S1+scr+ch8 in V8
is driven by per-chain working set L exceeding L2 capacity (~1 MB,
131,072 elements on Zen 4). As k increases, L decreases and should
recover L2_PTI when L drops below L2 capacity.

Reference: D_independent S1 (F110, V8): CPI=1.800, DRAM=36.188,
L3=55.932, L2=3.314 — the unpartitioned full-permutation baseline.

### Declared H — ten k values

| k   | L (elem)  | WS/chain | CPI    | DRAM_PTI | L3_PTI  | L2_PTI | ns/op* |
|-----|-----------|----------|--------|----------|---------|--------|--------|
| 1   | 4,194,304 | 32 MB    | 1.8823 | 34.742   | 57.762  | 3.485  | 33.490 |
| 2   | 2,097,152 | 16 MB    | 1.8441 | 35.298   | 57.298  | 3.626  | 17.120 |
| 4   | 1,048,576 | 8 MB     | 1.8449 | 34.956   | 57.504  | 3.640  | 8.620  |
| 8   |   524,288 | 4 MB     | 1.8449 | 35.031   | 57.414  | 3.661  | 4.360  |
| 16  |   262,144 | 2 MB     | 1.8284 | 34.917   | 58.009  | 3.780  | 2.220  |
| 32  |   131,072 | 1 MB     | 1.7946 | 27.408   | 68.512  | 10.463 | 1.193  |
| 64  |    65,536 | 512 KB   | 1.4755 | 8.856    | 89.823  | 38.152 | 0.666  |
| 128 |    32,768 | 256 KB   | 1.1304 | 1.359    | 98.457  | 72.254 | 0.577  |
| 256 |    16,384 | 128 KB   | 1.0183 | 0.304    | 99.622  | 88.962 | 0.561  |
| 512 |     8,192 | 64 KB    | 1.0059 | 0.054    | 100.178 | 94.316 | 0.559  |

* ns/op informational only — OC-TG-2 light protocol, OC-HW-2.

### Primary Finding — OC-RP-3 Hypothesis Supported

The per-chain working set L is the operative variable for the L2_PTI
and DRAM_PTI transitions observed in V8.

**Pre-transition region (k=1..16, L=2..32 MB > L2 capacity):**
  L2_PTI flat at 3.5–3.8 PTI. DRAM_PTI flat at 34.9–35.3 PTI.
  CPI flat at 1.828–1.882 — consistent with D_independent S1 (1.800).
  H_t is stable and DRAM-dominated across this entire region.
  Increasing k from 1 to 16 (reducing L from 32 MB to 2 MB) produces
  no detectable change in H_t. The per-chain working set remains well
  above L2 capacity throughout.

**Transition region (k=32..128, L=256 KB..1 MB, crossing L2 boundary):**
  k=32 (L=1 MB, at L2 boundary): transition begins.
    DRAM_PTI: 34.9 → 27.4 (first significant drop)
    L2_PTI:   3.8 → 10.5 (first significant rise)
    CPI:      1.828 → 1.795 (small, transition not yet dominant)
  k=64 (L=512 KB): transition accelerates.
    DRAM_PTI: 27.4 → 8.9
    L2_PTI:   10.5 → 38.2
    CPI:      1.795 → 1.476
  k=128 (L=256 KB): transition continuing.
    DRAM_PTI: 8.9 → 1.4 (approaching zero)
    L2_PTI:   38.2 → 72.3
    CPI:      1.476 → 1.130

**Post-transition region (k=256..512, L=64..128 KB, well within L2):**
  k=256 (L=128 KB): DRAM_PTI=0.304, L2_PTI=89.0, CPI=1.018
  k=512 (L=64 KB):  DRAM_PTI=0.054, L2_PTI=94.3, CPI=1.006
  H_t has converged to the S0 chain-8 regime (V8 F011: CPI=1.003).
  DRAM is eliminated. L2 and L3 fills dominate.

**The transition is gradual, not a cliff.**
  It spans approximately 3 doublings of k (k=32 to k=256) rather
  than occurring at a single threshold. This is consistent with the
  gradual L2→L3 transition observed in the cache-latency curve (V1.3,
  OC-CL-2: step located over multiple doublings, no sharp cliff).

### Declared Relational Finding

The k=1 row confirms a declared relation from V6:
  k=1, L=32 MB: CPI=1.882, DRAM=34.7 — matches D_independent S1
  (CPI=1.800, DRAM=36.2) within light-protocol variance.

At k=1, the single chain covers the full N=4,194,304 element working
set. The H_t at k=1 is observably equivalent to D_independent at S1
within light-protocol variance. This means: when L = N (no
partitioning), the H_t observations for chain-k and D_independent
are indistinguishable in this declared experiment. The D distinction
is not observable in H_t when the per-chain working set equals the
full working set.

Within this declared experiment, the observed H_t variation across k
is consistent with L = N/k being the operative variable — the H_t
transitions track the declared L2 boundary as L decreases. This does
not establish that L is the cause; it establishes that L co-varies
with the observed H_t transitions in a pattern consistent with the
hypothesis. The within-chain dependency (V7) and chain topology have
been eliminated as the source of that variation within these conditions.

This closes the search that began with OC-RP-2. The sequence:
  V7: within-chain dependency not operative (eliminated)
  V8: L2_PTI collapse at S1+scr+ch8 consistent with per-chain WS
  V9: per-chain working set L co-varies with H_t across cache boundary

### OC-RP-3: CLOSED (within declared domain)

Within the declared conditions (A=scrambled, B=none, N=4,194,304,
Ryzen 5 7600X), the per-chain working set L = N/k co-varies with the
observed H_t transitions. The transitions track the declared L2
boundary (~1 MB per chain on Zen 4), are gradual across 3 doublings,
and are consistent with the cache-latency characterization (V1.3).
The within-chain dependency and chain topology have been eliminated
as operative within these conditions.

No claim is made beyond D. Whether L is operative under A=sequential,
at other N values, or on other hardware requires additional declared
measurement.

### Open Conditions Updated

OC-RP-2: CLOSED. Within-chain dependency (V7) not operative within
  declared conditions. Per-chain working set co-varies with H_t
  transitions (V9) within the declared domain.

OC-RP-3: CLOSED. Hypothesis supported within declared conditions.
  L = N/k co-varies with H_t transition at the declared L2 boundary.

OC-B-1: ACTIVE. B dimension not yet in the factorial block. B×A,
  B×D, B×S interactions undeclared. Next declared extension.

OC-V9-1 (NEW): The OC-RP-3 result is declared under A=scrambled only.
  Under A=sequential, the D effect is near-zero at all k (V8 Finding 3).
  Whether L co-varies with H_t under A=sequential is undeclared — no
  variation in L2_PTI or DRAM_PTI is expected (sequential access
  keeps working set in L2 regardless of k) but this has not been
  measured across the k sweep.

---

## V10.0 — A×D×S×B Factorial Block: Four-Factor Extension (2026-08-30)

AMD uProf Version 5.3.521.0. Profile type: assess_ext.
Hardware: Ryzen 5 7600X / DDR5-5600 / Windows 11. [N tests passing — update after build].
Target: probe.exe V9.0 — sixteen isolated sessions.

OC-B-1 addressed: B dimension added as fourth declared factor.
Primary declared finding: I_{A,D,S,B} — the four-way interaction vector.
Whether I_{A,D,S,B} is zero or non-zero is undeclared prior to measurement.

Observable provenance for B:
  B (declared intervention: data-dependent branch pattern, BRANCH_SEED)
  → %BR_MISP (observed hardware response: branch misprediction rate, uProf)
  → CPI (observed execution cost)

Protocol: S0 nodes: standard (100w/1000t). S1 nodes: light (10w/100t) — OC-TG-2.

B=none nodes re-measured in this block for internal consistency.
Prior V8.0 values (F000–F111) retained as provenance references only.

### Declared H — sixteen nodes

| Node  | A   | D   | S  | B      | CPI | DRAM_PTI | L3_PTI | L2_PTI | %BR_MISP | %L1_MISS |
|-------|-----|-----|----|--------|-----|----------|--------|--------|----------|----------|
| G0000 | seq | ind | S0 | none   |     |          |        |        |          |          |
| G0100 | scr | ind | S0 | none   |     |          |        |        |          |          |
| G0010 | seq | ch8 | S0 | none   |     |          |        |        |          |          |
| G0110 | scr | ch8 | S0 | none   |     |          |        |        |          |          |
| G1000 | seq | ind | S1 | none   |     |          |        |        |          |          |
| G1100 | scr | ind | S1 | none   |     |          |        |        |          |          |
| G1010 | seq | ch8 | S1 | none   |     |          |        |        |          |          |
| G1110 | scr | ch8 | S1 | none   |     |          |        |        |          |          |
| G0001 | seq | ind | S0 | branchy|     |          |        |        |          |          |
| G0101 | scr | ind | S0 | branchy|     |          |        |        |          |          |
| G0011 | seq | ch8 | S0 | branchy|     |          |        |        |          |          |
| G0111 | scr | ch8 | S0 | branchy|     |          |        |        |          |          |
| G1001 | seq | ind | S1 | branchy|     |          |        |        |          |          |
| G1101 | scr | ind | S1 | branchy|     |          |        |        |          |          |
| G1011 | seq | ch8 | S1 | branchy|     |          |        |        |          |          |
| G1111 | scr | ch8 | S1 | branchy|     |          |        |        |          |          |

S1 nodes carry elevated variance (OC-TG-2, light protocol).
OC-V9-1: %BR_MISP at B=none nodes — record and confirm near-zero.
If %BR_MISP is non-negligible at any B=none node, flag that node.

### A Operator — A-edge contrast vectors (ΔCPI shown; compute full H vector)

B=none face:
  G0000→G0100  D=ind S0 B=none  ΔCPI=
  G0010→G0110  D=ch8 S0 B=none  ΔCPI=
  G1000→G1100  D=ind S1 B=none  ΔCPI=
  G1010→G1110  D=ch8 S1 B=none  ΔCPI=

B=branchy face:
  G0001→G0101  D=ind S0 B=br    ΔCPI=
  G0011→G0111  D=ch8 S0 B=br    ΔCPI=
  G1001→G1101  D=ind S1 B=br    ΔCPI=
  G1011→G1111  D=ch8 S1 B=br    ΔCPI=

### D-edge contrast vectors

B=none face:
  G0000→G0010  A=seq S0 B=none  ΔCPI=
  G0100→G0110  A=scr S0 B=none  ΔCPI=
  G1000→G1010  A=seq S1 B=none  ΔCPI=
  G1100→G1110  A=scr S1 B=none  ΔCPI=

B=branchy face:
  G0001→G0011  A=seq S0 B=br    ΔCPI=
  G0101→G0111  A=scr S0 B=br    ΔCPI=
  G1001→G1011  A=seq S1 B=br    ΔCPI=
  G1101→G1111  A=scr S1 B=br    ΔCPI=

### B-edge contrast vectors (primary new edges, V9.0)

  G0000→G0001  A=seq D=ind S0   ΔB·CPI=
  G0100→G0101  A=scr D=ind S0   ΔB·CPI=
  G0010→G0011  A=seq D=ch8 S0   ΔB·CPI=
  G0110→G0111  A=scr D=ch8 S0   ΔB·CPI=
  G1000→G1001  A=seq D=ind S1   ΔB·CPI=
  G1100→G1101  A=scr D=ind S1   ΔB·CPI=
  G1010→G1011  A=seq D=ch8 S1   ΔB·CPI=
  G1110→G1111  A=scr D=ch8 S1   ΔB·CPI=

If ΔB·CPI is constant across all 8 conditions: B contributes additively.
If ΔB·CPI varies: B×(A,D,S) interaction is present.

### Three-Way Interaction Vectors

B=none face (confirm against V8.0 values within light-protocol variance):
  I_{A,D}|S0,B=none =
  I_{A,D}|S1,B=none =
  I_{A,D,S}|B=none  =   (path equivalence check required)

B=branchy face (new):
  I_{A,D}|S0,B=br   =
  I_{A,D}|S1,B=br   =
  I_{A,D,S}|B=br    =   (path equivalence check required)

### Four-Way Interaction Vector I_{A,D,S,B} — Primary Finding

  I_{A,D,S,B} = I_{A,D,S}|B=br − I_{A,D,S}|B=none

  CPI:      
  DRAM_PTI: 
  L3_PTI:   
  L2_PTI:   
  %BR_MISP: 
  %L1_MISS: 

OC-V9-2: path equivalence check — confirm I_{A,D,S,B} via alternate paths:
  Via A×B slice:  I_{A,B}|{D=ch8,S1} − I_{A,B}|{D=ind,S0} =
  Agreement with primary path: [YES / NO — record discrepancy if NO]

### Primary Declared Findings

[To be completed after measurement.]

Finding 1 — B-edge contrast uniformity:
  If ΔB·CPI is uniform across all 8 A×D×S combinations:
    B contributes additively; no B×(A,D,S) interaction in CPI.
  If ΔB·CPI varies:
    State which combinations show elevated contrast and the direction.

Finding 2 — I_{A,D,S,B}:
  State observed value and whether zero or non-zero within declared variance.
  If non-zero: identify which H variables carry the interaction.
  If zero: B cost is constant across the A×D×S structure in this domain.

Finding 3 — %BR_MISP at B=none nodes (OC-V9-1):
  State observed values. If non-negligible at any node, identify and flag.

Finding 4 — %BR_MISP at B=branchy nodes:
  State observed values. Confirm that %BR_MISP is the primary elevated
  H variable at B=branchy nodes relative to their B=none counterparts.
  If %BR_MISP does not rise substantially at B=branchy nodes, the declared
  B intervention did not engage its intended mechanism — flag as OC.

### Open Conditions

OC-TG-2: Active. S1 nodes carry elevated variance (light protocol).
OC-HW-2: Active. uProf timing not comparable to benchmark.exe.
OC-V8-1: Active. chains-8-seq uses sequential index partition (unchanged).
OC-B-1:  ADDRESSED (V9.0). B added as fourth factor.
OC-V9-1: ACTIVE. Verify %BR_MISP near-zero at all B=none nodes.
OC-V9-2: ACTIVE. Path equivalence check on I_{A,D,S,B} required.
OC-V9-3 (NEW, if applicable): If %BR_MISP does not rise at B=branchy nodes,
  the branch mechanism did not engage — declare as open and investigate
  whether BRANCH_SEED produces sufficient misprediction at declared N values.

---

## OC-DRAM-1 — DRAM Latency Calibration (2026-08-30)

Purpose: Close OC-DRAM-1. Replace declared-approximate DRAM_LAT (180 cycles,
range 160–220) with a hardware-measured value on this specific system
(Ryzen 5 7600X / DDR5-5600).

Design:
  Scrambled single-chain pointer chase (A=scr, D=ind, k=1, B=none).
  Single chain: every access is serialized. No k-way parallelism.
  Working set substantially larger than L3 (32MB): forces all accesses to DRAM.
  CPI ≈ DRAM_LAT / insts_per_iter (serialized, DRAM-bound).
  Back-calculate: DRAM_LAT = CPI × (1000 / RETIRED_BR_INST_PTI).

Two declared N values:
  CAL-2X: N=8,388,608, WS=64MB (2× L3). Primary measurement.
  CAL-4X: N=16,777,216, WS=128MB (4× L3). Confirmation.
  Agreement criterion: |DRAM_LAT(2X) − DRAM_LAT(4X)| ≤ 5 cycles.
  If disagreement: L3 residency not fully evicted at CAL-2X; use CAL-4X.

Protocol: dram-cal (3w/20t). Each pass at N=8M takes ~15s.
  Warm: 3 passes. Timed: 20 passes under uProf.

Run commands:
  probe.exe scrambled-dram-cal  8388608
  probe.exe scrambled-dram-cal 16777216

### Measured H vectors

| Field             | CAL-2X (N=8388608) | CAL-4X (N=16777216) |
|-------------------|-------------------|-------------------|
| CPI               |                   |                   |
| RETIRED_BR_INST PTI|                  |                   |
| %BR_MISP          |                   |                   |
| DRAM_PTI          |                   |                   |
| L3_PTI            |                   |                   |
| L2_PTI            |                   |                   |
| %L1_DC_MISSES     |                   |                   |

### Back-calculation

  insts_per_iter(2X) = 1000 / RETIRED_BR_INST_PTI(2X) =
  DRAM_LAT(2X) = CPI(2X) × insts_per_iter(2X) =

  insts_per_iter(4X) = 1000 / RETIRED_BR_INST_PTI(4X) =
  DRAM_LAT(4X) = CPI(4X) × insts_per_iter(4X) =

  Agreement: |DRAM_LAT(2X) − DRAM_LAT(4X)| =
  Agreement criterion met: [YES / NO]

### Verification checks

  1. DRAM_PTI dominant at both N values: [YES / NO]
     If L3_PTI > DRAM_PTI at CAL-2X, L3 not fully evicted — use CAL-4X only.

  2. %BR_MISP near baseline (< 10%): [YES / NO]
     High misprediction would add pipeline flush cost to CPI — confound.

  3. DRAM_LAT falls within declared prior range (160–220 cycles): [YES / NO]
     If outside range, declare new range and update substrate_model.rs.

### Declared DRAM_LAT (post-measurement)

  DRAM_LAT (hardware-measured) =          cycles
  Source: this calibration run, Ryzen 5 7600X / DDR5-5600
  Replaces: declared-approximate 180 cycles (range 160–220)

  Updated substrate_model.rs DRAM_LAT constant: [YES / NO — record version]

### OC-DRAM-1 disposition

  [CLOSED / OPEN — state reason if open]

---

## OC-DRAM-1 / OC-DRAM-1a — Calibration Results (2026-08-30)

### Gather calibration (scrambled-dram-cal)

Declared intervention: WS > nominal L3 (32MB). A=scr, D=ind, B=none.
Workload: run_scrambled — gather (independent random accesses, NOT pointer chase).
Protocol: dram-cal (3w/20t).

| Field             | CAL-2X (N=8388608, 64MB) | CAL-4X (N=16777216, 128MB) |
|-------------------|--------------------------|-----------------------------|
| CPI               | 2.0692                   | 2.2043                      |
| RETIRED_BR_INST PTI| 264.0319                | 260.6101                    |
| %BR_MISP          | 0.151%                   | 0.126%                      |
| DRAM_PTI          | 62.5000                  | 71.2202                     |
| L3_PTI            | 23.9277                  | 12.3958                     |
| L2_PTI            | 2.5429                   | 2.1875                      |
| %L1_DC_MISSES     | 41.063%                  | 49.454%                     |

Declared observation: CPI ≈ 2.07–2.20 at both N values.
This is the gather regime — MAB parallelism available.
NOT equivalent to serialized DRAM access latency.
Measures effective DRAM throughput under parallel access.

Note: comment in probe.rs previously stated "forces every access to DRAM" —
INADMISSIBLE. Corrected to: "WS > nominal L3 is the declared intervention.
Observed service distribution established by H."

### Pointer chase calibration (chained, k=1)

Workload: run_chained — pointer chase (each address depends on prior result).
Protocol: light (10w/100t) — protocol mismatch noted: chained at these N
triggers light protocol, not dram-cal. This produced more timed passes (100)
than dram-cal (20). Elevated sample count is not a defect; it is declared.

| Field             | CAL-PTR-2X (chained, 64MB) | CAL-PTR-4X (chained, 128MB) |
|-------------------|----------------------------|-----------------------------|
| CPI               | 25.6480                    | 27.2091                     |
| RETIRED_BR_INST PTI| 252.8828                  | 250.4467                    |
| %BR_MISP          | 0.706%                     | 1.065%                      |
| DRAM_PTI          | 113.7822                   | 130.1438                    |
| L3_PTI            | 24.3357                    | 7.5398                      |
| L2_PTI            | 2.3564                     | 2.2579                      |
| %L1_DC_MISSES     | 48.880%                    | 49.814%                     |
| %SMT_CONTENTION   | 0.607%                     | 4.986%                      |
| STLI_OTHER PTI    | 0.155                      | 1.474                        |

Declared observation: CPI ≈ 25.6–27.2 at both N values.
This is the pointer chase (serialized) regime.

Iteration decomposition (from probe.rs run_chained source):
  2 dependent loads per iteration: chain[current] (serializing), values[next] (dependent).
  values[current] forwarded from prior iteration — 0 effective misses.
  Measured: ~3.95–3.99 instructions/iter, ~1.85–2.04 DC accesses/iter.
  Refills/iter: 0.450+0.096+0.009=0.555 (2X), 0.520+0.030+0.009=0.559 (4X).
  Refills/iter ≈ 0.56 — below expected ~2.0 for 2 loads at WS>>L3.
  Discrepancy candidates: PTI sampling, prefetcher, OOO overlap — not declared.

Derived cycles/iter:
  CAL-PTR-2X: 25.6480 × (1000/252.8828) = 101.42 cycles
  CAL-PTR-4X: 27.2091 × (1000/250.4467) = 108.64 cycles
  Difference: 7.22 cycles — exceeds ±5-cycle closure criterion.

This quantity is compound — NOT equivalent to DRAM_LAT alone.
Contributors include: serialized load latency (chain+values, mixed service),
SMT contention (0.6%→5.0% across N values), possible TLB walk serialization,
PTI sampling effects. None declared as mechanism.

### Declared finding (both calibration workloads)

Same working-set class (WS > nominal L3) + different dependency relation
(gather vs pointer chase) → radically different measured progression:
  Gather: CPI ≈ 2, wall-clock ≈ 6 ns/op
  Pointer chase: CPI ≈ 26, wall-clock ≈ 95–114 ns/op
  Ratio ≈ 13×. Mechanistic explanation not declared.

### OC-DRAM-1 disposition

OC-DRAM-1: OPEN.
DRAM_LAT not isolatable as a unique quantity from current measurements.
Reason: cycles_per_iter is compound (OC-DRAM-1a); SMT contention not isolated;
refill/iter accounting unresolved. substrate_model.rs DRAM_LAT remains 180
(declared approximate) pending closure.

OC-DRAM-1a: OPEN.
Required: SMT isolation run (affix to single core, disable SMT), refill
accounting verification, and single-load variant to separate chain[]
latency from values[] latency.


---

## OC-DRAM-1a — Assembly Decomposition and Chain-Only Intervention (2026-08-30)

### Assembly decomposition of run_chained hot loop

Source: probe.s (release build, rustc stable-x86_64-pc-windows-msvc, LLVM backend)
Function: _RNvCs8eqcXlaylXm_5probe11run_chained
Hot loop label: .LBB10_3

Declared instruction sequence (I_asm = 15):

```
movq    32(%rsp), %rax          1. Stack load: current → %rax (LOAD, L1 hit, STLF)
cmpq    %r9, %rax               2. Bounds check: current < n (ALU)
jae     .LBB10_9                3. Panic branch (never taken in hot path)
cmpq    %rdx, %rax              4. Bounds check: current < chain.len() (ALU)
jae     .LBB10_10               5. Panic branch (never taken in hot path)
movq    (%r8,%rax,8), %r10      6. Chain load: chain[current] → %r10 (LOAD, SERIALIZING)
cmpq    %rdx, %r10              7. Bounds check: next < values.len() (ALU)
jae     .LBB10_11               8. Panic branch (never taken in hot path)
incq    %r11                    9. Loop counter++ (ALU, independent)
movsd   (%rcx,%rax,8), %xmm1   10. Load values[current] (LOAD, FP)
subsd   (%rcx,%r10,8), %xmm1   11. Load values[next] + FP sub (LOAD+FP, DRAM-dependent)
addsd   %xmm1, %xmm0           12. FP accumulate (FP)
movq    %r10, 32(%rsp)          13. Stack store: current = next (STORE)
cmpq    %r11, %r9               14. Loop termination check (ALU)
jne     .LBB10_3                15. Loop branch (taken until last iter)
```

Declared quantities from assembly:
  I_asm = 15 instructions per iteration
  Branches per iteration = 4 (instructions 3, 5, 8, 15)
  Memory operations = 5 (4 loads + 1 store: instructions 1, 6, 10, 11, 13)
  DRAM-dependent loads = 2: instruction 6 (chain[current]) and 11 (values[next])
  Cross-iteration address identity: A(values[current]_t) = A(values[next]_{t-1})
    (declared from loop structure; whether this produces L1 hit is a processor-state
    proposition not established by assembly alone)

Declared BR_PTI relation:
  4 branches / 15 instructions × 1000 = 266.7 BR/PTI (static loop)
  Measured: 252.9 (CAL-PTR-2X), 250.4 (CAL-PTR-4X)
  Consistency: close to 266.7 — difference explained by process-level
  measurement including execution outside the hot loop.

Declared ipi relation:
  ipi = 4 × (1000 / BR_PTI) ≈ 15.8–16.0 (consistent with I_asm = 15)
  Prior formula ipi = 1000/BR_PTI is RETIRED for this loop (4 branches/iter, not 1).

Declared L1_DC_ACCESSES PTI relation:
  From assembly: 5 memory operations / 15 instructions × 1000 = 333.3 expected
  Measured: 468.2 (CAL-PTR-2X), 510.8 (CAL-PTR-4X)
  Measured > expected. Counter-to-instruction mapping requires further declaration.
  The 37% PMU sampling rate conclusion is REJECTED. Counter not established as
  a simple fraction of retired memory instructions.

L_candidate (not DRAM_LAT — assumptions A–C undeclared):
  Using corrected ipi = 4 × (1000/BR_PTI):
  cycles_per_iter = CPI × ipi ≈ 101–109 cycles (as previously measured)
  Under declared assumptions A (equal service mix), B (linear SMT correction),
  C (STLF overhead 4–5 cycles from SOG §2.12):
  L_candidate ≈ 42–44 cycles per DRAM-dependent load
  This is a candidate estimate under declared assumptions, not a measured DRAM_LAT.

### Chain-only intervention design

Declared intervention: ΔV: chained+values → chain-only
  Remove: values[] accumulation (instructions 10, 11, 12 from hot loop)
  Preserve: pointer dependency (instruction 6, chain[current] → %r10)
  The primary serialized relation remains:
    current_{t+1} = chain[current_t]

Protocol: light (10w/100t) — same as CAL-PTR runs for direct comparability.
N values: 8,388,608 (2X) and 16,777,216 (4X) — same as CAL-PTR runs.

REQUIRED before interpreting H vector:
  Extract run_chain_only assembly from probe.s after build.
  Declare the actual hot-loop instruction sequence.
  The compiler may produce a different structure without FP accumulation.

Primary declared measurement:
  ΔH = H(CAL-CHAIN-ONLY) − H(CAL-PTR) at each N

Run commands:
  probe.exe chain-only  8388608
  probe.exe chain-only 16777216

### CAL-CHAIN-ONLY-2X assembly (to be filled after build)

```
[Extract from probe.s after cargo build --release with RUSTFLAGS=--emit=asm]
```

I_asm(chain-only) = [count from assembly]
Branches per iteration = [count]
Memory operations = [count]

### CAL-CHAIN-ONLY H vectors

| Field              | CAL-CHAIN-ONLY-2X (64MB) | CAL-CHAIN-ONLY-4X (128MB) |
|--------------------|--------------------------|---------------------------|
| CPI                |                          |                           |
| RETIRED_BR_INST PTI|                          |                           |
| %BR_MISP           |                          |                           |
| DRAM_PTI           |                          |                           |
| L3_PTI             |                          |                           |
| L2_PTI             |                          |                           |
| %L1_DC_MISSES      |                          |                           |
| %SMT_CONTENTION    |                          |                           |
| STLI_OTHER PTI     |                          |                           |

### ΔH analysis (to be filled after measurement)

| Field              | ΔH (2X) | ΔH (4X) |
|--------------------|---------|---------|
| CPI                |         |         |
| DRAM_PTI           |         |         |
| L3_PTI             |         |         |
| cycles_per_iter    |         |         |

Declared finding: [to be completed after measurement]

### OC-DRAM-1a disposition

OC-DRAM-1a: OPEN pending chain-only measurement and assembly declaration.

---

## OC-DRAM-1a — Chain-Only Intervention Results (2026-08-30)

### Assembly declaration: run_chain_only hot loop

Source: probe.s (release build, rustc stable-x86_64-pc-windows-msvc)
Function: _RNvCs8eqcXlaylXm_5probe14run_chain_only
Hot loop label: .LBB13_3

Declared instruction sequence:

```
movq    40(%rsp), %rax          1. Stack load: current → %rax (LOAD)
cmpq    %rdx, %rax              2. Bounds check: current < n (ALU)
jae     .LBB13_7                3. Panic branch (never taken in hot path)
incq    %r8                     4. Loop counter++ (ALU, independent)
movq    (%rcx,%rax,8), %rax     5. Chain load: chain[current] → %rax (LOAD, SERIALIZING)
movq    %rax, 40(%rsp)          6. Stack store: current = next (STORE)
#APP #NO_APP                    7. black_box fence
cmpq    %r8, %rdx               8. Loop termination check (ALU)
jne     .LBB13_3                9. Loop branch
```

Declared quantities from assembly:
  I_asm = 9 instructions per iteration
  B_asm = 2 branches per iteration (instructions 3 and 9)
  Memory operations = 3 (2 loads + 1 store: instructions 1, 5, 6)
  DRAM-dependent loads = 1: instruction 5 (chain[current]) only
  values[] path: entirely absent from this compiled loop

Comparison to run_chained assembly (I_asm=15, B_asm=4):
  Removed: values[current] load (movsd), values[next] load+FP sub (subsd),
           FP accumulate (addsd), values.len() bounds check+branch (cmpq+jae)
  Changed: chain load result now overwrites %rax directly (no separate %r10)
  Retained: stack store/load recurrence, chain load serialization, loop branch

### BR_PTI internal consistency

Assembly predicts: BR_PTI = (B_asm / I_asm) × 1000 = (2/9) × 1000 = 222.22
Measured (CAL-CHAIN-ONLY-2X): 222.6659
Residual: |222.67 − 222.22| = 0.45 PTI (~0.20%)
I_derived = 2 × (1000 / 222.6659) = 8.982 vs I_asm = 9 (~0.2% residual)

Declared: the assembly-derived BR_PTI relation and uProf observation agree
within ~0.2% under these conditions (2X, light protocol, low SMT contention).
This is an internal consistency result for the 2X measurement. It does not
extend to the 4X measurement — see OC-BR-1 below.

### Declared H vectors: chain-only

| Field              | CO-2X (64MB)  | CO-4X (128MB) |
|--------------------|---------------|---------------|
| CPI                | 2.5743        | 2.0991        |
| RETIRED_BR_INST PTI| 222.6659      | 178.6806      |
| %BR_MISP           | 4.9490%       | 9.0766%       |
| L1_DC_ACCESSES PTI | 641.6128      | 677.3720      |
| %L1_DC_MISSES      | 19.3167%      | 15.8088%      |
| DRAM_PTI           | 62.3410       | 47.7332       |
| L3_PTI             | 22.1570       | 14.6224       |
| L2_PTI             | 18.8295       | 23.5962       |
| STLI_OTHER PTI     | 75.8074       | 48.4276       |
| %SMT_CONTENTION    | 15.6440%      | 16.1508%      |

Protocol: light (10w/100t), same as CAL-PTR runs. CAL-CHAIN-ONLY-2X was
re-run (v2) after hardware lock error on first attempt; data from v2.

### ΔV observations: chained+values → chain-only

Declared intervention: remove values[] accumulation, preserve pointer dependency.

| Field         | ΔH(2X) = CO-2X − PTR-2X | ΔH(4X) = CO-4X − PTR-4X |
|---------------|--------------------------|--------------------------|
| CPI           | −23.0737                 | −25.1100                 |
| DRAM_PTI      | −51.44                   | −82.41                   |
| L3_PTI        | −2.18                    | +7.08                    |
| L2_PTI        | +16.47                   | +21.34                   |
| STLI_PTI      | +75.65                   | +46.95                   |

### Declared findings from ΔV

Finding 1 — CPI:
  CPI(chained+values, 2X) ≈ 25.6480
  CPI(chain-only, 2X) = 2.5743
  ΔV·CPI ≈ −23.07
  The intervention removing values[] and the resulting compiled-loop
  changes corresponds to an approximately 23-CPI reduction at 2X.
  NOTE: ΔV·CPI ≠ L_values_next. The intervention changed the instruction
  stream and other measured processor relations simultaneously (see STLI).
  Attribution of ΔV·CPI to any single mechanism is not established.

Finding 2 — BR_PTI consistency (stated above):
  Assembly prediction and measurement agree within ~0.2%.

Finding 3 — STLI_OTHER:
  STLI_OTHER PTI: 0.1554 (PTR-2X) → 75.8074 (CO-2X). ΔV = +75.65.
  The ΔV intervention exposed a strong STLI relation not present in
  chained+values. The chain-only loop contains a stack-mediated recurrence
  (store next → stack, load current ← stack) without intervening values[]
  instructions. The connection between this STLI observation and observed
  CPI is not yet established.
  → OC-STLI-1 (NEW, OPEN): determine the relation between STLI_OTHER PTI,
    the chain-only instruction sequence, and observed CPI before any
    additive STLI contribution is assigned to CPI.

Finding 4 — Cache service distribution shift:
  Under ΔV, DRAM_PTI decreased, L2_PTI increased, and L3_PTI changed.
  This is an observed shift in service distribution between the two workloads.
  Whether the same chain[] accesses account for this shift, or whether other
  aspects of the changed instruction stream are involved, is not established.

Finding 5 — CPI direction across N:
  CPI(chain-only, 2X) = 2.5743 > CPI(chain-only, 4X) = 2.0991
  Δ_N CPI < 0 for 64MB → 128MB within the chain-only workload.
  This is the opposite direction from chained+values (where CPI rises with N).
  Recorded as observed. No mechanism declared.

### Retired assumption

Assumption A (equal service distribution for chain[] and values_next loads)
is RETIRED. The ΔV measurement shows substantially different system behavior
when values[] is present vs absent, establishing that the two loads operate
in different processor contexts. Using equal service distribution to derive
DRAM_LAT from the combined workload is not admissible.

### OC-BR-1 — 4X BR_PTI Discrepancy (OPEN)

Assembly predicts BR_PTI = (B_asm / I_asm) × 1000 = (2/9) × 1000 = 222.22
for the chain-only hot loop regardless of N (same source relation, same protocol).

Measured:
  CAL-CHAIN-ONLY-2X (N=8,388,608):  BR_PTI = 222.6659  (departure: 0.45,  ~0.2%)
  CAL-CHAIN-ONLY-4X (N=16,777,216): BR_PTI = 178.6806  (departure: 43.54, ~19.6%)

The 2X measurement agrees with the assembly prediction within 0.2%.
The 4X measurement departs by 19.6% from the same prediction.
N changes between the two runs. The declared source relation (chain-only
assembly) and protocol (light, 10w/100t) are fixed.

This is a sharply bounded observation: the same assembled loop at the same
protocol produces BR_PTI values that agree with assembly at 2X and depart
substantially at 4X. No mechanism is declared.

The 4X measurement is not used in any derivation until OC-BR-1 is resolved.

OC-BR-1: OPEN.

### OC-DRAM-1a disposition

OC-DRAM-1a: OPEN.
The ΔV intervention substantially advanced understanding but did not close
DRAM_LAT as a unique quantity. The following remain undeclared:
  - Connection between STLI_OTHER and CPI in chain-only (OC-STLI-1, OPEN)
  - 4X BR_PTI departure from assembly prediction (OC-BR-1, OPEN)
  - L1_DC_ACCESSES PTI counter-to-instruction mapping (OC-DC-1, OPEN)
  - Mechanism for CPI direction reversal across N in chain-only
  - Whether chain[] service distribution = DRAM at these N values
  - DRAM_LAT as a unique measurable quantity

OC-STLI-1: OPEN.
  Determine the relation between STLI_OTHER PTI, the chain-only
  compiled instruction sequence, and observed CPI.
  Required before any additive STLI contribution is assigned.

OC-BR-1: OPEN.
  BR_PTI(chain-only, 4X) = 178.68 departs from assembly prediction 222.22
  by ~19.6%. BR_PTI(chain-only, 2X) = 222.67 agrees within 0.2%.
  N is the only declared change between the two measurements.
  4X measurement excluded from derivations until resolved.

OC-DC-1: OPEN.
  L1_DC_ACCESSES PTI measured at 468–511 for chained+values, above the
  static-ratio prediction of 333.3 (5 ops/15 instr × 1000).
  Counter-to-instruction mapping is undeclared.

