# Processor Relation Ledger — Metatron Dynamics, Inc.
# Ryzen 5 7600X / AMD Zen 4 Microarchitecture
# Version 1.0 — 2026-08-30
#
# Sources:
#   [SOG]  AMD Software Optimization Guide for the AMD Zen4 Microarchitecture
#          Publication 57647 Rev. 1.01, April 2023 (AMD Public Use)
#   [PPR]  Processor Programming Reference for AMD Family 19h Models 10h-1Fh
#          Publication 55901 (referenced in SOG §1.2; not reproduced here)
#   [PROD] Ryzen 5 7600X product specification (AMD.com)
#
# Bounded over D. No claim beyond D.
#
# ── Founding Constraint ───────────────────────────────────────────────────────
#
# Nothing in this ledger is treated as a primitive.
# Every function is a declared relation between states.
# Every state variable is a container for declared relations, not an axiom.
# Names (Fetch, Execute, etc.) are shorthand for relations, not primitives.
#
# Three admissible observability declarations:
#   OBS   — directly observable: a specific uProf counter maps to this relation
#   PART  — partially observable: a counter correlates but does not directly
#            measure; the gap between counter and relation is declared
#   NONE  — not observable under current instrumentation (assess_ext profile);
#            architecturally declared but no current counter reaches it
#
# "NONE" is a complete and admissible declaration.
# It is not a gap to fill with inference.
#
# ── Layer Structure ───────────────────────────────────────────────────────────
#
# Layer 1: Workload relations    — what relations the workload presents
# Layer 2: Processor relations   — what transformations the processor performs
# Layer 3: Observable relations  — what instrumentation can actually measure
#
# This ledger declares Layer 2 in full.
# Layer 1 is declared in the factorial block (probe.rs, M_declaration.md).
# Layer 3 is declared in the H vector (execution_record.md).
#
# ── State Container Declarations ─────────────────────────────────────────────
#
# The processor state X_t is decomposed into the following declared containers.
# Each container is a set of relations, not a primitive.
#
# I — Instruction/Control-Flow State
#   Contains: instruction pointer (RIP), fetch address, decode state,
#   op-cache state, instruction byte queue contents.
#   Not a primitive: RIP is a relation between retired instructions and
#   the next fetch address. Fetch address is a relation between RIP,
#   branch prediction output, and cache line boundaries.
#
# R — Register State
#   Contains: architectural register file (64-bit GPRs, XMM/YMM/ZMM),
#   physical register file (224 integer PRF entries, SOG §2.10.3),
#   rename mapping table (logical → physical), flag register file (~108
#   free entries, SOG §2.10.3).
#   Not a primitive: a register value is a relation between the instruction
#   that produced it and the physical register allocated to hold it.
#   The value does not exist independently of that provenance chain.
#
# M — Memory State
#   Contains: DRAM contents at declared physical addresses, write-combining
#   buffer state (64-byte aligned, SOG §2.13.4), store queue committed
#   but not yet written entries.
#   Not a primitive: a memory value is a relation between a physical address,
#   a store operation, and the time ordering of all stores to that address.
#
# C — Cache Hierarchy State
#   Contains: L1 I-cache (32KB, 8-way, SOG §2.6.1), L1 D-cache (32KB,
#   8-way, SOG §2.6.2), L2 unified (1MB, 8-way, SOG §2.6.3), L3 shared
#   (32MB, 16-way, [PROD]; SOG §2.6.4 states "up to 96MB depending on
#   configuration"), shadow tags for each L2 in the complex (SOG §2.6.4).
#   Not a primitive: a cache line's presence at a tier is a relation between
#   a physical address, the last access to that address, and the replacement
#   policy at each tier.
#
# T — Address Translation State
#   Contains: L1 ITLB (64 entries, SOG §2.7.1), L1 DTLB (72 entries,
#   SOG §2.7.1), L2 ITLB (512 entries, SOG §2.7.2), L2 DTLB (3072
#   entries, SOG §2.7.2), Page Directory Cache (64 entries, SOG §2.7.3),
#   6 hardware page table walkers (SOG §2.7.3).
#   Not a primitive: a TLB entry is a relation between a virtual page
#   address and a physical page address, valid only within a declared
#   address space and until invalidated by a declared event.
#
# Q — Queue, Buffer, and In-Flight State
#   Contains: instruction byte queue (24 entries / 16 bytes each, SOG §2.9),
#   op-cache (6.75K ops, 64 sets × 12 ways, SOG §2.9.1), load queue
#   (48 uncompleted + 88 completed, SOG §2.12), store queue (64 entries,
#   SOG §2.12), miss address buffer (24 entries, SOG §2.12), write-combining
#   buffer (64-byte aligned entries, SOG §2.13.4), retire queue (320 entries
#   non-SMT, SOG §2.10.3), fetch window tracking structure (96 entries /
#   48 in SMT, SOG §2.8.1.6).
#   Not a primitive: a queue entry is a relation between an instruction,
#   its declared position in program order, and its current completion state.
#
# B — Branch Prediction State
#   Contains: L1 BTB (1536 entries, SOG §2.8.1.2), L2 BTB (7680 entries,
#   SOG §2.8.1.2), return address stack (32 entries per thread, SOG §2.8.1.3),
#   indirect target predictor (3072 entries, SOG §2.8.1.4), conditional
#   branch direction predictor (global history scheme, SOG §2.8.1.5).
#   Not a primitive: a branch prediction is a relation between the current
#   fetch address, the global branch history, and a predicted target address.
#   The prediction is not a fact about the branch — it is a relation between
#   observed prior branch behavior and expected future behavior.
#
# E — Execution Resource State
#   Contains: 4 integer ALU pipes (SOG §2.10.2), branch execution unit
#   (SOG §2.10.2), 3 AGUs (SOG §2.10.2), 3 store data movement units
#   (SOG §2.10.2), 4 FP pipes (SOG §2.11), integer divider, integer
#   multiplier (SOG §2.10.2).
#   Not a primitive: an execution unit's availability is a relation between
#   the current cycle, the instruction occupying that unit (if any), and
#   the latency of that instruction's operation.
#
# S — Scheduling and Dependency State
#   Contains: 4 integer scheduler queues (SOG §2.10.1), 2 FP scheduler
#   queues (SOG §2.11), operand availability tracking, dependency chains.
#   Not a primitive: a dependency is a relation between two instructions
#   where the output of one is required as input to the other, mediated
#   by the physical register file and the rename map.
#
# K — Clock and Progression State
#   Contains: cycle counter, retired instruction counter, in-flight
#   instruction count.
#   Not a primitive: a clock cycle is a relation between two successive
#   stable states of the processor's sequential logic. CPI is a derived
#   relation between the cycle count and the retired instruction count —
#   it is an observable output of the system, not an input or a primitive.

