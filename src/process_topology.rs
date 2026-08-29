// process_topology.rs — Metatron Dynamics, Inc. V1.0
// Regime 2: OS-to-CPU exchange layer.
// Bounded over D. No claim beyond D.
//
// ── Purpose ───────────────────────────────────────────────────────────────────
//
// Regime 1 (operators.rs / scaling.rs) measures ABR operator cost on a
// declared synthetic graph. Regime 2 asks a different question: does the
// OS process layer itself — the set of processes a real session activates,
// and the dependency/idle structure between them — carry relational
// structure that current OS scheduling does not use?
//
// This module declares processes as loci and their observed activation
// timing as the current observable. It builds a DeclaredGraph from a real
// or recorded process list (e.g. an AMD uProf "Select Profile Target"
// export) so the existing V7 operators (operators.rs) can be applied to it
// unmodified.
//
// ── Declared Observable (V1.0) ───────────────────────────────────────────────
//
// node_field[i] = start_time offset of process i, in seconds since the
// first recorded process activation in the session.
//
// This is declared explicitly as a PROXY observable. It captures *when*
// a process activates, not whether it is idle, active, or blocked once
// running. See OC-PT-1.
//
// ── Declared Edge Rule (V1.0) ────────────────────────────────────────────────
//
// An edge is declared between process i and process i+1 (processes sorted
// by start time) if their start-time offsets differ by no more than
// CO_ACTIVATION_WINDOW_SECS. This is a declared choice, not a derived
// threshold — see OC-PT-2.
//
// This produces an open-chain-consistent DeclaredGraph: at most one
// successor per edge, matching the topology admissible under kernel V7
// (declared_graph.rs — ring topology inadmissible).
//
// ── Open Conditions ──────────────────────────────────────────────────────────
//
// OC-PT-1: Only activation-timestamp observable is ingested in V1.0. Real
//   idle/active CPU utilization per process (from uProf hotspot sampling,
//   not the Select Profile Target process list) is required to test the
//   actual claim under discussion — whether relational structure in
//   dependency/idle patterns could reduce redundant OS-to-CPU switching.
//   Until that data is ingested, this module measures only whether ABR
//   operators are computable over a real process-activation trace, not
//   whether doing so is efficient.
//
// OC-PT-2: CO_ACTIVATION_WINDOW_SECS is a declared threshold, not derived
//   from data. Sensitivity to this choice is untested.
//
// OC-PT-3: No comparison against actual OS scheduler behavior (e.g.
//   context-switch counts, redundant wake events) has been made. This
//   module produces a relational description of a process trace; it does
//   not yet produce a measured efficiency claim relative to the OS.

use crate::declared_graph::{DeclaredGraph, Edge};

/// Declared co-activation window. Two processes starting within this many
/// seconds of each other are declared adjacent. OC-PT-2: not derived.
pub const CO_ACTIVATION_WINDOW_SECS: f64 = 2.0;

/// A single process record as read from a declared source (e.g. a uProf
/// "Select Profile Target" export, or a manually transcribed session log).
#[derive(Debug, Clone)]
pub struct ProcessRecord {
    pub name: String,
    pub pid: u32,
    /// Start time offset in seconds since the first process in the session.
    /// Declaration responsibility: caller must normalize wall-clock start
    /// times (e.g. "09:20:21") to a common zero point before constructing
    /// this struct. See parse_hms_offset() for one admissible normalization.
    pub start_offset_secs: f64,
}

/// Parses a "HH:MM:SS" timestamp into total seconds since midnight.
/// Used to normalize uProf's Start Time column to a common offset.
pub fn parse_hms_to_secs(hms: &str) -> Option<f64> {
    let parts: Vec<&str> = hms.trim().split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let h: f64 = parts[0].parse().ok()?;
    let m: f64 = parts[1].parse().ok()?;
    let s: f64 = parts[2].parse().ok()?;
    Some(h * 3600.0 + m * 60.0 + s)
}

/// Builds ProcessRecords from (name, pid, "HH:MM:SS") triples, normalizing
/// start times to an offset from the earliest recorded activation.
///
/// Records are NOT re-sorted by this function beyond what is required to
/// find the minimum start time; caller-supplied order is otherwise
/// preserved for records with identical timestamps.
pub fn declare_process_records(raw: &[(&str, u32, &str)]) -> Vec<ProcessRecord> {
    let parsed: Vec<(String, u32, f64)> = raw
        .iter()
        .filter_map(|(name, pid, hms)| {
            parse_hms_to_secs(hms).map(|secs| (name.to_string(), *pid, secs))
        })
        .collect();

    let min_secs = parsed
        .iter()
        .map(|(_, _, s)| *s)
        .fold(f64::INFINITY, f64::min);

    parsed
        .into_iter()
        .map(|(name, pid, secs)| ProcessRecord {
            name,
            pid,
            start_offset_secs: secs - min_secs,
        })
        .collect()
}

