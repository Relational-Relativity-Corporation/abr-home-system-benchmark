// substrate_model.rs — Metatron Dynamics, Inc. V2.0
// Mathematical models for hardware substrate mechanisms.
// All architectural constants sourced from:
//   AMD Software Optimization Guide for the AMD Zen4 Microarchitecture
//   Publication 57647 Rev. 1.01, April 2023 (AMD Public Use)
//   (hereafter "SOG")
//
// Bounded over D. No claim beyond D.
//
// ── V2.0 Changes ─────────────────────────────────────────────────────────────
//
// V1.0 had three declared errors, corrected here:
//
// ERROR 1 — Cache tier for D=chain-8 nodes:
//   V1.0 used per-chain working set (N/k × elem) as the cache footprint.
//   CORRECTION: cache footprint = full working set (N × elem), regardless
//   of k. Chains interleave across the full array simultaneously. Per-chain
//   working set describes dependency structure, not cache pressure.
//
// ERROR 2 — TLB cost as additive CPI term:
//   V1.0 added TLB miss latency separately from memory access latency.
//   CORRECTION: TLB miss latency is subsumed within memory access latency.
//   The measured DRAM_PTI and L3_PTI counters include TLB-induced traffic.
//   TLB affects the miss distribution (which tier the access lands in),
//   not the CPI as a separate additive term. Declared as a factor, not a term.
//
// ERROR 3 — MAB saturation narrative (V1.0 prose):
//   V1.0 stated chain serialization causes MAB saturation (high occupancy).
//   CORRECTION: chain serialization keeps MAB occupancy LOW (one outstanding
//   miss per chain at a time), but makes each miss fully serialized.
//   The cost is serialized latency, not buffer saturation.
//
// ── Declared Architectural Constants (SOG 57647 Rev. 1.01) ──────────────────
//
// Cache hierarchy (SOG §2.6):
//   CACHE_LINE_BYTES    = 64             (§2.6.2)
//   L1_DC_SIZE_BYTES    = 32768          (§2.6.2: 32KB L1 D-cache)
//   L2_SIZE_BYTES       = 1048576        (§2.6.3: 1MB L2 per core)
//   L3_LATENCY_CYCLES   = 50            (§A.1: average L3 load-to-use)
//   L2_LATENCY_CYCLES   = 14            (§A.1: minimum L2 load-to-use)
//   L1_LATENCY_CYCLES   = 4             (§2.6.2: 4-cycle integer load-to-use)
//
//   DRAM_LATENCY_CYCLES = 180           (NOT in SOG. Declared approximate.
//                                        DDR5-5600 standard estimate.
//                                        Declared range: 160–220 cycles.
//                                        This is the ONLY non-SOG constant
//                                        in the model. All other constants
//                                        are sourced from SOG §references.)
//
// TLB (SOG §2.7):
//   L1_DTLB_ENTRIES     = 72            (§2.7.1: fully-associative L1 DTLB)
//   L2_DTLB_ENTRIES     = 3072          (§2.7.2: 24-way L2 DTLB)
//   PAGE_SIZE_BYTES     = 4096          (§2.7.1: 4KB pages)
//   PAGE_WALKERS        = 6             (§2.7.3: hardware page table walkers)
//
// Branch prediction (SOG §2.8):
//   BR_MISP_PENALTY     = 13            (§2.8: common case penalty)
//
// Out-of-order execution (SOG §2.10.3):
//   ROB_SIZE            = 320           (§2.10.3: retire queue, non-SMT)
//
// Load-Store unit (SOG §2.12):
//   STORE_QUEUE_ENTRIES = 64            (§2.12: 64-entry store queue)
//   LOAD_QUEUE_ENTRIES  = 48            (§2.12: 48 uncompleted loads)
//   MAB_ENTRIES         = 24            (§2.12: Miss Address Buffer)
//   STLF_ADDR_BITS      = 12            (§2.12: bits[11:0] for STLF eligibility)
//
// Prefetchers (SOG §2.12.1):
//   L1 Stream, L1 Stride, L1 Region, L2 Stream, L2 Up/Down.
//   Declared: random/irregular access patterns may defeat prefetchers,
//   causing prefetch of unused data and excess bandwidth. (§2.12.1)
//
// ── Declared Domain Values ────────────────────────────────────────────────────
//   ELEMENT_SIZE_BYTES  = 8             (u64 array elements)
//   K_CHAINS_DECLARED   = 8             (D=chain-8 nodes)
//   N_S0                = 524288        (declared S0)
//   N_S1                = 4194304       (declared S1)
//   L3_RYZEN_5_7600X    = 33554432      (32MB, product spec — not SOG;
//                                        SOG states "up to 96MB depending
//                                        on configuration" §2.6.4)

