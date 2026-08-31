# chain_only_sim.py — Metatron Dynamics, Inc.
# Python exploration simulation of chain-only hot loop processor behavior.
#
# DECLARATION: This is a Python exploration model.
# Any result that enters the execution record or the kernel
# requires Rust confirmation before it is declared.
#
# Purpose: find parameter combinations (DRAM_LAT, STLF_WINDOW, ROB_ADVANCE,
# TLB_MISS_RATE) that reproduce the observed H vector at both N values.
#
# Sources:
#   [SOG]  AMD Software Optimization Guide, Zen 4, Pub 57647 Rev 1.01
#   [PRL]  processor_relation_ledger.md, Metatron Dynamics
#   [H]    execution_record.md, CAL-CHAIN-ONLY-2X and CAL-CHAIN-ONLY-4X
#   [ASM]  probe.s, run_chain_only hot loop (.LBB13_3), I_asm=9, B_asm=2
#
# Bounded over D. No claim beyond D.
# Python exploration only — Rust confirmation required before any result
# enters the kernel or the execution record.

import itertools
from dataclasses import dataclass, field
from typing import Optional

# ── Declared architectural constants (SOG 57647 Rev 1.01) ────────────────────

SOG_L1_DC_SIZE       = 32_768        # §2.6.2: 32KB L1 D-cache
SOG_L2_SIZE          = 1_048_576     # §2.6.3: 1MB L2
SOG_L3_RYZEN_5_7600X = 33_554_432   # product spec: 32MB L3
SOG_CACHE_LINE       = 64            # §2.6.2
SOG_L1_LAT           = 4             # §2.6.2: integer load-to-use
SOG_L2_LAT           = 14            # §A.1: minimum L2 load-to-use
SOG_L3_LAT           = 50            # §A.1: average L3 load-to-use
SOG_AGU_SIMPLE_LAT   = 4             # §2.12: simple addressing
SOG_AGU_COMPLEX_LAT  = 5             # §2.12: complex addressing (base+index×scale)
SOG_ALU_LAT          = 1             # §2.10.2: simple integer ALU
SOG_BR_PENALTY       = 13            # §2.8: common case misprediction penalty
SOG_ROB_SIZE         = 320           # §2.10.3: retire queue, non-SMT
SOG_RETIRE_PER_CYCLE = 8             # §2.10.3: maximum retirement rate
SOG_STQ_ENTRIES      = 64            # §2.12: store queue
SOG_LDQ_UNCOMPLETE   = 48            # §2.12: load queue uncompleted
SOG_MAB_ENTRIES      = 24            # §2.12: Miss Address Buffer
SOG_L1_DTLB          = 72            # §2.7.1: L1 DTLB entries
SOG_L2_DTLB          = 3072          # §2.7.2: L2 DTLB entries
SOG_PAGE_SIZE        = 4096          # §2.7.1
SOG_ELEM_SIZE        = 8             # declared: u64 elements

# ── Declared workload constants (from assembly + execution record) ────────────

I_ASM        = 9      # declared instructions per hot-loop iteration
B_ASM        = 2      # declared branches per iteration
DRAM_PTI_2X  = 62.341  # measured H: DRAM refills per 1000 inst, 2X
L3_PTI_2X    = 22.157  # measured H: L3 refills per 1000 inst, 2X
L2_PTI_2X    = 18.830  # measured H: L2 refills per 1000 inst, 2X
DRAM_PTI_4X  = 47.733  # measured H: DRAM refills per 1000 inst, 4X
L3_PTI_4X    = 14.622  # measured H: L3 refills per 1000 inst, 4X
L2_PTI_4X    = 23.596  # measured H: L2 refills per 1000 inst, 4X
STLI_PTI_2X  = 75.807  # measured H: STLI per 1000 inst, 2X (TARGET)
STLI_PTI_4X  = 48.428  # measured H: STLI per 1000 inst, 4X (TARGET)
CPI_2X       = 2.5743  # measured H: CPI, 2X (TARGET)
CPI_4X       = 2.0991  # measured H: CPI, 4X (TARGET)
BR_MISP_2X   = 0.04949 # measured H: %BR_MISP as fraction, 2X
N_2X         = 8_388_608
N_4X         = 16_777_216