---

## Part I — Instruction Pipeline Relations

### PRL-I-01: Instruction Pointer Progression
**Relation:** RIP_t → next fetch address
**Input state:** Current RIP (I), branch prediction output (B)
**Output state:** Fetch address (I)
**Transformation:** If no branch identified in current fetch block:
  next_addr = next 64-byte aligned block (SOG §2.8.1.1)
  If branch identified: next_addr = BTB prediction output (B)
**Source:** SOG §2.8.1.1
**Observable:** NONE — RIP progression is not directly countered.
  RETIRED_INST PTI (OBS) counts retired instructions, not fetch addresses.
**Verification status:** Architecturally declared. Not measurable per-cycle
  under assess_ext.

---

### PRL-I-02: Instruction Fetch
**Relation:** Fetch address → instruction bytes
**Input state:** Fetch address (I), L1 I-cache state (C)
**Output state:** Instruction byte queue entry (Q)
**Transformation:**
  If fetch address hits L1 I-cache: deliver 32 bytes per cycle (SOG §2.9)
  If L1 miss: request from L2 (SOG §2.6.1)
  If L2 miss: request from L3
  If L3 miss: request from DRAM
  Fetch block: 32-byte aligned, within 64-byte cache line (SOG §2.9)
**Source:** SOG §2.6.1, §2.9
**Observable:** NONE — instruction fetch hits/misses not in assess_ext H vector.
  L1_DC_ACCESSES (OBS) counts data cache, not instruction cache.
**Verification status:** Architecturally declared. L1 I-cache behavior
  not directly observable under current instrumentation.

---

### PRL-I-03: Op-Cache Lookup
**Relation:** Fetch address → cached macro-ops (if present)
**Input state:** Fetch address (I), op-cache state (Q)
**Output state:** Up to 9 macro-ops per cycle (Q), or cache miss → decode
**Transformation:**
  Op-cache: 64 sets × 12 ways, up to 9 macro-ops per entry (SOG §2.9.1)
  Hit: bypass fetch/decode pipeline, deliver 9 macro-ops/cycle
  Miss: fall through to instruction decode (PRL-I-04)
  Transition only at taken branches (SOG §2.9.1)
  Hot loop limit: 6.75K ops per thread (SOG §2.9.1)
**Source:** SOG §2.9.1
**Observable:** NONE — op-cache hit/miss rate not in assess_ext H vector.
**Verification status:** Architecturally declared. Op-cache behavior not
  directly measurable under current instrumentation.
**Declared significance:** For the chain workloads in the factorial block,
  the inner loop is small (estimated <20 instructions). Likely resident
  in op-cache after warm-up passes. Op-cache hit rate is therefore
  expected high at all nodes but unverified under current instrumentation.

---

### PRL-I-04: Instruction Decode
**Relation:** Instruction bytes → macro-ops
**Input state:** Instruction byte queue (Q)
**Output state:** Macro-ops dispatched to rename (Q)
**Transformation:**
  Up to 4 instructions decoded per cycle from 32-byte window (SOG §2.9)
  Fastpath single: 1 macro-op
  Fastpath double: 2 macro-ops
  Microcode: >2 macro-ops (variable)
  Branch fusion: CMP/TEST + conditional branch → 1 macro-op (SOG §2.9.3)
  NOP fusion: NOP + integer fastpath → 1 macro-op (SOG §2.9.5)
**Source:** SOG §2.3, §2.9, §2.9.3, §2.9.5
**Observable:** PART — RETIRED_INST PTI (OBS) counts retired instructions,
  not decoded macro-ops. Fusion events are not separately countered.
  Macro-op count per instruction is not directly observable.
**Verification status:** Architecturally declared. Decode throughput
  inferred from RETIRED_INST PTI combined with workload instruction mix.

---

### PRL-I-05: Register Rename
**Relation:** Logical register operands → physical register assignments
**Input state:** Macro-ops with logical register references (Q),
  rename mapping table (R), physical register file availability (R)
**Output state:** Macro-ops with physical register assignments (Q),
  updated rename map (R)
**Transformation:**
  Integer PRF: 224 physical registers (SOG §2.10.3)
  Up to 38 per thread mapped to architectural or µarch temporary state
  Remaining available for out-of-order rename
  Flag PRF: ~108 free registers (SOG §2.10.3)
  Zero-cycle moves: MOV r64,r64 and variants (SOG §2.9.6) — rename only,
    no execution unit required
**Source:** SOG §2.10.3, §2.9.6
**Observable:** NONE — rename mapping state is not externally observable.
  Physical register pressure is not countered under assess_ext.
**Verification status:** Architecturally declared. Register pressure at
  the factorial nodes is not measurable under current instrumentation.
**Declared significance:** For chain workloads: one architectural register
  holds the current chain pointer per iteration. Register pressure is
  expected low (small loop body). PRF exhaustion is not a candidate
  mechanism for observed CPI variations.

---

### PRL-I-06: Dispatch
**Relation:** Renamed macro-ops → scheduler queues
**Input state:** Renamed macro-ops (Q), scheduler availability (S)
**Output state:** Macro-ops in integer or FP scheduler (S)
**Transformation:**
  Up to 6 macro-ops dispatched per cycle (SOG §2.9.8)
  Distribution constraints (SOG §2.9.8):
    ≤1 integer divide per window of 6
    ≤2 integer multiplies
    ≤2 branches
    ≤3 stores + loads without ALU
    ≤4 ALU operations
  4 integer scheduler queues, 2 FP scheduler queues (SOG §2.10.1, §2.11)
**Source:** SOG §2.9.8, §2.10.1
**Observable:** NONE — dispatch rate and scheduler occupancy not in
  assess_ext H vector.
**Verification status:** Architecturally declared.

---

### PRL-I-07: Schedule and Issue
**Relation:** Ready macro-ops → execution units
**Input state:** Scheduler queues (S), operand availability (S, R),
  execution unit availability (E)
**Output state:** Micro-ops issued to execution pipelines (E)
**Transformation:**
  Scheduler tracks operand availability and dependency information (SOG §2.10.1)
  Issues when: operands available AND execution unit available
  Issues oldest eligible micro-op (in-order within dependency constraints)
  Can issue out-of-order across independent chains
  Each scheduler: 1 micro-op per cycle per associated pipeline (SOG §2.10.1)
**Source:** SOG §2.10.1
**Observable:** NONE — scheduler issue events not directly countered.
  %SMT_CONTENTION (OBS via assess_ext) measures resource contention in
  SMT mode; in single-thread mode this is near zero (confirmed in H vector).