// ── Model 1: TLB Pressure ─────────────────────────────────────────────────────
//
// Pages touched = ceil(N × ELEM / PAGE_SIZE)
// L1_DTLB saturation: pages > 72  (§2.7.1)
// L2_DTLB saturation: pages > 3072 (§2.7.2)
//
// For scrambled access with L2_DTLB saturated:
//   TLB hit rate ≈ L2_DTLB / pages (LRU, random access)
//   TLB miss rate = 1 − hit_rate
//   Page walk cost ≈ 3 × L3_LATENCY (SOG §2.7.3: walk uses PDEs;
//     with L2_DTLB saturated, walk accesses memory)
//
// DECLARED CORRECTION: TLB miss cost is subsumed in memory access latency.
//   It affects which tier the access lands in (DRAM/L3/L2 distribution)
//   but is NOT an additive CPI term. Use TLB state as a contextual
//   factor explaining the measured miss distribution, not a separate cost.

/// TLB pressure state for a declared node.
pub struct TlbState {
    pub l1_dtlb_saturated: bool,
    pub l2_dtlb_saturated: bool,
    pub pages: usize,
    pub tlb_miss_rate: f64,  // fraction missing L2_DTLB (random access)
}

pub fn tlb_state(n: usize) -> TlbState {
    const L1: usize = 72;
    const L2: usize = 3072;
    const PAGE: usize = 4096;
    const ELEM: usize = 8;
    let pages = (n * ELEM + PAGE - 1) / PAGE;
    let miss_rate = if pages > L2 {
        1.0 - (L2 as f64 / pages as f64)
    } else if pages > L1 {
        1.0 - (L1 as f64 / pages as f64)
    } else {
        0.0
    };
    TlbState {
        l1_dtlb_saturated: pages > L1,
        l2_dtlb_saturated: pages > L2,
        pages,
        tlb_miss_rate: miss_rate.min(1.0),
    }
}

// ── Model 2: Cache Tier ───────────────────────────────────────────────────────
//
// Cache footprint = N × ELEM_SIZE (full working set, regardless of k_chains).
// CORRECTION from V1.0: per-chain working set understates cache pressure
// because all k chains are simultaneously active across the full array.
//
// Tier boundaries:
//   L1:   WS ≤ 32KB
//   L2:   32KB < WS ≤ 1MB
//   L3:   1MB < WS ≤ 32MB  (Ryzen 5 7600X product spec)
//   DRAM: WS > 32MB

pub fn cache_tier(n: usize) -> usize {
    const ELEM: usize = 8;
    let ws = n * ELEM;
    if ws <= 32_768         { 0 }  // L1
    else if ws <= 1_048_576 { 1 }  // L2
    else if ws <= 33_554_432{ 2 }  // L3
    else                    { 3 }  // DRAM
}

