# abr-home-system-benchmark

**Relational substrate characterization on Ryzen 5 7600X / DDR5-5600**
**Metatron Dynamics, Inc. — Lompoc, California**

Bounded over D. No claim beyond D.

---

## What This Repo Is

This repository documents a structured hardware measurement study applying
the ABR/ABRCE relational mathematics framework to compute-substrate
characterization on declared consumer hardware. Within ABR/ABRCE, relation
rather than scalar state is treated as the primary structure for
characterization. The study uses declared relational operators alongside
conventional performance observables to characterize how measured hardware
behavior changes with configuration.

The work is empirical and bounded. Every result is traceable to a declared
observable through a declared measurement mapping M. Every open condition
is explicitly named. No claim is made beyond the declared domain D.

---

## Declared Hardware

- CPU: AMD Ryzen 5 7600X (Zen 4, 6 cores, 32 MB L3)
- RAM: 32 GB DDR5-5600 (2× 16 GB Micron, dual channel)
- OS: Windows 11 Home 64-bit
- Profiler: AMD uProf (assess_ext configuration)

---

## Central Finding — V12.0.0

A 16-node factorial experiment across four declared factors on Zen 4:

| Factor | Levels |
|--------|--------|
| A — Access pattern | sequential vs. scrambled |
| D — Dependency structure | independent vs. chain-8 |
| S — Working-set size | S0 ≈ 4 MB vs. S1 ≈ 32 MB |
| B — Branch pattern | branch-free vs. data-dependent |

**Four-way interaction: I_{A,D,S,B} = 1.937**

Measured under uProf assess_ext on declared hardware. Verified through
three independent paths through the 4-cube. Independent of all open
microarchitectural conditions.

The measured effect of changing one factor depends substantially on the
states of the other three. Access pattern, dependency structure,
working-set size, and branching do not compose independently at the
hardware level in this domain.

---

## Assembly Declaration

The chain-only pointer-chase workload (probe.rs, run_chain_only) was
compiled to release assembly and inspected directly. The hot loop
(.LBB13_3) contains nine instructions and two branches, with an
exact-address stack store/load pair at 40(%rsp) and a bounds-check
sequence (cmpq/jae) between the store and subsequent load.

The assembly establishes instruction structure. The timing contribution
of store-to-load forwarding, the bounds check, and their
microarchitectural interaction remain undeclared.

---

## Instrumentation Boundary

The experiment reached a precisely located instrumentation boundary.
Two events needed to disaggregate load-type-specific refills:

- `ls_stlf` (PMCx035) — store-to-load forward hits
- `ls_bad_status2.stli_other` (PMCx024, UMask 0x02) — non-forwardable conflicts

Neither is accessible through the uProf predefined event set on this
hardware. Raw event codes are rejected by AMDuProfCLI. Data Fabric
counters are likewise not exposed on desktop Ryzen through uProf.

This boundary is declared, not inferred. The open conditions it produces:

- **OC-DC-1:** load-type-specific refill disaggregation not achievable
  with available counters
- **OC-STLI-1:** STLI_OTHER rose from ~0 to 75.8 PTI between
  chained+values and chain-only workloads; timing relation to CPI
  undeclared

A technical post on the AMD Developer Forum describes this boundary and
requests input from AMD engineers.

---

## Processor Relation Ledger

docs/processor_relation_ledger.md declares 24 processor relations (PRLs)
across six parts of the Zen 4 pipeline, each with source, transformation,
observability status, and verification status. Observability is declared
as OBS (directly observable), PART (partially observable), or NONE (not
observable under current instrumentation). NONE is a complete and
admissible declaration.

---

## Measurement Infrastructure

**Pass B (assess_ext):** Standard uProf configuration. Collects CPI,
retired instructions, branch behavior, cache fills by tier. Used for
all 16 factorial nodes and chain-only variants.

**Pass A (load-type disaggregation):** Custom six-counter profile
(config/pass_a_load_type.xml). Designed to collect ls_stlf,
ls_bad_status2, and demand fills by source independently. Currently
blocked at instrumentation boundary — events not accessible through
uProf on this hardware. Config is retained as a declared instrument
specification for future use with AMD tooling.

---

## Open Condition Register (current)

| ID | Status | Description |
|----|--------|-------------|
| OC-DRAM-1 | OPEN | DRAM_LAT not isolatable until OC-DC-1 resolved |
| OC-DC-1 | OPEN | Refill PTI aggregates load types; load-type-specific counter not accessible |
| OC-STLI-1 | OPEN | STLI_OTHER timing relation to CPI undeclared |
| OC-BR-1 | OPEN | BR_PTI at 4X departs 19.6% from assembly prediction; unexplained |
| OC-TLB-1 | OPEN | TLB miss rate not directly measured |
| OC-OC-1 | OPEN | Op-cache residency unverified |
| OC-TG-2 | OPEN | S1 elevated variance (light protocol) |
| OC-HW-2 | OPEN | uProf timing not comparable to benchmark.exe timing |

---

## What Is Airtight

- I_{A,D,S,B} = 1.937 — independent of all open conditions
- Assembly declaration for run_chain_only hot loop
- Instrumentation boundary — three components, precisely located
- Pointer chase serialization finding — store-to-load dependency is
  load-bearing for workload structure; cannot be surgically removed

---

## Prior Methodology Note

Early versions of this repo included a comparison of the relational
approach against an O(N²) all-pairs baseline. That comparison was a
methodological error — any O(N) algorithm outperforms O(N²) at scale,
and the result carried no framework-specific signal. It has been retired
from the study.

---

## Build and Run

```powershell
cargo test --release
cargo build --release
```

Measurement sessions (from repo root):

```powershell
$probe = ".\target\release\probe.exe"
$uprof = "C:\Program Files\AMD\AMDuProf\bin\AMDuProfCLI.exe"
$out   = "$HOME\uprof_out"

# 16-node factorial block (Pass B)
& $uprof collect --config assess_ext -o "$out\G0000" $probe linear 524288
# ... (see run_probe_v9.ps1 for full sequence)

# chain-only variants
& $uprof collect --config assess_ext -o "$out\chain_only_2x" $probe chain-only 4194304
```

Assembly emission:

```powershell
cargo rustc --release --bin probe -- --emit=asm
```

---

## Repository Structure

| Path | Contents |
|------|----------|
| src/probe.rs | Isolated measurement binary — factorial block + chain-only workloads |
| src/substrate_model.rs | Processor substrate model — compound CPI predictions |
| src/operators.rs | ABR kernel V7 — A, B, R operators |
| docs/execution_record.md | Full measurement record — all declared hardware runs |
| docs/processor_relation_ledger.md | 24 PRLs — Zen 4 pipeline relation declarations |
| docs/M_declaration.md | Measurement mapping declaration |
| config/pass_a_load_type.xml | Pass A counter specification (instrumentation boundary) |
| sim/ | Python simulation files — chain-only loop exploration |

---

## Version History

| Version | Description |
|---------|-------------|
| V12.0.0 | Pass A load-type profile, ΔR_stack workload, assembly declaration, bounds-check refinement |
| V11.0.0 | 129 tests, four-way interaction I_{A,D,S,B}=1.937 hardware-validated, processor relation ledger |
| V9.0–V10.0 | A×D×S×B factorial block, B dimension added, substrate model |
| V1.0–V2.0 | Regime 1–3 scaffold, initial crossover matrix |

---

*Metatron Dynamics, Inc. — Lompoc, California*
*Bounded over D. No claim beyond D.*
