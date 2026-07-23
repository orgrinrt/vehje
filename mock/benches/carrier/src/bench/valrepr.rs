//! Value representation (static / runtime-tagged / NaN-boxed), one bench.
//!
//! One value-representation mini-IR, three interpreters over it. The shared setup
//! builds the program plus every cell's scratch (a plain u64 array for static, a
//! tag+bits pair for tagged, a nanbox array for nanbox), so no cell needs its own
//! setup and the only measured difference is the value representation.

use crate as c;
use mockspace_bench_matrix::bench_matrix;

/// Shared state: the value program and all three cells' scratch buffers.
pub struct St {
    pub prog: Vec<c::valrepr::VNode>,
    pub scr: Vec<u64>,
    pub tags: Vec<u8>,
    pub bits: Vec<u64>,
    pub vals: Vec<u64>,
}

bench_matrix! {
    name: "carrier_valrepr",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_d15b_a7c4_0002,
    sweep kind in [],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "static",
    regime: warm,

    setup |_profile: &str, n: usize| -> St {
        let prog = c::valrepr::gen_valprog(n, 0x1234_5678);
        let len = prog.len();
        St {
            prog,
            scr: vec![0u64; len],
            tags: vec![0u8; len],
            bits: vec![0u64; len],
            vals: vec![0u64; len],
        }
    }

    cell static |s, seed| { c::valrepr::interp_static(&s.prog, seed as i32, &mut s.scr) }
    cell tagged |s, seed| { c::valrepr::interp_tagged(&s.prog, seed as i32, &mut s.tags, &mut s.bits) }
    cell nanbox |s, seed| { c::valrepr::interp_nanbox(&s.prog, seed as i32, &mut s.vals) }
}
