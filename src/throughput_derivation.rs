// throughput_derivation.rs — Metatron Dynamics, Inc.
// Throughput derivation and MI355X ratio computation.
// Bounded over D. No claim beyond D.

pub const MI355X_ANALYSES_PER_SECOND: f64 = 7_629_394.0;

#[derive(Debug, Clone)]
pub struct ThroughputResult {
    pub mean_ns: f64,
    pub analyses_per_second: f64,
    pub mi355x_ratio: f64,
    pub epistemic_status: &'static str,
}

pub fn derive_throughput(mean_ns: f64) -> ThroughputResult {
    let analyses_per_second = 1.0e9 / mean_ns;
    let mi355x_ratio = MI355X_ANALYSES_PER_SECOND / analyses_per_second;
    ThroughputResult {
        mean_ns,
        analyses_per_second,
        mi355x_ratio,
        epistemic_status: "MIXED: MI355X is STRUCTURAL (abr-infinity-fabric); \
                            home system is MEASURED (wall-clock, L3-resident). \
                            OC-HB-1: L3 bandwidth not directly measured. \
                            OC-HB-3: MI355X correspondence requires instrument measurement.",
    }
}

pub fn throughput_report(result: &ThroughputResult) -> String {
    format!(
        "═══════════════════════════════════════════════════════════\n\
         ABR HOME SYSTEM BENCHMARK — THROUGHPUT DERIVATION\n\
         Ryzen 5 7600X / DDR5-5600 / 32 MB L3 (Zen 4)\n\
         Metatron Dynamics, Inc. · Bounded over D.\n\
         ═══════════════════════════════════════════════════════════\n\
         Mean time per ABR pass (A->B->R):  {:.1} ns\n\
         Home system throughput:            {:.0} analyses/second\n\
         MI355X declared throughput:        {:.0} analyses/second/module\n\
         MI355X / Home system ratio:        {:.1}x\n\
         ───────────────────────────────────────────────────────────\n\
         Epistemic status: {}\n\
         ═══════════════════════════════════════════════════════════",
        result.mean_ns,
        result.analyses_per_second,
        MI355X_ANALYSES_PER_SECOND,
        result.mi355x_ratio,
        result.epistemic_status,
    )
}
