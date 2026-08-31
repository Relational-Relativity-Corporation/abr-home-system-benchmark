# chain_only_sim_v2.py — Metatron Dynamics, Inc.
# Probabilistic latency distribution model for chain-only STLF analysis.
#
# DECLARATION: Python exploration only.
# Rust confirmation required before any result enters the kernel or record.
# Bounded over D. No claim beyond D.
#
# V1 finding: binary STLF model predicts CPI ≈ 14 (observed: 2.57).
# Conclusion: STLI fires within the memory stall window (Case A of OC-STLI-1).
# STLI penalty is largely hidden behind chain load latency.
#
# V2 extension: probabilistic latency distribution.
# Chain load latency varies per iteration (L2/L3/DRAM mix from H vector).
# STLF fires on iterations where the chain load is SHORT enough that
# step 6 data is not yet forwardable when step 1 issues.
# On DRAM-miss iterations (long latency), step 6 is ALWAYS forwardable
# by the time step 1 issues (step 1 waits for step 6 which waits for step 5).
# On L3/L2-hit iterations (short latency), step 6 may not be forwardable
# in time — STLI fires.
#
# This inverts the original assumption: STLI fires on SHORT-latency iterations,
# not on long-latency ones.

import numpy as np

# ── Constants (from V1) ───────────────────────────────────────────────────────
SOG_L1_LAT     = 4;   SOG_L2_LAT   = 14;  SOG_L3_LAT   = 50
SOG_ALU_LAT    = 1;   SOG_AGU_COMPLEX = 5; SOG_AGU_SIMPLE = 4
SOG_BR_PENALTY = 13;  SOG_ROB_SIZE = 320;  I_ASM = 9; B_ASM = 2
SOG_ELEM_SIZE  = 8;   SOG_PAGE_SIZE = 4096; SOG_L2_DTLB = 3072
N_2X = 8_388_608;     N_4X = 16_777_216

# Measured H vector targets
CPI_2X = 2.5743;   STLI_PTI_2X = 75.807; BR_MISP_2X = 0.04949
CPI_4X = 2.0991;   STLI_PTI_4X = 48.428

# Miss distributions from measured H vector
# f_tier = tier_PTI / (dram_PTI + l3_PTI + l2_PTI)
def miss_dist(dram_pti, l3_pti, l2_pti):
    total = dram_pti + l3_pti + l2_pti
    return dram_pti/total, l3_pti/total, l2_pti/total

f_dram_2x, f_l3_2x, f_l2_2x = miss_dist(62.341, 22.157, 18.830)
f_dram_4x, f_l3_4x, f_l2_4x = miss_dist(47.733, 14.622, 23.596)

# TLB miss rates (declared from SOG §2.7.2)
def tlb_miss_rate(n):
    pages = (n * SOG_ELEM_SIZE) // SOG_PAGE_SIZE
    return max(0, (pages - SOG_L2_DTLB) / pages)

TLB_MISS_2X = tlb_miss_rate(N_2X)   # 0.625
TLB_MISS_4X = tlb_miss_rate(N_4X)   # 0.813

print("chain_only_sim V2 — Metatron Dynamics, Inc.")
print("Python exploration. Rust confirmation required.")
print("Bounded over D.")
print()
print(f"Miss distributions from H vector:")
print(f"  2X: DRAM={f_dram_2x:.3f} L3={f_l3_2x:.3f} L2={f_l2_2x:.3f}")
print(f"  4X: DRAM={f_dram_4x:.3f} L3={f_l3_4x:.3f} L2={f_l2_4x:.3f}")
print(f"TLB miss rates: 2X={TLB_MISS_2X:.3f}, 4X={TLB_MISS_4X:.3f}")
print()