**Verification status:** Architecturally declared. Scheduler behavior
  is not directly observable under current instrumentation.

---

### PRL-I-08: Execute
**Relation:** Operand values → result values
**Input state:** Operand values in physical registers (R), execution
  unit resources (E)
**Output state:** Result values written to physical registers (R)
**Transformation (by unit type, SOG §2.10.2, §2.11):**
  ALU (4 pipes): integer arithmetic, logical, shift — 1-cycle latency
  ALU0: divide (8+ cycles, data-dependent), branch execution
  ALU1: multiply (3-cycle latency), CRC, PDEP/PEXT
  BRU: branch resolution (separate from ALU0)
  AGU (3 units): address generation for loads and stores
  FP pipes (4): FMUL, FADD, FCVT, FDIV, FMISC (SOG Table 2)
  Simple addressing: 4-cycle integer load-to-use latency (SOG §2.12)
  Complex addressing: 5-cycle integer load-to-use latency (SOG §2.12)
**Source:** SOG §2.10.2, §2.11, §2.12, §A.1
**Observable:** PART — CPI (OBS via assess_ext) measures aggregate
  execution throughput. Individual execution unit utilization rates
  are not in the assess_ext H vector. Integer multiply/divide latency
  effects are not separately countered.
**Verification status:** CPI is directly measured. Unit-level utilization
  is not measurable under current instrumentation.

---

### PRL-I-09: Retirement
**Relation:** Completed micro-ops → architectural state update
**Input state:** Retire queue (Q), completion status of all outstanding
  operations (Q)
**Output state:** Architectural register state updated (R), retire queue
  entry freed (Q), instruction count incremented (K)
**Transformation:**
  Retire control unit tracks up to 320 macro-ops in-flight (SOG §2.10.3)
  In-order commit: up to 8 retire queue entries per cycle (SOG §2.10.3)
  A macro-op retires when all corresponding micro-ops have completed
  Exception processing and recovery is the RCU's final arbiter (SOG §2.10.3)
**Source:** SOG §2.10.3
**Observable:** OBS — RETIRED_INST PTI directly counts retired instructions.
  Retirement rate per cycle is derivable from RETIRED_INST and CPI.
**Verification status:** Directly measured via RETIRED_INST PTI in H vector.

---

## Part II — Memory Access Relations

### PRL-M-01: Store Address Generation
**Relation:** Store instruction operands → store virtual address
**Input state:** Base register, index register, displacement (R)
**Output state:** Store virtual address in store queue (Q)
**Transformation:**
  AGU computes: base + index×scale + displacement (SOG §2.12)
  Simple mode (base+disp, base+index, unscaled): 4-cycle latency
  Complex mode (base+index+disp, scaled index): 5-cycle latency
  Store queue entry allocated at dispatch (SOG §2.12)
**Source:** SOG §2.12
**Observable:** NONE — store address generation latency not separately
  countered. Misaligned stores detectable via MISALIGNED_LOADS PTI (OBS)
  but does not distinguish address generation from data write.

---

### PRL-M-02: Load Address Generation
**Relation:** Load instruction operands → load virtual address
**Input state:** Base register, index register, displacement (R)
**Output state:** Load virtual address submitted to TLB and cache (T, C)
**Transformation:**
  Same AGU as stores (SOG §2.12)
  Load queue entry allocated at dispatch (SOG §2.12)
  Load queue: 48 uncompleted + 88 completed entries (SOG §2.12)
  MAB: 24 outstanding in-flight cache misses (SOG §2.12)
**Source:** SOG §2.12
**Observable:** PART — L1_DC_ACCESSES PTI (OBS) counts L1 D-cache accesses,
  which includes load address lookups. Does not separate address generation
  from cache lookup phase.

---

### PRL-M-03: Address Translation (TLB Lookup)
**Relation:** Virtual address → physical address
**Input state:** Virtual address (M), L1 DTLB state (T), L2 DTLB state (T)
**Output state:** Physical address (M), or page walk initiated (T)
**Transformation:**
  L1 DTLB lookup: 72 entries, fully associative (SOG §2.7.1)
    Hit: physical address delivered, proceed to cache lookup
    Miss: L2 DTLB lookup
  L2 DTLB lookup: 3072 entries, 24-way (SOG §2.7.2)
    Hit: physical address delivered (holds PDEs for faster walks)
    Miss: hardware page table walk initiated
  Page walk: up to 6 concurrent walkers (SOG §2.7.3)
    Walk uses PDEs cached in L2 DTLB when available
    With L2 DTLB saturated: walk accesses memory (3+ cache-level accesses)
  UTAG / way-predictor: linear-address-based tag enables single-way read
    before physical address resolved (SOG §2.6.2.3)
    Can mismatch: hit predicted but miss (fill request to L2)
    Can mismatch: miss predicted but hit (fill request, utag updated)
**Source:** SOG §2.7.1, §2.7.2, §2.7.3, §2.6.2.3
**Observable:** NONE — TLB hit/miss rate is not in the assess_ext H vector.
  TLB miss rate is derived (PART) from DRAM_PTI and L3_PTI as described
  in substrate_model.rs, but the derivation is indirect.
  A dedicated TLB-miss counter exists in the Zen 4 PMU [PPR] but is not
  included in the assess_ext profile used in this experiment.
**Verification status:** Architecturally declared. TLB behavior is partially
  characterized via miss distribution inference. Direct measurement requires
  a custom uProf profile including TLB PMU events from [PPR].
**Declared open condition:** OC-TLB-1 — TLB miss rate is inferred, not
  measured directly. A custom profile adding TLB miss events would close this.

---

### PRL-M-04: L1 Data Cache Lookup
**Relation:** Physical address → cache line data (or miss)
**Input state:** Physical address (M), L1 D-cache state (C)
**Output state:** Data returned to load queue (Q), or L2 fill request
**Transformation:**
  L1 D-cache: 32KB, 8-way set associative (SOG §2.6.2)
  Cache line: 64 bytes
  Hit latency: 4 cycles integer, 7 cycles FP (SOG §2.6.2, §A.1)
  Write-back cache: stores committed to L1 on retirement (SOG §2.6.2)
  3 memory operations per cycle max: all loads, or ≤2 stores (SOG §2.6.2)
  Bank conflicts: possible when address bits 5:3 collide (SOG §2.6.2.1)
  UTAG mismatch penalty: fill request to L2, utag updated (SOG §2.6.2.3)
**Source:** SOG §2.6.2, §2.6.2.1, §2.6.2.3
**Observable:** OBS — %L1_DC_MISSES (in H vector) measures L1 D-cache miss
  rate directly. L1_DC_ACCESSES PTI counts total L1 accesses.
