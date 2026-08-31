# run_probe_v10.ps1 — Metatron Dynamics, Inc.
# Pass B (assess_ext) + Pass A (load-type disaggregation) + ΔR_stack
# probe V10.0 — abr-home-system-benchmark V12.0.0
# Bounded over D. No claim beyond D.
#
# Usage:
#   cd "$HOME\Downloads\abr_home_system_benchmark_v12"
#   cargo test --release
#   cargo build --release
#   .\run_probe_v10.ps1
#
# REQUIRED before running ΔR_stack sections:
#   cargo rustc --release -- --emit=asm
#   Inspect target\release\deps\probe-*.s
#   Find run_chain_only_stack_spill hot loop
#   Confirm buf[toggle] and buf[1-toggle] are distinct stack addresses
#   Confirm chain[black_box(prev)] load unchanged from run_chain_only
#   Record assembly declaration in execution_record.md before proceeding

$probe  = ".\target\release\probe.exe"
$uprof  = "C:\Program Files\AMD\AMDuProf\bin\AMDuProfCLI.exe"
$config = ".\config\pass_a_load_type.xml"
$out    = "$HOME\uprof_out"

New-Item -ItemType Directory -Force -Path "$out\pass_a" | Out-Null

Write-Host ""
Write-Host "======================================================" -ForegroundColor White
Write-Host " probe V10.0 — abr-home-system-benchmark V12.0.0     " -ForegroundColor White
Write-Host " Metatron Dynamics, Inc.  Bounded over D.             " -ForegroundColor White
Write-Host "======================================================" -ForegroundColor White
Write-Host ""

# ══════════════════════════════════════════════════════════════════════════════
# PASS B — assess_ext (H vector baseline, unchanged from V9.0 / V11.0.0)
# ══════════════════════════════════════════════════════════════════════════════
Write-Host "PASS B — assess_ext (16-node factorial + chain-only variants)" -ForegroundColor White
Write-Host ""

# ── A×D×S×B factorial block (16 nodes) ───────────────────────────────────────
Write-Host "B=none nodes..." -ForegroundColor Cyan

& $uprof collect --config assess_ext -o "$out\G0000" $probe linear           524288
& $uprof collect --config assess_ext -o "$out\G0100" $probe scrambled         524288
& $uprof collect --config assess_ext -o "$out\G0010" $probe chains-8-seq      524288
& $uprof collect --config assess_ext -o "$out\G0110" $probe chains-8          524288

& $uprof collect --config assess_ext -o "$out\G1000" $probe linear            4194304
& $uprof collect --config assess_ext -o "$out\G1100" $probe scrambled         4194304
& $uprof collect --config assess_ext -o "$out\G1010" $probe chains-8-seq      4194304
& $uprof collect --config assess_ext -o "$out\G1110" $probe chains-8          4194304

Write-Host ""
Write-Host "B=branchy nodes..." -ForegroundColor Cyan

& $uprof collect --config assess_ext -o "$out\G0001" $probe linear-branchy        524288
& $uprof collect --config assess_ext -o "$out\G0101" $probe scrambled-branchy     524288
& $uprof collect --config assess_ext -o "$out\G0011" $probe chains-8-seq-branchy  524288
& $uprof collect --config assess_ext -o "$out\G0111" $probe chains-8-branchy      524288

& $uprof collect --config assess_ext -o "$out\G1001" $probe linear-branchy        4194304
& $uprof collect --config assess_ext -o "$out\G1101" $probe scrambled-branchy     4194304
& $uprof collect --config assess_ext -o "$out\G1011" $probe chains-8-seq-branchy  4194304
& $uprof collect --config assess_ext -o "$out\G1111" $probe chains-8-branchy      4194304

# ── chain-only variants (Pass B — assess_ext H vector) ───────────────────────
Write-Host ""
Write-Host "chain-only variants (Pass B)..." -ForegroundColor Cyan

& $uprof collect --config assess_ext -o "$out\chain_only_1x" $probe chain-only 524288
& $uprof collect --config assess_ext -o "$out\chain_only_2x" $probe chain-only 4194304
& $uprof collect --config assess_ext -o "$out\chain_only_4x" $probe chain-only 16777216

# ── Pass B reports ────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "Generating Pass B reports..." -ForegroundColor Cyan

$nodes_b = @("G0000","G0100","G0010","G0110","G1000","G1100","G1010","G1110",
             "G0001","G0101","G0011","G0111","G1001","G1101","G1011","G1111",
             "chain_only_1x","chain_only_2x","chain_only_4x")

foreach ($n in $nodes_b) {
    & $uprof report -i "$out\$n"
}

