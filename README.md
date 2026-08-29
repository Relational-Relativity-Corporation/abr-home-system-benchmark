# abr-home-system-benchmark

**Relational (ABR) vs standard binary/von Neumann substrate comparison —
Ryzen 5 7600X / DDR5-5600 — Metatron Dynamics, Inc.**

Bounded over D. No claim beyond D.

## V1.0 Scope

V0.3 measured one thing: ABR operator (A -> B -> R) wall-clock cost on a
declared synthetic graph. That measurement is preserved unchanged as
**Regime 1** below — it is real, reproduced-twice data and remains the
control/floor cost for everything else in this repo.

V1.0 expands the question from "what does an ABR pass cost on this
hardware" to "how does a relational approach to information handling on
this chip compare to the standard binary/Boolean approach the chip was
designed around, at three different layers":

| Regime | Layer | Module | Status |
|---|---|---|---|
| 1 | ABR operator cost (control data) | `operators.rs`, `scaling.rs`, `throughput_derivation.rs` | Measured, reproduced 3x (V0.3 x2, V1.0 x1) |
| 2 | OS-to-CPU process exchange layer | `process_topology.rs` | Structural scaffold; real idle/utilization data not yet ingested (OC-PT-1) |
| 3 | Task-complexity crossover matrix (5 binary algos) | `complexity_crossover.rs`, `binary_baselines.rs` | Sandbox run complete (V1.1); declared-hardware confirmation pending |

Gate-level Boolean logic itself (the physical transistor layer) is **not**
a regime here — logic gates are physical circuit structure, not code, and
are declared out of scope for a software-level comparison. Regime 2 is the
lowest software-addressable layer above that physical floor.

## Declared Hardware

- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2x 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit

## Regime 1 — ABR Operator Cost (unchanged from V0.3)

### Declared Operators (kernel V7)

- A(x)[e] = x[source(e)] - x[target(e)]
- B(g)[e] = g[e] + Σ_{f ∈ succ(e)} g[f]  (immediate successor inputs)
- rho[i] = rho_base * chi[i] / (1 + chi[i])  (node-indexed, rho_base=1.0)
- R(g)[e] = g[e] + rho[src(e)] * (Σ_succ g[f] - Σ_pred g[p])

### Key Finding — Scaling Measurement (V0.3, preserved)

V7 ABR compute time scales with approximately constant cost per declared
relation for the declared open-chain topology on this hardware.
Reproduced across two independent runs on 2026-08-08.

NS/EDGE range across both runs: 3.404-3.845 ns/edge. Binding mechanism
not identified — OC-HB-4 remains open.

Full run tables: see `docs/M_declaration.md`.

### Open Conditions (Regime 1)

- OC-HB-1: L3 bandwidth not directly measured
- OC-HB-2: L3 residency via warm-pass protocol only
- OC-HB-3: MI355X ratio mixed epistemic (structural vs measured)
- OC-HB-4: Binding mechanism not identified — operator isolation open

## Regime 2 — OS-to-CPU Process Exchange Layer

Applies the same V7 operators to a real process-activation trace instead
of a synthetic graph. Processes are declared as loci; the current
observable is activation-time offset only (a proxy — see OC-PT-1).
Edges are declared between temporally consecutive processes whose start
times fall within a declared co-activation window (OC-PT-2).

`example_session_trace()` provides a real, hand-transcribed 26-process
trace captured from an AMD uProf "Select Profile Target" screen
(2026-08-28) so this module has real data to build and test against
before a full uProf CSV export pipeline is wired up.

**This module currently answers "can V7 operators run over a real process
trace" (yes — confirmed by test), not "does doing so reduce redundant
OS-to-CPU switching."** That second, actual claim requires ingesting real
idle/active CPU utilization per process from uProf hotspot sampling, which
is not yet wired up.

### Open Conditions (Regime 2)

- OC-PT-1: Only activation-timestamp observable ingested. Idle/active
  utilization telemetry (uProf hotspot data, not the process list) is
  required to test the actual efficiency question under discussion.
- OC-PT-2: Co-activation window (2.0s) is declared, not derived.
- OC-PT-3: No comparison against actual OS scheduler behavior (context
  switches, redundant wake events) has been made yet.

## Regime 3 — Task-Complexity Crossover Matrix (V1.1)

Tests whether the quadratic-vs-linear crossover found in the language
model / token-count case (abr-relational-attention: crossover near five
paragraphs) reproduces for a generic hardware-level task — and, as of
V1.1, whether V1.0's finding was a genuine relational-structure effect or
an artifact of comparing against a single poor-scaling binary baseline.

V1.0 tested one binary baseline: all-pairs difference, O(N²). It found
relational cheaper at every declared tier (confirmed on declared hardware,
2026-08-29 — see `docs/M_declaration.md`), with no crossover in range —
OC-CC-1. But OC-CC-2 flagged that this alone couldn't distinguish "ABR is
genuinely efficient" from "any O(N) algorithm beats any O(N²) algorithm."

V1.1 (`binary_baselines.rs`) adds four more binary algorithms — a plain
linear scan (O(N)), a running prefix sum (O(N)), a bounded sliding-window
comparison (O(N·K), a more realistic conventional pattern than all-pairs),
and a sort-then-scan (O(N log N)) — and times all five against the same
relational ABR chain.

**Sandbox result (2026-08-28, not yet confirmed on declared hardware —
see OC-CC-3): relational wins ONLY against the O(N²) all-pairs baseline.**
Against the O(N) and O(N log N) baselines, binary wins outright, and the
margin widens with N (linear scan and prefix sum: relational runs 4.6x to
8.7x SLOWER, not faster, at LARGE). The bounded-window baseline sits near
parity, with binary slightly ahead at most tiers.

This is consistent with V1.0's finding having been a complexity-class
artifact rather than a genuine relational-structure advantage. **This
result requires confirmation on the declared hardware (Ryzen 5 7600X)
before being treated as admissible over D** — run `cargo run --release`
and see `docs/M_declaration.md` V1.1 addendum for the full table and next
steps once that run is captured.

### Open Conditions (Regime 3)

- OC-CC-1: Tier sizes declared, not derived. Confirmed (declared
  hardware, V1.0) no crossover exists against ALL_PAIRS in this range.
  Still open for the four algorithms added in V1.1.
- OC-CC-2: (narrowed in V1.1) — five binary algorithms across three
  complexity classes tested, not one. Still a declared representative
  set, not exhaustive (OC-BB-2).
- OC-CC-3: All timing must be run on the declared hardware to be
  admissible. Sandbox/CI timings are structural sanity checks only.
- OC-BB-1 (binary_baselines.rs): WINDOW_K=8 is declared, not derived.
- OC-BB-2 (binary_baselines.rs): five baselines are representative, not
  an exhaustive survey of binary algorithms.

## Build and Run

    cargo build --release
    cargo test
    cargo run --release

## Test Results

41/41 tests passing (V1.0): 24 from Regime 1 (unchanged), 10 from Regime
2, 6 from Regime 3, 1 shared graph test. See `docs/execution_record.md`.

## Grounding Documents

- `docs/M_declaration.md` — full declaration, Regime 1 run tables, V1.0 addendum
- `docs/execution_record.md` — cargo test / cargo run outputs
- kernel `operators.rs` V7 — ABR operator declaration (lines 890-988)
- `abr-infinity-fabric` — OC-IF-5 references this repo's Regime 1 measurement (unaffected by V1.0 expansion — Regime 1 code and data are unchanged)

*Metatron Dynamics, Inc. — Lompoc, California*
*Bounded over D. No claim beyond D.*