# ── PRL-active simplifications (declared with reason) ────────────────────────
#
# PRL-I-02/03: Op-cache/fetch — declared zero-latency after warm-up.
#   Reason: loop is 9 instructions, well within op-cache capacity (6.75K ops).
#   Warm-up passes (10 at light protocol) establish op-cache residency.
#
# PRL-I-05: Register rename — declared zero-latency bottleneck.
#   Reason: 4 live registers, PRF has 224 entries; no pressure.
#
# PRL-I-06: Dispatch — declared unlimited within ROB.
#   Reason: 9 instructions/iter, 6 dispatch/cycle; dispatch not bottleneck.
#
# PRL-I-08: ALU — declared 1-cycle latency, no resource contention.
#   Reason: 4 ALU pipes, chain-only uses ~3 ALU ops/iter.
#
# PRL-B-01/02/03: Branch prediction — loop branch always-correctly-predicted.
#   Reason: %BR_MISP ≈ 5% at 2X; modeled as stochastic with that rate.
#   Bounds branches always-not-taken (SOG §2.8.1.5).
#
# PRL-E-03: FP/SIMD — excluded.
#   Reason: no FP instructions in chain-only hot loop.
#
# PRL-M-10: Write-combining — excluded.
#   Reason: no WC memory type in workload.
#
# PRL-B-03: Return address stack — excluded.
#   Reason: no CALL/RET in hot loop.

# ── Simulation state ──────────────────────────────────────────────────────────

@dataclass
class Instruction:
    """One instruction in the hot loop, with its PRL relation."""
    step:       int
    name:       str
    prl:        str
    latency:    int          # cycles from issue to result available
    is_load:    bool = False
    is_store:   bool = False
    is_branch:  bool = False
    is_chain_load: bool = False   # the serializing DRAM-dependent load (step 5)
    is_stack_load: bool = False   # stack load dependent on prior store (step 1)
    is_stack_store: bool = False  # stack store that must commit before STLF (step 6)
    depends_on: list = field(default_factory=list)  # step numbers this depends on

def build_chain_only_loop() -> list:
    """
    Declare the chain-only hot loop instruction sequence.
    Source: probe.s .LBB13_3, declared I_asm=9.
    Each instruction declares its PRL relation, latency, and dependencies.
    """
    return [
        Instruction(1, "movq 40(%rsp),%rax",    "PRL-M-04/STLF",
                    latency=SOG_L1_LAT,          # L1 hit or STLF
                    is_load=True, is_stack_load=True,
                    depends_on=[6]),              # depends on prior iter step 6
        Instruction(2, "cmpq %rdx,%rax",         "PRL-I-08",
                    latency=SOG_ALU_LAT,
                    depends_on=[1]),
        Instruction(3, "jae .LBB13_7",           "PRL-B-02",
                    latency=SOG_ALU_LAT,
                    is_branch=True,
                    depends_on=[2]),
        Instruction(4, "incq %r8",               "PRL-I-08",
                    latency=SOG_ALU_LAT,
                    depends_on=[]),               # independent of memory path
        Instruction(5, "movq (%rcx,%rax,8),%rax","PRL-M-07/DRAM",
                    latency=0,                    # set by parameter DRAM_LAT
                    is_load=True, is_chain_load=True,
                    depends_on=[1]),              # address from step 1
        Instruction(6, "movq %rax,40(%rsp)",     "PRL-M-01/STQ",
                    latency=SOG_ALU_LAT,          # store address gen
                    is_store=True, is_stack_store=True,
                    depends_on=[5]),              # data from step 5
        Instruction(7, "black_box fence",        "PRL-I-07",
                    latency=0,
                    depends_on=[6]),
        Instruction(8, "cmpq %r8,%rdx",          "PRL-I-08",
                    latency=SOG_ALU_LAT,
                    depends_on=[4]),
        Instruction(9, "jne .LBB13_3",           "PRL-B-02",
                    latency=SOG_ALU_LAT,
                    is_branch=True,
                    depends_on=[8]),
    ]

# ── Core simulation ───────────────────────────────────────────────────────────