**Verification status:** Directly measured. %L1_DC_MISSES confirmed in H
  vector for all 16 nodes.

---

### PRL-M-05: L2 Cache Lookup
**Relation:** L1 miss address → cache line data (or miss)
**Input state:** Miss address (M), L2 cache state (C)
**Output state:** Cache line returned to L1 (C), or L3 fill request
**Transformation:**
  L2: 1MB unified, 8-way, inclusive of L1 (SOG §2.6.3)
  L2 to L1 data path: 32 bytes wide (SOG §2.6.3)
  Hit latency: ≥14 cycles (SOG §A.1)
**Source:** SOG §2.6.3, §A.1
**Observable:** OBS — L1_DEMAND_DC_REFILLS_LOCAL_L2 PTI (in H vector as
  L2_PTI) counts L2 refills to L1.
**Verification status:** Directly measured via L2_PTI in H vector.

---

### PRL-M-06: L3 Cache Lookup
**Relation:** L2 miss address → cache line data (or miss)
**Input state:** Miss address (M), L3 cache state (C), shadow tags (C)
**Output state:** Cache line returned (C), or DRAM request
**Transformation:**
  L3: 32MB, 16-way, shared by 8 cores in CCX [PROD]; SOG §2.6.4
  Write-back cache; populated by L2 victims (SOG §2.6.4)
  Hit: line invalidated from L3 if store hit or single-core read (SOG §2.6.4)
  Line remains on code fetch or multi-core read (SOG §2.6.4)
  Shadow tags: if L2 miss AND L3 miss, shadow tags consulted for
    cache-to-cache transfer within CCX (SOG §2.6.4)
  Hit latency: average 50 cycles (SOG §A.1)
**Source:** SOG §2.6.4, §A.1; [PROD] for 32MB capacity
**Observable:** OBS — L1_DEMAND_DC_REFILLS_LOCAL_CACHE PTI (L3_PTI in H
  vector) counts L3 refills to L1.
**Verification status:** Directly measured via L3_PTI in H vector.

---

### PRL-M-07: DRAM Access
**Relation:** L3 miss address → data from system memory
**Input state:** Miss address (M), DRAM state (M),
  memory controller state (not in SOG scope)
**Output state:** Cache line returned and inserted at L3 and L1 (C, M)
**Transformation:**
  Memory controller mediates DRAM access (not fully described in SOG)
  DDR5-5600 interface on Ryzen 5 7600X [PROD]
  Latency: approximately 160–220 cycles (declared approximate;
    not in SOG; standard DDR5-5600 range; only non-SOG constant in model)
**Source:** [PROD] for DDR5-5600 interface; latency declared approximate
**Observable:** OBS — L1_DEMAND_DC_REFILLS_LOCAL_DRAM PTI (DRAM_PTI in H
  vector) counts DRAM refills to L1.
**Verification status:** Directly measured via DRAM_PTI. Latency value
  is approximate (declared range 160–220 cycles).
**Declared open condition:** OC-DRAM-1 — DRAM access latency is the only
  non-SOG constant in the model. Closing this requires a memory latency
  measurement (e.g., pointer-chase at N large enough to force all DRAM
  accesses, measured with uProf timing).

---

### PRL-M-08: Store-to-Load Forwarding
**Relation:** Pending store data → load result (bypassing cache)
**Input state:** Store queue entry (Q), load virtual address (M)
**Output state:** Load result satisfied from store queue (Q), or
  forwarding failure → load must wait for store to commit
**Transformation:**
  STLF eligibility: linear address bits[11:0] match between store and load
    (SOG §2.12)
  STLF condition: older store must contain all load bytes, and store data
    must be available in store queue (SOG §2.12)
  No alignment constraint relative to store or 64B boundary (SOG §2.12)
  Failure (STLI): when store and load share bits[11:0] but differ in
    upper bits — forwarding attempted but fails; load must re-execute
    (SOG §2.12)
  STLI penalty: pipeline refill cost, approximately 13 cycles
    (declared approximate; exact penalty not stated in SOG §2.12;
    derived from store queue depth and pipeline depth)
**Source:** SOG §2.12
**Observable:** OBS — STLI_OTHER PTI (in H vector) directly counts
  store-to-load forwarding failures.
**Verification status:** Directly measured. STLI_OTHER PTI confirmed
  elevated at D=chain-8 nodes in H vector.

---

### PRL-M-09: Prefetch Relations
**Relation:** Observed access pattern → speculative cache line fetch
**Input state:** Access history (C, M), prefetcher state (not externally
  declared; internal to hardware)
**Output state:** Additional cache lines loaded speculatively (C)
**Transformation (SOG §2.12.1):**
  L1 Stream: detects sequential ascending/descending patterns
  L1 Stride: detects constant-stride patterns per-instruction
  L1 Region: detects correlated access patterns within a region
  L2 Stream: detects sequential patterns at L2 level
  L2 Up/Down: fetches next or previous line for all accesses
  Prefetcher state and configuration: internal, not externally observable
  SOG §2.12.1 declares: "random access patterns may be hard to predict"
    and "can lead to prefetching data that will not eventually be used"
**Source:** SOG §2.12.1
**Observable:** PART — INEFFECTIVE_SW_PF PTI (OBS) counts ineffective
  SOFTWARE prefetch instructions only; hardware prefetch effectiveness
  is not separately countered under assess_ext.
  Prefetch behavior is inferred from cache miss distribution (L2_PTI,
  L3_PTI, DRAM_PTI) — when these are low for sequential access, prefetch
  is effective; when high for scrambled access, prefetch has failed.
**Verification status:** Partially characterized by inference from miss
  distribution. Direct hardware prefetch counter not in assess_ext.
**Declared significance:** Prefetch failure is a declared contextual factor
  at scrambled-access nodes (G0100, G0110, G0101, G0111, G1100, G1110,
  G1101, G1111). Confirmed indirectly by elevated L3_PTI/DRAM_PTI at
  scrambled vs sequential counterpart nodes.

---

### PRL-M-10: Write-Combining
**Relation:** Multiple stores to aligned region → single merged write
**Input state:** Store operations to WC memory type or streaming stores (Q)
**Output state:** Single 64-byte write to memory (M)
**Transformation:**
  WCB: 64-byte aligned write buffers (SOG §2.13.4)
  Combines writes within 64-byte aligned regions
  Closed by: no WCB available, I/O op, serializing instruction, lock,
    UC read/write, TLB AD bit set, SFENCE/MFENCE, interrupt (SOG Table 3)
**Source:** SOG §2.13, §2.13.4, Table 3
**Observable:** OBS — WCB_WRITE PTI and %WCB_NOT_FULLLINE64B_TO_CLOSE
  and %WCB_CLOSE_TO_WRITE are in the H vector.
  At all 16 factorial nodes: WCB_WRITE PTI is near zero.
  WCB is not active in the current workload (array traversal, not streaming).