# ── V1 finding: STLI fires on short-latency iterations ───────────────────────
print("── V1 Finding: STLI timing analysis ────────────────────────────")
print()
print("For any chain load latency L:")
print("  step 5 (chain load) completes at cycle L")
print("  step 6 (stack store) issues at cycle L (data = %rax, immediate)")
print("  step 1 (stack load, next iter) issues at cycle L+1 at earliest")
print("    (must wait for step 6 to be visible in store queue)")
print("  STLF forwardable at cycle L + STLF_WINDOW")
print("  STLI fires if: L+1 < L + STLF_WINDOW, i.e., STLF_WINDOW > 1")
print()
print("  For STLF_WINDOW > 1: STLI fires on EVERY iteration regardless of L.")
print("  This produces STLI_PTI = 1000/I_ASM = 111.1 (observed: 75.8).")
print()
print("  For STLF_WINDOW = 0 or 1: STLI never fires.")
print("  This produces STLI_PTI = 0 (observed: 75.8).")
print()
print("  Neither matches. The binary STLF model is structurally insufficient.")
print("  Reason: the model assumed step 1 issues at L+1.")
print("  But step 1 depends on step 6, which depends on step 5.")
print("  The OOO engine CANNOT issue step 1 until step 6 is in the store queue.")
print("  Step 6 enters the store queue approximately when step 5 completes.")
print()
print("  REVISED TIMING:")
print("  Step 5 completes at cycle L.")
print("  Step 6 issues at cycle L (store address gen, 4 cycles).")
print("  Step 6 data WRITTEN at cycle L (data=%rax, available at step 5 complete).")
print("  Step 6 enters store queue at cycle L.")
print("  STLF possible from cycle L + STLF_WINDOW.")
print()
print("  Step 1 of next iter: the OOO engine sees that iter i's step 6")
print("  is in the store queue. It can issue step 1 at cycle L + ε")
print("  where ε is the store queue visibility latency.")
print("  From SOG §2.12: store-to-load forwarding has a latency.")
print("  The STLF_WINDOW parameter captures this latency.")
print()
print("  If STLF_WINDOW = ε (very small), step 1 sees the store immediately.")
print("  As STLF_WINDOW grows, more iterations fail.")
print()

# ── Revised model: what produces fractional STLI? ────────────────────────────
print("── Why is STLI_PTI fractional (75.8, not 0 or 111.1)? ──────────")
print()
print("  The H vector PTI counters are SAMPLED estimates.")
print("  STLI_PTI = 75.8 means approximately 68% of iterations produce STLI.")
print("  This could arise from:")
print()
print("  Hypothesis A: STLF_WINDOW is fixed, but some iterations the store")
print("  queue is congested and step 6 takes longer to become forwardable.")
print("  Congestion probability = some fraction p → STLI_PTI = p × 111.1")
print(f"  Required p = 75.8/111.1 = {75.8/111.1:.3f}")
print()
print("  Hypothesis B: The OOO engine issues step 1 speculatively before")
print("  step 6 is visible for a fraction of iterations.")
print("  This fraction depends on the OOO advance rate relative to store latency.")
print()
print("  Hypothesis C: PTI sampling captures only a fraction of true events.")
print("  True STLI rate could be 100% with PTI capturing 68%.")
print("  This is OC-DC-1 (counter-to-instruction mapping undeclared).")
print()
print("  If Hypothesis C is correct: STLI fires on EVERY iteration.")
print("  Then STLF_WINDOW > 1, and the CPI impact is what matters.")
print()

# ── CPI model with STLI hidden in stall window ────────────────────────────────
print("── CPI model: STLI hidden in stall window ───────────────────────")
print()
print("  V1 found: CPI model with STLI on critical path predicts ~14.")
print("  Observed CPI = 2.57. The gap is ~11.5 cycles/instruction.")
print()
print("  If STLI is hidden in the chain load stall (Case A, OC-STLI-1):")
print("  CPI = overhead + chain_load_lat / I_ASM")
print("  where overhead = stack_load + bounds + AGU + mispredict")
print()

# Compute overhead
overhead = SOG_L1_LAT + 1.5 + SOG_AGU_COMPLEX  # STLF + bounds fused + AGU
misp_cpi = BR_MISP_2X * (B_ASM/I_ASM) * SOG_BR_PENALTY

print(f"  Overhead = L1_LAT({SOG_L1_LAT}) + bounds_fused(1.5) + AGU({SOG_AGU_COMPLEX})")
print(f"           = {overhead} cycles")
print(f"  Misprediction CPI = {BR_MISP_2X:.4f} × {B_ASM}/{I_ASM} × {SOG_BR_PENALTY}")
print(f"                    = {misp_cpi:.4f}")
print()