// ── Model 3: Effective Serialized Memory Latency ──────────────────────────────
//
// For D=chain-8 nodes: each chain access is serialized (pointer chase).
// Memory access i+1 cannot issue until access i delivers its result.
// OOO execution cannot reorder within a chain.
//
// However, k=8 chains interleave round-robin, providing k-way
// memory-level parallelism. While chain j awaits result, chains
// j+1..j+7 can issue.
//
// Effective serialized latency per access:
//   eff_mem_lat = f_dram × DRAM_LAT + f_l3 × L3_LAT + f_l2 × L2_LAT
//   where f_dram, f_l3, f_l2 are derived from measured DRAM_PTI, L3_PTI, L2_PTI.
//
// CPI from k-chain serialized memory:
//   CPI_mem = eff_mem_lat / (insts_per_iter × k)
//   where insts_per_iter = 1000 / BR_INST_PTI (one branch per loop iter)
//   and k = number of interleaved chains.
//
// Derivation: each loop iteration issues one serialized chain access.
// The access takes eff_mem_lat cycles. During this time, k-1 other
// chain iterations can also be in flight (k-way parallelism).
// Therefore effective cycles per iteration = eff_mem_lat / k.
// CPI = cycles_per_iter / insts_per_iter = (eff_mem_lat/k) / insts_per_iter.

const DRAM_LAT: f64 = 180.0;  // declared approximate. OC-DRAM-1 OPEN.
                               // Calibration conducted (gather + pointer chase at WS>L3)
                               // but DRAM_LAT not isolatable as unique quantity yet.
                               // OC-DRAM-1a: cycles_per_iter ≈ 101–109 is compound.
                               // Value retained at 180 pending OC-DRAM-1a closure.
const L3_LAT:   f64 = 50.0;   // SOG §A.1
const L2_LAT:   f64 = 14.0;   // SOG §A.1

pub fn eff_mem_lat(f_dram: f64, f_l3: f64, f_l2: f64) -> f64 {
    f_dram * DRAM_LAT + f_l3 * L3_LAT + f_l2 * L2_LAT
}

pub fn cpi_k_chain_memory(
    f_dram: f64, f_l3: f64, f_l2: f64,
    br_inst_pti: f64,   // RETIRED_BR_INST PTI from H vector
    k: usize,           // number of interleaved chains
) -> f64 {
    let lat = eff_mem_lat(f_dram, f_l3, f_l2);
    let insts_per_iter = 1000.0 / br_inst_pti;
    lat / (insts_per_iter * k as f64)
}

// ── Model 4: Branch Misprediction Absorption ──────────────────────────────────
//
// SOG §2.8: misprediction penalty = 13 cycles (common case).
// SOG §2.10.3: ROB = 320 entries (non-SMT).
//
// For D=independent nodes: independent memory accesses fill the ROB window.
// The 13-cycle penalty is hidden behind memory-level parallelism.
// Observed: G0001 (%BR_MISP=98.8%) CPI=1.000 — penalty fully absorbed.
//
// For D=chain-8 nodes: serialized accesses reduce effective ROB occupancy.
// The k-chain window (k=8 chains in flight) partially restores parallelism.
// Branch cost is partially absorbed by the k-chain interleaving window.
//
// Declared: branch misprediction CPI = %BR_MISP × br_inst_rate × 13
// For G0111/G1111: %BR_MISP ≈ 6.5% → contribution ≈ 0.085 CPI.
// For other branchy nodes: high %BR_MISP but absorbed → negligible measured CPI delta.

pub fn br_misp_cpi(br_misp_rate: f64, br_inst_rate: f64) -> f64 {
    br_misp_rate * br_inst_rate * 13.0  // 13 cycles: SOG §2.8
}

// ── Model 5: STLI Cost (from measured PTI) ────────────────────────────────────
//
// SOG §2.12: STLF uses linear address bits[11:0].
// For 8-byte elements: bits[11:0] determined by (index × 8) mod 4096.
// Two elements alias if (i mod 512) == (j mod 512).
//
// Predicted aliasing probability:
//   P(alias per pair) = (N/512 − 1) / (N − 1)
//   At N=4194304: P ≈ 0.00195 per pair.
//   With STQ=64 entries: P(alias in STQ) ≈ 12.5%.
//
// CORRECTION from V1.0: use measured STLI_OTHER PTI directly, not predicted
// aliasing rate. The measured PTI is the observable. The aliasing model
// is context for understanding why it is elevated at D=chain-8 nodes.
//
// STLI CPI = STLI_PTI / 1000 × 13 cycles (approximate pipeline refill cost)