# ══════════════════════════════════════════════════════════════════════════════
# PASS A — load-type disaggregation (6 counters, no multiplexing)
# Addresses OC-DC-1 (refill aggregation) and OC-STLI-1 (STLF success rate)
# ══════════════════════════════════════════════════════════════════════════════
Write-Host ""
Write-Host "======================================================" -ForegroundColor White
Write-Host " PASS A — Load-Type Disaggregation                    " -ForegroundColor White
Write-Host " config: .\config\pass_a_load_type.xml                " -ForegroundColor White
Write-Host " Counters: ls_stlf, ls_bad_status2, ls_dmnd_fills     " -ForegroundColor White
Write-Host "======================================================" -ForegroundColor White
Write-Host ""

& $uprof collect --config $config -o "$out\pass_a\chain_only_1x" $probe chain-only 524288
& $uprof collect --config $config -o "$out\pass_a\chain_only_2x" $probe chain-only 4194304
& $uprof collect --config $config -o "$out\pass_a\chain_only_4x" $probe chain-only 16777216

# ── Pass A reports ────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "Generating Pass A reports..." -ForegroundColor Cyan

foreach ($n in @("chain_only_1x","chain_only_2x","chain_only_4x")) {
    & $uprof report -i "$out\pass_a\$n"
}

# ══════════════════════════════════════════════════════════════════════════════
# ΔR_stack — STOP HERE if assembly not yet declared
# Run: cargo rustc --release -- --emit=asm
# Inspect probe-*.s for run_chain_only_stack_spill
# Confirm distinct store/load addresses before proceeding
# ══════════════════════════════════════════════════════════════════════════════
Write-Host ""
Write-Host "======================================================" -ForegroundColor Yellow
Write-Host " ΔR_stack — ASSEMBLY DECLARATION REQUIRED BEFORE RUN  " -ForegroundColor Yellow
Write-Host " cargo rustc --release -- --emit=asm                   " -ForegroundColor Yellow
Write-Host " Inspect probe-*.s / run_chain_only_stack_spill        " -ForegroundColor Yellow
Write-Host " Declare in execution_record.md before proceeding      " -ForegroundColor Yellow
Write-Host "======================================================" -ForegroundColor Yellow
Write-Host ""
Write-Host "Press Enter to continue to ΔR_stack runs, or Ctrl+C to stop." -ForegroundColor White
Read-Host

# ── ΔR_stack: Pass B (assess_ext H vector) ───────────────────────────────────
Write-Host ""
Write-Host "ΔR_stack — Pass B (assess_ext)..." -ForegroundColor Magenta

& $uprof collect --config assess_ext -o "$out\stack_spill_1x" $probe chain-only-stack-spill 524288
& $uprof collect --config assess_ext -o "$out\stack_spill_2x" $probe chain-only-stack-spill 4194304

& $uprof report -i "$out\stack_spill_1x"
& $uprof report -i "$out\stack_spill_2x"

# ── ΔR_stack: Pass A (load-type disaggregation) ──────────────────────────────
Write-Host ""
Write-Host "ΔR_stack — Pass A (load-type)..." -ForegroundColor Magenta

& $uprof collect --config $config -o "$out\pass_a\stack_spill_1x" $probe chain-only-stack-spill 524288
& $uprof collect --config $config -o "$out\pass_a\stack_spill_2x" $probe chain-only-stack-spill 4194304

& $uprof report -i "$out\pass_a\stack_spill_1x"
& $uprof report -i "$out\pass_a\stack_spill_2x"

# ══════════════════════════════════════════════════════════════════════════════
# H VECTOR DISPLAY
# ══════════════════════════════════════════════════════════════════════════════
Write-Host ""
Write-Host "======================================================" -ForegroundColor White
Write-Host " Pass B H vectors — paste into execution_record.md    " -ForegroundColor White
Write-Host "======================================================" -ForegroundColor White

foreach ($n in $nodes_b) {
    Write-Host "`n=== $n ===" -ForegroundColor Cyan
    $rpt = "$out\$n\report.csv"
    if (Test-Path $rpt) {
        Get-Content $rpt | Select-Object -Skip 90 | Select-Object -First 6
    }
}

Write-Host ""
Write-Host "======================================================" -ForegroundColor White
Write-Host " Pass A H vectors — paste into execution_record.md    " -ForegroundColor White
Write-Host "======================================================" -ForegroundColor White

foreach ($n in @("chain_only_1x","chain_only_2x","chain_only_4x","stack_spill_1x","stack_spill_2x")) {
    Write-Host "`n=== pass_a\$n ===" -ForegroundColor Cyan
    $rpt = "$out\pass_a\$n\report.csv"
    if (Test-Path $rpt) {
        Get-Content $rpt | Where-Object {
            $_ -match "ls_stlf|ls_bad_status2|ls_dmnd_fills|ex_ret_instr|ls_not_halted"
        }
    }
}

Write-Host ""
Write-Host "Record all H vectors and derived quantities in execution_record.md V12." -ForegroundColor White
Write-Host "Compute: STLF_RATE = STLF_PTI / 111.11" -ForegroundColor White
Write-Host "         ΔR_stack = H(stack_spill) - H(chain_only) per counter" -ForegroundColor White