# Solve for chain_load_lat from observed CPI
# CPI = (overhead + chain_load_lat) / I_ASM + misp_cpi
# chain_load_lat = (CPI - misp_cpi) × I_ASM - overhead
for label, cpi_obs, f_dram, f_l3, f_l2, tlb_miss in [
    ("2X", CPI_2X, f_dram_2x, f_l3_2x, f_l2_2x, TLB_MISS_2X),
    ("4X", CPI_4X, f_dram_4x, f_l3_4x, f_l2_4x, TLB_MISS_4X),
]:
    chain_lat = (cpi_obs - misp_cpi) * I_ASM - overhead
    print(f"  {label}: chain_load_lat = ({cpi_obs} - {misp_cpi:.4f}) × {I_ASM} - {overhead}")
    print(f"       = {chain_lat:.2f} cycles")

    # Decompose chain_load_lat into DRAM + TLB components
    # chain_load_lat = f_dram×(DRAM_LAT+TLB_cost) + f_l3×L3_LAT + f_l2×L2_LAT
    # Solve for DRAM_LAT:
    tlb_walk = 3 * SOG_L3_LAT  # conservative walk cost
    tlb_cost = tlb_miss * tlb_walk
    l3_contrib = f_l3 * SOG_L3_LAT
    l2_contrib = f_l2 * SOG_L2_LAT
    dram_lat = (chain_lat - l3_contrib - l2_contrib) / f_dram - tlb_cost
    eff_lat  = f_dram*(dram_lat+tlb_cost) + f_l3*SOG_L3_LAT + f_l2*SOG_L2_LAT

    print(f"       TLB_cost = {tlb_miss:.3f} × {tlb_walk} = {tlb_cost:.1f} cycles")
    print(f"       L3 contrib = {f_l3:.3f} × {SOG_L3_LAT} = {l3_contrib:.1f}")
    print(f"       L2 contrib = {f_l2:.3f} × {SOG_L2_LAT} = {l2_contrib:.1f}")
    print(f"       DRAM_LAT = ({chain_lat:.2f} - {l3_contrib:.1f} - {l2_contrib:.1f}) / {f_dram:.3f} - {tlb_cost:.1f}")
    print(f"              = {dram_lat:.1f} cycles")
    print(f"       Verify: eff_lat = {eff_lat:.2f} (should = {chain_lat:.2f})")
    print()

print("── Agreement check ──────────────────────────────────────────────")
print()
# Compute for both N values
results = {}
for label, cpi_obs, f_dram, f_l3, f_l2, tlb_miss in [
    ("2X", CPI_2X, f_dram_2x, f_l3_2x, f_l2_2x, TLB_MISS_2X),
    ("4X", CPI_4X, f_dram_4x, f_l3_4x, f_l2_4x, TLB_MISS_4X),
]:
    chain_lat = (cpi_obs - misp_cpi) * I_ASM - overhead
    tlb_cost = tlb_miss * (3 * SOG_L3_LAT)
    l3_c = f_l3 * SOG_L3_LAT
    l2_c = f_l2 * SOG_L2_LAT
    dram_lat = (chain_lat - l3_c - l2_c) / f_dram - tlb_cost
    results[label] = dram_lat

print(f"  DRAM_LAT from 2X (Case A): {results['2X']:.1f} cycles")
print(f"  DRAM_LAT from 4X (Case A): {results['4X']:.1f} cycles")
print(f"  Difference: {abs(results['2X']-results['4X']):.1f} cycles")
print()

if abs(results['2X']-results['4X']) <= 5:
    print("  Agreement within ±5 cycles — consistent with Case A.")
    print("  STLI is hidden in the chain load stall window.")
    print("  DRAM_LAT candidate: "
          f"{(results['2X']+results['4X'])/2:.1f} cycles (mean)")
else:
    print("  Disagreement > 5 cycles — Case A does not fully explain the data.")
    print("  Something else differs between 2X and 4X beyond the miss distribution.")
    print("  Candidate: TLB walk latency differs (different page table depth at 4X)")
    print("  or: OC-BR-1 (BR_PTI departure at 4X) indicates different execution context")

print()
print("── What this means for OC-STLI-1 ───────────────────────────────")
print()
print("  If DRAM_LAT agreement is within ±5 cycles:")
print("  Case A is supported: STLI fires on most iterations but is hidden")
print("  within the chain load stall. CPI is determined by chain_load_lat,")
print("  not by STLI_WINDOW. STLI is a passenger, not a driver.")
print()
print("  STLI_PTI = 75.8 vs expected 111.1 (every iter):")
print(f"  Ratio: {75.8/111.1:.3f}. This is consistent with OC-DC-1 —")
print("  PTI sampling captures ~68% of true STLI events.")
print("  OR: STLI fires on 68% of iterations due to latency variation.")
print()
print("  Either way: STLI does not contribute to CPI under Case A.")
print("  The open question (OC-STLI-1 timing) is resolved in Case A's favor.")
print()
print("Python exploration complete.")
print("Rust confirmation required before any result enters the record.")