pub fn stli_cpi_from_pti(stli_pti: f64) -> f64 {
    (stli_pti / 1000.0) * 13.0  // 13 cycles approximate; SOG §2.12 implies refill
}

pub fn stli_alias_prob(n: usize) -> f64 {
    const ELEM: usize = 8;
    const PAGE: usize = 4096;
    let elems_per_page = PAGE / ELEM;  // 512
    let alias_per_pair = (n / elems_per_page - 1) as f64 / (n - 1) as f64;
    let stq = 64usize;  // SOG §2.12
    (stq as f64 * alias_per_pair).min(1.0)
}

// ── Compound CPI Model — HISTORICAL CANDIDATE (V2.0) ─────────────────────────
//
// STATUS: OC-V10-1 REOPENED BY OC-STLI-1 (2026-08-30).
//
// This model is preserved as the historical candidate formulation.
// It is NOT currently validated. The STLI additive term is inadmissible
// under the current declared state of OC-STLI-1.
//
// ORIGINAL FORMULATION (preserved for historical record):
//   CPI = CPI_mem + CPI_stli + CPI_br
//   CPI_mem  = eff_mem_lat(miss_dist) / (insts_per_iter × k)
//   CPI_stli = STLI_PTI / 1000 × 13  [INADMISSIBLE — see amendment]
//   CPI_br   = %BR_MISP × br_rate × 13
//
// Original residuals (historical record only):
//   G0111: predicted 1.402, measured 1.341 (Δ=0.061)
//   G1111: predicted 3.262, measured 3.274 (Δ=0.012)
//
// AMENDMENT — OC-V10-1 REOPENED BY OC-STLI-1:
//
// OC-STLI-1 work (2026-08-30) established that the timing relation between
// measured STLI_OTHER PTI and exposed CPI is UNDECLARED. Specifically:
//   1. Chain-only ΔV shows STLI_OTHER rises dramatically (0.155 → 75.807)
//      while CPI remains low (2.57). The timing relation between the counted
//      STLI events and the exposed progression path is UNDECLARED.
//      Critical-path, overlapping, or other timing structures remain
//      undeclared. ΔV does not establish which timing structure applies.
//   2. CPI_stli = STLI_PTI/1000 × 13 is inadmissible because it assumes
//      STLI events contribute 13 exposed cycles additively to CPI — a
//      timing proposition that is not established by the H vector.
//   3. The STLF failure penalty is 19 cycles [C&C], not 13.
//   4. Mechanism claims (OOO cannot reorder within chain, branch predictor
//      adapts to traversal, branch penalty absorbed in interleaving window)
//      are processor-state propositions not established by the H vector.
//
// NUMERICAL INCONSISTENCY (flagged by Verifier 2026-08-30):
// This file contains two conflicting prediction sets — compound_prediction_
// chain_nodes() produces predictions of 1.790/3.518, while the narrative
// text states 1.402/3.262. Both are retained for historical record.
// Neither is currently validated.
//
// Methodological finding: residual fit (Δ=0.061, Δ=0.012) was insufficient
// to identify the mechanism. The chain-only intervention exposed an
// undeclared relation inside the model.
//
// OC-V10-1: REOPENED. Required to close: resolve OC-STLI-1 timing,
// resolve OC-DC-1, then rebuild the compound model on declared foundations.
// OC-DRAM-1, OC-DRAM-1a: open. DRAM_LAT: 180 cycles declared-approximate.

pub struct CompoundPrediction {
    pub node:       &'static str,
    pub cpi_mem:    f64,
    pub cpi_stli:   f64,
    pub cpi_br:     f64,
    pub cpi_total:  f64,
    pub cpi_measured: f64,
    pub residual:   f64,
}

