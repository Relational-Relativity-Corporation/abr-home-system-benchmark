# run_probe_v9.ps1 — Metatron Dynamics, Inc.
# A×D×S×B factorial block — 16 uProf sessions
# probe V9.0 — abr-home-system-benchmark V11.0.0
# Bounded over D. No claim beyond D.
#
# Usage:
#   cd "$HOME\Downloads\abr_home_system_benchmark_v11"
#   cargo test --release
#   cargo build --release
#   .\run_probe_v9.ps1

$probe = ".\target\release\probe.exe"
$uprof = "C:\Program Files\AMD\AMDuProf\bin\AMDuProfCLI.exe"
$out   = "$HOME\uprof_out"

New-Item -ItemType Directory -Force -Path $out | Out-Null

Write-Host ""
Write-Host "===================================================" -ForegroundColor White
Write-Host " probe V9.0 — A×D×S×B factorial block (16 nodes)  " -ForegroundColor White
Write-Host " Metatron Dynamics, Inc.  Bounded over D.          " -ForegroundColor White
Write-Host "===================================================" -ForegroundColor White
Write-Host ""
Write-Host "OC-V9-1: Record %BR_MISP at ALL nodes including B=none." -ForegroundColor Yellow
Write-Host "Output folder: $out" -ForegroundColor White
Write-Host ""

# ── B=none nodes ──────────────────────────────────────────────────────────────
Write-Host "B=none nodes..." -ForegroundColor Cyan

& $uprof collect --config assess_ext -o "$out\G0000" $probe linear           524288
& $uprof collect --config assess_ext -o "$out\G0100" $probe scrambled         524288
& $uprof collect --config assess_ext -o "$out\G0010" $probe chains-8-seq      524288
& $uprof collect --config assess_ext -o "$out\G0110" $probe chains-8          524288

& $uprof collect --config assess_ext -o "$out\G1000" $probe linear            4194304
& $uprof collect --config assess_ext -o "$out\G1100" $probe scrambled          4194304
& $uprof collect --config assess_ext -o "$out\G1010" $probe chains-8-seq       4194304
& $uprof collect --config assess_ext -o "$out\G1110" $probe chains-8           4194304

# ── B=branchy nodes ───────────────────────────────────────────────────────────
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

# ── Generate reports ──────────────────────────────────────────────────────────
Write-Host ""
Write-Host "Generating reports..." -ForegroundColor Cyan

$nodes = @("G0000","G0100","G0010","G0110","G1000","G1100","G1010","G1110",
           "G0001","G0101","G0011","G0111","G1001","G1101","G1011","G1111")

foreach ($n in $nodes) {
    & $uprof report -i "$out\$n"
}

# ── Display H vectors ─────────────────────────────────────────────────────────
Write-Host ""
Write-Host "===================================================" -ForegroundColor White
Write-Host " H vectors — paste into execution_record.md V10.0  " -ForegroundColor White
Write-Host "===================================================" -ForegroundColor White

foreach ($n in $nodes) {
    Write-Host "`n=== $n ===" -ForegroundColor Cyan
    Get-Content "$out\$n\report.csv" | Select-Object -Skip 90 | Select-Object -First 6
}
