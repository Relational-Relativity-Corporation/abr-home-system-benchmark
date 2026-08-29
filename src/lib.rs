// lib.rs — Metatron Dynamics, Inc. V1.1
// abr-home-system-benchmark: relational (ABR) vs standard binary/von Neumann
// substrate comparison on Ryzen 5 7600X / DDR5-5600.
// Bounded over D. No claim beyond D.
//
// V1.0 expanded scope from a single ABR-operator timing benchmark (V0.3,
// Regime 1) to a three-regime comparison. V1.1 widens Regime 3 from one
// binary baseline (all-pairs, O(N^2)) to a five-algorithm matrix spanning
// O(N), O(N log N), and O(N^2) — see binary_baselines.rs and OC-CC-2.
//
//   Regime 1 — operators.rs / scaling.rs / throughput_derivation.rs / timing_harness.rs
//     ABR operator cost in isolation (unchanged since V0.3). Control data.
//   Regime 2 — process_topology.rs
//     OS-to-CPU exchange layer: applies V7 operators to a real process
//     activation trace.
//   Regime 3 — complexity_crossover.rs + binary_baselines.rs (V1.1)
//     Task-complexity matrix: five declared binary algorithms vs the
//     relational ABR chain, across three declared tiers.

pub mod declared_graph;
pub mod operators;
pub mod timing_harness;
pub mod throughput_derivation;
pub mod scaling;
pub mod process_topology;
pub mod binary_baselines;
pub mod complexity_crossover;
pub mod cache_latency_model;