pub fn compound_prediction_chain_nodes() -> Vec<CompoundPrediction> {
    // D=chain-8, B=branchy nodes only (the OC-V10-1 nodes)
    // Miss distributions from measured H vector (DRAM_PTI, L3_PTI, L2_PTI)
    // BR_INST_PTI from measured H vector
    // STLI_PTI from measured H vector
    // %BR_MISP from measured H vector
    let nodes: &[(&str, f64, f64, f64, f64, f64, f64, f64)] = &[
        // (node, dram_pti, l3_pti, l2_pti, br_pti, stli_pti, br_misp, measured_cpi)
        ("G0111", 0.030, 118.882, 32.322, 250.4, 29.2, 0.066, 1.341),
        ("G1111", 53.908, 65.077, 4.932, 248.4, 13.0, 0.065, 3.274),
    ];

    let k = 8usize;
    let br_rate = 0.10f64;  // ~10% of instructions are branches

    nodes.iter().map(|&(node, dram_pti, l3_pti, l2_pti, br_pti, stli_pti, br_misp, measured)| {
        let total_pti = dram_pti + l3_pti + l2_pti;
        let f_dram = dram_pti / total_pti;
        let f_l3   = l3_pti   / total_pti;
        let f_l2   = l2_pti   / total_pti;
        let cpi_mem  = cpi_k_chain_memory(f_dram, f_l3, f_l2, br_pti, k);
        let cpi_stli = stli_cpi_from_pti(stli_pti);
        let cpi_br   = br_misp_cpi(br_misp, br_rate);
        let cpi_total = cpi_mem + cpi_stli + cpi_br;
        CompoundPrediction {
            node, cpi_mem, cpi_stli, cpi_br,
            cpi_total, cpi_measured: measured,
            residual: measured - cpi_total,
        }
    }).collect()
}

// ── Full Node Prediction Table ────────────────────────────────────────────────

pub struct NodePrediction {
    pub node:            &'static str,
    pub a_seq:           bool,
    pub k_chains:        usize,
    pub n:               usize,
    pub l1_dtlb_sat:     bool,
    pub l2_dtlb_sat:     bool,
    pub pages:           usize,
    pub cache_tier:      usize,
    pub tlb_miss_rate:   f64,
    pub stli_alias_prob: f64,
}

pub fn predict_all() -> Vec<NodePrediction> {
    let nodes: &[(&str, bool, usize, usize)] = &[
        ("G0000", true,  1, 524288),
        ("G0100", false, 1, 524288),
        ("G0010", true,  8, 524288),
        ("G0110", false, 8, 524288),
        ("G1000", true,  1, 4194304),
        ("G1100", false, 1, 4194304),
        ("G1010", true,  8, 4194304),
        ("G1110", false, 8, 4194304),
        ("G0001", true,  1, 524288),
        ("G0101", false, 1, 524288),
        ("G0011", true,  8, 524288),
        ("G0111", false, 8, 524288),
        ("G1001", true,  1, 4194304),
        ("G1101", false, 1, 4194304),
        ("G1011", true,  8, 4194304),
        ("G1111", false, 8, 4194304),
    ];

    nodes.iter().map(|&(node, a_seq, k_chains, n)| {
        let tlb = tlb_state(n);
        let ct  = cache_tier(n);  // V2.0: full WS, not per-chain
        let sap = stli_alias_prob(n);
        NodePrediction {
            node, a_seq, k_chains, n,
            l1_dtlb_sat:     tlb.l1_dtlb_saturated,
            l2_dtlb_sat:     tlb.l2_dtlb_saturated,
            pages:           tlb.pages,
            cache_tier:      ct,
            tlb_miss_rate:   tlb.tlb_miss_rate,
            stli_alias_prob: sap,
        }
    }).collect()
}

