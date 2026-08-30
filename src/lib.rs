// lib.rs — Metatron Dynamics, Inc. V2.0
// abr-home-system-benchmark: relational (ABR) vs standard binary/von Neumann
// substrate comparison on Ryzen 5 7600X / DDR5-5600.
// Bounded over D. No claim beyond D.
//
// ── Version History ───────────────────────────────────────────────────────────
//
// V0.1-V0.3: Regime 1 only — ABR operator timing, allocation removal,
//   V7 operator corrections (B immediate-successor, rho node-indexed).
//
// V1.0: Three-regime expansion. Regime 2 (process topology). Regime 3
//   (task-complexity crossover, single O(N^2) binary baseline).
//
// V1.1: Regime 3 widened to 5-algorithm matrix spanning O(N), O(N log N),
//   O(N^2). OC-CC-2 narrowed.
//
// V1.2/V1.3: Mechanism-isolating baselines added (ScrambledAccess,
//   BranchyDataDependent). Cache-latency characterization module added
//   (cache_latency_model.rs). Declared-hardware runs obtained for all
//   three regimes plus cache-latency curve. OC-CC-3, OC-CL-1/2/3 closed.
//
// V2.0: Regime 3 tiers grounded in declared hardware observables.
//   V1.x tiers (SMALL=64, MEDIUM=1024, LARGE=8192) were undeclared —
//   all working sets fit in L2, so SCRAMBLED/BRANCHY never engaged their
//   declared mechanisms. V2.0 derives tiers from the cache-latency curve
//   (declared-hardware run 2026-08-29):
//
//     SMALL:   N=64        L1d  0.56 ns/access  (baseline)
//     MEDIUM:  N=1,024     L1d  0.56 ns/access  (V1.x continuity)
//     LARGE:   N=65,536    L2 saturation 0.70 ns/access
//     XLARGE:  N=262,144   L3 entry      0.88 ns/access
//     XXLARGE: N=2,097,152 L3-internal step 3.38 ns/access
//
//   ALL_PAIRS excluded at LARGE/XLARGE/XXLARGE (O(N^2) intractable).
//   XLARGE/XXLARGE use lighter timing protocol (OC-CC-4).
//   OC-CC-1 addressed. 65 tests passing (sandbox verification 2026-08-29).
//
// ── Regimes ───────────────────────────────────────────────────────────────────
//
//   Regime 1 — operators.rs / scaling.rs / throughput_derivation.rs / timing_harness.rs
//     ABR operator cost in isolation. Control data. Unchanged since V0.3.
//
//   Regime 2 — process_topology.rs
//     OS-to-CPU exchange layer: V7 operators applied to a real process
//     activation trace. OC-PT-1/2/3 open.
//
//   Regime 3 — complexity_crossover.rs + binary_baselines.rs
//     Hardware-grounded crossover matrix: 7 binary algorithms vs ABR chain
//     at 5 tiers spanning L1d through the L3-internal congestion step.

pub mod declared_graph;
pub mod operators;
pub mod timing_harness;
pub mod throughput_derivation;
pub mod scaling;
pub mod process_topology;
pub mod binary_baselines;
pub mod complexity_crossover;
pub mod cache_latency_model;
pub mod transition_gradient;

pub mod substrate_model;