**Verification status:** Measured. WCB is confirmed inactive at all nodes.
  Not a factor in the factorial block CPI variations.

---

## Part III — Branch and Speculation Relations

### PRL-B-01: Branch Target Prediction
**Relation:** Fetch address → predicted target address
**Input state:** Current fetch address (I), BTB state (B)
**Output state:** Predicted next fetch address (I)
**Transformation:**
  L1 BTB: 1536 entries; zero prediction bubbles for direct branches;
    1-cycle bubble for calls, returns, indirect (SOG §2.8.1.2)
  L2 BTB: 7680 entries; 3-cycle bubbles if differs from L1 (SOG §2.8.1.2)
  Each entry holds up to 2 branches (SOG §2.8.1.2)
  Pair prediction: 2 fetches predicted per cycle when applicable
**Source:** SOG §2.8.1.2
**Observable:** NONE — BTB hit/miss rate not in assess_ext H vector.
  Branch prediction is partially characterized by %RETIRED_BR_INST_MISP
  (OBS) which measures misprediction rate, not prediction source (L1/L2 BTB).

---

### PRL-B-02: Branch Direction Prediction
**Relation:** Conditional branch → predicted taken/not-taken
**Input state:** Branch address (I), global branch history (B),
  conditional predictor state (B)
**Output state:** Predicted direction (I), updated history (B)
**Transformation:**
  Global history scheme: tracks previously executed branches (SOG §2.8.1.5)
  Branches with both taken and not-taken history use conditional predictor
  Never-taken branches: not tracked in global history
  Biased not-taken branches preferred (SOG §2.8.1.5)
  Branch history depth: not stated in SOG; internal to predictor
**Source:** SOG §2.8.1.5
**Observable:** PART — %RETIRED_BR_INST_MISP (OBS) measures aggregate
  misprediction rate. Does not distinguish direction predictor from
  target predictor failures. Does not expose global history state.

---

### PRL-B-03: Return Address Prediction
**Relation:** RET instruction → predicted return address
**Input state:** RAS state (B, 32 entries per thread, SOG §2.8.1.3)
**Output state:** Predicted return address (I)
**Transformation:**
  CALL pushes return address onto RAS (SOG §2.8.1.3)
  RET pops RAS
  Recovery mechanisms for incorrect speculative pushes/pops (SOG §2.8.1.3)
  If unrecoverable: RAS invalidated and restored to consistent state
**Source:** SOG §2.8.1.3
**Observable:** NONE — RAS state not externally observable.
  RAS mispredictions included in %RETIRED_BR_INST_MISP (OBS) but
  not separately identified.
**Declared significance:** Not a factor at factorial block nodes.
  Workload is a tight inner loop with no subroutine calls.

---

### PRL-B-04: Speculative Execution
**Relation:** Predicted control flow → instructions fetched and executed
  before branch resolution
**Input state:** Branch prediction output (B), ROB state (Q)
**Output state:** Speculatively executed instructions in ROB (Q, R)
**Transformation:**
  ROB holds up to 320 speculative instructions in non-SMT mode (SOG §2.10.3)
  Instructions execute speculatively on predicted path
  Results held in physical registers pending retirement (R)
  Retirement gated on branch resolution (SOG §2.10.3)
**Source:** SOG §2.10.3
**Observable:** NONE — speculative instruction count not directly observable.
  ROB occupancy not in assess_ext H vector.

---

### PRL-B-05: Branch Resolution and Misprediction Recovery
**Relation:** Executed branch → confirmed or corrected control flow
**Input state:** Branch execution result (E), predicted target (B),
  ROB state (Q)
**Output state:** If correct: branch retired, ROB advances
  If mispredicted: ROB flushed from branch onward, RIP corrected (I),
  fetch restarted from correct target (I)
**Transformation:**
  Comparison: P(b) vs R(b) — predicted vs resolved target
  If P(b) = R(b): no action beyond retirement
  If P(b) ≠ R(b): pipeline flush, penalty 11–18 cycles (SOG §2.8)
    Common case: 13 cycles (SOG §2.8)
    Op-cache fed: lower end of range
    Decode fed: higher end of range
**Source:** SOG §2.8
**Observable:** OBS — %RETIRED_BR_INST_MISP directly measures fraction
  of retired branch instructions that were mispredicted.
  RETIRED_BR_INST PTI counts total retired branch instructions.
**Verification status:** Directly measured. Confirmed at all 16 nodes.
  Key finding: %BR_MISP collapses to baseline at G0111/G1111 despite
  B=branchy intervention — declared finding, not an anomaly.

---

## Part IV — Execution Resource Relations

### PRL-E-01: Integer ALU Operations
**Relation:** Integer operands → integer result
**Input state:** Operand values in PRF (R), ALU availability (E)
**Output state:** Result in PRF (R)
**Transformation:**
  4 ALU pipes; each handles general-purpose integer operations (SOG §2.10.2)
  ALU0: additionally handles divide and branch execution
  ALU1: additionally handles multiply (3-cycle), CRC, PDEP/PEXT
  Most simple operations: 1-cycle latency (SOG §2.10.2)
  Divide: 8 cycles + 1 cycle per 9 bits of quotient (SOG §2.10.2)
**Source:** SOG §2.10.2
**Observable:** NONE — individual ALU utilization not in assess_ext.
  Aggregate effect visible in CPI (OBS).

---

### PRL-E-02: Address Generation
**Relation:** Base, index, scale, displacement → effective address
**Input state:** Register operands (R), AGU availability (E)
**Output state:** Effective address → submitted to TLB (T)
**Transformation:**
  3 AGUs for all load and store address generation (SOG §2.10.2)
  Simple mode: 4-cycle load-to-use (SOG §2.12)
  Complex mode (scaled index, base+index+disp): 5-cycle (SOG §2.12)
  Non-zero segment base: +1 cycle (not additive with complex mode penalty)
**Source:** SOG §2.10.2, §2.12
**Observable:** NONE — AGU latency not separately countered.
  Effect visible in load-to-use latency contribution to CPI (OBS).

---

### PRL-E-03: Floating-Point and SIMD Operations
**Relation:** FP/vector operands → FP/vector result
**Input state:** FP/vector operand values (R), FP unit availability (E)
**Output state:** FP/vector result in PRF (R)
**Transformation:**
  4 FP execution pipes (SOG §2.11, Table 2)
  Pipe 0,1: FMUL, FMA, FP compares
  Pipe 0,1: FADD
  Pipe 0,1: FCVT (convert)
  Pipe 0: FDIV, SQRT (shared with FMISC)
  All pipes: FMISC (moves, logical)
  Store/F2I: pipes 4,5 (SOG §2.11)
  256-bit datapath; 512-bit uses 2 consecutive cycles (SOG §2.11)