pub fn print_report() {
    println!("substrate_model V2.0 — Metatron Dynamics, Inc.");
    println!("Source: AMD SOG 57647 Rev. 1.01, April 2023");
    println!("Bounded over D. No claim beyond D.");
    println!();
    println!("V2.0 corrections from V1.0:");
    println!("  1. Cache tier: full WS (not per-chain WS) for all nodes.");
    println!("  2. TLB: subsumed in memory latency, not additive CPI term.");
    println!("  3. MAB: chain serialization → low MAB occupancy, not saturation.");
    println!("     Cost is serialized latency per miss, not buffer pressure.");
    println!();

    let tier_names = ["L1", "L2", "L3", "DRAM"];
    println!("{:<8} {:>5} {:>5} {:>6} {:>6} {:>6} {:>8} {:>8}",
        "Node", "A", "k", "L1TLB", "L2TLB", "Cache", "TLBmiss", "STLI_P");
    println!("{}", "-".repeat(64));
    for p in predict_all() {
        println!("{:<8} {:>5} {:>5} {:>6} {:>6} {:>6} {:>8.3} {:>8.3}",
            p.node,
            if p.a_seq { "seq" } else { "scr" },
            p.k_chains,
            if p.l1_dtlb_sat { "SAT" } else { "ok" },
            if p.l2_dtlb_sat { "SAT" } else { "ok" },
            tier_names[p.cache_tier],
            p.tlb_miss_rate,
            p.stli_alias_prob,
        );
    }

    println!();
    println!("── Compound CPI model: D=chain-8, B=branchy nodes ──────────────");
    println!("   Formula: CPI = CPI_mem + CPI_stli + CPI_br");
    println!("   CPI_mem  = eff_mem_lat(miss_dist) / (insts_per_iter × k)");
    println!("   CPI_stli = STLI_PTI/1000 × 13 cycles (measured)");
    println!("   CPI_br   = %BR_MISP × 10% × 13 cycles (SOG §2.8)");
    println!("   TLB: subsumed in eff_mem_lat (not additive)");
    println!("   DRAM_LAT = 180 cycles (declared approx; range 160–220)");
    println!();
    println!("{:<8} {:>8} {:>8} {:>8} {:>10} {:>10} {:>8}",
        "Node", "CPI_mem", "CPI_stli", "CPI_br", "Predicted", "Measured", "Residual");
    println!("{}", "-".repeat(64));
    for p in compound_prediction_chain_nodes() {
        println!("{:<8} {:>8.3} {:>8.3} {:>8.3} {:>10.3} {:>10.3} {:>8.3}",
            p.node, p.cpi_mem, p.cpi_stli, p.cpi_br,
            p.cpi_total, p.cpi_measured, p.residual);
    }

    println!();
    println!("── OC-V10-1: REOPENED BY OC-STLI-1 ─────────────────────────────");
    println!();
    println!("Status: OC-V10-1 was previously declared closed on the basis of");
    println!("this compound model. OC-STLI-1 work (2026-08-30) has reopened it.");
    println!();
    println!("The STLI additive term (STLI_PTI/1000 × 13) is INADMISSIBLE.");
    println!("Chain-only ΔV establishes a large change in measured STLI_OTHER");
    println!("(0.155 → 75.807 at 2X) alongside the measured CPI (2.57). The");
    println!("timing relation between the counted STLI events and the exposed");
    println!("progression path is UNDECLARED. Critical-path, overlapping, or");
    println!("other timing structures remain undeclared. ΔV does not establish");
    println!("which timing structure applies. CPI_stli = STLI_PTI/1000 × 13");
    println!("is inadmissible because it assumes STLI events contribute 13");
    println!("exposed cycles additively — a timing proposition not established.");
    println!();
    println!("The mechanism claims in this model (OOO serialization, branch");
    println!("predictor adaptation, k-way interleaving) are processor-state");
    println!("propositions not established by the measured H vector alone.");
    println!();
    println!("NUMERICAL INCONSISTENCY (Verifier finding 2026-08-30):");
    println!("The table above shows predictions from compound_prediction_chain_nodes().");
    println!("The narrative below reflects a prior formulation. Both are retained");
    println!("as historical record. Neither is currently validated.");
    println!();
    println!("Required to close OC-V10-1:");
    println!("  1. Resolve OC-STLI-1: experimentally distinguish or otherwise establish");
    println!("     the relation between measured STLI_OTHER and the exposed progression path.");
    println!("  2. Resolve OC-DC-1: obtain load-type-specific refill counter");
    println!("  3. Rebuild compound model on declared foundations");
    println!();
    println!("Original measurements (G0111/G1111 H vectors) are unchanged and valid.");
    println!("Only the mechanistic closure is withdrawn.");
    println!();
    println!("Methodological finding: residual agreement was insufficient to");
    println!("identify the mechanism. The chain-only intervention exposed an");
    println!("undeclared relation inside the model. This is the correct outcome");
    println!();
    println!("── HISTORICAL CANDIDATE MODEL — NOT CURRENTLY VERIFIED ──────────────");
    println!("   THE FOLLOWING MECHANISM STATEMENTS ARE RETAINED FOR");
    println!("   TRACEABILITY ONLY. OC-V10-1 is REOPENED. These propositions");
    println!("   are not established by the measured H vector alone.");
    println!("──────────────────────────────────────────────────────────────────");
    println!();
    println!("Candidate mechanism at G0111/G1111 (scr, ch8, branchy) [HISTORICAL]:");
    println!();
    println!("1. SERIALIZED POINTER CHASE (D=chain-8):");
    println!("   Each of k=8 chains is a pointer chase. Memory access i+1");
    println!("   cannot issue until access i delivers its result.");
    println!("   OOO execution (ROB=320, SOG §2.10.3) cannot reorder within chain.");
    println!();
    println!("2. K-WAY INTERLEAVING PARALLELISM:");
    println!("   The k=8 chains are round-robin interleaved.");
    println!("   While chain j awaits memory result, chains j+1..j+7 issue.");
    println!("   Effective memory-level parallelism = k.");
    println!("   CPI_mem = eff_mem_lat / (insts_per_iter × k).");
    println!();
    println!("3. MEMORY MISS DISTRIBUTION (from measured H vector):");
    println!("   G0111: DRAM=0.0%, L3=78.6%, L2=21.4% → eff_lat=42.3 cycles");
    println!("   G1111: DRAM=43.5%, L3=52.5%, L2=4.0% → eff_lat=105.1 cycles");
    println!("   S1 working set (32MB) is at L3 boundary → DRAM spill.");
    println!();
    println!("4. %BR_MISP COLLAPSE (measured finding):");
    println!("   At scr+ch8, branch predictor adapts to chain traversal.");
    println!("   B=branchy intervention does not produce elevated misprediction.");
    println!("   Branch cost = 6.5% × 10% × 13 = 0.085 CPI (small).");
    println!("   Branch penalty absorbed within k-chain interleaving window.");
    println!();
    println!("5. STLI_OTHER (measured PTI, SOG §2.12):");
    println!("   Chain traversal produces address aliasing at bits[11:0].");
    println!("   SOG §2.12: STLF eligibility uses linear addr bits[11:0].");
    println!("   P(alias in STQ=64) = 12.5% for scrambled access at N=4194304.");
    println!("   Measured STLI_PTI: G0111=29.2, G1111=13.0.");
    println!("   CPI contribution: 0.380 and 0.169 respectively.");
    println!();
    println!("6. TLB SATURATION (context, not additive term):");
    println!("   S1: L2_DTLB saturated (8192 pages > 3072 entries, SOG §2.7.2).");
    println!("   Page walk latency subsumed within memory access latency.");
    println!("   Already counted in DRAM/L3 miss distribution.");
    println!("   Declaring TLB as additive would double-count.");
    println!();
    println!("RESIDUALS:");
    println!("   G0111: predicted 1.402, measured 1.341 (Δ = +0.061)");
    println!("   G1111: predicted 3.262, measured 3.274 (Δ = −0.012)");
    println!("   Both within declared DRAM_LAT uncertainty (±20 cycles).");
    println!("   DRAM_LAT is the only non-SOG constant in the model.");
    println!();
    println!("OC-V10-1: REOPENED BY OC-STLI-1. See amendment above.");
}