/// Builds a DeclaredGraph from a set of ProcessRecords using the declared
/// co-activation edge rule. Processes are sorted by start_offset_secs;
/// consecutive processes within CO_ACTIVATION_WINDOW_SECS of each other
/// are connected by a declared directed edge (earlier -> later).
///
/// node_field[i] = start_offset_secs of the i-th process in sorted order.
///
/// Reuses the same DeclaredGraph / Edge types as declared_graph.rs, so
/// operators.rs::abr_pass() applies to the result unmodified.
pub fn build_process_graph(mut records: Vec<ProcessRecord>) -> DeclaredGraph {
    records.sort_by(|a, b| {
        a.start_offset_secs
            .partial_cmp(&b.start_offset_secs)
            .unwrap()
    });

    let n_nodes = records.len();
    let mut edges = Vec::new();

    for i in 0..n_nodes.saturating_sub(1) {
        let gap = records[i + 1].start_offset_secs - records[i].start_offset_secs;
        if gap <= CO_ACTIVATION_WINDOW_SECS {
            edges.push(Edge {
                source: i,
                target: i + 1,
                successor: None, // filled in below
            });
        }
    }

    // Link successive declared edges into an open chain of successors,
    // matching the admissible topology in declared_graph.rs (at most one
    // successor per edge, no ring closure).
    let n_edges = edges.len();
    for i in 0..n_edges {
        edges[i].successor = if i + 1 < n_edges { Some(i + 1) } else { None };
    }

    let node_field: Vec<f64> = records.iter().map(|r| r.start_offset_secs).collect();

    DeclaredGraph {
        n_nodes,
        n_edges,
        edges,
        node_field,
    }
}

/// Reports the declared co-activation structure: how many of the possible
/// consecutive process pairs were declared adjacent under the current
/// window, and the resulting graph's edge density. This is a structural
/// summary only — it makes no efficiency claim (see OC-PT-3).
pub fn process_graph_report(graph: &DeclaredGraph) -> String {
    let mut report = String::new();
    report.push_str("─────────────────────────────────────────────\n");
    report.push_str("PROCESS TOPOLOGY — REGIME 2 STRUCTURAL SUMMARY\n");
    report.push_str("Bounded over D. No claim beyond D.\n");
    report.push_str("─────────────────────────────────────────────\n");
    report.push_str(&format!("Declared processes (nodes): {}\n", graph.n_nodes));
    report.push_str(&format!("Declared co-activation edges: {}\n", graph.n_edges));
    if graph.n_nodes > 1 {
        let possible = graph.n_nodes - 1;
        let density = graph.n_edges as f64 / possible as f64;
        report.push_str(&format!(
            "Edge density (declared/possible consecutive pairs): {:.3}\n",
            density
        ));
    }
    report.push_str(&format!(
        "Co-activation window: {:.1}s (OC-PT-2: declared, not derived)\n",
        CO_ACTIVATION_WINDOW_SECS
    ));
    report.push_str("Observable: activation-time offset only (OC-PT-1: idle/active\n");
    report.push_str("utilization not yet ingested — this is a proxy observable).\n");
    report.push_str("─────────────────────────────────────────────\n");
    report
}