**Source:** SOG §2.11, Table 2
**Observable:** NONE — FP unit utilization not in assess_ext.
**Declared significance:** Not a factor at factorial block nodes.
  Workload is integer pointer-chase with no FP operations.

---

## Part V — Dependency and Ordering Relations

### PRL-S-01: Data Dependency
**Relation:** Producer instruction output → consumer instruction input
**Input state:** Physical register containing produced value (R),
  consumer instruction awaiting operand (S)
**Output state:** Consumer operand available, consumer eligible for issue (S)
**Transformation:**
  Dependency tracked through physical register file (R)
  Consumer cannot issue until producer writes result (SOG §2.10.1)
  For chain workloads: each load depends on the result of the prior load
    (pointer chase). This is a RAW (read-after-write) dependency that
    prevents out-of-order reordering within a chain.
**Source:** SOG §2.10.1
**Observable:** NONE — dependency chain depth not directly countered.
  Effect visible in CPI elevation at D=chain-8 nodes (OBS, indirect).
**Verification status:** Architecturally declared. The dependency structure
  of the chain workload is declared in probe.rs (chain array construction).
  Its effect on CPI is confirmed by the k-chain compound model.

---

### PRL-S-02: Memory Ordering
**Relation:** Store ordering → load visibility
**Input state:** Store queue ordering (Q), load queue (Q)
**Output state:** Load sees correctly ordered store values
**Transformation:**
  AMD64 memory model: total store ordering within a processor thread
  Load bypassing: loads can bypass older non-conflicting stores (SOG §2.12)
  Load bypassing older loads: supported (SOG §2.12)
  STLF: load can bypass store if forwarding conditions met (PRL-M-08)
  Memory barriers (SFENCE, MFENCE, LFENCE): enforce ordering (SOG §2.15)
**Source:** SOG §2.12, §2.15
**Observable:** PART — STLI_OTHER PTI (OBS) captures ordering failures.
  Correct ordering (the common case) is not separately countered.

---

### PRL-S-03: Load-Store Reordering
**Relation:** Issued loads and stores → dynamic reordering for throughput
**Input state:** Load queue (Q), store queue (Q), MAB (Q)
**Output state:** Reordered execution respecting memory ordering rules
**Transformation:**
  LS unit dynamically reorders: loads bypass older loads (SOG §2.12)
  Loads bypass older non-conflicting stores (SOG §2.12)
  MAB: 24 outstanding in-flight cache misses (SOG §2.12)
  For D=chain-8: chain dependency (PRL-S-01) prevents reordering within
    a chain. Reordering across chains is possible (k-way parallelism).
**Source:** SOG §2.12
**Observable:** NONE — reordering events not directly countered.
  MAB occupancy not in assess_ext H vector.
**Declared significance:** k-way reordering across chains is the mechanism
  providing k=8 parallelism in the compound CPI model.

---

## Part VI — Clock and Throughput Relations

### PRL-K-01: CPI (Cycles Per Instruction)
**Relation:** Clock cycles → retired instructions
**Input state:** Cycle counter (K), retired instruction counter (K)
**Output state:** CPI = cycles / retired_instructions
**Transformation:**
  CPI is NOT a primitive. It is a derived relation between two observables.
  CPI = 1.0 when processor retires one instruction per cycle (throughput bound)
  CPI > 1.0 when processor stalls (memory bound, dependency bound, etc.)
  CPI < 1.0 is possible when multiple instructions retire per cycle
    (superscalar retirement, up to 8 per cycle, SOG §2.10.3)
  CPI observed at the 16 factorial nodes: 1.000–3.274
**Source:** SOG §2.10.3
**Observable:** OBS — CPI directly reported by assess_ext profile.
**Verification status:** Directly measured at all 16 nodes.

---

### PRL-K-02: Instruction Throughput
**Relation:** Dispatch bandwidth → retired instruction rate
**Input state:** Dispatch rate (Q, S), execution latency (E),
  retirement rate (Q)
**Output state:** Instructions retired per cycle
**Transformation:**
  Maximum dispatch: 6 macro-ops per cycle (SOG §2.9.8)
  Maximum retirement: 8 retire queue entries per cycle (SOG §2.10.3)
  Actual throughput = min(dispatch rate, execution rate, retirement rate)
  Bottleneck is the binding constraint in the above minimum
**Source:** SOG §2.9.8, §2.10.3
**Observable:** PART — RETIRED_INST PTI (OBS) measures retirement rate.
  Dispatch rate and the binding bottleneck are not separately observable.

---

## Part VII — Observability Summary

### Complete H vector mapping (assess_ext profile)

| H Vector Field          | Maps to PRL       | Observability |
|-------------------------|-------------------|---------------|
| CPI                     | PRL-K-01          | OBS           |
| %SMT_CONTENTION         | PRL-S-01 (SMT)    | OBS (zero)    |
| RETIRED_BR_INST PTI     | PRL-B-05          | OBS           |
| %RETIRED_BR_INST_MISP   | PRL-B-05          | OBS           |
| L1_DC_ACCESSES PTI      | PRL-M-04          | OBS           |
| %L1_DC_MISSES           | PRL-M-04          | OBS           |
| L1_DEMAND_DC_REFILLS_   |                   |               |
|   LOCAL_DRAM PTI        | PRL-M-07          | OBS           |
| L1_DEMAND_DC_REFILLS_   |                   |               |
|   LOCAL_CACHE PTI       | PRL-M-06          | OBS           |
| L1_DEMAND_DC_REFILLS_   |                   |               |
|   LOCAL_L2 PTI          | PRL-M-05          | OBS           |
| MISALIGNED_LOADS PTI    | PRL-M-01/02       | OBS           |
| STLI_OTHER PTI          | PRL-M-08          | OBS           |
| WCB_WRITE PTI           | PRL-M-10          | OBS           |
| %WCB_NOT_FULLLINE64B    | PRL-M-10          | OBS           |
| %WCB_CLOSE_TO_WRITE     | PRL-M-10          | OBS           |
| SSE_AVX_STALLS PTC      | PRL-E-03          | OBS           |
| INEFFECTIVE_SW_PF PTI   | PRL-M-09 (SW only)| OBS           |

### Processor functions NOT observable under current instrumentation