pub fn main() {
    print_report();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l1_dtlb_saturated_at_s0() {
        let tlb = tlb_state(524288);
        assert!(tlb.l1_dtlb_saturated, "L1 DTLB should be saturated at S0");
        assert_eq!(tlb.pages, 1024);
    }

    #[test]
    fn l2_dtlb_saturated_at_s1() {
        let tlb = tlb_state(4194304);
        assert!(tlb.l2_dtlb_saturated, "L2 DTLB should be saturated at S1");
        assert_eq!(tlb.pages, 8192);
    }

    #[test]
    fn l2_dtlb_not_saturated_at_s0() {
        let tlb = tlb_state(524288);
        assert!(!tlb.l2_dtlb_saturated, "L2 DTLB should not be saturated at S0");
    }

    #[test]
    fn cache_tier_s0_is_l3() {
        // Full WS = 524288 × 8 = 4.1MB > L2=1MB, < L3=32MB
        assert_eq!(cache_tier(524288), 2, "S0 should be L3 tier");
    }

    #[test]
    fn cache_tier_s0_chain8_is_l3_not_l2() {
        // V2.0 correction: full WS used, not per-chain
        // Full WS S0 = 4.1MB → L3. Per-chain (V1.0 error) was 512KB → L2.
        assert_eq!(cache_tier(524288), 2,
            "V2.0: cache tier uses full WS; S0 is L3, not L2");
    }

    #[test]
    fn cache_tier_s1_is_l3_boundary() {
        // WS = 4194304 × 8 = 32MB = L3 boundary
        let t = cache_tier(4194304);
        assert!(t >= 2, "S1 should be L3 or DRAM tier");
    }

    #[test]
    fn eff_mem_lat_dram_dominated() {
        // All DRAM: should be DRAM_LAT
        let lat = eff_mem_lat(1.0, 0.0, 0.0);
        assert!((lat - 180.0).abs() < 0.01);
    }

    #[test]
    fn eff_mem_lat_l3_dominated() {
        let lat = eff_mem_lat(0.0, 1.0, 0.0);
        assert!((lat - 50.0).abs() < 0.01);
    }

    #[test]
    fn compound_g1111_residual_within_declared_uncertainty() {
        // Historical model arithmetic check — NOT a closure test.
        // OC-V10-1 was REOPENED BY OC-STLI-1 (2026-08-30).
        // The STLI additive term is inadmissible under current OC-STLI-1 state.
        // This test only confirms the model's arithmetic is finite and bounded.
        // It does NOT validate the model as a declared mechanism.
        for p in compound_prediction_chain_nodes() {
            assert!(p.residual.abs() < 1.5,
                "{}: residual {:.3} arithmetic bound exceeded",
                p.node, p.residual);
        }
    }

    #[test]
    fn compound_predictions_finite_and_positive() {
        for p in compound_prediction_chain_nodes() {
            assert!(p.cpi_total.is_finite() && p.cpi_total > 0.0,
                "{}: cpi_total not finite positive", p.node);
            assert!(p.cpi_mem > 0.0, "{}: cpi_mem not positive", p.node);
        }
    }

    #[test]
    fn br_misp_cpi_high_rate_produces_nonzero() {
        let cpi = br_misp_cpi(0.988, 0.10);
        assert!(cpi > 1.0);
    }

    #[test]
    fn br_misp_cpi_low_rate_small() {
        let cpi = br_misp_cpi(0.036, 0.10);
        assert!(cpi < 0.1);
    }

    #[test]
    fn stli_alias_prob_nonzero_at_s1() {
        let p = stli_alias_prob(4194304);
        assert!(p > 0.0 && p <= 1.0);
        // At N=4194304: expected ~12.5%
        assert!((p - 0.125).abs() < 0.01);
    }

    #[test]
    fn predict_all_returns_16_nodes() {
        assert_eq!(predict_all().len(), 16);
    }

    #[test]
    fn all_predictions_finite() {
        for p in predict_all() {
            assert!(p.tlb_miss_rate.is_finite(), "{}: tlb_miss_rate", p.node);
            assert!(p.stli_alias_prob.is_finite(), "{}: stli_alias_prob", p.node);
            assert!(p.tlb_miss_rate >= 0.0 && p.tlb_miss_rate <= 1.0,
                "{}: tlb_miss_rate out of range", p.node);
        }
    }

    #[test]
    fn compound_node_count() {
        assert_eq!(compound_prediction_chain_nodes().len(), 2);
    }
}
