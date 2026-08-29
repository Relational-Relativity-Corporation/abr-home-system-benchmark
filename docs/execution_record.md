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