| PRL        | Function                        | Gap                          |
|------------|---------------------------------|------------------------------|
| PRL-I-01   | Instruction pointer progression | No per-cycle RIP counter     |
| PRL-I-02   | Instruction fetch               | No L1 I-cache miss counter   |
| PRL-I-03   | Op-cache lookup                 | No op-cache hit/miss counter |
| PRL-I-05   | Register rename                 | No PRF pressure counter      |
| PRL-I-06   | Dispatch                        | No dispatch rate counter     |
| PRL-I-07   | Schedule and issue              | No scheduler occupancy       |
| PRL-M-03   | TLB lookup                      | No TLB hit/miss counter      |
| PRL-M-09   | Hardware prefetch               | No HW prefetch counter       |
| PRL-B-01   | Branch target prediction        | No BTB hit/miss counter      |
| PRL-B-03   | Return address prediction       | No RAS counter               |
| PRL-B-04   | Speculative execution           | No ROB occupancy counter     |
| PRL-E-01   | Integer ALU utilization         | No per-unit counter          |
| PRL-E-02   | AGU utilization                 | No per-AGU counter           |
| PRL-S-01   | Dependency chain depth          | No dependency counter        |
| PRL-S-03   | Load-store reordering           | No reorder event counter     |

### Open conditions from this ledger

**OC-TLB-1:** TLB miss rate is inferred from miss distribution,
  not measured directly. A custom uProf profile including Zen 4 TLB
  PMU events [PPR] would close this.

**OC-DRAM-1:** DRAM access latency (180 cycles, declared approximate)
  is the only non-SOG constant in the substrate model. A pointer-chase
  calibration run at large N (forcing all DRAM) would measure this
  directly.

**OC-OC-1:** Op-cache behavior is unobserved. For the chain workloads,
  the inner loop is estimated small enough to reside in op-cache after
  warm-up. This is consistent with low CPI at B=none nodes but is not
  confirmed. A run exceeding op-cache capacity (>6.75K ops hot code)
  would isolate this effect.

---

## Part VIII — Declared Completeness Statement

The Processor Relation Ledger declares all processor functions
identified in SOG 57647 Rev. 1.01 that are activated by the
factorial block workloads. For each function:

- The mathematical transformation is declared (input → relation → output)
- The source is cited
- The observability under assess_ext is declared (OBS / PART / NONE)
- Where NONE: the gap between architectural knowledge and current
  measurement is stated explicitly

The ledger does not claim to be exhaustive of all Zen 4 functions —
SMT-specific interactions, power management relations, security features,
and microcode paths are not activated by the factorial workloads and
are therefore outside the declared domain D.

Within the declared domain:

**Every declared processor function has a disposition.**
**No function is treated as a primitive.**
**No observation is attributed to an unmeasured mechanism.**

The compound CPI model (substrate_model.rs V2.0) is derived from
this ledger. Its predictions use only OBS and declared-approximate
constants. Its residuals fall within declared measurement uncertainty.

OC-V10-1: CLOSED.
OC-TLB-1: OPEN — requires custom PMU profile.
OC-DRAM-1: OPEN — requires calibration run.
OC-OC-1: OPEN — requires op-cache saturation run.

The three remaining open conditions do not affect the compound CPI
model's closure — they represent paths to tighter future verification,
not gaps in the current declared mechanism.

---

## Part VIII Amendment — OC-DRAM-1 Status Correction (2026-08-30)

OC-DRAM-1 declared OPEN (not CLOSED as previously stated in Part VIII).

Calibration runs conducted:
  scrambled-dram-cal at N=8,388,608 and N=16,777,216 (gather workload)
  chained at N=8,388,608 and N=16,777,216 (pointer chase workload)

Declared observations from calibration:
  Gather (scrambled-dram-cal): CPI ≈ 2.07–2.20 at both N values.
    DRAM_PTI: 62.5–71.2. L3_PTI: 23.9–12.4.
    This is a gather workload — independent random accesses.
    MAB parallelism available. NOT equivalent to serialized DRAM access.
    Measures: effective DRAM throughput under parallel access.

  Pointer chase (chained): CPI ≈ 25.6–27.2 at both N values.
    DRAM_PTI: 113.8–130.1. L3_PTI: 24.3–7.5.
    This is a pointer chase — each address depends on prior result.
    Serialized. Measured cycles_per_iter ≈ 101–109.
    This quantity is compound (see OC-DRAM-1a). Not equivalent to DRAM_LAT alone.

Declared finding (independent of mechanism):
  Same working-set class (WS > nominal L3) + different dependency relation
  (gather vs pointer chase) → radically different measured progression.
  Gather: CPI ≈ 2. Pointer chase: CPI ≈ 26. Ratio ≈ 13×.
  Wall-clock: 6.2–6.6 ns/op (gather) vs 94.8–113.6 ns/op (pointer chase).

OC-DRAM-1: OPEN.
  DRAM_LAT not yet isolated as a unique quantity.
  The declaration "WS > nominal L3 forces every access to DRAM" is
  INADMISSIBLE — it is contradicted by measured H vector showing
  substantial local-cache refills alongside DRAM refills.
  Corrected declaration: WS > nominal L3 capacity is the declared
  intervention. Observed DRAM/cache service distribution established by H.

OC-DRAM-1a: NEW — OPEN.
  The chained pointer chase yields cycles_per_iter ≈ 101–109.
  This is a compound quantity. Initial analysis and open frontier
  were stated here prior to assembly decomposition and ΔV intervention.
  Those statements are superseded by the Status Supersession below.

Iteration decomposition for run_chained (source: probe.rs lines 268-279):
  Per iteration: 2 dependent loads.
    Load 1: chain[current] — serializing (pointer chase)
    Load 2: values[current] — forwarded from prior Load 3 (0 effective misses)
    Load 3: values[next] — dependent on Load 1
  Instructions per iteration: ≈ 3.95–3.99 (measured from BR_PTI,
    formula 1000/BR_PTI — SUPERSEDED. See Status Supersession below.)
  DC accesses per iteration: ≈ 1.85–2.04 (measured from L1_DC_ACCESSES PTI
    — interpretation pending. See Status Supersession below.)
  Refills per iteration: ≈ 0.56 (measured — interpretation pending.)

### OC-DRAM-1a Status Supersession (2026-08-30, post assembly decomposition)

The statements above predate the assembly decomposition of run_chained
(I_asm=15, B_asm=4) and the ΔV chain-only intervention. The following
supersedes them. Historical measurements are preserved; interpretations
are corrected.

**Assembly declaration for run_chained (source: probe.s, release build):**
  I_asm = 15 instructions per iteration
  B_asm = 4 branches per iteration (3 bounds checks + 1 loop branch)
  Memory operations = 5 (4 loads + 1 store)
  DRAM-dependent loads = 2: chain[current] (step 4) and values[next] (step 8)
  values[current] (step 7): L1 hit in steady state (reuse from prior step 8)

