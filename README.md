# abr-home-system-benchmark

**ABR operator timing on Ryzen 5 7600X / DDR5-5600 — Metatron Dynamics, Inc.**

Bounded over D. No claim beyond D.

Measures wall-clock time per A -> B -> R pass over a declared graph with
working set <= 1 MB, resident in L3 cache (32 MB, Zen 4), on a consumer
desktop system. All operators declared from and consistent with kernel
operators.rs V7 -- Metatron Dynamics (ABR formulas lines 890-988).

Provides the grounding measurement for OC-IF-5 in abr-infinity-fabric:
confirms that V7 ABR execution cost scales with approximately constant
cost per declared relation on this hardware, for the declared open-chain
topology and tested edge range. Reproduced across two independent runs.

## Declared Hardware

- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2x 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit

## Declared Operators (kernel V7)

- A(x)[e] = x[source(e)] - x[target(e)]
- B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]  (immediate successor inputs)
- rho[i] = rho_base * chi[i] / (1 + chi[i])  (node-indexed, rho_base=1.0)
- R(g)[e] = g[e] + rho[src(e)] * (Σ_succ g[f] - Σ_pred g[p])

## Version History

### V0.1 -- superseded
Four heap allocations per pass. B recursive (non-V7). rho edge-local (non-V7).
113,927 ns/pass. Epistemic status: implementation cost, non-V7 operators.

### V0.2 -- superseded
Allocation removed. B and rho still non-V7.
35,349 ns/pass, ~4.5 ns/edge. Epistemic status: non-V7 operators.

### V0.3 -- current
Operators corrected to match kernel V7 exactly.
B: immediate-successor input values. rho: node-indexed with rho_base.
24/24 tests passing. Two independent runs recorded.

Run 1: 30,776 ns/pass, 32,493 analyses/second, MI355X ratio 234.8x.
Run 2: 28,922 ns/pass, 34,575 analyses/second, MI355X ratio 220.7x.
Run-to-run variation ~6% -- consistent with OC-HB-2 (OS scheduling).
Epistemic status: V7 ABR operator traversal cost. Admissible.

## Key Finding -- Scaling Measurement (V0.3)

V7 ABR compute time scales with approximately constant cost per declared
relation for the declared open-chain topology on this hardware.
Reproduced across two independent runs on 2026-08-08.

### Run 1
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,862     | 3.775   | 3,100    |
| 2,047   | 48.0    | 7,734     | 3.778   | 6,100    |
| 4,095   | 96.0    | 14,792    | 3.612   | 12,500   |
| 8,191   | 192.0   | 31,050    | 3.791   | 25,100   |
| 16,383  | 384.0   | 62,997    | 3.845   | 52,900   |
NS/EDGE ratios: 1.0008, 0.9561, 1.0494, 1.0144

### Run 2
| N_EDGES | WS (KB) | MEAN (ns) | NS/EDGE | MIN (ns) |
|---------|---------|-----------|---------|----------|
| 1,023   | 24.0    | 3,509     | 3.430   | 3,200    |
| 2,047   | 48.0    | 7,040     | 3.439   | 6,400    |
| 4,095   | 96.0    | 14,105    | 3.445   | 13,100   |
| 8,191   | 192.0   | 27,883    | 3.404   | 26,500   |
| 16,383  | 384.0   | 59,770    | 3.648   | 55,100   |
NS/EDGE ratios: 1.0026, 1.0015, 0.9883, 1.0717

NS/EDGE range across both runs: 3.404-3.845 ns/edge. Approximately
constant per-edge cost confirmed across both runs. Binding mechanism
not identified. OC-HB-4 remains open: operator isolation required.

## Declared Compute Relationship

For the declared benchmark topology and tested range on this hardware:

  ABR compute time scales approximately as n_declared_relations x ~3.4-3.8 ns/edge

Measured over 1,023-16,383 edges under the declared warm-pass protocol.
Hardware-specific and declaration-specific. Does not generalize to
arbitrary ABR declarations or graph topologies without further measurement.

Application to declared graphs outside this range or with different
topology is a structural extrapolation -- not a direct measurement from
this repo. See docs/M_declaration.md for the full extrapolation note.

## Build and Run

cargo build --release
cargo test
cargo run --release

## Test Results

24/24 tests passing (V0.3). See docs/execution_record.md for full output
including both independent runs.

## Open Conditions

- OC-HB-1: L3 bandwidth not directly measured
- OC-HB-2: L3 residency via warm-pass protocol only
- OC-HB-3: MI355X ratio mixed epistemic (structural vs measured)
- OC-HB-4: Binding mechanism not identified -- operator isolation open

## Grounding Documents

- docs/M_declaration.md -- full declaration including both run results
- docs/execution_record.md -- all cargo test and run --release outputs
- kernel operators.rs V7 -- ABR operator declaration (lines 890-988)
- abr-infinity-fabric -- OC-IF-5 references this repo

*Metatron Dynamics, Inc. -- Lompoc, California*
*Bounded over D. No claim beyond D.*