@dataclass
class SimParams:
    """Free parameters for the simulation. All declared with source."""
    dram_lat:       float  # P1: chain load DRAM latency (OC-DRAM-1, unknown)
    stlf_window:    int    # P2: cycles after store issue before STLF succeeds
                           #     From SOG §2.12: store data must be available
                           #     Value = latency from store issue to data ready
    tlb_miss_rate:  float  # P4: fraction of chain loads that miss L2_DTLB
                           #     OC-TLB-1: estimated (pages-L2_DTLB)/pages
    tlb_walk_lat:   int    # P4b: TLB walk latency in cycles (SOG §2.7.3: 3+ accesses)
    br_misp_rate:   float  # from measured %BR_MISP (not truly free)
    # P3 (ROB_ADVANCE) is derived from dram_lat and I_asm, not free:
    # max_iters_in_flight = min(ROB_SIZE // I_ASM, MAB_ENTRIES)

@dataclass
class SimResult:
    """Simulation output for one parameter set at one N value."""
    cpi:            float
    stli_pti:       float
    cycles_per_iter: float
    iters_in_flight: int
    stlf_success_rate: float
    chain_load_lat: float   # effective latency including TLB

def simulate_chain_only(params: SimParams, n: int,
                        dram_pti: float, l3_pti: float, l2_pti: float) -> SimResult:
    """
    Simulate one chain-only execution at declared N.

    Model structure (PRL-grounded):

    Per iteration, the critical dependency chain is:
      [prior iter step 6 STORE] → STLF → [step 1 LOAD] → [step 5 CHAIN LOAD]
      → [step 6 STORE] → next iter

    The chain load (step 5) takes:
      effective_chain_lat = f_dram × (dram_lat + tlb_cost) + f_l3 × L3_LAT + f_l2 × L2_LAT
    where f_dram, f_l3, f_l2 are from measured H vector (not free parameters).

    During the chain load stall, the OOO engine advances speculatively.
    It can have at most ROB_SIZE/I_ASM iterations in flight simultaneously.
    Also bounded by MAB_ENTRIES (24) for outstanding cache misses.

    The STLF relation (PRL-M-08):
      Each iteration's step 1 (stack load) depends on the prior iter's step 6 (store).
      STLF succeeds if: store data is available in STQ when load issues.
      Store data is available stlf_window cycles after store issues.
      If the OOO engine issues step 1 before prior step 6 + stlf_window,
      STLF fails → STLI fires.

    The OOO engine advances during the chain load stall.
    Specifically: while waiting for iter i's chain load (step 5),
    it can execute iterations i+1, i+2, ... up to ROB capacity.
    Iter i+1's step 1 (stack load) will attempt STLF from iter i's step 6.
    If iter i's step 6 has not yet been forwardable (within stlf_window cycles),
    STLF fails.
    """

    # ── Compute TLB miss cost ─────────────────────────────────────────────────
    # PRL-M-03: TLB walk adds latency to chain load address translation
    # Walk cost modeled as tlb_walk_lat cycles when miss occurs
    tlb_cost = params.tlb_miss_rate * params.tlb_walk_lat

    # ── Compute effective chain load latency ──────────────────────────────────
    # PRL-M-07: DRAM access, PRL-M-06: L3, PRL-M-05: L2
    # Service distribution from measured H vector (declared inputs, not free)
    total_pti = dram_pti + l3_pti + l2_pti
    if total_pti == 0:
        f_dram, f_l3, f_l2 = 1.0, 0.0, 0.0
    else:
        f_dram = dram_pti / total_pti
        f_l3   = l3_pti   / total_pti
        f_l2   = l2_pti   / total_pti

    chain_load_lat = (f_dram * (params.dram_lat + tlb_cost) +
                      f_l3   * SOG_L3_LAT +
                      f_l2   * SOG_L2_LAT)

    # ── Compute non-memory overhead per iteration ─────────────────────────────
    # PRL-I-08: ALU operations
    # stack load (step 1): STLF latency or L1 hit (SOG_L1_LAT)
    # bounds check (step 2+3): 1+1 = 2 cycles (fused, SOG §2.9.3)
    # counter (step 4): 1 cycle (independent)
    # AGU for chain load (step 5): SOG_AGU_COMPLEX_LAT = 5 cycles
    #   (base+index×8 is complex addressing)
    # stack store (step 6): SOG_AGU_SIMPLE_LAT = 4 cycles
    # loop check+branch (steps 8+9): 1+1 = 2 cycles (fused)
    # STLF overhead: SOG_L1_LAT for successful STLF
    stlf_lat    = SOG_L1_LAT       # step 1: STLF success latency
    agu_complex = SOG_AGU_COMPLEX_LAT  # step 5: address gen for chain load
    overhead    = stlf_lat + 1.5 + agu_complex  # bounds fused + AGU

    # ── OOO advance during chain load stall ──────────────────────────────────
    # PRL-B-04: Speculative execution during memory stall
    # PRL-S-03: Load-store reordering
    # How many iterations can OOO advance while step 5 is pending?
    # Bounded by:
    #   (a) ROB capacity: floor(SOG_ROB_SIZE / I_ASM) iterations
    #   (b) MAB entries: SOG_MAB_ENTRIES outstanding misses
    #   (c) Dependency: iter i+1's step 5 depends on iter i's step 5
    #       (because step 1 depends on prior step 6, which depends on prior step 5)
    #       So actually iter i+1 CANNOT issue its step 5 until iter i's step 5 completes.
    #       The OOO window is limited to the NON-step-5 instructions of subsequent iters.
    #
    # With one serialized DRAM load per iter, the OOO engine cannot advance
    # past the NEXT iteration's step 5 (which depends on step 5 of the current iter).
    # It CAN execute the non-chain-load instructions of iterations i+1, i+2, ...
    # up to the point where step 5 of those iterations would need to issue.
    #
    # For the STLI question: the OOO engine can issue iter i+1's step 1 (stack load)
    # during iter i's chain load stall. The question is whether iter i's step 6
    # is forwardable at that point.
    #
    # Step 6 of iter i: issues after step 5 of iter i completes.
    # Step 6 latency: SOG_AGU_SIMPLE_LAT = 4 cycles (address gen for store).
    # Store data (%rax) is available when step 5 completes (= chain_load_lat).
    # STLF success: requires store data in STQ, which means step 6 must have
    # issued AND stlf_window cycles must have passed.
    # Step 6 issues at: step 5 complete + step 6 latency (address gen) ≈ 0
    #   (store address is known immediately; data is what matters for STLF)
    # STLF window: stlf_window cycles after step 6 issues.
    #
    # Step 1 of iter i+1: issues when step 6 of iter i is complete.
    # But the OOO engine may issue step 1 of iter i+1 SPECULATIVELY
    # before step 6 of iter i has had stlf_window cycles to become forwardable.
    #
    # Timing model for STLI:
    # Cycle 0: iter i step 5 (chain load) issues, begins waiting for DRAM
    # Cycle k: OOO engine executes iter i+1 steps 2,3,4 (non-dependent on step 5)
    # Cycle k+?: OOO engine tries to issue iter i+1 step 1 (stack load)
    #   It needs data from iter i step 6.
    #   If iter i step 6 has been in STQ for < stlf_window cycles → STLI fires
    #   If iter i step 6 has been in STQ for >= stlf_window cycles → STLF succeeds
    #
    # Step 6 of iter i issues approximately when step 5 completes (cycle chain_load_lat).
    # Step 1 of iter i+1 tries to issue approximately params.stlf_window cycles
    # after step 6 of iter i issues, if the OOO engine is that far ahead.
    #
    # The OOO engine's advance during chain load stall:
    # Non-step-5 instructions of iter i+1 = steps 1,2,3,4,6,7,8,9 = 8 instructions
    # Steps 2,3,4,8,9 are independent of step 5 (counter, bounds, loop branch)
    # Step 1 depends on iter i step 6, which depends on iter i step 5
    # Step 6 depends on iter i+1 step 5 (which cannot issue yet)
    # So during the chain load stall, OOO can execute iter i+1 steps 4,8 (independent)
    # and ATTEMPT to issue step 1 (which needs STLF from iter i step 6)
    #
    # The advance is limited: only steps 4 and 8 are truly independent.
    # Steps 1,2,3,6,7,9 all have dependencies that block them.
    # So the OOO "advance" is small — 2-3 independent instructions.
    # This means step 1 of iter i+1 is attempted almost immediately after
    # the OOO engine sees that iter i step 6 might be available.

    # Model: step 1 of iter i+1 is issued at cycle T_issue_step1
    # T_issue_step1 ≈ overhead_before_step1 = bounds + counter ≈ 2-3 cycles
    # after the OOO engine starts working on iter i+1.
    # OOO starts iter i+1 at: cycle where iter i step 5 issues (cycle 0)
    # + the time for the OOO engine to free up slots.
    # For simplicity: model T_issue_step1 ≈ 2 cycles after OOO starts iter i+1.

    ooo_start_next_iter = 2.0  # cycles after chain load issues before iter i+1 starts
    t_step1_issue = ooo_start_next_iter + 2.0  # steps 4,8 then step 1 attempted

    # Step 6 of iter i issues at: chain_load_lat (when step 5 delivers result)
    # + store data latency (0 — data is %rax, immediately available when step 5 done)
    # + store address gen (immediate — address is fixed 40(%rsp))
    # STLF forwardable at: t_step6_issue + stlf_window
    t_step6_issue = chain_load_lat  # step 6 issues when step 5 delivers
    t_stlf_forwardable = t_step6_issue + params.stlf_window

    # STLI fires if step 1 issues BEFORE STLF is forwardable
    # i.e., if t_step1_issue < t_stlf_forwardable
    # Step 1 cannot issue before its address is known (40(%rsp) always known)
    # Step 1 issues when OOO schedules it, which is t_step1_issue
    # But step 1 also waits for prior iter step 6 to be VISIBLE in store queue
    # If step 6 hasn't issued yet (step 6 issues at chain_load_lat), step 1 waits.
    # So step 1 actually issues at: max(t_step1_issue, t_step6_issue + ε)
    # where ε is a small delay for store queue visibility.

    t_step1_actual = max(t_step1_issue, t_step6_issue + 1.0)

    if t_step1_actual < t_stlf_forwardable:
        # STLF fails — STLI fires
        stlf_success_rate = 0.0
        stli_fires = True
    else:
        # STLF succeeds
        stlf_success_rate = 1.0
        stli_fires = False

    # ── STLI_PTI prediction ───────────────────────────────────────────────────
    # STLI_PTI = STLI events per 1000 retired instructions
    # STLI fires once per iteration if the condition is met (stlf_success_rate=0)
    # or never if STLF succeeds (stlf_success_rate=1)
    # With measured STLI_PTI = 75.8 at 2X (not 1000/9=111), partial firing is implied.
    # Model: STLI fires with probability (1 - stlf_success_rate)
    # STLI_PTI = (1 - stlf_success_rate) × (1000 / I_ASM)
    stli_per_iter = (1.0 - stlf_success_rate)
    stli_pti_pred = stli_per_iter * (1000.0 / I_ASM)

    # ── CPI prediction ────────────────────────────────────────────────────────
    # Critical path per iteration:
    #   overhead (non-memory) + chain_load_lat
    # If STLI fires: add STLI penalty to critical path
    # STLI penalty: the load must wait for store to commit → adds cycles
    # Model: STLI adds (t_stlf_forwardable - t_step1_actual) cycles to step 1

    if stli_fires:
        stli_penalty = t_stlf_forwardable - t_step1_actual
    else:
        stli_penalty = 0.0

    # Misprediction contribution (PRL-B-05)
    misp_cpi = params.br_misp_rate * (B_ASM / I_ASM) * SOG_BR_PENALTY

    cycles_per_iter = overhead + chain_load_lat + stli_penalty
    cpi = cycles_per_iter / I_ASM + misp_cpi

    # OOO iterations in flight during DRAM stall
    iters_in_flight = min(SOG_ROB_SIZE // I_ASM, SOG_MAB_ENTRIES)

    return SimResult(
        cpi=cpi,
        stli_pti=stli_pti_pred,
        cycles_per_iter=cycles_per_iter,
        iters_in_flight=iters_in_flight,
        stlf_success_rate=stlf_success_rate,
        chain_load_lat=chain_load_lat,
    )

# ── TLB parameters from declared structure ───────────────────────────────────

def tlb_miss_rate(n: int) -> float:
    """
    Declared TLB miss rate from PRL-M-03 + OC-TLB-1.
    Pages = N × ELEM_SIZE / PAGE_SIZE.
    L2_DTLB saturated when pages > SOG_L2_DTLB (3072).
    Miss rate = max(0, (pages - SOG_L2_DTLB) / pages) under LRU approximation.
    This is declared from SOG §2.7.2, not measured. OC-TLB-1 open.
    """
    pages = (n * SOG_ELEM_SIZE) // SOG_PAGE_SIZE
    if pages <= SOG_L2_DTLB:
        return 0.0
    return (pages - SOG_L2_DTLB) / pages

TLB_MISS_2X = tlb_miss_rate(N_2X)  # 64MB: 8192 pages, L2_DTLB=3072 → 62.5%
TLB_MISS_4X = tlb_miss_rate(N_4X)  # 128MB: 16384 pages → 81.25%

# ── Parameter sweep ───────────────────────────────────────────────────────────

def sweep(verbose=True):
    """
    Sweep DRAM_LAT and STLF_WINDOW to find combinations consistent with
    observed CPI and STLI_PTI at both N values.

    Declared tolerance:
      CPI:      ±0.15 (6% of observed 2.5743)
      STLI_PTI: ±15.0 (20% of observed 75.8)
    These tolerances reflect measurement uncertainty in PTI sampling
    and the approximate nature of the overhead model.
    """

    # TLB walk latency: 3 L3 accesses (SOG §2.7.3, conservative)
    tlb_walk_lat = 3 * SOG_L3_LAT  # 150 cycles

    results = []

    for dram_lat in range(50, 220, 5):       # P1: 50–220 cycles
        for stlf_win in range(0, 50, 2):     # P2: 0–50 cycles

            p = SimParams(
                dram_lat     = float(dram_lat),
                stlf_window  = stlf_win,
                tlb_miss_rate= TLB_MISS_2X,
                tlb_walk_lat = tlb_walk_lat,
                br_misp_rate = BR_MISP_2X,
            )

            r2x = simulate_chain_only(p, N_2X, DRAM_PTI_2X, L3_PTI_2X, L2_PTI_2X)

            p4x = SimParams(
                dram_lat     = float(dram_lat),
                stlf_window  = stlf_win,
                tlb_miss_rate= TLB_MISS_4X,
                tlb_walk_lat = tlb_walk_lat,
                br_misp_rate = BR_MISP_2X,
            )
            r4x = simulate_chain_only(p4x, N_4X, DRAM_PTI_4X, L3_PTI_4X, L2_PTI_4X)

            # Check against observed H vector
            cpi_err_2x   = abs(r2x.cpi - CPI_2X)
            stli_err_2x  = abs(r2x.stli_pti - STLI_PTI_2X)
            cpi_err_4x   = abs(r4x.cpi - CPI_4X)
            stli_err_4x  = abs(r4x.stli_pti - STLI_PTI_4X)

            cpi_tol  = 0.15
            stli_tol = 15.0

            if (cpi_err_2x <= cpi_tol and stli_err_2x <= stli_tol and
                cpi_err_4x <= cpi_tol and stli_err_4x <= stli_tol):
                results.append((dram_lat, stlf_win, r2x, r4x,
                                 cpi_err_2x, stli_err_2x,
                                 cpi_err_4x, stli_err_4x))

    return results

# ── Run and report ────────────────────────────────────────────────────────────

def main():
    print("chain_only_sim — Metatron Dynamics, Inc.")
    print("Python exploration. Rust confirmation required before any result")
    print("enters the kernel or the execution record.")
    print("Bounded over D. No claim beyond D.")
    print()
    print(f"Declared targets:")
    print(f"  CPI(2X)      = {CPI_2X}  STLI_PTI(2X) = {STLI_PTI_2X}")
    print(f"  CPI(4X)      = {CPI_4X}  STLI_PTI(4X) = {STLI_PTI_4X}")
    print(f"  TLB_MISS(2X) = {TLB_MISS_2X:.3f}  TLB_MISS(4X) = {TLB_MISS_4X:.3f}")
    print(f"  Tolerances: CPI ±0.15, STLI_PTI ±15.0")
    print()
    print("Sweeping DRAM_LAT [50,215] × STLF_WINDOW [0,48]...")

    results = sweep()

    if not results:
        print()
        print("No parameter combinations found within declared tolerances.")
        print("Reporting best fits:")
        # Find best fit even if outside tolerance
        best = []
        for dram_lat in range(50, 220, 5):
            for stlf_win in range(0, 50, 2):
                p = SimParams(float(dram_lat), stlf_win,
                              TLB_MISS_2X, 3*SOG_L3_LAT, BR_MISP_2X)
                r2x = simulate_chain_only(p, N_2X, DRAM_PTI_2X, L3_PTI_2X, L2_PTI_2X)
                p4x = SimParams(float(dram_lat), stlf_win,
                               TLB_MISS_4X, 3*SOG_L3_LAT, BR_MISP_2X)
                r4x = simulate_chain_only(p4x, N_4X, DRAM_PTI_4X, L3_PTI_4X, L2_PTI_4X)
                score = (abs(r2x.cpi-CPI_2X)/CPI_2X +
                         abs(r4x.cpi-CPI_4X)/CPI_4X +
                         abs(r2x.stli_pti-STLI_PTI_2X)/max(STLI_PTI_2X,1) +
                         abs(r4x.stli_pti-STLI_PTI_4X)/max(STLI_PTI_4X,1))
                best.append((score, dram_lat, stlf_win, r2x, r4x))
        best.sort()
        for score, dl, sw, r2x, r4x in best[:5]:
            print(f"  DRAM_LAT={dl:3d} STLF_WIN={sw:2d} | "
                  f"CPI: {r2x.cpi:.3f}/{r4x.cpi:.3f} "
                  f"STLI: {r2x.stli_pti:.1f}/{r4x.stli_pti:.1f} | "
                  f"score={score:.3f}")
    else:
        print(f"\n{len(results)} parameter combinations within tolerance:")
        print(f"  {'DRAM_LAT':>8} {'STLF_WIN':>8} | "
              f"{'CPI_2X':>7} {'CPI_4X':>7} | "
              f"{'STLI_2X':>8} {'STLI_4X':>8} | "
              f"{'chain_lat_2x':>13}")
        print("  " + "─"*75)
        for dl, sw, r2x, r4x, ce2, se2, ce4, se4 in results:
            print(f"  {dl:>8} {sw:>8} | "
                  f"{r2x.cpi:>7.3f} {r4x.cpi:>7.3f} | "
                  f"{r2x.stli_pti:>8.1f} {r4x.stli_pti:>8.1f} | "
                  f"{r2x.chain_load_lat:>13.1f}")

    print()
    print("── STLF structural analysis ─────────────────────────────────")
    print()
    print(f"  t_step6_issue ≈ DRAM_LAT (chain load completes)")
    print(f"  t_step1_actual ≈ max(2+2, DRAM_LAT+1) cycles")
    print(f"  STLF succeeds when: t_step1_actual >= t_step6_issue + STLF_WINDOW")
    print(f"  i.e., when: DRAM_LAT+1 >= DRAM_LAT + STLF_WINDOW")
    print(f"  i.e., when: STLF_WINDOW <= 1")
    print()
    print(f"  This means for any STLF_WINDOW > 1, STLI fires on every iteration.")
    print(f"  For STLF_WINDOW <= 1, STLF always succeeds.")
    print(f"  The observed STLI_PTI = 75.8 (not 111.1 = 1000/9)")
    print(f"  implies STLF fires on {75.8/111.1*100:.1f}% of iterations.")
    print(f"  This fractional firing is not captured by the binary model above.")
    print(f"  A probabilistic model is needed — see next section.")
    print()
    print("── Diagnosis: binary STLF model is insufficient ─────────────")
    print()
    print("  The observed STLI_PTI = 75.8 at 2X corresponds to:")
    print(f"  {75.8 / (1000/I_ASM) * 100:.1f}% of iterations producing STLI.")
    print("  A binary (always fires / never fires) model cannot reproduce this.")
    print("  A probabilistic model is required — one in which STLF succeeds")
    print("  on some iterations and fails on others.")
    print()
    print("  Likely source of probabilistic behavior:")
    print("  The chain load latency varies per iteration (L2/L3/DRAM mix).")
    print("  When chain load hits L3 (shorter latency), step 6 issues earlier,")
    print("  giving more time for STLF to succeed before step 1 issues.")
    print("  When chain load goes to DRAM (longer latency), step 6 issues later,")
    print("  and STLF may or may not succeed depending on OOO timing.")
    print()
    print("  This requires modeling latency as a DISTRIBUTION, not a single value.")
    print("  The miss distribution (f_dram, f_l3, f_l2) from the H vector")
    print("  provides the probability weights for each latency tier.")
    print()
    print("NEXT STEP: extend to probabilistic latency distribution model.")

if __name__ == "__main__":
    main()