**ipi formula correction:**
  Prior formula ipi = 1000/BR_PTI is RETIRED for run_chained.
  Reason: 4 branches per iteration, not 1.
  Corrected: ipi = 4 × (1000/BR_PTI) ≈ 15.8–16.0 (consistent with I_asm=15).
  The ≈3.95 figure in the prior section was derived from the retired formula
  and should not be used.

**DC accesses per iteration:**
  From assembly: 5 memory operations per iteration.
  Measured L1_DC_ACCESSES PTI: 468–511 (above the static ratio 333.3).
  Counter-to-instruction mapping requires further declaration (OC-DC-1, see below).
  The ≈1.85–2.04 figure and the 37% PMU sampling rate conclusion are RETIRED.

**ΔV intervention (chained+values → chain-only):**
  Assembly for run_chain_only: I_asm=9, B_asm=2 (declared from probe.s).
  BR_PTI(chain-only, 2X) = 222.6659. Assembly predicts 222.22. Residual: 0.45 (0.2%).
  BR_PTI(chain-only, 4X) = 178.6806. Assembly predicts 222.22.
    Departure: |178.68 − 222.22| = 43.54 PTI (~19.6%).
    This departure is unexplained. See OC-BR-1 below.
  ΔV·CPI(2X) = −23.07. ΔV·CPI(4X) = −25.11.
  The dominant measured CPI cost in chained+values is in the values[] path.
  ΔV·CPI ≠ L_values_next — attribution to a single mechanism not established.
  Assumption A (equal service distribution for chain[] and values_next loads)
  is RETIRED — incompatible with ΔH.

**Current open frontier (replacing prior "Required to close" statement):**

  OC-STLI-1 (OPEN): STLI_OTHER PTI rises from 0.155 → 75.807 (2X) under ΔV.
    Relation between STLI_OTHER PTI, the chain-only instruction sequence,
    and observed CPI is undeclared. Required before any additive STLI
    contribution is assigned to CPI.

  OC-BR-1 (OPEN): BR_PTI(chain-only, 4X) = 178.68 departs from assembly
    prediction 222.22 by ~19.6%, while BR_PTI(chain-only, 2X) = 222.67
    agrees within 0.2%. The departure is unexplained. N changes while
    declared source relation and protocol remain fixed. This is a sharply
    bounded observation requiring declaration before the 4X measurement
    is used in any derivation.

  OC-DC-1 (OPEN): L1_DC_ACCESSES PTI measured at 468–511 for chained+values,
    above the static-ratio prediction of 333.3 (5 ops/15 instr × 1000).
    Counter-to-instruction mapping is undeclared.

  OC-DRAM-1a (OPEN): DRAM_LAT remains undeclared as a unique quantity.
    chain_load_lat is not isolatable until OC-STLI-1 and OC-BR-1 are resolved.

  OC-DRAM-1 (OPEN): DRAM_LAT not yet isolated. Dependent on OC-DRAM-1a closure.

---

## Part IX — Declared Completeness Boundary

### Statement

The Processor Relation Ledger, the substrate model, and the factorial block
measurements constitute the complete declared analysis accessible through
software-based profiling instrumentation (AMD uProf assess_ext) on this hardware.

This boundary is declared, not apologized for.

### What software-based instrumentation can reach

The assess_ext profile exposes 16 hardware performance counter fields.
Every field is mapped to a specific PRL in this ledger (see Part VII).
Every mapped PRL has a declared observability status (OBS, PART, or NONE).
The four-way interaction finding (I_{A,D,S,B} = 1.937) is fully grounded
in OBS fields and requires no model assumptions.

### What software-based instrumentation cannot reach

Fifteen of the twenty-four declared PRLs are marked NONE — not observable
under current instrumentation. These include:
  - Instruction fetch and op-cache behavior
  - Register rename and dispatch state
  - Scheduler occupancy and issue events
  - TLB hit/miss rate (direct)
  - Hardware prefetch effectiveness
  - Branch target prediction source (L1/L2 BTB)
  - ROB occupancy and speculative instruction count
  - Individual execution unit utilization
  - Dependency chain depth

The compound CPI model's residuals (G0111: −0.449, G1111: +0.652 with
DRAM_LAT=180 declared-approximate) lie in the space between OBS fields
and the NONE functions. The residuals are declared, not explained away.

### The instrumentation design question

AMD uProf assess_ext was designed for software performance optimization —
identifying bottlenecks and guiding code changes. It was not designed as
a structural diagnostic instrument for the silicon substrate.

The counters it exposes are a projection of processor internal state onto
a small number of observable dimensions — the ones AMD determined would be
useful for software optimization. Everything outside that projection is not
reachable through this path.

A complete structural accounting of the processor's relational network
would require:
  - Hardware-level access (die-level measurement, internal bus tracing)
  - AMD's internal profiling infrastructure with access to counters not
    exposed in public documentation
  - Or: custom PMU profiles using events from the PPR [PPR] that are not
    included in the assess_ext preset

### Implication for the AMD conversation

What is demonstrable from the outside — with consumer-grade profiling
tools — is already sufficient to:
  1. Establish the four-way interaction finding on real hardware
  2. Ground it in declared processor mathematics (SOG 57647)
  3. Declare where the model closes and where it does not

What AMD could verify from the inside — with access to internal counters
and microarchitectural state not publicly exposed — would close the
remaining open conditions and complete the model in ways that are not
accessible from outside the silicon.

This is not a gap in the work. It is the natural boundary condition of
the measurement framework, declared explicitly rather than hidden.

The remaining open conditions (OC-DRAM-1, OC-DRAM-1a, OC-TLB-1,
OC-OC-1, OC-G1111-2) represent the frontier of what this framework
can address. They are the natural starting point for a collaborative
investigation using AMD's internal instrumentation.

### Op-cache candidate relation (amendment to PRL-I-03)

PRL-I-03 previously stated the inner loop is "likely resident in op-cache
after warm-up." This language is inadmissible in the processor declaration.

Corrected declaration: The SOG §2.9.1 states op-cache capacity is 6.75K ops
and that hot code regions approaching this size may transition between IC
and OC mode. The inner loop instruction count at the factorial nodes is
estimated small (< 20 instructions) but has not been measured. Op-cache
residency is NOT observable under assess_ext. It is NOT declared.

Candidate relation (not a processor declaration):
  If the inner loop is resident in op-cache: fetch/decode overhead is
  eliminated and throughput is up to 9 macro-ops/cycle (vs 4 for IC mode).
  This would lower the effective per-iteration overhead, affecting the
  instructions-per-iteration count.
  Experimental test: run a workload that exceeds op-cache capacity
  (> 6.75K ops hot code) and compare CPI — declared as OC-OC-1.

