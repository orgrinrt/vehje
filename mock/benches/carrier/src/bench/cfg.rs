//! CFG register-VM dispatch (switch / fntable / threaded / trace), one bench.
//!
//! Own workload: a nested-loop register-VM kernel sized by N. `switch` and
//! `fntable` interpret the block set directly. `threaded` flattens to a token
//! stream (in its per-cell setup, so the flatten is not timed). `trace` profiles
//! and anchors the hottest self-loop (in its per-cell setup, so the O(N)
//! trace-selection pass is not timed) and runs the linearized superblock.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state for the switch/fntable cells: the block set.
pub struct St {
    pub blocks: Vec<c::cfg::Block>,
}

/// The threaded cell holds the flattened token stream (built in its setup).
#[cfg(feature = "threaded")]
pub struct StThreaded {
    pub code: Vec<c::cfg_threaded::TInstr>,
}

/// The trace cell holds the block set and the selected trace (both built in its
/// setup). `interp_traced` needs both, and `Trace` is owned (block indices, not
/// borrows), so no leak is required.
pub struct StTrace {
    pub blocks: Vec<c::cfg::Block>,
    pub trace: c::trace::Trace,
}

bench_matrix! {
    name: "carrier_cfg",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep kind in [],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",
    regime: warm,

    setup |_profile: &str, n: usize| -> St {
        St { blocks: c::cfg::build_nested_loop(n as u64, 4) }
    }

    cell switch  |s, seed| { let (rr, _, _) = c::cfg::interp(&s.blocks, seed, u64::MAX); rr }
    cell fntable |s, seed| { let (rr, _, _) = c::cfg::interp_fntable(&s.blocks, seed, u64::MAX); rr }
    cell threaded
        #[feature = "threaded"]
        setup |_profile: &str, n: usize| -> StThreaded {
            let blocks = c::cfg::build_nested_loop(n as u64, 4);
            StThreaded { code: c::cfg_threaded::flatten(&blocks) }
        }
        |s, seed| { let (rr, _, _) = c::cfg_threaded::interp_flat(&s.code, seed, u64::MAX); rr }
    cell trace
        setup |_profile: &str, n: usize| -> StTrace {
            let blocks = c::cfg::build_nested_loop(n as u64, 4);
            let trace = c::trace::select_trace(&blocks, 0).expect("trace");
            StTrace { blocks, trace }
        }
        |s, seed| { let (rr, _, _) = c::trace::interp_traced(&s.blocks, &s.trace, seed, u64::MAX); rr }
}