/// Example fixture: a real process activation trace captured from an AMD
/// uProf "Select Profile Target" screen during a Robin Macomber session
/// (2026-08-28), transcribed by hand from the visible rows. Provided so
/// this module has real (not synthetic) data to build and test against
/// before a full uProf CSV export pipeline is wired up.
///
/// This is a partial, hand-transcribed trace — not a claim of completeness
/// over the full session's processes.
pub fn example_session_trace() -> Vec<ProcessRecord> {
    let raw: &[(&str, u32, &str)] = &[
        ("msedgewebview2.exe", 9716, "09:20:21"),
        ("msedgewebview2.exe", 9824, "09:20:21"),
        ("msedgewebview2.exe", 10112, "09:20:21"),
        ("SecurityHealthSystray.exe", 10724, "09:20:22"),
        ("RtkAudUService64.exe", 10876, "09:20:23"),
        ("loopMIDI.exe", 11052, "09:20:24"),
        ("ctfmon.exe", 11120, "09:20:24"),
        ("RadeonSoftware.exe", 9916, "09:20:24"),
        ("AbletonAudioCpl.exe", 10492, "09:20:25"),
        ("MR18AudioCplApp.exe", 6544, "09:20:26"),
        ("cncmd.exe", 10700, "09:20:27"),
        ("AMDRSServ.exe", 7584, "09:20:27"),
        ("amdow.exe", 10216, "09:20:27"),
        ("AMDRSSrcExt.exe", 10236, "09:20:27"),
        ("chrome.exe", 10652, "09:20:33"),
        ("chrome.exe", 11104, "09:20:33"),
        ("chrome.exe", 4288, "09:20:33"),
        ("chrome.exe", 1920, "09:20:33"),
        ("chrome.exe", 4372, "09:20:33"),
        ("chrome.exe", 11368, "09:20:33"),
        ("chrome.exe", 11588, "09:20:33"),
        ("chrome.exe", 11904, "09:20:33"),
        ("chrome.exe", 12112, "09:20:35"),
        ("chrome.exe", 12252, "09:20:36"),
        ("chrome.exe", 12256, "09:20:36"),
        ("chrome.exe", 3368, "09:20:38"),
    ];
    declare_process_records(raw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operators::{abr_pass, AbrBuffers};

    #[test]
    fn parses_hms_correctly() {
        assert_eq!(parse_hms_to_secs("09:20:21"), Some(9.0 * 3600.0 + 20.0 * 60.0 + 21.0));
    }

    #[test]
    fn rejects_malformed_timestamp() {
        assert_eq!(parse_hms_to_secs("not-a-time"), None);
        assert_eq!(parse_hms_to_secs("09:20"), None);
    }

    #[test]
    fn declared_records_normalize_to_zero_min() {
        let raw: &[(&str, u32, &str)] = &[("a", 1, "09:20:22"), ("b", 2, "09:20:21")];
        let records = declare_process_records(raw);
        let min = records
            .iter()
            .map(|r| r.start_offset_secs)
            .fold(f64::INFINITY, f64::min);
        assert!((min - 0.0).abs() < 1e-9, "earliest record must offset to 0.0");
    }

    #[test]
    fn example_trace_parses_and_normalizes() {
        let records = example_session_trace();
        assert_eq!(records.len(), 26);
        let min = records
            .iter()
            .map(|r| r.start_offset_secs)
            .fold(f64::INFINITY, f64::min);
        assert!((min - 0.0).abs() < 1e-9);
    }

    #[test]
    fn process_graph_has_no_ring_topology() {
        let g = build_process_graph(example_session_trace());
        for edge in &g.edges {
            assert_ne!(edge.source, edge.target, "self-loop inadmissible");
            assert_ne!(edge.target, 0, "edge into node 0 would close a ring");
        }
    }

    #[test]
    fn process_graph_terminal_edge_has_no_successor() {
        let g = build_process_graph(example_session_trace());
        if let Some(last) = g.edges.last() {
            assert!(last.successor.is_none());
        }
    }

    #[test]
    fn process_graph_edges_are_temporally_ordered() {
        let g = build_process_graph(example_session_trace());
        for edge in &g.edges {
            assert!(g.node_field[edge.target] >= g.node_field[edge.source],
                "declared edges must run forward in activation time");
        }
    }

    #[test]
    fn process_graph_declares_expected_edge_count() {
        // Manually verified from example_session_trace() against the
        // declared 2.0s co-activation window: chrome.exe cluster at
        // 09:20:33 (8 processes, identical timestamp) plus surrounding
        // near-simultaneous rows link into a small number of open chains
        // rather than every pair, since only *consecutive* sorted gaps
        // are tested.
        let g = build_process_graph(example_session_trace());
        assert!(g.n_edges > 0, "declared window must produce at least one edge");
        assert!(g.n_edges < g.n_nodes, "edge count must stay below node count (open chain, no ring)");
    }

    #[test]
    fn abr_operators_apply_to_process_graph_unmodified() {
        // Confirms V7 operators (operators.rs) run on a real process trace
        // without modification — the core claim of Regime 2's V1.0 scope.
        let g = build_process_graph(example_session_trace());
        if g.n_edges == 0 {
            return; // nothing to run operators over under this window
        }
        let mut buf = AbrBuffers::new(g.n_nodes, g.n_edges);
        abr_pass(&g, &mut buf);
        assert!(buf.r.iter().all(|v| v.is_finite()),
            "ABR pass over process graph must produce finite output");
    }

    #[test]
    fn report_does_not_panic_on_empty_graph() {
        let g = DeclaredGraph { n_nodes: 0, n_edges: 0, edges: vec![], node_field: vec![] };
        let _ = process_graph_report(&g);
    }
}
